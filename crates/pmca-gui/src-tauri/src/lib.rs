//! Tauri backend for the PMCA-RE RELOADED GUI.
//!
//! Every command is a thin wrapper around [`pmca_app`], which holds the shared
//! orchestration logic used by the CLI as well. Commands fail with
//! [`CommandError`], which pairs a stable code with the backend's own wording
//! so the window can translate the failure without parsing English text.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard, PoisonError};

use pmca_app::devices::{Capability, DeviceLists, Support};
use pmca_app::dto::{
    AppBackupOutcome, CameraSummary, CommunityList, FirmwareOutcome, InstallOutcome, PhotoSummary,
    ProgressEvent, StreamingSettings, TweakChange, TweakReport, WifiAccessPoint,
};
use pmca_app::{ApkSource, InstallRequest, InstallSeries};
use pmca_i18n::Localizer;
use tauri::{AppHandle, Emitter, Manager, State, Window};
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons, MessageDialogKind};

mod commands;
mod remote;

use commands::*;
pub use remote::run_broker;

/// A command's result, computed here or relayed as JSON from the macOS broker.
///
/// Both serialize to the same JSON, so the window cannot tell them apart.
#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum Reply<T> {
    #[cfg_attr(target_os = "macos", allow(dead_code))]
    Here(T),
    #[cfg_attr(not(target_os = "macos"), allow(dead_code))]
    Relayed(serde_json::Value),
}

/// Event name the front-end subscribes to for installation progress.
const PROGRESS_EVENT: &str = "pmca://progress";

/// A failure handed to the webview.
///
/// The front-end looks up `err-{code}` in its catalogue and appends `detail`;
/// `message` is the untranslated wording it falls back to when no such key
/// exists. Sending all three keeps translation a front-end concern while
/// guaranteeing that the user still sees *something* meaningful for a code that
/// has not been translated yet.
#[derive(Debug, serde::Serialize)]
pub struct CommandError {
    /// Stable identifier of the failure, never the English text.
    pub code: String,
    /// The backend's own wording, used when the code has no translation.
    pub message: String,
    /// Variable part of the message (a path, a status byte, a device error).
    pub detail: Option<String>,
    /// An action the front-end may offer to make the operation possible, or
    /// `None` when the user has to intervene on the camera itself.
    pub recovery: Option<String>,
}

impl CommandError {
    /// Wrap a message that is already localized and should be shown verbatim.
    ///
    /// The `operation` code intentionally has no translation key, so the
    /// front-end falls through to `message`.
    fn localized(message: String) -> Self {
        Self { code: "operation".into(), message, detail: None, recovery: None }
    }

    /// Report a failure of the GUI process itself, such as a poisoned lock.
    ///
    /// These are bugs rather than conditions the user can act on, so they are
    /// reported under one code instead of being enumerated.
    fn internal<E: std::fmt::Display>(e: E) -> Self {
        let message = e.to_string();
        Self {
            code: "internal".into(),
            message: message.clone(),
            detail: Some(message),
            recovery: None,
        }
    }
}

impl From<pmca_app::Error> for CommandError {
    fn from(e: pmca_app::Error) -> Self {
        (&e).into()
    }
}

impl From<&pmca_app::Error> for CommandError {
    fn from(e: &pmca_app::Error) -> Self {
        pmca_app::broker::RemoteError::from(e).into()
    }
}

impl From<pmca_app::broker::RemoteError> for CommandError {
    fn from(e: pmca_app::broker::RemoteError) -> Self {
        Self { code: e.code, message: e.message, detail: e.detail, recovery: e.recovery }
    }
}

impl From<std::io::Error> for CommandError {
    fn from(e: std::io::Error) -> Self {
        let message = e.to_string();
        Self { code: "io".into(), message: message.clone(), detail: Some(message), recovery: None }
    }
}

/// Process-wide state: the selected locale plus the resolved asset paths.
///
/// The [`Localizer`] itself is *not* kept here: Fluent's bundles are `Rc`-based
/// and therefore neither `Send` nor `Sync`, while Tauri requires managed state
/// to be both. Building one per command is cheap and keeps the state simple.
struct AppState {
    locale: Mutex<String>,
    /// Root of the firmware header tree used by the firmware view.
    fdat_root: PathBuf,
    /// The open service-mode session, if the user has started one.
    ///
    /// The only long-lived thing this backend owns. Every other operation
    /// opens the camera, does its work and closes it again; a service shell is
    /// a conversation, so the connection has to outlive the command that
    /// started it. It lives behind a lock because Tauri runs commands on a
    /// thread pool and the camera cannot serve two at once.
    service: Mutex<Option<pmca_app::service::ServiceSession>>,
    /// The open updater-mode session, if the user has started one.
    ///
    /// Kept separately from [`Self::service`] because the two are different
    /// conversations with different payloads, and because holding one lock for
    /// both would let an idle service shell block the updater shell.
    updater: Mutex<Option<pmca_app::updater::UpdaterSession>>,
    /// The service-mode session the tweak view is working through, if any.
    ///
    /// Unlike the two above, this one is not something the user opens and
    /// closes: it is held purely so listing the tweaks and then applying a
    /// change do not each pay for a mode switch and two driver assignments.
    /// The view closes it when the user navigates away.
    tweaks: Mutex<Option<pmca_app::tweaks::TweakSession>>,
    /// The function analysis under way, if any.
    ///
    /// Holds no camera session: every step opens and ends its own. It is kept
    /// across navigation so a write test cannot be lost by leaving the view
    /// before its value is restored.
    analysis: Mutex<Option<pmca_app::analysis::AnalysisRun>>,
    /// Store apps downloaded ahead of a batch installation, by source and package.
    ///
    /// A batch fetches every selected app before it touches the camera, so a
    /// network failure cannot leave the camera stranded in app-install mode.
    /// The bytes wait here rather than in the webview, which would have to
    /// send them back through the IPC channel one app at a time.
    downloads: Mutex<HashMap<String, Vec<u8>>>,
    /// Plugins downloaded from a source, waiting for the user's agreement.
    plugin_downloads: Mutex<HashMap<String, Vec<u8>>>,
    /// The batch installation under way, if any.
    ///
    /// Its tasks leave the camera in app-install mode; the window ends the
    /// series with [`finish_install_series`], which tells the user so once.
    install_series: Mutex<Option<InstallSeries>>,
    /// Held by every operation that talks to the camera; `true` once the
    /// start-up sweep has run. See [`with_camera`].
    camera: Mutex<bool>,
    /// macOS: the elevated copy holding the service-mode work, once started.
    #[cfg(target_os = "macos")]
    broker: Mutex<Option<pmca_app::broker::Broker>>,
}

impl AppState {
    fn new(fdat_root: PathBuf) -> Self {
        Self {
            locale: Mutex::new("en-US".to_string()),
            fdat_root,
            service: Mutex::new(None),
            updater: Mutex::new(None),
            tweaks: Mutex::new(None),
            analysis: Mutex::new(None),
            downloads: Mutex::new(HashMap::new()),
            plugin_downloads: Mutex::new(HashMap::new()),
            install_series: Mutex::new(None),
            camera: Mutex::new(false),
            #[cfg(target_os = "macos")]
            broker: Mutex::new(None),
        }
    }

    /// Build a localizer for the currently selected language.
    fn localizer(&self) -> Result<Localizer, CommandError> {
        let locale = self.locale.lock().map_err(CommandError::internal)?;
        Ok(Localizer::new(&locale))
    }

    /// Take the camera for one operation, running the start-up sweep first if
    /// nothing has yet.
    fn lock_camera(&self) -> MutexGuard<'_, bool> {
        // A panic in one operation must not lock the camera away for good.
        let mut swept = self.camera.lock().unwrap_or_else(PoisonError::into_inner);
        if !*swept {
            sweep_leftovers();
            *swept = true;
        }
        swept
    }

    /// Whether a service, updater or tweak session is open, whose driver
    /// bindings are in use rather than left behind. A poisoned lock counts as
    /// open, so its bindings are never taken away underneath it.
    fn holds_session(&self) -> bool {
        self.service.lock().map_or(true, |s| s.is_some())
            || self.updater.lock().map_or(true, |s| s.is_some())
            || self.tweaks.lock().map_or(true, |s| s.is_some())
    }
}

/// Undo whatever a previous run left behind.
///
/// A driver binding that outlives its session keeps the camera off the
/// mass-storage and updater paths, and a camera left in service mode refuses
/// the next session's authentication, so starting from a swept state is what
/// stops one crashed run from making the application look broken ever after.
fn sweep_leftovers() {
    let swept = pmca_app::service_mode::sweep_leftovers();
    if swept.is_empty() {
        return;
    }
    // Nothing in the window asked for this, so there is nowhere to show it but
    // the console, and the user's locale is not known this early.
    let l = Localizer::new("en-US");
    if swept.packages > 0 {
        eprintln!("{}", l.tr_arg("driver-swept", "count", &swept.packages.to_string()));
    }
    if swept.camera_restored {
        eprintln!("{}", l.tr("driver-swept-camera"));
    }
}

/// Run `work` on a worker thread so the window's event loop stays free.
///
/// Tauri runs a plain synchronous command on the main thread; anything slow
/// there turns the window "not responding" and holds back the progress events
/// that would explain the wait.
async fn in_background<T: Send + 'static>(
    work: impl FnOnce() -> Result<T, CommandError> + Send + 'static,
) -> Result<T, CommandError> {
    tauri::async_runtime::spawn_blocking(work).await.map_err(CommandError::internal)?
}

/// Run camera work in the background, strictly one operation at a time.
///
/// Mode switches and driver changes re-enumerate the camera, and an operation
/// starting in the middle of one finds it half gone or switches it again. One
/// lock around every camera command makes them queue instead of overlap.
async fn with_camera<T: Send + 'static>(
    app: AppHandle,
    work: impl FnOnce(&AppHandle, &AppState) -> Result<T, CommandError> + Send + 'static,
) -> Result<T, CommandError> {
    in_background(move || {
        let state = app.state::<AppState>();
        let _camera = state.lock_camera();
        if !state.holds_session() {
            pmca_app::service_mode::release_leftovers(&state.localizer()?)?;
        }
        work(&app, &state)
    })
    .await
}

/// [`with_camera`] without releasing leftover driver bindings first, for the
/// commands that must still work while that release is impossible: the
/// elevated restart that makes it possible, and closing a session.
async fn with_camera_as_is<T: Send + 'static>(
    app: AppHandle,
    work: impl FnOnce(&AppHandle, &AppState) -> Result<T, CommandError> + Send + 'static,
) -> Result<T, CommandError> {
    in_background(move || {
        let state = app.state::<AppState>();
        let _camera = state.lock_camera();
        work(&app, &state)
    })
    .await
}

/// A progress sink that forwards every event to the window.
fn progress(app: &AppHandle) -> impl FnMut(ProgressEvent) + '_ {
    move |event: ProgressEvent| {
        let _ = app.emit(PROGRESS_EVENT, event);
    }
}

/// Resolve a bundled resource, falling back to the repository layout during
/// `cargo tauri dev`, where no bundle exists yet.
fn resolve_resource(app: &AppHandle, relative: &str, dev_fallback: &Path) -> PathBuf {
    app.path()
        .resolve(relative, tauri::path::BaseDirectory::Resource)
        .ok()
        .filter(|p| p.exists())
        .unwrap_or_else(|| dev_fallback.to_path_buf())
}

/// Build and start the Tauri application.
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let handle = app.handle();
            let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
            let fdat_root =
                resolve_resource(handle, "bodyfs", &manifest.join("../../../updatershell/bodyfs"));
            // Before any Localizer: plugins may add languages and messages.
            pmca_app::plugins::load();
            app.manage(AppState::new(fdat_root));
            // Load whatever key material the user has already imported, so the
            // protected features find it. Missing keys are not an error here.
            pmca_app::keys::load_from_disk();
            // The helper's rights prompt opens in front of this window.
            #[cfg(windows)]
            if let Some(window) = app.get_webview_window("main")
                && let Ok(hwnd) = window.hwnd()
            {
                pmca_app::drivers::set_parent_window(hwnd.0 as isize);
            }

            // The sweep takes seconds and the window already exists, so it runs
            // off the main thread. Whichever comes first, this thread or the
            // first camera command, performs it under the camera lock; nothing
            // can reach the camera before it has finished.
            let handle = handle.clone();
            std::thread::spawn(move || drop(handle.state::<AppState>().lock_camera()));
            Ok(())
        })
        // A camera left in service mode stalls the next session's
        // authentication, and closing the window is the one way out of the
        // application that no command sees. The session is therefore ended
        // here as well, while the process is still alive to send the request.
        .on_window_event(|window, event| {
            if matches!(event, tauri::WindowEvent::Destroyed) {
                let state = window.state::<AppState>();
                // Let a running operation finish rather than cut the camera off
                // in the middle of a mode switch or a write.
                let _camera = state.camera.lock().unwrap_or_else(PoisonError::into_inner);
                if let Ok(mut guard) = state.service.lock() {
                    guard.take();
                }
                // The updater session holds the firmware write open; dropping
                // it lets the write finalise instead of leaving the camera
                // waiting for a chunk that will never arrive.
                if let Ok(mut guard) = state.updater.lock() {
                    guard.take();
                }
                // A tweak session held for the next button press is the same
                // hazard as an open service shell: the camera is in service
                // mode and stays there unless it is told otherwise.
                if let Ok(mut guard) = state.tweaks.lock() {
                    guard.take();
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            available_locales,
            locale_catalog,
            set_locale,
            catalog_status,
            export_catalog,
            import_catalog,
            reset_catalog,
            about_info,
            check_for_update,
            host_platform,
            check_interfering_services,
            get_camera_info,
            read_photo,
            device_lists,
            device_support,
            save_usage_log,
            pick_file,
            ask_choice,
            show_message,
            flash_firmware,
            list_community_apps,
            community_app_version,
            install_app,
            download_community_app,
            install_downloaded_app,
            finish_install_series,
            discard_downloads,
            backup_apps,
            list_tweaks,
            apply_tweaks,
            apply_tweaks_whole_image,
            undo_tweaks,
            tweaks_close,
            analysis_state,
            analysis_start,
            analysis_snapshot,
            analysis_write,
            analysis_menu,
            analysis_restore,
            analysis_cross_check,
            analysis_export,
            analysis_discard,
            analysis_pending,
            analysis_restore_pending,
            list_wifi,
            save_wifi,
            get_streaming_settings,
            save_streaming_settings,
            update_gps,
            switch_camera_mode,
            start_driver_helper,
            driver_rights,
            foreign_drivers,
            remove_foreign_drivers,
            set_window_title,
            keys_status,
            keys_import,
            plugins_list,
            plugins_inspect,
            plugins_install,
            plugins_remove,
            plugins_compatibility,
            plugins_entries,
            plugins_run_tool,
            plugins_run_diagnostic,
            sources_list,
            sources_add,
            sources_remove,
            source_repo,
            sources_search,
            list_community_plugins,
            download_community_plugin,
            install_downloaded_plugin,
            discard_plugin_download,
            service_help,
            service_open,
            service_run,
            service_close,
            service_set_terminal,
            service_read_terminal,
            service_write_terminal,
            updater_help,
            updater_open,
            updater_run,
            updater_close,
            run_tool,
        ])
        .run(tauri::generate_context!())
        .expect("failed to start the PMCA-RE RELOADED application");
}
