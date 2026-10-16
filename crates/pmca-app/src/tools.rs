//! The file utilities: everything the toolkit can do without a camera.
//!
//! Converting between APK and SPK containers, dumping a `Backup.bin` or a
//! downloaded `usagelog.bin`, locating the payload inside a firmware `.dat` and
//! checksumming an XPD file are all
//! pure file operations. They lived in `pmca-console`'s `main` for as long as
//! the console was the only front-end that offered them, which made them
//! unavailable to the window and made "the CLI can do things the GUI cannot" a
//! statement about where the code happened to sit rather than about the tool.
//!
//! They are gathered here for the same reason the two shells are: one
//! implementation, one wording, one set of failure messages, whichever
//! front-end the user reaches for.
//!
//! Every operation reads a file and produces text; the two converting ones also
//! write a file. Nothing here touches USB, so nothing here can damage a camera.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use pmca_core::{backup, firmware, photo, spk, usagelog, xpd};
use pmca_i18n::Localizer;

use crate::dto::PhotoSummary;
use crate::service::hexdump;
use crate::{Error, Result};

/// One file operation, with the paths it works on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToolRequest {
    /// Wrap an `.apk` in the SPK container the cameras expect.
    ApkToSpk {
        /// The `.apk` to read.
        input: PathBuf,
        /// The `.spk` to write.
        output: PathBuf,
    },
    /// Unwrap an `.spk` back into a plain `.apk`.
    SpkToApk {
        /// The `.spk` to read.
        input: PathBuf,
        /// The `.apk` to write.
        output: PathBuf,
    },
    /// Dump the properties stored in a `Backup.bin` image.
    PrintBackup {
        /// The backup image to read.
        input: PathBuf,
    },
    /// Dump the entries stored in a downloaded `usagelog.bin`.
    PrintUsageLog {
        /// The usage log to read.
        input: PathBuf,
    },
    /// Report where the `FDAT` payload sits inside a firmware `.dat`.
    GuessFirmware {
        /// The firmware container to read.
        input: PathBuf,
    },
    /// Compute the CIC (HMAC-SHA256) checksum of a file.
    XpdChecksum {
        /// The file to read.
        input: PathBuf,
    },
}

impl ToolRequest {
    /// The file this operation writes, if it writes one.
    ///
    /// A front-end uses this to decide whether to ask for a destination at all,
    /// rather than keeping its own list of which operations produce a file.
    #[must_use]
    pub fn output(&self) -> Option<&Path> {
        match self {
            Self::ApkToSpk { output, .. } | Self::SpkToApk { output, .. } => Some(output),
            _ => None,
        }
    }
}

/// What an operation produced.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolOutcome {
    /// One line describing what happened, already translated.
    pub message: String,
    /// Fixed-width output — a property dump — if the operation produced one.
    pub report: Option<String>,
}

/// Carry out one file operation.
///
/// Parse failures from [`pmca_core`] are wrapped with a sentence naming the
/// kind of file that was expected: on its own, "unexpected end of input" does
/// not tell a user which of the two files they picked was the wrong one.
pub fn run(l: &Localizer, request: &ToolRequest) -> Result<ToolOutcome> {
    match request {
        ToolRequest::ApkToSpk { input, output } => {
            let apk = std::fs::read(input)?;
            let packed = spk::dump(&apk).map_err(|e| tool_error(l, "tools-apk-invalid", &e))?;
            std::fs::write(output, packed)?;
            Ok(written(l, output))
        }
        ToolRequest::SpkToApk { input, output } => {
            let data = std::fs::read(input)?;
            let apk = spk::parse(&data).map_err(|e| tool_error(l, "tools-spk-invalid", &e))?;
            std::fs::write(output, apk)?;
            Ok(written(l, output))
        }
        ToolRequest::PrintBackup { input } => {
            let data = std::fs::read(input)?;
            Ok(ToolOutcome { message: l.tr("msg-done"), report: Some(backup_report(l, &data)?) })
        }
        ToolRequest::PrintUsageLog { input } => {
            let data = std::fs::read(input)?;
            let log =
                usagelog::parse(&data).map_err(|e| invalid(l, "tools-usagelog-invalid", &e))?;
            Ok(ToolOutcome { message: l.tr("msg-done"), report: Some(usage_log_report(l, &log)) })
        }
        ToolRequest::GuessFirmware { input } => {
            let data = std::fs::read(input)?;
            let located =
                firmware::read_dat(&data).map_err(|e| invalid(l, "firmware-invalid", &e))?;
            Ok(ToolOutcome {
                message: l.tr_args(
                    "tools-fdat-found",
                    &[
                        ("offset", &format!("0x{:x}", located.offset)),
                        ("size", &format!("0x{:x}", located.size)),
                    ],
                ),
                report: None,
            })
        }
        ToolRequest::XpdChecksum { input } => {
            let data = std::fs::read(input)?;
            let checksum =
                xpd::calculate_checksum(&data).map_err(|e| tool_error(l, "tools-checksum", &e))?;
            Ok(ToolOutcome {
                message: format!("{}: {}", l.tr("tools-checksum"), checksum),
                report: None,
            })
        }
    }
}

/// Read the shutter count and capture details from a photo.
///
/// The camera is not involved: everything comes from the file, which is only
/// read. A Sony image without a usable count is still a success; the reason is
/// carried in [`PhotoSummary::count_unavailable`].
pub fn photo_summary(l: &Localizer, path: &Path) -> Result<PhotoSummary> {
    let data = std::fs::read(path)?;
    let info = photo::read(&data).map_err(|e| invalid(l, "shutter-invalid", &e))?;
    let name = path.file_name().map_or_else(String::new, |n| n.to_string_lossy().into_owned());
    Ok(PhotoSummary::new(name, info))
}

/// Render a parsed usage log as text.
///
/// Three blocks: the header fields the layout justifies naming, a tally of how
/// often each event number occurs, and then every entry in order.
///
/// The tally comes first because it is the part a reader can act on without
/// knowing what any single number means — an event that occurs eleven thousand
/// times and one that occurs twice are obviously different kinds of thing, and
/// that is visible before any entry is read. Nothing here translates an event
/// number into words; see [`pmca_core::usagelog`] for why.
fn usage_log_report(l: &Localizer, log: &usagelog::UsageLog) -> String {
    let mut out = String::new();
    let mut field = |key: &str, value: u32| {
        let _ = writeln!(out, "{}: {value} (0x{value:x})", l.tr(key));
    };
    field("usagelog-size", log.log_size());
    field("usagelog-cursor", log.cursor());
    field("usagelog-spare", log.spare_size());
    let _ = writeln!(out, "{}: {}", l.tr("usagelog-count"), log.entries.len());

    // BTreeMap rather than a hash: the same log has to produce the same report
    // twice, and ordering by event number keeps neighbouring numbers together,
    // which is how related events turn out to be numbered.
    let mut tally: BTreeMap<u16, usize> = BTreeMap::new();
    for entry in &log.entries {
        *tally.entry(entry.event).or_default() += 1;
    }
    let _ = writeln!(out, "\n{}:", l.tr("usagelog-events"));
    for (event, count) in &tally {
        let _ = writeln!(out, "  0x{event:04x}: {count}");
    }

    let _ = writeln!(out, "\n{}:", l.tr("usagelog-entries"));
    for entry in &log.entries {
        let time = entry.time().map_or_else(String::new, |t| format!("  {t}"));
        let _ = writeln!(
            out,
            "  0x{:08x}  id=0x{:04x}  src=0x{:04x}  0x{:08x}{time}",
            entry.offset, entry.event, entry.source, entry.value
        );
    }
    out
}

/// Render every property of a `Backup.bin` image as text.
///
/// Shared by `print-backup` and the service shell's `bk dump-txt`, so a dump
/// taken from the camera reads exactly like one printed from a file later.
/// Attribute bits and the names of known settings are spelled out, as in the
/// LAK132 fork's text dump.
pub(crate) fn backup_report(l: &Localizer, data: &[u8]) -> Result<String> {
    let image =
        backup::BackupFile::new(data).map_err(|e| invalid(l, "tools-backup-invalid", &e))?;
    let mut report = String::new();
    let _ = writeln!(report, "{}: {}", l.tr("info-region"), image.region());
    for (id, property) in
        image.list_properties().map_err(|e| invalid(l, "tools-backup-invalid", &e))?
    {
        let _ = write!(
            report,
            "id=0x{:08x}, size=0x{:04x}, attr=0x{:02x}",
            id,
            property.data.len(),
            property.attr
        );
        let names = backup::attr::names(property.attr);
        if !names.is_empty() {
            let _ = write!(report, " ({})", names.join("|"));
        }
        if let Some(def) = pmca_platform::props::PROPERTIES
            .iter()
            .find(|p| p.part_size == property.data.len() && p.ids().any(|i| i == id))
        {
            let _ = write!(report, ", name=\"{}\"", def.name);
        }
        report.push_str(":\n");
        indent(&mut report, &hexdump(&property.data));
        // Only worth showing when it differs: a reset value equal to the
        // current one is noise on every single property.
        if let Some(reset) = &property.reset_data
            && reset != &property.data
        {
            let _ = writeln!(report, "  {}", l.tr("tools-reset-value"));
            indent(&mut report, &hexdump(reset));
        }
    }
    Ok(report)
}

/// Report that a file was written, naming where it went.
fn written(l: &Localizer, path: &Path) -> ToolOutcome {
    ToolOutcome {
        message: l.tr_arg("tools-written", "path", &path.display().to_string()),
        report: None,
    }
}

/// Wrap a parser failure in a sentence naming the file that was expected.
fn invalid(l: &Localizer, key: &str, e: &pmca_core::Error) -> Error {
    Error::Operation(format!("{}: {e}", l.tr(key)))
}

/// Like [`invalid`], but a missing-key failure becomes [`Error::MissingKeys`]
/// so the front-end can send the user to the key import instead.
fn tool_error(l: &Localizer, key: &str, e: &pmca_core::Error) -> Error {
    match e {
        pmca_core::Error::KeyUnavailable { .. } => Error::MissingKeys(l.tr("keys-not-set-up")),
        _ => invalid(l, key, e),
    }
}

/// Append `block` to `out`, indented by two spaces.
fn indent(out: &mut String, block: &str) {
    for line in block.lines() {
        let _ = writeln!(out, "  {line}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn l() -> Localizer {
        Localizer::new("en-US")
    }

    /// The round trip the console has always guaranteed, now through the shared
    /// entry point both front-ends use.
    #[test]
    fn an_apk_survives_the_round_trip_through_spk() {
        let _g = pmca_core::keystore::test_guard();
        pmca_core::keystore::install_test_keys();
        let dir = std::env::temp_dir().join(format!("pmca-tools-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let apk = dir.join("in.apk");
        let spk = dir.join("out.spk");
        let back = dir.join("back.apk");
        // Not a real package: the container does not inspect what it wraps.
        let payload = b"PK\x03\x04 not really an apk, but bytes are bytes";
        std::fs::write(&apk, payload).unwrap();

        run(&l(), &ToolRequest::ApkToSpk { input: apk, output: spk.clone() }).unwrap();
        run(&l(), &ToolRequest::SpkToApk { input: spk, output: back.clone() }).unwrap();

        assert_eq!(std::fs::read(&back).unwrap(), payload);
        std::fs::remove_dir_all(&dir).unwrap();
        pmca_core::keystore::clear();
    }

    /// A front-end has to be able to ask, rather than keep its own list.
    #[test]
    fn only_the_converting_operations_write_a_file() {
        let path = PathBuf::from("x");
        assert!(
            ToolRequest::ApkToSpk { input: path.clone(), output: path.clone() }.output().is_some()
        );
        assert!(
            ToolRequest::SpkToApk { input: path.clone(), output: path.clone() }.output().is_some()
        );
        assert!(ToolRequest::PrintBackup { input: path.clone() }.output().is_none());
        assert!(ToolRequest::PrintUsageLog { input: path.clone() }.output().is_none());
        assert!(ToolRequest::GuessFirmware { input: path.clone() }.output().is_none());
        assert!(ToolRequest::XpdChecksum { input: path }.output().is_none());
    }

    /// A wrong file must say which kind was expected, not only what the parser
    /// tripped over.
    #[test]
    fn a_malformed_file_names_the_expected_format() {
        let dir = std::env::temp_dir().join(format!("pmca-tools-bad-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let file = dir.join("garbage.bin");
        std::fs::write(&file, b"not any of the formats").unwrap();

        for request in [
            ToolRequest::SpkToApk { input: file.clone(), output: dir.join("out") },
            ToolRequest::PrintBackup { input: file.clone() },
            ToolRequest::PrintUsageLog { input: file.clone() },
            ToolRequest::GuessFirmware { input: file.clone() },
        ] {
            let message = run(&l(), &request).unwrap_err().to_string();
            assert!(message.contains(": "), "{request:?} produced {message}");
            assert!(!message.contains("tools-"), "{request:?} produced {message}");
        }
        std::fs::remove_dir_all(&dir).unwrap();
    }

    /// The usage-log dump has to show the header, the tally and the entries,
    /// and has to leave a payload slot without an invented date.
    #[test]
    fn a_usage_log_is_dumped_with_its_tally() {
        let dir = std::env::temp_dir().join(format!("pmca-tools-log-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let file = dir.join("usagelog.bin");

        let mut data = Vec::new();
        for word in [0x0002_0000u32, 0x0003_0001, 64, 24 + 3 * 8, 0, 0] {
            data.extend_from_slice(&word.to_le_bytes());
        }
        for (event, source, value) in [
            (0x005du16, 0x0830u16, 1_527_364_591u32),
            (0x005d, 0x0830, 1_527_364_594),
            (0x4453, 0x5048, 0x0027_5048),
        ] {
            data.extend_from_slice(&event.to_le_bytes());
            data.extend_from_slice(&source.to_le_bytes());
            data.extend_from_slice(&value.to_le_bytes());
        }
        data.resize(24 + 64, 0);
        std::fs::write(&file, &data).unwrap();

        let outcome = run(&l(), &ToolRequest::PrintUsageLog { input: file }).unwrap();
        let report = outcome.report.unwrap();

        assert!(report.contains("48 (0x30)"), "write position missing:\n{report}");
        assert!(report.contains("0x005d: 2"), "tally missing:\n{report}");
        assert!(report.contains("2018-05-26 19:56:31 UTC"), "date missing:\n{report}");
        // The payload slot keeps its bytes and gets no date column.
        assert!(report.contains("src=0x5048  0x00275048\n"), "payload slot wrong:\n{report}");

        std::fs::remove_dir_all(&dir).unwrap();
    }

    /// Every key this module can produce has to exist in the catalogue.
    #[test]
    fn every_message_key_is_translated() {
        let l = l();
        for key in [
            "tools-apk-invalid",
            "tools-spk-invalid",
            "tools-backup-invalid",
            "tools-usagelog-invalid",
            "tools-reset-value",
            "tools-checksum",
            "firmware-invalid",
            "info-region",
            "usagelog-size",
            "usagelog-cursor",
            "usagelog-spare",
            "usagelog-count",
            "usagelog-events",
            "usagelog-entries",
            "msg-done",
        ] {
            assert_ne!(l.tr(key), key, "{key} is missing from the catalogue");
        }
        // A message with a placeholder only renders once its arguments are
        // supplied, so it has to be asked for the way this module asks for it.
        let found = l.tr_args("tools-fdat-found", &[("offset", "0x0"), ("size", "0x0")]);
        assert_ne!(found, "tools-fdat-found", "tools-fdat-found is missing from the catalogue");
        let written = l.tr_arg("tools-written", "path", "out");
        assert_ne!(written, "tools-written", "tools-written is missing from the catalogue");
    }
}
