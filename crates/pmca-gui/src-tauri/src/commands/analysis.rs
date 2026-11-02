//! Function analysis.
//!
//! On macOS the run lives in the elevated broker; see [`crate::remote`].

use crate::*;

/// A step of the analysis, as both the window and the macOS broker run it.
pub(crate) type Step<'a> = Box<
    dyn FnOnce(
            &mut pmca_app::analysis::AnalysisRun,
            &Localizer,
            &mut dyn pmca_app::ProgressSink,
        ) -> Result<(), CommandError>
        + 'a,
>;

/// Run `step` on the analysis in `slot`, then save its report once there is
/// something to report, and return where the run stands.
pub(crate) fn analysis_step(
    slot: &mut Option<pmca_app::analysis::AnalysisRun>,
    localizer: &Localizer,
    sink: &mut dyn pmca_app::ProgressSink,
    step: Step<'_>,
) -> Result<pmca_app::analysis::RunState, CommandError> {
    let run = slot
        .as_mut()
        .ok_or_else(|| CommandError::localized(localizer.tr("analysis-write-not-allowed")))?;
    let result = step(run, localizer, sink);
    // Saved after failures too: a report that stops half-way still says where.
    if run.taken() > 0 {
        let _ = run.save_report();
    }
    result?;
    Ok(run.state(localizer))
}

/// [`analysis_step`] on the window's own run.
#[cfg_attr(target_os = "macos", allow(dead_code))]
pub(crate) fn with_analysis(
    app: &AppHandle,
    state: &AppState,
    step: Step<'_>,
) -> Result<pmca_app::analysis::RunState, CommandError> {
    let localizer = state.localizer()?;
    let mut guard = state.analysis.lock().map_err(CommandError::internal)?;
    analysis_step(&mut guard, &localizer, &mut progress(app), step)
}

/// A camera step: in the broker on macOS, here elsewhere.
fn camera_step(
    app: &AppHandle,
    state: &AppState,
    #[cfg_attr(not(target_os = "macos"), allow(unused_variables))] request: remote::Request,
    #[cfg_attr(target_os = "macos", allow(unused_variables))] step: Step<'_>,
) -> Result<Reply<pmca_app::analysis::RunState>, CommandError> {
    #[cfg(target_os = "macos")]
    {
        remote::call(app, state, request).map(Reply::Relayed)
    }
    #[cfg(not(target_os = "macos"))]
    {
        with_analysis(app, state, step).map(Reply::Here)
    }
}

/// The analysis under way, if any. Lets the view pick up where it left off.
#[tauri::command]
pub(crate) async fn analysis_state(
    app: AppHandle,
) -> Result<Option<Reply<pmca_app::analysis::RunState>>, CommandError> {
    in_background(move || {
        let state = app.state::<AppState>();
        #[cfg(target_os = "macos")]
        {
            // Not worth a password prompt: without a broker there is no run.
            let relayed = remote::call_if_running(&app, &state, remote::Request::AnalysisState)?;
            Ok(relayed.filter(|v| !v.is_null()).map(Reply::Relayed))
        }
        #[cfg(not(target_os = "macos"))]
        {
            let localizer = state.localizer()?;
            let guard = state.analysis.lock().map_err(CommandError::internal)?;
            Ok(guard.as_ref().map(|run| Reply::Here(run.state(&localizer))))
        }
    })
    .await
}

/// Start an analysis. Touches no camera; on macOS it starts the broker the run
/// will live in.
///
/// Refused while a written value is still waiting to be restored.
#[tauri::command]
pub(crate) async fn analysis_start(
    app: AppHandle,
    setting: String,
    values: Vec<String>,
    write_test: bool,
) -> Result<Reply<pmca_app::analysis::RunState>, CommandError> {
    in_background(move || {
        let state = app.state::<AppState>();
        #[cfg(target_os = "macos")]
        {
            let request = remote::Request::AnalysisStart { setting, values, write_test };
            remote::call(&app, &state, request).map(Reply::Relayed)
        }
        #[cfg(not(target_os = "macos"))]
        {
            let localizer = state.localizer()?;
            let mut guard = state.analysis.lock().map_err(CommandError::internal)?;
            Ok(Reply::Here(start_run(&mut guard, &localizer, &setting, &values, write_test)?))
        }
    })
    .await
}

/// Start a run in `slot`, unless the one there still has a value to restore.
pub(crate) fn start_run(
    slot: &mut Option<pmca_app::analysis::AnalysisRun>,
    localizer: &Localizer,
    setting: &str,
    values: &[String],
    write_test: bool,
) -> Result<pmca_app::analysis::RunState, CommandError> {
    if slot.as_ref().is_some_and(|run| run.needs_restore()) {
        return Err(CommandError::localized(localizer.tr("analysis-pending")));
    }
    let run = pmca_app::analysis::AnalysisRun::new(localizer, setting, values, write_test)?;
    Ok(slot.insert(run).state(localizer))
}

/// Read the complete backup once, in a service session of its own.
#[tauri::command]
pub(crate) async fn analysis_snapshot(
    app: AppHandle,
) -> Result<Reply<pmca_app::analysis::RunState>, CommandError> {
    with_camera(app, |app, state| {
        camera_step(
            app,
            state,
            remote::Request::AnalysisSnapshot,
            Box::new(|run, l, sink| {
                run.take_snapshot(l, sink)?;
                Ok(())
            }),
        )
    })
    .await
}

/// Write the value found for the second menu value. The front-end asks for
/// consent twice before calling this; the run refuses without the first.
#[tauri::command]
pub(crate) async fn analysis_write(
    app: AppHandle,
) -> Result<Reply<pmca_app::analysis::RunState>, CommandError> {
    with_camera(app, |app, state| {
        camera_step(
            app,
            state,
            remote::Request::AnalysisWrite,
            Box::new(|run, l, sink| {
                run.write(l, sink)?;
                Ok(())
            }),
        )
    })
    .await
}

/// Note what the camera's menu showed.
#[tauri::command]
pub(crate) async fn analysis_menu(
    app: AppHandle,
    observed: String,
) -> Result<Reply<pmca_app::analysis::RunState>, CommandError> {
    in_background(move || {
        let state = app.state::<AppState>();
        #[cfg(target_os = "macos")]
        {
            remote::call(&app, &state, remote::Request::AnalysisMenu { observed })
                .map(Reply::Relayed)
        }
        #[cfg(not(target_os = "macos"))]
        {
            with_analysis(
                &app,
                &state,
                Box::new(move |run, _, _| {
                    run.record_menu(&observed);
                    Ok(())
                }),
            )
            .map(Reply::Here)
        }
    })
    .await
}

/// Write the original value back.
#[tauri::command]
pub(crate) async fn analysis_restore(
    app: AppHandle,
) -> Result<Reply<pmca_app::analysis::RunState>, CommandError> {
    with_camera(app, |app, state| {
        camera_step(
            app,
            state,
            remote::Request::AnalysisRestore,
            Box::new(|run, l, sink| {
                run.restore(l, sink)?;
                Ok(())
            }),
        )
    })
    .await
}

/// Read the candidates from another camera. Only reads.
#[tauri::command]
pub(crate) async fn analysis_cross_check(
    app: AppHandle,
) -> Result<Reply<pmca_app::analysis::RunState>, CommandError> {
    with_camera(app, |app, state| {
        camera_step(
            app,
            state,
            remote::Request::AnalysisCrossCheck,
            Box::new(|run, l, sink| {
                run.cross_check(l, sink)?;
                Ok(())
            }),
        )
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
    let json = report_json(&app, &state)?;
    let Some(path) = ask_save_path(&app, &window, &state, "analysis", "pmca-analysis.json")? else {
        return Ok(None);
    };
    std::fs::write(&path, json)?;
    Ok(Some(absolute_display(&path)))
}

/// The report of the run under way, as JSON.
fn report_json(
    #[cfg_attr(not(target_os = "macos"), allow(unused_variables))] app: &AppHandle,
    state: &AppState,
) -> Result<String, CommandError> {
    #[cfg(target_os = "macos")]
    {
        let relayed = remote::call_if_running(app, state, remote::Request::AnalysisReportJson)?
            .ok_or_else(|| CommandError::internal("no analysis"))?;
        serde_json::from_value(relayed).map_err(CommandError::internal)
    }
    #[cfg(not(target_os = "macos"))]
    {
        let guard = state.analysis.lock().map_err(CommandError::internal)?;
        let run = guard.as_ref().ok_or_else(|| CommandError::internal("no analysis"))?;
        Ok(run.report_json()?)
    }
}

/// Forget the analysis under way. Refused while a written value is still
/// waiting to be restored.
#[tauri::command]
pub(crate) async fn analysis_discard(app: AppHandle) -> Result<(), CommandError> {
    in_background(move || {
        let state = app.state::<AppState>();
        #[cfg(target_os = "macos")]
        {
            remote::call_if_running(&app, &state, remote::Request::AnalysisDiscard)?;
            Ok(())
        }
        #[cfg(not(target_os = "macos"))]
        {
            let localizer = state.localizer()?;
            let mut guard = state.analysis.lock().map_err(CommandError::internal)?;
            discard_run(&mut guard, &localizer)
        }
    })
    .await
}

/// Empty `slot`, unless its run still has a value to restore.
pub(crate) fn discard_run(
    slot: &mut Option<pmca_app::analysis::AnalysisRun>,
    localizer: &Localizer,
) -> Result<(), CommandError> {
    if slot.as_ref().is_some_and(|run| run.needs_restore()) {
        return Err(CommandError::localized(localizer.tr("analysis-pending")));
    }
    slot.take();
    Ok(())
}

/// Whether an earlier write test left a value that was never restored.
#[tauri::command]
pub(crate) async fn analysis_pending(app: AppHandle) -> Result<bool, CommandError> {
    in_background(move || {
        let state = app.state::<AppState>();
        #[cfg(target_os = "macos")]
        let running = remote::call_if_running(&app, &state, remote::Request::AnalysisState)?
            .is_some_and(|v| !v.is_null());
        #[cfg(not(target_os = "macos"))]
        let running = state.analysis.lock().map_err(CommandError::internal)?.is_some();
        // The run under way restores its own value.
        Ok(!running && pmca_app::analysis::has_pending_restore())
    })
    .await
}

/// Write back what an interrupted write test left on the connected camera.
#[tauri::command]
pub(crate) async fn analysis_restore_pending(app: AppHandle) -> Result<Reply<usize>, CommandError> {
    with_camera(app, |app, state| {
        #[cfg(target_os = "macos")]
        {
            remote::call(app, state, remote::Request::AnalysisRestorePending).map(Reply::Relayed)
        }
        #[cfg(not(target_os = "macos"))]
        {
            let localizer = state.localizer()?;
            Ok(Reply::Here(pmca_app::analysis::restore_pending(&localizer, &mut progress(app))?))
        }
    })
    .await
}

// ---------------------------------------------------------------------------
// Reference check: verify a known finding against the connected camera.
// ---------------------------------------------------------------------------

use pmca_app::analysis::{ReferenceCheck, ReferenceEntry, ReferenceState};

/// A step of the reference check, as both the window and the macOS broker run it.
pub(crate) type ReferenceStep<'a> = Box<
    dyn FnOnce(
            &mut ReferenceCheck,
            &Localizer,
            &mut dyn pmca_app::ProgressSink,
        ) -> Result<(), CommandError>
        + 'a,
>;

/// Run `step` on the reference check in `slot` and return where it stands.
pub(crate) fn reference_step(
    slot: &mut Option<ReferenceCheck>,
    localizer: &Localizer,
    sink: &mut dyn pmca_app::ProgressSink,
    step: ReferenceStep<'_>,
) -> Result<ReferenceState, CommandError> {
    let run = slot
        .as_mut()
        .ok_or_else(|| CommandError::localized(localizer.tr("reference-not-started")))?;
    step(run, localizer, sink)?;
    Ok(run.state())
}

/// [`reference_step`] on the window's own check.
#[cfg_attr(target_os = "macos", allow(dead_code))]
pub(crate) fn with_reference(
    app: &AppHandle,
    state: &AppState,
    step: ReferenceStep<'_>,
) -> Result<ReferenceState, CommandError> {
    let localizer = state.localizer()?;
    let mut guard = state.reference.lock().map_err(CommandError::internal)?;
    reference_step(&mut guard, &localizer, &mut progress(app), step)
}

/// A camera step of the reference check: in the broker on macOS, here elsewhere.
fn reference_camera_step(
    app: &AppHandle,
    state: &AppState,
    #[cfg_attr(not(target_os = "macos"), allow(unused_variables))] request: remote::Request,
    #[cfg_attr(target_os = "macos", allow(unused_variables))] step: ReferenceStep<'_>,
) -> Result<Reply<ReferenceState>, CommandError> {
    #[cfg(target_os = "macos")]
    {
        remote::call(app, state, request).map(Reply::Relayed)
    }
    #[cfg(not(target_os = "macos"))]
    {
        with_reference(app, state, step).map(Reply::Here)
    }
}

/// Start a check in `slot`, unless the one there still has a value to restore.
pub(crate) fn start_reference(
    slot: &mut Option<ReferenceCheck>,
    localizer: &Localizer,
    entries: Vec<ReferenceEntry>,
) -> Result<ReferenceState, CommandError> {
    if slot.as_ref().is_some_and(|run| run.needs_restore()) {
        return Err(CommandError::localized(localizer.tr("reference-pending")));
    }
    let run = ReferenceCheck::new(localizer, entries)?;
    Ok(slot.insert(run).state())
}

/// Empty `slot`, unless its check still has a value to restore.
pub(crate) fn discard_reference(
    slot: &mut Option<ReferenceCheck>,
    localizer: &Localizer,
) -> Result<(), CommandError> {
    if slot.as_ref().is_some_and(|run| run.needs_restore()) {
        return Err(CommandError::localized(localizer.tr("reference-pending")));
    }
    slot.take();
    Ok(())
}

/// Build findings from a shared analysis report the user picks. Local only.
#[tauri::command]
pub(crate) async fn reference_import(
    app: AppHandle,
    path: String,
) -> Result<Vec<ReferenceEntry>, CommandError> {
    in_background(move || {
        let state = app.state::<AppState>();
        let localizer = state.localizer()?;
        let text = std::fs::read_to_string(&path)?;
        Ok(pmca_app::analysis::entries_from_report(&localizer, &text)?)
    })
    .await
}

/// The reference check under way, if any. Lets the view pick up where it left off.
#[tauri::command]
pub(crate) async fn reference_state(
    app: AppHandle,
) -> Result<Option<Reply<ReferenceState>>, CommandError> {
    in_background(move || {
        let state = app.state::<AppState>();
        #[cfg(target_os = "macos")]
        {
            let relayed = remote::call_if_running(&app, &state, remote::Request::ReferenceState)?;
            Ok(relayed.filter(|v| !v.is_null()).map(Reply::Relayed))
        }
        #[cfg(not(target_os = "macos"))]
        {
            let guard = state.reference.lock().map_err(CommandError::internal)?;
            Ok(guard.as_ref().map(|run| Reply::Here(run.state())))
        }
    })
    .await
}

/// Begin a reference check from the entered or imported findings. Touches no
/// camera.
#[tauri::command]
pub(crate) async fn reference_start(
    app: AppHandle,
    entries: Vec<ReferenceEntry>,
) -> Result<Reply<ReferenceState>, CommandError> {
    in_background(move || {
        let state = app.state::<AppState>();
        #[cfg(target_os = "macos")]
        {
            remote::call(&app, &state, remote::Request::ReferenceStart { entries })
                .map(Reply::Relayed)
        }
        #[cfg(not(target_os = "macos"))]
        {
            let localizer = state.localizer()?;
            let mut guard = state.reference.lock().map_err(CommandError::internal)?;
            Ok(Reply::Here(start_reference(&mut guard, &localizer, entries)?))
        }
    })
    .await
}

/// Read the findings' properties from the connected camera. Only reads.
#[tauri::command]
pub(crate) async fn reference_check(app: AppHandle) -> Result<Reply<ReferenceState>, CommandError> {
    with_camera(app, |app, state| {
        reference_camera_step(
            app,
            state,
            remote::Request::ReferenceCheck,
            Box::new(|run, l, sink| {
                run.check(l, sink)?;
                Ok(())
            }),
        )
    })
    .await
}

/// Write the chosen value of one finding. The front-end asks for consent twice
/// before calling this.
#[tauri::command]
pub(crate) async fn reference_write(
    app: AppHandle,
    index: usize,
    variant: usize,
) -> Result<Reply<ReferenceState>, CommandError> {
    with_camera(app, move |app, state| {
        reference_camera_step(
            app,
            state,
            remote::Request::ReferenceWrite { index, variant },
            Box::new(move |run, l, sink| {
                run.write(l, sink, index, variant)?;
                Ok(())
            }),
        )
    })
    .await
}

/// Note what the camera's menu showed. Touches no camera.
#[tauri::command]
pub(crate) async fn reference_menu(
    app: AppHandle,
    observed: String,
) -> Result<Reply<ReferenceState>, CommandError> {
    in_background(move || {
        let state = app.state::<AppState>();
        #[cfg(target_os = "macos")]
        {
            remote::call(&app, &state, remote::Request::ReferenceMenu { observed })
                .map(Reply::Relayed)
        }
        #[cfg(not(target_os = "macos"))]
        {
            with_reference(
                &app,
                &state,
                Box::new(move |run, _, _| {
                    run.record_menu(&observed);
                    Ok(())
                }),
            )
            .map(Reply::Here)
        }
    })
    .await
}

/// Write the original value back.
#[tauri::command]
pub(crate) async fn reference_restore(
    app: AppHandle,
) -> Result<Reply<ReferenceState>, CommandError> {
    with_camera(app, |app, state| {
        reference_camera_step(
            app,
            state,
            remote::Request::ReferenceRestore,
            Box::new(|run, l, sink| {
                run.restore(l, sink)?;
                Ok(())
            }),
        )
    })
    .await
}

/// Forget the reference check under way. Refused while a written value is still
/// waiting to be restored.
#[tauri::command]
pub(crate) async fn reference_discard(app: AppHandle) -> Result<(), CommandError> {
    in_background(move || {
        let state = app.state::<AppState>();
        #[cfg(target_os = "macos")]
        {
            remote::call_if_running(&app, &state, remote::Request::ReferenceDiscard)?;
            Ok(())
        }
        #[cfg(not(target_os = "macos"))]
        {
            let localizer = state.localizer()?;
            let mut guard = state.reference.lock().map_err(CommandError::internal)?;
            discard_reference(&mut guard, &localizer)
        }
    })
    .await
}
