//! Firmware flashing and app installation.

use crate::*;

/// Validate a `.dat` firmware image against the camera, and write it once the
/// front-end has confirmed.
///
/// Flashing is the only irreversible operation in the GUI, so the CLI's two
/// gates are preserved verbatim: `flash` selects between a read-only check and a
/// real write, and `confirm` must accompany a write. The front-end runs a check
/// first, so the user sees the actual version transition before agreeing to it.
#[tauri::command]
pub(crate) async fn flash_firmware(
    app: AppHandle,
    file: String,
    flash: bool,
    confirm: bool,
) -> Result<FirmwareOutcome, CommandError> {
    with_camera(app, move |app, state| {
        let localizer = state.localizer()?;
        let request = pmca_app::firmware::FlashRequest { file: Path::new(&file), flash, confirm };
        Ok(pmca_app::firmware::flash_firmware(&localizer, &request, &mut progress(app))?)
    })
    .await
}

/// Fetch the community app list (`OpenMemories-AppList`).
#[tauri::command]
pub(crate) async fn list_community_apps() -> Result<Vec<CommunityApp>, CommandError> {
    in_background(|| Ok(pmca_app::store::community_apps()?)).await
}

/// Install an APK, or serve the camera an empty task when `apk` is `None`.
///
/// With `series` set the task belongs to a batch: the camera stays in
/// app-install mode until [`finish_install_series`]. Otherwise it stands alone
/// and ends with the reminder that the camera stays in that mode.
///
/// Progress is streamed to the webview via the [`PROGRESS_EVENT`] event rather
/// than being buffered until the command returns.
#[tauri::command]
pub(crate) async fn install_app(
    app: AppHandle,
    apk: Option<String>,
    series: Option<bool>,
) -> Result<InstallOutcome, CommandError> {
    with_camera(app, move |app, state| {
        let localizer = state.localizer()?;
        let apk_path = apk.as_deref().map(Path::new);
        let request = InstallRequest { apk: apk_path.map(ApkSource::File), cert: None };
        let mut sink = progress(app);
        if series == Some(true) {
            return in_series(state, |series| series.install(&localizer, &request, &mut sink));
        }
        Ok(pmca_app::install_app(&localizer, &request, &mut sink)?)
    })
    .await
}

/// Run one task of the current batch, starting the batch if there is none.
///
/// The series is taken out of its lock for the duration, so a task that takes
/// minutes does not hold the lock.
pub(crate) fn in_series(
    state: &AppState,
    task: impl FnOnce(&mut InstallSeries) -> pmca_app::Result<InstallOutcome>,
) -> Result<InstallOutcome, CommandError> {
    let mut series =
        state.install_series.lock().map_err(CommandError::internal)?.take().unwrap_or_default();
    let result = task(&mut series);
    *state.install_series.lock().map_err(CommandError::internal)? = Some(series);
    Ok(result?)
}

/// End the current batch, telling the user once that the camera stays in
/// app-install mode.
#[tauri::command]
pub(crate) async fn finish_install_series(app: AppHandle) -> Result<(), CommandError> {
    with_camera(app, |app, state| {
        let localizer = state.localizer()?;
        let series = state.install_series.lock().map_err(CommandError::internal)?.take();
        if let Some(series) = series {
            series.finish(&localizer, &mut progress(app));
        }
        Ok(())
    })
    .await
}

/// The version label of the newest release of a store app.
///
/// Costs one request per app on first use; the answer is kept for the life of
/// the process, and a later download reuses it.
#[tauri::command]
pub(crate) async fn community_app_version(
    app: AppHandle,
    package: String,
) -> Result<String, CommandError> {
    in_background(move || {
        let localizer = app.state::<AppState>().localizer()?;
        Ok(pmca_app::store::latest_version(&localizer, &package)?)
    })
    .await
}

/// What the window learns about an app downloaded ahead of a batch install.
#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DownloadedApp {
    /// Android package name, the key the bytes are kept under.
    package: String,
    /// The version the camera will report once the app is installed.
    version: String,
}

/// Download an app from the community store and keep it for installation.
///
/// Progress is streamed over [`PROGRESS_EVENT`]; the bytes stay in the backend
/// until [`install_downloaded_app`] takes them.
#[tauri::command]
pub(crate) async fn download_community_app(
    app: AppHandle,
    package: String,
) -> Result<DownloadedApp, CommandError> {
    in_background(move || {
        let state = app.state::<AppState>();
        let localizer = state.localizer()?;
        let download =
            pmca_app::store::download_community_app(&localizer, &package, &mut progress(&app))?;
        state
            .downloads
            .lock()
            .map_err(CommandError::internal)?
            .insert(package.clone(), download.data);
        Ok(DownloadedApp { package, version: download.version })
    })
    .await
}

/// Install an app fetched earlier by [`download_community_app`], as one task of
/// the current batch.
///
/// The bytes are taken out of the cache whatever the outcome, so a retry
/// downloads afresh instead of reusing a copy that may be the cause.
#[tauri::command]
pub(crate) async fn install_downloaded_app(
    app: AppHandle,
    package: String,
) -> Result<InstallOutcome, CommandError> {
    with_camera(app, move |app, state| {
        let localizer = state.localizer()?;
        let data =
            state.downloads.lock().map_err(CommandError::internal)?.remove(&package).ok_or_else(
                || CommandError::internal(format!("{package} has not been downloaded")),
            )?;
        let request = InstallRequest { apk: Some(ApkSource::Bytes(&data)), cert: None };
        let mut sink = progress(app);
        in_series(state, |series| series.install(&localizer, &request, &mut sink))
    })
    .await
}

/// Drop every download a batch did not install, for instance after a stop.
#[tauri::command]
pub(crate) fn discard_downloads(state: State<'_, AppState>) -> Result<(), CommandError> {
    state.downloads.lock().map_err(CommandError::internal)?.clear();
    Ok(())
}
