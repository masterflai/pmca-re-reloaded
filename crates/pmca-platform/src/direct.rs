//! Settings read and written one backup property at a time.
//!
//! On cameras whose backup image is larger than about 1 MB, `PDT_READ` stalls
//! at 0xF8000 bytes and takes the USB session with it, so the image-based
//! [`crate::BackupPatch`] cannot be used. Single-property reads and writes
//! still work there (upstream issues #733 and #744). This is the same staged
//! editor on top of them; the transport stays with the caller.

use std::collections::BTreeMap;

use pmca_core::util::latin1_trim_nul;

use crate::{Error, Result, Settings, props};

/// Offset of the region string in the backup status block (`PDT_STAT`).
const STATUS_REGION_OFFSET: usize = 0x14;

/// Offset of the image size in the status block; measured on a camera whose
/// status read `40 00 00 00 a7 02 0d 00` for an 852,647-byte image.
const STATUS_SIZE_OFFSET: usize = 4;

/// Largest image a camera has been seen to deliver through `PDT_READ`.
pub const FULL_READ_LIMIT: u32 = 0xF_8000;

/// The region string from the backup status block, as the reference reads it.
pub fn status_region(status: &[u8]) -> String {
    status.get(STATUS_REGION_OFFSET..).map(latin1_trim_nul).unwrap_or_default()
}

/// Whether the status block announces an image too large for `PDT_READ`.
pub fn full_read_too_large(status: &[u8]) -> bool {
    status
        .get(STATUS_SIZE_OFFSET..STATUS_SIZE_OFFSET + 4)
        .is_some_and(|w| u32::from_le_bytes([w[0], w[1], w[2], w[3]]) > FULL_READ_LIMIT)
}

/// One property whose value a [`DirectPatch`] would change.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectWrite {
    /// Backup property id.
    pub id: u32,
    /// Value the camera holds now.
    pub old: Vec<u8>,
    /// Value to write.
    pub new: Vec<u8>,
}

/// A staged editor over property values read individually from the camera.
pub struct DirectPatch {
    region: String,
    values: BTreeMap<u32, Vec<u8>>,
    patch: BTreeMap<u32, Vec<u8>>,
}

impl DirectPatch {
    /// Wrap the values read for [`Self::wanted_ids`]; missing ids are fine.
    pub fn new(region: impl Into<String>, values: BTreeMap<u32, Vec<u8>>) -> Self {
        Self { region: region.into(), values, patch: BTreeMap::new() }
    }

    /// Every property id a known setting uses, i.e. what to read first.
    pub fn wanted_ids() -> impl Iterator<Item = u32> {
        props::all().flat_map(|p| p.ids())
    }

    /// The property ids that tell one camera from another: model name and
    /// code, serial number and platform version. Enough to recognise a body
    /// without reading every setting.
    pub fn identity_ids() -> impl Iterator<Item = u32> {
        props::IDENTITY.iter().filter_map(|name| props::find(name)).flat_map(|p| p.ids())
    }

    /// The staged changes that differ from what the camera holds.
    pub fn writes(&self) -> Vec<DirectWrite> {
        self.patch
            .iter()
            .filter_map(|(&id, new)| {
                let old = self.values.get(&id)?;
                (old != new).then(|| DirectWrite { id, old: old.clone(), new: new.clone() })
            })
            .collect()
    }
}

impl Settings for DirectPatch {
    fn region(&self) -> &str {
        &self.region
    }

    fn read(&self, name: &str) -> Result<Vec<u8>> {
        let def = props::find(name).ok_or_else(|| Error::Unknown(name.to_string()))?;
        let mut out = Vec::with_capacity(def.size());
        for id in def.ids() {
            let part = self
                .patch
                .get(&id)
                .or_else(|| self.values.get(&id))
                .ok_or_else(|| Error::Unavailable(name.to_string()))?;
            if part.len() != def.part_size {
                return Err(Error::Size {
                    name: name.to_string(),
                    expected: def.part_size,
                    actual: part.len(),
                });
            }
            out.extend_from_slice(part);
        }
        Ok(out)
    }

    fn write(&mut self, name: &str, data: &[u8]) -> Result<()> {
        let def = props::find(name).ok_or_else(|| Error::Unknown(name.to_string()))?;
        if data.len() != def.size() {
            return Err(Error::Size {
                name: name.to_string(),
                expected: def.size(),
                actual: data.len(),
            });
        }
        // Checked for every part before anything is staged.
        self.read(name)?;
        for (i, id) in def.ids().enumerate() {
            let chunk = &data[i * def.part_size..(i + 1) * def.part_size];
            self.patch.insert(id, chunk.to_vec());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tweaks;

    fn status(words: [u32; 5], region: &str) -> Vec<u8> {
        let mut out: Vec<u8> = words.iter().flat_map(|w| w.to_le_bytes()).collect();
        out.extend_from_slice(region.as_bytes());
        out.resize(0x40, 0);
        out
    }

    fn sample() -> DirectPatch {
        let values = BTreeMap::from([
            (0x0107_04fd, vec![0]),
            (0x003c_0373, vec![0]),
            (0x003c_0374, vec![29]),
            (0x003c_0375, vec![50]),
            (0x003e_0005, b"ILCE-7M4\0\0\0\0\0\0\0\0".to_vec()),
        ]);
        DirectPatch::new("CH89101_CEC", values)
    }

    #[test]
    fn reads_the_region_from_the_status_block() {
        assert_eq!(status_region(&status([0; 5], "CX12345_CEC")), "CX12345_CEC");
        assert_eq!(status_region(&[0; 4]), "");
    }

    #[test]
    fn a_size_in_the_stall_range_selects_the_direct_path() {
        // A real status block, read in full without trouble.
        assert!(!full_read_too_large(&status([0x40, 852_647, 0, 0, 0], "CX12345_CEC")));
        // 2,554,856 bytes: the CX8960_J1 camera from the cteamEpic notes.
        assert!(full_read_too_large(&status([0x40, 2_554_856, 0, 0, 0], "CX8960_J1")));
        // Only the size word counts.
        assert!(!full_read_too_large(&status([0x0020_0000, 0, 0, 0, 0x0020_0000], "EU")));
        assert!(!full_read_too_large(&[0; 6]));
    }

    #[test]
    fn wanted_ids_cover_compound_settings() {
        let ids: Vec<u32> = DirectPatch::wanted_ids().collect();
        assert!(ids.contains(&0x010d_008f));
        assert!(ids.contains(&0x010d_00b1));
        assert!(ids.contains(&0x003c_0375));
    }

    #[test]
    fn identity_ids_name_model_code_serial_and_platform() {
        let ids: Vec<u32> = DirectPatch::identity_ids().collect();
        assert_eq!(ids, vec![0x003e_0005, 0x00e7_0000, 0x00e7_0003, 0x0166_0024]);
    }

    #[test]
    fn tweaks_work_on_individually_read_values() {
        let mut patch = sample();
        let ids: Vec<String> = tweaks::list(&patch).into_iter().map(|t| t.id).collect();
        assert_eq!(ids, vec!["recLimit", "palNtsc"]);

        tweaks::set_enabled(&mut patch, "palNtsc", true, false).unwrap();
        tweaks::set_enabled(&mut patch, "recLimit", false, false).unwrap();
        // recLimit already holds its off value, so only palNtsc has to be written.
        assert_eq!(
            patch.writes(),
            vec![DirectWrite { id: 0x0107_04fd, old: vec![0], new: vec![1] }]
        );
    }

    #[test]
    fn a_setting_the_camera_did_not_return_cannot_be_written() {
        let mut patch = sample();
        assert!(patch.write("usbAppInstaller", &[1]).is_err());
        assert!(patch.write("palNtsc", &[1, 2]).is_err());
        assert!(patch.writes().is_empty());
    }
}
