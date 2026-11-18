//! Firmware names for the properties of a backup, as plugins supply them.
//!
//! The tables live in [`pmca_platform::names`]; this module works out which
//! camera a backup came from and renders a name the same way everywhere.

use pmca_core::backup::BackupFile;
use pmca_platform::names::{self, Named};

pub use pmca_platform::names::{CameraKey, NameTable, NameView, Scope, tables};

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
    let tag = if named.conflict { "conflict" } else { named.evidence.as_str() };
    format!("fw=\"{}\" [{tag}]", named.name)
}
