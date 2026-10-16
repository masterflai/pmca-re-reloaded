//! Wi-Fi, streaming settings and GPS assist data.

use crate::*;

/// Read the camera's Wi-Fi access points.
///
/// The returned passphrases are in clear text — the camera stores them that
/// way. The front-end masks them until the user asks to see them.
#[tauri::command]
pub(crate) async fn list_wifi(
    app: AppHandle,
    multi: bool,
) -> Result<Vec<WifiAccessPoint>, CommandError> {
    with_camera(app, move |_, state| {
        let localizer = state.localizer()?;
        Ok(pmca_app::network::read_wifi(&localizer, multi)?)
    })
    .await
}

/// Replace the camera's stored Wi-Fi access points.
///
/// This overwrites the list rather than merging into it, so an empty change set
/// is refused here instead of silently erasing every configured network.
#[tauri::command]
pub(crate) async fn save_wifi(
    app: AppHandle,
    multi: bool,
    access_points: Vec<WifiAccessPoint>,
) -> Result<Vec<WifiAccessPoint>, CommandError> {
    with_camera(app, move |_, state| {
        let localizer = state.localizer()?;
        if access_points.is_empty() {
            return Err(CommandError::localized(localizer.tr("wifi-empty-refused")));
        }
        pmca_app::network::write_wifi(&localizer, multi, &access_points)?;
        Ok(pmca_app::network::read_wifi(&localizer, multi)?)
    })
    .await
}

/// Read the camera's live-streaming configuration.
#[tauri::command]
pub(crate) async fn get_streaming_settings(
    app: AppHandle,
) -> Result<StreamingSettings, CommandError> {
    with_camera(app, |_, state| {
        let localizer = state.localizer()?;
        Ok(pmca_app::network::read_streaming(&localizer)?)
    })
    .await
}

/// Write the camera's live-streaming configuration and report the read-back.
#[tauri::command]
pub(crate) async fn save_streaming_settings(
    app: AppHandle,
    settings: StreamingSettings,
) -> Result<StreamingSettings, CommandError> {
    with_camera(app, move |_, state| {
        let localizer = state.localizer()?;
        pmca_app::network::write_streaming(&localizer, &settings)?;
        Ok(pmca_app::network::read_streaming(&localizer)?)
    })
    .await
}

/// Upload GPS assist data to the camera.
///
/// Without a local `file` the current data is downloaded from Sony over HTTPS
/// before the camera is opened, so a network failure cannot interrupt a
/// transfer that is already running.
#[tauri::command]
pub(crate) async fn update_gps(app: AppHandle, file: Option<String>) -> Result<(), CommandError> {
    with_camera(app, move |app, state| {
        let localizer = state.localizer()?;
        let local = match file.as_deref() {
            Some(path) => Some(std::fs::read(path)?),
            None => None,
        };
        Ok(pmca_app::network::update_gps(&localizer, local.as_deref(), &mut progress(app))?)
    })
    .await
}
