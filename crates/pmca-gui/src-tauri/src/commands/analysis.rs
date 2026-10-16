//! Function analysis.

use crate::*;

/// Run `step` on the analysis under way, then save its report once there is
/// something to report, and return where the run stands.
pub(crate) fn with_analysis(
    app: &AppHandle,
    state: &AppState,
    step: impl FnOnce(
        &mut pmca_app::analysis::AnalysisRun,
        &Localizer,
        &mut dyn pmca_app::ProgressSink,
    ) -> Result<(), CommandError>,
) -> Result<pmca_app::analysis::RunState, CommandError> {
    let localizer = state.localizer()?;
    let mut guard = state.analysis.lock().map_err(CommandError::internal)?;
    let run = guard
        .as_mut()
        .ok_or_else(|| CommandError::localized(localizer.tr("analysis-write-not-allowed")))?;
    let result = step(run, &localizer, &mut progress(app));
    // Saved after failures too: a report that stops half-way still says where.
    if run.taken() > 0 {
        let _ = run.save_report();
    }
    result?;
    Ok(run.state(&localizer))
}

/// The analysis under way, if any. Lets the view pick up where it left off.
#[tauri::command]
pub(crate) fn analysis_state(
    state: State<'_, AppState>,
) -> Result<Option<pmca_app::analysis::RunState>, CommandError> {
    let localizer = state.localizer()?;
    let guard = state.analysis.lock().map_err(CommandError::internal)?;
    Ok(guard.as_ref().map(|run| run.state(&localizer)))
}

/// Start an analysis. Touches no camera.
///
/// Refused while a written value is still waiting to be restored.
#[tauri::command]
pub(crate) fn analysis_start(
    state: State<'_, AppState>,
    setting: String,
    values: Vec<String>,
    write_test: bool,
) -> Result<pmca_app::analysis::RunState, CommandError> {
    let localizer = state.localizer()?;
    let mut guard = state.analysis.lock().map_err(CommandError::internal)?;
    if guard.as_ref().is_some_and(|run| run.needs_restore()) {
        return Err(CommandError::localized(localizer.tr("analysis-pending")));
    }
    let run = pmca_app::analysis::AnalysisRun::new(&localizer, &setting, &values, write_test)?;
    let run = guard.insert(run);
    Ok(run.state(&localizer))
}

/// Read the complete backup once, in a service session of its own.
#[tauri::command]
pub(crate) async fn analysis_snapshot(
    app: AppHandle,
) -> Result<pmca_app::analysis::RunState, CommandError> {
    with_camera(app, |app, state| {
        with_analysis(app, state, |run, l, sink| {
            run.take_snapshot(l, sink)?;
            Ok(())
        })
    })
    .await
}

/// Write the value found for the second menu value. The front-end asks for
/// consent twice before calling this; the run refuses without the first.
#[tauri::command]
pub(crate) async fn analysis_write(
    app: AppHandle,
) -> Result<pmca_app::analysis::RunState, CommandError> {
    with_camera(app, |app, state| {
        with_analysis(app, state, |run, l, sink| {
            run.write(l, sink)?;
            Ok(())
        })
    })
    .await
}

/// Note what the camera's menu showed.
#[tauri::command]
pub(crate) fn analysis_menu(
    app: AppHandle,
    state: State<'_, AppState>,
    observed: String,
) -> Result<pmca_app::analysis::RunState, CommandError> {
    with_analysis(&app, &state, |run, _, _| {
        run.record_menu(&observed);
        Ok(())
    })
}

/// Write the original value back.
#[tauri::command]
pub(crate) async fn analysis_restore(
    app: AppHandle,
) -> Result<pmca_app::analysis::RunState, CommandError> {
    with_camera(app, |app, state| {
        with_analysis(app, state, |run, l, sink| {
            run.restore(l, sink)?;
            Ok(())
        })
    })
    .await
}

/// Read the candidates from another camera. Only reads.
#[tauri::command]
pub(crate) async fn analysis_cross_check(
    app: AppHandle,
) -> Result<pmca_app::analysis::RunState, CommandError> {
    with_camera(app, |app, state| {
        with_analysis(app, state, |run, l, sink| {
            run.cross_check(l, sink)?;
            Ok(())
        })
    })
    .await
}

/// Ask where the report should go and write it there.
///
/// The dialog is opened here rather than in the window so that the only path
/// this command ever writes to is one the user picked in a native dialog.
/// `None` when the dialog was dismissed.
#[tauri::command(async)]
pub(crate) fn analysis_export(
    app: AppHandle,
    window: Window,
    state: State<'_, AppState>,
) -> Result<Option<String>, CommandError> {
    let json = {
        let guard = state.analysis.lock().map_err(CommandError::internal)?;
        let run = guard.as_ref().ok_or_else(|| CommandError::internal("no analysis"))?;
        run.report_json()?
    };
    let Some(path) = ask_save_path(&app, &window, &state, "analysis", "pmca-analysis.json")? else {
        return Ok(None);
    };
    std::fs::write(&path, json)?;
    Ok(Some(absolute_display(&path)))
}

/// Forget the analysis under way. Refused while a written value is still
/// waiting to be restored.
#[tauri::command]
pub(crate) fn analysis_discard(state: State<'_, AppState>) -> Result<(), CommandError> {
    let localizer = state.localizer()?;
    let mut guard = state.analysis.lock().map_err(CommandError::internal)?;
    if guard.as_ref().is_some_and(|run| run.needs_restore()) {
        return Err(CommandError::localized(localizer.tr("analysis-pending")));
    }
    guard.take();
    Ok(())
}

/// Whether an earlier write test left a value that was never restored.
#[tauri::command]
pub(crate) fn analysis_pending(state: State<'_, AppState>) -> Result<bool, CommandError> {
    let guard = state.analysis.lock().map_err(CommandError::internal)?;
    // The run under way restores its own value.
    Ok(guard.is_none() && pmca_app::analysis::has_pending_restore())
}

/// Write back what an interrupted write test left on the connected camera.
#[tauri::command]
pub(crate) async fn analysis_restore_pending(app: AppHandle) -> Result<usize, CommandError> {
    with_camera(app, |app, state| {
        let localizer = state.localizer()?;
        Ok(pmca_app::analysis::restore_pending(&localizer, &mut progress(app))?)
    })
    .await
}
