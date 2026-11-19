//! One camera's settings store, row by row, for reading.
//!
//! The store stays in the backend; a front-end asks for the groups, then for
//! one filtered page of a group at a time, because a store holds over ten
//! thousand properties. A second reading of the same camera can be laid over
//! the first, so every value that changed in between is marked.

use std::collections::{BTreeMap, HashMap};
use std::path::Path;

use pmca_i18n::Localizer;
use serde::{Deserialize, Serialize};

use crate::Result;
use crate::compare::{self, DumpCamera, DumpFile, Privacy, PropertyView};
use crate::names::{self, FwName, NameTable, NameTableInfo, NameView};
use crate::own_names::OwnName;

/// The most value bytes a row carries; the detail and the dump have them all.
const VALUE_BYTES: usize = 64;

/// The most rows one page holds.
const MAX_PAGE: usize = 1000;

/// A store as the view holds it, with the reading before it if one was laid under.
pub struct Memory {
    /// The store on screen.
    pub dump: DumpFile,
    before: Option<HashMap<String, String>>,
    same_camera: Option<bool>,
}

impl Memory {
    /// A store on its own.
    #[must_use]
    pub fn new(dump: DumpFile) -> Self {
        Self { dump, before: None, same_camera: None }
    }

    /// A newer reading, with `previous` kept to mark what changed.
    #[must_use]
    pub fn after(dump: DumpFile, previous: &Self) -> Self {
        let before =
            rows(&previous.dump).map(|(image, p)| (row_id(image, p), p.hex.clone())).collect();
        let same = !dump.camera.camera_id.is_empty()
            && dump.camera.camera_id == previous.dump.camera.camera_id;
        Self { dump, before: Some(before), same_camera: Some(same) }
    }

    fn before_of(&self, id: &str) -> Option<&String> {
        self.before.as_ref()?.get(id)
    }

    fn changed(&self, id: &str, hex: &str) -> bool {
        self.before.as_ref().is_some_and(|b| b.get(id).is_none_or(|old| old != hex))
    }
}

/// Which rows the view asks for.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemoryQuery {
    /// Part of an id, a name, a note or a value; empty for all.
    #[serde(default)]
    pub text: String,
    /// Only properties with a toolkit, firmware or own name.
    #[serde(default)]
    pub only_named: bool,
    /// Only properties that differ from their factory value.
    #[serde(default)]
    pub only_touched: bool,
    /// Only properties a value could be written to.
    #[serde(default)]
    pub only_writable: bool,
    /// Only properties that changed since the reading before.
    #[serde(default)]
    pub only_changed: bool,
    /// Leave out values that look like they carry nothing.
    #[serde(default)]
    pub hide_empty: bool,
    /// Include the images behind the first.
    #[serde(default)]
    pub further: bool,
    /// Only this group, as [`MemoryGroup::key`] names it.
    #[serde(default)]
    pub group: Option<String>,
    /// First matching row to return.
    #[serde(default)]
    pub offset: usize,
    /// How many rows to return.
    #[serde(default)]
    pub limit: usize,
}

/// One property as the view shows it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct MemoryRow {
    /// Composite id, as hex; `2:0x…` for the second image.
    pub id: String,
    /// The group it belongs to.
    pub group: String,
    /// The toolkit's name for the setting, where it knows one.
    pub name: Option<String>,
    /// The firmware's name, where an installed plugin or the dump knows it.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, ts(optional))]
    pub fw_name: Option<FwName>,
    /// The user's own name and note.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, ts(optional))]
    pub own: Option<OwnName>,
    /// What a known switch's value means: `on`, `off`, `PAL`, `NTSC`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, ts(optional))]
    pub meaning: Option<String>,
    /// Length of the value.
    pub size: usize,
    /// Attribute bitfield, as hex.
    pub attr: String,
    /// The names of the bits that are set.
    pub attr_names: Vec<String>,
    /// The value as spaced hex, cut after 64 bytes.
    pub value: String,
    /// Whether `value` was cut.
    pub truncated: bool,
    /// Whether the value differs from the factory value.
    pub touched: bool,
    /// Whether a value could be written here at all.
    pub writable: bool,
    /// Whether the value looks like it carries nothing.
    pub empty: bool,
    /// Whether the value was withheld.
    pub redacted: bool,
    /// Whether the value changed since the reading before.
    pub changed: bool,
}

/// One page of rows and how many match in all.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct MemoryPage {
    /// Rows that match the query, over all pages.
    pub total: usize,
    /// The rows of this page.
    pub rows: Vec<MemoryRow>,
}

/// The properties of one subsystem that match a query.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct MemoryGroup {
    /// The subsystem, as hex; `2:0x…` for the second image.
    pub key: String,
    /// The word the firmware names in this subsystem begin with most often.
    pub label: Option<String>,
    /// Matching properties.
    pub total: usize,
    /// How many of them have a name of any kind.
    pub named: usize,
    /// How many of them changed since the reading before.
    pub changed: usize,
}

/// Everything about one property.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct MemoryDetail {
    /// The row, as the table shows it.
    pub row: MemoryRow,
    /// The whole value, as spaced hex.
    pub hex: String,
    /// The factory value, where the property has one.
    pub reset: Option<String>,
    /// The value in the reading before, where one was laid under.
    pub before: Option<String>,
    /// First byte of the value in the image, as hex.
    pub offset: String,
    /// Bytes reserved for the value.
    pub max_size: usize,
}

/// What the view says about the store before the rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct MemoryInfo {
    /// The file that was read.
    pub file: String,
    /// Which camera it came from.
    pub camera: DumpCamera,
    /// Properties of the first image.
    pub properties: usize,
    /// Properties of the images behind the first.
    pub further: usize,
    /// Properties of the first image with a firmware name.
    pub named: usize,
    /// Properties with an own name.
    pub own: usize,
    /// The installed name tables that fit the camera, best first.
    pub tables: Vec<NameTableInfo>,
    /// How many values changed since the reading before, if one was laid under.
    pub changed: Option<usize>,
    /// Whether that reading came from the same camera.
    pub same_camera: Option<bool>,
}

/// Read a dump or a `Backup.bin` and name what the installed tables know.
pub fn open(l: &Localizer, path: &Path) -> Result<DumpFile> {
    let mut dump = compare::read_dump(l, path, Privacy::MASKED)?;
    fill_names(&mut dump, names::tables());
    Ok(dump)
}

/// Name the properties a dump made without the installed tables left unnamed.
fn fill_names(dump: &mut DumpFile, tables: &[NameTable]) {
    let view = NameView::new(tables, &camera_key(&dump.camera));
    for p in &mut dump.properties {
        if p.fw_name.is_none() {
            p.fw_name = parse_id(&p.id).and_then(|id| names::lookup(&view, id));
        }
    }
}

fn camera_key(camera: &DumpCamera) -> names::CameraKey {
    names::camera_key_of(
        camera.model.as_deref(),
        camera.firmware.as_deref(),
        camera.build.as_deref(),
    )
}

fn parse_id(id: &str) -> Option<u32> {
    u32::from_str_radix(id.strip_prefix("0x")?, 16).ok()
}

/// Every property with the number of the image it belongs to.
fn rows(dump: &DumpFile) -> impl Iterator<Item = (usize, &PropertyView)> {
    dump.properties.iter().map(|p| (1, p)).chain(
        dump.further_images
            .iter()
            .enumerate()
            .flat_map(|(i, f)| f.properties.iter().map(move |p| (i + 2, p))),
    )
}

fn row_id(image: usize, p: &PropertyView) -> String {
    if image > 1 { format!("{image}:{}", p.id) } else { p.id.clone() }
}

fn group_of(image: usize, p: &PropertyView) -> String {
    if image > 1 { format!("{image}:{}", p.subsystem) } else { p.subsystem.clone() }
}

/// The facts the view shows above the rows.
#[must_use]
pub fn info(memory: &Memory, own: &BTreeMap<String, OwnName>, path: &Path) -> MemoryInfo {
    let dump = &memory.dump;
    MemoryInfo {
        file: path.display().to_string(),
        camera: dump.camera.clone(),
        properties: dump.properties.len(),
        further: dump.further_images.iter().map(|f| f.properties.len()).sum(),
        named: dump.properties.iter().filter(|p| p.fw_name.is_some()).count(),
        own: own.len(),
        tables: names::table_infos(Some(&camera_key(&dump.camera))),
        changed: memory
            .before
            .as_ref()
            .map(|_| rows(dump).filter(|(i, p)| memory.changed(&row_id(*i, p), &p.hex)).count()),
        same_camera: memory.same_camera,
    }
}

/// The rows that match `query`, whichever group they are in.
fn matching<'a>(
    memory: &'a Memory,
    own: &'a BTreeMap<String, OwnName>,
    query: &'a MemoryQuery,
) -> impl Iterator<Item = MemoryRow> + 'a {
    let needle = query.text.trim().to_lowercase();
    rows(&memory.dump)
        .filter(move |(image, _)| query.further || *image == 1)
        .map(move |(image, p)| row(memory, own, image, p))
        .filter(move |r| keeps(r, query, &needle))
}

fn keeps(r: &MemoryRow, query: &MemoryQuery, needle: &str) -> bool {
    let named = r.name.is_some() || r.fw_name.is_some() || r.own.is_some();
    if (query.only_named && !named)
        || (query.only_touched && !r.touched)
        || (query.only_writable && !r.writable)
        || (query.only_changed && !r.changed)
        // A value that changed to nothing is still a change.
        || (query.hide_empty && r.empty && !query.only_changed)
        || query.group.as_ref().is_some_and(|g| *g != r.group)
    {
        return false;
    }
    if needle.is_empty() {
        return true;
    }
    let has = |s: &str| s.to_lowercase().contains(needle);
    r.id.contains(needle)
        || r.name.as_deref().is_some_and(has)
        || r.fw_name.as_ref().is_some_and(|f| has(&f.name))
        || r.own.as_ref().is_some_and(|o| has(&o.name) || has(&o.note))
        || r.value.contains(needle)
}

fn row(
    memory: &Memory,
    own: &BTreeMap<String, OwnName>,
    image: usize,
    p: &PropertyView,
) -> MemoryRow {
    let id = row_id(image, p);
    let bytes: Vec<&str> = p.hex.split_whitespace().collect();
    let data: Vec<u8> = bytes.iter().filter_map(|b| u8::from_str_radix(b, 16).ok()).collect();
    MemoryRow {
        group: group_of(image, p),
        name: p.name.clone(),
        fw_name: p.fw_name.clone(),
        own: if image == 1 { own.get(&p.id).cloned() } else { None },
        meaning: p
            .name
            .as_deref()
            .and_then(|n| pmca_platform::tweaks::meaning(n, &data))
            .map(str::to_string),
        size: p.size,
        attr: p.attr.clone(),
        attr_names: p.attr_names.clone(),
        value: bytes[..bytes.len().min(VALUE_BYTES)].join(" "),
        truncated: bytes.len() > VALUE_BYTES,
        touched: p.touched,
        writable: p.writable,
        empty: p.empty,
        redacted: p.redacted,
        changed: memory.changed(&id, &p.hex),
        id,
    }
}

/// One page of the rows `query` asks for.
#[must_use]
pub fn page(memory: &Memory, own: &BTreeMap<String, OwnName>, query: &MemoryQuery) -> MemoryPage {
    let limit = query.limit.clamp(1, MAX_PAGE);
    let mut total = 0;
    let mut rows = Vec::new();
    for r in matching(memory, own, query) {
        if total >= query.offset && rows.len() < limit {
            rows.push(r);
        }
        total += 1;
    }
    MemoryPage { total, rows }
}

/// The groups that hold matching rows, in id order. The group filter is ignored.
#[must_use]
pub fn groups(
    memory: &Memory,
    own: &BTreeMap<String, OwnName>,
    query: &MemoryQuery,
) -> Vec<MemoryGroup> {
    let query = MemoryQuery { group: None, ..query.clone() };
    let mut groups: BTreeMap<(usize, String), MemoryGroup> = BTreeMap::new();
    for r in matching(memory, own, &query) {
        let image = r.id.split_once(':').map_or(1, |(i, _)| i.parse().unwrap_or(1));
        let g = groups.entry((image, r.group.clone())).or_insert_with(|| MemoryGroup {
            key: r.group.clone(),
            label: None,
            total: 0,
            named: 0,
            changed: 0,
        });
        g.total += 1;
        g.named += usize::from(r.name.is_some() || r.fw_name.is_some() || r.own.is_some());
        g.changed += usize::from(r.changed);
    }
    let labels = labels(&memory.dump);
    groups
        .into_values()
        .map(|mut g| {
            g.label = labels.get(g.key.rsplit(':').next().unwrap_or(&g.key)).cloned();
            g
        })
        .collect()
}

/// For each subsystem, the first word after `BKID_` its firmware names share most.
fn labels(dump: &DumpFile) -> HashMap<String, String> {
    let mut words: HashMap<&str, HashMap<&str, usize>> = HashMap::new();
    for p in &dump.properties {
        let Some(fw) = &p.fw_name else { continue };
        let rest = fw.name.strip_prefix("BKID_").unwrap_or(&fw.name);
        let word = rest.split('_').next().unwrap_or(rest);
        if !word.is_empty() && !fw.conflict {
            *words.entry(&p.subsystem).or_default().entry(word).or_default() += 1;
        }
    }
    words
        .into_iter()
        .filter_map(|(sub, counts)| {
            let (word, _) = counts.into_iter().max_by(|a, b| a.1.cmp(&b.1).then(b.0.cmp(a.0)))?;
            Some((sub.to_string(), word.to_string()))
        })
        .collect()
}

/// Everything about the property `id`.
#[must_use]
pub fn detail(memory: &Memory, own: &BTreeMap<String, OwnName>, id: &str) -> Option<MemoryDetail> {
    let (image, p) = rows(&memory.dump).find(|(i, p)| row_id(*i, p) == id)?;
    Some(MemoryDetail {
        row: row(memory, own, image, p),
        hex: p.hex.clone(),
        reset: p.reset.clone(),
        before: memory.before_of(id).filter(|old| **old != p.hex).cloned(),
        offset: p.offset.clone(),
        max_size: p.max_size,
    })
}

/// Every row `query` keeps, as CSV with the whole value.
#[must_use]
pub fn csv(memory: &Memory, own: &BTreeMap<String, OwnName>, query: &MemoryQuery) -> String {
    let quote = |s: &str| format!("\"{}\"", s.replace('"', "\"\""));
    let mut out = String::from(
        "id,name,firmware_name,evidence,own_name,note,size,attributes,value,factory_value,touched,writable,changed\r\n",
    );
    let query = MemoryQuery { offset: 0, limit: usize::MAX, ..query.clone() };
    for r in matching(memory, own, &query) {
        let full = detail(memory, own, &r.id);
        let fw = r.fw_name.as_ref();
        let own = r.own.as_ref();
        let fields = [
            r.id.clone(),
            r.name.clone().unwrap_or_default(),
            fw.map(|f| f.name.clone()).unwrap_or_default(),
            fw.map(|f| if f.conflict { "conflict".to_string() } else { f.evidence.clone() })
                .unwrap_or_default(),
            own.map(|o| o.name.clone()).unwrap_or_default(),
            own.map(|o| o.note.clone()).unwrap_or_default(),
            r.size.to_string(),
            r.attr_names.join(" "),
            // Withheld at reading; the zeros the dump keeps in its place are no value.
            if r.redacted {
                "withheld".to_string()
            } else {
                full.as_ref().map(|d| d.hex.clone()).unwrap_or_default()
            },
            if r.redacted { String::new() } else { full.and_then(|d| d.reset).unwrap_or_default() },
            r.touched.to_string(),
            r.writable.to_string(),
            r.changed.to_string(),
        ];
        out.push_str(&fields.iter().map(|f| quote(f)).collect::<Vec<_>>().join(","));
        out.push_str("\r\n");
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compare::{DumpLayout, FreeSpaceView};

    fn property(id: &str, name: Option<&str>, hex: &str, touched: bool) -> PropertyView {
        PropertyView {
            id: id.to_string(),
            subsystem: id[..6].to_string(),
            name: name.map(str::to_string),
            fw_name: None,
            offset: "0x00000000".to_string(),
            size: hex.split_whitespace().count(),
            max_size: hex.split_whitespace().count(),
            attr: "0x00".to_string(),
            attr_names: Vec::new(),
            hex: hex.to_string(),
            reset: None,
            touched,
            writable: true,
            empty: hex.split_whitespace().all(|b| b == "00"),
            redacted: false,
        }
    }

    fn dump(pal: &str) -> DumpFile {
        let free = FreeSpaceView {
            header: 0,
            subsystem_table: 0,
            property_table: 0,
            table_reserve: 0,
            data_size: 0,
            data_used: 0,
            data_free: 0,
            data_slack: 0,
            largest_gap: 0,
            gap_count: 0,
            gaps: Vec::new(),
            trailing: 0,
        };
        let layout = DumpLayout {
            revision: 4,
            image_size: 0,
            data_offset: "0x0".to_string(),
            table_end: "0x0".to_string(),
            num_subsystems: 0,
            num_properties: 0,
            present: 0,
            subsystems: Default::default(),
            free,
        };
        DumpFile {
            schema: compare::DUMP_SCHEMA.to_string(),
            tool: String::new(),
            taken: String::new(),
            camera: DumpCamera {
                model: Some("ILCE-6000".to_string()),
                camera_id: "c1".to_string(),
                ..DumpCamera::default()
            },
            layout,
            properties: vec![
                property("0x010704fd", Some("palNtsc"), pal, true),
                property("0x00020000", None, "03", false),
                property("0x00020001", None, &"ab ".repeat(100), false),
                property("0x00020002", None, "00 00", false),
            ],
            further_images: Vec::new(),
            privacy: String::new(),
        }
    }

    fn query(text: &str) -> MemoryQuery {
        MemoryQuery { text: text.to_string(), limit: 50, ..MemoryQuery::default() }
    }

    #[test]
    fn a_page_filters_by_text_flags_and_group_and_cuts_long_values() {
        let m = Memory::new(dump("01"));
        let own = BTreeMap::new();
        assert_eq!(page(&m, &own, &query("")).total, 4);
        assert_eq!(page(&m, &own, &MemoryQuery { hide_empty: true, ..query("") }).total, 3);
        assert_eq!(page(&m, &own, &query("PALNTSC")).rows[0].meaning.as_deref(), Some("NTSC"));
        assert_eq!(page(&m, &own, &query("0002")).total, 3);
        assert_eq!(page(&m, &own, &MemoryQuery { only_touched: true, ..query("") }).total, 1);
        let group = MemoryQuery { group: Some("0x0002".to_string()), ..query("") };
        assert_eq!(page(&m, &own, &group).total, 3);
        let long = &page(&m, &own, &query("0x00020001")).rows[0];
        assert!(long.truncated);
        assert_eq!(long.value.split(' ').count(), VALUE_BYTES);
        assert_eq!(detail(&m, &own, "0x00020001").unwrap().hex.split_whitespace().count(), 100);
        let second = page(&m, &own, &MemoryQuery { offset: 1, limit: 1, ..query("") });
        assert_eq!((second.total, second.rows[0].id.as_str()), (4, "0x00020000"));
    }

    #[test]
    fn groups_count_their_rows_and_a_second_reading_marks_changes() {
        let first = Memory::new(dump("00"));
        let own: BTreeMap<String, OwnName> = [(
            "0x00020000".to_string(),
            OwnName {
                name: "Mine".to_string(),
                note: String::new(),
                evidence: "camera".to_string(),
            },
        )]
        .into_iter()
        .collect();
        let groups = groups(&first, &own, &query(""));
        assert_eq!(
            groups.iter().map(|g| (g.key.as_str(), g.total, g.named)).collect::<Vec<_>>(),
            [("0x0002", 3, 1), ("0x0107", 1, 1)]
        );
        assert_eq!(page(&first, &own, &query("mine")).total, 1);

        let second = Memory::after(dump("01"), &first);
        let changed = MemoryQuery { only_changed: true, ..query("") };
        assert_eq!(page(&second, &own, &changed).rows[0].id, "0x010704fd");
        assert_eq!(info(&second, &own, Path::new("x")).changed, Some(1));
        assert_eq!(detail(&second, &own, "0x010704fd").unwrap().before.as_deref(), Some("00"));
        assert!(csv(&second, &own, &changed).contains("\"0x010704fd\",\"palNtsc\""));
    }

    #[test]
    fn installed_tables_name_what_the_dump_left_unnamed() {
        let mut d = dump("01");
        let table = NameTable {
            plugin: "p".to_string(),
            id: "t".to_string(),
            scope: names::Scope { models: vec!["ILCE-6000".to_string()], ..Default::default() },
            source: String::new(),
            entries: pmca_platform::names::parse_table(
                "#pmca-names/1\n0x00020000\tBKID_TEST_ONE\t\tfw-table\t",
            )
            .unwrap(),
        };
        fill_names(&mut d, &[table]);
        assert_eq!(d.properties[1].fw_name.as_ref().unwrap().name, "BKID_TEST_ONE");
        let m = Memory::new(d);
        assert_eq!(page(&m, &BTreeMap::new(), &query("bkid_test")).total, 1);
        assert_eq!(groups(&m, &BTreeMap::new(), &query(""))[0].label.as_deref(), Some("TEST"));
    }
}
