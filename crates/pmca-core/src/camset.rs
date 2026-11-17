//! Read-only reader for the settings file a camera writes with Save/Load
//! Settings (`PRIVATE/SONY/CAMSET/CAMSET*.DAT`).
//!
//! The file is not [`crate::backup`]'s image. It holds the settings a user can
//! reach in the menu, each under a 16-bit record number, whereas `Backup.bin`
//! holds the camera's whole settings store under 32-bit property ids. Neither
//! file names the link, but it was measured on an ILCE-7CR: record `N` is
//! property `0x02cf000N` of the same camera's backup. [`bridge`] checks that
//! rule record by record against a given backup instead of assuming it.
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

use crate::backup::{BackupFile, Property, attr};
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

/// The backup subsystem that holds the settings a settings file carries.
pub const BACKUP_SUBSYSTEM: u32 = 0x02cf;

/// How a live record sits in the backup, relative to property `N` of
/// [`BACKUP_SUBSYSTEM`], where `N` is the record number.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Placement {
    /// One property holding the same bytes.
    Same,
    /// One property holding the same number at another width.
    Width,
    /// Element `i` at property `N + i`.
    Consecutive,
    /// Element `i` at property `N + 2i`; each element's memory-set array lies
    /// between two elements.
    Stride2,
    /// Only the first elements were found in one of the two layouts; the rest
    /// sit somewhere else.
    Partial,
    /// Property `N` exists but holds another value.
    Differs,
    /// The backup has no property `N`.
    Missing,
}

impl Placement {
    /// A stable, untranslated name for reports.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Same => "same",
            Self::Width => "width",
            Self::Consecutive => "consecutive",
            Self::Stride2 => "stride2",
            Self::Partial => "partial",
            Self::Differs => "differs",
            Self::Missing => "missing",
        }
    }
}

/// Where one live record was found.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiveLink {
    /// The record number.
    pub id: u16,
    /// How it sits in the backup.
    pub placement: Placement,
    /// The backup property of each element that was found, in element order.
    pub properties: Vec<u32>,
}

/// Where the memory-set values of one element of one record were found.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetLink {
    /// The record number.
    pub id: u16,
    /// The element within the record.
    pub element: u16,
    /// The array property whose element `s` holds memory set `s`, if one was
    /// found unambiguously. Its element 0 is not part of the settings file.
    pub array: Option<u32>,
    /// Whether that array is the property right after the live element.
    pub beside: bool,
}

/// How the records of a settings file line up with a `Backup.bin` of the same
/// camera.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bridge {
    /// One entry per record of the live section.
    pub live: Vec<LiveLink>,
    /// How many memory sets the file holds.
    pub sets: usize,
    /// One entry per element of every record of the first memory set.
    pub set_links: Vec<SetLink>,
    /// How many properties the backup holds in [`BACKUP_SUBSYSTEM`].
    pub backup_properties: usize,
}

/// Set the records of `file` against the properties of `backup`.
///
/// The rule behind it was measured on an ILCE-7CR and reported for an
/// ILCE-7RM5: record `N` of the live section is property `N` of subsystem
/// `0x02cf`, and a memory set is element `s` of an array property, usually the
/// one right after the live value. Values are compared, not assumed, so a
/// backup from another camera or another day shows up as `Differs`.
pub fn bridge<D: AsRef<[u8]>>(file: &Camset, backup: &BackupFile<D>) -> Result<Bridge> {
    let props: BTreeMap<u16, Property> = backup
        .list_properties()?
        .into_iter()
        .filter(|(id, _)| id >> 16 == BACKUP_SUBSYSTEM)
        .map(|(id, p)| (id as u16, p))
        .collect();
    let at = |n: u16, step: u16, i: u16| {
        i.checked_mul(step)
            .and_then(|o| n.checked_add(o))
            .and_then(|k| props.get(&k).map(|p| (k, p)))
    };

    let mut live = Vec::new();
    for r in file.sections.first().into_iter().flatten() {
        let elems: Vec<&[u8]> = r.data.chunks_exact(usize::from(r.elem_size).max(1)).collect();
        let found = |step: u16| -> Vec<u16> {
            elems
                .iter()
                .zip(0u16..)
                .map_while(|(e, i)| at(r.id, step, i).filter(|(_, p)| same_number(&p.data, e)))
                .map(|(k, _)| k)
                .collect()
        };
        let (placement, indices) = if r.count == 1 {
            match props.get(&r.id) {
                None => (Placement::Missing, vec![]),
                Some(p) if p.data == r.data => (Placement::Same, vec![r.id]),
                Some(p) if same_number(&p.data, &r.data) => (Placement::Width, vec![r.id]),
                Some(_) => (Placement::Differs, vec![]),
            }
        } else {
            let (one, two) = (found(1), found(2));
            if one.len() == elems.len() {
                (Placement::Consecutive, one)
            } else if two.len() == elems.len() {
                (Placement::Stride2, two)
            } else if !one.is_empty() || !two.is_empty() {
                (Placement::Partial, if one.len() >= two.len() { one } else { two })
            } else if props.contains_key(&r.id) {
                (Placement::Differs, vec![])
            } else {
                (Placement::Missing, vec![])
            }
        };
        let properties = indices.iter().map(|&k| (BACKUP_SUBSYSTEM << 16) | u32::from(k)).collect();
        live.push(LiveLink { id: r.id, placement, properties });
    }

    let sets = file.sections.len().saturating_sub(1);
    let mut set_links = Vec::new();
    if sets > 0 {
        let width = |p: &Property| p.data.len() / (sets + 1);
        let arrays: Vec<(u16, &Property)> = props
            .iter()
            .filter(|(_, p)| p.attr & attr::ARRAY_DATA != 0 && !p.data.is_empty())
            .filter(|(_, p)| p.data.len() % (sets + 1) == 0)
            .map(|(k, p)| (*k, p))
            .collect();
        let by_set: Vec<BTreeMap<u16, &Record>> =
            file.sections[1..].iter().map(|s| s.iter().map(|r| (r.id, r)).collect()).collect();
        let holds = |p: &Property, id: u16, i: usize| {
            let w = width(p);
            (1..=sets).all(|s| {
                by_set[s - 1]
                    .get(&id)
                    .and_then(|r| element(r, i))
                    .is_some_and(|e| same_number(&p.data[s * w..(s + 1) * w], e))
            })
        };
        let live_at: BTreeMap<u16, &LiveLink> = live.iter().map(|l| (l.id, l)).collect();
        for r in &file.sections[1] {
            let varies = (0..usize::from(r.count)).map(|i| {
                let first = by_set[0].get(&r.id).and_then(|x| element(x, i));
                by_set.iter().any(|m| m.get(&r.id).and_then(|x| element(x, i)) != first)
            });
            for (i, varies) in varies.enumerate() {
                let index = live_at
                    .get(&r.id)
                    .and_then(|l| l.properties.get(i))
                    .map_or_else(|| r.id.checked_add(i as u16), |&id| Some(id as u16));
                let next = index.and_then(|k| k.checked_add(1));
                let beside = next.and_then(|k| props.get(&k)).is_some_and(|p| {
                    p.attr & attr::ARRAY_DATA != 0
                        && p.data.len() % (sets + 1) == 0
                        && holds(p, r.id, i)
                });
                let array = if beside {
                    next
                } else if varies {
                    // Only values that differ between the sets single out one array.
                    let mut hits = arrays.iter().filter(|(_, p)| holds(p, r.id, i));
                    match (hits.next(), hits.next()) {
                        (Some((k, _)), None) => Some(*k),
                        _ => None,
                    }
                } else {
                    None
                };
                set_links.push(SetLink {
                    id: r.id,
                    element: i as u16,
                    array: array.map(|k| (BACKUP_SUBSYSTEM << 16) | u32::from(k)),
                    beside,
                });
            }
        }
    }
    Ok(Bridge { live, sets, set_links, backup_properties: props.len() })
}

/// Element `i` of a record, if it has one.
fn element(r: &Record, i: usize) -> Option<&[u8]> {
    let size = usize::from(r.elem_size);
    r.data.get(i * size..(i + 1) * size)
}

/// Whether two little-endian integers of possibly different widths hold the
/// same number: the wider one is the narrower one zero- or sign-extended.
fn same_number(a: &[u8], b: &[u8]) -> bool {
    let (short, long) = if a.len() <= b.len() { (a, b) } else { (b, a) };
    let Some(&top) = short.last() else { return long.is_empty() };
    if short != &long[..short.len()] {
        return false;
    }
    let rest = &long[short.len()..];
    rest.iter().all(|&x| x == 0) || (top & 0x80 != 0 && rest.iter().all(|&x| x == 0xff))
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

    /// A revision-4 backup whose subsystem `0x02cf` holds `props` as
    /// `(index, attr, value)`; every other index has no entry.
    fn backup(props: &[(u16, u16, &[u8])]) -> Vec<u8> {
        const SUBSYSTEMS: usize = 0x2d0;
        let count = usize::from(props.iter().map(|p| p.0).max().unwrap()) + 1;
        let table = 0x100 + SUBSYSTEMS * 6;
        let data_offset = (table + count * 6).next_multiple_of(0x100);
        let mut buf = vec![0u8; data_offset];
        buf[12..16].copy_from_slice(b"BK4\0");
        buf[16..20].copy_from_slice(&(SUBSYSTEMS as u32).to_le_bytes());
        buf[20..24].copy_from_slice(&(count as u32).to_le_bytes());
        buf[24..28].copy_from_slice(&(data_offset as u32).to_le_bytes());
        let sub = 0x100 + 0x2cf * 6;
        buf[sub..sub + 2].copy_from_slice(&(count as u16).to_le_bytes());
        for i in 0..count {
            let e = table + i * 6;
            buf[e + 2..e + 6].copy_from_slice(&u32::MAX.to_le_bytes());
        }
        for &(index, attr, value) in props {
            let e = table + usize::from(index) * 6;
            let ptr = ((value.len() as u32) << 24) | buf.len() as u32;
            buf[e..e + 2].copy_from_slice(&attr.to_le_bytes());
            buf[e + 2..e + 6].copy_from_slice(&ptr.to_le_bytes());
            buf.extend_from_slice(value);
        }
        let size = (buf.len() - data_offset) as u32;
        buf[28..32].copy_from_slice(&size.to_le_bytes());
        let sum = buf
            .iter()
            .enumerate()
            .filter(|(i, _)| !(0x20..0x24).contains(i))
            .fold(0u32, |s, (_, &b)| s.wrapping_add(u32::from(b)));
        buf[0x20..0x24].copy_from_slice(&sum.to_le_bytes());
        buf
    }

    #[test]
    fn records_are_found_in_the_backup_with_their_memory_sets() {
        const ARRAY: u16 = attr::ARRAY_DATA;
        let file = parse(&build(&[
            &[
                (0x10, 1, &[5]),
                (0x20, 4, &[2, 0, 0, 0]),
                (0x30, 1, &[1, 2]),
                (0x40, 1, &[9]),
                (0x50, 1, &[1]),
                (0x60, 1, &[1]),
            ],
            &[(0x10, 1, &[7]), (0x30, 1, &[4, 5]), (0x60, 1, &[3])],
            &[(0x10, 1, &[8]), (0x30, 1, &[6, 7]), (0x60, 1, &[4])],
        ]))
        .unwrap();
        let image = backup(&[
            (0x10, 0, &[5]),
            (0x11, ARRAY, &[5, 7, 8]),
            (0x20, 0, &[2]),
            (0x30, 0, &[1]),
            (0x31, ARRAY, &[1, 4, 6]),
            (0x32, 0, &[2]),
            (0x33, ARRAY, &[2, 5, 7]),
            (0x40, 0, &[3]),
            (0x60, 0, &[1]),
            (0x70, ARRAY, &[1, 3, 4]),
        ]);
        let b = bridge(&file, &BackupFile::new(&image).unwrap()).unwrap();

        let placements: Vec<_> = b.live.iter().map(|l| (l.id, l.placement)).collect();
        assert_eq!(
            placements,
            [
                (0x10, Placement::Same),
                (0x20, Placement::Width),
                (0x30, Placement::Stride2),
                (0x40, Placement::Differs),
                (0x50, Placement::Missing),
                (0x60, Placement::Same),
            ]
        );
        assert_eq!(b.live[2].properties, [0x02cf_0030, 0x02cf_0032]);
        assert_eq!(b.sets, 2);

        let sets: Vec<_> =
            b.set_links.iter().map(|s| (s.id, s.element, s.array, s.beside)).collect();
        assert_eq!(
            sets,
            [
                (0x10, 0, Some(0x02cf_0011), true),
                (0x30, 0, Some(0x02cf_0031), true),
                (0x30, 1, Some(0x02cf_0033), true),
                (0x60, 0, Some(0x02cf_0070), false),
            ]
        );
    }

    #[test]
    fn numbers_match_across_widths_but_not_across_signs() {
        assert!(same_number(&[2], &[2, 0, 0, 0]));
        assert!(same_number(&[0xfe, 0xff], &[0xfe]));
        assert!(!same_number(&[0xfe, 0xff], &[0xfe, 0]));
        assert!(!same_number(&[1], &[2]));
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
