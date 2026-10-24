//! The macOS broker: the requests the window sends it, how it carries them
//! out, and the window's end of the call.
//!
//! On macOS the service-mode sessions, the tweaks, the function analysis,
//! everything that reboots the camera into updater mode and reading a camera
//! in mass-storage mode live in an elevated copy of this program; [`pmca_app::broker`] explains why
//! and carries the bytes. The requests mirror the window's commands, and the
//! broker answers them with the same functions the commands use elsewhere, so
//! there is one implementation of each.

// Only macOS sends requests; elsewhere only the tests and the broker entry use this.
#![cfg_attr(not(target_os = "macos"), allow(dead_code))]

use pmca_app::ProgressSink;
use pmca_app::analysis::AnalysisRun;
use pmca_app::broker::RemoteError;
use pmca_app::dto::TweakChange;
use pmca_app::service::ServiceSession;
use pmca_app::tweaks::TweakSession;
use pmca_app::updater::UpdaterSession;
use pmca_i18n::Localizer;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::PathBuf;

use crate::*;

/// One request from the window.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "kebab-case")]
pub(crate) enum Request {
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
    FirmwareFlash { file: PathBuf, flash: bool, confirm: bool },
    UpdaterOpen { fdat_file: Option<PathBuf>, fdat_dir: PathBuf, flash: bool },
    UpdaterRun { line: String },
    UpdaterClose,
}

/// A request with the window's current language.
#[derive(Debug, Serialize, Deserialize)]
struct Envelope {
    locale: String,
    request: Request,
}

/// What the broker holds for the window.
#[derive(Default)]
pub(crate) struct Sessions {
    tweaks: Option<TweakSession>,
    service: Option<ServiceSession>,
    analysis: Option<AnalysisRun>,
    updater: Option<UpdaterSession>,
}

impl From<CommandError> for RemoteError {
    fn from(e: CommandError) -> Self {
        Self { code: e.code, message: e.message, detail: e.detail, recovery: e.recovery }
    }
}

/// Carry out one request from the window.
pub(crate) fn dispatch(
    sessions: &mut Sessions,
    envelope: Value,
    sink: &mut dyn ProgressSink,
) -> Result<Value, RemoteError> {
    let envelope: Envelope = serde_json::from_value(envelope)
        .map_err(|e| RemoteError::from(&pmca_app::Error::Json(e)))?;
    let l = Localizer::new(&envelope.locale);
    run(sessions, &l, envelope.request, sink).map_err(Into::into)
}

fn json(value: impl Serialize) -> Result<Value, CommandError> {
    serde_json::to_value(value).map_err(CommandError::internal)
}

fn run(
    s: &mut Sessions,
    l: &Localizer,
    request: Request,
    sink: &mut dyn ProgressSink,
) -> Result<Value, CommandError> {
    use pmca_app::tweaks::with_session;
    match request {
        Request::CameraInfo => json(pmca_app::camera_info()?),
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
            let outputs = pmca_app::service::parse(&line, l)
                .ok()
                .flatten()
                .map(|r| r.local_outputs())
                .unwrap_or_default();
            let outcome = run_service_line(&mut s.service, l, &line);
            // Running as root; what the command wrote belongs to the user.
            pmca_app::ownership::return_paths(&outputs);
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
            let run = s.analysis.as_ref().ok_or_else(|| CommandError::internal("no analysis"))?;
            json(run.report_json()?)
        }
        Request::AnalysisDiscard => {
            discard_run(&mut s.analysis, l)?;
            json(())
        }
        Request::AnalysisRestorePending => json(pmca_app::analysis::restore_pending(l, sink)?),
        Request::FirmwareFlash { file, flash, confirm } => {
            let request = pmca_app::firmware::FlashRequest { file: &file, flash, confirm };
            json(pmca_app::firmware::flash_firmware(l, &request, sink)?)
        }
        Request::UpdaterOpen { fdat_file, fdat_dir, flash } => {
            s.updater.take();
            let request =
                pmca_app::updater::OpenRequest { model: None, fdat_file, fdat_dir, flash };
            s.updater = Some(UpdaterSession::open(l, &request, sink)?);
            json(())
        }
        Request::UpdaterRun { line } => {
            let outputs = pmca_app::updater::parse(&line, l)
                .ok()
                .flatten()
                .map(|r| r.local_outputs())
                .unwrap_or_default();
            let outcome = run_updater_line(&mut s.updater, l, &line);
            pmca_app::ownership::return_paths(&outputs);
            json(outcome?)
        }
        Request::UpdaterClose => {
            s.updater.take();
            json(())
        }
    }
}

/// Run as the broker: connect to the window's socket and answer its requests
/// until it closes the connection. Returns the exit code.
pub fn run_broker(socket: Option<std::path::PathBuf>) -> i32 {
    #[cfg(unix)]
    {
        let Some(socket) = socket else { return 2 };
        let _ownership = pmca_app::ownership::OwnershipGuard::new();
        pmca_app::keys::load_from_disk();
        // Relative paths in the service shell land in the user's home, not in `/`.
        if let Some(home) = std::env::var_os("HOME") {
            let _ = std::env::set_current_dir(home);
        }
        let Ok(stream) = pmca_app::broker::connect_to_window(&socket) else { return 2 };
        let Ok(reader) = stream.try_clone() else { return 2 };
        let mut sessions = Sessions::default();
        pmca_app::broker::serve(std::io::BufReader::new(reader), stream, |request, sink| {
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

/// Send `request` to the broker, starting it first if need be, and relay its
/// progress to the window.
#[cfg(target_os = "macos")]
pub(crate) fn call(
    app: &AppHandle,
    state: &AppState,
    request: Request,
) -> Result<Value, CommandError> {
    let mut guard = state.broker.lock().map_err(CommandError::internal)?;
    if guard.is_none() {
        *guard = Some(pmca_app::broker::Broker::start(&state.localizer()?)?);
    }
    relay(app, state, &mut guard, request)
}

/// [`call`] when the broker is already running; `None` otherwise, for the
/// requests that are not worth a password prompt.
#[cfg(target_os = "macos")]
pub(crate) fn call_if_running(
    app: &AppHandle,
    state: &AppState,
    request: Request,
) -> Result<Option<Value>, CommandError> {
    let mut guard = state.broker.lock().map_err(CommandError::internal)?;
    if guard.is_none() {
        return Ok(None);
    }
    relay(app, state, &mut guard, request).map(Some)
}

#[cfg(target_os = "macos")]
fn relay(
    app: &AppHandle,
    state: &AppState,
    broker: &mut Option<pmca_app::broker::Broker>,
    request: Request,
) -> Result<Value, CommandError> {
    let locale = state.locale.lock().map_err(CommandError::internal)?.clone();
    let envelope = json(Envelope { locale, request })?;
    let running = broker.as_mut().ok_or_else(|| CommandError::internal("no broker"))?;
    match running.call(&envelope, &mut progress(app)) {
        Ok(value) => Ok(value),
        Err(pmca_app::broker::CallError::Remote(e)) => Err(e.into()),
        Err(pmca_app::broker::CallError::Broken(e)) => {
            // Its sessions went with it; the next request starts a fresh one.
            broker.take();
            Err(e.into())
        }
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
        let mut sink = |_: pmca_app::dto::ProgressEvent| {};
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
        let mut sink = |_: pmca_app::dto::ProgressEvent| {};
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
