//! Service shell and updater shell sessions.

use crate::*;

/// The service shell's command reference.
///
/// Available without a camera, and without a session: the point of a reference
/// is to be readable before deciding what to type.
#[tauri::command]
pub(crate) fn service_help() -> &'static str {
    pmca_app::service::HELP
}

/// Open a service-mode session and keep it for the commands that follow.
///
/// Getting there takes a mode switch, a re-enumeration and an authentication
/// step, so progress is streamed over [`PROGRESS_EVENT`] exactly as the longer
/// camera operations do.
#[tauri::command]
pub(crate) async fn service_open(app: AppHandle) -> Result<(), CommandError> {
    with_camera(app, |app, state| {
        let localizer = state.localizer()?;
        let mut guard = state.service.lock().map_err(CommandError::internal)?;
        // Opening twice would leave the first connection behind with no way to
        // close it, so the existing one is ended first.
        guard.take();
        *guard = Some(pmca_app::service::ServiceSession::open(&localizer, &mut progress(app))?);
        Ok(())
    })
    .await
}

/// Run one line of input against the open session.
///
/// A line is parsed and executed by [`pmca_app::service`], the same code the
/// console runs, so the two cannot disagree about what a command means.
#[tauri::command]
pub(crate) async fn service_run(
    app: AppHandle,
    line: String,
) -> Result<Option<pmca_app::service::ServiceOutcome>, CommandError> {
    with_camera(app, move |_, state| {
        let localizer = state.localizer()?;
        let mut guard = state.service.lock().map_err(CommandError::internal)?;
        let session = guard.as_mut().ok_or_else(|| CommandError {
            code: "no-service-session".into(),
            message: localizer.tr("service-not-connected"),
            detail: None,
            recovery: None,
        })?;
        let outcome = session.run_line(&localizer, &line)?;
        // `exit` is a command like any other, and the session it ends is this one.
        if outcome.as_ref().is_some_and(|o| o.finished) {
            guard.take();
        }
        Ok(outcome)
    })
    .await
}

/// Leave service mode.
///
/// Doing nothing when no session is open is deliberate: this is also the
/// front-end's cleanup path, which cannot always know whether the session is
/// still there.
#[tauri::command]
pub(crate) async fn service_close(app: AppHandle) -> Result<(), CommandError> {
    with_camera_as_is(app, |_, state| {
        state.service.lock().map_err(CommandError::internal)?.take();
        Ok(())
    })
    .await
}

/// Turn the camera's debug terminal on (`enable`) or off.
///
/// The serial console is polled with [`service_read_terminal`] and written to
/// with [`service_write_terminal`]; this just opens or closes it.
#[tauri::command]
pub(crate) async fn service_set_terminal(app: AppHandle, enable: bool) -> Result<(), CommandError> {
    with_camera_as_is(app, move |_, state| {
        let localizer = state.localizer()?;
        let mut guard = state.service.lock().map_err(CommandError::internal)?;
        let session = guard.as_mut().ok_or_else(|| service_not_connected(&localizer))?;
        if enable {
            session.enable_terminal()?;
        } else {
            session.disable_terminal()?;
        }
        Ok(())
    })
    .await
}

/// Poll the debug terminal for output, as bytes the front-end decodes as UTF-8.
///
/// Returns whatever was waiting, which may be nothing. The front-end drives the
/// loop so the window stays responsive while the console is idle.
#[tauri::command]
pub(crate) async fn service_read_terminal(app: AppHandle) -> Result<Vec<u8>, CommandError> {
    with_camera_as_is(app, |_, state| {
        let localizer = state.localizer()?;
        let mut guard = state.service.lock().map_err(CommandError::internal)?;
        let session = guard.as_mut().ok_or_else(|| service_not_connected(&localizer))?;
        Ok(session.read_terminal())
    })
    .await
}

/// Send a typed line to the debug terminal.
#[tauri::command]
pub(crate) async fn service_write_terminal(
    app: AppHandle,
    data: Vec<u8>,
) -> Result<(), CommandError> {
    with_camera_as_is(app, move |_, state| {
        let localizer = state.localizer()?;
        let mut guard = state.service.lock().map_err(CommandError::internal)?;
        let session = guard.as_mut().ok_or_else(|| service_not_connected(&localizer))?;
        session.write_terminal(&data)?;
        Ok(())
    })
    .await
}

/// The error for a terminal command sent with no session open.
fn service_not_connected(localizer: &Localizer) -> CommandError {
    CommandError {
        code: "no-service-session".into(),
        message: localizer.tr("service-not-connected"),
        detail: None,
        recovery: None,
    }
}

/// The updater shell's command reference.
///
/// A separate reference from the service shell's: the two speak different
/// protocols, and the commands one of them offers the other does not have.
#[tauri::command]
pub(crate) fn updater_help() -> &'static str {
    pmca_app::updater::HELP
}

/// Reboot the camera into updater mode, inject the payload and keep the shell.
///
/// `confirm` carries the user's agreement to modify the camera, exactly as the
/// firmware view's confirmation does. Without it the flow stops after the
/// non-destructive checks and reports what it would have done.
///
/// There is no model parameter: the window reads the model from the camera
/// while it is still in normal mode. Naming one by hand stays a console
/// option, for a camera that already sits in updater mode.
#[tauri::command]
pub(crate) async fn updater_open(
    app: AppHandle,
    fdat_file: Option<String>,
    confirm: bool,
) -> Result<(), CommandError> {
    with_camera(app, move |app, state| {
        let localizer = state.localizer()?;
        let mut guard = state.updater.lock().map_err(CommandError::internal)?;
        // Opening twice would leave the first session behind with no way to
        // close it, so the existing one is ended first.
        guard.take();
        let request = pmca_app::updater::OpenRequest {
            model: None,
            fdat_file: fdat_file.filter(|p| !p.is_empty()).map(PathBuf::from),
            fdat_dir: state.fdat_root.clone(),
            flash: confirm,
        };
        *guard = Some(pmca_app::updater::UpdaterSession::open(
            &localizer,
            &request,
            &mut progress(app),
        )?);
        Ok(())
    })
    .await
}

/// Run one line of input against the open updater session.
///
/// Parsed and executed by [`pmca_app::updater`], the same code the console
/// runs. The one command it will not run here is `shell`, which needs a
/// terminal; it answers with the console invocation instead.
#[tauri::command]
pub(crate) async fn updater_run(
    app: AppHandle,
    line: String,
) -> Result<Option<pmca_app::updater::UpdaterOutcome>, CommandError> {
    with_camera(app, move |_, state| {
        let localizer = state.localizer()?;
        let mut guard = state.updater.lock().map_err(CommandError::internal)?;
        let session = guard.as_mut().ok_or_else(|| CommandError {
            code: "no-updater-session".into(),
            message: localizer.tr("service-not-connected"),
            detail: None,
            recovery: None,
        })?;
        let outcome = session.run_line(&localizer, &line)?;
        // `exit` is a command like any other, and the session it ends is this one.
        if outcome.as_ref().is_some_and(|o| o.finished) {
            guard.take();
        }
        Ok(outcome)
    })
    .await
}

/// End the updater session, letting the firmware write finalise.
///
/// Doing nothing when no session is open is deliberate: this is also the
/// front-end's cleanup path, which cannot always know whether the session is
/// still there.
#[tauri::command]
pub(crate) async fn updater_close(app: AppHandle) -> Result<(), CommandError> {
    with_camera_as_is(app, |_, state| {
        state.updater.lock().map_err(CommandError::internal)?.take();
        Ok(())
    })
    .await
}
