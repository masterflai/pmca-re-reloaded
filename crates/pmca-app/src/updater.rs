//! The interactive updater-shell session, shared by the console and the window.
//!
//! Where [`crate::service`] talks to a factory mode Sony built into the camera,
//! this module talks to code *this project* put there: the payload under
//! `updatershell/` is written like a firmware image, and once it is running the
//! host speaks the maintenance-shell protocol to it. The two shells overlap in
//! what they can do — both read and write the backup region, both copy files,
//! both dump memory — while sharing no protocol at all. That overlap is exactly
//! why the command set lives here rather than in a front-end: `bk r 0x01070047`
//! has to mean the same thing typed into `pmca-console` as typed into the
//! window, and the only way to guarantee that is one parser and one dispatcher.
//!
//! # Getting there is destructive, staying there is not
//!
//! Reaching this shell means rebooting the camera into updater mode and writing
//! a payload, which is why [`OpenRequest::flash`] exists and defaults to off.
//! Once running, the shell itself is reversible: turning the camera off and on
//! again discards the payload. The commands it offers are a different matter —
//! `bk w` writes the camera's persistent settings memory.
//!
//! # Why the session runs on a thread of its own
//!
//! The payload only becomes reachable *inside* the firmware write: the updater
//! protocol hands control to a completion callback once the final chunk is on
//! the wire, and the shell lives for as long as that callback is on the stack.
//! A console can simply run its prompt loop in there. A window cannot — its
//! commands arrive one IPC call at a time, each on a thread from a pool, and
//! none of them may block inside somebody else's stack frame. So the whole flow
//! is moved onto one owned thread and driven over channels: the callback runs
//! the loop, and every front-end sends it lines.

use std::path::{Path, PathBuf};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::thread::JoinHandle;

use pmca_i18n::Localizer;
use pmca_usb::command::SonyExtCmdTransport;
use pmca_usb::discovery::{self, CameraMode};
use pmca_usb::platform_shell::PlatformShell;
use pmca_usb::sony_updater::SonyUpdaterCamera;

use crate::dto::{ProgressEvent, ProgressKind};
use crate::firmware::BoxedUpdater;
use crate::service::{hex, hex_bytes, hexdump, tokenize, usage};
use crate::{Error, ProgressSink, Result};

/// The command reference, in the syntax the parser accepts.
///
/// Untranslated for the same reason as the service shell's: these are words
/// that have to be typed exactly as they appear.
pub const HELP: &str = "\
info                              read the backup status block
bk r <hexid>                      read a backup entry
bk w <hexid> <hex...>             write a backup entry (WRITES)
bk s                              synchronise the backup to flash (WRITES)
bk lock | bk unlock               toggle protected backup settings (WRITES)
push <local> <remote>             copy a local file to the camera (WRITES)
pull <remote> [local]             copy a camera file to the computer
rmem <hexoff> <hexlen> <outfile>  dump device memory
shell [tcp <port>]                open a root shell on the camera, console only (WRITES)
help                              show this help
exit                              leave the shell";

/// One command, already parsed and validated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpdaterRequest {
    /// Read the backup status block.
    Info,
    /// Read one entry of the backup region.
    BackupRead {
        /// Entry identifier.
        id: u32,
    },
    /// Overwrite one entry of the backup region.
    BackupWrite {
        /// Entry identifier.
        id: u32,
        /// Replacement contents.
        bytes: Vec<u8>,
    },
    /// Persist the backup region to flash.
    BackupSync,
    /// Lock or unlock the protected backup settings.
    BackupProtect {
        /// Whether protection should be on.
        lock: bool,
    },
    /// Copy a file onto the camera.
    Push {
        /// Source on this computer.
        local: PathBuf,
        /// Destination path on the camera.
        remote: String,
    },
    /// Copy a file off the camera.
    Pull {
        /// Path on the camera.
        remote: String,
        /// Destination on this computer.
        local: PathBuf,
    },
    /// Dump a region of device memory to a file.
    ReadMemory {
        /// First address to read.
        offset: u32,
        /// Number of bytes to read.
        size: u32,
        /// Destination on this computer.
        local: PathBuf,
    },
    /// Tunnel a root shell on the camera to a local endpoint.
    ///
    /// Carried by this enum although only the console can carry it out, so that
    /// both front-ends agree on what the word `shell` means and the window can
    /// say where to type it instead of reporting an unknown command.
    Shell {
        /// Loopback port to expose the shell on, or `None` for the terminal.
        tcp: Option<u16>,
    },
    /// Show the command reference.
    Help,
    /// Leave the shell.
    Exit,
}

impl UpdaterRequest {
    /// Whether carrying this out changes the camera.
    ///
    /// `shell` counts: it is an unauthenticated root shell on the camera's
    /// Android system, and what happens in there is not this parser's to know.
    #[must_use]
    pub fn writes(&self) -> bool {
        matches!(
            self,
            Self::BackupWrite { .. }
                | Self::BackupSync
                | Self::BackupProtect { .. }
                | Self::Push { .. }
                | Self::Shell { .. }
        )
    }
}

/// What a command produced.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdaterOutcome {
    /// One line describing what happened, already translated.
    pub message: String,
    /// Fixed-width output — a hex dump or the command reference — if any.
    pub dump: Option<String>,
    /// Whether the session should end after this command.
    pub finished: bool,
}

impl UpdaterOutcome {
    fn message(message: String) -> Self {
        Self { message, dump: None, finished: false }
    }

    fn dump(message: String, dump: String) -> Self {
        Self { message, dump: Some(dump), finished: false }
    }
}

/// Turn one line of input into a request.
///
/// An empty line yields `None`, which is not an error: pressing return at a
/// prompt is a normal thing to do.
pub fn parse(line: &str, l: &Localizer) -> Result<Option<UpdaterRequest>> {
    let tokens = tokenize(line, l)?;
    let Some((command, rest)) = tokens.split_first() else {
        return Ok(None);
    };
    let request = match (command.as_str(), rest) {
        ("exit" | "quit", _) => UpdaterRequest::Exit,
        ("help" | "?", _) => UpdaterRequest::Help,
        ("info", _) => UpdaterRequest::Info,
        ("bk", [sub, args @ ..]) => backup(l, sub, args)?,
        ("bk", _) => return Err(usage(l, "bk r | bk w | bk s | bk lock | bk unlock")),
        ("push", [local, remote]) => {
            UpdaterRequest::Push { local: PathBuf::from(local), remote: remote.clone() }
        }
        ("push", _) => return Err(usage(l, "push <local> <remote>")),
        // The camera's own file name, not its whole path: the reference keeps
        // only the base name here, and a remote `/foo/bar` would otherwise be
        // written to a local directory that need not exist.
        ("pull", [remote]) => UpdaterRequest::Pull {
            remote: remote.clone(),
            local: PathBuf::from(
                Path::new(remote).file_name().unwrap_or_else(|| "pull.bin".as_ref()),
            ),
        },
        ("pull", [remote, local]) => {
            UpdaterRequest::Pull { remote: remote.clone(), local: PathBuf::from(local) }
        }
        ("pull", _) => return Err(usage(l, "pull <remote> [local]")),
        ("rmem", [offset, size, local]) => UpdaterRequest::ReadMemory {
            offset: hex(l, offset)?,
            size: hex(l, size)?,
            local: PathBuf::from(local),
        },
        ("rmem", _) => return Err(usage(l, "rmem <hexoff> <hexlen> <outfile>")),
        ("shell", []) => UpdaterRequest::Shell { tcp: None },
        ("shell", [tcp, port]) if tcp == "tcp" => UpdaterRequest::Shell {
            tcp: Some(
                port.parse()
                    .map_err(|_| Error::Operation(l.tr_arg("service-bad-hex", "value", port)))?,
            ),
        },
        ("shell", _) => return Err(usage(l, "shell [tcp <port>]")),
        (other, _) => {
            return Err(Error::Operation(l.tr_arg("service-unknown-command", "command", other)));
        }
    };
    Ok(Some(request))
}

/// Parse the `bk` subcommands.
fn backup(l: &Localizer, sub: &str, args: &[String]) -> Result<UpdaterRequest> {
    match (sub, args) {
        ("r", [id]) => Ok(UpdaterRequest::BackupRead { id: hex(l, id)? }),
        ("w", [id, bytes @ ..]) if !bytes.is_empty() => {
            Ok(UpdaterRequest::BackupWrite { id: hex(l, id)?, bytes: hex_bytes(l, bytes)? })
        }
        ("s", []) => Ok(UpdaterRequest::BackupSync),
        ("lock", []) => Ok(UpdaterRequest::BackupProtect { lock: true }),
        ("unlock", []) => Ok(UpdaterRequest::BackupProtect { lock: false }),
        _ => Err(usage(l, "bk r <hexid> | bk w <hexid> <hex...> | bk s | bk lock | bk unlock")),
    }
}

/// Carry out one command against a running payload.
///
/// Generic over the transport so the console can call it on the shell it owns
/// while the window calls it from inside [`UpdaterSession`]; there is no second
/// implementation either could drift from.
///
/// [`UpdaterRequest::Shell`] is refused here rather than silently ignored: the
/// tunnel needs a local endpoint that can be polled without blocking the
/// camera's half of the conversation, which a request/response IPC call is not.
pub fn execute<T: SonyExtCmdTransport>(
    shell: &mut PlatformShell<T>,
    l: &Localizer,
    request: &UpdaterRequest,
) -> Result<UpdaterOutcome> {
    match request {
        UpdaterRequest::Help => Ok(UpdaterOutcome::dump(l.tr("service-help"), HELP.to_string())),
        UpdaterRequest::Exit => {
            Ok(UpdaterOutcome { message: l.tr("updater-shell-done"), dump: None, finished: true })
        }
        UpdaterRequest::Shell { .. } => Err(Error::Operation(l.tr("updater-shell-console-only"))),
        UpdaterRequest::Info => {
            let status = shell.backup_status()?;
            Ok(UpdaterOutcome::dump(
                l.tr_arg("service-read-bytes", "count", &status.len().to_string()),
                hexdump(&status),
            ))
        }
        UpdaterRequest::BackupRead { id } => {
            let value = shell.read_backup(*id)?;
            Ok(UpdaterOutcome::dump(
                l.tr_arg("service-read-bytes", "count", &value.len().to_string()),
                hexdump(&value),
            ))
        }
        UpdaterRequest::BackupWrite { id, bytes } => {
            shell.write_backup(*id, bytes)?;
            Ok(UpdaterOutcome::message(l.tr_arg(
                "service-backup-written",
                "id",
                &format!("0x{id:x}"),
            )))
        }
        UpdaterRequest::BackupSync => {
            shell.sync_backup()?;
            Ok(UpdaterOutcome::message(l.tr("updater-backup-synced")))
        }
        UpdaterRequest::BackupProtect { lock } => {
            shell.set_backup_protection(*lock)?;
            Ok(UpdaterOutcome::message(l.tr(if *lock {
                "updater-backup-locked"
            } else {
                "updater-backup-writable"
            })))
        }
        UpdaterRequest::Push { local, remote } => {
            let mut file = std::fs::File::open(local)?;
            shell.write_file(remote, &mut file)?;
            Ok(UpdaterOutcome::message(l.tr_args(
                "service-pushed",
                &[("local", &local.display().to_string()), ("remote", remote)],
            )))
        }
        UpdaterRequest::Pull { remote, local } => {
            let mut file = std::fs::File::create(local)?;
            shell.read_file(remote, &mut file)?;
            Ok(UpdaterOutcome::message(l.tr_args(
                "service-pulled",
                &[("remote", remote), ("local", &local.display().to_string())],
            )))
        }
        UpdaterRequest::ReadMemory { offset, size, local } => {
            let mut file = std::fs::File::create(local)?;
            shell.read_memory(*offset, *size, &mut file)?;
            Ok(UpdaterOutcome::message(l.tr_args(
                "service-dumped",
                &[
                    ("size", &format!("0x{size:x}")),
                    ("offset", &format!("0x{offset:x}")),
                    ("local", &local.display().to_string()),
                ],
            )))
        }
    }
}

/// Which payload to inject, and whether the camera may be modified to do it.
#[derive(Debug, Clone, Default)]
pub struct OpenRequest {
    /// Camera model to pick a bundled payload for, or `None` to detect it.
    pub model: Option<String>,
    /// An explicit payload file, bypassing the bundled tree entirely.
    pub fdat_file: Option<PathBuf>,
    /// Root of the bundled updater-body templates (`updatershell/bodyfs`).
    pub fdat_dir: PathBuf,
    /// Permission to reboot the camera into updater mode and write the payload.
    pub flash: bool,
}

/// Open the camera, resolve the payload and validate it against the device.
///
/// Nothing is written: this is `init` + guard check + version query, the same
/// read-only sequence [`crate::firmware::flash_firmware`] runs before a flash.
/// Returned alongside the updater is whether the camera was *already* in
/// updater mode, which decides whether reaching the shell costs a reboot.
pub fn prepare(
    l: &Localizer,
    request: &OpenRequest,
    sink: &mut dyn ProgressSink,
) -> Result<(Vec<u8>, BoxedUpdater, bool)> {
    let camera = discovery::find_single_camera()?;
    let already_updater = camera.mode == CameraMode::Updater;
    let transport = discovery::open_extcmd_transport(&camera)?;

    let (payload, transport): (Vec<u8>, Box<dyn SonyExtCmdTransport>) =
        match request.fdat_file.as_deref() {
            Some(path) => (std::fs::read(path)?, transport),
            None => {
                let (model, transport) = match request.model.as_deref() {
                    Some(model) => (model.to_string(), transport),
                    // In updater mode the camera no longer answers the normal
                    // camera-info command, so the model cannot be detected.
                    None if already_updater => {
                        return Err(Error::Operation(l.tr("updater-model-required")));
                    }
                    None => discovery::read_model_name(transport)?,
                };
                sink.emit(ProgressEvent::step(
                    ProgressKind::Info,
                    format!("{}: {model}", l.tr("updater-model")),
                ));
                (crate::fdat::generate(l, &request.fdat_dir, &model)?, transport)
            }
        };

    let mut updater = SonyUpdaterCamera::new(transport);
    updater.init()?;
    updater.check_guard(&payload)?;
    sink.emit(ProgressEvent::step(ProgressKind::Info, l.tr("updater-guard-ok")));
    let versions = updater.get_firmware_version()?;
    sink.emit(ProgressEvent::step(
        ProgressKind::Info,
        format!("{}: {}", l.tr("updater-firmware-current"), versions.old),
    ));

    Ok((payload, updater, already_updater))
}

/// A message from the session thread to whoever is driving it.
enum Reply {
    /// Progress from the flow that gets the payload running.
    Progress(ProgressEvent),
    /// The payload is live and the shell is accepting commands.
    Ready,
    /// The result of one command.
    ///
    /// Boxed because the success value is far smaller than the error, and a
    /// channel sizes its slots for the largest variant it carries.
    Outcome(Box<Result<UpdaterOutcome>>),
}

/// A live updater shell, held across the commands a front-end sends it.
///
/// The camera work happens on the thread this owns; nothing but parsed requests
/// and finished outcomes crosses the boundary.
pub struct UpdaterSession {
    /// Dropped by [`Self::close`], which is what tells the thread to wind down.
    commands: Option<Sender<UpdaterRequest>>,
    replies: Receiver<Reply>,
    thread: Option<JoinHandle<Result<()>>>,
}

impl UpdaterSession {
    /// Inject the payload and wait for the shell to come up.
    ///
    /// Blocks until the shell answers, forwarding the progress of the reboot
    /// and the write to `sink` meanwhile — that stretch takes long enough that
    /// a front-end showing nothing would look stalled.
    pub fn open(l: &Localizer, request: &OpenRequest, sink: &mut dyn ProgressSink) -> Result<Self> {
        let (command_tx, command_rx) = channel();
        let (reply_tx, reply_rx) = channel();
        // The thread needs a localizer of its own: Fluent's bundles are `Rc`
        // based and cannot be shared across threads, but the language they were
        // negotiated to is just a string.
        let locale = l.current().to_string();
        let request = request.clone();
        let thread =
            std::thread::spawn(move || run_session(&locale, &request, &reply_tx, &command_rx));

        loop {
            match reply_rx.recv() {
                Ok(Reply::Progress(event)) => sink.emit(event),
                Ok(Reply::Ready) => {
                    return Ok(Self {
                        commands: Some(command_tx),
                        replies: reply_rx,
                        thread: Some(thread),
                    });
                }
                // The shell never answers a command before announcing itself.
                Ok(Reply::Outcome(_)) => {}
                // The thread ended before the shell came up, so its own error
                // is the reason. A panic is the one case with no error to
                // report and is surfaced as such rather than as a hang.
                Err(_) => {
                    return Err(match thread.join() {
                        Ok(Ok(())) => Error::Operation(l.tr("updater-flash-required")),
                        Ok(Err(e)) => e,
                        Err(_) => Error::Operation(l.tr("updater-session-panic")),
                    });
                }
            }
        }
    }

    /// Parse and carry out one line of input.
    ///
    /// Parsing happens on the caller's thread, so a malformed line is reported
    /// without the camera being involved at all.
    pub fn run_line(&mut self, l: &Localizer, line: &str) -> Result<Option<UpdaterOutcome>> {
        let Some(request) = parse(line, l)? else {
            return Ok(None);
        };
        let commands = self
            .commands
            .as_ref()
            .ok_or_else(|| Error::Operation(l.tr("service-not-connected")))?;
        commands.send(request).map_err(|_| Error::Operation(l.tr("service-not-connected")))?;

        loop {
            match self.replies.recv() {
                Ok(Reply::Outcome(result)) => {
                    let outcome = (*result)?;
                    if outcome.finished {
                        self.close();
                    }
                    return Ok(Some(outcome));
                }
                Ok(_) => {}
                Err(_) => return Err(Error::Operation(l.tr("service-not-connected"))),
            }
        }
    }

    /// End the session and wait for the camera to be released.
    ///
    /// Dropping the command channel is the signal; the thread then stops the
    /// shell, finishes the firmware write it is still inside, and returns. The
    /// wait is deliberate — returning while the camera is still mid-write would
    /// let the next operation open a device that is not there yet.
    ///
    /// Safe to call more than once, because the paths that have to call it —
    /// `exit`, navigating away, and the window closing — cannot all know
    /// whether one of the others got there first.
    pub fn close(&mut self) {
        drop(self.commands.take());
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

impl Drop for UpdaterSession {
    fn drop(&mut self) {
        self.close();
    }
}

/// The whole session, from a camera in normal mode to a released one.
///
/// Runs on the session thread. The command loop sits inside the firmware
/// write's completion callback because that is the only place the payload is
/// reachable: it starts answering once the final chunk is on the wire, and the
/// write finalises only after the callback returns.
fn run_session(
    locale: &str,
    request: &OpenRequest,
    replies: &Sender<Reply>,
    commands: &Receiver<UpdaterRequest>,
) -> Result<()> {
    let l = Localizer::new(locale);
    let mut sink = |event: ProgressEvent| {
        let _ = replies.send(Reply::Progress(event));
    };

    let (payload, updater, already_updater) = prepare(&l, request, &mut sink)?;
    let Some(mut updater) = crate::firmware::enter_updater_mode(
        &l,
        updater,
        &payload,
        already_updater,
        request.flash,
        &mut sink,
    )?
    else {
        // `enter_updater_mode` reports the refusal through the sink and returns
        // nothing; the caller is waiting for a shell that will never come up,
        // so it has to be an error here.
        return Err(Error::Operation(l.tr("updater-flash-required")));
    };

    sink.emit(ProgressEvent::step(ProgressKind::Progress, l.tr("updater-flashing")));
    updater.write_firmware(
        &payload,
        Some(&mut |camera| {
            let mut shell = PlatformShell::new(camera.transport_mut());
            shell.start()?;
            let _ = replies.send(Reply::Ready);
            // A disconnected command channel means the front-end has closed the
            // session, which is the ordinary way out as well as the way out
            // after a crash on the other side.
            while let Ok(request) = commands.recv() {
                let outcome = execute(&mut shell, &l, &request);
                let finished = matches!(&outcome, Ok(o) if o.finished);
                if replies.send(Reply::Outcome(Box::new(outcome))).is_err() || finished {
                    break;
                }
            }
            shell.stop()?;
            Ok(())
        }),
    )?;
    updater.complete()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn l() -> Localizer {
        Localizer::new("en-US")
    }

    /// The two shells share a tokenizer, so a quoted path works in both.
    #[test]
    fn quoted_arguments_are_kept_whole() {
        assert_eq!(
            parse(r#"push "C:\My Files\a.bin" /dev/nflasha1"#, &l()).unwrap().unwrap(),
            UpdaterRequest::Push {
                local: PathBuf::from(r"C:\My Files\a.bin"),
                remote: "/dev/nflasha1".to_string(),
            }
        );
    }

    /// `pull` with one argument keeps the camera's file name, not its path.
    #[test]
    fn pull_defaults_to_the_remote_base_name() {
        assert_eq!(
            parse("pull /setting/x", &l()).unwrap().unwrap(),
            UpdaterRequest::Pull { remote: "/setting/x".to_string(), local: PathBuf::from("x") }
        );
    }

    /// `bk` here is not `bk` in the service shell, and the difference is the
    /// point of parsing them separately.
    #[test]
    fn the_backup_subcommands_are_this_shells_own() {
        assert_eq!(parse("bk s", &l()).unwrap().unwrap(), UpdaterRequest::BackupSync);
        assert_eq!(
            parse("bk lock", &l()).unwrap().unwrap(),
            UpdaterRequest::BackupProtect { lock: true }
        );
        assert_eq!(
            parse("bk unlock", &l()).unwrap().unwrap(),
            UpdaterRequest::BackupProtect { lock: false }
        );
        // The service shell's `bk s <subsystem>` and `bk d <file>` do not exist
        // here, and must be refused rather than half-understood.
        assert!(parse("bk s 0", &l()).is_err());
        assert!(parse("bk d out.bin", &l()).is_err());
    }

    /// Every command that changes the camera has to say so.
    #[test]
    fn the_writing_commands_are_the_ones_that_write() {
        let writes = |line: &str| parse(line, &l()).unwrap().unwrap().writes();
        assert!(writes("bk w 1 ff"));
        assert!(writes("bk s"));
        assert!(writes("bk lock"));
        assert!(writes("push a /b"));
        // A root shell on the camera is not a read-only operation.
        assert!(writes("shell"));
        assert!(!writes("info"));
        assert!(!writes("bk r 1"));
        assert!(!writes("pull /b"));
        assert!(!writes("rmem 0 10 out.bin"));
    }

    /// The tunnel is a command both front-ends know, even though one of them
    /// has to hand it back.
    #[test]
    fn the_shell_tunnel_parses_in_both_forms() {
        assert_eq!(parse("shell", &l()).unwrap().unwrap(), UpdaterRequest::Shell { tcp: None });
        assert_eq!(
            parse("shell tcp 2222", &l()).unwrap().unwrap(),
            UpdaterRequest::Shell { tcp: Some(2222) }
        );
        assert!(parse("shell tcp", &l()).is_err());
        assert!(parse("shell tcp nope", &l()).is_err());
    }

    /// A malformed line has to be reported, not guessed at.
    #[test]
    fn wrong_argument_counts_are_refused() {
        assert!(parse("push only-one", &l()).is_err());
        assert!(parse("pull", &l()).is_err());
        assert!(parse("rmem 0 10", &l()).is_err());
        assert!(parse("bk w 1", &l()).is_err());
        assert!(parse("frobnicate", &l()).is_err());
    }

    /// A parse failure must be readable, not the bare key.
    #[test]
    fn parse_failures_are_translated() {
        let l = l();
        for line in ["pull", "frobnicate", "bk w 1 zz", "shell tcp nope"] {
            let message = parse(line, &l).unwrap_err().to_string();
            assert!(!message.contains("service-"), "{line} produced {message}");
            assert!(!message.contains("updater-"), "{line} produced {message}");
        }
    }

    /// Every key this module can produce has to exist in the catalogue, or the
    /// window shows an identifier where a sentence belongs.
    #[test]
    fn every_message_key_is_translated() {
        let l = l();
        for key in [
            "updater-backup-synced",
            "updater-backup-locked",
            "updater-backup-writable",
            "updater-shell-console-only",
            "updater-shell-done",
            "updater-session-panic",
            "updater-flash-required",
            "updater-model-required",
        ] {
            assert_ne!(l.tr(key), key, "{key} is missing from the catalogue");
        }
    }
}
