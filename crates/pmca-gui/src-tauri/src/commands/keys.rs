//! Importing and reporting the Sony-derived key material the user supplies.

use crate::*;

/// The current state of the key store: which classes are present.
#[tauri::command]
pub(crate) fn keys_status() -> pmca_app::keys::KeyStatus {
    pmca_app::keys::status()
}

/// Import key material from the sources the user entered, then report the state.
///
/// Runs in the background and streams progress to the window like every other
/// slow operation. The sources are downloaded and scanned; nothing is executed.
#[tauri::command]
pub(crate) async fn keys_import(
    app: AppHandle,
    urls: Vec<String>,
) -> Result<pmca_app::keys::KeyStatus, CommandError> {
    in_background(move || {
        let state = app.state::<AppState>();
        let localizer = state.localizer()?;
        let mut sink = progress(&app);
        Ok(pmca_app::keys::import_from_urls(&urls, &localizer, &mut sink)?)
    })
    .await
}
