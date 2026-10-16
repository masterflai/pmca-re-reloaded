//! Read-only identifying values stored in the camera's `Backup.bin`.
//!
//! Port of the backup-backed half of `PropertyInterface`
//! (`pmca/platform/properties.py`), which the earlier stages left out.
//!
//! These values are deliberately **not** tweaks. A tweak is a pair of known
//! "off" and "on" values, and [`crate::tweaks`] refuses to write one unless
//! the camera's current value is a value the toolkit recognises. A serial
//! number or a model name has no such reference value — every byte pattern
//! looks equally plausible — so that safety check could not be applied and the
//! only honest thing to do with them is show them.

use pmca_core::util::latin1_trim_nul;

use crate::Settings;

/// One identifying value read from a backup image.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentityValue {
    /// Stable identifier; matches the property name where the value is one.
    pub id: &'static str,
    /// Human-readable rendering of the camera's current value.
    pub value: String,
}

/// How a value's raw bytes are turned into text.
enum Kind {
    /// NUL-padded Latin-1 text.
    Text,
    /// Opaque bytes, rendered as lowercase hex.
    ///
    /// The reference strips leading zeros here (`HexBackupProperty`); this
    /// port keeps them, because the same serial number read over USB is
    /// rendered full-width and one camera reporting two different serials in
    /// two places of the same window would be a defect, not fidelity.
    Hex,
    /// Not a property at all — the region string from the image header.
    Region,
}

/// The values to report, in the order the reference lists them.
const VALUES: &[(&str, Kind)] = &[
    ("modelName", Kind::Text),
    ("modelCode", Kind::Hex),
    ("serialNumber", Kind::Hex),
    ("backupRegion", Kind::Region),
    ("androidPlatformVersion", Kind::Text),
    ("photographer1", Kind::Text),
    ("photographer2", Kind::Text),
    ("copyright1", Kind::Text),
    ("copyright2", Kind::Text),
];

/// Where a value lives on bodies that keep it elsewhere: `(id, setting)`.
const FALLBACKS: &[(&str, &str)] =
    &[("photographer1", "photographer1Alt"), ("copyright1", "copyright1Alt")];

/// Read a setting, or its fallback where the first place does not hold it.
fn read_any<S: Settings + ?Sized>(patch: &S, id: &str) -> crate::Result<Vec<u8>> {
    patch.read(id).or_else(|e| match FALLBACKS.iter().find(|(v, _)| *v == id) {
        Some((_, alt)) => patch.read(alt),
        None => Err(e),
    })
}

/// Read every identifying value the given backup image carries.
///
/// A value the camera does not store is skipped rather than turned into an
/// error, mirroring the reference's per-property `try`: a model that lacks,
/// say, the Android platform version should still report everything else.
pub fn list<S: Settings + ?Sized>(patch: &S) -> Vec<IdentityValue> {
    let mut out = Vec::with_capacity(VALUES.len());
    for (id, kind) in VALUES {
        let value = match kind {
            Kind::Region => patch.region().to_string(),
            Kind::Text => match read_any(patch, id) {
                Ok(data) => latin1_trim_nul(&data),
                Err(_) => continue,
            },
            Kind::Hex => match patch.read(id) {
                Ok(data) => data.iter().map(|b| format!("{b:02x}")).collect(),
                Err(_) => continue,
            },
        };
        if value.is_empty() {
            continue;
        }
        out.push(IdentityValue { id, value });
    }
    // Numbered only on bodies that store a second pair at all.
    for (single, first, second) in [
        ("photographer", "photographer1", "photographer2"),
        ("copyright", "copyright1", "copyright2"),
    ] {
        if read_any(patch, second).is_err()
            && let Some(value) = out.iter_mut().find(|v| v.id == first)
        {
            value.id = single;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BackupPatch;
    use crate::tests::{fix_checksum, sample_image};

    fn values() -> Vec<(&'static str, String)> {
        let patch = BackupPatch::new(sample_image()).unwrap();
        list(&patch).into_iter().map(|v| (v.id, v.value)).collect()
    }

    #[test]
    fn reads_the_sample_camera_identity() {
        assert_eq!(
            values(),
            vec![
                ("modelName", "ILCE-5100".to_string()),
                ("modelCode", "1122334455".to_string()),
                ("serialNumber", "01234567".to_string()),
                ("backupRegion", "EU_CE3".to_string()),
                ("androidPlatformVersion", "3.0.0".to_string()),
                ("photographer", "A".repeat(0x2f)),
            ]
        );
    }

    #[test]
    fn a_missing_property_is_skipped_not_an_error() {
        // A camera that does not store one of the identifying values must
        // still report the rest, instead of failing the whole read.
        let mut image = sample_image();
        // Empty the model name's subsystem so the property cannot be resolved.
        let entry = 0x100 + 0x003e * 6;
        image[entry..entry + 2].copy_from_slice(&0u16.to_le_bytes());
        fix_checksum(&mut image);

        let ids: Vec<&str> =
            list(&BackupPatch::new(image).unwrap()).into_iter().map(|v| v.id).collect();
        assert!(!ids.contains(&"modelName"));
        assert!(ids.contains(&"serialNumber"));
    }

    #[test]
    fn photographer_and_copyright_are_found_where_newer_bodies_keep_them() {
        let mut name = b"Jane Doe".to_vec();
        name.resize(0x2f, 0);
        // On these bodies the ids from the LAK132 fork hold single bytes.
        let values = std::collections::BTreeMap::from([
            (0x02cf_0bcd, name.clone()),
            (0x02cf_0bce, name),
            (0x02cf_0bd1, vec![0]),
            (0x02cf_0bd2, vec![0]),
        ]);
        let found: Vec<(&str, String)> = list(&crate::DirectPatch::new("EU", values))
            .into_iter()
            .filter(|v| v.id != "backupRegion")
            .map(|v| (v.id, v.value))
            .collect();
        assert_eq!(
            found,
            vec![("photographer", "Jane Doe".to_string()), ("copyright", "Jane Doe".to_string())]
        );
    }

    #[test]
    fn two_pairs_keep_their_numbers() {
        let text = |s: &str| {
            let mut v = s.as_bytes().to_vec();
            v.resize(0x2f, 0);
            v
        };
        let values = std::collections::BTreeMap::from([
            (0x02cf_0bd1, text("One")),
            (0x02cf_0c11, text("Two")),
            (0x02cf_0c12, text("")),
        ]);
        let ids: Vec<&str> =
            list(&crate::DirectPatch::new("EU", values)).into_iter().map(|v| v.id).collect();
        assert_eq!(ids, vec!["backupRegion", "photographer1", "photographer2"]);
    }

    #[test]
    fn the_region_always_comes_from_the_header() {
        let patch = BackupPatch::new(sample_image()).unwrap();
        let region = list(&patch).into_iter().find(|v| v.id == "backupRegion").unwrap();
        assert_eq!(region.value, patch.region());
    }
}
