//! Tweaks through single-property backup access, for cameras whose backup
//! image cannot be read in one piece.
//!
//! Nothing here is ever left half done on purpose: the old values are saved to
//! a rollback file before the first write, every write is read back, a refused
//! or mismatching write undoes the ones before it, and the backup protection is
//! switched back on whatever happened.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use pmca_platform::direct::DirectWrite;
use pmca_platform::{DirectPatch, Settings};
use pmca_usb::UsbBackend;
use pmca_usb::senser_camera::SenserCamera;

/// The single-property backup commands this module needs.
pub(crate) trait BackupAccess {
    fn read_prop(&mut self, id: u32) -> pmca_usb::Result<Vec<u8>>;
    fn write_prop(&mut self, id: u32, data: &[u8]) -> pmca_usb::Result<()>;
    /// `true` sets the backup write protection (`ID1`), `false` clears it.
    fn protect(&mut self, on: bool) -> pmca_usb::Result<()>;
    fn persist(&mut self) -> pmca_usb::Result<()>;
}

impl<B: UsbBackend> BackupAccess for SenserCamera<B> {
    fn read_prop(&mut self, id: u32) -> pmca_usb::Result<Vec<u8>> {
        self.read_backup(id)
    }

    fn write_prop(&mut self, id: u32, data: &[u8]) -> pmca_usb::Result<()> {
        self.write_backup(id, data)
    }

    fn protect(&mut self, on: bool) -> pmca_usb::Result<()> {
        self.set_backup_id1(on).map(|_| ())
    }

    fn persist(&mut self) -> pmca_usb::Result<()> {
        self.save_backup(0)
    }
}

/// Read every known setting one property at a time.
///
/// A property the camera does not have is answered with a senser error and
/// simply left out, as the reference's per-property `try` does. Any other
/// error means the camera is no longer reachable, e.g. a held session whose
/// camera was unplugged, and ends the read.
pub(crate) fn read(cam: &mut impl BackupAccess, region: &str) -> pmca_usb::Result<DirectPatch> {
    read_ids(cam, region, DirectPatch::wanted_ids())
}

/// Read the given properties one at a time, with the same rules as [`read`].
pub(crate) fn read_ids(
    cam: &mut impl BackupAccess,
    region: &str,
    ids: impl Iterator<Item = u32>,
) -> pmca_usb::Result<DirectPatch> {
    let mut values = BTreeMap::new();
    for id in ids {
        match cam.read_prop(id) {
            Ok(data) => {
                values.insert(id, data);
            }
            Err(pmca_usb::Error::Senser(_)) => {}
            Err(e) => return Err(e),
        }
    }
    Ok(DirectPatch::new(region, values))
}

/// Why a direct write did not go through.
#[derive(Debug)]
pub(crate) enum WriteError {
    /// A value was refused or did not stick.
    Refused {
        cause: pmca_usb::Error,
        /// Every value written before the failure was put back.
        unchanged: bool,
    },
    /// Protection or persisting failed after all values were written.
    Usb(pmca_usb::Error),
}

/// Write the values, verify each one, and persist them.
pub(crate) fn write(
    cam: &mut impl BackupAccess,
    writes: &[DirectWrite],
    progress: &mut dyn FnMut(usize),
) -> Result<(), WriteError> {
    cam.protect(false).map_err(|cause| WriteError::Refused { cause, unchanged: true })?;
    let outcome = write_verified(cam, writes, progress);
    let relock = cam.protect(true);
    outcome?;
    relock.map_err(WriteError::Usb)?;
    cam.persist().map_err(WriteError::Usb)
}

fn write_verified(
    cam: &mut impl BackupAccess,
    writes: &[DirectWrite],
    progress: &mut dyn FnMut(usize),
) -> Result<(), WriteError> {
    for (done, w) in writes.iter().enumerate() {
        if let Err(cause) = write_one(cam, w.id, &w.new) {
            // Every value gets its attempt, even after one restore failed.
            let mut unchanged = true;
            for w in writes[..=done].iter().rev() {
                unchanged &= restore(cam, w);
            }
            return Err(WriteError::Refused { cause, unchanged });
        }
        progress(done + 1);
    }
    Ok(())
}

fn restore(cam: &mut impl BackupAccess, w: &DirectWrite) -> bool {
    match cam.read_prop(w.id) {
        Ok(current) if current == w.old => true,
        _ => write_one(cam, w.id, &w.old).is_ok(),
    }
}

fn write_one(cam: &mut impl BackupAccess, id: u32, data: &[u8]) -> pmca_usb::Result<()> {
    cam.write_prop(id, data)?;
    if cam.read_prop(id)? != data {
        return Err(pmca_usb::Error::Protocol("the camera did not keep the written value"));
    }
    Ok(())
}

/// The values a camera held before the toolkit first changed them.
#[derive(Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub(crate) struct Rollback {
    pub values: BTreeMap<u32, Vec<u8>>,
}

/// Where rollback records live, if the platform has a per-user data folder.
pub(crate) fn rollback_dir() -> Option<PathBuf> {
    Some(pmca_i18n::catalog::user_data_dir()?.join("rollback"))
}

/// A file name for this camera, built from its model and serial number.
pub(crate) fn camera_key(settings: &impl Settings) -> String {
    let ids = pmca_platform::identity::list(settings);
    let get = |id: &str| ids.iter().find(|v| v.id == id).map(|v| v.value.clone());
    let raw = format!(
        "{}-{}",
        get("modelName").unwrap_or_else(|| "camera".into()),
        get("serialNumber").unwrap_or_default()
    );
    raw.chars().map(|c| if c.is_ascii_alphanumeric() || c == '-' { c } else { '_' }).collect()
}

fn record_path(dir: &Path, key: &str) -> PathBuf {
    dir.join(format!("{key}.json"))
}

pub(crate) fn load(dir: &Path, key: &str) -> Option<Rollback> {
    let text = std::fs::read_to_string(record_path(dir, key)).ok()?;
    serde_json::from_str(&text).ok()
}

/// Add the old values of `writes`, keeping any value recorded earlier: undo
/// has to return to the state before the first change, not the last one.
pub(crate) fn remember(dir: &Path, key: &str, writes: &[DirectWrite]) -> std::io::Result<()> {
    let mut record = load(dir, key).unwrap_or_default();
    for w in writes {
        record.values.entry(w.id).or_insert_with(|| w.old.clone());
    }
    std::fs::create_dir_all(dir)?;
    let json = serde_json::to_string_pretty(&record).map_err(std::io::Error::other)?;
    std::fs::write(record_path(dir, key), json)
}

pub(crate) fn forget(dir: &Path, key: &str) {
    let _ = std::fs::remove_file(record_path(dir, key));
}

/// The writes that bring the camera back to the recorded values.
pub(crate) fn undo_writes(cam: &mut impl BackupAccess, record: &Rollback) -> Vec<DirectWrite> {
    record
        .values
        .iter()
        .filter_map(|(&id, old)| {
            let current = cam.read_prop(id).ok()?;
            (current != *old).then(|| DirectWrite { id, old: current, new: old.clone() })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An in-memory camera that refuses writes to the ids in `refuse`.
    #[derive(Default)]
    struct FakeCamera {
        values: BTreeMap<u32, Vec<u8>>,
        refuse: Vec<u32>,
        protected: bool,
        saved: bool,
        unplugged: bool,
    }

    impl BackupAccess for FakeCamera {
        fn read_prop(&mut self, id: u32) -> pmca_usb::Result<Vec<u8>> {
            if self.unplugged {
                return Err(pmca_usb::Error::Transfer("device disconnected".into()));
            }
            self.values.get(&id).cloned().ok_or(pmca_usb::Error::Senser("error 129".into()))
        }
        fn write_prop(&mut self, id: u32, data: &[u8]) -> pmca_usb::Result<()> {
            if self.protected || self.refuse.contains(&id) {
                return Err(pmca_usb::Error::Senser("senser adjust control error 129".into()));
            }
            self.values.insert(id, data.to_vec());
            Ok(())
        }
        fn protect(&mut self, on: bool) -> pmca_usb::Result<()> {
            self.protected = on;
            Ok(())
        }
        fn persist(&mut self) -> pmca_usb::Result<()> {
            self.saved = true;
            Ok(())
        }
    }

    fn camera() -> FakeCamera {
        FakeCamera {
            values: BTreeMap::from([(1, vec![2]), (2, vec![2]), (3, vec![2])]),
            protected: true,
            ..Default::default()
        }
    }

    fn writes(ids: &[u32]) -> Vec<DirectWrite> {
        ids.iter().map(|&id| DirectWrite { id, old: vec![2], new: vec![1] }).collect()
    }

    #[test]
    fn writes_are_verified_persisted_and_protection_restored() {
        let mut cam = camera();
        let mut steps = Vec::new();
        write(&mut cam, &writes(&[1, 2, 3]), &mut |n| steps.push(n)).unwrap();
        assert_eq!(cam.values.values().collect::<Vec<_>>(), vec![&vec![1]; 3]);
        assert_eq!(steps, vec![1, 2, 3]);
        assert!(cam.protected && cam.saved);
    }

    #[test]
    fn a_refused_write_undoes_the_earlier_ones() {
        let mut cam = FakeCamera { refuse: vec![3], ..camera() };
        let err = write(&mut cam, &writes(&[1, 2, 3]), &mut |_| {}).unwrap_err();
        assert!(matches!(err, WriteError::Refused { unchanged: true, .. }));
        assert_eq!(cam.values.values().collect::<Vec<_>>(), vec![&vec![2]; 3]);
        assert!(cam.protected);
        assert!(!cam.saved);
    }

    #[test]
    fn missing_properties_are_left_out_of_a_read() {
        let mut cam =
            FakeCamera { values: BTreeMap::from([(0x0107_04fd, vec![0])]), ..Default::default() };
        let patch = read(&mut cam, "EU").unwrap();
        assert_eq!(patch.read("palNtsc").unwrap(), vec![0]);
        assert!(patch.read("language").is_err());
    }

    #[test]
    fn an_unreachable_camera_fails_the_read_instead_of_reading_empty() {
        let mut cam = FakeCamera { unplugged: true, ..camera() };
        assert!(matches!(read(&mut cam, "EU"), Err(pmca_usb::Error::Transfer(_))));
    }

    #[test]
    fn the_rollback_record_keeps_the_first_old_value() {
        let dir = std::env::temp_dir().join(format!("pmca-rollback-{}", std::process::id()));
        remember(&dir, "cam", &writes(&[1])).unwrap();
        remember(&dir, "cam", &[DirectWrite { id: 1, old: vec![1], new: vec![3] }]).unwrap();
        remember(&dir, "cam", &[DirectWrite { id: 2, old: vec![5], new: vec![6] }]).unwrap();
        let record = load(&dir, "cam").unwrap();
        assert_eq!(record.values, BTreeMap::from([(1, vec![2]), (2, vec![5])]));

        let mut cam = camera();
        cam.values.insert(1, vec![3]);
        let undo = undo_writes(&mut cam, &record);
        assert_eq!(
            undo,
            vec![
                DirectWrite { id: 1, old: vec![3], new: vec![2] },
                DirectWrite { id: 2, old: vec![2], new: vec![5] },
            ]
        );

        forget(&dir, "cam");
        assert!(load(&dir, "cam").is_none());
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn every_message_of_the_direct_path_is_translated() {
        let l = pmca_i18n::Localizer::new("en-US");
        for key in [
            "tweaks-full-read-failed",
            "tweaks-direct-mode",
            "tweaks-direct-forced",
            "tweaks-direct-refused",
            "tweaks-direct-revert-failed",
            "tweaks-direct-restart",
            "tweaks-rollback-unsaved",
            "tweaks-undo-none",
            "tweaks-undo-confirm",
            "tweaks-undo-done",
            "tweaks-undo-available",
            "btn-undo-tweaks",
        ] {
            assert_ne!(l.tr_arg(key, "detail", "d"), key, "{key} is missing from the catalogue");
        }
    }

    #[test]
    fn the_camera_key_is_a_safe_file_name() {
        let patch = DirectPatch::new(
            "EU",
            BTreeMap::from([
                (0x003e_0005, b"ILCE/7M4\0\0\0\0\0\0\0\0".to_vec()),
                (0x00e7_0003, vec![0x01, 0x23, 0x45, 0x67]),
            ]),
        );
        assert_eq!(camera_key(&patch), "ILCE_7M4-01234567");
    }
}
