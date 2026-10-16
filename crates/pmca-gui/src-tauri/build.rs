//! Generates the Tauri context (configuration, capabilities, icons) at build
//! time, and refuses to build when the version numbers disagree.
//!
//! The version is written down in four places: the workspace (which the
//! window and the console show), this crate, `tauri.conf.json` (which the
//! installer and the Windows file properties use) and `package.json`. Raising
//! it in one and forgetting another would ship a build that names one version
//! in its window and another in its installer, so the build stops instead.

use std::path::Path;

fn main() {
    check_versions();
    tauri_build::build()
}

fn check_versions() {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace = manifest.join("../../../Cargo.toml");
    let tauri_conf = manifest.join("tauri.conf.json");
    let package_json = manifest.join("../package.json");
    for path in [&workspace, &tauri_conf, &package_json] {
        println!("cargo:rerun-if-changed={}", path.display());
    }

    let expected = workspace_version(&read(&workspace));
    let found = [
        ("crates/pmca-gui/src-tauri/Cargo.toml", env!("CARGO_PKG_VERSION").to_owned()),
        ("crates/pmca-gui/src-tauri/tauri.conf.json", json_version(&read(&tauri_conf))),
        ("crates/pmca-gui/package.json", json_version(&read(&package_json))),
    ];
    let wrong: Vec<String> = found
        .iter()
        .filter(|(_, version)| *version != expected)
        .map(|(file, version)| format!("  {file}: {version}"))
        .collect();
    assert!(
        wrong.is_empty(),
        "version mismatch: the workspace is {expected}, but\n{}",
        wrong.join("\n")
    );
}

fn read(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()))
}

/// `version = "…"` in the `[workspace.package]` table.
fn workspace_version(toml: &str) -> String {
    let mut in_table = false;
    for line in toml.lines().map(str::trim) {
        if line.starts_with('[') {
            in_table = line == "[workspace.package]";
        } else if in_table
            && let Some(rest) = line.strip_prefix("version")
            && let Some(value) = rest.trim_start().strip_prefix('=')
        {
            return value.trim().trim_matches('"').to_owned();
        }
    }
    panic!("no version in [workspace.package]");
}

/// The first top-level `"version": "…"` of a JSON file.
fn json_version(json: &str) -> String {
    json.lines()
        .map(str::trim)
        .find_map(|line| line.strip_prefix("\"version\""))
        .and_then(|rest| rest.trim_start().strip_prefix(':'))
        .map(|value| value.trim().trim_end_matches(',').trim_matches('"').to_owned())
        .expect("no \"version\" field")
}
