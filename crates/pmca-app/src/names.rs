//! Firmware names for the properties of a backup, as plugins supply them.
//!
//! The tables live in [`pmca_platform::names`]; this module works out which
//! camera a backup came from and renders a name the same way everywhere.

use pmca_core::backup::BackupFile;
use pmca_platform::names::{self, Named};
use serde::{Deserialize, Serialize};

pub use pmca_platform::names::{CameraKey, NameTable, NameView, Scope, tables};

/// The most hits one search returns.
pub const MAX_HITS: usize = 500;

/// A firmware name as dumps, reports and front-ends carry it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct FwName {
    /// The name, or the competing names joined with ` | `.
    pub name: String,
    /// `fw-table`, `fw-log`, `camset`, `analysis` or `camera`.
    pub evidence: String,
    /// Whether equally fitting tables disagree; such a name is not to be acted on.
    pub conflict: bool,
    /// The table it came from, as `plugin/table`.
    pub table: String,
}

impl FwName {
    /// The report form of a looked-up name.
    #[must_use]
    pub fn of(named: &Named<'_>) -> Self {
        Self {
            name: named.name.clone(),
            evidence: named.evidence.as_str().to_string(),
            conflict: named.conflict,
            table: format!("{}/{}", named.table.plugin, named.table.id),
        }
    }
}

/// The name of property `id` in `view`, in report form.
#[must_use]
pub fn lookup(view: &NameView<'_>, id: u32) -> Option<FwName> {
    view.get(id).map(|n| FwName::of(&n))
}

/// One name for a property several cameras hold: the name they agree on, or
/// all of them marked as a conflict.
#[must_use]
pub fn merge(names: impl IntoIterator<Item = FwName>) -> Option<FwName> {
    let mut names = names.into_iter();
    let mut merged = names.next()?;
    for other in names {
        merged.conflict |= other.conflict;
        for name in other.name.split(" | ") {
            if !merged.name.split(" | ").any(|n| n == name) {
                merged.name = format!("{} | {name}", merged.name);
                merged.conflict = true;
            }
        }
    }
    Some(merged)
}

/// The camera key of a dump or report, which carries no image to read it from.
#[must_use]
pub fn camera_key_of(
    model: Option<&str>,
    firmware: Option<&str>,
    build: Option<&str>,
) -> CameraKey {
    CameraKey {
        model: model.map(str::to_string),
        firmware: firmware.map(str::to_string),
        build: build.filter(|b| !b.is_empty()).map(str::to_string),
        platform: model.and_then(crate::devices::arch_of).map(str::to_string),
    }
}

/// One installed table, for listings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct NameTableInfo {
    /// The plugin that brings it.
    pub plugin: String,
    /// Its id within the plugin.
    pub id: String,
    /// How many names it holds.
    pub entries: usize,
    /// The cameras it is for, in one line.
    pub scope: String,
    /// Where the names were taken from, as the author wrote it.
    pub source: String,
}

/// The installed tables, or those that fit `camera`, best first.
#[must_use]
pub fn table_infos(camera: Option<&CameraKey>) -> Vec<NameTableInfo> {
    let info = |t: &NameTable| NameTableInfo {
        plugin: t.plugin.clone(),
        id: t.id.clone(),
        entries: t.entries.len(),
        scope: describe_scope(&t.scope),
        source: t.source.clone(),
    };
    match camera {
        Some(camera) => names::for_camera(camera).tables().map(info).collect(),
        None => tables().iter().map(info).collect(),
    }
}

/// A property found by [`search`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct NameHit {
    /// Property id, `0x` and eight hex digits.
    pub id: String,
    /// Its name.
    pub fw: FwName,
}

/// The properties whose name contains `text` or whose id is `text`, at most
/// [`MAX_HITS`].
///
/// With a camera, the tables that fit it are asked together, so competing
/// names show as a conflict. Without one, each table answers on its own: tables
/// for unrelated cameras do not contradict each other.
#[must_use]
pub fn search(all: &[NameTable], text: &str, camera: Option<&CameraKey>) -> Vec<NameHit> {
    let text = text.trim();
    if text.is_empty() {
        return Vec::new();
    }
    let id = parse_id(text);
    let needle = text.to_ascii_uppercase();
    let hit = |id: u32, fw: FwName| NameHit { id: format!("{id:#010x}"), fw };
    let mut hits: Vec<NameHit> = match camera {
        Some(camera) => {
            let view = NameView::new(all, camera);
            let mut found = view.find(text);
            if let Some(id) = id.filter(|id| !found.iter().any(|(f, _)| f == id)) {
                found.extend(view.get(id).map(|n| (id, n)));
            }
            found.into_iter().map(|(id, n)| hit(id, FwName::of(&n))).collect()
        }
        None => all
            .iter()
            .flat_map(|t| {
                t.entries
                    .iter()
                    .filter(|(i, e)| {
                        Some(**i) == id || e.name.to_ascii_uppercase().contains(&needle)
                    })
                    .map(move |(i, e)| {
                        let fw = FwName {
                            name: e.name.clone(),
                            evidence: e.evidence.as_str().to_string(),
                            conflict: false,
                            table: format!("{}/{}", t.plugin, t.id),
                        };
                        hit(*i, fw)
                    })
            })
            .collect(),
    };
    hits.sort_by(|a, b| a.id.cmp(&b.id).then_with(|| a.fw.table.cmp(&b.fw.table)));
    hits.truncate(MAX_HITS);
    hits
}

/// `0x010704fd` or `010704fd` as an id; anything shorter is a name fragment.
fn parse_id(text: &str) -> Option<u32> {
    let hex = text.strip_prefix("0x").or_else(|| text.strip_prefix("0X")).unwrap_or(text);
    (hex.len() == 8).then(|| u32::from_str_radix(hex, 16).ok()).flatten()
}

/// The camera key of a `Backup.bin` on disk.
pub fn camera_key_of_file(
    path: &std::path::Path,
    firmware: Option<&str>,
) -> crate::Result<CameraKey> {
    let data = std::fs::read(path)?;
    let image = BackupFile::new(&data[..])
        .map_err(|e| crate::Error::Operation(format!("{}: {e}", path.display())))?;
    Ok(camera_key(&image, firmware))
}

/// A camera key from a model name alone, with its processor family filled in.
#[must_use]
pub fn camera_key_of_model(model: &str, firmware: Option<&str>) -> CameraKey {
    CameraKey {
        model: Some(model.to_string()),
        firmware: firmware.map(str::to_string),
        build: None,
        platform: crate::devices::arch_of(model).map(str::to_string),
    }
}

/// The scope of a table in one line, for listings.
#[must_use]
pub fn describe_scope(scope: &Scope) -> String {
    [
        ("models", &scope.models),
        ("firmware", &scope.firmware),
        ("builds", &scope.builds),
        ("platforms", &scope.platforms),
    ]
    .iter()
    .filter(|(_, list)| !list.is_empty())
    .map(|(what, list)| format!("{what}: {}", list.join(", ")))
    .collect::<Vec<_>>()
    .join("; ")
}

/// What a backup says about the camera it came from.
///
/// The model is read from the store itself; `firmware` comes from a connected
/// camera or a settings file, since the backup only carries a build string.
#[must_use]
pub fn camera_key<D: AsRef<[u8]>>(image: &BackupFile<D>, firmware: Option<&str>) -> CameraKey {
    let model = pmca_platform::props::find("modelName")
        .and_then(|def| image.get_property(def.base).ok())
        .map(|p| pmca_core::util::latin1_trim_nul(&p.data))
        .filter(|m| !m.is_empty());
    let platform = model.as_deref().and_then(crate::devices::arch_of).map(str::to_string);
    let build = Some(image.build()).filter(|b| !b.is_empty());
    CameraKey { model, firmware: firmware.map(str::to_string), build, platform }
}

/// The installed tables that fit the camera a backup came from.
#[must_use]
pub fn for_image<D: AsRef<[u8]>>(
    image: &BackupFile<D>,
    firmware: Option<&str>,
) -> NameView<'static> {
    names::for_camera(&camera_key(image, firmware))
}

/// `fw="NAME" [evidence]`, or the competing names marked as a conflict.
#[must_use]
pub fn render(named: &Named<'_>) -> String {
    render_fw(&FwName::of(named))
}

/// [`render`] for a name in report form.
#[must_use]
pub fn render_fw(fw: &FwName) -> String {
    let tag = if fw.conflict { "conflict" } else { fw.evidence.as_str() };
    format!("fw=\"{}\" [{tag}]", fw.name)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn table(plugin: &str, models: &[&str], rows: &str) -> NameTable {
        NameTable {
            plugin: plugin.to_string(),
            id: "t".to_string(),
            scope: Scope {
                models: models.iter().map(|m| (*m).to_string()).collect(),
                ..Scope::default()
            },
            source: String::new(),
            entries: names::parse_table(&format!("{}\n{rows}", names::HEADER)).unwrap(),
        }
    }

    fn fw(name: &str) -> FwName {
        FwName {
            name: name.to_string(),
            evidence: "fw-table".to_string(),
            conflict: false,
            table: "p/t".to_string(),
        }
    }

    #[test]
    fn cameras_that_agree_give_one_name_and_others_a_conflict() {
        assert_eq!(merge([fw("A"), fw("A")]), Some(fw("A")));
        let both = merge([fw("A"), fw("B"), fw("A")]).unwrap();
        assert_eq!(both.name, "A | B");
        assert!(both.conflict);
        assert_eq!(merge(Vec::new()), None);
    }

    #[test]
    fn search_finds_names_and_ids_with_and_without_a_camera() {
        let all = [
            table("a", &["ILCE-6000"], "0x010704fd\tBKID_APP_MENU_NTSC_PAL\t\tfw-log\t"),
            table("b", &["ILCE-7CR"], "0x02cf0001\tBKID_OTHER\t\tfw-table\t"),
        ];
        let hits = search(&all, "ntsc", None);
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].id, "0x010704fd");
        assert_eq!(hits[0].fw.table, "a/t");
        assert_eq!(search(&all, "0x02CF0001", None)[0].fw.name, "BKID_OTHER");

        let a6000 = camera_key_of(Some("ILCE-6000"), None, None);
        assert_eq!(search(&all, "bkid", Some(&a6000)).len(), 1);
        assert_eq!(search(&all, "010704fd", Some(&a6000)).len(), 1);
        assert!(search(&all, "other", Some(&a6000)).is_empty());
        assert!(search(&all, "  ", None).is_empty());
    }
}
