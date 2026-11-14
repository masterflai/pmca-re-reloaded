//! Languages, translation catalogues, about page and host facts.

use crate::*;

use tauri_plugin_updater::UpdaterExt;
/// List the locales shipped with the application.
#[tauri::command]
pub(crate) fn available_locales() -> Vec<String> {
    Localizer::new("en-US").available()
}

/// Serve the raw Fluent catalogue for `locale` to the front-end.
///
/// The webview cannot `fetch` bundled resources — they live on disk next to the
/// executable, not in the embedded asset tree — so the catalogue is handed over
/// through IPC instead. This also means the window and the backend translate
/// from the exact same source, including any correction the user has uploaded.
#[tauri::command]
pub(crate) fn locale_catalog(locale: String) -> Result<String, CommandError> {
    pmca_i18n::catalog::effective_source(&locale).ok_or(CommandError {
        code: "unknown-locale".into(),
        message: format!("unknown locale: {locale}"),
        detail: Some(locale),
        recovery: None,
    })
}

/// Switch the backend locale and report the one actually negotiated.
#[tauri::command]
pub(crate) fn set_locale(
    state: State<'_, AppState>,
    locale: String,
) -> Result<String, CommandError> {
    let negotiated = Localizer::new(&locale).current().to_string();
    *state.locale.lock().map_err(CommandError::internal)? = negotiated.clone();
    Ok(negotiated)
}

/// Provenance, acknowledgements and the licence, for the License page.
///
/// Only the parts that are not catalogue keys travel here: the version, the
/// addresses and the licence text. The prose is looked up in the window's own
/// catalogue like every other label, so it follows a language change without a
/// round trip.
#[tauri::command]
pub(crate) fn about_info() -> pmca_app::about::AboutInfo {
    pmca_app::about::info()
}

/// The newest release, if it is newer than this build. The window only asks
/// when the user has switched the update notification on.
#[tauri::command]
pub(crate) async fn check_for_update()
-> Result<Option<pmca_app::dto::AvailableUpdate>, CommandError> {
    in_background(|| Ok(pmca_app::update::check()?)).await
}

/// The portable Windows archive carries this marker file beside the executable,
/// so the program it unpacks knows it must not try to replace itself in place.
const PORTABLE_MARKER: &str = "portable.marker";

fn portable_marker_present() -> bool {
    std::env::current_exe()
        .ok()
        .and_then(|exe| exe.parent().map(|dir| dir.join(PORTABLE_MARKER)))
        .is_some_and(|marker| marker.exists())
}

/// Whether this build can replace itself through the in-app updater.
///
/// False for a debug build (nothing to replace it with), for the portable
/// Windows archive (the user unpacked it and replaces it by hand), and for a
/// Linux build not started as an AppImage (a `.deb` is the system package
/// manager's to update). The window hides the update button accordingly.
#[tauri::command]
pub(crate) fn update_supported() -> bool {
    if cfg!(debug_assertions) {
        return false;
    }
    if cfg!(target_os = "linux") && std::env::var_os("APPIMAGE").is_none() {
        return false;
    }
    !portable_marker_present()
}

/// Why an application update cannot be applied at this moment, as reason codes
/// the window translates; an empty list means it is safe to update now.
#[tauri::command]
pub(crate) fn update_blockers(state: State<'_, AppState>) -> Vec<String> {
    state.update_blockers().iter().map(|&code| code.to_string()).collect()
}

/// Event the window listens to for application-update download progress.
const UPDATE_PROGRESS_EVENT: &str = "pmca://update-progress";

/// How far an application-update download has got, for the window's progress bar.
#[derive(Clone, serde::Serialize)]
struct UpdateProgress {
    /// Bytes received so far.
    downloaded: u64,
    /// Total bytes, when the server announced a content length.
    total: Option<u64>,
}

/// Wrap an updater-plugin failure as a [`CommandError`] under a stable code the
/// window can translate; the plugin's own wording is kept as the fallback.
fn update_error(code: &str, e: impl std::fmt::Display) -> CommandError {
    let message = e.to_string();
    CommandError {
        code: code.into(),
        message: message.clone(),
        detail: Some(message),
        recovery: None,
    }
}

/// Refuse while a camera operation, session, batch install or un-restored write
/// test is in progress, so an update never interrupts work on the camera.
fn refuse_if_busy(state: &AppState) -> Result<(), CommandError> {
    let blockers = state.update_blockers();
    if blockers.is_empty() {
        return Ok(());
    }
    Err(CommandError {
        code: "update-busy".into(),
        message: "an operation is in progress; the update has to wait".into(),
        detail: Some(blockers.join(",")),
        recovery: None,
    })
}

/// Download the newest release and verify its signature, keeping the installer
/// in memory for [`update_install`]. Returns the version that was fetched.
///
/// The signature is checked against the public key built into the application
/// before the bytes are returned, so a tampered download never reaches the
/// install step. Progress is reported through [`UPDATE_PROGRESS_EVENT`]; the
/// camera guard is checked first so a download cannot start mid-operation.
#[tauri::command]
pub(crate) async fn update_download(
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<String, CommandError> {
    refuse_if_busy(&state)?;
    let update = app
        .updater()
        .map_err(|e| update_error("update-check-failed", e))?
        .check()
        .await
        .map_err(|e| update_error("update-check-failed", e))?
        .ok_or_else(|| CommandError {
            code: "update-none".into(),
            message: "no newer release is available".into(),
            detail: None,
            recovery: None,
        })?;
    let version = update.version.clone();
    let reporter = app.clone();
    let mut downloaded: u64 = 0;
    let bytes = update
        .download(
            move |chunk, total| {
                downloaded += chunk as u64;
                let _ = reporter.emit(UPDATE_PROGRESS_EVENT, UpdateProgress { downloaded, total });
            },
            || {},
        )
        .await
        .map_err(|e| update_error("update-download-failed", e))?;
    *state.pending_update.lock().map_err(CommandError::internal)? =
        Some(PendingUpdate { update, bytes });
    Ok(version)
}

/// Install the update fetched by [`update_download`] and restart into it.
///
/// The camera guard is checked again: the user may have started an operation
/// between downloading and confirming, and replacing the program while the
/// camera is being written could brick it. This never returns on success — the
/// process is replaced by the restarted application.
#[tauri::command]
pub(crate) async fn update_install(
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<(), CommandError> {
    refuse_if_busy(&state)?;
    let pending =
        state.pending_update.lock().map_err(CommandError::internal)?.take().ok_or_else(|| {
            CommandError {
                code: "update-not-downloaded".into(),
                message: "no update has been downloaded yet".into(),
                detail: None,
                recovery: None,
            }
        })?;
    // The elevated helper holds a copy of the running program open; it has to
    // go before the installer replaces the files underneath it.
    #[cfg(target_os = "macos")]
    if let Ok(mut broker) = state.broker.lock() {
        broker.take();
    }
    pending.update.install(&pending.bytes).map_err(|e| update_error("update-install-failed", e))?;
    app.restart();
}

/// Which operating system the window is running on.
///
/// The driver advice differs per platform and is not a matter of taste: on
/// Windows a kernel driver has to be displaced before raw USB works, on Linux
/// it is a permissions question, and on macOS neither applies. Showing all
/// three at once would leave the reader to guess which paragraph is theirs, so
/// the window asks and shows one.
#[tauri::command]
pub(crate) fn host_platform() -> &'static str {
    if cfg!(windows) {
        "windows"
    } else if cfg!(target_os = "macos") {
        "macos"
    } else {
        "unix"
    }
}

/// Running Windows services known to keep the camera from being reached.
///
/// Asks the service manager only; touches neither the camera nor the services.
#[tauri::command]
pub(crate) async fn check_interfering_services()
-> Result<Vec<pmca_app::dto::InterferingService>, CommandError> {
    in_background(|| Ok(pmca_app::interfering_services())).await
}

/// Reject a locale the application does not ship a catalogue for.
///
/// Only the shipped languages can be edited. Accepting anything else would
/// store a file nothing ever reads, and would let a name from the window decide
/// where that file goes.
pub(crate) fn known_locale(locale: &str) -> Result<(), CommandError> {
    if pmca_i18n::locale_source(locale).is_some() {
        return Ok(());
    }
    Err(CommandError {
        code: "unknown-locale".into(),
        message: format!("unknown locale: {locale}"),
        detail: Some(locale.to_string()),
        recovery: None,
    })
}

/// Where the user's own catalogue for `locale` is stored, if there is one.
///
/// `None` means the built-in translation is in force. The window shows the path
/// so the file remains findable outside the application — it is an ordinary
/// text file, and hiding its location would make it impossible to back up or to
/// hand to someone else.
#[tauri::command]
pub(crate) fn catalog_status(locale: String) -> Result<Option<String>, CommandError> {
    known_locale(&locale)?;
    let path = pmca_i18n::catalog::user_catalog_path(&locale)
        .filter(|p| p.exists())
        .map(|p| absolute_display(&p));
    Ok(path)
}

/// Ask where the editable catalogue for `locale` should go and write it there.
///
/// Like [`analysis_export`], the save dialog is opened here so the window never
/// supplies a path to write to. `None` when the dialog was dismissed.
#[tauri::command(async)]
pub(crate) fn export_catalog(
    app: AppHandle,
    window: Window,
    state: State<'_, AppState>,
    locale: String,
) -> Result<Option<String>, CommandError> {
    known_locale(&locale)?;
    let text = pmca_i18n::catalog::export(&locale).ok_or_else(|| CommandError {
        code: "unknown-locale".into(),
        message: format!("unknown locale: {locale}"),
        detail: Some(locale.clone()),
        recovery: None,
    })?;
    let Some(path) =
        ask_save_path(&app, &window, &state, "catalog", &format!("pmca-{locale}.ftl"))?
    else {
        return Ok(None);
    };
    std::fs::write(&path, text)?;
    Ok(Some(absolute_display(&path)))
}

/// Check an edited catalogue and store it when it passes.
///
/// The verdict is returned as a value rather than as an error, because a file
/// with mistakes in it is a normal outcome of translating and the window has to
/// show *which* lines are at fault. Nothing is written unless the report is
/// clean, so a rejected upload leaves the previous state exactly as it was.
#[tauri::command]
pub(crate) fn import_catalog(
    locale: String,
    file: String,
) -> Result<pmca_i18n::catalog::CatalogReport, CommandError> {
    known_locale(&locale)?;
    // Read as bytes so a file in the wrong encoding is reported as such rather
    // than as a generic I/O failure the user cannot act on.
    let bytes = std::fs::read(&file)?;
    let text = String::from_utf8(bytes).map_err(|e| CommandError {
        code: "catalog-encoding".into(),
        message: "the translation file is not valid UTF-8".to_string(),
        detail: Some(e.to_string()),
        recovery: None,
    })?;
    let source = pmca_i18n::catalog::normalize(&text);
    let report =
        pmca_i18n::catalog::validate_for(&locale, &source).ok_or_else(|| CommandError {
            code: "unknown-locale".into(),
            message: format!("unknown locale: {locale}"),
            detail: Some(locale.clone()),
            recovery: None,
        })?;
    if report.is_acceptable() {
        pmca_i18n::catalog::store_user_catalog(&locale, &source)?;
    }
    Ok(report)
}

/// Discard the user's catalogue for `locale`, restoring the built-in wording.
#[tauri::command]
pub(crate) fn reset_catalog(locale: String) -> Result<bool, CommandError> {
    known_locale(&locale)?;
    Ok(pmca_i18n::catalog::remove_user_catalog(&locale)?)
}
