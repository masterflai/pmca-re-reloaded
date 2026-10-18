//! Plugins: extensions the user installs from a local file.
//!
//! There is no plugin store and nothing is fetched: a plugin reaches this
//! program only because the user chose its file. A plugin file
//! (`*.pmcaplugin`) is a ZIP archive holding a `plugin.json` manifest and,
//! depending on what it brings,
//!
//! * `locales/<lang>.ftl` — its own messages (`plg-<id>-…`), or a complete
//!   language pack for a language the program does not ship;
//! * switches for camera settings, declared in the manifest;
//! * rows for the compatibility overview, declared in the manifest;
//! * tools and diagnostics as WebAssembly modules, run in the sandbox of
//!   `wasm`.
//!
//! Installing copies the checked file into the user's data folder. Plugins are
//! loaded once at start-up ([`load`]), so installing or removing one takes
//! effect at the next start; nothing a running session relies on changes under
//! it. A plugin can never reword one of the program's own messages, change a
//! camera's identity, or see the imported key material.

mod manifest;
mod wasm;

pub use manifest::{API_VERSION, CompatEntry};

use std::collections::HashMap;
use std::io::{Cursor, Read};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use pmca_i18n::Localizer;
use pmca_platform::props::{self, PropDef};
use pmca_platform::tweaks::{self, PluginTweak};
use serde::Serialize;
use serde_json::json;

use crate::{Error, Result};
use manifest::Manifest;

/// File extension of a plugin archive.
pub const EXTENSION: &str = "pmcaplugin";

const MAX_ARCHIVE_BYTES: u64 = 32 << 20;
const MAX_ENTRY_BYTES: u64 = 16 << 20;
const MAX_CATALOG_BYTES: u64 = 1 << 20;
const MAX_ENTRIES: usize = 256;
const MAX_INPUT_BYTES: u64 = 256 << 20;

/// A plugin as installed or offered, for listing and confirming.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct PluginInfo {
    /// Stable identifier.
    pub id: String,
    /// Name as the author wrote it.
    pub name: String,
    /// Version as the author wrote it.
    pub version: String,
    /// Author as written in the manifest.
    pub author: String,
    /// Description as written in the manifest.
    pub description: String,
    /// File name of the archive.
    pub file: String,
    /// What the plugin may do, as slugs the front-ends translate
    /// (`plugin-perm-<slug>`).
    pub permissions: Vec<String>,
    /// Languages it adds that the program does not ship.
    pub languages: Vec<String>,
    /// Ids of the switches it adds.
    pub tweaks: Vec<String>,
    /// Its tools.
    pub tools: Vec<PluginEntry>,
    /// Its diagnostics.
    pub diagnostics: Vec<PluginEntry>,
    /// Number of compatibility rows it adds.
    pub compatibility: u32,
    /// Whether it is in effect in this session.
    pub active: bool,
}

/// A tool or diagnostic of a plugin.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct PluginEntry {
    /// The plugin it belongs to.
    pub plugin: String,
    /// Entry id within the plugin.
    pub id: String,
    /// Fluent id of its name.
    pub label_key: String,
    /// Fluent id of a one-line explanation, if the plugin provides one.
    pub hint_key: Option<String>,
    /// File extensions a tool reads (empty: any).
    pub input: Vec<String>,
    /// Host operations a diagnostic may call.
    pub host: Vec<String>,
}

/// A plugin file that could not be loaded.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct PluginFailure {
    /// File name.
    pub file: String,
    /// Why, as technical English text.
    pub reason: String,
}

/// The installed plugins.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct PluginList {
    /// Plugins whose file is valid.
    pub plugins: Vec<PluginInfo>,
    /// Files that are not.
    pub failed: Vec<PluginFailure>,
    /// Whether the installed set differs from the one this session loaded.
    pub restart_required: bool,
}

/// The compatibility rows of one plugin.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct PluginCompat {
    /// Plugin id.
    pub plugin: String,
    /// Plugin name, to head its table.
    pub name: String,
    /// Its rows.
    pub entries: Vec<CompatEntry>,
}

/// What running a tool or diagnostic produced.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct PluginRunOutcome {
    /// `ok`, `warning` or `error`, as the plugin judged it.
    pub status: String,
    /// Text for the user, as the plugin wrote it.
    pub report: String,
    /// Extension of the output file, if the plugin produced one.
    pub output_extension: Option<String>,
    /// The output file; kept by the caller, never sent to the window.
    #[serde(skip)]
    pub output: Vec<u8>,
}

/// A plugin read from its archive and checked.
struct Plugin {
    manifest: Manifest,
    catalogs: Vec<(String, String)>,
    modules: HashMap<String, Vec<u8>>,
    file: String,
}

struct Loaded {
    plugins: Vec<Plugin>,
    failed: Vec<PluginFailure>,
}

static LOADED: OnceLock<Loaded> = OnceLock::new();

/// Where installed plugins are kept, under the user's data folder.
#[must_use]
pub fn plugin_dir() -> Option<PathBuf> {
    Some(pmca_i18n::catalog::user_data_dir()?.join("plugins"))
}

fn read_entry<R: Read + std::io::Seek>(
    zip: &mut zip::ZipArchive<R>,
    name: &str,
    limit: u64,
) -> std::result::Result<Vec<u8>, String> {
    let file = zip.by_name(name).map_err(|_| format!("`{name}` is missing"))?;
    if file.size() > limit {
        return Err(format!("`{name}` is too large"));
    }
    let mut out = Vec::new();
    file.take(limit + 1).read_to_end(&mut out).map_err(|e| format!("`{name}`: {e}"))?;
    if out.len() as u64 > limit {
        return Err(format!("`{name}` is too large"));
    }
    Ok(out)
}

/// Read and check a plugin archive.
fn read_archive(bytes: &[u8], file: &str) -> std::result::Result<Plugin, String> {
    let mut zip = zip::ZipArchive::new(Cursor::new(bytes))
        .map_err(|e| format!("not a plugin archive: {e}"))?;
    if zip.len() > MAX_ENTRIES {
        return Err(format!("more than {MAX_ENTRIES} files in the archive"));
    }
    let manifest: Manifest = serde_json::from_slice(&read_entry(&mut zip, "plugin.json", 1 << 20)?)
        .map_err(|e| format!("plugin.json: {e}"))?;
    manifest.validate()?;

    let prefix = format!("plg-{}-", manifest.id);
    let names: Vec<String> = zip
        .file_names()
        .map(|n| n.map(|n| n.into_owned()).map_err(|e| format!("archive: {e}")))
        .collect::<std::result::Result<_, String>>()?;
    let mut catalogs = Vec::new();
    let mut english = Vec::new();
    for name in &names {
        let Some(lang) = name.strip_prefix("locales/").and_then(|r| r.strip_suffix(".ftl")) else {
            continue;
        };
        let source = String::from_utf8(read_entry(&mut zip, name, MAX_CATALOG_BYTES)?)
            .map_err(|_| format!("`{name}` is not UTF-8"))?;
        let ids = pmca_i18n::check_plugin_catalog(lang, &source, &prefix)?;
        if lang == pmca_i18n::DEFAULT_LOCALE {
            english = ids;
        }
        catalogs.push((lang.to_string(), source));
    }
    if let Some(key) = manifest.label_keys().into_iter().find(|k| !english.contains(k)) {
        return Err(format!("locales/en-US.ftl must name `{key}`"));
    }

    let mut modules = HashMap::new();
    for path in manifest.module_paths() {
        if modules.contains_key(path) {
            continue;
        }
        let module = read_entry(&mut zip, path, MAX_ENTRY_BYTES)?;
        wasm::check(&module).map_err(|e| format!("{path}: {e}"))?;
        modules.insert(path.to_string(), module);
    }
    Ok(Plugin { manifest, catalogs, modules, file: file.to_string() })
}

fn read_file(path: &Path) -> std::result::Result<(Plugin, Vec<u8>), String> {
    let file = path.file_name().map(|f| f.to_string_lossy().into_owned()).unwrap_or_default();
    let size = std::fs::metadata(path).map_err(|e| e.to_string())?.len();
    if size > MAX_ARCHIVE_BYTES {
        return Err("the file is too large for a plugin".to_string());
    }
    let bytes = std::fs::read(path).map_err(|e| e.to_string())?;
    Ok((read_archive(&bytes, &file)?, bytes))
}

fn installed_files() -> Vec<PathBuf> {
    let Some(dir) = plugin_dir() else { return Vec::new() };
    let Ok(entries) = std::fs::read_dir(dir) else { return Vec::new() };
    let mut files: Vec<PathBuf> = entries
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|e| e == EXTENSION))
        .collect();
    files.sort();
    files
}

impl Plugin {
    fn entries(&self, diagnostics: bool) -> Vec<PluginEntry> {
        let m = &self.manifest;
        let english = self
            .catalogs
            .iter()
            .find(|(l, _)| l == pmca_i18n::DEFAULT_LOCALE)
            .map(|(_, s)| s.as_str())
            .unwrap_or_default();
        let entry = |id: &str, input: &[String], host: &[String]| {
            let label_key = m.key(id);
            let hint = format!("{label_key}-hint");
            let has_hint =
                english.lines().any(|l| l.split_once('=').is_some_and(|(k, _)| k.trim() == hint));
            PluginEntry {
                plugin: m.id.clone(),
                id: id.to_string(),
                label_key,
                hint_key: has_hint.then_some(hint),
                input: input.to_vec(),
                host: host.to_vec(),
            }
        };
        if diagnostics {
            m.diagnostics.iter().map(|d| entry(&d.id, &[], &d.host)).collect()
        } else {
            m.tools.iter().map(|t| entry(&t.id, &t.input, &[])).collect()
        }
    }

    fn info(&self, active: bool) -> PluginInfo {
        let m = &self.manifest;
        let languages: Vec<String> = self
            .catalogs
            .iter()
            .map(|(l, _)| l.clone())
            .filter(|l| !pmca_i18n::is_shipped(l))
            .collect();
        let mut permissions = Vec::new();
        if !languages.is_empty() {
            permissions.push("languages");
        }
        if !m.tweaks.is_empty() {
            permissions.push("camera-settings");
        }
        if !m.compatibility.is_empty() {
            permissions.push("compatibility");
        }
        if !m.tools.is_empty() || !m.diagnostics.is_empty() {
            permissions.push("code");
        }
        for op in manifest::HOST_OPS {
            if m.diagnostics.iter().any(|d| d.host.iter().any(|h| h == op)) {
                permissions.push(if *op == "os" { "host-os" } else { "host-cameras" });
            }
        }
        PluginInfo {
            id: m.id.clone(),
            name: m.name.clone(),
            version: m.version.clone(),
            author: m.author.clone(),
            description: m.description.clone(),
            file: self.file.clone(),
            permissions: permissions.into_iter().map(str::to_string).collect(),
            languages,
            tweaks: m.tweaks.iter().map(|t| m.key(&t.id)).collect(),
            tools: self.entries(false),
            diagnostics: self.entries(true),
            compatibility: m.compatibility.len() as u32,
            active,
        }
    }
}

/// Load every installed plugin and register what it contributes.
///
/// Infallible on purpose, like the key import: a plugin that fails its checks
/// is skipped and reported by [`list`], and the program runs without it. Must
/// be called once at start-up, before the first [`Localizer`] is built.
pub fn load() {
    let mut plugins: Vec<Plugin> = Vec::new();
    let mut failed = Vec::new();
    for path in installed_files() {
        let file = path.file_name().map(|f| f.to_string_lossy().into_owned()).unwrap_or_default();
        match read_file(&path) {
            Ok((plugin, _)) if plugins.iter().any(|p| p.manifest.id == plugin.manifest.id) => {
                failed.push(PluginFailure {
                    file,
                    reason: "another file has the same plugin id".to_string(),
                });
            }
            Ok((plugin, _)) => plugins.push(plugin),
            Err(reason) => failed.push(PluginFailure { file, reason }),
        }
    }

    pmca_i18n::register_plugin_catalogs(plugins.iter().flat_map(|p| p.catalogs.clone()).collect());

    let mut defs = Vec::new();
    let mut switches = Vec::new();
    for plugin in &plugins {
        for tweak in &plugin.manifest.tweaks {
            let Ok((base, off, on)) = tweak.parsed() else { continue };
            // Lives for the rest of the process, like the built-in names.
            let id: &'static str = Box::leak(plugin.manifest.key(&tweak.id).into_boxed_str());
            defs.push(PropDef { name: id, base, count: tweak.count, part_size: tweak.part_size });
            switches.push(PluginTweak { id, prop: id, off, on, models: tweak.models.clone() });
        }
    }
    props::register_extra(defs);
    tweaks::register_plugin_tweaks(switches);
    let _ = LOADED.set(Loaded { plugins, failed });
}

fn loaded() -> &'static [Plugin] {
    LOADED.get().map_or(&[][..], |l| l.plugins.as_slice())
}

fn invalid(l: &Localizer, reason: &str) -> Error {
    Error::Operation(l.tr_arg("plugin-invalid", "reason", reason))
}

/// The installed plugins, and whether a restart would change what is in effect.
#[must_use]
pub fn list() -> PluginList {
    let mut plugins = Vec::new();
    let mut failed = Vec::new();
    for path in installed_files() {
        let file = path.file_name().map(|f| f.to_string_lossy().into_owned()).unwrap_or_default();
        match read_file(&path) {
            Ok((plugin, _)) => {
                let active = loaded().iter().any(|p| {
                    p.manifest.id == plugin.manifest.id
                        && p.manifest.version == plugin.manifest.version
                });
                plugins.push(plugin.info(active));
            }
            Err(reason) => failed.push(PluginFailure { file, reason }),
        }
    }
    let load_failed = LOADED.get().map_or(0, |l| l.failed.len());
    let restart_required = plugins.iter().any(|p| !p.active)
        || loaded().iter().any(|l| !plugins.iter().any(|p| p.id == l.manifest.id))
        || load_failed != failed.len();
    PluginList { plugins, failed, restart_required }
}

/// Check a plugin file the user picked, without installing it.
pub fn inspect(l: &Localizer, path: &Path) -> Result<PluginInfo> {
    let (plugin, _) = read_file(path).map_err(|r| invalid(l, &r))?;
    Ok(plugin.info(false))
}

/// Install (or update) a plugin from a file the user picked.
///
/// The bytes that passed the checks are the bytes written, so the file cannot
/// change between checking and copying. Takes effect at the next start.
pub fn install(l: &Localizer, path: &Path) -> Result<PluginInfo> {
    let (plugin, bytes) = read_file(path).map_err(|r| invalid(l, &r))?;
    let dir = plugin_dir().ok_or_else(|| Error::Operation(l.tr("plugin-no-folder")))?;
    std::fs::create_dir_all(&dir)?;
    std::fs::write(dir.join(format!("{}.{EXTENSION}", plugin.manifest.id)), &bytes)?;
    Ok(plugin.info(false))
}

/// Remove an installed plugin. Takes effect at the next start.
pub fn remove(l: &Localizer, id: &str) -> Result<()> {
    let missing = || Error::Operation(l.tr_arg("plugin-not-installed", "id", id));
    if !manifest::valid_id(id, 32) {
        return Err(missing());
    }
    let path = plugin_dir().ok_or_else(missing)?.join(format!("{id}.{EXTENSION}"));
    if !path.is_file() {
        return Err(missing());
    }
    std::fs::remove_file(path)?;
    Ok(())
}

/// Every file below `dir`, as archive names with `/` separators. Hidden files
/// and folders (a leading dot) are left out.
fn collect(root: &Path, dir: &Path, out: &mut Vec<(String, PathBuf)>) -> std::io::Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let path = entry?.path();
        if path.file_name().is_some_and(|n| n.to_string_lossy().starts_with('.')) {
            continue;
        }
        if path.is_dir() {
            collect(root, &path, out)?;
        } else if let Ok(relative) = path.strip_prefix(root) {
            let name: Vec<String> = relative
                .components()
                .map(|c| c.as_os_str().to_string_lossy().into_owned())
                .collect();
            out.push((name.join("/"), path));
        }
    }
    Ok(())
}

/// Build a plugin file from a folder, checked exactly as installing checks it.
///
/// For plugin authors: the folder holds `plugin.json`, `locales/` and the
/// modules. Nothing is written unless the result would install.
pub fn pack(l: &Localizer, dir: &Path, out: &Path) -> Result<PluginInfo> {
    use std::io::Write;

    let mut files = Vec::new();
    collect(dir, dir, &mut files)?;
    files.sort();
    let mut zip = zip::ZipWriter::new(Cursor::new(Vec::new()));
    for (name, path) in &files {
        zip.start_file(name.as_str(), zip::write::SimpleFileOptions::default())
            .map_err(std::io::Error::other)?;
        zip.write_all(&std::fs::read(path)?)?;
    }
    let bytes = zip.finish().map_err(std::io::Error::other)?.into_inner();
    let file = out.file_name().map(|f| f.to_string_lossy().into_owned()).unwrap_or_default();
    let plugin = read_archive(&bytes, &file).map_err(|r| invalid(l, &r))?;
    std::fs::write(out, &bytes)?;
    Ok(plugin.info(false))
}

/// The compatibility rows the loaded plugins add.
#[must_use]
pub fn compatibility() -> Vec<PluginCompat> {
    loaded()
        .iter()
        .filter(|p| !p.manifest.compatibility.is_empty())
        .map(|p| PluginCompat {
            plugin: p.manifest.id.clone(),
            name: p.manifest.name.clone(),
            entries: p.manifest.compatibility.clone(),
        })
        .collect()
}

/// The tools and diagnostics of the loaded plugins.
#[must_use]
pub fn entries(diagnostics: bool) -> Vec<PluginEntry> {
    loaded().iter().flat_map(|p| p.entries(diagnostics)).collect()
}

fn run(
    l: &Localizer,
    plugin: &str,
    entry: &str,
    diagnostic: bool,
    data: &[u8],
    file_name: Option<&str>,
) -> Result<PluginRunOutcome> {
    let unknown =
        || Error::Operation(l.tr_arg("plugin-unknown-entry", "id", &format!("{plugin}/{entry}")));
    let p = loaded().iter().find(|p| p.manifest.id == plugin).ok_or_else(unknown)?;
    let (module, host) = if diagnostic {
        let d = p.manifest.diagnostics.iter().find(|d| d.id == entry).ok_or_else(unknown)?;
        (&d.module, d.host.clone())
    } else {
        let t = p.manifest.tools.iter().find(|t| t.id == entry).ok_or_else(unknown)?;
        (&t.module, Vec::new())
    };
    let module = p.modules.get(module).ok_or_else(unknown)?;
    let request = json!({
        "api": API_VERSION,
        "kind": if diagnostic { "diagnostic" } else { "tool" },
        "entry": entry,
        "locale": l.current(),
        "fileName": file_name,
    });
    let response = wasm::run(module, &request, data, &host, wasm::FUEL)
        .map_err(|e| Error::Operation(l.tr_arg("plugin-failed", "reason", &e)))?;
    Ok(PluginRunOutcome {
        status: response.status,
        report: response.report,
        output_extension: response.output_extension,
        output: response.output,
    })
}

/// Run a plugin tool on a file. The output, if any, is returned, not written.
pub fn run_tool(l: &Localizer, plugin: &str, tool: &str, input: &Path) -> Result<PluginRunOutcome> {
    if std::fs::metadata(input)?.len() > MAX_INPUT_BYTES {
        return Err(Error::Operation(l.tr_arg("plugin-failed", "reason", "input file too large")));
    }
    let data = std::fs::read(input)?;
    let name = input.file_name().map(|f| f.to_string_lossy().into_owned());
    run(l, plugin, tool, false, &data, name.as_deref())
}

/// Run a plugin diagnostic.
pub fn run_diagnostic(l: &Localizer, plugin: &str, diagnostic: &str) -> Result<PluginRunOutcome> {
    run(l, plugin, diagnostic, true, &[], None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn archive(files: &[(&str, &[u8])]) -> Vec<u8> {
        let mut zip = zip::ZipWriter::new(Cursor::new(Vec::new()));
        for (name, data) in files {
            zip.start_file(*name, zip::write::SimpleFileOptions::default()).unwrap();
            zip.write_all(data).unwrap();
        }
        zip.finish().unwrap().into_inner()
    }

    const MANIFEST: &str = r#"{
        "api": 1, "id": "demo", "name": "Demo", "version": "1.0",
        "tweaks": [{ "id": "flag", "property": "0x01230456", "partSize": 1,
                     "off": "00", "on": "01", "models": ["ILCE-5100"] }],
        "compatibility": [{ "model": "ILCE-6400", "service": true }]
    }"#;

    fn data_plugin(manifest: &str, english: &str) -> std::result::Result<Plugin, String> {
        read_archive(
            &archive(&[
                ("plugin.json", manifest.as_bytes()),
                ("locales/en-US.ftl", english.as_bytes()),
                ("locales/eo.ftl", b"app-title = Titolo\nplg-demo-flag = Flago\n"),
            ]),
            "demo.pmcaplugin",
        )
    }

    #[test]
    fn a_data_plugin_is_read_with_its_languages_switches_and_rows() {
        let plugin = data_plugin(MANIFEST, "plg-demo-flag = Demo flag\n").unwrap();
        let info = plugin.info(false);
        assert_eq!(info.id, "demo");
        assert_eq!(info.languages, ["eo"]);
        assert_eq!(info.tweaks, ["plg-demo-flag"]);
        assert_eq!(info.compatibility, 1);
        assert_eq!(info.permissions, ["languages", "camera-settings", "compatibility"]);
    }

    #[test]
    fn a_plugin_cannot_reword_a_program_message() {
        let err = read_archive(
            &archive(&[
                ("plugin.json", MANIFEST.as_bytes()),
                ("locales/en-US.ftl", b"plg-demo-flag = Demo flag\nbtn-cancel = Sure\n"),
            ]),
            "x",
        )
        .err()
        .unwrap();
        assert!(err.contains("btn-cancel"), "{err}");
    }

    #[test]
    fn every_entry_needs_an_english_name() {
        let err = data_plugin(MANIFEST, "plg-demo-other = Other\n").err().unwrap();
        assert!(err.contains("plg-demo-flag"), "{err}");
    }

    #[test]
    fn a_plugin_cannot_touch_a_cameras_identity() {
        // 0x00e70003 is the serial number.
        let manifest = MANIFEST
            .replace("0x01230456", "0x00e70003")
            .replace("\"00\"", "\"00000000\"")
            .replace("\"01\"", "\"00000001\"")
            .replace("\"partSize\": 1", "\"partSize\": 4");
        let err = data_plugin(&manifest, "plg-demo-flag = Demo flag\n").err().unwrap();
        assert!(err.contains("identity"), "{err}");
    }

    #[test]
    fn malformed_manifests_are_refused_with_a_reason() {
        for (from, to, expect) in [
            ("\"demo\"", "\"Demo!\"", "plugin id"),
            ("\"api\": 1", "\"api\": 2", "interface"),
            ("\"on\": \"01\"", "\"on\": \"00\"", "same value"),
            ("\"on\": \"01\"", "\"on\": \"0102\"", "1 bytes"),
            ("\"name\"", "\"nmae\"", "unknown field"),
        ] {
            let err =
                data_plugin(&MANIFEST.replacen(from, to, 1), "plg-demo-flag = x\n").err().unwrap();
            assert!(err.contains(expect), "{from} -> {err}");
        }
        assert!(read_archive(b"not a zip", "x").err().unwrap().contains("archive"));
    }

    #[test]
    fn a_tool_module_must_be_present_and_runnable() {
        let manifest = r#"{ "api": 1, "id": "demo", "name": "Demo", "version": "1",
            "tools": [{ "id": "conv", "module": "conv.wasm", "input": ["bin"] }] }"#;
        let english = b"plg-demo-conv = Convert\nplg-demo-conv-hint = Turns a file into another\n";
        let missing = read_archive(
            &archive(&[("plugin.json", manifest.as_bytes()), ("locales/en-US.ftl", english)]),
            "x",
        );
        assert!(missing.err().unwrap().contains("conv.wasm"));

        let module = wat::parse_str(
            r#"(module (memory (export "memory") 1)
                (func (export "pmca_alloc") (param i32) (result i32) (i32.const 0))
                (func (export "pmca_run") (param i32 i32 i32 i32) (result i64) (i64.const 0)))"#,
        )
        .unwrap();
        let plugin = read_archive(
            &archive(&[
                ("plugin.json", manifest.as_bytes()),
                ("locales/en-US.ftl", english),
                ("conv.wasm", &module),
            ]),
            "x",
        )
        .unwrap();
        let info = plugin.info(false);
        assert_eq!(info.permissions, ["code"]);
        assert_eq!(info.tools[0].hint_key.as_deref(), Some("plg-demo-conv-hint"));
        assert_eq!(info.tools[0].input, ["bin"]);
    }
}
