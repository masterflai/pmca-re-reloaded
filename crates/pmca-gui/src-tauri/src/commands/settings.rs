//! Languages, translation catalogues, about page and host facts.

use crate::*;

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
