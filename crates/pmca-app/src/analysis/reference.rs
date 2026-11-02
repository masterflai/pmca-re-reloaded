//! Checking a known finding against the connected camera.
//!
//! The discovery run ([`super::AnalysisRun`]) finds where a menu setting is
//! stored by cycling the menu on a camera the user owns. This is its mirror for
//! cameras nobody here owns: the user types in, or imports from a shared report,
//! where a setting sits (a property id) and the values it can take, and the
//! program reads just those properties from the connected camera to see whether
//! it stores the same thing in the same place.
//!
//! Reading is the default and the only thing that happens unless the user asks
//! to write one of the values back. That optional write test uses the same
//! consent, verification and rollback as the discovery run's: the original
//! value is saved before the write, so a run cut short can be undone at the next
//! start ([`restore_pending_reference`]).

use serde::{Deserialize, Serialize};

use super::*;

/// Appended to the camera key for the reference write test's rollback record,
/// so it never mixes with the discovery run's or the tweak view's.
const ROLLBACK_SUFFIX: &str = "-reference";

/// The most findings one check accepts, to keep a pasted report in hand.
pub const MAX_ENTRIES: usize = 64;

/// One finding to check: a property and the values it is said to take.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct ReferenceEntry {
    /// Property id, as text: `0x`-prefixed or bare hex, underscores allowed.
    pub id: String,
    /// The setting's name, as the source gave it.
    pub label: String,
    /// The values the setting is said to take.
    pub variants: Vec<ReferenceVariant>,
}

/// One value a finding's setting is said to take.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct ReferenceVariant {
    /// The menu value this stands for.
    pub label: String,
    /// The stored value as hex, spaces and underscores allowed.
    pub hex: String,
}

/// What the connected camera holds at one finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct ReferenceResult {
    /// Property id, `0x` and eight hex digits.
    pub id: String,
    /// The setting's name, echoed from the finding.
    pub label: String,
    /// The camera has the property at all.
    pub present: bool,
    /// Its size on the camera.
    pub size: Option<usize>,
    /// Its value as hex, for values up to [`MAX_WRITE_SIZE`] bytes.
    pub hex: Option<String>,
    /// Attribute bits, `0x` and two hex digits, when a full image was read.
    pub attr: Option<String>,
    /// Names of the attribute bits, when known.
    pub attr_names: Vec<String>,
    /// The write test may be offered for this finding on this camera.
    pub writable: bool,
    /// The menu value whose stored value the camera's value equals, if any.
    pub matches: Option<String>,
}

/// The result of checking every finding against the connected camera.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct ReferenceResultView {
    /// When, UTC.
    pub at: String,
    /// The camera, without anything that identifies the unit.
    pub camera: CrossCamera,
    /// One result per finding, in the order given.
    pub results: Vec<ReferenceResult>,
    /// The backup image size in bytes, as the status block announced it.
    pub backup_size: Option<usize>,
    /// A full backup image could be read; attributes and the write test need it.
    pub full_image: bool,
}

/// The reference write test as it stands, for a front-end.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct ReferenceWriteState {
    /// Property id.
    pub id: String,
    /// The setting's name.
    pub label: String,
    /// The menu value whose stored value was written.
    pub variant_label: String,
    /// The value written, as hex.
    pub written_hex: String,
    /// The original value, as hex.
    pub original_hex: String,
    /// What the menu showed after the write, as the user said.
    pub menu_after_write: Option<String>,
    /// The original value was written back.
    pub restored: bool,
    /// The written value was still there in the next session.
    pub survived_restart: Option<bool>,
    /// What the menu showed after the restore.
    pub menu_after_restore: Option<String>,
}

/// Where a reference check stands, for a front-end.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct ReferenceState {
    /// The findings being checked, echoed as entered.
    pub entries: Vec<ReferenceEntry>,
    /// The last read of the camera, once one has happened.
    pub result: Option<ReferenceResultView>,
    /// The write test, if one is under way.
    pub write: Option<ReferenceWriteState>,
}

/// One finding after parsing and validation.
#[derive(Debug, Clone)]
struct Finding {
    id: u32,
    label: String,
    variants: Vec<(String, Vec<u8>)>,
}

/// The write test's state while it runs.
#[derive(Debug, Clone)]
struct WriteTest {
    id: u32,
    label: String,
    variant_label: String,
    original: Vec<u8>,
    written: Vec<u8>,
    menu_after_write: Option<String>,
    restored: bool,
    survived_restart: Option<bool>,
    menu_after_restore: Option<String>,
}

/// One check of a set of findings against the connected camera.
pub struct ReferenceCheck {
    entries: Vec<ReferenceEntry>,
    findings: Vec<Finding>,
    result: Option<ReferenceResultView>,
    key: Option<String>,
    write: Option<WriteTest>,
}

impl ReferenceCheck {
    /// Start a check from the findings the user entered or imported.
    pub fn new(l: &Localizer, entries: Vec<ReferenceEntry>) -> Result<Self> {
        if entries.is_empty() || entries.len() > MAX_ENTRIES {
            return Err(Error::Operation(l.tr_arg(
                "reference-invalid-count",
                "max",
                &MAX_ENTRIES.to_string(),
            )));
        }
        let findings = entries.iter().map(|e| parse_finding(l, e)).collect::<Result<Vec<_>>>()?;
        Ok(Self { entries, findings, result: None, key: None, write: None })
    }

    /// Read the findings' properties from the connected camera. Only reads.
    pub fn check(&mut self, l: &Localizer, sink: &mut dyn ProgressSink) -> Result<ReferenceState> {
        let firmware = crate::camera_info().ok().and_then(|c| c.firmware);

        let mut cam = Service::open(l, sink)?;
        sink.emit(ProgressEvent::step(ProgressKind::Info, l.tr("tweaks-reading")));
        let identity = identify(&mut cam.0)?;
        self.key = Some(direct::camera_key(&identity));

        let status = cam.0.get_backup_preset_data_status().ok();
        let backup_size = status.as_deref().and_then(status_size);
        let large = status.as_deref().is_some_and(pmca_platform::direct::full_read_too_large);

        let results = if large {
            self.read_direct(&mut cam.0)?
        } else {
            match cam.0.get_backup_preset_data(true) {
                Ok(image) => self.read_image(l, &image)?,
                // A camera that refuses the full read is treated like a large
                // one: fall back to the per-property path rather than fail.
                Err(_) => self.read_direct(&mut cam.0)?,
            }
        };
        drop(cam);

        let ids = pmca_platform::identity::list(&identity);
        let get = |id: &str| ids.iter().find(|v| v.id == id).map(|v| v.value.clone());
        self.result = Some(ReferenceResultView {
            at: timestamp(),
            camera: CrossCamera {
                model_name: get("modelName"),
                model_code: get("modelCode"),
                region: identity.region().to_string(),
                platform_version: get("androidPlatformVersion"),
                firmware,
            },
            results,
            backup_size,
            full_image: !large,
        });
        sink.emit(ProgressEvent::step(ProgressKind::Done, l.tr("analysis-snapshot-done")));
        Ok(self.state())
    }

    /// Read every finding from a full backup image, keeping its attributes.
    fn read_image(&self, l: &Localizer, image: &[u8]) -> Result<Vec<ReferenceResult>> {
        let snapshot = Snapshot::parse("", image).map_err(|e| Error::Operation(e.to_string()))?;
        let _ = l;
        Ok(self
            .findings
            .iter()
            .map(|f| match snapshot.get(f.id) {
                Some(entry) => result_for(f, Some(&entry.data), Some(entry.attr)),
                None => result_for(f, None, None),
            })
            .collect())
    }

    /// Read every finding one property at a time, without attributes.
    fn read_direct(&self, cam: &mut SenserCamera<NusbBackend>) -> Result<Vec<ReferenceResult>> {
        self.findings
            .iter()
            .map(|f| {
                let data = match cam.read_backup(f.id) {
                    Ok(data) => Some(data),
                    Err(pmca_usb::Error::Senser(_)) => None,
                    Err(e) => return Err(Error::from(e)),
                };
                Ok(result_for(f, data.as_deref(), None))
            })
            .collect()
    }

    /// Write the chosen value of one finding, after the user consented.
    ///
    /// Refused unless a check found the finding writable on this camera. The
    /// original value is recorded before the write.
    pub fn write(
        &mut self,
        l: &Localizer,
        sink: &mut dyn ProgressSink,
        index: usize,
        variant: usize,
    ) -> Result<ReferenceState> {
        if self.write.is_some() {
            return Err(Error::Operation(l.tr("reference-write-not-allowed")));
        }
        let finding = self
            .findings
            .get(index)
            .ok_or_else(|| Error::Operation(l.tr("reference-no-finding")))?;
        let (variant_label, new) = finding
            .variants
            .get(variant)
            .cloned()
            .ok_or_else(|| Error::Operation(l.tr("reference-no-finding")))?;
        let writable =
            self.result.as_ref().and_then(|r| r.results.get(index)).is_some_and(|r| r.writable);
        if !writable {
            return Err(Error::Operation(l.tr("reference-not-writable")));
        }
        let (id, label) = (finding.id, finding.label.clone());
        let dir = direct::rollback_dir()
            .ok_or_else(|| Error::Operation(l.tr("tweaks-rollback-unsaved")))?;

        let mut cam = Service::open(l, sink)?;
        self.ensure_same_camera(l, &mut cam.0)?;
        let original = cam.0.read_backup(id)?;
        if original == new {
            return Err(Error::Operation(l.tr("reference-already-value")));
        }
        let writes = [DirectWrite { id, old: original.clone(), new: new.clone() }];
        let key = self.rollback_key();
        direct::remember(&dir, &key, &writes)
            .map_err(|e| Error::Operation(format!("{} ({e})", l.tr("tweaks-rollback-unsaved"))))?;
        sink.emit(ProgressEvent::step(ProgressKind::Info, l.tr("tweaks-writing")));
        if let Err(e) = write_checked(&mut cam.0, l, &writes) {
            if matches!(e, Failure::Unchanged(_)) {
                direct::forget(&dir, &key);
            }
            return Err(e.into());
        }
        drop(cam);

        self.write = Some(WriteTest {
            id,
            label,
            variant_label,
            original,
            written: new,
            menu_after_write: None,
            restored: false,
            survived_restart: None,
            menu_after_restore: None,
        });
        sink.emit(ProgressEvent::step(ProgressKind::Done, l.tr("analysis-written")));
        Ok(self.state())
    }

    /// Note what the camera's menu showed after the last write or restore.
    pub fn record_menu(&mut self, observed: &str) {
        let observed = observed.trim().to_string();
        let Some(w) = self.write.as_mut() else { return };
        if w.restored {
            w.menu_after_restore = Some(observed);
        } else {
            w.menu_after_write = Some(observed);
        }
    }

    /// Write the original value back. Returns whether the written value was
    /// still there, i.e. survived the restart in between.
    pub fn restore(
        &mut self,
        l: &Localizer,
        sink: &mut dyn ProgressSink,
    ) -> Result<ReferenceState> {
        let key = self.rollback_key();
        let Some(w) = self.write.as_ref().filter(|w| !w.restored).cloned() else {
            return Err(Error::Operation(l.tr("reference-write-not-allowed")));
        };
        let mut cam = Service::open(l, sink)?;
        self.ensure_same_camera(l, &mut cam.0)?;
        let current = cam.0.read_backup(w.id)?;
        let survived = current == w.written;
        if current != w.original {
            sink.emit(ProgressEvent::step(ProgressKind::Info, l.tr("tweaks-writing")));
            let writes = [DirectWrite { id: w.id, old: current, new: w.original.clone() }];
            write_checked(&mut cam.0, l, &writes).map_err(Error::from)?;
        }
        drop(cam);
        if let Some(dir) = direct::rollback_dir() {
            direct::forget(&dir, &key);
        }
        if let Some(w) = self.write.as_mut() {
            w.survived_restart = Some(survived);
            w.restored = true;
        }
        sink.emit(ProgressEvent::step(ProgressKind::Done, l.tr("analysis-restored")));
        Ok(self.state())
    }

    /// Whether the write test wrote a value that is not yet restored.
    pub fn needs_restore(&self) -> bool {
        self.write.as_ref().is_some_and(|w| !w.restored)
    }

    /// Where the check stands, for a front-end.
    pub fn state(&self) -> ReferenceState {
        ReferenceState {
            entries: self.entries.clone(),
            result: self.result.clone(),
            write: self.write.as_ref().map(|w| ReferenceWriteState {
                id: format!("{:#010x}", w.id),
                label: w.label.clone(),
                variant_label: w.variant_label.clone(),
                written_hex: hex(&w.written),
                original_hex: hex(&w.original),
                menu_after_write: w.menu_after_write.clone(),
                restored: w.restored,
                survived_restart: w.survived_restart,
                menu_after_restore: w.menu_after_restore.clone(),
            }),
        }
    }

    /// Refuse to touch any camera but the one the check read.
    fn ensure_same_camera(&self, l: &Localizer, cam: &mut SenserCamera<NusbBackend>) -> Result<()> {
        let key = direct::camera_key(&identify(cam)?);
        if self.key.as_deref() == Some(key.as_str()) {
            return Ok(());
        }
        let model =
            self.result.as_ref().and_then(|r| r.camera.model_name.clone()).unwrap_or_default();
        Err(Error::Operation(l.tr_arg("analysis-wrong-camera", "model", &model)))
    }

    fn rollback_key(&self) -> String {
        format!("{}{ROLLBACK_SUFFIX}", self.key.as_deref().unwrap_or("camera"))
    }
}

/// The result row for one finding, given what the camera holds for it.
fn result_for(finding: &Finding, data: Option<&[u8]>, attr: Option<u16>) -> ReferenceResult {
    let small = data.is_some_and(|d| d.len() <= MAX_WRITE_SIZE);
    let matches = data.and_then(|d| {
        finding.variants.iter().find(|(_, v)| v.as_slice() == d).map(|(label, _)| label.clone())
    });
    let writable = data.is_some()
        && small
        && attr.is_some_and(|a| a & (attr::READ_ONLY | attr::PROTECTED) == 0);
    ReferenceResult {
        id: format!("{:#010x}", finding.id),
        label: finding.label.clone(),
        present: data.is_some(),
        size: data.map(<[u8]>::len),
        hex: data.filter(|d| d.len() <= MAX_WRITE_SIZE).map(hex),
        attr: attr.map(|a| format!("{a:#04x}")),
        attr_names: attr
            .map(|a| attr::names(a).iter().map(|s| s.to_string()).collect())
            .unwrap_or_default(),
        writable,
        matches,
    }
}

/// The image size the status block announces, for display only.
fn status_size(status: &[u8]) -> Option<usize> {
    status.get(4..8).map(|w| u32::from_le_bytes([w[0], w[1], w[2], w[3]]) as usize)
}

/// Parse and validate one finding.
fn parse_finding(l: &Localizer, entry: &ReferenceEntry) -> Result<Finding> {
    let label = entry.label.trim();
    if label.is_empty() {
        return Err(Error::Operation(l.tr("reference-invalid-setting")));
    }
    let id = parse_id(&entry.id)
        .ok_or_else(|| Error::Operation(l.tr_arg("reference-invalid-id", "id", &entry.id)))?;
    if entry.variants.is_empty() {
        return Err(Error::Operation(l.tr_arg("reference-no-variants", "setting", label)));
    }
    let variants = entry
        .variants
        .iter()
        .map(|v| {
            let vlabel = v.label.trim();
            if vlabel.is_empty() {
                return Err(Error::Operation(l.tr("reference-invalid-variant-label")));
            }
            let bytes = parse_hex_bytes(&v.hex).ok_or_else(|| {
                Error::Operation(l.tr_arg("reference-invalid-hex", "value", &v.hex))
            })?;
            Ok((vlabel.to_string(), bytes))
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(Finding { id, label: label.to_string(), variants })
}

/// Parse a property id from `0x`-prefixed or bare hex, underscores allowed.
fn parse_id(text: &str) -> Option<u32> {
    let cleaned: String =
        text.trim().trim_start_matches("0x").trim_start_matches("0X").replace(['_', ' '], "");
    if cleaned.is_empty() || cleaned.len() > 8 || !cleaned.bytes().all(|b| b.is_ascii_hexdigit()) {
        return None;
    }
    u32::from_str_radix(&cleaned, 16).ok()
}

/// Parse a byte string from hex with any spaces or underscores between bytes.
fn parse_hex_bytes(text: &str) -> Option<Vec<u8>> {
    let cleaned: String = text.chars().filter(|c| !c.is_whitespace() && *c != '_').collect();
    if cleaned.is_empty()
        || !cleaned.len().is_multiple_of(2)
        || !cleaned.bytes().all(|b| b.is_ascii_hexdigit())
    {
        return None;
    }
    (0..cleaned.len()).step_by(2).map(|i| u8::from_str_radix(&cleaned[i..i + 2], 16).ok()).collect()
}

/// Build findings from a shared analysis report (`pmca-analysis/1`).
///
/// Each candidate becomes one finding: its property id, the run's setting name,
/// and every snapshot value that has a hex form, de-duplicated.
pub fn entries_from_report(l: &Localizer, json: &str) -> Result<Vec<ReferenceEntry>> {
    let value: serde_json::Value =
        serde_json::from_str(json).map_err(|e| Error::Operation(e.to_string()))?;
    let setting =
        value.get("setting").and_then(serde_json::Value::as_str).unwrap_or("").to_string();
    let candidates =
        value.get("candidates").and_then(serde_json::Value::as_array).cloned().unwrap_or_default();
    let mut out = Vec::new();
    for c in &candidates {
        let id = c.get("id").and_then(serde_json::Value::as_str).unwrap_or_default().to_string();
        let mut variants: Vec<ReferenceVariant> = Vec::new();
        if let Some(values) = c.get("values").and_then(serde_json::Value::as_array) {
            for sv in values {
                let (Some(label), Some(hex)) = (
                    sv.get("label").and_then(serde_json::Value::as_str),
                    sv.get("hex").and_then(serde_json::Value::as_str),
                ) else {
                    continue;
                };
                let variant = ReferenceVariant { label: label.to_string(), hex: hex.to_string() };
                if !variants.contains(&variant) {
                    variants.push(variant);
                }
            }
        }
        if id.is_empty() || variants.is_empty() {
            continue;
        }
        out.push(ReferenceEntry { id, label: setting.clone(), variants });
    }
    if out.is_empty() {
        return Err(Error::Operation(l.tr("reference-import-empty")));
    }
    Ok(out)
}

/// Whether a reference write test left a value behind that was never restored.
pub fn has_pending_reference_restore() -> bool {
    let Some(dir) = direct::rollback_dir() else { return false };
    std::fs::read_dir(dir).is_ok_and(|entries| {
        entries.filter_map(|e| e.ok()).any(|e| {
            e.file_name().to_str().is_some_and(|n| n.ends_with(&format!("{ROLLBACK_SUFFIX}.json")))
        })
    })
}

/// Write back what an interrupted reference write test left on the camera.
pub fn restore_pending_reference(l: &Localizer, sink: &mut dyn ProgressSink) -> Result<usize> {
    let dir =
        direct::rollback_dir().ok_or_else(|| Error::Operation(l.tr("analysis-pending-none")))?;
    let mut cam = Service::open(l, sink)?;
    let key = format!("{}{ROLLBACK_SUFFIX}", direct::camera_key(&identify(&mut cam.0)?));
    let record =
        direct::load(&dir, &key).ok_or_else(|| Error::Operation(l.tr("analysis-pending-none")))?;
    let writes = direct::undo_writes(&mut cam.0, &record);
    if !writes.is_empty() {
        sink.emit(ProgressEvent::step(ProgressKind::Info, l.tr("tweaks-writing")));
        write_checked(&mut cam.0, l, &writes).map_err(Error::from)?;
    }
    drop(cam);
    direct::forget(&dir, &key);
    sink.emit(ProgressEvent::step(ProgressKind::Done, l.tr("analysis-restored")));
    Ok(writes.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(id: &str, variants: &[(&str, &str)]) -> ReferenceEntry {
        ReferenceEntry {
            id: id.to_string(),
            label: "ISO Limit".to_string(),
            variants: variants
                .iter()
                .map(|(l, h)| ReferenceVariant { label: l.to_string(), hex: h.to_string() })
                .collect(),
        }
    }

    #[test]
    fn parses_id_forms() {
        assert_eq!(parse_id("0x0164_0001"), Some(0x0164_0001));
        assert_eq!(parse_id("01640001"), Some(0x0164_0001));
        assert_eq!(parse_id(" 164 0001 "), Some(0x0164_0001));
        assert_eq!(parse_id("0X01640001"), Some(0x0164_0001));
        assert_eq!(parse_id(""), None);
        assert_eq!(parse_id("zz"), None);
        assert_eq!(parse_id("0123456789"), None); // more than 8 nibbles
    }

    #[test]
    fn parses_hex_bytes() {
        assert_eq!(parse_hex_bytes("00"), Some(vec![0x00]));
        assert_eq!(parse_hex_bytes("01 02_03"), Some(vec![1, 2, 3]));
        assert_eq!(parse_hex_bytes("0"), None); // odd length
        assert_eq!(parse_hex_bytes(""), None);
        assert_eq!(parse_hex_bytes("gg"), None);
    }

    #[test]
    fn rejects_empty_and_bad_findings() {
        let l = Localizer::new("en-US");
        assert!(ReferenceCheck::new(&l, vec![]).is_err());
        assert!(ReferenceCheck::new(&l, vec![entry("nothex", &[("A", "00")])]).is_err());
        assert!(ReferenceCheck::new(&l, vec![entry("01640001", &[])]).is_err());
        assert!(ReferenceCheck::new(&l, vec![entry("01640001", &[("A", "zz")])]).is_err());
        assert!(ReferenceCheck::new(&l, vec![entry("01640001", &[("A", "00")])]).is_ok());
    }

    #[test]
    fn matches_variant_and_marks_writable() {
        let f = Finding {
            id: 0x0164_0001,
            label: "ISO Limit".to_string(),
            variants: vec![("Standard".to_string(), vec![0]), ("High".to_string(), vec![1])],
        };
        // Present, equals the second variant, plain attributes: writable.
        let r = result_for(&f, Some(&[1]), Some(0x00));
        assert!(r.present);
        assert_eq!(r.matches.as_deref(), Some("High"));
        assert!(r.writable);
        // Read-only attribute: not writable.
        let r = result_for(&f, Some(&[1]), Some(attr::READ_ONLY));
        assert!(!r.writable);
        // No attributes known (per-property read): not writable.
        let r = result_for(&f, Some(&[1]), None);
        assert!(!r.writable);
        // Absent property.
        let r = result_for(&f, None, None);
        assert!(!r.present);
        assert_eq!(r.matches, None);
        // Present but none of the variants.
        let r = result_for(&f, Some(&[9]), Some(0x00));
        assert!(r.present);
        assert_eq!(r.matches, None);
    }
}
