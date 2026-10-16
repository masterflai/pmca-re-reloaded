//! The interactive service-mode session, shared by the console and the window.
//!
//! Service mode is the most dangerous thing this application can do: it grants
//! raw access to the camera's memory, its filesystem and its calibration data.
//! That is precisely why the command set lives here rather than in either
//! front-end. There is one parser, one dispatcher and one set of safeguards; a
//! command cannot mean one thing when typed into `pmca-console` and something
//! else when typed into the window.
//!
//! # Why a command line and not a form
//!
//! The window could offer a button per operation, and for the rest of the
//! application that is the right shape. Here it is not: the audience for
//! service mode is someone who would otherwise be using the console, the
//! arguments are hexadecimal offsets and device paths rather than choices, and
//! a second dialect of the same commands would be one more thing that can
//! disagree with the first. The window therefore runs the same line-based
//! syntax, and gets file pickers as a convenience on top of it.
//!
//! # Leaving the camera behind
//!
//! A camera left in service mode stalls the *next* session's authentication,
//! so [`ServiceSession::close`] is called on every path out — after a failure,
//! and when the window is closed with a session still open. [`Drop`] is the
//! last line of that defence rather than the first, because a stop request
//! travelling over USB is not something to leave to an unwinding stack.

use std::fmt::Write as _;
use std::path::PathBuf;

use pmca_i18n::Localizer;
use pmca_usb::nusb_backend::NusbBackend;
use pmca_usb::senser::{SENSER_HEADER_SIZE, SenserPacketHeader, WireTrace};
use pmca_usb::senser_camera::SenserCamera;

use crate::{Error, ProgressSink, Result};

/// The command reference, in the syntax the parser accepts.
///
/// Deliberately not translated: these are literal words that have to be typed
/// exactly as they appear, and a translated `pull` would not work. The
/// explanations beside them are short enough to be read as syntax rather than
/// prose, and keeping them next to the words they describe is what makes the
/// listing usable at all.
pub const HELP: &str = "\
hasp                              read the HASP block
pull <remote> [local]             copy a camera file to the computer
push <local> <remote>             copy a local file to the camera (WRITES)
rm <remote>                       delete a camera file (WRITES)
rmem <hexoff> <hexlen> <outfile>  dump device memory
bk r <hexid>                      read a backup entry
bk d <outfile>                    save the whole backup image (rollback copy)
bk dump-txt <file>                save the image as <file>.bin plus a <file>.txt listing
bk stat                           show the backup status block
bk w <hexid> <hex...>             write a backup entry (WRITES)
bk s <hexsubsystem>               persist a subsystem's backup (WRITES)
bk lock | bk unlock               toggle protected backup settings (WRITES)
term on | term off                toggle the debug terminal (WRITES)
help                              show this help
exit                              leave the shell

Diagnostics and repair:
bk trace <outfile>                record the raw backup transfer; may stall the camera
bk rewrite                        write the backup image back unchanged (WRITES)
bk restore-attrs <reference>      reset every entry's attribute from a saved image (WRITES)";

/// One command, already parsed and validated.
///
/// Parsing is separated from execution so a front-end can tell the user that a
/// line is malformed without a camera being involved, and so
/// [`ServiceRequest::writes`] can be consulted *before* anything happens.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServiceRequest {
    /// Read the HASP block.
    Hasp,
    /// Copy a file off the camera.
    Pull {
        /// Path on the camera.
        remote: String,
        /// Destination on this computer.
        local: PathBuf,
    },
    /// Copy a file onto the camera.
    Push {
        /// Source on this computer.
        local: PathBuf,
        /// Destination path on the camera.
        remote: String,
    },
    /// Delete a file on the camera.
    Remove {
        /// Path on the camera.
        remote: String,
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
    /// Read one entry of the backup region.
    BackupRead {
        /// Entry identifier.
        id: u32,
    },
    /// Save the whole backup image, as a rollback copy.
    BackupDump {
        /// Destination on this computer.
        local: PathBuf,
    },
    /// Save the whole backup image together with a readable listing of it.
    BackupDumpText {
        /// Destination; `.bin` and `.txt` are appended, replacing either of
        /// them if it is already there.
        local: PathBuf,
    },
    /// Show the backup status block (`PDT_STAT`).
    BackupStatus,
    /// Diagnostic: record how the camera sends the whole
    /// backup image on the wire. May leave the camera mid-reply.
    BackupTrace {
        /// Destination for the raw bytes.
        local: PathBuf,
    },
    /// Diagnostic: read the image, write it back unchanged
    /// and read it again. Tests the image write path; never persisted.
    BackupRewrite,
    /// Repair: set every property attribute back to the
    /// one in a saved image, e.g. after a write stopped between its two images.
    BackupRestoreAttributes {
        /// The saved image whose attributes are the reference.
        reference: PathBuf,
    },
    /// Overwrite one entry of the backup region.
    BackupWrite {
        /// Entry identifier.
        id: u32,
        /// Replacement contents.
        bytes: Vec<u8>,
    },
    /// Persist a subsystem's backup to flash.
    BackupSave {
        /// Subsystem identifier.
        subsystem: u16,
    },
    /// Set or clear the backup write protection (`ID1`).
    BackupProtect {
        /// Whether protection is switched on.
        lock: bool,
    },
    /// Turn the camera's debug terminal on or off.
    Terminal {
        /// Whether the terminal should be enabled.
        enable: bool,
    },
    /// Show the command reference.
    Help,
    /// Leave the shell.
    Exit,
}

impl ServiceRequest {
    /// Whether carrying this out changes the camera.
    ///
    /// Front-ends use this to mark a command before it runs. The read-only
    /// commands are the ones that can be tried freely; everything else can
    /// leave a camera that no longer works, so the distinction is worth making
    /// visible rather than leaving it to be inferred from the command name.
    #[must_use]
    pub fn writes(&self) -> bool {
        matches!(
            self,
            Self::Push { .. }
                | Self::Remove { .. }
                | Self::BackupWrite { .. }
                | Self::BackupSave { .. }
                | Self::BackupProtect { .. }
                | Self::BackupRewrite
                | Self::BackupRestoreAttributes { .. }
                | Self::Terminal { .. }
        )
    }
}

/// What a command produced.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceOutcome {
    /// One line describing what happened, already translated.
    pub message: String,
    /// Fixed-width output — a hex dump or the command reference — if any.
    pub dump: Option<String>,
    /// Whether the session should end after this command.
    pub finished: bool,
}

impl ServiceOutcome {
    fn message(message: String) -> Self {
        Self { message, dump: None, finished: false }
    }

    fn dump(message: String, dump: String) -> Self {
        Self { message, dump: Some(dump), finished: false }
    }
}

/// Split a command line into arguments, honouring double quotes.
///
/// Quoting exists for one reason: a Windows path routinely contains a space,
/// and both front-ends now hand paths to this parser — the window inserts them
/// from a file picker, where the user never sees the text at all. Splitting on
/// whitespace alone would silently turn one such path into two arguments.
///
/// An unterminated quote is an error rather than a quote that runs to the end
/// of the line, because the alternative is acting on a path the user did not
/// finish typing.
///
/// Single quotes and backslash escapes are deliberately *not* syntax here,
/// which is where this parser departs from the Python original it was ported
/// from. Both omissions are the same decision: on Windows `\` is the path
/// separator, so treating it as an escape would make `"C:\dir\"` swallow its
/// own closing quote — and `'` is common enough in user names (`O'Brien`) that
/// giving it meaning would break unquoted paths that work today. Escaping is
/// worth having in a language; in a line that mostly holds Windows paths it
/// costs more than it returns.
///
/// Shared with [`crate::updater`]: the two shells speak different protocols but
/// the same line syntax, and a quoted path has to survive both.
pub(crate) fn tokenize(line: &str, l: &Localizer) -> Result<Vec<String>> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut quoted = false;
    let mut started = false;

    for c in line.chars() {
        match c {
            '"' => {
                quoted = !quoted;
                started = true;
            }
            c if c.is_whitespace() && !quoted => {
                if started {
                    tokens.push(std::mem::take(&mut current));
                    started = false;
                }
            }
            c => {
                current.push(c);
                started = true;
            }
        }
    }
    if quoted {
        return Err(Error::Operation(l.tr("service-unterminated-quote")));
    }
    if started {
        tokens.push(current);
    }
    Ok(tokens)
}

/// Report a command that was given the wrong arguments.
///
/// The usage line is not translated: it spells out words that have to be typed
/// literally. Only the sentence around it is.
pub(crate) fn usage(l: &Localizer, syntax: &str) -> Error {
    Error::Operation(l.tr_arg("service-usage", "usage", syntax))
}

/// Parse a hexadecimal number, with or without a `0x` prefix.
pub(crate) fn hex(l: &Localizer, s: &str) -> Result<u32> {
    u32::from_str_radix(s.trim_start_matches("0x"), 16)
        .map_err(|_| Error::Operation(l.tr_arg("service-bad-hex", "value", s)))
}

/// Parse a sequence of hexadecimal byte values.
pub(crate) fn hex_bytes(l: &Localizer, tokens: &[String]) -> Result<Vec<u8>> {
    tokens
        .iter()
        .map(|t| {
            u8::from_str_radix(t.trim_start_matches("0x"), 16)
                .map_err(|_| Error::Operation(l.tr_arg("service-bad-hex", "value", t)))
        })
        .collect()
}

/// Turn one line of input into a request.
///
/// An empty line yields `None`, which is not an error: pressing return at a
/// prompt is a normal thing to do.
pub fn parse(line: &str, l: &Localizer) -> Result<Option<ServiceRequest>> {
    let tokens = tokenize(line, l)?;
    let Some((command, rest)) = tokens.split_first() else {
        return Ok(None);
    };
    let request = match (command.as_str(), rest) {
        ("exit" | "quit", _) => ServiceRequest::Exit,
        ("help" | "?", _) => ServiceRequest::Help,
        ("hasp", _) => ServiceRequest::Hasp,
        ("pull", [remote]) => {
            ServiceRequest::Pull { remote: remote.clone(), local: PathBuf::from(remote) }
        }
        ("pull", [remote, local]) => {
            ServiceRequest::Pull { remote: remote.clone(), local: PathBuf::from(local) }
        }
        ("pull", _) => return Err(usage(l, "pull <remote> [local]")),
        ("push", [local, remote]) => {
            ServiceRequest::Push { local: PathBuf::from(local), remote: remote.clone() }
        }
        ("push", _) => return Err(usage(l, "push <local> <remote>")),
        ("rm", [remote]) => ServiceRequest::Remove { remote: remote.clone() },
        ("rm", _) => return Err(usage(l, "rm <remote>")),
        ("rmem", [offset, size, local]) => ServiceRequest::ReadMemory {
            offset: hex(l, offset)?,
            size: hex(l, size)?,
            local: PathBuf::from(local),
        },
        ("rmem", _) => return Err(usage(l, "rmem <hexoff> <hexlen> <outfile>")),
        ("bk", [sub, args @ ..]) => backup(l, sub, args)?,
        ("term", [state]) if state == "on" || state == "off" => {
            ServiceRequest::Terminal { enable: state == "on" }
        }
        ("term", _) => return Err(usage(l, "term on | term off")),
        (other, _) => {
            return Err(Error::Operation(l.tr_arg("service-unknown-command", "command", other)));
        }
    };
    Ok(Some(request))
}

/// Parse the `bk` subcommands, which is where most of the syntax lives.
fn backup(l: &Localizer, sub: &str, args: &[String]) -> Result<ServiceRequest> {
    match (sub, args) {
        ("r", [id]) => Ok(ServiceRequest::BackupRead { id: hex(l, id)? }),
        ("d", [local]) => Ok(ServiceRequest::BackupDump { local: PathBuf::from(local) }),
        ("dump-txt", [local]) => Ok(ServiceRequest::BackupDumpText { local: PathBuf::from(local) }),
        ("stat", []) => Ok(ServiceRequest::BackupStatus),
        ("trace", [local]) => Ok(ServiceRequest::BackupTrace { local: PathBuf::from(local) }),
        ("rewrite", []) => Ok(ServiceRequest::BackupRewrite),
        ("restore-attrs", [reference]) => {
            Ok(ServiceRequest::BackupRestoreAttributes { reference: PathBuf::from(reference) })
        }
        ("w", [id, bytes @ ..]) if !bytes.is_empty() => {
            Ok(ServiceRequest::BackupWrite { id: hex(l, id)?, bytes: hex_bytes(l, bytes)? })
        }
        ("s", [subsystem]) => {
            let value = hex(l, subsystem)?;
            let subsystem = u16::try_from(value)
                .map_err(|_| Error::Operation(l.tr_arg("service-bad-hex", "value", subsystem)))?;
            Ok(ServiceRequest::BackupSave { subsystem })
        }
        ("lock", []) => Ok(ServiceRequest::BackupProtect { lock: true }),
        ("unlock", []) => Ok(ServiceRequest::BackupProtect { lock: false }),
        _ => Err(usage(
            l,
            "bk r <hexid> | bk d <outfile> | bk dump-txt <file> | bk stat | bk w <hexid> <hex...> | bk s <hexsub> | bk lock | bk unlock | bk trace <outfile> | bk rewrite | bk restore-attrs <reference>",
        )),
    }
}

/// Render bytes as a classic hex-and-ASCII dump.
///
/// Returned as a string rather than printed so the console and the window show
/// the same bytes laid out the same way.
#[must_use]
pub fn hexdump(data: &[u8]) -> String {
    const WIDTH: usize = 16;
    let mut out = String::new();
    for row in data.chunks(WIDTH) {
        let hex: Vec<String> = row.iter().map(|b| format!("{b:02x}")).collect();
        let ascii: String =
            row.iter().map(|&b| if (0x21..=0x7e).contains(&b) { b as char } else { '.' }).collect();
        let _ = writeln!(out, "{:<width$} {}", hex.join(" "), ascii, width = WIDTH * 3);
    }
    out
}

/// The file name for one half of `bk dump-txt`.
///
/// Only `.bin` and `.txt` count as an extension to replace: a name such as
/// `5.20-backup` would otherwise lose everything after its first dot.
fn dump_path(local: &std::path::Path, ext: &str) -> PathBuf {
    let known = local
        .extension()
        .and_then(|e| e.to_str())
        .is_some_and(|e| e.eq_ignore_ascii_case("bin") || e.eq_ignore_ascii_case("txt"));
    let mut name =
        if known { local.with_extension("") } else { local.to_path_buf() }.into_os_string();
    name.push(".");
    name.push(ext);
    PathBuf::from(name)
}

/// Compare an image written back with what the camera returns afterwards.
///
/// The header is counted apart because the camera regenerates it; differences
/// after it are either running state (counters, the camera's own log) or a
/// write that did not take, which the offsets help tell apart.
fn rewrite_summary(sent: &[u8], read_back: &[u8]) -> String {
    let skip = crate::tweaks::VERIFY_SKIP;
    let differing: Vec<usize> =
        (0..sent.len().min(read_back.len())).filter(|&i| sent[i] != read_back[i]).collect();
    let header = differing.iter().filter(|&&i| i < skip).count();
    let body: Vec<String> =
        differing.iter().filter(|&&i| i >= skip).take(32).map(|i| format!("{i:#x}")).collect();
    let mut out = String::new();
    let _ = writeln!(out, "sent {:#x} bytes, read back {:#x}", sent.len(), read_back.len());
    let _ = writeln!(out, "differing in header (< {skip:#x}): {header}");
    let _ = writeln!(out, "differing after header: {}", differing.len() - header);
    if !body.is_empty() {
        let _ = writeln!(out, "first offsets: {}", body.join(" "));
    }
    out
}

/// Upper bound for `bk trace`, well above any backup image seen so far.
const TRACE_LIMIT: usize = 16 << 20;

/// Describe a wire trace: one line per run of equally long transfers, with
/// header-sized transfers decoded.
fn trace_summary(trace: &WireTrace) -> String {
    let mut out = String::new();
    let t = &trace.transfers;
    let (mut i, mut offset) = (0, 0);
    while i < t.len() {
        let len = t[i];
        if len == SENSER_HEADER_SIZE
            && let Ok(h) = SenserPacketHeader::unpack(&trace.data[offset..offset + len])
        {
            let _ = writeln!(
                out,
                "{offset:#09x}  header size={:#x} pfunc={:#x} seq={} response={}",
                h.size, h.p_func, h.sequence, h.response
            );
            offset += len;
            i += 1;
            continue;
        }
        let run = t[i..].iter().take_while(|&&n| n == len).count();
        let _ = writeln!(out, "{offset:#09x}  {run} x {len:#x}");
        offset += run * len;
        i += run;
    }
    let _ = writeln!(out, "total {offset:#x} bytes in {} transfers", t.len());
    if let Some(end) = &trace.end {
        let _ = writeln!(out, "end: {end}");
    }
    out
}

/// An authenticated, open service-mode connection to the camera.
pub struct ServiceSession {
    cam: SenserCamera<NusbBackend>,
    /// Cleared by [`Self::close`] so [`Drop`] does not send a second request.
    open: bool,
}

impl ServiceSession {
    /// Bring the attached camera into service mode and authenticate.
    ///
    /// This is the same procedure the tweaks view uses, including the switch
    /// from MTP to mass storage: service mode is only reachable from mass
    /// storage, and a camera whose own menu is set to MTP comes back as MTP
    /// after every session. Making the caller do that by hand was a papercut
    /// the console used to have and this does not.
    pub fn open(l: &Localizer, sink: &mut dyn ProgressSink) -> Result<Self> {
        Ok(Self { cam: crate::tweaks::open_service_camera(l, sink)?, open: true })
    }

    /// Carry out one command.
    pub fn execute(&mut self, l: &Localizer, request: &ServiceRequest) -> Result<ServiceOutcome> {
        match request {
            ServiceRequest::Help => {
                Ok(ServiceOutcome::dump(l.tr("service-help"), HELP.to_string()))
            }
            ServiceRequest::Exit => Ok(ServiceOutcome {
                message: l.tr("service-shell-done"),
                dump: None,
                finished: true,
            }),
            ServiceRequest::Hasp => {
                let data = self.cam.read_hasp()?;
                Ok(ServiceOutcome::dump(
                    l.tr_arg("service-read-bytes", "count", &data.len().to_string()),
                    hexdump(&data),
                ))
            }
            ServiceRequest::Pull { remote, local } => {
                let mut file = std::fs::File::create(local)?;
                self.cam.read_file(remote, &mut file)?;
                Ok(ServiceOutcome::message(l.tr_args(
                    "service-pulled",
                    &[("remote", remote), ("local", &local.display().to_string())],
                )))
            }
            ServiceRequest::Push { local, remote } => {
                let data = std::fs::read(local)?;
                self.cam.write_file(remote, &data)?;
                Ok(ServiceOutcome::message(l.tr_args(
                    "service-pushed",
                    &[("local", &local.display().to_string()), ("remote", remote)],
                )))
            }
            ServiceRequest::Remove { remote } => {
                self.cam.delete_file(remote)?;
                Ok(ServiceOutcome::message(l.tr_arg("service-removed", "remote", remote)))
            }
            ServiceRequest::ReadMemory { offset, size, local } => {
                let mut file = std::fs::File::create(local)?;
                self.cam.read_memory(*offset, *size, &mut file)?;
                Ok(ServiceOutcome::message(l.tr_args(
                    "service-dumped",
                    &[
                        ("size", &format!("0x{size:x}")),
                        ("offset", &format!("0x{offset:x}")),
                        ("local", &local.display().to_string()),
                    ],
                )))
            }
            ServiceRequest::BackupRead { id } => {
                let data = self.cam.read_backup(*id)?;
                Ok(ServiceOutcome::dump(
                    l.tr_arg("service-read-bytes", "count", &data.len().to_string()),
                    hexdump(&data),
                ))
            }
            ServiceRequest::BackupDump { local } => {
                // The whole image, as `print-backup` consumes it. The hardware
                // test plan requires this copy before any tweak is written.
                let image = self.cam.get_backup_preset_data(true)?;
                std::fs::write(local, &image)?;
                Ok(ServiceOutcome::message(l.tr_args(
                    "service-backup-saved",
                    &[("count", &image.len().to_string()), ("local", &local.display().to_string())],
                )))
            }
            ServiceRequest::BackupDumpText { local } => {
                let image = self.cam.get_backup_preset_data(true)?;
                let bin = dump_path(local, "bin");
                let txt = dump_path(local, "txt");
                // The image first: it is the rollback copy, and it must exist
                // even if the camera sent something the listing cannot parse.
                std::fs::write(&bin, &image)?;
                std::fs::write(&txt, crate::tools::backup_report(l, &image)?)?;
                Ok(ServiceOutcome::message(l.tr_args(
                    "service-backup-text-saved",
                    &[
                        ("count", &image.len().to_string()),
                        ("bin", &bin.display().to_string()),
                        ("txt", &txt.display().to_string()),
                    ],
                )))
            }
            ServiceRequest::BackupStatus => {
                let status = self.cam.get_backup_preset_data_status()?;
                // The reference reads the region from offset 0x14 of this
                // block; what precedes it is undocumented, so it is shown raw.
                let region =
                    status.get(0x14..).map(pmca_core::util::latin1_trim_nul).unwrap_or_default();
                Ok(ServiceOutcome::dump(
                    l.tr_arg("service-backup-status", "region", &region),
                    hexdump(&status),
                ))
            }
            ServiceRequest::BackupTrace { local } => {
                let trace = self.cam.trace_backup_preset_data(TRACE_LIMIT)?;
                std::fs::write(local, &trace.data)?;
                Ok(ServiceOutcome::dump(
                    l.tr_arg("service-read-bytes", "count", &trace.data.len().to_string()),
                    trace_summary(&trace),
                ))
            }
            ServiceRequest::BackupRewrite => {
                let image = self.cam.get_backup_preset_data(true)?;
                self.cam.set_backup_preset_data(crate::tweaks::BACKUP_PROTECT_MODE, &image)?;
                let read_back = self.cam.get_backup_preset_data(true)?;
                Ok(ServiceOutcome::dump(
                    l.tr_arg("service-backup-rewritten", "count", &image.len().to_string()),
                    rewrite_summary(&image, &read_back),
                ))
            }
            ServiceRequest::BackupRestoreAttributes { reference } => {
                let reference_image = std::fs::read(reference)?;
                let changed = crate::tweaks::restore_attributes(&mut self.cam, &reference_image)?;
                // Read back: nothing may differ from the reference any more.
                let now = self.cam.get_backup_preset_data(true)?;
                let (_, left) = pmca_platform::restore_attributes(&now, &reference_image)
                    .map_err(|e| Error::Operation(e.to_string()))?;
                if !left.is_empty() {
                    return Err(Error::Operation(l.tr("tweaks-write-failed")));
                }
                let mut dump = String::new();
                for (id, was, restored) in &changed {
                    let _ = writeln!(dump, "{id:#010x}  attr {was:#04x} -> {restored:#04x}");
                }
                Ok(ServiceOutcome::dump(
                    l.tr_args(
                        "service-backup-attrs-restored",
                        &[
                            ("count", &changed.len().to_string()),
                            ("local", &reference.display().to_string()),
                        ],
                    ),
                    dump,
                ))
            }
            ServiceRequest::BackupWrite { id, bytes } => {
                self.cam.write_backup(*id, bytes)?;
                Ok(ServiceOutcome::message(l.tr_arg(
                    "service-backup-written",
                    "id",
                    &format!("0x{id:x}"),
                )))
            }
            ServiceRequest::BackupSave { subsystem } => {
                self.cam.save_backup(*subsystem)?;
                Ok(ServiceOutcome::message(l.tr_arg(
                    "service-backup-persisted",
                    "id",
                    &format!("0x{subsystem:x}"),
                )))
            }
            ServiceRequest::BackupProtect { lock } => {
                self.cam.set_backup_id1(*lock)?;
                Ok(ServiceOutcome::message(l.tr(if *lock {
                    "updater-backup-locked"
                } else {
                    "updater-backup-writable"
                })))
            }
            ServiceRequest::Terminal { enable } => {
                self.cam.set_terminal_enable(*enable)?;
                Ok(ServiceOutcome::message(l.tr(if *enable {
                    "service-terminal-on"
                } else {
                    "service-terminal-off"
                })))
            }
        }
    }

    /// Parse and carry out one line of input.
    pub fn run_line(&mut self, l: &Localizer, line: &str) -> Result<Option<ServiceOutcome>> {
        match parse(line, l)? {
            Some(request) => self.execute(l, &request).map(Some),
            None => Ok(None),
        }
    }

    /// Leave service mode and release the driver binding that reached it.
    ///
    /// Safe to call more than once, because the paths that have to call it —
    /// the end of a session, a failure part-way through, and the window being
    /// closed — cannot all know whether one of the others got there first.
    pub fn close(&mut self) {
        if std::mem::take(&mut self.open) {
            crate::tweaks::leave_service_mode(&mut self.cam);
        }
    }
}

impl Drop for ServiceSession {
    fn drop(&mut self) {
        self.close();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn l() -> Localizer {
        Localizer::new("en-US")
    }

    /// A Windows path with a space in it has to survive as one argument.
    #[test]
    fn quoted_arguments_are_kept_whole() {
        let request = parse(r#"push "C:\My Files\a.bin" /setting/x"#, &l()).unwrap().unwrap();
        assert_eq!(
            request,
            ServiceRequest::Push {
                local: PathBuf::from(r"C:\My Files\a.bin"),
                remote: "/setting/x".to_string(),
            }
        );
    }

    /// Acting on half a path is worse than refusing the line.
    #[test]
    fn an_unterminated_quote_is_refused() {
        assert!(parse(r#"pull "/setting/x"#, &l()).is_err());
    }

    /// Pressing return at a prompt is not a mistake.
    #[test]
    fn a_blank_line_is_not_a_command() {
        assert!(parse("   ", &l()).unwrap().is_none());
    }

    /// `pull` with one argument keeps the camera's own file name.
    #[test]
    fn pull_defaults_the_local_name_to_the_remote_one() {
        let request = parse("pull /setting/x", &l()).unwrap().unwrap();
        assert_eq!(
            request,
            ServiceRequest::Pull {
                remote: "/setting/x".to_string(),
                local: PathBuf::from("/setting/x"),
            }
        );
    }

    /// The `0x` prefix is optional, as it is everywhere else in this tool.
    #[test]
    fn hexadecimal_arguments_accept_an_optional_prefix() {
        assert_eq!(
            parse("rmem 0x1000 20 out.bin", &l()).unwrap().unwrap(),
            ServiceRequest::ReadMemory {
                offset: 0x1000,
                size: 0x20,
                local: PathBuf::from("out.bin"),
            }
        );
    }

    /// Every command that changes the camera has to say so.
    #[test]
    fn the_writing_commands_are_the_ones_that_write() {
        let writes = |line: &str| parse(line, &l()).unwrap().unwrap().writes();
        assert!(writes("push a /b"));
        assert!(writes("rm /b"));
        assert!(writes("bk w 1 ff"));
        assert!(writes("bk s 0"));
        assert!(writes("bk lock"));
        assert!(writes("bk unlock"));
        assert!(writes("term on"));
        assert!(!writes("hasp"));
        assert!(!writes("pull /b"));
        assert!(!writes("rmem 0 10 out.bin"));
        assert!(!writes("bk r 1"));
        assert!(!writes("bk d out.bin"));
        assert!(!writes("bk dump-txt out"));
        assert!(!writes("bk stat"));
        assert!(!writes("bk trace out.raw"));
        assert!(writes("bk rewrite"));
        assert!(writes("bk restore-attrs before.bin"));
    }

    #[test]
    fn the_rewrite_summary_separates_header_and_body() {
        let sent = vec![0u8; 0x300];
        let mut back = sent.clone();
        back[0x20] = 1;
        back[0x200] = 1;
        assert_eq!(
            rewrite_summary(&sent, &back),
            "sent 0x300 bytes, read back 0x300\n\
             differing in header (< 0x100): 1\n\
             differing after header: 1\n\
             first offsets: 0x200\n"
        );
    }

    #[test]
    fn the_text_dump_keeps_dotted_names() {
        let p = |s: &str, ext: &str| dump_path(std::path::Path::new(s), ext);
        assert_eq!(p("5.20-backup-cam", "bin"), PathBuf::from("5.20-backup-cam.bin"));
        assert_eq!(p("out", "txt"), PathBuf::from("out.txt"));
        assert_eq!(p("out.bin", "txt"), PathBuf::from("out.txt"));
        assert_eq!(p("out.TXT", "bin"), PathBuf::from("out.bin"));
    }

    #[test]
    fn the_trace_summary_decodes_headers_and_groups_runs() {
        let header = SenserPacketHeader { size: 0x10000, sequence: 3, ..Default::default() };
        let mut data = header.pack().to_vec();
        data.resize(SENSER_HEADER_SIZE + 0x10000, 0);
        let trace = WireTrace {
            transfers: vec![SENSER_HEADER_SIZE, 0x8000, 0x8000, 0],
            data,
            end: Some("timeout".into()),
        };
        assert_eq!(
            trace_summary(&trace),
            "0x0000000  header size=0x10000 pfunc=0x0 seq=3 response=0\n\
             0x000000c  2 x 0x8000\n\
             0x001000c  1 x 0x0\n\
             total 0x1000c bytes in 4 transfers\n\
             end: timeout\n"
        );
    }

    #[test]
    fn the_backup_listing_commands_parse() {
        assert_eq!(parse("bk stat", &l()).unwrap().unwrap(), ServiceRequest::BackupStatus);
        assert_eq!(
            parse("bk dump-txt out", &l()).unwrap().unwrap(),
            ServiceRequest::BackupDumpText { local: PathBuf::from("out") }
        );
        assert!(parse("bk stat x", &l()).is_err());
        assert!(parse("bk dump-txt", &l()).is_err());
        assert_eq!(
            parse("bk unlock", &l()).unwrap().unwrap(),
            ServiceRequest::BackupProtect { lock: false }
        );
        assert!(parse("bk lock now", &l()).is_err());
    }

    /// A malformed line has to be reported, not guessed at.
    #[test]
    fn wrong_argument_counts_are_refused() {
        assert!(parse("pull", &l()).is_err());
        assert!(parse("push only-one", &l()).is_err());
        assert!(parse("rmem 0 10", &l()).is_err());
        assert!(parse("bk w 1", &l()).is_err());
        assert!(parse("term maybe", &l()).is_err());
        assert!(parse("frobnicate", &l()).is_err());
    }

    /// A parse failure must be readable, not the bare key.
    #[test]
    fn parse_failures_are_translated() {
        let l = l();
        for line in ["pull", "frobnicate", "bk w 1 zz"] {
            let message = parse(line, &l).unwrap_err().to_string();
            assert!(!message.contains("service-"), "{line} produced {message}");
        }
    }

    /// The dump is the shared rendering, so its shape is worth pinning down.
    #[test]
    fn the_hex_dump_shows_bytes_and_printable_characters() {
        assert_eq!(hexdump(b"AB\0"), "41 42 00                                         AB.\n");
    }

    /// Every key this module can produce has to exist in the catalogue.
    ///
    /// Several of these messages name something, and a message with an unfilled
    /// placeable does not format at all — so every placeable any of them uses is
    /// supplied here. Fluent ignores the ones a given message does not mention,
    /// which lets one list cover all of them.
    #[test]
    fn every_message_is_translated() {
        let l = l();
        let args = [
            ("usage", "u"),
            ("value", "v"),
            ("command", "c"),
            ("count", "1"),
            ("remote", "r"),
            ("local", "p"),
            ("size", "s"),
            ("offset", "o"),
            ("id", "i"),
            ("region", "EU"),
            ("bin", "b"),
            ("txt", "t"),
        ];
        for key in [
            "service-usage",
            "service-bad-hex",
            "service-unknown-command",
            "service-unterminated-quote",
            "service-help",
            "service-read-bytes",
            "service-pulled",
            "service-pushed",
            "service-removed",
            "service-dumped",
            "service-backup-saved",
            "service-backup-text-saved",
            "service-backup-status",
            "service-backup-rewritten",
            "service-backup-attrs-restored",
            "service-backup-written",
            "service-backup-persisted",
            "service-terminal-on",
            "service-terminal-off",
            "updater-backup-locked",
            "updater-backup-writable",
            "service-shell-done",
        ] {
            assert_ne!(l.tr_args(key, &args), key, "{key} is missing from the catalogue");
        }
    }
}
