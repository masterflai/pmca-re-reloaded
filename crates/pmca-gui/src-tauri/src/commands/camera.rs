//! Camera identity, photo and usage log, mode switch, driver rights and window title.

use crate::*;

/// Detect the connected camera and report its identity and current USB mode.
///
/// On macOS a camera in mass-storage mode (which "Auto" picks on a Mac) is
/// only reachable with administrator rights, so it is read by the broker.
#[tauri::command]
pub(crate) async fn get_camera_info(app: AppHandle) -> Result<Reply<CameraSummary>, CommandError> {
    with_camera(app, |app, state| {
        let here = pmca_app::camera_info().map_err(CommandError::from);
        #[cfg(target_os = "macos")]
        if matches!(&here, Err(e) if e.code == "usb-needs-root") {
            return remote::call(app, state, remote::Request::CameraInfo).map(Reply::Relayed);
        }
        #[cfg(not(target_os = "macos"))]
        let _ = (app, state);
        Ok(Reply::Here(here?))
    })
    .await
}

/// Read the shutter count and capture details from a photo on disk.
///
/// Needs no camera: the count is taken from the file the camera wrote.
#[tauri::command]
pub(crate) async fn read_photo(app: AppHandle, file: String) -> Result<PhotoSummary, CommandError> {
    in_background(move || {
        let localizer = app.state::<AppState>().localizer()?;
        Ok(pmca_app::tools::photo_summary(&localizer, Path::new(&file))?)
    })
    .await
}

/// The device lists, or only the models with `capability`.
#[tauri::command]
pub(crate) fn device_lists(capability: Option<Capability>) -> DeviceLists {
    match capability {
        Some(capability) => pmca_app::devices::capable(capability),
        None => pmca_app::devices::lists(),
    }
}

/// What the device lists say about `model`, as a verdict and as a sentence.
#[tauri::command]
pub(crate) fn device_support(
    state: State<'_, AppState>,
    model: Option<String>,
    capability: Capability,
) -> Result<SupportCheck, CommandError> {
    let localizer = state.localizer()?;
    Ok(SupportCheck {
        support: model.as_deref().map(|m| pmca_app::devices::support(m, capability)),
        text: pmca_app::devices::support_text(&localizer, model.as_deref(), capability),
    })
}

/// The answer of [`device_support`]; `support` is `None` for an unidentified camera.
#[derive(serde::Serialize)]
pub(crate) struct SupportCheck {
    support: Option<Support>,
    text: String,
}

/// Ask where the camera's usage log should go, then download and write it.
///
/// The log's structure is undocumented, so it is stored verbatim; the byte
/// count is all this layer can honestly report about it. The save dialog is
/// opened here so the window never supplies a path to write to; `None` when it
/// was dismissed, in which case the camera is not touched.
#[tauri::command]
pub(crate) async fn save_usage_log(
    app: AppHandle,
    window: Window,
) -> Result<Option<UsageLogOutcome>, CommandError> {
    let dialog_app = app.clone();
    let chosen = in_background(move || {
        let state = dialog_app.state::<AppState>();
        ask_save_path(&dialog_app, &window, state.inner(), "usage-log", "Usagelog.bin")
    })
    .await?;
    let Some(path) = chosen else { return Ok(None) };
    with_camera(app, move |_, _| {
        let log = pmca_app::usage_log()?;
        std::fs::write(&path, &log)?;
        Ok(Some(UsageLogOutcome { path: absolute_display(&path), bytes: log.len() }))
    })
    .await
}

/// Result of writing the usage log, reported with the path actually used.
#[derive(Debug, serde::Serialize)]
pub struct UsageLogOutcome {
    /// Absolute path of the file that was written.
    pub path: String,
    /// Size of the log in bytes.
    pub bytes: usize,
}

/// Ask the camera to reconnect in mass-storage mode.
///
/// Offered by the front-end below a `wrong-mode` failure that carries the
/// `switch-to-mass-storage` recovery action. This physically re-enumerates the
/// camera and is one-way: only a power cycle or replug brings it back to the
/// USB mode configured in its own menu.
#[tauri::command]
pub(crate) async fn switch_camera_mode(app: AppHandle) -> Result<(), CommandError> {
    with_camera(app, |app, state| {
        let localizer = state.localizer()?;
        Ok(pmca_app::switch_to_mass_storage(&localizer, &mut progress(app))?)
    })
    .await
}

/// Start the elevated driver helper: the one rights prompt of this run.
///
/// Offered by the rights badge and below a `needs-elevation` failure. The
/// window itself never runs elevated; only the helper does, and it ends with
/// the window.
#[tauri::command]
pub(crate) async fn start_driver_helper(app: AppHandle) -> Result<(), CommandError> {
    in_background(move || {
        let state = app.state::<AppState>();
        Ok(pmca_app::drivers::start_helper(&state.localizer()?)?)
    })
    .await
}

/// Which rights driver tasks run with: `elevated`, `helper` or `standard`.
#[tauri::command]
pub(crate) fn driver_rights() -> pmca_app::drivers::Rights {
    pmca_app::drivers::rights()
}

/// Driver packages Zadig or a similar tool left for Sony devices. Reads the
/// driver database only, so it needs no rights and no camera.
#[tauri::command]
pub(crate) fn foreign_drivers() -> Vec<pmca_app::drivers::ForeignDriver> {
    pmca_app::drivers::foreign_drivers()
}

/// Remove those packages after the user agreed. Asks for administrator rights
/// when the helper is not running yet, and holds the camera meanwhile, since
/// a device bound to one of them is re-enumerated.
#[tauri::command]
pub(crate) async fn remove_foreign_drivers(app: AppHandle) -> Result<Vec<String>, CommandError> {
    with_camera_as_is(app, |_, state| {
        Ok(pmca_app::drivers::remove_foreign_drivers(&state.localizer()?)?)
    })
    .await
}

/// Set the title bar text; the front-end adds the rights in the user's language.
#[tauri::command]
pub(crate) fn set_window_title(window: Window, title: String) -> Result<(), CommandError> {
    window.set_title(&title).map_err(CommandError::internal)
}
