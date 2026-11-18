//! One camera's settings store, row by row, for reading.
//!
//! The dump stays in the backend; a front-end asks for one filtered page at a
//! time, because a store holds over ten thousand properties.

use std::path::Path;

use pmca_i18n::Localizer;
use serde::{Deserialize, Serialize};

use crate::Result;
use crate::compare::{self, DumpCamera, DumpFile, Privacy, PropertyView};
use crate::names::{self, FwName, NameTable, NameTableInfo, NameView};

/// The most value bytes a row carries; the dump file has them all.
const VALUE_BYTES: usize = 64;

/// The most rows one page holds.
const MAX_PAGE: usize = 1000;

/// Which rows a page holds.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemoryQuery {
    /// Part of an id, a name, a firmware name or a value; empty for all.
    #[serde(default)]
    pub text: String,
    /// Only properties with a toolkit or firmware name.
    #[serde(default)]
    pub only_named: bool,
    /// Only properties that differ from their factory value.
    #[serde(default)]
    pub only_touched: bool,
    /// Only properties a value could be written to.
    #[serde(default)]
    pub only_writable: bool,
    /// Include the images behind the first.
    #[serde(default)]
    pub further: bool,
    /// First matching row to return.
    #[serde(default)]
    pub offset: usize,
    /// How many rows to return.
    pub limit: usize,
}

/// One property as the view shows it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct MemoryRow {
    /// Composite id, as hex; `2:0x…` for the second image.
    pub id: String,
    /// The toolkit's name for the setting, where it knows one.
    pub name: Option<String>,
    /// The firmware's name, where an installed plugin or the dump knows it.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, ts(optional))]
    pub fw_name: Option<FwName>,
    /// Length of the value.
    pub size: usize,
    /// Attribute bitfield, as hex.
    pub attr: String,
    /// The names of the bits that are set.
    pub attr_names: Vec<String>,
    /// The value as spaced hex, cut after [`VALUE_BYTES`] bytes.
    pub value: String,
    /// Whether `value` was cut.
    pub truncated: bool,
    /// Whether the value differs from the factory value.
    pub touched: bool,
    /// Whether a value could be written here at all.
    pub writable: bool,
    /// Whether the value was withheld.
    pub redacted: bool,
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
    /// The installed name tables that fit the camera, best first.
    pub tables: Vec<NameTableInfo>,
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

/// The facts the view shows above the rows.
#[must_use]
pub fn info(dump: &DumpFile, path: &Path) -> MemoryInfo {
    MemoryInfo {
        file: path.display().to_string(),
        camera: dump.camera.clone(),
        properties: dump.properties.len(),
        further: dump.further_images.iter().map(|f| f.properties.len()).sum(),
        named: dump.properties.iter().filter(|p| p.fw_name.is_some()).count(),
        tables: names::table_infos(Some(&camera_key(&dump.camera))),
    }
}

/// The rows of `dump` that `query` asks for.
#[must_use]
pub fn page(dump: &DumpFile, query: &MemoryQuery) -> MemoryPage {
    let first = dump.properties.iter().map(|p| (1, p));
    let further = dump
        .further_images
        .iter()
        .enumerate()
        .filter(|_| query.further)
        .flat_map(|(i, f)| f.properties.iter().map(move |p| (i + 2, p)));
    let needle = query.text.trim().to_lowercase();
    let mut total = 0;
    let mut rows = Vec::new();
    let limit = query.limit.min(MAX_PAGE);
    for (image, p) in first.chain(further) {
        if !keeps(p, query, &needle) {
            continue;
        }
        if total >= query.offset && rows.len() < limit {
            rows.push(row(image, p));
        }
        total += 1;
    }
    MemoryPage { total, rows }
}

fn keeps(p: &PropertyView, query: &MemoryQuery, needle: &str) -> bool {
    if query.only_named && p.name.is_none() && p.fw_name.is_none() {
        return false;
    }
    if (query.only_touched && !p.touched) || (query.only_writable && !p.writable) {
        return false;
    }
    if needle.is_empty() {
        return true;
    }
    let fw = p.fw_name.as_ref().map(|f| f.name.to_lowercase());
    p.id.contains(needle)
        || p.name.as_ref().is_some_and(|n| n.to_lowercase().contains(needle))
        || fw.is_some_and(|n| n.contains(needle))
        || p.hex.contains(needle)
}

fn row(image: usize, p: &PropertyView) -> MemoryRow {
    let bytes: Vec<&str> = p.hex.split_whitespace().collect();
    let truncated = bytes.len() > VALUE_BYTES;
    MemoryRow {
        id: if image > 1 { format!("{image}:{}", p.id) } else { p.id.clone() },
        name: p.name.clone(),
        fw_name: p.fw_name.clone(),
        size: p.size,
        attr: p.attr.clone(),
        attr_names: p.attr_names.clone(),
        value: bytes[..bytes.len().min(VALUE_BYTES)].join(" "),
        truncated,
        touched: p.touched,
        writable: p.writable,
        redacted: p.redacted,
    }
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
            empty: false,
            redacted: false,
        }
    }

    fn dump() -> DumpFile {
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
            camera: DumpCamera { model: Some("ILCE-6000".to_string()), ..DumpCamera::default() },
            layout,
            properties: vec![
                property("0x010704fd", Some("palNtsc"), "01", true),
                property("0x00020000", None, "03", false),
                property("0x00020001", None, &"ab ".repeat(100), false),
            ],
            further_images: Vec::new(),
            privacy: String::new(),
        }
    }

    fn query(text: &str) -> MemoryQuery {
        MemoryQuery { text: text.to_string(), limit: 50, ..MemoryQuery::default() }
    }

    #[test]
    fn a_page_filters_by_text_and_flags_and_cuts_long_values() {
        let d = dump();
        assert_eq!(page(&d, &query("")).total, 3);
        assert_eq!(page(&d, &query("PALNTSC")).rows[0].id, "0x010704fd");
        assert_eq!(page(&d, &query("0002")).total, 2);
        assert_eq!(page(&d, &MemoryQuery { only_touched: true, ..query("") }).total, 1);
        assert_eq!(page(&d, &MemoryQuery { only_named: true, ..query("") }).total, 1);
        let long = &page(&d, &query("0x00020001")).rows[0];
        assert!(long.truncated);
        assert_eq!(long.value.split(' ').count(), VALUE_BYTES);
        let second = page(&d, &MemoryQuery { offset: 1, limit: 1, ..query("") });
        assert_eq!((second.total, second.rows[0].id.as_str()), (3, "0x00020000"));
    }

    #[test]
    fn installed_tables_name_what_the_dump_left_unnamed() {
        let mut d = dump();
        let table = NameTable {
            plugin: "p".to_string(),
            id: "t".to_string(),
            scope: names::Scope { models: vec!["ILCE-6000".to_string()], ..Default::default() },
            source: String::new(),
            entries: pmca_platform::names::parse_table(
                "#pmca-names/1\n0x00020000\tBKID_TEST\t\tfw-table\t",
            )
            .unwrap(),
        };
        fill_names(&mut d, &[table]);
        assert_eq!(d.properties[1].fw_name.as_ref().unwrap().name, "BKID_TEST");
        assert_eq!(page(&d, &query("bkid_test")).total, 1);
    }
}
