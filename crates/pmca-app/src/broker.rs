//! macOS: camera work in an elevated copy of the program.
//!
//! macOS hands a mass-storage camera only to root, and the capture that grants
//! it belongs to the process that made it. The window runs as the user and so
//! cannot hold such a camera itself. On first need it starts the small
//! `pmca-broker` executable beside it as root through `sudo`, the way a camera
//! was reached from a terminal, and keeps it for the rest of the run:
//! service-mode sessions, the tweaks and the function analysis live there (see
//! [`crate::elevated`]), and the window sends it one request at a time over a
//! Unix socket in its private temporary folder. The broker ends when the
//! connection does, so it never outlives the window.
//!
//! The protocol is JSON lines over any byte stream and is tested on every
//! platform; only starting the broker is macOS-specific.

use std::io::{BufRead, Write};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::dto::ProgressEvent;
use crate::{Error, ProgressSink};

/// First argument that turns the executable into the broker; the second is
/// the window's socket.
pub const BROKER_FLAG: &str = "--pmca-elevated-broker";

/// The broker's executable, next to the window's.
pub const BROKER_EXE: &str = "pmca-broker";

/// Set for the broker executable when `sudo` runs it as its password helper:
/// the question to ask.
pub const ASKPASS_PROMPT: &str = "PMCA_ASKPASS_PROMPT";

/// The dialog asking for the password. Prompt and title arrive as arguments,
/// so neither can break out of the script.
const ASKPASS_SCRIPT: [&str; 4] = [
    "on run argv",
    "display dialog (item 1 of argv) default answer \"\" with hidden answer with title (item 2 of argv) with icon caution",
    "return text returned of result",
    "end run",
];

/// When `sudo` runs this executable as its password helper: show the dialog,
/// whose answer goes straight to `sudo`. Returns the exit code if that was
/// asked for and the dialog could not be shown.
#[cfg(unix)]
pub fn run_askpass_if_requested() -> Option<i32> {
    use std::os::unix::process::CommandExt;

    let prompt = std::env::var_os(ASKPASS_PROMPT)?;
    let error = std::process::Command::new("/usr/bin/osascript")
        .args(askpass_args(&prompt, "PMCA-RE RELOADED"))
        .exec();
    eprintln!("osascript: {error}");
    Some(1)
}

#[cfg_attr(not(unix), allow(dead_code))]
fn askpass_args(prompt: &std::ffi::OsStr, title: &str) -> Vec<std::ffi::OsString> {
    let mut args: Vec<std::ffi::OsString> = Vec::new();
    for line in ASKPASS_SCRIPT {
        args.push("-e".into());
        args.push(line.into());
    }
    args.push(prompt.into());
    args.push(title.into());
    args
}

/// What `sudo` is asked to run: the broker as root, with the user's home.
/// `sudo` itself sets `SUDO_UID`, which [`connect_to_window`] checks.
#[cfg_attr(not(target_os = "macos"), allow(dead_code))]
fn sudo_args(
    home: &std::path::Path,
    broker: &std::path::Path,
    socket: &std::path::Path,
) -> Vec<std::ffi::OsString> {
    let mut home_var = std::ffi::OsString::from("HOME=");
    home_var.push(home);
    vec![
        "-A".into(),
        "--".into(),
        "/usr/bin/env".into(),
        home_var,
        broker.into(),
        BROKER_FLAG.into(),
        socket.into(),
    ]
}

/// A failure as the broker reported it, flattened to what a front-end shows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RemoteError {
    /// [`Error::code`].
    pub code: String,
    /// The error's own wording.
    pub message: String,
    /// [`Error::detail`].
    pub detail: Option<String>,
    /// [`Error::recovery`].
    pub recovery: Option<String>,
}

impl From<&Error> for RemoteError {
    fn from(e: &Error) -> Self {
        Self {
            code: e.code().to_string(),
            message: e.to_string(),
            detail: e.detail(),
            recovery: e.recovery().map(str::to_string),
        }
    }
}

/// One line from the broker.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum Reply {
    Progress(ProgressEvent),
    Done(Value),
    Failed(RemoteError),
}

/// How a call to the broker failed.
#[derive(Debug)]
pub enum CallError {
    /// The broker ran the request and it failed there.
    Remote(RemoteError),
    /// The connection is gone; the broker has to be started again.
    Broken(std::io::Error),
}

/// The socket to connect to when `args` (without the program name) start the
/// broker: `Some(None)` for a malformed request, which must not fall through
/// to starting the window as root.
pub fn broker_socket(args: &[String]) -> Option<Option<PathBuf>> {
    if args.first().map(String::as_str) != Some(BROKER_FLAG) {
        return None;
    }
    match args {
        [_, socket] => Some(Some(PathBuf::from(socket))),
        _ => Some(None),
    }
}

/// Answer requests read from `reader` on `writer` until the window closes the
/// connection.
pub fn serve(
    reader: impl BufRead,
    mut writer: impl Write,
    mut handle: impl FnMut(Value, &mut dyn ProgressSink) -> Result<Value, RemoteError>,
) {
    for line in reader.lines() {
        let Ok(line) = line else { break };
        if line.trim().is_empty() {
            continue;
        }
        let reply = match serde_json::from_str::<Value>(&line) {
            Ok(request) => {
                let mut sink = |event: ProgressEvent| {
                    let _ = send(&mut writer, &Reply::Progress(event));
                };
                match handle(request, &mut sink) {
                    Ok(value) => Reply::Done(value),
                    Err(e) => Reply::Failed(e),
                }
            }
            Err(e) => Reply::Failed((&Error::Json(e)).into()),
        };
        if send(&mut writer, &reply).is_err() {
            break;
        }
    }
}

fn send(writer: &mut impl Write, reply: &Reply) -> std::io::Result<()> {
    let mut text = serde_json::to_string(reply).map_err(std::io::Error::other)?;
    text.push('\n');
    writer.write_all(text.as_bytes())?;
    writer.flush()
}

/// The window's end of the connection.
pub struct Client<R, W> {
    reader: R,
    writer: W,
}

impl<R: BufRead, W: Write> Client<R, W> {
    /// Wrap the two halves of a connection to the broker.
    pub fn new(reader: R, writer: W) -> Self {
        Self { reader, writer }
    }

    /// Send one request and relay the broker's progress until it answers.
    pub fn call(
        &mut self,
        request: &Value,
        sink: &mut dyn ProgressSink,
    ) -> Result<Value, CallError> {
        let broken = CallError::Broken;
        let mut text =
            serde_json::to_string(request).map_err(|e| broken(std::io::Error::other(e)))?;
        text.push('\n');
        self.writer
            .write_all(text.as_bytes())
            .and_then(|()| self.writer.flush())
            .map_err(broken)?;
        loop {
            let mut line = String::new();
            if self.reader.read_line(&mut line).map_err(broken)? == 0 {
                return Err(broken(std::io::ErrorKind::UnexpectedEof.into()));
            }
            match serde_json::from_str::<Reply>(&line) {
                Ok(Reply::Progress(event)) => sink.emit(event),
                Ok(Reply::Done(value)) => return Ok(value),
                Ok(Reply::Failed(e)) => return Err(CallError::Remote(e)),
                Err(e) => return Err(broken(std::io::Error::other(e))),
            }
        }
    }
}

/// Connect the broker to the window's socket: only a socket, and only one the
/// user the broker was started for owns.
#[cfg(unix)]
pub fn connect_to_window(
    socket: &std::path::Path,
) -> std::io::Result<std::os::unix::net::UnixStream> {
    use std::os::unix::fs::{FileTypeExt, MetadataExt};

    let meta = std::fs::symlink_metadata(socket)?;
    let owner = std::env::var("SUDO_UID").ok().and_then(|v| v.parse::<u32>().ok());
    if !meta.file_type().is_socket() || owner != Some(meta.uid()) {
        return Err(std::io::Error::other("not the window's socket"));
    }
    std::os::unix::net::UnixStream::connect(socket)
}

/// A running broker, as the window holds it.
#[cfg(target_os = "macos")]
pub struct Broker {
    client:
        Client<std::io::BufReader<std::os::unix::net::UnixStream>, std::os::unix::net::UnixStream>,
    _sudo: std::process::Child,
}

#[cfg(target_os = "macos")]
impl Broker {
    /// Start the broker through `sudo`, asking for the password in a dialog,
    /// and wait until it has connected.
    pub fn start(l: &pmca_i18n::Localizer) -> crate::Result<Self> {
        use std::io::Read;
        use std::os::unix::net::UnixListener;

        let broker = std::env::current_exe()?.with_file_name(BROKER_EXE);
        if !broker.is_file() {
            let missing = format!("{} ({})", l.tr("macos-admin-failed"), broker.display());
            return Err(Error::Operation(missing));
        }
        let home = std::env::var_os("HOME")
            .map(PathBuf::from)
            .ok_or_else(|| std::io::Error::other("HOME is not set"))?;
        let socket = std::env::temp_dir().join(format!("pmca-broker-{}.sock", std::process::id()));
        let _ = std::fs::remove_file(&socket);
        let listener = UnixListener::bind(&socket)?;
        listener.set_nonblocking(true)?;

        let mut sudo = std::process::Command::new("/usr/bin/sudo")
            .args(sudo_args(&home, &broker, &socket))
            .env("SUDO_ASKPASS", &broker)
            .env(ASKPASS_PROMPT, l.tr("macos-admin-prompt"))
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::piped())
            .spawn()?;

        let stream = loop {
            match listener.accept() {
                Ok((stream, _)) => break stream,
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    if sudo.try_wait()?.is_some() {
                        let _ = std::fs::remove_file(&socket);
                        let mut stderr = String::new();
                        if let Some(mut pipe) = sudo.stderr.take() {
                            let _ = pipe.read_to_string(&mut stderr);
                        }
                        // A cancelled dialog leaves `sudo` without a password.
                        let message = if stderr.contains("no password was provided")
                            || stderr.contains("a password is required")
                        {
                            l.tr("macos-admin-declined")
                        } else {
                            format!("{} ({})", l.tr("macos-admin-failed"), stderr.trim())
                        };
                        return Err(Error::Operation(message));
                    }
                    std::thread::sleep(std::time::Duration::from_millis(100));
                }
                Err(e) => {
                    let _ = std::fs::remove_file(&socket);
                    return Err(e.into());
                }
            }
        };
        let _ = std::fs::remove_file(&socket);
        stream.set_nonblocking(false)?;
        let reader = std::io::BufReader::new(stream.try_clone()?);
        Ok(Self { client: Client::new(reader, stream), _sudo: sudo })
    }

    /// See [`Client::call`].
    pub fn call(
        &mut self,
        request: &Value,
        sink: &mut dyn ProgressSink,
    ) -> Result<Value, CallError> {
        self.client.call(request, sink)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dto::ProgressKind;
    use std::io::BufReader;

    #[test]
    fn a_reply_survives_the_wire() {
        let replies = [
            Reply::Progress(ProgressEvent::step(ProgressKind::Info, "reading")),
            Reply::Done(serde_json::json!({ "tweaks": [] })),
            Reply::Failed((&Error::DirectRefused("refused".into())).into()),
        ];
        for reply in replies {
            let text = serde_json::to_string(&reply).unwrap();
            assert!(!text.contains('\n'));
            assert_eq!(serde_json::from_str::<Reply>(&text).unwrap(), reply);
        }
    }

    #[test]
    fn a_refusal_keeps_its_recovery_across_the_broker() {
        let remote: RemoteError = (&Error::DirectRefused("refused".into())).into();
        assert_eq!(remote.recovery.as_deref(), Some(crate::RECOVERY_WHOLE_IMAGE));
        assert_eq!(remote.code, "operation");
    }

    #[test]
    fn only_the_flag_starts_a_broker_and_a_bad_request_never_starts_the_window() {
        let args = |a: &[&str]| a.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        assert_eq!(broker_socket(&args(&[])), None);
        assert_eq!(broker_socket(&args(&["--lang", "de-DE"])), None);
        assert_eq!(broker_socket(&args(&[BROKER_FLAG])), Some(None));
        assert_eq!(broker_socket(&args(&[BROKER_FLAG, "/tmp/s", "x"])), Some(None));
        assert_eq!(
            broker_socket(&args(&[BROKER_FLAG, "/tmp/s"])),
            Some(Some(PathBuf::from("/tmp/s")))
        );
    }

    #[test]
    fn sudo_runs_the_broker_with_the_home_and_nothing_through_a_shell() {
        let args = sudo_args(
            std::path::Path::new("/Users/ann o'neil"),
            std::path::Path::new("/Applications/X.app/Contents/MacOS/pmca-broker"),
            std::path::Path::new("/tmp/s"),
        );
        let args: Vec<_> = args.iter().map(|a| a.to_string_lossy()).collect();
        assert_eq!(
            args,
            [
                "-A",
                "--",
                "/usr/bin/env",
                "HOME=/Users/ann o'neil",
                "/Applications/X.app/Contents/MacOS/pmca-broker",
                BROKER_FLAG,
                "/tmp/s"
            ]
        );
    }

    #[test]
    fn the_password_question_is_an_argument_not_part_of_the_script() {
        let args = askpass_args(std::ffi::OsStr::new("\" & do shell script \"x"), "T");
        let script: Vec<_> = args.iter().skip(1).step_by(2).take(ASKPASS_SCRIPT.len()).collect();
        assert!(script.iter().zip(ASKPASS_SCRIPT).all(|(a, s)| *a == s));
        let rest: Vec<_> =
            args[ASKPASS_SCRIPT.len() * 2..].iter().map(|a| a.to_string_lossy()).collect();
        assert_eq!(rest, ["\" & do shell script \"x", "T"]);
    }

    /// The whole exchange over a real socket pair: progress arrives before the
    /// answer, a failure keeps its fields, and a closed window ends the broker.
    #[test]
    fn a_call_relays_progress_and_returns_the_answer() {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let broker = std::thread::spawn(move || {
            let (stream, _) = listener.accept().unwrap();
            let reader = BufReader::new(stream.try_clone().unwrap());
            serve(reader, stream, |request, sink| {
                sink.emit(ProgressEvent::step(ProgressKind::Info, "working"));
                match request["op"].as_str() {
                    Some("echo") => Ok(request["value"].clone()),
                    _ => Err((&Error::Operation("unknown".into())).into()),
                }
            });
        });

        let stream = std::net::TcpStream::connect(address).unwrap();
        let mut client = Client::new(BufReader::new(stream.try_clone().unwrap()), stream);
        let mut seen = Vec::new();
        let mut sink = |e: ProgressEvent| seen.push(e.message);
        let answer =
            client.call(&serde_json::json!({ "op": "echo", "value": 7 }), &mut sink).unwrap();
        assert_eq!(answer, 7);
        match client.call(&serde_json::json!({ "op": "nonesuch" }), &mut sink) {
            Err(CallError::Remote(e)) => assert_eq!(e.message, "unknown"),
            other => panic!("{other:?}"),
        }
        drop(client);
        broker.join().unwrap();
        assert_eq!(seen, ["working", "working"]);
    }
}
