//! Parser and editor for `Backup.bin`, the Sony camera settings store.
//!
//! Faithful port of `pmca/backup/__init__.py` (revisions 2 and 4).
//!
//! # Security hardening
//!
//! The original relied on Python raising on bad slices. Here every table index
//! and property pointer is validated against the declared data region before
//! use, so a crafted `Backup.bin` cannot trigger out-of-bounds access. The
//! write paths reuse exactly the same resolution logic, so they can never
//! place data outside the region the header declares.

use crate::util::{dump32le, latin1_trim_nul, parse16le, parse32le};
use crate::{Error, Result};

const HEADER_SIZE: usize = 0x100;
const SUBSYSTEM_ENTRY_SIZE: usize = 6; // u16 numProperties + u32 ptr
const CHECKSUM_OFFSET: usize = 0x20;
const ID1_OFFSET: usize = 0x28;

/// Property attribute bits, as named in the LAK132 fork of the reference.
pub mod attr {
    /// Cannot be written with `Backup_write()`.
    pub const READ_ONLY: u16 = 0x01;
    /// Not changed by `Backup_protect()`.
    pub const PROTECTED: u16 = 0x02;
    /// Writing the property triggers callbacks on the camera.
    pub const CALLBACKS: u16 = 0x08;
    /// Any of these bits: a factory value follows the data and `Backup_reset()`
    /// restores it.
    pub const HAS_RESET: u16 = 0x04 | 0x10 | 0x20 | 0x40;
    /// The data is an array read with `Backup_read_setting_attr()`.
    pub const ARRAY_DATA: u16 = 0x80;

    /// Every named bit, in the order the fork prints them.
    const NAMES: &[(u16, &str)] = &[
        (READ_ONLY, "READ_ONLY"),
        (PROTECTED, "PROTECTED"),
        (CALLBACKS, "CALLBACKS"),
        (0x04, "HAS_RESET_1"),
        (0x10, "HAS_RESET_2"),
        (0x20, "HAS_RESET_3"),
        (0x40, "HAS_RESET_4"),
        (ARRAY_DATA, "ARRAY_DATA"),
    ];

    /// The names of the bits set in `value`, for a human-readable dump.
    pub fn names(value: u16) -> Vec<&'static str> {
        NAMES.iter().filter(|(bit, _)| value & bit != 0).map(|(_, name)| *name).collect()
    }
}

/// A single decoded backup property.
#[derive(Debug, Clone)]
pub struct Property {
    /// Attribute bitfield (width depends on revision).
    pub attr: u16,
    /// Current value bytes.
    pub data: Vec<u8>,
    /// Factory-reset value bytes, if the property is resettable.
    pub reset_data: Option<Vec<u8>>,
}

/// Parsed, validated view over a `Backup.bin` buffer.
///
/// `D` is the backing storage. Any `AsRef<[u8]>` (e.g. `&[u8]`, `Vec<u8>`)
/// enables the read API; adding `AsMut<[u8]>` also enables the edit API.
pub struct BackupFile<D> {
    data: D,
    revision: u8,
    num_subsystems: u32,
    num_properties: u32,
    data_offset: u32,
    size: usize,
    property_entry_size: usize,
    property_table_offset: usize,
}

impl<D: AsRef<[u8]>> BackupFile<D> {
    /// Parse and validate the header of a `Backup.bin` buffer.
    pub fn new(data: D) -> Result<Self> {
        let buf = data.as_ref();
        if buf.len() < HEADER_SIZE {
            return Err(Error::OutOfBounds { context: "backup header" });
        }
        let revision_field = &buf[12..16];
        if &revision_field[..2] != b"BK" || revision_field[3] != 0 {
            return Err(Error::Invalid("invalid backup file"));
        }
        let revision = revision_field[2].wrapping_sub(b'0');
        if revision != 2 && revision != 4 {
            return Err(Error::Invalid("invalid backup revision"));
        }

        let num_subsystems = parse32le(&buf[16..20])?;
        let num_properties = parse32le(&buf[20..24])?;
        let data_offset = parse32le(&buf[24..28])?;
        let data_size = parse32le(&buf[28..32])?;
        let checksum = parse32le(&buf[32..36])?;

        let size = (data_offset as usize)
            .checked_add(data_size as usize)
            .ok_or(Error::OutOfBounds { context: "backup size" })?;
        if size > buf.len() {
            return Err(Error::OutOfBounds { context: "backup size" });
        }

        if checksum != Self::calc_checksum(&buf[..size]) {
            return Err(Error::Checksum);
        }

        let property_entry_size = if revision >= 4 { 6 } else { 5 };
        let subsystem_table_offset = HEADER_SIZE;
        let property_table_offset = subsystem_table_offset
            .checked_add(num_subsystems as usize * SUBSYSTEM_ENTRY_SIZE)
            .ok_or(Error::OutOfBounds { context: "subsystem table" })?;

        let table_end = property_table_offset
            .checked_add(num_properties as usize * property_entry_size)
            .ok_or(Error::OutOfBounds { context: "property table" })?;
        if table_end > data_offset as usize {
            return Err(Error::Invalid("invalid data offset"));
        }

        Ok(Self {
            data,
            revision,
            num_subsystems,
            num_properties,
            data_offset,
            size,
            property_entry_size,
            property_table_offset,
        })
    }

    /// Borrow the underlying buffer.
    pub fn as_bytes(&self) -> &[u8] {
        self.data.as_ref()
    }

    /// Consume the parser and return the backing storage.
    pub fn into_inner(self) -> D {
        self.data
    }

    /// Backup format revision (2 or 4).
    pub fn revision(&self) -> u8 {
        self.revision
    }

    /// Total number of meaningful bytes (`dataOffset + dataSize`).
    ///
    /// Trailing bytes beyond this are padding and are excluded from the
    /// checksum.
    pub fn size(&self) -> usize {
        self.size
    }

    /// The region string from the header (e.g. `"EU_"`).
    pub fn region(&self) -> String {
        latin1_trim_nul(&self.data.as_ref()[0xc0..0xc0 + 0x20])
    }

    /// The `id1` header field, the camera's backup write-protection flag.
    pub fn id1(&self) -> Result<u32> {
        parse32le(&self.data.as_ref()[ID1_OFFSET..ID1_OFFSET + 4])
    }

    /// The checksum is a plain byte sum, excluding the checksum field itself.
    fn calc_checksum(data: &[u8]) -> u32 {
        let mut sum: u32 = 0;
        for (i, &b) in data.iter().enumerate() {
            if (CHECKSUM_OFFSET..CHECKSUM_OFFSET + 4).contains(&i) {
                continue;
            }
            sum = sum.wrapping_add(b as u32);
        }
        sum
    }

    fn read_subsystem(&self, i: u32) -> Result<(u16, u32)> {
        if i >= self.num_subsystems {
            return Err(Error::Invalid("invalid subsystem id"));
        }
        let off = HEADER_SIZE + i as usize * SUBSYSTEM_ENTRY_SIZE;
        let entry = self
            .data
            .as_ref()
            .get(off..off + SUBSYSTEM_ENTRY_SIZE)
            .ok_or(Error::OutOfBounds { context: "subsystem entry" })?;
        let num_props = parse16le(&entry[0..2])?;
        let ptr = parse32le(&entry[2..6])?;
        if ptr as u64 + num_props as u64 > self.num_properties as u64 {
            return Err(Error::Invalid("invalid subsystem"));
        }
        Ok((num_props, ptr))
    }

    /// Byte offset of a property's entry in the property table.
    fn property_entry_offset(&self, id: u32) -> Result<usize> {
        let (sub_num_props, sub_ptr) = self.read_subsystem(id >> 16)?;
        let i = id & 0xffff;
        if i >= sub_num_props as u32 {
            return Err(Error::Invalid("invalid property id"));
        }
        Ok(self.property_table_offset + (sub_ptr as usize + i as usize) * self.property_entry_size)
    }

    fn property_table_entry(&self, id: u32) -> Result<(u16, u32)> {
        let off = self.property_entry_offset(id)?;
        let entry = self
            .data
            .as_ref()
            .get(off..off + self.property_entry_size)
            .ok_or(Error::OutOfBounds { context: "property entry" })?;
        let (attr, ptr_bytes) = if self.revision >= 4 {
            (parse16le(&entry[0..2])?, &entry[2..6])
        } else {
            (entry[0] as u16, &entry[1..5])
        };
        Ok((attr, parse32le(ptr_bytes)?))
    }

    /// Resolve a property's `(attr, size, max_size, offset)`.
    fn resolve_property(&self, id: u32) -> Result<(u16, usize, usize, usize)> {
        let (attr, ptr) = self.property_table_entry(id)?;
        if ptr == 0xffff_ffff {
            return Err(Error::Invalid("invalid property"));
        }
        let mut size = (ptr >> 24) as usize;
        let mut max_size = size;
        let mut offset = (ptr & 0xff_ffff) as usize;

        if offset < self.data_offset as usize {
            return Err(Error::Invalid("invalid offset"));
        }

        let buf = self.data.as_ref();
        if size == 0xff {
            let s = parse16le(
                buf.get(offset..offset + 2)
                    .ok_or(Error::OutOfBounds { context: "oversize prop" })?,
            )? as usize;
            size = s;
            max_size = s;
            offset += 2;
        } else if size == 0 {
            let hdr = buf
                .get(offset..offset + 4)
                .ok_or(Error::OutOfBounds { context: "varsize prop" })?;
            size = parse16le(&hdr[0..2])? as usize;
            max_size = parse16le(&hdr[2..4])? as usize;
            offset += 4;
        }

        if size > max_size || offset + max_size > self.size {
            return Err(Error::Invalid("invalid size"));
        }
        Ok((attr, size, max_size, offset))
    }

    /// Read a single property by composite id (`subsystem << 16 | index`).
    pub fn get_property(&self, id: u32) -> Result<Property> {
        let (attr, size, max_size, offset) = self.resolve_property(id)?;
        let buf = self.data.as_ref();
        let data = buf[offset..offset + size].to_vec();
        // An array has no factory value behind it: reading one there would
        // return the bytes of whichever property follows (found by LAK132).
        let reset_data = if attr & attr::ARRAY_DATA == 0 && attr & attr::HAS_RESET != 0 {
            let r = offset + max_size;
            buf.get(r..r + size).map(|s| s.to_vec())
        } else {
            None
        };
        Ok(Property { attr, data, reset_data })
    }

    /// Iterate over all present `(id, Property)` pairs.
    pub fn list_properties(&self) -> Result<Vec<(u32, Property)>> {
        let mut out = Vec::new();
        for i in 0..self.num_subsystems {
            let (num_props, _) = self.read_subsystem(i)?;
            for j in 0..num_props as u32 {
                let id = (i << 16) | j;
                if let Ok((_, ptr)) = self.property_table_entry(id)
                    && ptr != 0xffff_ffff
                {
                    out.push((id, self.get_property(id)?));
                }
            }
        }
        Ok(out)
    }
}

impl<D: AsRef<[u8]> + AsMut<[u8]>> BackupFile<D> {
    /// Overwrite a property's value in place.
    ///
    /// The new value must have exactly the property's current size; backup
    /// properties are fixed-length slots and growing one would corrupt its
    /// neighbours.
    pub fn set_property(&mut self, id: u32, data: &[u8]) -> Result<()> {
        let (_, size, _, offset) = self.resolve_property(id)?;
        if data.len() != size {
            return Err(Error::Invalid("wrong data size"));
        }
        self.data.as_mut()[offset..offset + size].copy_from_slice(data);
        Ok(())
    }

    /// Replace a property's attribute bitfield, keeping its data pointer.
    ///
    /// Used to temporarily clear the read-only bit (`0x01`) so a value can be
    /// written, as the reference implementation does.
    pub fn set_property_attr(&mut self, id: u32, attr: u16) -> Result<()> {
        let off = self.property_entry_offset(id)?;
        let entry_size = self.property_entry_size;
        let buf = self
            .data
            .as_mut()
            .get_mut(off..off + entry_size)
            .ok_or(Error::OutOfBounds { context: "property entry" })?;
        if self.revision >= 4 {
            buf[0..2].copy_from_slice(&attr.to_le_bytes());
        } else {
            if attr > u8::MAX as u16 {
                return Err(Error::Invalid("attribute too large for revision 2"));
            }
            buf[0] = attr as u8;
        }
        Ok(())
    }

    /// Set the `id1` write-protection flag.
    pub fn set_id1(&mut self, enable: bool) {
        let value = dump32le(u32::from(enable));
        self.data.as_mut()[ID1_OFFSET..ID1_OFFSET + 4].copy_from_slice(&value);
    }

    /// Recompute and store the header checksum.
    ///
    /// Must be called after any edit; the camera rejects a backup whose
    /// checksum does not match.
    pub fn update_checksum(&mut self) {
        let sum = Self::calc_checksum(&self.data.as_ref()[..self.size]);
        let bytes = dump32le(sum);
        self.data.as_mut()[CHECKSUM_OFFSET..CHECKSUM_OFFSET + 4].copy_from_slice(&bytes);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a minimal but structurally valid revision-4 backup with a single
    /// subsystem holding two fixed-size properties.
    fn sample_backup() -> Vec<u8> {
        let num_subsystems: u32 = 1;
        let num_properties: u32 = 2;
        let data_offset: u32 = 0x200;
        let data_size: u32 = 0x40;
        let mut buf = vec![0u8; (data_offset + data_size) as usize];

        buf[12..16].copy_from_slice(b"BK4\0");
        buf[16..20].copy_from_slice(&num_subsystems.to_le_bytes());
        buf[20..24].copy_from_slice(&num_properties.to_le_bytes());
        buf[24..28].copy_from_slice(&data_offset.to_le_bytes());
        buf[28..32].copy_from_slice(&data_size.to_le_bytes());
        buf[0xc0..0xc4].copy_from_slice(b"XX_U");

        // Subsystem 0: two properties starting at table index 0.
        buf[HEADER_SIZE..HEADER_SIZE + 2].copy_from_slice(&2u16.to_le_bytes());
        buf[HEADER_SIZE + 2..HEADER_SIZE + 6].copy_from_slice(&0u32.to_le_bytes());

        // Property entries: attr (u16) + ptr (u32) = (size << 24) | offset.
        let table = HEADER_SIZE + SUBSYSTEM_ENTRY_SIZE;
        let entries: [(u16, u32, u32); 2] = [(0x00, 3, 0x200), (0x01, 2, 0x210)];
        for (i, (attr, size, off)) in entries.iter().enumerate() {
            let e = table + i * 6;
            buf[e..e + 2].copy_from_slice(&attr.to_le_bytes());
            buf[e + 2..e + 6].copy_from_slice(&((size << 24) | off).to_le_bytes());
        }

        buf[0x200..0x203].copy_from_slice(&[0, 29, 50]);
        buf[0x210..0x212].copy_from_slice(&[0x2c, 0x01]);

        let end = (data_offset + data_size) as usize;
        let sum = BackupFile::<&[u8]>::calc_checksum(&buf[..end]);
        buf[CHECKSUM_OFFSET..CHECKSUM_OFFSET + 4].copy_from_slice(&sum.to_le_bytes());
        buf
    }

    #[test]
    fn rejects_short_buffer() {
        assert!(BackupFile::new(&[0u8; 10][..]).is_err());
    }

    #[test]
    fn rejects_bad_revision_marker() {
        let mut buf = vec![0u8; HEADER_SIZE];
        buf[12..16].copy_from_slice(b"BK9\0"); // revision 9 -> invalid
        assert!(BackupFile::new(&buf).is_err());
    }

    #[test]
    fn reads_sample_properties() {
        let buf = sample_backup();
        let bf = BackupFile::new(&buf).unwrap();
        assert_eq!(bf.revision(), 4);
        assert_eq!(bf.region(), "XX_U");
        assert_eq!(bf.get_property(0).unwrap().data, vec![0, 29, 50]);
        assert_eq!(bf.get_property(1).unwrap().data, vec![0x2c, 0x01]);
        assert_eq!(bf.list_properties().unwrap().len(), 2);
    }

    #[test]
    fn writes_property_and_refreshes_checksum() {
        let mut bf = BackupFile::new(sample_backup()).unwrap();
        bf.set_property(0, &[13, 1, 0]).unwrap();
        bf.update_checksum();
        let buf = bf.into_inner();
        // Re-parsing succeeds only if the checksum was updated correctly.
        let bf = BackupFile::new(&buf).unwrap();
        assert_eq!(bf.get_property(0).unwrap().data, vec![13, 1, 0]);
    }

    #[test]
    fn rejects_property_write_of_wrong_size() {
        let mut bf = BackupFile::new(sample_backup()).unwrap();
        assert!(bf.set_property(0, &[1, 2]).is_err());
        assert!(bf.set_property(0, &[1, 2, 3, 4]).is_err());
    }

    #[test]
    fn stale_checksum_is_rejected_on_reparse() {
        let mut bf = BackupFile::new(sample_backup()).unwrap();
        bf.set_property(0, &[1, 2, 3]).unwrap();
        let buf = bf.into_inner();
        assert!(matches!(BackupFile::new(&buf), Err(Error::Checksum)));
    }

    #[test]
    fn clears_and_restores_the_read_only_attribute() {
        let mut bf = BackupFile::new(sample_backup()).unwrap();
        assert_eq!(bf.get_property(1).unwrap().attr, 0x01);
        bf.set_property_attr(1, 0x00).unwrap();
        assert_eq!(bf.get_property(1).unwrap().attr, 0x00);
        // The data pointer must survive an attribute rewrite.
        assert_eq!(bf.get_property(1).unwrap().data, vec![0x2c, 0x01]);
        bf.set_property_attr(1, 0x01).unwrap();
        assert_eq!(bf.get_property(1).unwrap().attr, 0x01);
    }

    #[test]
    fn reads_reset_data_only_for_resettable_non_arrays() {
        let mut buf = sample_backup();
        let table = HEADER_SIZE + SUBSYSTEM_ENTRY_SIZE;
        let rewrite = |buf: &mut Vec<u8>, attr: u16| {
            buf[table..table + 2].copy_from_slice(&attr.to_le_bytes());
            let sum = BackupFile::<&[u8]>::calc_checksum(&buf[..0x240]);
            buf[CHECKSUM_OFFSET..CHECKSUM_OFFSET + 4].copy_from_slice(&sum.to_le_bytes());
        };

        rewrite(&mut buf, 0x40);
        let reset = BackupFile::new(&buf).unwrap().get_property(0).unwrap().reset_data;
        assert_eq!(reset, Some(vec![0, 0, 0]));

        // Same reset bit, but an array: the bytes behind it belong to the next
        // property and must not be reported as a factory value.
        rewrite(&mut buf, 0xc0);
        let reset = BackupFile::new(&buf).unwrap().get_property(0).unwrap().reset_data;
        assert_eq!(reset, None);
    }

    #[test]
    fn names_the_attribute_bits() {
        assert_eq!(
            attr::names(0xe8),
            vec!["CALLBACKS", "HAS_RESET_3", "HAS_RESET_4", "ARRAY_DATA"]
        );
        assert!(attr::names(0).is_empty());
    }

    #[test]
    fn toggles_the_id1_protection_flag() {
        let mut bf = BackupFile::new(sample_backup()).unwrap();
        assert_eq!(bf.id1().unwrap(), 0);
        bf.set_id1(true);
        assert_eq!(bf.id1().unwrap(), 1);
        bf.set_id1(false);
        assert_eq!(bf.id1().unwrap(), 0);
    }
}
