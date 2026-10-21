//! The `plugin.json` manifest and the rules it has to satisfy.

use serde::{Deserialize, Serialize};

/// The plugin interface version this program implements.
pub const API_VERSION: u32 = 1;

/// Host operations a diagnostic may ask for, each only if its manifest says so.
pub(crate) const HOST_OPS: &[&str] = &["os", "cameras"];

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct Manifest {
    pub api: u32,
    pub id: String,
    pub name: String,
    pub version: String,
    #[serde(default)]
    pub author: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub tweaks: Vec<TweakDef>,
    #[serde(default)]
    pub compatibility: Vec<CompatEntry>,
    #[serde(default)]
    pub tools: Vec<ToolDef>,
    #[serde(default)]
    pub diagnostics: Vec<DiagnosticDef>,
}

/// A switch for one backup setting: two exact values and the models it is for.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct TweakDef {
    pub id: String,
    /// First backup property id, as hex (`"0x003c0373"`).
    pub property: String,
    #[serde(default = "one")]
    pub count: u32,
    pub part_size: usize,
    /// The value meaning "off", as hex bytes.
    pub off: String,
    /// The value meaning "on", as hex bytes.
    pub on: String,
    #[serde(default)]
    pub models: Vec<String>,
}

fn one() -> u32 {
    1
}

/// One row a plugin adds to the compatibility overview.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct CompatEntry {
    /// Model name as the camera reports it, e.g. `ILCE-6400`.
    pub model: String,
    /// Processor family, where known.
    #[serde(default)]
    pub arch: Option<String>,
    /// Whether apps can be installed; `None` where unknown.
    #[serde(default)]
    pub apps: Option<bool>,
    /// Whether service mode works; `None` where unknown.
    #[serde(default)]
    pub service: Option<bool>,
    /// Whether the updater shell works; `None` where unknown.
    #[serde(default)]
    pub updater: Option<bool>,
    /// A remark from the plugin's author, shown as written.
    #[serde(default)]
    pub note: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct ToolDef {
    pub id: String,
    /// Path of the WebAssembly module inside the archive.
    pub module: String,
    /// File extensions the tool reads, for the front-ends' hint.
    #[serde(default)]
    pub input: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct DiagnosticDef {
    pub id: String,
    pub module: String,
    /// Host operations the diagnostic may call (see [`HOST_OPS`]).
    #[serde(default)]
    pub host: Vec<String>,
}

/// A plugin or entry id: lower-case letters, digits and single hyphens.
pub(crate) fn valid_id(id: &str, max: usize) -> bool {
    !id.is_empty()
        && id.len() <= max
        && id.starts_with(|c: char| c.is_ascii_lowercase())
        && !id.ends_with('-')
        && !id.contains("--")
        && id.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

/// Decode hex bytes, tolerating spaces between them.
pub(crate) fn hex_bytes(text: &str) -> Option<Vec<u8>> {
    let digits: String = text.chars().filter(|c| !c.is_whitespace()).collect();
    if digits.is_empty() || !digits.len().is_multiple_of(2) {
        return None;
    }
    (0..digits.len()).step_by(2).map(|i| u8::from_str_radix(&digits[i..i + 2], 16).ok()).collect()
}

impl TweakDef {
    /// The property id and the two values, checked against each other.
    pub(crate) fn parsed(&self) -> Result<(u32, Vec<u8>, Vec<u8>), String> {
        let base = self
            .property
            .strip_prefix("0x")
            .and_then(|h| u32::from_str_radix(h, 16).ok())
            .ok_or_else(|| {
            format!("tweak `{}`: property must be hex like 0x003c0373", self.id)
        })?;
        if !(1..=64).contains(&self.count) || !(1..=256).contains(&self.part_size) {
            return Err(format!("tweak `{}`: count or partSize out of range", self.id));
        }
        let size = self.count as usize * self.part_size;
        let off = hex_bytes(&self.off);
        let on = hex_bytes(&self.on);
        let (Some(off), Some(on)) = (off, on) else {
            return Err(format!("tweak `{}`: off and on must be hex bytes", self.id));
        };
        if off.len() != size || on.len() != size {
            return Err(format!("tweak `{}`: off and on must be {size} bytes long", self.id));
        }
        if off == on {
            return Err(format!("tweak `{}`: off and on are the same value", self.id));
        }
        let mut identity = pmca_platform::props::IDENTITY
            .iter()
            .filter_map(|name| pmca_platform::props::find(name))
            .flat_map(|p| p.ids());
        let ids = base..base.saturating_add(self.count);
        if identity.any(|id| ids.contains(&id)) {
            return Err(format!(
                "tweak `{}`: the identity of a camera (model, serial number) cannot be changed by a plugin",
                self.id
            ));
        }
        Ok((base, off, on))
    }
}

impl Manifest {
    /// Every rule that can be checked on the manifest alone.
    pub(crate) fn validate(&self) -> Result<(), String> {
        if self.api != API_VERSION {
            return Err(format!(
                "written for plugin interface {}, this program implements {API_VERSION}",
                self.api
            ));
        }
        if !valid_id(&self.id, 32) {
            return Err(format!("`{}` is not a valid plugin id", self.id));
        }
        let text_fields = [
            ("name", &self.name, 80, true),
            ("version", &self.version, 32, true),
            ("author", &self.author, 80, false),
            ("description", &self.description, 500, false),
        ];
        for (field, value, max, required) in text_fields {
            if (required && value.trim().is_empty()) || value.chars().count() > max {
                return Err(format!(
                    "`{field}` must be {}at most {max} characters",
                    if required { "set and " } else { "" }
                ));
            }
        }

        let mut seen = std::collections::HashSet::new();
        let entries = self
            .tweaks
            .iter()
            .map(|t| &t.id)
            .chain(self.tools.iter().map(|t| &t.id))
            .chain(self.diagnostics.iter().map(|d| &d.id));
        for id in entries {
            if !valid_id(id, 32) {
                return Err(format!("`{id}` is not a valid entry id"));
            }
            if matches!(id.as_str(), "name" | "description") {
                return Err(format!("entry id `{id}` is reserved for the plugin's own {id}"));
            }
            if !seen.insert(id) {
                return Err(format!("entry id `{id}` is used twice"));
            }
        }

        for tweak in &self.tweaks {
            tweak.parsed()?;
            if tweak.models.iter().any(|m| m.trim().is_empty() || m.len() > 32) {
                return Err(format!("tweak `{}`: invalid model name", tweak.id));
            }
        }
        if self.compatibility.len() > 1000 {
            return Err("at most 1000 compatibility entries".to_string());
        }
        for row in &self.compatibility {
            let too_long = row.model.trim().is_empty()
                || row.model.len() > 32
                || row.arch.as_ref().is_some_and(|a| a.len() > 16)
                || row.note.as_ref().is_some_and(|n| n.chars().count() > 200);
            if too_long {
                return Err(format!("compatibility entry `{}` has an invalid field", row.model));
            }
        }
        for module in
            self.tools.iter().map(|t| &t.module).chain(self.diagnostics.iter().map(|d| &d.module))
        {
            if !module.ends_with(".wasm") || module.starts_with('/') || module.contains("..") {
                return Err(format!("`{module}` is not a module path inside the plugin"));
            }
        }
        for tool in &self.tools {
            if tool.input.iter().any(|e| {
                e.is_empty() || e.len() > 8 || !e.chars().all(|c| c.is_ascii_alphanumeric())
            }) {
                return Err(format!(
                    "tool `{}`: input lists file extensions without a dot",
                    tool.id
                ));
            }
        }
        for diagnostic in &self.diagnostics {
            if let Some(op) = diagnostic.host.iter().find(|op| !HOST_OPS.contains(&op.as_str())) {
                return Err(format!("diagnostic `{}`: unknown host access `{op}`", diagnostic.id));
            }
        }
        Ok(())
    }

    /// The Fluent id naming an entry: `plg-<plugin>-<entry>`.
    pub(crate) fn key(&self, entry: &str) -> String {
        format!("plg-{}-{entry}", self.id)
    }

    /// The label ids the plugin must define in English.
    pub(crate) fn label_keys(&self) -> Vec<String> {
        self.tweaks
            .iter()
            .map(|t| &t.id)
            .chain(self.tools.iter().map(|t| &t.id))
            .chain(self.diagnostics.iter().map(|d| &d.id))
            .map(|id| self.key(id))
            .collect()
    }

    /// The archive paths of every module the manifest names.
    pub(crate) fn module_paths(&self) -> impl Iterator<Item = &str> {
        self.tools
            .iter()
            .map(|t| t.module.as_str())
            .chain(self.diagnostics.iter().map(|d| d.module.as_str()))
    }
}
