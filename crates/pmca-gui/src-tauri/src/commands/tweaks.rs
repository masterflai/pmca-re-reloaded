//! Tweaks: list, apply, undo, close.
//!
//! On macOS the session lives in the elevated broker; see [`crate::remote`].

use crate::*;

/// Read the tweaks through the view's service-mode session, opening it if
/// necessary.
#[cfg_attr(target_os = "macos", allow(dead_code))]
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
pub(crate) async fn list_tweaks(app: AppHandle) -> Result<Reply<TweakReport>, CommandError> {
    with_camera(app, |app, state| {
        #[cfg(target_os = "macos")]
        {
            remote::call(app, state, remote::Request::TweaksRead).map(Reply::Relayed)
        }
        #[cfg(not(target_os = "macos"))]
        {
            read_tweaks(app, state).map(Reply::Here)
        }
    })
    .await
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
) -> Result<Reply<TweakReport>, CommandError> {
    with_camera(app, move |app, state| {
        #[cfg(target_os = "macos")]
        {
            remote::call(app, state, remote::Request::TweaksApply { changes }).map(Reply::Relayed)
        }
        #[cfg(not(target_os = "macos"))]
        {
            if changes.is_empty() {
                return read_tweaks(app, state).map(Reply::Here);
            }
            let localizer = state.localizer()?;
            let mut guard = state.tweaks.lock().map_err(CommandError::internal)?;
            Ok(Reply::Here(pmca_app::tweaks::with_session(
                &mut guard,
                &localizer,
                &mut progress(app),
                |session, sink| session.apply(&localizer, &changes, sink),
            )?))
        }
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
) -> Result<Reply<TweakReport>, CommandError> {
    with_camera(app, move |app, state| {
        #[cfg(target_os = "macos")]
        {
            remote::call(app, state, remote::Request::TweaksApplyWholeImage { changes })
                .map(Reply::Relayed)
        }
        #[cfg(not(target_os = "macos"))]
        {
            if changes.is_empty() {
                return read_tweaks(app, state).map(Reply::Here);
            }
            let localizer = state.localizer()?;
            let mut guard = state.tweaks.lock().map_err(CommandError::internal)?;
            Ok(Reply::Here(pmca_app::tweaks::with_session(
                &mut guard,
                &localizer,
                &mut progress(app),
                |session, sink| session.apply_whole_image(&localizer, &changes, sink),
            )?))
        }
    })
    .await
}

/// Write back the values saved before the first per-property tweak change.
#[tauri::command]
pub(crate) async fn undo_tweaks(app: AppHandle) -> Result<Reply<TweakReport>, CommandError> {
    with_camera(app, |app, state| {
        #[cfg(target_os = "macos")]
        {
            remote::call(app, state, remote::Request::TweaksUndo).map(Reply::Relayed)
        }
        #[cfg(not(target_os = "macos"))]
        {
            let localizer = state.localizer()?;
            let mut guard = state.tweaks.lock().map_err(CommandError::internal)?;
            Ok(Reply::Here(pmca_app::tweaks::with_session(
                &mut guard,
                &localizer,
                &mut progress(app),
                |session, sink| session.undo(&localizer, sink),
            )?))
        }
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
    with_camera_as_is(app, |_app, state| {
        #[cfg(target_os = "macos")]
        remote::call_if_running(_app, state, remote::Request::TweaksClose)?;
        state.tweaks.lock().map_err(CommandError::internal)?.take();
        Ok(())
    })
    .await
}
