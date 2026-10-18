//! macOS: camera work in an elevated copy of the program.
//!
//! macOS hands a mass-storage camera only to root, and the capture that grants
//! it belongs to the process that made it. The window runs as the user and so
//! cannot hold such a camera itself. On first need it starts its own executable
//! with administrator rights, through the system's password dialog, and keeps
//! that copy as a broker for the rest of the run: service-mode sessions, the
//! tweaks and the function analysis live there, and the window sends it one
//! request at a time over a Unix socket in its private temporary folder. The
//! broker ends when the connection does, so it never outlives the window.
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
    _osascript: std::process::Child,
}

#[cfg(target_os = "macos")]
impl Broker {
    /// Start the broker through the system's password dialog and wait until it
    /// has connected.
    pub fn start(l: &pmca_i18n::Localizer) -> crate::Result<Self> {
        use std::io::Read;
        use std::os::unix::fs::MetadataExt;
        use std::os::unix::net::UnixListener;

        let exe = std::env::current_exe()?;
        let home = std::env::var_os("HOME")
            .map(PathBuf::from)
            .ok_or_else(|| std::io::Error::other("HOME is not set"))?;
        let owner = std::fs::metadata(&home)?;
        let socket = std::env::temp_dir().join(format!("pmca-broker-{}.sock", std::process::id()));
        let _ = std::fs::remove_file(&socket);
        let listener = UnixListener::bind(&socket)?;
        listener.set_nonblocking(true)?;

        let command: [std::ffi::OsString; 3] =
            [exe.into(), BROKER_FLAG.into(), socket.clone().into()];
        let args =
            osascript_args(&home, owner.uid(), owner.gid(), &l.tr("macos-admin-prompt"), &command);
        let mut osascript = std::process::Command::new("/usr/bin/osascript")
            .args(args)
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::piped())
            .spawn()?;

        let stream = loop {
            match listener.accept() {
                Ok((stream, _)) => break stream,
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    if osascript.try_wait()?.is_some() {
                        let _ = std::fs::remove_file(&socket);
                        let mut stderr = String::new();
                        if let Some(mut pipe) = osascript.stderr.take() {
                            let _ = pipe.read_to_string(&mut stderr);
                        }
                        // A cancelled password dialog is AppleScript error -128.
                        let message = if stderr.contains("-128") {
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
        Ok(Self { client: Client::new(reader, stream), _osascript: osascript })
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

/// The AppleScript that runs a command as root. Every value arrives as an
/// argument and is quoted by AppleScript itself, so none can break out of the
/// shell command. The broker lives as long as the window, far longer than
/// AppleScript's default timeout.
const SCRIPT: [&str; 9] = [
    "on run argv",
    "set cmd to \"HOME=\" & quoted form of item 1 of argv & \" SUDO_UID=\" & quoted form of item 2 of argv & \" SUDO_GID=\" & quoted form of item 3 of argv",
    "repeat with i from 5 to count of argv",
    "set cmd to cmd & \" \" & quoted form of item i of argv",
    "end repeat",
    "with timeout of 31536000 seconds",
    "do shell script cmd with prompt (item 4 of argv) with administrator privileges",
    "end timeout",
    "end run",
];

#[cfg_attr(not(target_os = "macos"), allow(dead_code))]
fn osascript_args(
    home: &std::path::Path,
    uid: u32,
    gid: u32,
    prompt: &str,
    command: &[std::ffi::OsString],
) -> Vec<std::ffi::OsString> {
    let mut args: Vec<std::ffi::OsString> = Vec::new();
    for line in SCRIPT {
        args.push("-e".into());
        args.push(line.into());
    }
    args.push(home.into());
    args.push(uid.to_string().into());
    args.push(gid.to_string().into());
    args.push(prompt.into());
    args.extend(command.iter().cloned());
    args
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
    fn every_value_is_an_argument_not_part_of_the_script() {
        let command: [std::ffi::OsString; 3] =
            ["/Applications/X.app/Contents/MacOS/x".into(), BROKER_FLAG.into(), "/tmp/s".into()];
        let args = osascript_args(std::path::Path::new("/Users/ann"), 501, 20, "Allow?", &command);
        let script: Vec<_> = args.iter().skip(1).step_by(2).take(SCRIPT.len()).collect();
        assert!(script.iter().zip(SCRIPT).all(|(a, s)| *a == s));
        let rest: Vec<_> = args[SCRIPT.len() * 2..].iter().map(|a| a.to_string_lossy()).collect();
        assert_eq!(
            rest,
            [
                "/Users/ann",
                "501",
                "20",
                "Allow?",
                "/Applications/X.app/Contents/MacOS/x",
                BROKER_FLAG,
                "/tmp/s"
            ]
        );
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
