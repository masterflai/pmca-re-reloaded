//! Read-only reader for the settings file a camera writes with Save/Load
//! Settings (`PRIVATE/SONY/CAMSET/CAMSET*.DAT`).
//!
//! The file is not [`crate::backup`]'s image. It holds the settings a user can
//! reach in the menu, each under a 16-bit record number, whereas `Backup.bin`
//! holds the camera's whole settings store under 32-bit property ids. Nothing
//! in either file links the two numberings; this reader exists so that two
//! files saved before and after one menu change can be compared, and the
//! record that moved set against the backup property that moved with it.
//!
//! The layout is the one the camset project documents
//! (<https://github.com/camset-dev/camset>, `docs/formats/sony_camset.ksy`),
//! verified there on the ILCE-1M2 and taken to be shared by Sony bodies of the
//! same menu generation. Both of its checksums are checked, so a file from a
//! body that lays the container out differently is rejected rather than misread.
//!
//! Records are not named. The numbers differ between models and firmware
//! versions, and a name measured on one body would read as documentation for
//! all of them.
//!
//! A settings file identifies its owner: it can hold a title name, the names
//! and serial numbers of lenses registered for preset focus, and network
//! settings.

use std::collections::BTreeMap;

use crate::util::{parse16le, parse32le};
use crate::{Error, Result};

/// The bytes every settings file starts with.
pub const MAGIC: &[u8] = b"SONY1";

/// Size of the header; the first section starts here.
const HEADER_SIZE: usize = 0x3c;

/// Offset of the header checksum, which covers every byte before it.
const HEADER_CHECKSUM_AT: usize = 0x39;

/// The constant second field of every record.
const RECORD_MARKER: u16 = 0x02cf;

/// Record id, marker, element size and element count.
const RECORD_HEAD: usize = 8;

/// The trailing body checksum.
const TRAILER: usize = 4;

/// A parsed settings file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Camset {
    /// The model that saved the file, such as `ILCE-1M2`.
    pub model: String,
    /// When it was saved, as the camera wrote it (`YYYYMMDD_HHMMSS`).
    pub saved: String,
    /// Firmware major and minor version.
    pub firmware: [u8; 2],
    /// The NTSC/PAL selector: 0 for NTSC, 1 for PAL.
    pub video_system: u8,
    /// Section 0 holds the live settings, every further section one memory set.
    pub sections: Vec<Vec<Record>>,
}

/// One setting's storage: `count` elements of `elem_size` bytes each.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record {
    /// Where the record begins in the file, for cross-checking by hand.
    pub offset: usize,
    /// The record number.
    pub id: u16,
    /// Bytes per element.
    pub elem_size: u16,
    /// Number of elements.
    pub count: u16,
    /// The value bytes, `elem_size * count` of them.
    pub data: Vec<u8>,
}

impl Record {
    /// The elements as little-endian signed integers, if they have an
    /// integer's width.
    #[must_use]
    pub fn values(&self) -> Option<Vec<i64>> {
        let size = usize::from(self.elem_size);
        if !matches!(size, 1 | 2 | 4 | 8) {
            return None;
        }
        Some(
            self.data
                .chunks_exact(size)
                .map(|c| {
                    let mut b = [if c[size - 1] & 0x80 != 0 { 0xff } else { 0 }; 8];
                    b[..size].copy_from_slice(c);
                    i64::from_le_bytes(b)
                })
                .collect(),
        )
    }
}

/// A record that differs between two files.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Change<'a> {
    /// The section both records sit in.
    pub section: usize,
    /// The record number.
    pub id: u16,
    /// The record in the first file, if it has one.
    pub before: Option<&'a Record>,
    /// The record in the second file, if it has one.
    pub after: Option<&'a Record>,
}

/// Parse a settings file, checking both checksums.
pub fn parse(data: &[u8]) -> Result<Camset> {
    if is_older_format(data) {
        return Err(Error::Invalid("settings file in the older format, not supported"));
    }
    if data.len() < HEADER_SIZE + TRAILER {
        return Err(Error::OutOfBounds { context: "settings file header" });
    }
    if !data.starts_with(MAGIC) {
        return Err(Error::BadMagic { expected: "SONY1" });
    }
    if header_checksum(data) != data[HEADER_CHECKSUM_AT] {
        return Err(Error::Checksum);
    }
    let end = data.len() - TRAILER;
    if body_checksum(&data[HEADER_SIZE..end]) != parse32le(&data[end..])? {
        return Err(Error::Checksum);
    }

    let mut sections = Vec::new();
    let mut pos = HEADER_SIZE;
    while pos < end {
        let count = parse32le(&data[pos..end])? as usize;
        pos += 4;
        if count > (end - pos) / RECORD_HEAD {
            return Err(Error::OutOfBounds { context: "settings file record count" });
        }
        let mut records = Vec::with_capacity(count);
        for _ in 0..count {
            if RECORD_HEAD > end - pos {
                return Err(Error::OutOfBounds { context: "settings file record" });
            }
            let head = &data[pos..pos + RECORD_HEAD];
            if parse16le(&head[2..])? != RECORD_MARKER {
                return Err(Error::Invalid("settings file record marker"));
            }
            let elem_size = parse16le(&head[4..])?;
            let elem_count = parse16le(&head[6..])?;
            let start = pos + RECORD_HEAD;
            let len = usize::from(elem_size) * usize::from(elem_count);
            if len > end - start {
                return Err(Error::OutOfBounds { context: "settings file record value" });
            }
            records.push(Record {
                offset: pos,
                id: parse16le(head)?,
                elem_size,
                count: elem_count,
                data: data[start..start + len].to_vec(),
            });
            pos = start + len;
        }
        sections.push(records);
    }

    Ok(Camset {
        model: text(&data[0x05..0x15]),
        saved: text(&data[0x15..0x35]),
        firmware: [data[0x36], data[0x37]],
        video_system: data[0x38],
        sections,
    })
}

/// Whether `data` is a settings file in the older, fixed-layout format that
/// bodies with the menu from before 2021 (ILCE-7RM4, ILCE-9M2) write.
///
/// Those files are about 9 KB, start with `SON` and a `!` in the fifth byte
/// (the fourth is `Y` or 0) and carry no numbered records, so this reader
/// cannot interpret them.
#[must_use]
pub fn is_older_format(data: &[u8]) -> bool {
    data.len() > 4 && data.starts_with(b"SON") && data[4] == b'!'
}

/// Every record that differs between `a` and `b`, section by section and by
/// ascending record number.
///
/// Records are paired by section and number rather than by position, so a
/// record present in only one file shows up with the other side empty.
#[must_use]
pub fn changes<'a>(a: &'a Camset, b: &'a Camset) -> Vec<Change<'a>> {
    let mut out = Vec::new();
    for section in 0..a.sections.len().max(b.sections.len()) {
        let index = |file: &'a Camset| -> BTreeMap<u16, &'a Record> {
            file.sections.get(section).into_iter().flatten().map(|r| (r.id, r)).collect()
        };
        let (before, after) = (index(a), index(b));
        let mut ids: Vec<u16> = before.keys().chain(after.keys()).copied().collect();
        ids.sort_unstable();
        ids.dedup();
        for id in ids {
            let (x, y) = (before.get(&id).copied(), after.get(&id).copied());
            let same = matches!((x, y), (Some(x), Some(y))
                if (x.elem_size, x.count, &x.data) == (y.elem_size, y.count, &y.data));
            if !same {
                out.push(Change { section, id, before: x, after: y });
            }
        }
    }
    out
}

/// Low byte of the sum of `byte[p] << (p mod 8)` over everything before the
/// header checksum.
fn header_checksum(data: &[u8]) -> u8 {
    let sum = data[..HEADER_CHECKSUM_AT]
        .iter()
        .enumerate()
        .fold(0u32, |sum, (p, &b)| sum.wrapping_add(u32::from(b) << (p % 8)));
    sum as u8
}

/// The sum of every byte between the header and the trailer.
fn body_checksum(body: &[u8]) -> u32 {
    body.iter().fold(0u32, |sum, &b| sum.wrapping_add(u32::from(b)))
}

/// A NUL-terminated ASCII field; anything after the first NUL is padding.
fn text(field: &[u8]) -> String {
    let end = field.iter().position(|&b| b == 0).unwrap_or(field.len());
    field[..end].iter().map(|&b| b as char).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A file with the given sections of `(id, elem_size, values)` records.
    fn build(sections: &[&[(u16, u16, &[u8])]]) -> Vec<u8> {
        let mut data = vec![0u8; HEADER_SIZE];
        data[..5].copy_from_slice(MAGIC);
        data[0x05..0x0d].copy_from_slice(b"ILCE-7CR");
        data[0x15..0x24].copy_from_slice(b"20261110_120000");
        data[0x36] = 2;
        data[0x38] = 1;
        data[HEADER_CHECKSUM_AT] = header_checksum(&data);
        for records in sections {
            data.extend_from_slice(&(records.len() as u32).to_le_bytes());
            for &(id, size, bytes) in *records {
                data.extend_from_slice(&id.to_le_bytes());
                data.extend_from_slice(&RECORD_MARKER.to_le_bytes());
                data.extend_from_slice(&size.to_le_bytes());
                data.extend_from_slice(&((bytes.len() / usize::from(size)) as u16).to_le_bytes());
                data.extend_from_slice(bytes);
            }
        }
        let sum = body_checksum(&data[HEADER_SIZE..]);
        data.extend_from_slice(&sum.to_le_bytes());
        data
    }

    #[test]
    fn reads_the_header_and_every_section() {
        let data = build(&[&[(0x18, 1, &[1]), (0x0407, 4, &[0xfe, 0xff, 0xff, 0xff])], &[]]);
        let file = parse(&data).unwrap();
        assert_eq!(file.model, "ILCE-7CR");
        assert_eq!(file.saved, "20261110_120000");
        assert_eq!(file.firmware, [2, 0]);
        assert_eq!(file.video_system, 1);
        assert_eq!(file.sections.len(), 2);
        assert_eq!(file.sections[0][0].offset, HEADER_SIZE + 4);
        assert_eq!(file.sections[0][1].values(), Some(vec![-2]));
        assert!(file.sections[1].is_empty());
    }

    #[test]
    fn a_damaged_file_fails_one_of_the_checksums() {
        let good = build(&[&[(0x18, 1, &[1])]]);
        let mut header = good.clone();
        header[0x10] ^= 1;
        assert!(matches!(parse(&header), Err(Error::Checksum)));
        let mut body = good;
        let last = body.len() - TRAILER - 1;
        body[last] ^= 1;
        assert!(matches!(parse(&body), Err(Error::Checksum)));
    }

    #[test]
    fn a_record_count_beyond_the_file_is_rejected_before_allocating() {
        let mut data = build(&[&[]]);
        data[HEADER_SIZE..HEADER_SIZE + 4].copy_from_slice(&u32::MAX.to_le_bytes());
        let end = data.len() - TRAILER;
        let sum = body_checksum(&data[HEADER_SIZE..end]);
        data[end..].copy_from_slice(&sum.to_le_bytes());
        assert!(matches!(parse(&data), Err(Error::OutOfBounds { .. })));
    }

    #[test]
    fn something_else_is_not_mistaken_for_a_settings_file() {
        assert!(matches!(parse(&[0u8; 0x80]), Err(Error::BadMagic { .. })));
        assert!(matches!(parse(b"SONY1"), Err(Error::OutOfBounds { .. })));
    }

    #[test]
    fn the_older_format_is_recognised_as_such() {
        let mut older = vec![0u8; 0x2400];
        older[..14].copy_from_slice(b"SONY!ILCE-7RM4");
        assert!(is_older_format(&older));
        assert!(matches!(parse(&older), Err(Error::Invalid(_))));
        older[3] = 0;
        assert!(is_older_format(&older));
        assert!(!is_older_format(&build(&[&[]])));
    }

    #[test]
    fn changes_pair_records_by_section_and_number() {
        let a = parse(&build(&[&[(1, 1, &[0]), (2, 1, &[5])], &[(1, 1, &[0])]])).unwrap();
        let b =
            parse(&build(&[&[(1, 1, &[1]), (2, 1, &[5]), (3, 1, &[9])], &[(1, 1, &[0])]])).unwrap();
        let found: Vec<_> = changes(&a, &b)
            .iter()
            .map(|c| (c.section, c.id, c.before.is_some(), c.after.is_some()))
            .collect();
        assert_eq!(found, [(0, 1, true, true), (0, 3, false, true)]);
        assert!(changes(&a, &a).is_empty());
    }
}
