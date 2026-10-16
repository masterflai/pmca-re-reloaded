//! Camera settings and tweaks for PMCA-RE RELOADED.
//!
//! Port of `pmca/platform/backup.py`, `properties.py` and `tweaks.py`. The
//! crate is deliberately transport-free: it operates on a `Backup.bin` image
//! that some backend (service mode or the on-camera shell) has read from the
//! camera, and produces the modified image(s) to write back. That keeps the
//! risky part — deciding which bytes change — fully unit-testable without
//! hardware.
//!
//! # Security
//!
//! `#![forbid(unsafe_code)]`. All property resolution is delegated to the
//! bounds-checked parser in `pmca-core`, so a hostile or corrupt backup image
//! can only produce errors, never out-of-bounds access. Writes are additionally
//! size-checked against the property's existing slot.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use std::collections::BTreeMap;

use pmca_core::backup::BackupFile;

pub mod analysis;
pub mod direct;
pub mod identity;
pub mod languages;
pub mod props;
pub mod tweaks;

pub use direct::DirectPatch;

/// Errors produced by the settings/tweak layer.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The underlying backup image could not be parsed or addressed.
    #[error(transparent)]
    Core(#[from] pmca_core::Error),
    /// The requested setting or tweak does not exist.
    #[error("unknown setting: {0}")]
    Unknown(String),
    /// A value did not have the size the camera's property slot expects.
    #[error("wrong value size for {name}: expected {expected}, got {actual}")]
    Size {
        /// Name of the setting.
        name: String,
        /// Expected size in bytes.
        expected: usize,
        /// Size that was supplied.
        actual: usize,
    },
    /// The setting exists but this camera cannot use it.
    #[error("setting not available on this camera: {0}")]
    Unavailable(String),
}

/// Convenience result alias.
pub type Result<T> = std::result::Result<T, Error>;

/// Named settings of one camera, however they were obtained.
///
/// [`BackupPatch`] works on a complete backup image, [`DirectPatch`] on values
/// read one property at a time. The tweak and identity logic only needs this.
pub trait Settings {
    /// The camera's region string (e.g. `"EU_CE3"`).
    fn region(&self) -> &str;
    /// Read a named setting, concatenating its parts.
    fn read(&self, name: &str) -> Result<Vec<u8>>;
    /// Stage a change to a named setting.
    fn write(&mut self, name: &str, data: &[u8]) -> Result<()>;
}

impl Settings for BackupPatch {
    fn region(&self) -> &str {
        BackupPatch::region(self)
    }

    fn read(&self, name: &str) -> Result<Vec<u8>> {
        BackupPatch::read(self, name)
    }

    fn write(&mut self, name: &str, data: &[u8]) -> Result<()> {
        BackupPatch::write(self, name, data)
    }
}

/// An in-memory, staged editor for a camera's `Backup.bin`.
///
/// Port of `BackupPatchDataInterface`. Changes accumulate in a patch map and
/// are only materialised by [`BackupPatch::build_images`], so nothing is
/// written to the camera until the caller explicitly asks for the result.
pub struct BackupPatch {
    image: Vec<u8>,
    region: String,
    patch: BTreeMap<u32, Vec<u8>>,
}

impl BackupPatch {
    /// Parse a backup image read from the camera.
    pub fn new(image: Vec<u8>) -> Result<Self> {
        let region = BackupFile::new(&image[..])?.region();
        Ok(Self { image, region, patch: BTreeMap::new() })
    }

    /// The camera's region string (e.g. `"EU_CE3"`), as read.
    pub fn region(&self) -> &str {
        &self.region
    }

    /// `true` if any value has been changed.
    pub fn is_dirty(&self) -> bool {
        !self.patch.is_empty()
    }

    /// Read a single backup property, preferring a staged change.
    fn read_prop(&self, id: u32) -> Result<Vec<u8>> {
        if let Some(data) = self.patch.get(&id) {
            return Ok(data.clone());
        }
        Ok(BackupFile::new(&self.image[..])?.get_property(id)?.data)
    }

    /// Stage a change to a single backup property.
    fn write_prop(&mut self, id: u32, data: Vec<u8>) -> Result<()> {
        let current = BackupFile::new(&self.image[..])?.get_property(id)?.data;
        if current.len() != data.len() {
            return Err(Error::Size {
                name: format!("{id:#010x}"),
                expected: current.len(),
                actual: data.len(),
            });
        }
        self.patch.insert(id, data);
        Ok(())
    }

    /// Read a named setting, concatenating its parts.
    pub fn read(&self, name: &str) -> Result<Vec<u8>> {
        let def = props::find(name).ok_or_else(|| Error::Unknown(name.to_string()))?;
        let mut out = Vec::with_capacity(def.size());
        for id in def.ids() {
            let part = self.read_prop(id)?;
            if part.len() != def.part_size {
                return Err(Error::Size {
                    name: name.to_string(),
                    expected: def.part_size,
                    actual: part.len(),
                });
            }
            out.extend_from_slice(&part);
        }
        Ok(out)
    }

    /// Stage a change to a named setting, splitting it across its parts.
    pub fn write(&mut self, name: &str, data: &[u8]) -> Result<()> {
        let def = props::find(name).ok_or_else(|| Error::Unknown(name.to_string()))?;
        if data.len() != def.size() {
            return Err(Error::Size {
                name: name.to_string(),
                expected: def.size(),
                actual: data.len(),
            });
        }
        for (i, id) in def.ids().enumerate() {
            let chunk = &data[i * def.part_size..(i + 1) * def.part_size];
            self.write_prop(id, chunk.to_vec())?;
        }
        Ok(())
    }

    /// Materialise the staged changes as the image(s) to write to the camera.
    ///
    /// Returns an empty vector when nothing changed. Otherwise it returns one
    /// or two images that must be written **in order**:
    ///
    /// 1. Properties marked read-only (`attr & 1`) temporarily have that bit
    ///    cleared, because the camera refuses to accept a value for them.
    /// 2. If step 1 cleared any bit, a second image restores the original
    ///    attributes so the camera is left in its normal state.
    ///
    /// This mirrors the two-phase `apply()` of `BackupPatchDataInterface`.
    /// `protect` sets the `id1` write-protection flag in the header.
    pub fn build_images(&self, protect: bool) -> Result<Vec<Vec<u8>>> {
        if !self.is_dirty() {
            return Ok(Vec::new());
        }

        let mut file = BackupFile::new(self.image.clone())?;
        let mut restore: Vec<(u32, u16)> = Vec::new();

        for (&id, data) in &self.patch {
            let prop = file.get_property(id)?;
            if prop.data != *data && prop.attr & 1 != 0 {
                restore.push((id, prop.attr));
                file.set_property_attr(id, prop.attr & !1)?;
            }
            file.set_property(id, data)?;
        }
        file.set_id1(protect);
        file.update_checksum();

        let mut images = vec![file.as_bytes().to_vec()];

        if !restore.is_empty() {
            for (id, attr) in restore {
                file.set_property_attr(id, attr)?;
            }
            file.set_id1(protect);
            file.update_checksum();
            images.push(file.into_inner());
        }
        Ok(images)
    }

    /// The image this patch was read from.
    pub fn original(&self) -> &[u8] {
        &self.image
    }

    /// Whether `read_back` holds what `sent` says for every staged property,
    /// value and attribute alike.
    ///
    /// Only the staged properties are compared: the camera keeps running
    /// counters in the same image, and one of them ticking between the write
    /// and the read-back must not count as a rejected write.
    pub fn stored(&self, sent: &[u8], read_back: &[u8]) -> Result<bool> {
        let sent = BackupFile::new(sent)?;
        let back = BackupFile::new(read_back)?;
        for &id in self.patch.keys() {
            let (a, b) = (sent.get_property(id)?, back.get_property(id)?);
            if a.data != b.data || a.attr != b.attr {
                return Ok(false);
            }
        }
        Ok(true)
    }
}

/// One attribute set back by [`restore_attributes`]: `(id, was, now)`.
pub type AttributeChange = (u32, u16, u16);

/// `current` with every property attribute set back to the one in `reference`.
///
/// Returns the new image and the changes; the
/// image is unchanged apart from its checksum when that list is empty. Used to
/// put read-only bits back after a two-image write stopped half-way.
pub fn restore_attributes(
    current: &[u8],
    reference: &[u8],
) -> Result<(Vec<u8>, Vec<AttributeChange>)> {
    let mut file = BackupFile::new(current.to_vec())?;
    let reference = BackupFile::new(reference)?;
    let mut changed = Vec::new();
    for (id, prop) in reference.list_properties()? {
        if let Ok(now) = file.get_property(id)
            && now.attr != prop.attr
        {
            file.set_property_attr(id, prop.attr)?;
            changed.push((id, now.attr, prop.attr));
        }
    }
    file.update_checksum();
    Ok((file.into_inner(), changed))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A backup image exposing the `recLimit` (writable) and `palNtscSelector`
    /// (read-only) settings at their real property ids.
    pub(crate) fn sample_image() -> Vec<u8> {
        // Subsystems are addressed by the high 16 bits of a property id, so the
        // table must be large enough to contain 0x02cf (the copyright strings).
        let num_subsystems: u32 = 0x02d0;
        let data_size: u32 = 0x100;

        // Properties, laid out one after another in the data region.
        // (id, size, attr, initial value)
        let entries: &[(u32, usize, u16, &[u8])] = &[
            (0x003c_0373, 1, 0x00, &[0]),
            (0x003c_0374, 1, 0x00, &[29]),
            (0x003c_0375, 1, 0x00, &[50]),
            (0x003c_04b6, 2, 0x00, &[0x2c, 0x01]),
            (0x0107_0148, 1, 0x01, &[0]), // read-only, like on a real camera
            (0x0107_04fd, 1, 0x00, &[0]), // video system, 0 = PAL
            (0x0164_0001, 1, 0x00, &[0]),
            // The identifying values, all read-only on a real camera.
            (0x003e_0005, 16, 0x01, b"ILCE-5100\0\0\0\0\0\0\0"),
            (0x00e7_0000, 5, 0x01, &[0x11, 0x22, 0x33, 0x44, 0x55]),
            (0x00e7_0003, 4, 0x01, &[0x01, 0x23, 0x45, 0x67]),
            (0x0166_0024, 8, 0x01, b"3.0.0\0\0\0"),
            (0x02cf_0bd1, 0x2f, 0x00, &[b'A'; 0x2f]),
            (0x02cf_0bd2, 0x2f, 0x00, &[0; 0x2f]), // copyright left empty
        ];

        // One subsystem table entry per subsystem; each subsystem gets a slice
        // of the flat property table.
        let mut sub_props: Vec<u16> = vec![0; num_subsystems as usize];
        for (id, ..) in entries {
            let sub = (id >> 16) as usize;
            let idx = (id & 0xffff) as u16;
            sub_props[sub] = sub_props[sub].max(idx + 1);
        }
        let num_properties: u32 = sub_props.iter().map(|&n| n as u32).sum();

        // A subsystem's slice runs from index 0 to its highest used property,
        // so the table's size follows from the ids and cannot be a constant:
        // adding one high-numbered property adds hundreds of empty entries.
        let table = 0x100 + num_subsystems as usize * 6;
        let data_offset = (table as u32 + num_properties * 6).next_multiple_of(0x1000);
        let mut buf = vec![0u8; (data_offset + data_size) as usize];

        buf[12..16].copy_from_slice(b"BK4\0");
        buf[16..20].copy_from_slice(&num_subsystems.to_le_bytes());
        buf[20..24].copy_from_slice(&num_properties.to_le_bytes());
        buf[24..28].copy_from_slice(&data_offset.to_le_bytes());
        buf[28..32].copy_from_slice(&data_size.to_le_bytes());
        buf[0xc0..0xc6].copy_from_slice(b"EU_CE3");

        let mut ptrs: Vec<u32> = Vec::with_capacity(num_subsystems as usize);
        let mut next = 0u32;
        for &n in &sub_props {
            ptrs.push(next);
            next += n as u32;
        }

        for (sub, (&n, &ptr)) in sub_props.iter().zip(ptrs.iter()).enumerate() {
            let off = 0x100 + sub * 6;
            buf[off..off + 2].copy_from_slice(&n.to_le_bytes());
            buf[off + 2..off + 6].copy_from_slice(&ptr.to_le_bytes());
        }
        // Absent properties must be marked with the "no data" pointer.
        for i in 0..num_properties as usize {
            let e = table + i * 6;
            buf[e + 2..e + 6].copy_from_slice(&0xffff_ffffu32.to_le_bytes());
        }

        let mut data_ptr = data_offset;
        for (id, size, attr, value) in entries {
            let sub = (id >> 16) as usize;
            let idx = (id & 0xffff) as usize;
            let e = table + (ptrs[sub] as usize + idx) * 6;
            buf[e..e + 2].copy_from_slice(&attr.to_le_bytes());
            let ptr = ((*size as u32) << 24) | data_ptr;
            buf[e + 2..e + 6].copy_from_slice(&ptr.to_le_bytes());
            buf[data_ptr as usize..data_ptr as usize + size].copy_from_slice(value);
            data_ptr += *size as u32;
        }

        let end = (data_offset + data_size) as usize;
        fix_checksum(&mut buf[..end]);
        buf
    }

    /// Recompute the header checksum over a whole backup image.
    ///
    /// The camera's checksum is a plain byte sum that excludes the checksum
    /// field itself, so any test that edits an image has to restore it before
    /// the parser will accept the result.
    pub(crate) fn fix_checksum(buf: &mut [u8]) {
        let sum: u32 = buf
            .iter()
            .enumerate()
            .filter(|(i, _)| !(0x20..0x24).contains(i))
            .fold(0u32, |acc, (_, &b)| acc.wrapping_add(b as u32));
        buf[0x20..0x24].copy_from_slice(&sum.to_le_bytes());
    }

    #[test]
    fn reads_the_region_from_the_header() {
        let patch = BackupPatch::new(sample_image()).unwrap();
        assert_eq!(patch.region(), "EU_CE3");
        assert!(!patch.is_dirty());
    }

    #[test]
    fn reads_a_compound_setting_as_one_value() {
        let patch = BackupPatch::new(sample_image()).unwrap();
        assert_eq!(patch.read("recLimit").unwrap(), vec![0, 29, 50]);
    }

    #[test]
    fn staged_write_is_visible_to_read_but_not_to_the_image() {
        let mut patch = BackupPatch::new(sample_image()).unwrap();
        patch.write("recLimit", &[13, 1, 0]).unwrap();
        assert!(patch.is_dirty());
        assert_eq!(patch.read("recLimit").unwrap(), vec![13, 1, 0]);
        // The original image is untouched until build_images runs.
        let original = BackupFile::new(&patch.image[..]).unwrap();
        assert_eq!(original.get_property(0x003c_0373).unwrap().data, vec![0]);
    }

    #[test]
    fn rejects_a_value_of_the_wrong_length() {
        let mut patch = BackupPatch::new(sample_image()).unwrap();
        assert!(matches!(patch.write("recLimit", &[1, 2]), Err(Error::Size { .. })));
    }

    #[test]
    fn rejects_an_unknown_setting() {
        let mut patch = BackupPatch::new(sample_image()).unwrap();
        assert!(matches!(patch.read("nope"), Err(Error::Unknown(_))));
        assert!(matches!(patch.write("nope", &[0]), Err(Error::Unknown(_))));
    }

    #[test]
    fn no_changes_produce_no_images() {
        let patch = BackupPatch::new(sample_image()).unwrap();
        assert!(patch.build_images(true).unwrap().is_empty());
    }

    #[test]
    fn writable_property_needs_a_single_image() {
        let mut patch = BackupPatch::new(sample_image()).unwrap();
        patch.write("recLimit", &[13, 1, 0]).unwrap();
        let images = patch.build_images(false).unwrap();
        assert_eq!(images.len(), 1);

        let result = BackupFile::new(&images[0][..]).unwrap();
        assert_eq!(result.get_property(0x003c_0373).unwrap().data, vec![13]);
        assert_eq!(result.get_property(0x003c_0375).unwrap().data, vec![0]);
        assert_eq!(result.id1().unwrap(), 0);
    }

    #[test]
    fn read_only_property_needs_a_second_restoring_image() {
        let mut patch = BackupPatch::new(sample_image()).unwrap();
        patch.write("palNtscSelector", &[1]).unwrap();
        let images = patch.build_images(true).unwrap();
        assert_eq!(images.len(), 2);

        // First image: read-only bit cleared so the camera accepts the write.
        let first = BackupFile::new(&images[0][..]).unwrap();
        assert_eq!(first.get_property(0x0107_0148).unwrap().attr, 0x00);
        assert_eq!(first.get_property(0x0107_0148).unwrap().data, vec![1]);

        // Second image: original attribute restored, value kept.
        let second = BackupFile::new(&images[1][..]).unwrap();
        assert_eq!(second.get_property(0x0107_0148).unwrap().attr, 0x01);
        assert_eq!(second.get_property(0x0107_0148).unwrap().data, vec![1]);
        assert_eq!(second.id1().unwrap(), 1);
    }

    #[test]
    fn every_generated_image_carries_a_valid_checksum() {
        let mut patch = BackupPatch::new(sample_image()).unwrap();
        patch.write("palNtscSelector", &[1]).unwrap();
        patch.write("recLimit", &[13, 1, 0]).unwrap();
        for image in patch.build_images(true).unwrap() {
            // BackupFile::new verifies the checksum.
            BackupFile::new(&image[..]).expect("checksum must be valid");
        }
    }

    /// A camera property that is not part of the patch, changed after writing.
    fn with_counter_ticked(image: &[u8]) -> Vec<u8> {
        let mut file = BackupFile::new(image.to_vec()).unwrap();
        file.set_property(0x0164_0001, &[7]).unwrap();
        file.update_checksum();
        file.into_inner()
    }

    #[test]
    fn verification_ignores_properties_the_patch_does_not_touch() {
        let mut patch = BackupPatch::new(sample_image()).unwrap();
        patch.write("palNtscSelector", &[1]).unwrap();
        for image in patch.build_images(true).unwrap() {
            assert!(patch.stored(&image, &with_counter_ticked(&image)).unwrap());
        }
    }

    #[test]
    fn verification_notices_a_value_or_attribute_that_did_not_stick() {
        let mut patch = BackupPatch::new(sample_image()).unwrap();
        patch.write("palNtscSelector", &[1]).unwrap();
        let images = patch.build_images(true).unwrap();
        // The original image differs in the value; the first image differs
        // from the second only in the restored read-only bit.
        assert!(!patch.stored(&images[0], patch.original()).unwrap());
        assert!(!patch.stored(&images[1], &images[0]).unwrap());
    }

    #[test]
    fn attributes_cleared_by_a_stopped_write_are_put_back() {
        let mut patch = BackupPatch::new(sample_image()).unwrap();
        patch.write("palNtscSelector", &[1]).unwrap();
        let half_way = &patch.build_images(true).unwrap()[0];

        let (image, changed) = restore_attributes(half_way, patch.original()).unwrap();
        assert_eq!(changed, vec![(0x0107_0148, 0x00, 0x01)]);
        let file = BackupFile::new(&image[..]).unwrap();
        assert_eq!(file.get_property(0x0107_0148).unwrap().attr, 0x01);
        // The value stays as written; only the attribute is put back.
        assert_eq!(file.get_property(0x0107_0148).unwrap().data, vec![1]);

        let (_, nothing) = restore_attributes(&image, patch.original()).unwrap();
        assert!(nothing.is_empty());
    }
}
