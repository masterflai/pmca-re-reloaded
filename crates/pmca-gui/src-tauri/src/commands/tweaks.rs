//! Tweaks: list, apply, undo, close.

use crate::*;

/// Read the tweaks through the view's service-mode session, opening it if
/// necessary.
pub(crate) fn read_tweaks(app: &AppHandle, state: &AppState) -> Result<TweakReport, CommandError> {
    let localizer = state.localizer()?;
    let mut guard = state.tweaks.lock().map_err(CommandError::internal)?;
    Ok(pmca_app::tweaks::with_session(
        &mut guard,
        &localizer,
        &mut progress(app),
        |session, sink| session.read(&localizer, sink),
    )?)
}

/// Read the connected camera's tweakable settings.
///
/// This only reads; the camera is left unmodified. The report also carries the
/// camera's identifying values, which come from the same backup image.
///
/// The service-mode session is kept afterwards, so applying a change straight
/// after listing does not have to fetch the camera back into service mode. It
/// is closed by [`tweaks_close`], which the view calls when the user leaves it.
#[tauri::command]
pub(crate) async fn list_tweaks(app: AppHandle) -> Result<TweakReport, CommandError> {
    with_camera(app, read_tweaks).await
}

/// Apply tweak changes to the connected camera and report the new state.
///
/// This rewrites the camera's backup memory. The confirmation prompt lives in
/// the front-end; an empty change set is answered with a plain read so a stray
/// invocation cannot cause a write.
#[tauri::command]
pub(crate) async fn apply_tweaks(
    app: AppHandle,
    changes: Vec<TweakChange>,
) -> Result<TweakReport, CommandError> {
    with_camera(app, move |app, state| {
        if changes.is_empty() {
            return read_tweaks(app, state);
        }
        let localizer = state.localizer()?;
        let mut guard = state.tweaks.lock().map_err(CommandError::internal)?;
        Ok(pmca_app::tweaks::with_session(
            &mut guard,
            &localizer,
            &mut progress(app),
            |session, sink| session.apply(&localizer, &changes, sink),
        )?)
    })
    .await
}

/// [`apply_tweaks`] through the whole backup image, for a camera that refused
/// the changes one value at a time. The front-end asks for its own
/// confirmation first; the image read beforehand is saved by `pmca-app`.
#[tauri::command]
pub(crate) async fn apply_tweaks_whole_image(
    app: AppHandle,
    changes: Vec<TweakChange>,
) -> Result<TweakReport, CommandError> {
    with_camera(app, move |app, state| {
        if changes.is_empty() {
            return read_tweaks(app, state);
        }
        let localizer = state.localizer()?;
        let mut guard = state.tweaks.lock().map_err(CommandError::internal)?;
        Ok(pmca_app::tweaks::with_session(
            &mut guard,
            &localizer,
            &mut progress(app),
            |session, sink| session.apply_whole_image(&localizer, &changes, sink),
        )?)
    })
    .await
}

/// Write back the values saved before the first per-property tweak change.
#[tauri::command]
pub(crate) async fn undo_tweaks(app: AppHandle) -> Result<TweakReport, CommandError> {
    with_camera(app, |app, state| {
        let localizer = state.localizer()?;
        let mut guard = state.tweaks.lock().map_err(CommandError::internal)?;
        Ok(pmca_app::tweaks::with_session(
            &mut guard,
            &localizer,
            &mut progress(app),
            |session, sink| session.undo(&localizer, sink),
        )?)
    })
    .await
}

/// End the tweak view's service-mode session.
///
/// Called when the user leaves the view. Nothing is open in the normal case
/// and this does nothing, which is the point: the front-end cannot know
/// whether a session was ever started.
#[tauri::command]
pub(crate) async fn tweaks_close(app: AppHandle) -> Result<(), CommandError> {
    with_camera_as_is(app, |_, state| {
        state.tweaks.lock().map_err(CommandError::internal)?.take();
        Ok(())
    })
    .await
}
