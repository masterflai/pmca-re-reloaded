//! Named camera settings stored in `Backup.bin`.
//!
//! Port of the property registry in `BackupInterface` (`pmca/platform/backup.py`).
//! Every setting is one or more consecutive backup properties of equal size;
//! compound settings (the recording limit, the language table) are read and
//! written as a single contiguous byte string, exactly as the reference does.

/// Definition of a named setting.
///
/// The value occupies `count` consecutive property ids starting at `base`,
/// each holding `part_size` bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PropDef {
    /// Stable identifier used by the tweak layer and the UI.
    pub name: &'static str,
    /// First backup property id (`subsystem << 16 | index`).
    pub base: u32,
    /// Number of consecutive property ids the value spans.
    pub count: u32,
    /// Size in bytes of each individual property.
    pub part_size: usize,
}

impl PropDef {
    /// Total size of the assembled value in bytes.
    pub const fn size(&self) -> usize {
        self.count as usize * self.part_size
    }

    /// Iterate over the individual property ids that make up this setting.
    pub fn ids(&self) -> impl Iterator<Item = u32> + '_ {
        (0..self.count).map(move |i| self.base + i)
    }
}

/// All settings the toolkit knows about.
pub const PROPERTIES: &[PropDef] = &[
    PropDef { name: "androidPlatformVersion", base: 0x0166_0024, count: 1, part_size: 8 },
    PropDef { name: "modelCode", base: 0x00e7_0000, count: 1, part_size: 5 },
    PropDef { name: "modelName", base: 0x003e_0005, count: 1, part_size: 16 },
    PropDef { name: "serialNumber", base: 0x00e7_0003, count: 1, part_size: 4 },
    PropDef { name: "recLimit", base: 0x003c_0373, count: 3, part_size: 1 },
    PropDef { name: "recLimit4k", base: 0x003c_04b6, count: 1, part_size: 2 },
    PropDef { name: "palNtscSelector", base: 0x0107_0148, count: 1, part_size: 1 },
    PropDef { name: "palNtsc", base: 0x0107_04fd, count: 1, part_size: 1 },
    PropDef { name: "language", base: 0x010d_008f, count: 35, part_size: 1 },
    PropDef { name: "usbAppInstaller", base: 0x0164_0001, count: 1, part_size: 1 },
    // From the LAK132 fork. Newer models only: on older bodies the copyright
    // subsystem does not exist, so `BackupPatch::read` reports "not stored".
    PropDef { name: "photographer1", base: 0x02cf_0bd1, count: 1, part_size: 0x2f },
    PropDef { name: "photographer2", base: 0x02cf_0c11, count: 1, part_size: 0x2f },
    PropDef { name: "copyright1", base: 0x02cf_0bd2, count: 1, part_size: 0x2f },
    PropDef { name: "copyright2", base: 0x02cf_0c12, count: 1, part_size: 0x2f },
    // The same two strings four ids earlier, measured on a newer body; its
    // ids above hold single bytes, which the size check rejects.
    PropDef { name: "photographer1Alt", base: 0x02cf_0bcd, count: 1, part_size: 0x2f },
    PropDef { name: "copyright1Alt", base: 0x02cf_0bce, count: 1, part_size: 0x2f },
];

/// Look up a setting by name, built-in or registered by a plugin.
pub fn find(name: &str) -> Option<&'static PropDef> {
    all().find(|p| p.name == name)
}

/// Settings that tell one camera from another. Plugins may not define a tweak
/// on any of their property ids.
pub const IDENTITY: &[&str] = &["modelName", "modelCode", "serialNumber", "androidPlatformVersion"];

/// Settings defined by installed plugins.
static EXTRA: std::sync::OnceLock<Vec<PropDef>> = std::sync::OnceLock::new();

/// Register the settings installed plugins define, once per process.
///
/// Returns `false` if settings were already registered.
pub fn register_extra(defs: Vec<PropDef>) -> bool {
    EXTRA.set(defs).is_ok()
}

/// Every known setting: the built-in ones first, then those of plugins.
pub fn all() -> impl Iterator<Item = &'static PropDef> {
    PROPERTIES.iter().chain(EXTRA.get().map_or(&[][..], Vec::as_slice))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compound_sizes_match_the_reference() {
        assert_eq!(find("recLimit").unwrap().size(), 3);
        assert_eq!(find("language").unwrap().size(), 35);
        assert_eq!(find("modelName").unwrap().size(), 16);
    }

    #[test]
    fn compound_ids_are_consecutive() {
        let ids: Vec<u32> = find("recLimit").unwrap().ids().collect();
        assert_eq!(ids, vec![0x003c_0373, 0x003c_0374, 0x003c_0375]);
    }

    #[test]
    fn unknown_property_is_none() {
        assert!(find("nope").is_none());
    }
}
