//! Reading a whole settings store into one structure, and comparing several.
//!
//! The function analysis in [`crate::analysis`] is the narrow instrument: it
//! changes one menu entry and watches which property follows. This is the wide
//! one. It takes the complete backup region of two or more cameras and sorts
//! every property into a handful of classes — the same everywhere, nearly the
//! same, different, structurally different, apparently unused, missing on at
//! least one body. A front-end paints those classes and the reader sees at a
//! glance where two cameras disagree.
//!
//! # What "empty" means here, and what it does not
//!
//! The parser knows two states: a property has a table entry, or it has none.
//! A property that exists and holds nothing but zeros is as present as any
//! other, and nothing in the format marks it as unused. [`Class::Empty`] is
//! therefore not a fact read off the image but a verdict this module forms:
//! the value is all `0x00` or all `0xFF`, it equals the factory value where
//! there is one, and it reads the same on every camera compared. That is a
//! good guess and no more — `0x00` is also how most settings spell "off" — so
//! a front-end should word it as a suspicion.
//!
//! [`Class::Absent`], by contrast, is a fact: there is no table entry, so
//! there are no bytes either.
//!
//! # Free space
//!
//! [`Dump::free_space`] walks every property's slot and reports what is left
//! over. This answers where unclaimed bytes sit, and it is worth being plain
//! about what that is good for: nothing can be made to happen by writing into
//! a gap. The backup region is a passive store; a byte has an effect only
//! where firmware reads that property id. The gaps are a measurement, an aid
//! to understanding the layout — not a place to put new features.

use std::collections::{BTreeMap, BTreeSet};

use pmca_core::backup::{BackupFile, attr};

use crate::Result;

/// One property as it was found in an image.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DumpEntry {
    /// Composite property id (`subsystem << 16 | index`).
    pub id: u32,
    /// Attribute bitfield.
    pub attr: u16,
    /// First byte of the value in the image.
    pub offset: usize,
    /// Bytes reserved for the value.
    pub max_size: usize,
    /// Total bytes the property claims, factory value included.
    pub slot_size: usize,
    /// The value.
    pub data: Vec<u8>,
    /// The factory value, where the property has one.
    pub reset: Option<Vec<u8>>,
}

impl DumpEntry {
    /// Whether the camera or its user has moved this property off its factory
    /// value. Unknowable where there is no factory value.
    #[must_use]
    pub fn touched(&self) -> bool {
        self.reset.as_ref().is_some_and(|r| r != &self.data)
    }

    /// Whether the value carries no information as far as this module can
    /// tell: nothing but zeros or nothing but `0xFF`, and the factory value,
    /// if any, says the same.
    ///
    /// A judgement, not a reading — see the module documentation.
    #[must_use]
    pub fn looks_empty(&self) -> bool {
        !self.data.is_empty()
            && (self.data.iter().all(|&b| b == 0x00) || self.data.iter().all(|&b| b == 0xff))
            && self.reset.as_ref().is_none_or(|r| r == &self.data)
    }

    /// Whether a value could be written here at all.
    ///
    /// The same two bits the function analysis refuses to write through, so a
    /// candidate found by one is judged the same way by the other.
    #[must_use]
    pub fn writable(&self) -> bool {
        self.attr & (attr::READ_ONLY | attr::PROTECTED) == 0
    }
}

/// Every property of one camera's settings store, with the shape of the image
/// it came from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dump {
    /// The region string from the image header.
    pub region: String,
    /// Backup format revision (2 or 4).
    pub revision: u8,
    /// Meaningful bytes in the image (`dataOffset + dataSize`).
    pub image_size: usize,
    /// Bytes the buffer held, padding beyond `image_size` included.
    pub buffer_size: usize,
    /// First byte of the data region.
    pub data_offset: usize,
    /// First byte after the property table.
    pub table_end: usize,
    /// Number of subsystems the header declares.
    pub num_subsystems: u32,
    /// Number of property table entries the header declares.
    pub num_properties: u32,
    /// Every property that has a table entry, by ascending id.
    pub entries: Vec<DumpEntry>,
}

impl Dump {
    /// Read every property and its address out of a parsed image.
    pub fn from_image<D: AsRef<[u8]>>(image: &BackupFile<D>) -> Result<Self> {
        let mut entries = Vec::new();
        for (id, layout) in image.list_layouts()? {
            let property = image.get_property(id)?;
            entries.push(DumpEntry {
                id,
                attr: layout.attr,
                offset: layout.offset,
                max_size: layout.max_size,
                slot_size: layout.slot_size,
                data: property.data,
                reset: property.reset_data,
            });
        }
        Ok(Self {
            region: image.region(),
            revision: image.revision(),
            image_size: image.size(),
            buffer_size: image.as_bytes().len(),
            data_offset: image.data_offset(),
            table_end: image.table_end(),
            num_subsystems: image.num_subsystems(),
            num_properties: image.num_properties(),
            entries,
        })
    }

    /// The entry for one id, if the camera has it.
    #[must_use]
    pub fn entry(&self, id: u32) -> Option<&DumpEntry> {
        self.entries.binary_search_by_key(&id, |e| e.id).ok().map(|i| &self.entries[i])
    }

    /// How many properties each subsystem holds.
    ///
    /// A fingerprint of the model generation: a subsystem one body has and
    /// another does not is the clearest sign that an id from one camera cannot
    /// simply be looked for on the other.
    #[must_use]
    pub fn subsystems(&self) -> BTreeMap<u16, usize> {
        let mut out = BTreeMap::new();
        for entry in &self.entries {
            *out.entry((entry.id >> 16) as u16).or_default() += 1;
        }
        out
    }

    /// Which bytes of the image are claimed, and which are not.
    #[must_use]
    pub fn free_space(&self) -> FreeSpace {
        let data_size = self.image_size - self.data_offset;

        let mut claimed: Vec<(usize, usize)> = self
            .entries
            .iter()
            .map(|e| {
                let start = e.offset.min(self.image_size);
                (start, (start + e.slot_size).min(self.image_size))
            })
            .filter(|(start, end)| end > start)
            .collect();
        claimed.sort_unstable();

        // Merge first: two properties may share a slot, and overlapping ranges
        // would otherwise be counted twice and produce a negative remainder.
        let mut merged: Vec<(usize, usize)> = Vec::with_capacity(claimed.len());
        for (start, end) in claimed {
            match merged.last_mut() {
                Some(last) if start <= last.1 => last.1 = last.1.max(end),
                _ => merged.push((start, end)),
            }
        }

        let mut gaps = Vec::new();
        let mut cursor = self.data_offset;
        for &(start, end) in &merged {
            if start > cursor {
                gaps.push(Gap { offset: cursor, size: start - cursor });
            }
            cursor = cursor.max(end);
        }
        if cursor < self.image_size {
            gaps.push(Gap { offset: cursor, size: self.image_size - cursor });
        }

        let used: usize = merged.iter().map(|(s, e)| e - s).sum();
        FreeSpace {
            header: HEADER_SIZE,
            subsystem_table: self.num_subsystems as usize * SUBSYSTEM_ENTRY_SIZE,
            property_table: self
                .table_end
                .saturating_sub(HEADER_SIZE + self.num_subsystems as usize * SUBSYSTEM_ENTRY_SIZE),
            table_reserve: self.data_offset.saturating_sub(self.table_end),
            data_size,
            data_used: used,
            data_slack: self.entries.iter().map(|e| e.max_size.saturating_sub(e.data.len())).sum(),
            largest_gap: gaps.iter().map(|g| g.size).max().unwrap_or(0),
            gaps,
            trailing: self.buffer_size.saturating_sub(self.image_size),
        }
    }
}

/// Size of the image header, repeated here so [`FreeSpace`] can name it.
const HEADER_SIZE: usize = 0x100;
/// Size of one subsystem table entry.
const SUBSYSTEM_ENTRY_SIZE: usize = 6;

/// A run of data-region bytes no property claims.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Gap {
    /// First unclaimed byte.
    pub offset: usize,
    /// How many bytes the run holds.
    pub size: usize,
}

/// How the bytes of one image divide up.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FreeSpace {
    /// Bytes of header.
    pub header: usize,
    /// Bytes of subsystem table.
    pub subsystem_table: usize,
    /// Bytes of property table.
    pub property_table: usize,
    /// Bytes between the end of the table and the data region: room for table
    /// entries that do not exist.
    pub table_reserve: usize,
    /// Bytes in the data region.
    pub data_size: usize,
    /// Data-region bytes claimed by a property.
    pub data_used: usize,
    /// Bytes reserved by variable-size properties beyond what they hold.
    pub data_slack: usize,
    /// Runs of data-region bytes no property claims.
    pub gaps: Vec<Gap>,
    /// Size of the longest such run.
    pub largest_gap: usize,
    /// Bytes past the end the header declares; outside the checksum.
    pub trailing: usize,
}

impl FreeSpace {
    /// Data-region bytes no property claims.
    #[must_use]
    pub fn data_free(&self) -> usize {
        self.gaps.iter().map(|g| g.size).sum()
    }
}

/// How one property compares across the dumps.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Class {
    /// Every camera holds the same value.
    Identical,
    /// The values differ in no more than [`Options::similar_distance`] bytes.
    Similar,
    /// The values differ.
    Different,
    /// The property has different lengths on different cameras, so the values
    /// cannot meaningfully be held side by side.
    Structural,
    /// Present everywhere, the same everywhere, and apparently carrying no
    /// information. A judgement; see the module documentation.
    Empty,
    /// At least one camera has no such property.
    Absent,
    /// Known to change on its own, so a difference here means nothing.
    Noise,
}

impl Class {
    /// A stable name for the class, for reports and front-end styling.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Identical => "identical",
            Self::Similar => "similar",
            Self::Different => "different",
            Self::Structural => "structural",
            Self::Empty => "empty",
            Self::Absent => "absent",
            Self::Noise => "noise",
        }
    }

    /// Every class, in the order a report lists them.
    #[must_use]
    pub fn all() -> [Self; 7] {
        [
            Self::Identical,
            Self::Similar,
            Self::Different,
            Self::Structural,
            Self::Empty,
            Self::Absent,
            Self::Noise,
        ]
    }

    /// The class named by [`Self::as_str`], if the name is one of them.
    #[must_use]
    pub fn named(name: &str) -> Option<Self> {
        Self::all().into_iter().find(|c| c.as_str() == name)
    }
}

/// The most differing bytes that still count as a near miss.
///
/// One by default: a setting stored as a small number differs in its lowest
/// byte and nowhere else, and that is the case worth separating from a value
/// that is different throughout.
pub const DEFAULT_SIMILAR_DISTANCE: usize = 1;

/// What the *shape* of a property suggests, regardless of how it compared.
///
/// A separate axis from [`Class`]: that one records the outcome of the
/// comparison, this one the form of the value. A switch candidate is always
/// also [`Class::Similar`], so making it a class would force a choice between
/// two true statements.
///
/// Both marks are hints and prove nothing. Not every byte that reads `00` or
/// `01` is a setting — the firmware keeps status flags the same way — and a
/// setting need not be a whole byte: one stored as a single bit inside a byte
/// is invisible to this.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Shape {
    /// Nothing about the value suggests a setting.
    Plain,
    /// One byte, every camera reads `00` or `01`, and they disagree: the shape
    /// of an on/off setting. This is how `appMenu` was found — `00` in one
    /// sales region, `01` in the others.
    Switch,
    /// One byte, every value at most `0f`, and they disagree: the shape of a
    /// short list of choices rather than a yes/no, as `palNtsc` is.
    SmallChoice,
}

impl Shape {
    /// A stable name for the mark, for reports and front-end styling.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Plain => "plain",
            Self::Switch => "switch",
            Self::SmallChoice => "choice",
        }
    }
}

/// The largest byte a [`Shape::SmallChoice`] value may hold.
const SMALL_CHOICE_MAX: u8 = 0x0f;

/// What the comparison treats as near, and what it ignores.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Options {
    /// Largest number of differing bytes still called [`Class::Similar`].
    pub similar_distance: usize,
    /// Properties known to change without anybody changing them — counters,
    /// clocks, the record of the last power-on. Nothing is assumed here: the
    /// set is supplied by the caller, and [`drift`] derives one by reading the
    /// same camera twice.
    pub noise: BTreeSet<u32>,
}

impl Default for Options {
    fn default() -> Self {
        Self { similar_distance: DEFAULT_SIMILAR_DISTANCE, noise: BTreeSet::new() }
    }
}

/// One property, seen across every dump.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Row {
    /// Composite property id.
    pub id: u32,
    /// Which class the property fell into.
    pub class: Class,
    /// The value on each camera, in the order the dumps were given; `None`
    /// where that camera has no such property.
    pub values: Vec<Option<Vec<u8>>>,
    /// The common length, where every camera agrees on one.
    pub size: Option<usize>,
    /// The attribute bits, from the first camera that has the property.
    pub attr: Option<u16>,
    /// Whether the cameras disagree about the attribute bits. Unlike a length
    /// difference this does not stop the values being compared, so it is
    /// reported beside the class rather than as one.
    pub attr_differs: bool,
    /// How many byte positions differ at most between any value and the first.
    pub distance: usize,
    /// Whether at least one camera holds something other than its factory
    /// value here.
    pub touched: bool,
    /// Whether the property could be written at all.
    pub writable: bool,
    /// What the form of the value suggests, independent of the class.
    pub shape: Shape,
}

/// Every property of every dump, classified.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Comparison {
    /// One row per property id seen in any dump, by ascending id.
    pub rows: Vec<Row>,
}

impl Comparison {
    /// How many rows fell into each class.
    #[must_use]
    pub fn counts(&self) -> BTreeMap<Class, usize> {
        let mut out: BTreeMap<Class, usize> = Class::all().into_iter().map(|c| (c, 0)).collect();
        for row in &self.rows {
            *out.entry(row.class).or_default() += 1;
        }
        out
    }
}

/// Sort every property of every dump into a class.
///
/// Works with a single dump as well: everything is then either
/// [`Class::Identical`] or [`Class::Empty`], which is still the quickest way
/// to see how much of one camera's store is in use.
#[must_use]
pub fn compare(dumps: &[&Dump], options: &Options) -> Comparison {
    let ids: BTreeSet<u32> = dumps.iter().flat_map(|d| d.entries.iter().map(|e| e.id)).collect();
    let rows = ids.into_iter().map(|id| row(id, dumps, options)).collect();
    Comparison { rows }
}

fn row(id: u32, dumps: &[&Dump], options: &Options) -> Row {
    let found: Vec<Option<&DumpEntry>> = dumps.iter().map(|d| d.entry(id)).collect();
    let present: Vec<&DumpEntry> = found.iter().flatten().copied().collect();
    let first = present.first().copied();

    let sizes_agree = present.windows(2).all(|w| w[0].data.len() == w[1].data.len());
    let attr_differs = present.windows(2).any(|w| w[0].attr != w[1].attr);
    let values_agree = present.windows(2).all(|w| w[0].data == w[1].data);

    let distance = match first {
        Some(base) if sizes_agree => present
            .iter()
            .map(|e| e.data.iter().zip(&base.data).filter(|(a, b)| a != b).count())
            .max()
            .unwrap_or(0),
        _ => 0,
    };

    let class = if options.noise.contains(&id) {
        Class::Noise
    } else if found.iter().any(Option::is_none) {
        Class::Absent
    } else if !sizes_agree {
        Class::Structural
    } else if values_agree {
        if present.iter().all(|e| e.looks_empty()) { Class::Empty } else { Class::Identical }
    } else if distance <= options.similar_distance {
        Class::Similar
    } else {
        Class::Different
    };

    let writable = present.iter().all(|e| e.writable()) && !present.is_empty();
    Row {
        id,
        class,
        values: found.iter().map(|e| e.map(|e| e.data.clone())).collect(),
        size: if sizes_agree { first.map(|e| e.data.len()) } else { None },
        attr: first.map(|e| e.attr),
        attr_differs,
        distance,
        touched: present.iter().any(|e| e.touched()),
        shape: shape(&present, writable, values_agree),
        writable,
    }
}

/// The mark for a row, from the values every camera holds.
fn shape(present: &[&DumpEntry], writable: bool, values_agree: bool) -> Shape {
    // Agreeing cameras say nothing about what a value is for, and a value that
    // cannot be written is not a setting anyone could reach.
    if values_agree || !writable || present.len() < 2 {
        return Shape::Plain;
    }
    if !present.iter().all(|e| e.data.len() == 1) {
        return Shape::Plain;
    }
    let bytes = present.iter().filter_map(|e| e.data.first().copied());
    let highest = bytes.clone().max().unwrap_or(u8::MAX);
    if highest <= 1 {
        Shape::Switch
    } else if highest <= SMALL_CHOICE_MAX {
        Shape::SmallChoice
    } else {
        Shape::Plain
    }
}

/// The properties that differ between two readings of the *same* camera.
///
/// Nothing was changed between them, so every difference is the camera's own
/// doing: shutter counts, the clock, the record of the last power-on. Feeding
/// the result into [`Options::noise`] keeps those out of a comparison that is
/// meant to be about the difference between two bodies.
///
/// This is the only honest way to obtain such a list. A fixed one shipped with
/// the program would be a guess about models nobody here has measured.
#[must_use]
pub fn drift(before: &Dump, after: &Dump) -> BTreeSet<u32> {
    before
        .entries
        .iter()
        .filter(|e| after.entry(e.id).is_none_or(|other| other.data != e.data))
        .map(|e| e.id)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(id: u32, data: &[u8]) -> DumpEntry {
        DumpEntry {
            id,
            attr: 0,
            offset: 0x200 + id as usize * 0x10,
            max_size: data.len(),
            slot_size: data.len(),
            data: data.to_vec(),
            reset: None,
        }
    }

    fn dump(entries: Vec<DumpEntry>) -> Dump {
        Dump {
            region: "XX_U".into(),
            revision: 4,
            image_size: 0x400,
            buffer_size: 0x400,
            data_offset: 0x200,
            table_end: 0x120,
            num_subsystems: 1,
            num_properties: entries.len() as u32,
            entries,
        }
    }

    #[test]
    fn equal_values_are_identical_and_zeros_are_empty() {
        let a = dump(vec![entry(0, &[1, 2]), entry(1, &[0, 0])]);
        let b = dump(vec![entry(0, &[1, 2]), entry(1, &[0, 0])]);
        let rows = compare(&[&a, &b], &Options::default()).rows;
        assert_eq!(rows[0].class, Class::Identical);
        assert_eq!(rows[1].class, Class::Empty);
    }

    #[test]
    fn one_differing_byte_is_similar_and_two_are_different() {
        let a = dump(vec![entry(0, &[1, 2, 3]), entry(1, &[1, 2, 3])]);
        let b = dump(vec![entry(0, &[1, 2, 9]), entry(1, &[9, 9, 3])]);
        let rows = compare(&[&a, &b], &Options::default()).rows;
        assert_eq!(rows[0].class, Class::Similar);
        assert_eq!(rows[0].distance, 1);
        assert_eq!(rows[1].class, Class::Different);
        assert_eq!(rows[1].distance, 2);
    }

    #[test]
    fn a_missing_property_is_absent_not_empty() {
        let a = dump(vec![entry(0, &[0, 0])]);
        let b = dump(vec![]);
        let rows = compare(&[&a, &b], &Options::default()).rows;
        assert_eq!(rows[0].class, Class::Absent);
        assert_eq!(rows[0].values, vec![Some(vec![0, 0]), None]);
    }

    #[test]
    fn different_lengths_are_structural() {
        let a = dump(vec![entry(0, &[1])]);
        let b = dump(vec![entry(0, &[1, 0])]);
        let rows = compare(&[&a, &b], &Options::default()).rows;
        assert_eq!(rows[0].class, Class::Structural);
        assert_eq!(rows[0].size, None);
    }

    #[test]
    fn differing_attributes_are_reported_beside_the_class() {
        let a = dump(vec![entry(0, &[1])]);
        let mut second = entry(0, &[1]);
        second.attr = attr::READ_ONLY;
        let b = dump(vec![second]);
        let rows = compare(&[&a, &b], &Options::default()).rows;
        assert_eq!(rows[0].class, Class::Identical);
        assert!(rows[0].attr_differs);
        assert!(!rows[0].writable);
    }

    #[test]
    fn a_noisy_property_is_never_called_different() {
        let a = dump(vec![entry(0, &[1])]);
        let b = dump(vec![entry(0, &[2])]);
        let options = Options { noise: [0].into_iter().collect(), ..Options::default() };
        assert_eq!(compare(&[&a, &b], &options).rows[0].class, Class::Noise);
    }

    #[test]
    fn drift_names_what_moved_on_its_own() {
        let before = dump(vec![entry(0, &[1]), entry(1, &[5])]);
        let after = dump(vec![entry(0, &[1]), entry(1, &[6])]);
        assert_eq!(drift(&before, &after), [1].into_iter().collect());
    }

    #[test]
    fn a_value_equal_to_the_factory_value_is_untouched() {
        let mut e = entry(0, &[7]);
        e.reset = Some(vec![7]);
        assert!(!e.touched());
        e.data = vec![8];
        assert!(e.touched());
    }

    #[test]
    fn a_zero_value_with_a_nonzero_factory_value_is_not_empty() {
        let mut e = entry(0, &[0]);
        e.reset = Some(vec![1]);
        assert!(!e.looks_empty());
        e.reset = Some(vec![0]);
        assert!(e.looks_empty());
    }

    #[test]
    fn free_space_finds_the_hole_between_two_properties() {
        let mut first = entry(0, &[1, 2, 3, 4]);
        first.offset = 0x200;
        first.slot_size = 4;
        let mut second = entry(1, &[9]);
        second.offset = 0x300;
        second.slot_size = 1;
        let d = dump(vec![first, second]);

        let free = d.free_space();
        assert_eq!(free.data_used, 5);
        assert_eq!(
            free.gaps,
            vec![Gap { offset: 0x204, size: 0xfc }, Gap { offset: 0x301, size: 0xff }]
        );
        assert_eq!(free.data_free(), 0xfc + 0xff);
        assert_eq!(free.data_size, 0x200);
        assert_eq!(free.largest_gap, 0xff);
    }

    #[test]
    fn overlapping_slots_are_counted_once() {
        let mut first = entry(0, &[1, 2, 3, 4]);
        first.offset = 0x200;
        first.slot_size = 8;
        let mut second = entry(1, &[9, 9]);
        second.offset = 0x204;
        second.slot_size = 2;
        let free = dump(vec![first, second]).free_space();
        assert_eq!(free.data_used, 8);
        assert_eq!(free.data_free(), 0x200 - 8);
    }

    #[test]
    fn subsystems_are_tallied_by_their_high_half() {
        let d = dump(vec![entry(0x0001_0000, &[1]), entry(0x0001_0001, &[1])]);
        assert_eq!(d.subsystems(), [(1u16, 2usize)].into_iter().collect());
    }

    #[test]
    fn a_disagreeing_single_byte_of_zero_and_one_looks_like_a_switch() {
        let a = dump(vec![entry(0, &[0])]);
        let b = dump(vec![entry(0, &[1])]);
        assert_eq!(compare(&[&a, &b], &Options::default()).rows[0].shape, Shape::Switch);
    }

    #[test]
    fn a_disagreeing_small_byte_looks_like_a_short_list() {
        let a = dump(vec![entry(0, &[0])]);
        let b = dump(vec![entry(0, &[3])]);
        assert_eq!(compare(&[&a, &b], &Options::default()).rows[0].shape, Shape::SmallChoice);
    }

    #[test]
    fn agreeing_or_unwritable_or_wide_values_carry_no_mark() {
        // Agreeing cameras say nothing about what the value is for.
        let same = dump(vec![entry(0, &[1])]);
        assert_eq!(compare(&[&same, &same], &Options::default()).rows[0].shape, Shape::Plain);

        // Read-only: not a setting anyone could reach.
        let a = dump(vec![entry(0, &[0])]);
        let mut locked = entry(0, &[1]);
        locked.attr = attr::READ_ONLY;
        let b = dump(vec![locked]);
        assert_eq!(compare(&[&a, &b], &Options::default()).rows[0].shape, Shape::Plain);

        // Too large a byte, and more than one byte.
        let c = dump(vec![entry(0, &[0x40])]);
        let d = dump(vec![entry(0, &[0x41])]);
        assert_eq!(compare(&[&c, &d], &Options::default()).rows[0].shape, Shape::Plain);
        let e = dump(vec![entry(0, &[0, 1])]);
        let f = dump(vec![entry(0, &[1, 1])]);
        assert_eq!(compare(&[&e, &f], &Options::default()).rows[0].shape, Shape::Plain);
    }
}
