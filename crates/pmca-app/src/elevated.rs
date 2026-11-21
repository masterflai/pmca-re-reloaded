//! The macOS broker's side of the conversation: the requests the window sends,
//! the sessions the broker holds between them, and how it carries them out.
//!
//! The broker is its own small executable (`pmca-broker`), started as root
//! through `sudo`; [`crate::broker`] carries the bytes. The window builds the
//! same [`Request`]s, and on the other platforms runs the same session helpers
//! below itself, so there is one implementation of each.

use std::path::PathBuf;

use pmca_i18n::Localizer;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::ProgressSink;
use crate::analysis::{AnalysisRun, ReferenceCheck, ReferenceEntry, ReferenceState, RunState};
use crate::broker::RemoteError;
use crate::dto::TweakChange;
use crate::service::{ServiceOutcome, ServiceSession};
use crate::tweaks::TweakSession;
use crate::updater::{UpdaterOutcome, UpdaterSession};

/// One request from the window.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "kebab-case")]
#[allow(missing_docs)]
pub enum Request {
    CameraInfo,
    TweaksRead,
    TweaksApply { changes: Vec<TweakChange> },
    TweaksApplyWholeImage { changes: Vec<TweakChange> },
    TweaksUndo,
    TweaksClose,
    ServiceOpen,
    ServiceRun { line: String },
    ServiceClose,
    ServiceSetTerminal { enable: bool },
    ServiceReadTerminal,
    ServiceWriteTerminal { data: Vec<u8> },
    AnalysisState,
    AnalysisStart { setting: String, values: Vec<String>, write_test: bool },
    AnalysisSnapshot,
    AnalysisWrite,
    AnalysisMenu { observed: String },
    AnalysisRestore,
    AnalysisCrossCheck,
    AnalysisReportJson,
    AnalysisDiscard,
    AnalysisRestorePending,
    ReferenceState,
    ReferenceStart { entries: Vec<ReferenceEntry> },
    ReferenceCheck,
    ReferenceWrite { index: usize, variant: usize },
    ReferenceMenu { observed: String },
    ReferenceRestore,
    ReferenceDiscard,
    BackupSave { path: PathBuf },
    BackupRestore { path: PathBuf },
    CompareDumpCamera { folder: PathBuf, include_sensitive: bool, keep_image: bool },
    FirmwareFlash { file: PathBuf, flash: bool, confirm: bool, reflash: bool },
    FirmwareRestoreVersion,
    FirmwareSetVersion { version: String },
    UpdaterOpen { fdat_file: Option<PathBuf>, fdat_dir: PathBuf, flash: bool },
    UpdaterRun { line: String },
    UpdaterClose,
    AppsBackup { fdat_dir: PathBuf, confirm: bool },
}

/// A request with the window's current language.
#[derive(Debug, Serialize, Deserialize)]
pub struct Envelope {
    /// The window's locale, for the broker's messages.
    pub locale: String,
    /// What to do.
    pub request: Request,
}

/// What the broker holds for the window between requests.
#[derive(Default)]
pub struct Sessions {
    tweaks: Option<TweakSession>,
    service: Option<ServiceSession>,
    analysis: Option<AnalysisRun>,
    reference: Option<ReferenceCheck>,
    updater: Option<UpdaterSession>,
}

impl RemoteError {
    /// A message that is already localized and is shown verbatim.
    pub fn localized(message: String) -> Self {
        Self { code: "operation".into(), message, detail: None, recovery: None }
    }

    /// A failure of the program itself, such as a poisoned lock.
    pub fn internal<E: std::fmt::Display>(e: E) -> Self {
        let message = e.to_string();
        Self {
            code: "internal".into(),
            message: message.clone(),
            detail: Some(message),
            recovery: None,
        }
    }
}

impl From<crate::Error> for RemoteError {
    fn from(e: crate::Error) -> Self {
        (&e).into()
    }
}

/// Carry out one request from the window.
pub fn dispatch(
    sessions: &mut Sessions,
    envelope: Value,
    sink: &mut dyn ProgressSink,
) -> Result<Value, RemoteError> {
    let envelope: Envelope =
        serde_json::from_value(envelope).map_err(|e| RemoteError::from(&crate::Error::Json(e)))?;
    let l = Localizer::new(&envelope.locale);
    run(sessions, &l, envelope.request, sink)
}

fn json(value: impl Serialize) -> Result<Value, RemoteError> {
    serde_json::to_value(value).map_err(RemoteError::internal)
}

fn run(
    s: &mut Sessions,
    l: &Localizer,
    request: Request,
    sink: &mut dyn ProgressSink,
) -> Result<Value, RemoteError> {
    use crate::tweaks::with_session;
    match request {
        Request::CameraInfo => json(crate::camera_info()?),
        Request::TweaksRead => json(with_session(&mut s.tweaks, l, sink, |t, k| t.read(l, k))?),
        Request::TweaksApply { changes } if changes.is_empty() => {
            json(with_session(&mut s.tweaks, l, sink, |t, k| t.read(l, k))?)
        }
        Request::TweaksApply { changes } => {
            json(with_session(&mut s.tweaks, l, sink, |t, k| t.apply(l, &changes, k))?)
        }
        Request::TweaksApplyWholeImage { changes } if changes.is_empty() => {
            json(with_session(&mut s.tweaks, l, sink, |t, k| t.read(l, k))?)
        }
        Request::TweaksApplyWholeImage { changes } => {
            json(with_session(&mut s.tweaks, l, sink, |t, k| t.apply_whole_image(l, &changes, k))?)
        }
        Request::TweaksUndo => json(with_session(&mut s.tweaks, l, sink, |t, k| t.undo(l, k))?),
        Request::TweaksClose => {
            s.tweaks.take();
            json(())
        }
        Request::ServiceOpen => {
            s.service.take();
            s.service = Some(ServiceSession::open(l, sink)?);
            json(())
        }
        Request::ServiceRun { line } => {
            let outputs = crate::service::parse(&line, l)
                .ok()
                .flatten()
                .map(|r| r.local_outputs())
                .unwrap_or_default();
            let outcome = run_service_line(&mut s.service, l, &line);
            // Running as root; what the command wrote belongs to the user.
            crate::ownership::return_paths(&outputs);
            json(outcome?)
        }
        Request::ServiceClose => {
            s.service.take();
            json(())
        }
        Request::ServiceSetTerminal { enable } => {
            let session = s.service.as_mut().ok_or_else(|| service_not_connected(l))?;
            if enable {
                session.enable_terminal()?;
            } else {
                session.disable_terminal()?;
            }
            json(())
        }
        Request::ServiceReadTerminal => {
            let session = s.service.as_mut().ok_or_else(|| service_not_connected(l))?;
            json(session.read_terminal())
        }
        Request::ServiceWriteTerminal { data } => {
            let session = s.service.as_mut().ok_or_else(|| service_not_connected(l))?;
            session.write_terminal(&data)?;
            json(())
        }
        Request::AnalysisState => json(s.analysis.as_ref().map(|run| run.state(l))),
        Request::AnalysisStart { setting, values, write_test } => {
            json(start_run(&mut s.analysis, l, &setting, &values, write_test)?)
        }
        Request::AnalysisSnapshot => json(analysis_step(
            &mut s.analysis,
            l,
            sink,
            Box::new(|run, l, sink| {
                run.take_snapshot(l, sink)?;
                Ok(())
            }),
        )?),
        Request::AnalysisWrite => json(analysis_step(
            &mut s.analysis,
            l,
            sink,
            Box::new(|run, l, sink| {
                run.write(l, sink)?;
                Ok(())
            }),
        )?),
        Request::AnalysisMenu { observed } => json(analysis_step(
            &mut s.analysis,
            l,
            sink,
            Box::new(move |run, _, _| {
                run.record_menu(&observed);
                Ok(())
            }),
        )?),
        Request::AnalysisRestore => json(analysis_step(
            &mut s.analysis,
            l,
            sink,
            Box::new(|run, l, sink| {
                run.restore(l, sink)?;
                Ok(())
            }),
        )?),
        Request::AnalysisCrossCheck => json(analysis_step(
            &mut s.analysis,
            l,
            sink,
            Box::new(|run, l, sink| {
                run.cross_check(l, sink)?;
                Ok(())
            }),
        )?),
        Request::AnalysisReportJson => {
            let run = s.analysis.as_ref().ok_or_else(|| RemoteError::internal("no analysis"))?;
            json(run.report_json()?)
        }
        Request::AnalysisDiscard => {
            discard_run(&mut s.analysis, l)?;
            json(())
        }
        Request::AnalysisRestorePending => json(crate::analysis::restore_pending(l, sink)?),
        Request::ReferenceState => json(s.reference.as_ref().map(|r| r.state())),
        Request::ReferenceStart { entries } => json(start_reference(&mut s.reference, l, entries)?),
        Request::ReferenceCheck => json(reference_step(
            &mut s.reference,
            l,
            sink,
            Box::new(|run, l, sink| {
                run.check(l, sink)?;
                Ok(())
            }),
        )?),
        Request::ReferenceWrite { index, variant } => json(reference_step(
            &mut s.reference,
            l,
            sink,
            Box::new(move |run, l, sink| {
                run.write(l, sink, index, variant)?;
                Ok(())
            }),
        )?),
        Request::ReferenceMenu { observed } => json(reference_step(
            &mut s.reference,
            l,
            sink,
            Box::new(move |run, _, _| {
                run.record_menu(&observed);
                Ok(())
            }),
        )?),
        Request::ReferenceRestore => json(reference_step(
            &mut s.reference,
            l,
            sink,
            Box::new(|run, l, sink| {
                run.restore(l, sink)?;
                Ok(())
            }),
        )?),
        Request::ReferenceDiscard => {
            discard_reference(&mut s.reference, l)?;
            json(())
        }
        Request::BackupSave { path } => json(crate::backup::save(l, sink, &path)?),
        Request::BackupRestore { path } => json(crate::backup::restore(l, sink, &path)?),
        Request::CompareDumpCamera { folder, include_sensitive, keep_image } => {
            let privacy = crate::compare::Privacy::including(include_sensitive);
            json(crate::compare::dump_camera(l, sink, &folder, privacy, keep_image)?)
        }
        Request::FirmwareFlash { file, flash, confirm, reflash } => {
            let request = crate::firmware::FlashRequest { file: &file, flash, confirm, reflash };
            json(crate::firmware::flash_firmware(l, &request, sink)?)
        }
        Request::FirmwareRestoreVersion => json(crate::firmware::restore_pending_version(l, sink)?),
        Request::FirmwareSetVersion { version } => {
            json(crate::firmware::set_firmware_version(l, &version, sink)?)
        }
        Request::UpdaterOpen { fdat_file, fdat_dir, flash } => {
            s.updater.take();
            let request = crate::updater::OpenRequest { model: None, fdat_file, fdat_dir, flash };
            s.updater = Some(UpdaterSession::open(l, &request, sink)?);
            json(())
        }
        Request::UpdaterRun { line } => {
            let outputs = crate::updater::parse(&line, l)
                .ok()
                .flatten()
                .map(|r| r.local_outputs())
                .unwrap_or_default();
            let outcome = run_updater_line(&mut s.updater, l, &line);
            crate::ownership::return_paths(&outputs);
            json(outcome?)
        }
        Request::UpdaterClose => {
            s.updater.take();
            json(())
        }
        Request::AppsBackup { fdat_dir, confirm } => {
            let request = crate::app_backup::BackupRequest { fdat_dir, out_dir: None, confirm };
            let outcome = crate::app_backup::backup_apps(l, &request, sink);
            // Running as root; the backups belong to the user, wherever they went.
            crate::ownership::return_user_data();
            if let Ok(outcome) = &outcome {
                let folder = PathBuf::from(&outcome.folder);
                let mut paths = vec![folder.clone()];
                for app in &outcome.apps {
                    paths.push(folder.join(&app.file));
                    paths.push(folder.join(format!("{}.sha256", app.file)));
                }
                crate::ownership::return_paths(&paths);
            }
            json(outcome?)
        }
    }
}

/// Run as the broker: connect to the window's socket and answer its requests
/// until it closes the connection. Returns the exit code.
pub fn serve_window(socket: Option<PathBuf>) -> i32 {
    #[cfg(unix)]
    {
        let Some(socket) = socket else { return 2 };
        let _ownership = crate::ownership::OwnershipGuard::new();
        crate::keys::load_from_disk();
        // Relative paths in the service shell land in the user's home, not in `/`.
        if let Some(home) = std::env::var_os("HOME") {
            let _ = std::env::set_current_dir(home);
        }
        let Ok(stream) = crate::broker::connect_to_window(&socket) else { return 2 };
        let Ok(reader) = stream.try_clone() else { return 2 };
        let mut sessions = Sessions::default();
        crate::broker::serve(std::io::BufReader::new(reader), stream, |request, sink| {
            dispatch(&mut sessions, request, sink)
        });
        0
    }
    #[cfg(not(unix))]
    {
        let _ = socket;
        2
    }
}

// ---------------------------------------------------------------------------
// Session helpers, run by the broker and, off macOS, by the window itself.
// ---------------------------------------------------------------------------

/// A step of the analysis.
pub type Step<'a> = Box<
    dyn FnOnce(&mut AnalysisRun, &Localizer, &mut dyn ProgressSink) -> Result<(), RemoteError> + 'a,
>;

/// Run `step` on the analysis in `slot`, then save its report once there is
/// something to report, and return where the run stands.
pub fn analysis_step(
    slot: &mut Option<AnalysisRun>,
    localizer: &Localizer,
    sink: &mut dyn ProgressSink,
    step: Step<'_>,
) -> Result<RunState, RemoteError> {
    let run = slot
        .as_mut()
        .ok_or_else(|| RemoteError::localized(localizer.tr("analysis-write-not-allowed")))?;
    let result = step(run, localizer, sink);
    // Saved after failures too: a report that stops half-way still says where.
    if run.taken() > 0 {
        let _ = run.save_report();
    }
    if let Some((model, id, setting)) = run.finding() {
        let day = run.report().started.get(..10).unwrap_or_default().to_string();
        let _ = crate::own_names::remember_finding(&model, id, &setting, &day);
    }
    result?;
    Ok(run.state(localizer))
}

/// Start a run in `slot`, unless the one there still has a value to restore.
pub fn start_run(
    slot: &mut Option<AnalysisRun>,
    localizer: &Localizer,
    setting: &str,
    values: &[String],
    write_test: bool,
) -> Result<RunState, RemoteError> {
    if slot.as_ref().is_some_and(|run| run.needs_restore()) {
        return Err(RemoteError::localized(localizer.tr("analysis-pending")));
    }
    let run = AnalysisRun::new(localizer, setting, values, write_test)?;
    Ok(slot.insert(run).state(localizer))
}

/// Empty `slot`, unless its run still has a value to restore.
pub fn discard_run(
    slot: &mut Option<AnalysisRun>,
    localizer: &Localizer,
) -> Result<(), RemoteError> {
    if slot.as_ref().is_some_and(|run| run.needs_restore()) {
        return Err(RemoteError::localized(localizer.tr("analysis-pending")));
    }
    slot.take();
    Ok(())
}

/// A step of the reference check.
pub type ReferenceStep<'a> = Box<
    dyn FnOnce(&mut ReferenceCheck, &Localizer, &mut dyn ProgressSink) -> Result<(), RemoteError>
        + 'a,
>;

/// Run `step` on the reference check in `slot` and return where it stands.
pub fn reference_step(
    slot: &mut Option<ReferenceCheck>,
    localizer: &Localizer,
    sink: &mut dyn ProgressSink,
    step: ReferenceStep<'_>,
) -> Result<ReferenceState, RemoteError> {
    let run = slot
        .as_mut()
        .ok_or_else(|| RemoteError::localized(localizer.tr("reference-not-started")))?;
    step(run, localizer, sink)?;
    Ok(run.state())
}

/// Start a check in `slot`, unless the one there still has a value to restore.
pub fn start_reference(
    slot: &mut Option<ReferenceCheck>,
    localizer: &Localizer,
    entries: Vec<ReferenceEntry>,
) -> Result<ReferenceState, RemoteError> {
    if slot.as_ref().is_some_and(|run| run.needs_restore()) {
        return Err(RemoteError::localized(localizer.tr("reference-pending")));
    }
    let run = ReferenceCheck::new(localizer, entries)?;
    Ok(slot.insert(run).state())
}

/// Empty `slot`, unless its check still has a value to restore.
pub fn discard_reference(
    slot: &mut Option<ReferenceCheck>,
    localizer: &Localizer,
) -> Result<(), RemoteError> {
    if slot.as_ref().is_some_and(|run| run.needs_restore()) {
        return Err(RemoteError::localized(localizer.tr("reference-pending")));
    }
    slot.take();
    Ok(())
}

/// Run `line` on the service session in `slot`, ending it on `exit`.
pub fn run_service_line(
    slot: &mut Option<ServiceSession>,
    localizer: &Localizer,
    line: &str,
) -> Result<Option<ServiceOutcome>, RemoteError> {
    let session = slot.as_mut().ok_or_else(|| service_not_connected(localizer))?;
    let outcome = session.run_line(localizer, line)?;
    // `exit` is a command like any other, and the session it ends is this one.
    if outcome.as_ref().is_some_and(|o| o.finished) {
        slot.take();
    }
    Ok(outcome)
}

/// The error for a service command sent with no session open.
pub fn service_not_connected(localizer: &Localizer) -> RemoteError {
    RemoteError {
        code: "no-service-session".into(),
        message: localizer.tr("service-not-connected"),
        detail: None,
        recovery: None,
    }
}

/// Run `line` on the updater session in `slot`, ending it on `exit`.
pub fn run_updater_line(
    slot: &mut Option<UpdaterSession>,
    localizer: &Localizer,
    line: &str,
) -> Result<Option<UpdaterOutcome>, RemoteError> {
    let session = slot.as_mut().ok_or_else(|| updater_not_connected(localizer))?;
    let outcome = session.run_line(localizer, line)?;
    // `exit` is a command like any other, and the session it ends is this one.
    if outcome.as_ref().is_some_and(|o| o.finished) {
        slot.take();
    }
    Ok(outcome)
}

/// The error for an updater command sent with no session open.
pub fn updater_not_connected(localizer: &Localizer) -> RemoteError {
    RemoteError {
        code: "no-updater-session".into(),
        message: localizer.tr("service-not-connected"),
        detail: None,
        recovery: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_request_survives_the_wire() {
        let request = Request::TweaksApply {
            changes: vec![TweakChange {
                id: "recLimit".into(),
                enabled: true,
                accept_caution: false,
            }],
        };
        let envelope = json(Envelope { locale: "de-DE".into(), request }).unwrap();
        let back: Envelope = serde_json::from_value(envelope).unwrap();
        assert_eq!(back.locale, "de-DE");
        assert!(matches!(back.request, Request::TweaksApply { changes } if changes.len() == 1));
    }

    #[test]
    fn requests_without_a_session_fail_politely() {
        let mut sessions = Sessions::default();
        let mut sink = |_: crate::dto::ProgressEvent| {};
        let ask = |s: &mut Sessions, sink: &mut dyn ProgressSink, request| {
            dispatch(s, json(Envelope { locale: "en-US".into(), request }).unwrap(), sink)
        };
        assert_eq!(ask(&mut sessions, &mut sink, Request::AnalysisState).unwrap(), Value::Null);
        assert_eq!(ask(&mut sessions, &mut sink, Request::ServiceClose).unwrap(), Value::Null);
        assert_eq!(ask(&mut sessions, &mut sink, Request::AnalysisDiscard).unwrap(), Value::Null);
        assert_eq!(ask(&mut sessions, &mut sink, Request::UpdaterClose).unwrap(), Value::Null);
        let e =
            ask(&mut sessions, &mut sink, Request::UpdaterRun { line: "info".into() }).unwrap_err();
        assert_eq!(e.code, "no-updater-session");
        let e = ask(&mut sessions, &mut sink, Request::ServiceReadTerminal).unwrap_err();
        assert_eq!(e.code, "no-service-session");
        let e = ask(&mut sessions, &mut sink, Request::AnalysisSnapshot).unwrap_err();
        assert_eq!(e.code, "operation");
    }

    #[test]
    fn an_analysis_lives_in_the_broker_between_requests() {
        let mut sessions = Sessions::default();
        let mut sink = |_: crate::dto::ProgressEvent| {};
        let mut ask = |request| {
            dispatch(
                &mut sessions,
                json(Envelope { locale: "en-US".into(), request }).unwrap(),
                &mut sink,
            )
        };
        let started = ask(Request::AnalysisStart {
            setting: "Quality".into(),
            values: vec!["Fine".into(), "Extra fine".into()],
            write_test: false,
        })
        .unwrap();
        assert_eq!(started["setting"], "Quality");
        assert_eq!(ask(Request::AnalysisState).unwrap()["setting"], "Quality");
        ask(Request::AnalysisDiscard).unwrap();
        assert_eq!(ask(Request::AnalysisState).unwrap(), Value::Null);
    }
}
