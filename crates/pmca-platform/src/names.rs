//! Firmware names for backup properties, supplied by plugins.
//!
//! Sony's firmware names its backup properties (`BKID_…`). A plugin may bring
//! tables that map property ids to those names for a set of cameras; this
//! module parses them, keeps them for the process, and picks the table that
//! fits a given camera best. Names are for showing and finding properties
//! only: nothing here allows a value to be written.
//!
//! Older bodies (CXD4132, CXD90014) share one id space, so a table for them is
//! scoped by platform. On newer bodies the same name sits at a different index
//! per model and firmware, so their tables are scoped by model and firmware or
//! by the build string a `Backup.bin` carries at offset 0x40.
//!
//! A row may also name one stored value of a property (a menu option) rather
//! than the property itself, e.g. firmware shows `0` as `VAL_OFF`. Such a row
//! starts with `=value` instead of the id and reads
//! `=value<TAB>id<TAB>value<TAB>name<TAB>evidence`, where `id` is a property
//! already named by a row earlier or later in the same table, and `value` is
//! decimal or `0x` hex. A value matches a property's stored bytes read as a
//! little-endian unsigned integer of their own size (1, 2, 4 or 8 bytes);
//! other sizes never match. The list position of a menu option is never the
//! stored value, only an explicit row counts. Older files and plugins without
//! such rows keep parsing exactly as before.

use std::collections::BTreeMap;
use std::sync::OnceLock;

/// The first line every name table starts with.
pub const HEADER: &str = "#pmca-names/1";

/// The most rows one table may hold.
const MAX_ROWS: usize = 100_000;

/// The most value-name rows one table may hold, over all its properties.
const MAX_VALUE_ROWS: usize = 100_000;

/// The most value names one property may list.
const MAX_VALUES_PER_ID: usize = 4096;

/// Marks a row that names one stored value of a property, not the property itself.
const VALUE_ROW: &str = "=value";

/// Where a name comes from, from weakest to strongest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Evidence {
    /// A log or error message in the firmware, tied to the id by the code next to it.
    FwLog,
    /// A table in the firmware that pairs ids with names.
    FwTable,
    /// A settings file and a backup of the same camera hold the value at this id.
    Camset,
    /// The toolkit's function analysis saw the value change with the setting.
    Analysis,
    /// Shown on the camera by changing the setting.
    Camera,
}

impl Evidence {
    /// The token used in tables and reports.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FwLog => "fw-log",
            Self::FwTable => "fw-table",
            Self::Camset => "camset",
            Self::Analysis => "analysis",
            Self::Camera => "camera",
        }
    }

    fn parse(text: &str) -> Option<Self> {
        [Self::FwLog, Self::FwTable, Self::Camset, Self::Analysis, Self::Camera]
            .into_iter()
            .find(|e| e.as_str() == text)
    }
}

/// One row of a table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NameEntry {
    /// The firmware's name, such as `BKID_MODEL_NAME`.
    pub name: String,
    /// Element size and count, where the table states them.
    pub shape: Option<(u32, u32)>,
    /// Where the name comes from.
    pub evidence: Evidence,
    /// A remark, as written in the table.
    pub note: String,
    /// Names for this property's stored values (menu options), by value.
    pub values: BTreeMap<u64, ValueEntry>,
}

/// The firmware's name for one stored value of a property (a menu option).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValueEntry {
    /// The option's name, such as `VAL_16_9`.
    pub name: String,
    /// Where the pairing comes from.
    pub evidence: Evidence,
}

/// Which cameras a table is for. Every list that is not empty has to match.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Scope {
    /// Model names as the camera reports them.
    pub models: Vec<String>,
    /// Firmware versions as the camera reports them (`2.00`).
    pub firmware: Vec<String>,
    /// Build strings from the backup header (`32013_05.2025101101`).
    pub builds: Vec<String>,
    /// Processor families (`CXD90014`).
    pub platforms: Vec<String>,
}

/// A table as a plugin supplies it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NameTable {
    /// The plugin that brings it.
    pub plugin: String,
    /// Its id within the plugin.
    pub id: String,
    /// The cameras it is for.
    pub scope: Scope,
    /// Where the names were taken from, as the author wrote it.
    pub source: String,
    /// The rows, by property id.
    pub entries: BTreeMap<u32, NameEntry>,
}

/// What is known about the camera a backup came from. Unknown parts do not
/// exclude a table, they only make it a weaker match.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CameraKey {
    /// Model name.
    pub model: Option<String>,
    /// Firmware version.
    pub firmware: Option<String>,
    /// Build string from the backup header.
    pub build: Option<String>,
    /// Processor family.
    pub platform: Option<String>,
}

/// The name a camera gives one property.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Named<'a> {
    /// The name, or the competing names joined with ` | ` on a conflict.
    pub name: String,
    /// The strongest evidence among the tables that agree.
    pub evidence: Evidence,
    /// Whether equally fitting tables disagree. Such a name is never to be acted on.
    pub conflict: bool,
    /// The entry of the best table.
    pub entry: &'a NameEntry,
    /// The best table.
    pub table: &'a NameTable,
}

/// The name a camera gives one stored value (menu option) of a property.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValueNamed<'a> {
    /// The option's name.
    pub name: String,
    /// Where the pairing comes from.
    pub evidence: Evidence,
    /// The entry of the table that answered.
    pub entry: &'a ValueEntry,
    /// The table it came from.
    pub table: &'a NameTable,
}

/// Parse a table file.
pub fn parse_table(text: &str) -> Result<BTreeMap<u32, NameEntry>, String> {
    let text = text.strip_prefix('\u{feff}').unwrap_or(text);
    let mut lines = text.lines().map(|l| l.trim_end_matches('\r'));
    if lines.next().map(str::trim) != Some(HEADER) {
        return Err(format!("the first line must be `{HEADER}`"));
    }
    let mut out = BTreeMap::new();
    // Value rows may name a property before its own name row is seen, so they
    // are parsed once names are complete.
    let mut value_rows = Vec::new();
    for (n, line) in lines.enumerate() {
        let row = n + 2;
        if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }
        let mut fields = line.split('\t');
        let first = fields.next().unwrap_or_default().trim();
        if first == VALUE_ROW {
            if value_rows.len() >= MAX_VALUE_ROWS {
                return Err(format!("more than {MAX_VALUE_ROWS} value rows"));
            }
            value_rows.push((row, fields.collect::<Vec<_>>()));
            continue;
        }
        if out.len() >= MAX_ROWS {
            return Err(format!("more than {MAX_ROWS} rows"));
        }
        let id = parse_id(first)
            .ok_or_else(|| format!("row {row}: the id must be hex like 0x003e0005"))?;
        let name = fields.next().unwrap_or_default().trim();
        if name.is_empty()
            || name.len() > 128
            || !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
        {
            return Err(format!("row {row}: invalid name"));
        }
        let shape = parse_shape(fields.next().unwrap_or_default().trim())
            .ok_or_else(|| format!("row {row}: shape must be empty, N or NxM"))?;
        let evidence = Evidence::parse(fields.next().unwrap_or_default().trim())
            .ok_or_else(|| format!("row {row}: unknown evidence"))?;
        let note = fields.collect::<Vec<_>>().join(" ").trim().to_string();
        if note.chars().count() > 200 || note.chars().any(char::is_control) {
            return Err(format!("row {row}: the note is too long or holds control characters"));
        }
        let entry =
            NameEntry { name: name.to_string(), shape, evidence, note, values: BTreeMap::new() };
        if out.insert(id, entry).is_some() {
            return Err(format!("row {row}: id {id:#010x} is listed twice"));
        }
    }
    for (row, fields) in value_rows {
        let mut fields = fields.into_iter();
        let id = fields
            .next()
            .map(str::trim)
            .and_then(parse_id)
            .ok_or_else(|| format!("row {row}: the id must be hex like 0x003e0005"))?;
        let value = fields
            .next()
            .map(str::trim)
            .and_then(parse_value)
            .ok_or_else(|| format!("row {row}: the value must be decimal or 0x hex"))?;
        let name = fields.next().unwrap_or_default().trim();
        if name.is_empty()
            || name.len() > 128
            || !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
        {
            return Err(format!("row {row}: invalid value name"));
        }
        let evidence = Evidence::parse(fields.next().unwrap_or_default().trim())
            .ok_or_else(|| format!("row {row}: unknown evidence"))?;
        let entry = out
            .get_mut(&id)
            .ok_or_else(|| format!("row {row}: id {id:#010x} has no name row in this table"))?;
        if entry.values.len() >= MAX_VALUES_PER_ID {
            return Err(format!(
                "row {row}: more than {MAX_VALUES_PER_ID} values for one property"
            ));
        }
        if entry.values.insert(value, ValueEntry { name: name.to_string(), evidence }).is_some() {
            return Err(format!("row {row}: id {id:#010x} value {value} is listed twice"));
        }
    }
    Ok(out)
}

/// `0x` and up to eight hex digits, as a property id.
fn parse_id(text: &str) -> Option<u32> {
    text.strip_prefix("0x")
        .filter(|h| (1..=8).contains(&h.len()))
        .and_then(|h| u32::from_str_radix(h, 16).ok())
}

/// Decimal or `0x` hex, as a property's stored value.
fn parse_value(text: &str) -> Option<u64> {
    if let Some(hex) = text.strip_prefix("0x") {
        (1..=16).contains(&hex.len()).then(|| u64::from_str_radix(hex, 16).ok()).flatten()
    } else {
        (!text.is_empty() && text.chars().all(|c| c.is_ascii_digit()))
            .then(|| text.parse().ok())
            .flatten()
    }
}

/// `""` for none, `"4"` for one element of four bytes, `"1x13"` for an array.
fn parse_shape(text: &str) -> Option<Option<(u32, u32)>> {
    if text.is_empty() {
        return Some(None);
    }
    let (size, count) = text.split_once('x').unwrap_or((text, "1"));
    let size: u32 = size.parse().ok().filter(|s| (1..=65535).contains(s))?;
    let count: u32 = count.parse().ok().filter(|c| (1..=4096).contains(c))?;
    Some(Some((size, count)))
}

impl Scope {
    /// Whether the scope names anything a camera can be matched against.
    #[must_use]
    pub fn is_bound(&self) -> bool {
        !self.models.is_empty() || !self.platforms.is_empty()
    }

    /// How well a camera fits, or `None` if it does not. A known firmware or
    /// build that matches counts most; one that differs excludes the table.
    fn rank(&self, camera: &CameraKey) -> Option<u8> {
        let listed = |list: &[String], value: &Option<String>| {
            value.as_deref().map(|v| list.iter().any(|l| l.eq_ignore_ascii_case(v.trim())))
        };
        let mut rank = 0;
        if !self.models.is_empty() {
            listed(&self.models, &camera.model).filter(|m| *m)?;
            rank += 2;
        }
        if !self.platforms.is_empty() {
            listed(&self.platforms, &camera.platform).filter(|m| *m)?;
            rank += 1;
        }
        for (list, value) in [(&self.firmware, &camera.firmware), (&self.builds, &camera.build)] {
            if list.is_empty() {
                continue;
            }
            match listed(list, value) {
                Some(true) => rank += 4,
                Some(false) => return None,
                None => {}
            }
        }
        Some(rank)
    }
}

/// The tables that fit one camera, best first.
pub struct NameView<'a> {
    ranked: Vec<(u8, &'a NameTable)>,
}

impl<'a> NameView<'a> {
    /// The tables of `tables` that fit `camera`.
    #[must_use]
    pub fn new(tables: &'a [NameTable], camera: &CameraKey) -> Self {
        let mut ranked: Vec<(u8, &NameTable)> =
            tables.iter().filter_map(|t| t.scope.rank(camera).map(|r| (r, t))).collect();
        ranked.sort_by_key(|r| std::cmp::Reverse(r.0));
        Self { ranked }
    }

    /// Whether any table fits.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.ranked.is_empty()
    }

    /// The fitting tables, best first.
    pub fn tables(&self) -> impl Iterator<Item = &'a NameTable> + '_ {
        self.ranked.iter().map(|(_, t)| *t)
    }

    /// The name of property `id` on this camera.
    #[must_use]
    pub fn get(&self, id: u32) -> Option<Named<'a>> {
        let mut hits =
            self.ranked.iter().filter_map(|(r, t)| t.entries.get(&id).map(|e| (*r, e, *t)));
        let (best, entry, table) = hits.next()?;
        let mut names = vec![entry.name.as_str()];
        let mut evidence = entry.evidence;
        for (_, other, _) in hits.take_while(|(r, ..)| *r == best) {
            if !names.contains(&other.name.as_str()) {
                names.push(&other.name);
            }
            evidence = evidence.max(other.evidence);
        }
        Some(Named { name: names.join(" | "), evidence, conflict: names.len() > 1, entry, table })
    }

    /// The name of the stored `value` of property `id` on this camera.
    ///
    /// Takes the best-ranked table that names it. Unlike [`get`](Self::get),
    /// tables that are equally well ranked but disagree are not merged into a
    /// conflict: the first one wins.
    #[must_use]
    pub fn value_name(&self, id: u32, value: u64) -> Option<ValueNamed<'a>> {
        self.ranked.iter().find_map(|(_, t)| {
            let entry = t.entries.get(&id)?.values.get(&value)?;
            Some(ValueNamed { name: entry.name.clone(), evidence: entry.evidence, entry, table: t })
        })
    }

    /// Every property whose name contains `text`, ignoring case.
    #[must_use]
    pub fn find(&self, text: &str) -> Vec<(u32, Named<'a>)> {
        let needle = text.to_ascii_uppercase();
        let mut ids: Vec<u32> = self
            .ranked
            .iter()
            .flat_map(|(_, t)| t.entries.iter())
            .filter(|(_, e)| e.name.to_ascii_uppercase().contains(&needle))
            .map(|(id, _)| *id)
            .collect();
        ids.sort_unstable();
        ids.dedup();
        ids.into_iter().filter_map(|id| self.get(id).map(|n| (id, n))).collect()
    }
}

static TABLES: OnceLock<Vec<NameTable>> = OnceLock::new();

/// Register the tables installed plugins bring, once per process.
///
/// Returns `false` if tables were already registered.
pub fn register(tables: Vec<NameTable>) -> bool {
    TABLES.set(tables).is_ok()
}

/// Every registered table.
#[must_use]
pub fn tables() -> &'static [NameTable] {
    TABLES.get().map_or(&[][..], Vec::as_slice)
}

/// The registered tables that fit `camera`.
#[must_use]
pub fn for_camera(camera: &CameraKey) -> NameView<'static> {
    NameView::new(tables(), camera)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn table(id: &str, scope: Scope, rows: &str) -> NameTable {
        NameTable {
            plugin: "p".into(),
            id: id.into(),
            scope,
            source: String::new(),
            entries: parse_table(&format!("{HEADER}\n{rows}")).unwrap(),
        }
    }

    fn key(model: &str, platform: &str, build: Option<&str>) -> CameraKey {
        CameraKey {
            model: Some(model.into()),
            platform: Some(platform.into()),
            build: build.map(Into::into),
            firmware: None,
        }
    }

    #[test]
    fn a_table_parses_with_comments_shapes_and_notes() {
        let rows = parse_table(&format!(
            "{HEADER}\r\n# id\tname\n0x003e0005\tBKID_MODEL_NAME\t16\tfw-log\tA6000 RX1RM2\n\n0x02cf0006\tBKID_X\t1x13\tcamera\t\n"
        ))
        .unwrap();
        assert_eq!(rows.len(), 2);
        let model = &rows[&0x003e_0005];
        assert_eq!(model.name, "BKID_MODEL_NAME");
        assert_eq!(model.shape, Some((16, 1)));
        assert_eq!(model.evidence, Evidence::FwLog);
        assert_eq!(model.note, "A6000 RX1RM2");
        assert_eq!(rows[&0x02cf_0006].shape, Some((1, 13)));
    }

    #[test]
    fn malformed_rows_are_refused_with_their_line() {
        for (rows, expected) in [
            ("0x1\tA B\t\tfw-log", "row 2: invalid name"),
            ("12\tA\t\tfw-log", "row 2: the id"),
            ("0x1\tA\t0\tfw-log", "row 2: shape"),
            ("0x1\tA\t\tguess", "row 2: unknown evidence"),
            ("0x1\tA\t\tfw-log\n0x1\tB\t\tfw-log", "row 3: id 0x00000001 is listed twice"),
        ] {
            let err = parse_table(&format!("{HEADER}\n{rows}")).unwrap_err();
            assert!(err.starts_with(expected), "{rows:?}: {err}");
        }
        assert!(parse_table("0x1\tA\t\tfw-log").is_err());
    }

    #[test]
    fn value_rows_parse_decimal_and_hex_and_are_kept_on_their_property() {
        let rows = parse_table(&format!(
            "{HEADER}\n0x00510014\tBKID_ASPECT\t\tfw-table\t\n=value\t0x00510014\t0\tVAL_OFF\tfw-table\n=value\t0x00510014\t0x03\tVAL_16_9\tcamera\n"
        ))
        .unwrap();
        let entry = &rows[&0x0051_0014];
        assert_eq!(entry.values.len(), 2);
        assert_eq!(entry.values[&0].name, "VAL_OFF");
        assert_eq!(entry.values[&0].evidence, Evidence::FwTable);
        assert_eq!(entry.values[&3].name, "VAL_16_9");
        assert_eq!(entry.values[&3].evidence, Evidence::Camera);
    }

    #[test]
    fn malformed_value_rows_are_refused_with_their_line() {
        for (rows, expected) in [
            ("0x1\tA\t\tfw-log\n=value\t0x2\t0\tX\tfw-log", "row 3: id 0x00000002 has no name row"),
            ("0x1\tA\t\tfw-log\n=value\tZZ\t0\tX\tfw-log", "row 3: the id"),
            ("0x1\tA\t\tfw-log\n=value\t0x1\tnope\tX\tfw-log", "row 3: the value"),
            ("0x1\tA\t\tfw-log\n=value\t0x1\t0\tA B\tfw-log", "row 3: invalid value name"),
            ("0x1\tA\t\tfw-log\n=value\t0x1\t0\tX\tguess", "row 3: unknown evidence"),
            (
                "0x1\tA\t\tfw-log\n=value\t0x1\t0\tX\tfw-log\n=value\t0x1\t0x0\tY\tcamera",
                "row 4: id 0x00000001 value 0 is listed twice",
            ),
        ] {
            let err = parse_table(&format!("{HEADER}\n{rows}")).unwrap_err();
            assert!(err.starts_with(expected), "{rows:?}: {err}");
        }
    }

    #[test]
    fn the_most_specific_table_wins_and_a_wrong_build_excludes() {
        let platform = Scope { platforms: vec!["CXD90014".into()], ..Scope::default() };
        let model = Scope {
            models: vec!["ILCE-6000".into()],
            builds: vec!["34135_14.2019012401".into()],
            ..Scope::default()
        };
        let tables = [
            table("common", platform, "0x1\tCOMMON\t\tfw-table\n0x2\tONLY_COMMON\t\tfw-log"),
            table("a6000", model, "0x1\tSPECIFIC\t\tcamera"),
        ];
        let view =
            NameView::new(&tables, &key("ILCE-6000", "CXD90014", Some("34135_14.2019012401")));
        assert_eq!(view.get(1).unwrap().name, "SPECIFIC");
        assert_eq!(view.get(2).unwrap().name, "ONLY_COMMON");
        assert!(view.get(3).is_none());

        let other_build = NameView::new(&tables, &key("ILCE-6000", "CXD90014", Some("other")));
        assert_eq!(other_build.get(1).unwrap().name, "COMMON");
        let unknown_build = NameView::new(&tables, &key("ILCE-6000", "CXD90014", None));
        assert_eq!(unknown_build.get(1).unwrap().name, "SPECIFIC");
        assert!(NameView::new(&tables, &key("DSC-RX1R", "CXD4132", None)).is_empty());
    }

    #[test]
    fn equally_fitting_tables_that_disagree_are_a_conflict() {
        let scope = Scope { platforms: vec!["CXD90014".into()], ..Scope::default() };
        let tables = [
            table("a", scope.clone(), "0x1\tFIRST\t\tfw-log\n0x2\tSAME\t\tfw-log"),
            table("b", scope, "0x1\tSECOND\t\tfw-table\n0x2\tSAME\t\tcamera"),
        ];
        let view = NameView::new(&tables, &key("ILCE-6000", "CXD90014", None));
        let first = view.get(1).unwrap();
        assert!(first.conflict);
        assert_eq!(first.name, "FIRST | SECOND");
        let same = view.get(2).unwrap();
        assert!(!same.conflict);
        assert_eq!(same.evidence, Evidence::Camera);
        assert_eq!(view.find("first").len(), 1);
        assert_eq!(view.find("s").len(), 2);
    }

    #[test]
    fn value_name_follows_the_same_ranking_as_names_and_does_not_merge_ties() {
        let platform = Scope { platforms: vec!["CXD90014".into()], ..Scope::default() };
        let model = Scope { models: vec!["ILCE-6000".into()], ..Scope::default() };
        let tables = [
            table(
                "common",
                platform,
                "0x1\tASPECT\t\tfw-table\n=value\t0x1\t0\tCOMMON_OFF\tfw-table",
            ),
            table("a6000", model, "0x1\tASPECT\t\tcamera\n=value\t0x1\t0\tSPECIFIC_OFF\tcamera"),
        ];
        let view = NameView::new(&tables, &key("ILCE-6000", "CXD90014", None));
        assert_eq!(view.value_name(1, 0).unwrap().name, "SPECIFIC_OFF");
        assert!(view.value_name(1, 9).is_none());
        assert!(
            NameView::new(&tables, &key("DSC-RX1R", "CXD4132", None)).value_name(1, 0).is_none()
        );

        let scope = Scope { platforms: vec!["CXD90014".into()], ..Scope::default() };
        let tied = [
            table("a", scope.clone(), "0x1\tASPECT\t\tfw-log\n=value\t0x1\t0\tFIRST\tfw-log"),
            table("b", scope, "0x1\tASPECT\t\tfw-table\n=value\t0x1\t0\tSECOND\tfw-table"),
        ];
        let tied_view = NameView::new(&tied, &key("ILCE-6000", "CXD90014", None));
        assert_eq!(tied_view.value_name(1, 0).unwrap().name, "FIRST");
    }
}
