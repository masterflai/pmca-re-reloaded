//! Driver assignment, the one task that needs administrator rights.
//!
//! An elevated process does it itself. Any other process starts an elevated
//! helper once, on first need, and hands it each request over a private pipe:
//! one rights prompt per program run instead of one per service session. The
//! helper is the same executable started with [`HELPER_FLAG`]; it accepts five
//! requests, binds only Sony devices, removes only packages it finds itself,
//! and ends with the program that started it, releasing whatever is still
//! bound on the way out.

use std::io::{BufRead, BufReader, Write};
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicIsize, Ordering};
use std::time::Duration;

use pmca_i18n::Localizer;
use pmca_usb::winusb_install::{self as winusb, Cleanup, helper_pipe};
use serde::{Deserialize, Serialize};

/// First argument that turns an executable into the driver helper.
pub const HELPER_FLAG: &str = "--pmca-driver-helper";

/// Hardware ids the helper accepts: Sony's vendor id and nothing else.
const SONY_PREFIX: &str = r"USB\VID_054C&PID_";

/// How long the program waits for a started helper to open its pipe.
const CONNECT_ATTEMPTS: usize = 100;
const CONNECT_DELAY: Duration = Duration::from_millis(100);

/// Window the consent prompt belongs to, as a raw handle; 0 for none.
static PARENT_WINDOW: AtomicIsize = AtomicIsize::new(0);

/// The connection to a running helper, if one was started.
static HELPER: Mutex<Option<Connection>> = Mutex::new(None);

/// Whether [`HELPER`] holds a connection, readable without its lock.
///
/// The lock is held while the helper starts, and starting it waits for the
/// consent prompt, which needs the window's main thread. The window asks for
/// [`rights`] on that same thread, so taking the lock there deadlocks.
static HELPER_RUNNING: AtomicBool = AtomicBool::new(false);

/// Which rights the driver tasks currently run with.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub enum Rights {
    /// This process is elevated and assigns drivers itself.
    Elevated,
    /// An elevated helper is running for this process.
    Helper,
    /// Neither; the next driver task will ask Windows for the rights.
    Standard,
}

/// Remember the window the consent prompt should open in front of.
pub fn set_parent_window(handle: isize) {
    PARENT_WINDOW.store(handle, Ordering::Relaxed);
}

/// Which rights the driver tasks currently run with.
pub fn rights() -> Rights {
    if winusb::can_assign_drivers() {
        Rights::Elevated
    } else if HELPER_RUNNING.load(Ordering::Acquire) {
        Rights::Helper
    } else {
        Rights::Standard
    }
}

/// Start the helper now rather than at the first driver task, so the one
/// rights prompt of this program run comes when the user asks for it rather
/// than mid-task. Does nothing when the process is elevated or a helper is
/// already running.
///
/// A dismissed prompt comes back as [`crate::Error::NeedsElevation`]: nothing
/// was started, and the next driver task simply asks again.
pub fn start_helper(l: &Localizer) -> crate::Result<()> {
    if winusb::can_assign_drivers() {
        return Ok(());
    }
    let started = HELPER.lock().map_err(|_| winusb::Error::Install("helper lock".into())).and_then(
        |mut helper| {
            if helper.is_none() {
                *helper = Some(Connection::start()?);
                HELPER_RUNNING.store(true, Ordering::Release);
            }
            Ok(())
        },
    );
    started.map_err(|e| localize_error(l, e))
}

/// A driver-assignment failure in the user's language.
pub(crate) fn localize_error(l: &Localizer, e: winusb::Error) -> crate::Error {
    match e {
        winusb::Error::NeedsElevation => {
            crate::Error::NeedsElevation(l.tr("driver-helper-declined"))
        }
        e => crate::Error::Operation(l.tr_arg("driver-bind-failed", "detail", &e.to_string())),
    }
}

/// Assign WinUSB to a Sony device, starting the helper if necessary.
pub(crate) fn bind(hardware_id: &str) -> winusb::Result<String> {
    if winusb::can_assign_drivers() {
        return winusb::bind(hardware_id);
    }
    let reply = call(&Request::Bind { id: hardware_id.to_string() }, true)?;
    Ok(reply.published)
}

/// [`bind`] without asking for rights: only when they are already there.
/// For work nobody asked for, such as the start-up rescue of a camera.
pub(crate) fn bind_quietly(hardware_id: &str) -> winusb::Result<String> {
    if winusb::can_assign_drivers() {
        return winusb::bind(hardware_id);
    }
    let reply = call(&Request::Bind { id: hardware_id.to_string() }, false)?;
    Ok(reply.published)
}

/// Remove every package this program staged. Runs through the helper when one
/// is running; never starts one, since a session that bound a driver had it.
pub(crate) fn unbind_all() -> winusb::Result<Cleanup> {
    if winusb::can_assign_drivers() {
        return winusb::unbind_all();
    }
    let reply = call(&Request::UnbindAll, false)?;
    Ok(Cleanup { packages: reply.packages, certificates: 0 })
}

/// Remove leftovers of earlier runs, starting the helper if necessary. Only
/// called right before a camera task that needs the rights anyway.
pub(crate) fn sweep() -> Cleanup {
    if winusb::can_assign_drivers() {
        return winusb::sweep();
    }
    match call(&Request::Sweep, true) {
        Ok(reply) => Cleanup { packages: reply.packages, certificates: 0 },
        Err(_) => Cleanup::default(),
    }
}

/// A driver package Zadig or another libwdi-based tool left for a Sony device.
///
/// The original Sony-PMCA-RE had users install these by hand. Windows keeps
/// them and hands them to the camera on its own, ahead of the driver this
/// program assigns, which breaks service mode and, on a mass-storage product
/// id, the camera's drive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct ForeignDriver {
    /// Published name in the driver store, `oem<n>.inf`.
    pub published: String,
    /// Provider named in the package, e.g. `libusb-win32`.
    pub provider: String,
    /// The Sony hardware ids it claims.
    pub devices: Vec<String>,
}

/// The packages of this kind on this machine. Needs no rights.
pub fn foreign_drivers() -> Vec<ForeignDriver> {
    winusb::foreign_packages()
        .into_iter()
        .map(|p| ForeignDriver { published: p.published, provider: p.provider, devices: p.devices })
        .collect()
}

/// `oem122.inf (libusb-win32), …` for messages.
pub fn describe(drivers: &[ForeignDriver]) -> String {
    drivers
        .iter()
        .map(|d| format!("{} ({})", d.published, d.provider))
        .collect::<Vec<_>>()
        .join(", ")
}

/// Remove those packages, starting the helper if necessary, and return the
/// published names removed. The helper looks them up again itself.
pub fn remove_foreign_drivers(l: &Localizer) -> crate::Result<Vec<String>> {
    let removed = if winusb::can_assign_drivers() {
        winusb::remove_foreign().map(|cleanup| cleanup.packages)
    } else {
        call(&Request::RemoveForeign, true).map(|reply| reply.packages)
    };
    removed.map_err(|e| localize_error(l, e))
}

/// One request to the helper.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "op", rename_all = "kebab-case")]
enum Request {
    Bind { id: String },
    UnbindAll,
    Sweep,
    RemoveForeign,
}

/// The helper's answer. `error` is set on failure, with `needs-elevation`,
/// `bad-id` or `install` as its kind.
#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
struct Reply {
    error: Option<String>,
    message: String,
    published: String,
    packages: Vec<String>,
}

/// Send a request, starting the helper first when `start` allows it.
fn call(request: &Request, start: bool) -> winusb::Result<Reply> {
    let mut helper = HELPER.lock().map_err(|_| winusb::Error::Install("helper lock".into()))?;
    if helper.is_none() {
        if !start {
            return Err(winusb::Error::NeedsElevation);
        }
        *helper = Some(Connection::start()?);
        HELPER_RUNNING.store(true, Ordering::Release);
    }
    let Some(connection) = helper.as_mut() else { return Err(winusb::Error::NeedsElevation) };
    match connection.send(request) {
        Ok(reply) => reply_result(reply),
        Err(e) => {
            // A helper that stopped answering is gone; the next task starts a
            // new one rather than talking to a dead pipe.
            *helper = None;
            HELPER_RUNNING.store(false, Ordering::Release);
            Err(winusb::Error::Install(format!("driver helper: {e}")))
        }
    }
}

fn reply_result(reply: Reply) -> winusb::Result<Reply> {
    match reply.error.as_deref() {
        None => Ok(reply),
        Some("needs-elevation") => Err(winusb::Error::NeedsElevation),
        Some("bad-id") => Err(winusb::Error::BadHardwareId(reply.message)),
        Some(_) => Err(winusb::Error::Install(reply.message)),
    }
}

/// The program's end of the pipe.
struct Connection {
    writer: std::fs::File,
    reader: BufReader<std::fs::File>,
}

impl Connection {
    /// Start the helper elevated and connect to it. This is the one moment
    /// Windows asks the user for administrator rights.
    fn start() -> winusb::Result<Self> {
        let pipe = pipe_name();
        let sid = helper_pipe::current_user_sid()?;
        let arguments = format!("{HELPER_FLAG} {pipe} {} {sid}", std::process::id());
        let parent = match PARENT_WINDOW.load(Ordering::Relaxed) {
            0 => None,
            handle => Some(handle),
        };
        let helper_pid = winusb::launch_elevated(&arguments, parent)?;

        let path = helper_pipe::pipe_path(&pipe);
        for _ in 0..CONNECT_ATTEMPTS {
            if let Ok(file) = std::fs::OpenOptions::new().read(true).write(true).open(&path) {
                // Only the helper this process started may answer.
                if helper_pipe::server_pid(&file)? != helper_pid {
                    return Err(winusb::Error::Install("foreign driver helper pipe".into()));
                }
                let reader = BufReader::new(
                    file.try_clone().map_err(|e| winusb::Error::Install(e.to_string()))?,
                );
                return Ok(Self { writer: file, reader });
            }
            std::thread::sleep(CONNECT_DELAY);
        }
        Err(winusb::Error::Install("the driver helper did not start".into()))
    }

    fn send(&mut self, request: &Request) -> std::io::Result<Reply> {
        let mut line = serde_json::to_string(request).map_err(std::io::Error::other)?;
        line.push('\n');
        self.writer.write_all(line.as_bytes())?;
        self.writer.flush()?;
        let mut answer = String::new();
        if self.reader.read_line(&mut answer)? == 0 {
            return Err(std::io::Error::other("helper closed the pipe"));
        }
        serde_json::from_str(&answer).map_err(std::io::Error::other)
    }
}

/// A pipe name nobody can guess in advance.
fn pipe_name() -> String {
    use std::hash::{BuildHasher, Hasher};
    let mut hasher = std::collections::hash_map::RandomState::new().build_hasher();
    hasher.write_u32(std::process::id());
    hasher.write_u128(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or_default(),
    );
    let second = std::collections::hash_map::RandomState::new().build_hasher().finish();
    format!("pmca-driver-helper-{:016x}{second:016x}", hasher.finish())
}

/// Run as the helper, if `args` (without the program name) ask for it.
///
/// Returns `None` for an ordinary start; otherwise the exit code. Front-ends
/// call this first thing in `main`.
pub fn run_helper_if_requested(args: &[String]) -> Option<i32> {
    if args.first().map(String::as_str) != Some(HELPER_FLAG) {
        return None;
    }
    Some(match parse_helper_args(&args[1..]) {
        Some((pipe, parent, sid)) => serve(&pipe, parent, &sid),
        None => 2,
    })
}

fn parse_helper_args(args: &[String]) -> Option<(String, u32, String)> {
    let [pipe, parent, sid] = args else { return None };
    let pipe_ok = pipe.len() <= 64 && pipe.chars().all(|c| c.is_ascii_alphanumeric() || c == '-');
    let sid_ok = sid.strip_prefix("S-1-").is_some_and(|rest| {
        !rest.is_empty() && rest.chars().all(|c| c.is_ascii_digit() || c == '-')
    });
    let parent = parent.parse().ok()?;
    (pipe_ok && sid_ok).then(|| (pipe.clone(), parent, sid.clone()))
}

/// The helper's main loop: one client, one request per line.
fn serve(pipe: &str, parent: u32, sid: &str) -> i32 {
    // Whatever happens to the pipe, the helper must not outlive its program.
    std::thread::spawn(move || {
        helper_pipe::wait_for_exit(parent);
        let _ = winusb::unbind_all();
        std::process::exit(0);
    });

    let Ok(file) = helper_pipe::serve(pipe, sid, parent) else { return 1 };
    let Ok(read_half) = file.try_clone() else { return 1 };
    let mut writer = file;
    for line in BufReader::new(read_half).lines() {
        let Ok(line) = line else { break };
        let reply = match serde_json::from_str::<Request>(&line) {
            Ok(request) => handle(&request),
            Err(e) => {
                Reply { error: Some("install".into()), message: e.to_string(), ..Reply::default() }
            }
        };
        let Ok(mut text) = serde_json::to_string(&reply) else { break };
        text.push('\n');
        if writer.write_all(text.as_bytes()).and_then(|()| writer.flush()).is_err() {
            break;
        }
    }
    // The program went away: leave nothing bound behind it.
    let _ = winusb::unbind_all();
    0
}

/// Carry out one request with the helper's own rights.
fn handle(request: &Request) -> Reply {
    let failure = |e: winusb::Error| {
        let kind = match e {
            winusb::Error::NeedsElevation => "needs-elevation",
            winusb::Error::BadHardwareId(_) => "bad-id",
            _ => "install",
        };
        Reply { error: Some(kind.into()), message: e.to_string(), ..Reply::default() }
    };
    match request {
        Request::Bind { id } if !id.to_ascii_uppercase().starts_with(SONY_PREFIX) => {
            failure(winusb::Error::BadHardwareId(id.clone()))
        }
        Request::Bind { id } => match winusb::bind(id) {
            Ok(published) => Reply { published, ..Reply::default() },
            Err(e) => failure(e),
        },
        Request::UnbindAll => match winusb::unbind_all() {
            Ok(cleanup) => Reply { packages: cleanup.packages, ..Reply::default() },
            Err(e) => failure(e),
        },
        Request::Sweep => Reply { packages: winusb::sweep().packages, ..Reply::default() },
        Request::RemoveForeign => match winusb::remove_foreign() {
            Ok(cleanup) => Reply { packages: cleanup.packages, ..Reply::default() },
            Err(e) => failure(e),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn requests_and_replies_round_trip() {
        let request = Request::Bind { id: r"USB\VID_054C&PID_0336".into() };
        let text = serde_json::to_string(&request).unwrap();
        assert_eq!(text, r#"{"op":"bind","id":"USB\\VID_054C&PID_0336"}"#);
        assert_eq!(serde_json::from_str::<Request>(&text).unwrap(), request);
        assert_eq!(serde_json::to_string(&Request::UnbindAll).unwrap(), r#"{"op":"unbind-all"}"#);
        assert_eq!(
            serde_json::to_string(&Request::RemoveForeign).unwrap(),
            r#"{"op":"remove-foreign"}"#
        );
    }

    #[test]
    fn the_helper_binds_sony_devices_only() {
        let reply = handle(&Request::Bind { id: r"USB\VID_046D&PID_C52B".into() });
        assert_eq!(reply.error.as_deref(), Some("bad-id"));
    }

    #[test]
    fn helper_arguments_are_checked() {
        let ok =
            |a: &[&str]| parse_helper_args(&a.iter().map(|s| s.to_string()).collect::<Vec<_>>());
        assert!(ok(&["pmca-driver-helper-0a1b", "1234", "S-1-5-21-1-2-3-1001"]).is_some());
        assert!(ok(&["../evil", "1234", "S-1-5-21-1"]).is_none());
        assert!(ok(&["pipe", "x", "S-1-5-21-1"]).is_none());
        assert!(ok(&["pipe", "1", "D:P(A;;GA;;;WD)"]).is_none());
        assert!(ok(&["pipe", "1"]).is_none());
    }

    #[test]
    fn errors_come_back_as_their_kind() {
        let reply = Reply { error: Some("bad-id".into()), message: "x".into(), ..Reply::default() };
        assert!(matches!(reply_result(reply), Err(winusb::Error::BadHardwareId(_))));
        let reply = Reply { error: Some("needs-elevation".into()), ..Reply::default() };
        assert!(matches!(reply_result(reply), Err(winusb::Error::NeedsElevation)));
    }

    #[test]
    fn pipe_names_differ() {
        assert_ne!(pipe_name(), pipe_name());
        assert!(pipe_name().len() <= 64);
    }
}
