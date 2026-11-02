//! Saving the camera's whole backup image and writing one back.
//!
//! Saving opens a save dialog here, so the only path it writes to is one the
//! user picked. Restoring takes the path the view already let the user choose
//! (dropped or browsed), so the file is named and shown before the warnings.

use pmca_app::backup::{BackupRestoreOutcome, BackupSaveOutcome};

use crate::*;

/// Read the camera's backup image and save it to a file the user picks.
///
/// `None` when the save dialog was dismissed.
#[tauri::command(async)]
pub(crate) fn backup_save(
    app: AppHandle,
    window: Window,
    state: State<'_, AppState>,
) -> Result<Option<BackupSaveOutcome>, CommandError> {
    let Some(path) = ask_save_path(&app, &window, &state, "backup", "camera-backup.bin")? else {
        return Ok(None);
    };
    let _camera = state.lock_camera();
    if !state.holds_session() {
        pmca_app::service_mode::release_leftovers(&state.localizer()?)?;
    }
    #[cfg(target_os = "macos")]
    {
        let relayed =
            remote::call(&app, &state, remote::Request::BackupSave { path: path.clone() })?;
        pmca_app::ownership::return_paths(&[path]);
        serde_json::from_value(relayed).map(Some).map_err(CommandError::internal)
    }
    #[cfg(not(target_os = "macos"))]
    {
        let l = state.localizer()?;
        Ok(Some(pmca_app::backup::save(&l, &mut progress(&app), &path)?))
    }
}

/// Write the backup image at `path` back to the connected camera.
///
/// The view chose and showed `path` before asking for confirmation, so this
/// opens no dialog of its own.
#[tauri::command]
pub(crate) async fn backup_restore(
    app: AppHandle,
    path: String,
) -> Result<BackupRestoreOutcome, CommandError> {
    with_camera(app, move |app, state| {
        let path = std::path::PathBuf::from(&path);
        #[cfg(target_os = "macos")]
        {
            let relayed = remote::call(app, state, remote::Request::BackupRestore { path })?;
            pmca_app::ownership::return_user_data();
            serde_json::from_value(relayed).map_err(CommandError::internal)
        }
        #[cfg(not(target_os = "macos"))]
        {
            let l = state.localizer()?;
            Ok(pmca_app::backup::restore(&l, &mut progress(app), &path)?)
        }
    })
    .await
}
