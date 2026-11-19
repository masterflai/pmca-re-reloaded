//! Names and notes the user gives properties, kept per camera model.
//!
//! What the firmware does not name, the user can: a label seen to change with
//! a menu setting, or a note on what a value seems to hold. A finished
//! function analysis adds its finding here by itself. The collection can be
//! exported as a name-table plugin and shared like any other.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use pmca_i18n::Localizer;
use serde::{Deserialize, Serialize};

use crate::plugins::PluginInfo;
use crate::{Error, Result};

const FILE: &str = "own-names.json";

/// The longest name and note accepted; a name table allows no more.
const MAX_NAME: usize = 128;
const MAX_NOTE: usize = 200;

/// One property's own name.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct OwnName {
    /// The name, as the user wrote it.
    pub name: String,
    /// A remark.
    #[serde(default)]
    pub note: String,
    /// `camera` when given by hand, `analysis` when a function analysis found it.
    #[serde(default = "by_hand")]
    pub evidence: String,
}

fn by_hand() -> String {
    "camera".to_string()
}

#[derive(Default, Serialize, Deserialize)]
struct Store {
    #[serde(default)]
    models: BTreeMap<String, BTreeMap<String, OwnName>>,
}

fn path() -> Option<PathBuf> {
    pmca_i18n::catalog::user_data_dir().map(|dir| dir.join(FILE))
}

fn load() -> Store {
    path()
        .and_then(|p| std::fs::read(p).ok())
        .and_then(|data| serde_json::from_slice(&data).ok())
        .unwrap_or_default()
}

fn save(store: &Store) -> Result<()> {
    let path = path().ok_or_else(|| Error::Operation("no per-user data folder".to_string()))?;
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    std::fs::write(path, serde_json::to_vec_pretty(store)?)?;
    Ok(())
}

/// The own names of one model, by property id (`0x` and eight hex digits).
#[must_use]
pub fn for_model(model: &str) -> BTreeMap<String, OwnName> {
    load().models.remove(model).unwrap_or_default()
}

/// Set or, with both fields empty, remove the own name of one property.
pub fn set(l: &Localizer, model: &str, id: &str, name: &str, note: &str) -> Result<()> {
    let id = canonical_id(id).ok_or_else(|| Error::Operation(l.tr("own-name-invalid")))?;
    let (name, note) = (name.trim(), note.trim());
    let bad = |s: &str, max: usize| s.chars().count() > max || s.chars().any(char::is_control);
    if model.trim().is_empty() || bad(name, MAX_NAME) || bad(note, MAX_NOTE) {
        return Err(Error::Operation(l.tr("own-name-invalid")));
    }
    let mut store = load();
    let names = store.models.entry(model.to_string()).or_default();
    if name.is_empty() && note.is_empty() {
        names.remove(&id);
    } else {
        names.insert(
            id,
            OwnName { name: name.to_string(), note: note.to_string(), evidence: by_hand() },
        );
    }
    store.models.retain(|_, names| !names.is_empty());
    save(&store)
}

/// Note what a function analysis found, unless the property is named already.
pub fn remember_finding(model: &str, id: u32, setting: &str, day: &str) -> Result<bool> {
    let mut store = load();
    let names = store.models.entry(model.to_string()).or_default();
    let key = format!("{id:#010x}");
    if names.contains_key(&key) {
        return Ok(false);
    }
    let name: String = setting.trim().chars().take(MAX_NAME).collect();
    names.insert(
        key,
        OwnName {
            name,
            note: format!("function analysis {day}"),
            evidence: "analysis".to_string(),
        },
    );
    save(&store)?;
    Ok(true)
}

/// Write the own names of one model as a name-table plugin to `out`.
pub fn export(l: &Localizer, model: &str, out: &Path) -> Result<PluginInfo> {
    let names = for_model(model);
    let mut table = format!("{}\n", pmca_platform::names::HEADER);
    for (id, own) in &names {
        let name = table_name(&own.name);
        if name.is_empty() {
            continue;
        }
        let note: String =
            own.note.chars().map(|c| if c.is_control() { ' ' } else { c }).take(MAX_NOTE).collect();
        table.push_str(&format!("{id}\t{name}\t\t{}\t{note}\n", own.evidence));
    }
    if table.lines().count() < 2 {
        return Err(Error::Operation(l.tr("own-names-none")));
    }
    let day = crate::analysis::report::timestamp().replace('-', "");
    let manifest = serde_json::json!({
        "api": 1,
        "id": plugin_id(model),
        "name": format!("Own names: {model}"),
        "version": day.get(..8).unwrap_or("1"),
        "description": format!("Property names observed on a {model}. Display only."),
        "names": [{
            "id": "own",
            "file": "names/own.tsv",
            "scope": { "models": [model] },
            "source": "own observations",
        }],
    });
    let files = vec![
        ("plugin.json".to_string(), serde_json::to_vec_pretty(&manifest)?),
        ("names/own.tsv".to_string(), table.into_bytes()),
    ];
    crate::plugins::pack_files(l, files, out)
}

/// `0x` and eight lowercase hex digits, from any spelling of a first-image id.
fn canonical_id(id: &str) -> Option<String> {
    let hex = id.trim().strip_prefix("0x")?;
    (hex.len() == 8)
        .then(|| u32::from_str_radix(hex, 16).ok())
        .flatten()
        .map(|v| format!("{v:#010x}"))
}

/// A name as a table accepts it: letters, digits and underscores.
fn table_name(name: &str) -> String {
    name.trim()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .take(MAX_NAME)
        .collect()
}

/// A plugin id for one model: `own-names-dsc-rx1r`.
fn plugin_id(model: &str) -> String {
    let mut id = String::from("own-names-");
    for c in model.to_ascii_lowercase().chars() {
        let c = if c.is_ascii_alphanumeric() { c } else { '-' };
        if !(c == '-' && id.ends_with('-')) {
            id.push(c);
        }
    }
    id.truncate(32);
    id.trim_end_matches('-').to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids_names_and_plugin_ids_take_the_forms_tables_accept() {
        assert_eq!(canonical_id("0x010704FD").as_deref(), Some("0x010704fd"));
        assert_eq!(canonical_id("2:0x010704fd"), None);
        assert_eq!(table_name(" PAL/NTSC switch "), "PAL_NTSC_switch");
        assert_eq!(plugin_id("DSC-RX1R"), "own-names-dsc-rx1r");
        assert_eq!(plugin_id("ILCE--7CR "), "own-names-ilce-7cr");
    }
}
