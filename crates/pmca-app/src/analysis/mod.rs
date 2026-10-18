//! Function analysis: find where a menu setting is stored, and optionally
//! prove it by writing.
//!
//! The procedure is the one from tests 5.28 and 5.29, made repeatable for
//! cameras nobody here owns. The user sets a menu value, the program reads the
//! complete backup image in its own short service session, and that repeats for
//! every value and once more for the first one. Which properties followed the
//! menu is decided by [`pmca_platform::analysis`].
//!
//! Reading is the default and the only thing that happens unless the user asks
//! for the write test. That test writes one value the camera itself produced,
//! checks the menu, and writes the original value back. A record of the
//! original value is saved before the write, so a run that is cut short can be
//! undone at the next start ([`restore_pending`]).
//!
//! The report leaves out what identifies a person or a camera: no serial
//! number, no backup image, and no value of any property other than the
//! candidates. Large candidate values are reduced to their size and the
//! offsets that changed.

use std::path::PathBuf;

use pmca_core::backup::{BackupFile, attr};
use pmca_i18n::Localizer;
use pmca_platform::analysis::{self as find, Candidate, MAX_WRITE_SIZE, Refusal, Snapshot};
use pmca_platform::direct::DirectWrite;
use pmca_platform::{BackupPatch, Settings};
use pmca_usb::nusb_backend::NusbBackend;
use pmca_usb::senser_camera::SenserCamera;
use serde::Serialize;

use crate::direct;
use crate::dto::{ProgressEvent, ProgressKind};
use crate::service_mode::{leave_service_mode, open_service_camera};
use crate::{Error, ProgressSink, Result};

/// Identifies the report format; bumped when a field changes meaning.
pub const REPORT_SCHEMA: &str = "pmca-analysis/1";

/// The most menu values one run accepts. Every value costs a service session.
pub const MAX_VALUES: usize = 8;

/// Appended to the camera key for the write test's rollback record, so it
/// never mixes with the tweak view's own record.
const ROLLBACK_SUFFIX: &str = "-analysis";

/// What a step does to the camera.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub enum Access {
    /// Reads from the camera, changes nothing in its settings.
    Read,
    /// Changes the camera's stored settings.
    Write,
    /// Done by the user on the camera itself.
    User,
    /// Happens on this computer only.
    Local,
}

/// The kind of a step in the plan.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub enum StepKind {
    /// Read the complete backup image.
    Snapshot,
    /// Set the menu to the next value.
    Change,
    /// Compare the images.
    Evaluate,
    /// Write the value of the second menu value.
    Write,
    /// Switch the camera off and on and read the menu.
    CheckMenu,
    /// Write the original value back.
    Restore,
    /// Save the report.
    Report,
    /// Read the candidates from another camera.
    CrossCheck,
}

/// One line of the plan shown before and during a run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct PlanStep {
    /// What the step does to the camera.
    pub access: Access,
    /// What the step is.
    pub kind: StepKind,
    /// Localized description.
    pub text: String,
    /// Whether the step has been carried out.
    pub done: bool,
}

/// What one snapshot read.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct SnapshotInfo {
    /// 1-based number of the snapshot.
    pub index: usize,
    /// How many snapshots the run takes.
    pub total: usize,
    /// The menu value it was taken at.
    pub label: String,
    /// Properties in the image.
    pub properties: usize,
}

/// One candidate value at one step.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct StepValue {
    /// The menu value at that step.
    pub label: String,
    /// The value as hex, for values up to [`MAX_WRITE_SIZE`] bytes.
    pub hex: Option<String>,
    /// Byte offsets that differ from the first snapshot.
    pub changed_offsets: Vec<usize>,
}

/// A candidate as front-ends and the report show it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct CandidateView {
    /// Property id, `0x` and eight hex digits.
    pub id: String,
    /// Value size in bytes.
    pub size: usize,
    /// Attribute bits, `0x` and two hex digits.
    pub attr: String,
    /// Names of the attribute bits.
    pub attr_names: Vec<String>,
    /// Reset value as hex, for small values that have one.
    pub reset: Option<String>,
    /// Whether menu values and property values correspond one to one.
    pub consistent: bool,
    /// The value at every snapshot.
    pub values: Vec<StepValue>,
}

/// What the write test would write.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct WritePreview {
    /// Property id.
    pub id: String,
    /// Value size in bytes.
    pub size: usize,
    /// Attribute bits.
    pub attr: String,
    /// The menu value that is current now.
    pub old_label: String,
    /// The value stored now.
    pub old_hex: String,
    /// The menu value the write should produce.
    pub new_label: String,
    /// The value that will be written: the one the camera stored itself.
    pub new_hex: String,
}

/// The result of comparing the snapshots.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct EvaluationView {
    /// Properties compared.
    pub compared: usize,
    /// Properties that followed the menu.
    pub candidates: Vec<CandidateView>,
    /// Properties that changed on their own.
    pub running: usize,
    /// The small ones among them, with their values: the setting itself shows
    /// up here when the first snapshot was not taken at the first menu value.
    pub changed: Vec<CandidateView>,
    /// What the write test would write, if it can be offered.
    pub write: Option<WritePreview>,
    /// Why it cannot, localized.
    pub write_refusal: Option<String>,
}

/// The facts about the camera that go into the report.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct CameraFacts {
    /// Model name from the backup, e.g. `ILCE-5100`.
    pub model_name: Option<String>,
    /// Model code from the backup.
    pub model_code: Option<String>,
    /// Region string from the backup header.
    pub region: String,
    /// Android platform version, where the camera has one.
    pub platform_version: Option<String>,
    /// Firmware version as reported over USB, when it could be read.
    pub firmware: Option<String>,
    /// Backup image size in bytes.
    pub backup_size: usize,
    /// Backup format revision.
    pub backup_revision: u8,
}

/// One step that was carried out.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct StepRecord {
    /// What the step did to the camera.
    pub access: Access,
    /// What the step was.
    pub kind: StepKind,
    /// The menu value involved.
    pub value: Option<String>,
    /// When, UTC.
    pub at: String,
}

/// The write test as it went.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct WriteTestReport {
    /// Property id.
    pub id: String,
    /// The menu value whose stored value was written.
    pub written_label: String,
    /// The value written.
    pub written_hex: String,
    /// The original value.
    pub original_hex: String,
    /// The camera kept the value when read back.
    pub verified: bool,
    /// What the menu showed after the write and a restart, as the user said.
    pub menu_after_write: Option<String>,
    /// The value was still there in the next session.
    pub survived_restart: Option<bool>,
    /// The original value was written back.
    pub restored: bool,
    /// What the menu showed after the restore and a restart.
    pub menu_after_restore: Option<String>,
}

/// The file a user can forward.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct AnalysisReport {
    /// [`REPORT_SCHEMA`].
    pub schema: String,
    /// Program name and version.
    pub tool: String,
    /// When the run started, UTC.
    pub started: String,
    /// The camera, without anything that identifies the unit.
    pub camera: CameraFacts,
    /// The menu setting, as the user named it.
    pub setting: String,
    /// The menu values, first one current at the start.
    pub values: Vec<String>,
    /// Every snapshot was taken.
    pub complete: bool,
    /// Steps carried out.
    pub steps: Vec<StepRecord>,
    /// Properties compared.
    pub compared: usize,
    /// Properties that followed the menu.
    pub candidates: Vec<CandidateView>,
    /// Ids of properties that changed without returning.
    pub running: Vec<String>,
    /// Those of them up to [`MAX_WRITE_SIZE`] bytes, with their values.
    pub running_values: Vec<CandidateView>,
    /// When the user agreed to the write test, if they did.
    pub write_test_consent: Option<String>,
    /// Why the write test was not offered, as a stable code.
    pub write_test_refusal: Option<String>,
    /// The write test, if it ran.
    pub write_test: Option<WriteTestReport>,
    /// The candidates as read from other cameras.
    pub cross_checks: Vec<CrossCheckView>,
    /// What the report leaves out.
    pub privacy: String,
}

/// Another camera, without anything that identifies the unit.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct CrossCamera {
    /// Model name, e.g. `ILCE-5100`.
    pub model_name: Option<String>,
    /// Model code.
    pub model_code: Option<String>,
    /// Region string from the backup status.
    pub region: String,
    /// Android platform version, where the camera has one.
    pub platform_version: Option<String>,
    /// Firmware version as reported over USB, when it could be read.
    pub firmware: Option<String>,
}

/// One candidate as found on another camera.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct CrossValue {
    /// Property id.
    pub id: String,
    /// The camera has the property at all.
    pub present: bool,
    /// Its size there.
    pub size: Option<usize>,
    /// Its value there, for values up to [`MAX_WRITE_SIZE`] bytes.
    pub hex: Option<String>,
    /// The menu value whose stored value it equals, if any.
    pub matches: Option<String>,
}

/// The candidates read from one other camera.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct CrossCheckView {
    /// When, UTC.
    pub at: String,
    /// The camera.
    pub camera: CrossCamera,
    /// Every candidate.
    pub values: Vec<CrossValue>,
}

/// Where a run stands, for a front-end.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct RunState {
    /// The menu setting.
    pub setting: String,
    /// The menu values.
    pub values: Vec<String>,
    /// Every step, with what it does to the camera.
    pub plan: Vec<PlanStep>,
    /// The menu value the next snapshot expects.
    pub next_label: Option<String>,
    /// Snapshots taken.
    pub taken: usize,
    /// Snapshots the run takes.
    pub total: usize,
    /// The comparison, once every snapshot is taken.
    pub evaluation: Option<EvaluationView>,
    /// The user agreed to the write test.
    pub write_requested: bool,
    /// The write test wrote its value.
    pub written: bool,
    /// What the menu showed after the write.
    pub menu_after_write: Option<String>,
    /// The original value was written back.
    pub restored: bool,
    /// The written value survived the restart.
    pub survived_restart: Option<bool>,
    /// What the menu showed after the restore.
    pub menu_after_restore: Option<String>,
    /// Where the report was saved.
    pub saved: Option<String>,
    /// Another camera can be checked now.
    pub can_cross_check: bool,
    /// The other cameras checked so far.
    pub cross_checks: Vec<CrossCheckView>,
}

/// The write test's state while it runs.
#[derive(Debug, Clone)]
struct WriteTest {
    id: u32,
    original: Vec<u8>,
    written: Vec<u8>,
    label: String,
    menu_after_write: Option<String>,
    survived_restart: Option<bool>,
    restored: bool,
    menu_after_restore: Option<String>,
}

/// One analysis, from the first snapshot to the report.
pub struct AnalysisRun {
    setting: String,
    values: Vec<String>,
    write_consent: Option<String>,
    started: String,
    snapshots: Vec<Snapshot>,
    /// The comparison of `snapshots`, computed on first use after a snapshot
    /// was added. The window asks for the state after every step, and each
    /// state needs the evaluation more than once.
    evaluation: std::cell::OnceCell<find::Evaluation>,
    camera: Option<CameraFacts>,
    key: Option<String>,
    steps: Vec<StepRecord>,
    write: Option<WriteTest>,
    cross_checks: Vec<CrossCheckView>,
    saved: Option<PathBuf>,
}

mod cross_check;
mod report;
mod run;

use cross_check::*;
use report::*;
pub use report::{hex, report_dir};
use run::*;
pub use run::{has_pending_restore, restore_pending};

#[cfg(test)]
mod tests {
    use super::*;

    /// A minimal backup image holding the given properties.
    fn image(entries: &[(u32, u16, &[u8])]) -> Vec<u8> {
        let num_subsystems: usize =
            entries.iter().map(|(id, ..)| (id >> 16) as usize + 1).max().unwrap_or(1);
        let mut counts = vec![0u16; num_subsystems];
        for (id, ..) in entries {
            let sub = (id >> 16) as usize;
            counts[sub] = counts[sub].max((id & 0xffff) as u16 + 1);
        }
        let num_properties: usize = counts.iter().map(|&n| n as usize).sum();
        let data_size: usize = entries.iter().map(|(.., v)| v.len()).sum::<usize>().max(1);
        let table = 0x100 + num_subsystems * 6;
        let data_offset = (table + num_properties * 6).next_multiple_of(0x1000);
        let mut buf = vec![0u8; data_offset + data_size];
        buf[12..16].copy_from_slice(b"BK4\0");
        buf[16..20].copy_from_slice(&(num_subsystems as u32).to_le_bytes());
        buf[20..24].copy_from_slice(&(num_properties as u32).to_le_bytes());
        buf[24..28].copy_from_slice(&(data_offset as u32).to_le_bytes());
        buf[28..32].copy_from_slice(&(data_size as u32).to_le_bytes());
        buf[0xc0..0xc6].copy_from_slice(b"EU_CE3");
        let mut starts = Vec::with_capacity(num_subsystems);
        let mut next = 0usize;
        for (sub, &n) in counts.iter().enumerate() {
            starts.push(next);
            let off = 0x100 + sub * 6;
            buf[off..off + 2].copy_from_slice(&n.to_le_bytes());
            buf[off + 2..off + 6].copy_from_slice(&(next as u32).to_le_bytes());
            next += n as usize;
        }
        for i in 0..num_properties {
            let e = table + i * 6;
            buf[e + 2..e + 6].copy_from_slice(&u32::MAX.to_le_bytes());
        }
        let mut ptr = data_offset;
        for (id, attr, value) in entries {
            let e = table + (starts[(id >> 16) as usize] + (id & 0xffff) as usize) * 6;
            buf[e..e + 2].copy_from_slice(&attr.to_le_bytes());
            buf[e + 2..e + 6]
                .copy_from_slice(&(((value.len() as u32) << 24) | ptr as u32).to_le_bytes());
            buf[ptr..ptr + value.len()].copy_from_slice(value);
            ptr += value.len();
        }
        let sum: u32 = buf
            .iter()
            .enumerate()
            .filter(|(i, _)| !(0x20..0x24).contains(i))
            .fold(0u32, |acc, (_, &b)| acc.wrapping_add(u32::from(b)));
        buf[0x20..0x24].copy_from_slice(&sum.to_le_bytes());
        buf
    }

    const SETTING: u32 = 0x0002_0001;
    const COUNTER: u32 = 0x0002_0002;
    const BLOB: u32 = 0x0002_0003;

    fn camera(setting: u8, counter: u8, blob: u8) -> Vec<u8> {
        let mut big = [0u8; 32];
        big[12] = blob;
        image(&[(SETTING, 0x40, &[setting]), (COUNTER, 0x00, &[counter, 0]), (BLOB, 0x00, &big)])
    }

    fn run(write: bool) -> AnalysisRun {
        let l = Localizer::new("en-US");
        let values = vec!["Low".to_string(), "Mid".to_string(), "High".to_string()];
        AnalysisRun::new(&l, "Auto Pwr OFF Temp.", &values, write).unwrap()
    }

    #[test]
    fn rejects_unusable_input() {
        let l = Localizer::new("en-US");
        let v = |s: &[&str]| s.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        assert!(AnalysisRun::new(&l, " ", &v(&["a", "b"]), false).is_err());
        assert!(AnalysisRun::new(&l, "x", &v(&["a"]), false).is_err());
        assert!(AnalysisRun::new(&l, "x", &v(&["a", "a"]), false).is_err());
        assert!(AnalysisRun::new(&l, "x", &v(&["a", ""]), false).is_err());
        assert!(AnalysisRun::new(&l, "x", &v(&["a", "b"]), false).is_ok());
    }

    #[test]
    fn a_multi_valued_run_finds_the_setting_and_reports_it_without_other_values() {
        let l = Localizer::new("en-US");
        let mut r = run(false);
        assert_eq!(r.labels(), ["Low", "Mid", "High", "Low"]);
        for (setting, counter, blob) in [(0, 1, 0), (1, 2, 5), (2, 3, 0), (0, 4, 0)] {
            assert!(!r.complete());
            r.add_image(&l, camera(setting, counter, blob), None).unwrap();
        }
        assert!(r.complete());

        let eval = r.evaluate(&l);
        assert_eq!(eval.running, 1);
        // The blob followed only one step and came back: a candidate, but not
        // a consistent one, so no write is offered.
        assert_eq!(eval.candidates.len(), 2);
        assert_eq!(eval.candidates[0].id, "0x00020001");
        assert!(eval.candidates[0].consistent);
        assert!(eval.write.is_none());
        assert!(eval.write_refusal.is_some());

        let report = r.report();
        assert_eq!(report.schema, REPORT_SCHEMA);
        assert_eq!(report.running, ["0x00020002"]);
        assert_eq!(report.running_values.len(), 1);
        assert_eq!(report.running_values[0].values[3].hex.as_deref(), Some("04 00"));
        assert_eq!(report.write_test_refusal.as_deref(), Some("ambiguous"));
        let blob = &report.candidates[1];
        assert_eq!(blob.hex_values(), [None, None, None, None]);
        assert_eq!(blob.values[1].changed_offsets, [12]);
        let json = r.report_json().unwrap();
        assert!(!json.contains("serialNumber"));
        // The plan marks every snapshot done and only reads.
        let plan = r.plan(&l);
        assert!(plan.iter().all(|s| s.access != Access::Write));
        assert!(plan.iter().filter(|s| s.kind == StepKind::Snapshot).all(|s| s.done));
    }

    #[test]
    fn the_plan_shows_the_write_steps_only_with_consent() {
        let l = Localizer::new("en-US");
        let plan = run(true).plan(&l);
        let writes: Vec<_> = plan.iter().filter(|s| s.access == Access::Write).collect();
        assert_eq!(writes.len(), 2);
        assert!(plan.iter().all(|s| !s.done));
        assert!(run(false).plan(&l).iter().all(|s| s.access != Access::Write));
    }

    #[test]
    fn a_two_valued_run_offers_the_write_the_camera_produced() {
        let l = Localizer::new("en-US");
        let values = vec!["Standard".to_string(), "High".to_string()];
        let mut r = AnalysisRun::new(&l, "Auto Pwr OFF Temp.", &values, true).unwrap();
        for (setting, counter) in [(0, 1), (1, 2), (0, 3)] {
            r.add_image(&l, camera(setting, counter, 0), None).unwrap();
        }
        let eval = r.evaluate(&l);
        let preview = eval.write.unwrap();
        assert_eq!(preview.id, "0x00020001");
        assert_eq!((preview.old_hex.as_str(), preview.new_hex.as_str()), ("00", "01"));
        assert_eq!(preview.new_label, "High");
    }

    #[test]
    fn a_different_camera_midway_is_refused() {
        let l = Localizer::new("en-US");
        let mut r = run(false);
        r.add_image(&l, camera(0, 0, 0), None).unwrap();
        let other =
            image(&[(SETTING, 0x40, &[0]), (0x003e_0005, 0x01, b"ILCE-5100\0\0\0\0\0\0\0")]);
        assert!(r.add_image(&l, other, None).is_err());
    }

    #[test]
    fn a_cross_check_names_the_menu_value_and_tells_missing_from_failed() {
        let l = Localizer::new("en-US");
        let mut r = run(false);
        for (setting, counter, blob) in [(0, 1, 0), (1, 2, 5), (2, 3, 0), (0, 4, 0)] {
            r.add_image(&l, camera(setting, counter, blob), None).unwrap();
        }
        let candidates = find::evaluate(&r.snapshots).candidates;
        let values = cross_values(&candidates, &r.labels(), |id| match id {
            SETTING => Ok(vec![2]),
            _ => Err(pmca_usb::Error::Senser("no such property".into())),
        })
        .unwrap();
        assert_eq!(values[0].hex.as_deref(), Some("02"));
        assert_eq!(values[0].matches.as_deref(), Some("High"));
        assert!(!values[1].present);

        let unknown = cross_values(&candidates[..1], &r.labels(), |_| Ok(vec![9])).unwrap();
        assert_eq!(unknown[0].matches, None);
        let unplugged =
            cross_values(&candidates, &r.labels(), |_| Err(pmca_usb::Error::Protocol("gone")));
        assert!(unplugged.is_err());
    }

    #[test]
    fn formats_utc() {
        assert_eq!(format_utc(0), "1970-01-01T00:00:00Z");
        assert_eq!(format_utc(1_791_996_693), "2026-10-14T16:51:33Z");
    }

    impl CandidateView {
        fn hex_values(&self) -> Vec<Option<String>> {
            self.values.iter().map(|v| v.hex.clone()).collect()
        }
    }
}
