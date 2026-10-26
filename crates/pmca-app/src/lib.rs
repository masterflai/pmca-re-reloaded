//! # pmca-app
//!
//! Application-level orchestration shared by the CLI (`pmca-console`) and the
//! GUI (Tauri) front-ends.
//!
//! The lower crates are deliberately narrow: [`pmca_usb`] speaks to the camera,
//! [`pmca_market`] serves the install protocol, [`pmca_core`] parses files.
//! Gluing them into a *user-visible operation* ("install this app", "tell me
//! about the attached camera") used to live inside the CLI binary, which made
//! it unavailable to the GUI. That glue lives here instead, so both front-ends
//! drive identical logic and only differ in how they render progress.
//!
//! Front-ends supply a [`ProgressSink`]; the CLI prints, the GUI forwards the
//! events to the web view. All results are plain serializable [`dto`] types.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod about;
pub mod analysis;
pub mod app_backup;
pub mod broker;
pub mod devices;
mod direct;
pub mod drivers;
pub mod dto;
pub mod fdat;
pub mod firmware;
pub mod keys;
pub mod network;
pub mod ownership;
pub mod plugins;
pub mod proxy;
pub mod service;
pub mod service_mode;
pub mod sources;
pub mod store;
pub mod tools;
pub mod tweaks;
pub mod update;
pub mod updater;

use std::path::Path;
use std::time::Duration;

use pmca_i18n::Localizer;
use pmca_usb::discovery::{self, CameraMode};

pub use dto::{
    ApkSummary, AppBackupOutcome, BackedUpApp, CameraSummary, CommunityApp, DeviceProperty,
    FirmwareOutcome, InstallOutcome, InstalledDevice, PhotoSummary, ProgressEvent, ProgressKind,
    TweakChange, TweakChoice, TweakReport, TweakState,
};

/// The XPD content type the camera expects for the install task payload.
const XPD_MIME: &str = "application/x-psn-dstartup2";

/// How many times to poll for a camera after asking it to change mode.
///
/// Windows has to enumerate the device under its new product id and bind a
/// driver to it before it becomes visible at all, which takes considerably
/// longer than the camera's own switch.
const MODE_SWITCH_ATTEMPTS: usize = 60;

/// Delay between mode-switch polls.
const MODE_SWITCH_DELAY: Duration = Duration::from_millis(500);

/// A reconnect-wait message followed by a note about prompts on the camera.
///
/// Every reconnect as a mass-storage device makes some cameras show a prompt
/// about Sony's PC software. It blocks nothing, but a user who sees it during a
/// wait would otherwise assume the camera is waiting for them.
pub(crate) fn wait_message(l: &Localizer, key: &str) -> String {
    format!("{} {}", l.tr(key), l.tr("camera-confirm-hint"))
}

/// Tells the user to pick mass storage on a camera that shows its USB mode choice.
pub(crate) fn choice_hint<'a>(
    l: &'a Localizer,
    sink: &'a mut dyn ProgressSink,
) -> impl FnMut() + 'a {
    move || sink.emit(ProgressEvent::step(ProgressKind::Waiting, l.tr("camera-choose-usb-mode")))
}

/// A wait that ran out: a camera still at its USB mode choice says so, anything
/// else is reported as the timeout `key`.
pub(crate) fn timeout_or(l: &Localizer, e: pmca_usb::Error, key: &str) -> Error {
    match e {
        pmca_usb::Error::CameraNotReady => Error::Usb(e),
        _ => Error::Operation(l.tr(key)),
    }
}

/// Errors surfaced to a front-end.
///
/// Transport failures keep their original detail; [`Error::Operation`] carries
/// an already-localized message that is safe to show verbatim.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// A USB transport or protocol failure.
    #[error(transparent)]
    Usb(#[from] pmca_usb::Error),
    /// A failure of the local install server or a community-store download.
    #[error(transparent)]
    Market(#[from] pmca_market::Error),
    /// An APK could not be inspected.
    #[error(transparent)]
    Apk(#[from] pmca_apk::Error),
    /// A filesystem failure.
    #[error(transparent)]
    Io(#[from] std::io::Error),
    /// Malformed JSON received from the camera.
    #[error("invalid JSON from camera: {0}")]
    Json(#[from] serde_json::Error),
    /// An operation failed with an already-localized explanation.
    #[error("{0}")]
    Operation(String),
    /// Assigning a driver was refused because the process is not elevated.
    ///
    /// Kept apart from [`Error::Operation`] because it is the one failure a
    /// front-end can resolve on its own: restarting elevated makes the very
    /// same attempt succeed, and there is nothing for the user to do to the
    /// camera in between.
    #[error("{0}")]
    NeedsElevation(String),
    /// The camera is in a mode the requested operation cannot use.
    ///
    /// Kept apart from [`Error::Operation`] so a front-end can offer the mode
    /// change instead of only printing the reason.
    #[error("{message}")]
    WrongMode {
        /// Already-localized explanation, safe to show verbatim.
        message: String,
        /// The mode the camera was actually found in.
        mode: CameraMode,
    },
    /// The camera refused a single-value write and every value is as before.
    ///
    /// Kept apart from [`Error::Operation`] so a front-end can offer writing
    /// the whole backup image instead, which some models require.
    #[error("{0}")]
    DirectRefused(String),
    /// A Sony-derived key the user has not supplied is needed for this
    /// operation.
    ///
    /// Kept apart from [`Error::Operation`] so a front-end can send the user to
    /// the key import instead of only printing the reason.
    #[error("{0}")]
    MissingKeys(String),
}

/// Recovery action identifier for a camera stuck in MTP mode.
///
/// Front-ends match on this to decide whether to offer the mode change; see
/// [`Error::recovery`].
pub const RECOVERY_SWITCH_TO_MASS_STORAGE: &str = "switch-to-mass-storage";

/// Recovery action identifier for an operation that needs administrator rights.
///
/// Front-ends match on this to offer starting the elevated driver helper
/// again, after the user dismissed its rights prompt; see [`Error::recovery`].
pub const RECOVERY_START_DRIVER_HELPER: &str = "start-driver-helper";

/// Recovery action identifier for a single-value write the camera refused,
/// which the whole backup image can still carry; see [`Error::recovery`].
pub const RECOVERY_WHOLE_IMAGE: &str = "whole-image";

/// Recovery action identifier for an operation that needs key material the user
/// has not supplied yet; see [`Error::recovery`].
pub const RECOVERY_IMPORT_KEYS: &str = "import-keys";

impl Error {
    /// A stable identifier for this kind of failure.
    ///
    /// Front-ends map the code onto a translated message instead of matching on
    /// English text. [`Error::Operation`] and [`Error::WrongMode`] are reported
    /// as `operation` and `wrong-mode`, for which no translation must exist:
    /// their message has already been localized by the layer that produced it
    /// and is meant to be shown verbatim.
    pub fn code(&self) -> &'static str {
        match self {
            Error::Usb(e) => e.code(),
            Error::Market(e) => e.code(),
            Error::Apk(_) => "apk",
            Error::Io(_) => "io",
            Error::Json(_) => "json",
            Error::Operation(_) | Error::DirectRefused(_) => "operation",
            Error::NeedsElevation(_) => "needs-elevation",
            Error::WrongMode { .. } => "wrong-mode",
            Error::MissingKeys(_) => "missing-keys",
        }
    }

    /// The variable part of the message, if this variant carries one.
    pub fn detail(&self) -> Option<String> {
        match self {
            Error::Usb(e) => e.detail(),
            Error::Market(e) => e.detail(),
            Error::Apk(e) => Some(e.to_string()),
            Error::Io(e) => Some(e.to_string()),
            Error::Json(e) => Some(e.to_string()),
            Error::Operation(_) | Error::DirectRefused(_) => None,
            Error::NeedsElevation(_) => None,
            Error::WrongMode { mode, .. } => Some(mode.id().to_string()),
            Error::MissingKeys(_) => None,
        }
    }

    /// An action a front-end may offer to make the failed operation possible.
    ///
    /// An MTP camera can be told over USB to reconnect as a mass-storage
    /// device, a dismissed rights prompt can be answered by starting the
    /// driver helper again, a refused single value can go through the whole
    /// image, and a missing key can be imported. Every other failure needs
    /// either a power cycle or a decision only the user can make.
    pub fn recovery(&self) -> Option<&'static str> {
        match self {
            Error::WrongMode { mode: CameraMode::Mtp, .. } => Some(RECOVERY_SWITCH_TO_MASS_STORAGE),
            Error::NeedsElevation(_) => Some(RECOVERY_START_DRIVER_HELPER),
            Error::DirectRefused(_) => Some(RECOVERY_WHOLE_IMAGE),
            Error::MissingKeys(_) => Some(RECOVERY_IMPORT_KEYS),
            Error::Usb(pmca_usb::Error::MissingKeys) => Some(RECOVERY_IMPORT_KEYS),
            _ => None,
        }
    }
}

/// Convenience alias.
pub type Result<T> = std::result::Result<T, Error>;

/// Receives progress notifications from a long-running operation.
///
/// Implementations must not block for long: the same thread also drives the USB
/// transfers, and a stalled sink stalls the camera session.
pub trait ProgressSink {
    /// Handle one notification.
    fn emit(&mut self, event: ProgressEvent);
}

/// A sink that discards every event.
pub struct NullSink;

impl ProgressSink for NullSink {
    fn emit(&mut self, _event: ProgressEvent) {}
}

impl<F: FnMut(ProgressEvent)> ProgressSink for F {
    fn emit(&mut self, event: ProgressEvent) {
        self(event)
    }
}

/// Identify the single attached camera.
pub fn camera_info() -> Result<CameraSummary> {
    let camera = discovery::find_single_camera()?;
    Ok(discovery::read_camera_info(&camera)?.into())
}

/// The running Windows services known to interfere with camera access.
///
/// Always empty on other platforms. Needs no camera and changes nothing.
pub fn interfering_services() -> Vec<dto::InterferingService> {
    pmca_usb::host_services::interfering_services().into_iter().map(Into::into).collect()
}

/// Read the camera's usage log ("KikiLog") as raw bytes.
///
/// The format is undocumented, so the blob is passed through untouched rather
/// than being parsed into a shape that cannot be verified. The camera is only
/// read from, never modified.
pub fn usage_log() -> Result<Vec<u8>> {
    let camera = discovery::find_single_camera()?;
    let transport = discovery::open_extcmd_transport(&camera)?;
    Ok(pmca_usb::sony_extcmd::SonyExtCmdCamera::new(transport).get_usage_log()?)
}

/// Where the APK to install comes from.
#[derive(Debug, Clone, Copy)]
pub enum ApkSource<'a> {
    /// A file on disk.
    File(&'a Path),
    /// An already-downloaded archive (e.g. from the community store).
    Bytes(&'a [u8]),
}

/// Options for [`install_app`].
#[derive(Debug, Clone)]
pub struct InstallRequest<'a> {
    /// APK to install. `None` serves the camera an empty task, which is useful
    /// for probing the install protocol without changing anything.
    pub apk: Option<ApkSource<'a>>,
    /// PEM file holding the certificate + key the local HTTPS server presents.
    /// `None` — the normal case — has the server generate both at start-up.
    pub cert: Option<&'a Path>,
}

/// Install an application on the attached camera.
///
/// Mirrors the reference `installCommand`: switch the camera to
/// application-install mode, start a local HTTPS market server, hand the camera
/// the XPD startup file, then relay its TLS traffic to that server until the
/// installation task reports completion. The camera stays in app-install mode
/// afterwards; see `stays_in_app_install_mode`.
pub fn install_app(
    l: &Localizer,
    request: &InstallRequest<'_>,
    sink: &mut dyn ProgressSink,
) -> Result<InstallOutcome> {
    let outcome = run_install_task(l, request, sink)?;
    stays_in_app_install_mode(l, sink);
    sink.emit(ProgressEvent::step(ProgressKind::Done, l.tr("install-done")));
    Ok(outcome)
}

/// Several installation tasks in a row, telling the user about app-install mode
/// only once.
///
/// The next task opens a camera that is still in app-install mode directly, so
/// the reminder after every app would only repeat itself. A series gives it
/// once, in [`InstallSeries::finish`].
#[derive(Debug, Default)]
pub struct InstallSeries {
    /// Whether a task has completed, which is what left the camera in the mode.
    completed: bool,
}

impl InstallSeries {
    /// Run one task of the series; the camera is left in app-install mode.
    pub fn install(
        &mut self,
        l: &Localizer,
        request: &InstallRequest<'_>,
        sink: &mut dyn ProgressSink,
    ) -> Result<InstallOutcome> {
        let outcome = run_install_task(l, request, sink)?;
        self.completed = true;
        sink.emit(ProgressEvent::step(ProgressKind::Done, l.tr("install-done")));
        Ok(outcome)
    }

    /// End the series. Does nothing if no task has completed.
    pub fn finish(self, l: &Localizer, sink: &mut dyn ProgressSink) {
        if self.completed {
            stays_in_app_install_mode(l, sink);
        }
    }
}

/// Tell the user that the camera remains in app-install mode.
///
/// The mode offers no command that ends it (measured: its command set has none)
/// and no tested camera leaves it by itself: one was still there after
/// minutes. Waiting for it only delayed every installation, so the user is told
/// how to get out instead, as a notice rather than an error — further
/// installations work in this mode.
fn stays_in_app_install_mode(l: &Localizer, sink: &mut dyn ProgressSink) {
    sink.emit(ProgressEvent::step(ProgressKind::Notice, l.tr("install-stays-app-mode")));
}

/// Run one installation task and return its outcome.
fn run_install_task(
    l: &Localizer,
    request: &InstallRequest<'_>,
    sink: &mut dyn ProgressSink,
) -> Result<InstallOutcome> {
    use pmca_market::server::TlsMarketServer;

    // 1. Obtain an app-install proxy connection, switching modes if needed.
    let camera = discovery::find_single_camera()?;
    let mut app = match camera.mode {
        // An MTP device may already be an app-install device; otherwise it is a
        // normal command device that must be switched.
        CameraMode::Mtp => match discovery::open_app_install(&camera) {
            Ok(app) => app,
            Err(_) => switch_to_app_install(l, &camera, sink)?,
        },
        CameraMode::AppInstall => discovery::open_app_install(&camera)?,
        CameraMode::MassStorage => switch_to_app_install(l, &camera, sink)?,
        mode => return Err(Error::WrongMode { message: l.tr("install-bad-mode"), mode }),
    };

    // 2. Start the local HTTPS market server the camera will talk to.
    let mut server =
        TlsMarketServer::bind("127.0.0.1:0".parse().expect("valid literal"), request.cert)?;
    let apk_summary = match request.apk {
        Some(source) => {
            let data = match source {
                ApkSource::File(path) => std::fs::read(path)?,
                ApkSource::Bytes(data) => data.to_vec(),
            };
            // Inspecting the archive is advisory: an APK the parser cannot read
            // may still install, so a failure only suppresses the report.
            let summary = pmca_apk::read(&data).ok().map(ApkSummary::from);
            if let Some(summary) = &summary {
                report_apk(l, summary, sink);
            } else {
                sink.emit(ProgressEvent::step(ProgressKind::Info, l.tr("apk-unreadable")));
            }
            server.set_apk(data);
            summary
        }
        None => None,
    };
    server.start()?;
    let addr = server.local_addr();
    let xpd = server.xpd()?;

    // 3. Run the installation, relaying the camera's TLS traffic to the server.
    sink.emit(ProgressEvent::step(ProgressKind::Info, l.tr("install-starting")));
    let mut observer = SinkObserver { l, sink };
    let complete = pmca_usb::app_install::install(
        &mut app,
        &mut proxy::TcpConnector,
        &addr.ip().to_string(),
        addr.port(),
        &xpd,
        XPD_MIME,
        &mut observer,
    );
    // The server owns a background thread and a bound port; stop it even when
    // the relay failed, so a retry can bind again.
    server.stop();
    // When the relay broke, the server thread usually knows why: the camera is
    // on the far side of a tunnel, so a connection the server rejected reaches
    // the relay only as a dropped socket, which it reports as a USB fault. That
    // sends people looking for a bad cable, so prefer the real cause.
    let complete = match (complete, server.last_error()) {
        (Err(_), Some(cause)) => {
            return Err(Error::Operation(format!("{} {cause}", l.tr("install-server-failed"))));
        }
        (result, _) => result?,
    };

    // 4. Interpret the camera's completion result.
    let complete: serde_json::Value = serde_json::from_slice(&complete)?;
    check_result_code(l, &complete)?;

    // 5. Report the device info the server captured.
    let outcome = match server.result() {
        Some(result) => InstallOutcome {
            device: Some(InstalledDevice {
                name: result.device.name.clone(),
                product_code: result.device.product_code.clone(),
                device_id: result.device.device_id.clone(),
                firmware_version: result.device.firmware_version.clone(),
            }),
            apk: apk_summary,
            raw: Some(result.raw.clone()),
        },
        None => InstallOutcome { apk: apk_summary, ..InstallOutcome::default() },
    };
    Ok(outcome)
}

/// Emit the APK's identity and, where applicable, a compatibility warning.
fn report_apk(l: &Localizer, summary: &ApkSummary, sink: &mut dyn ProgressSink) {
    sink.emit(ProgressEvent::step(
        ProgressKind::Info,
        format!(
            "{} {} {}",
            l.tr("apk-package"),
            summary.package,
            summary.version.as_deref().unwrap_or_default()
        ),
    ));
    if !summary.compatible {
        sink.emit(ProgressEvent::step(ProgressKind::Info, l.tr("apk-incompatible")));
    }
    if !summary.signed {
        sink.emit(ProgressEvent::step(ProgressKind::Info, l.tr("apk-unsigned")));
    }
}

/// Reject a camera payload whose `resultCode` is not `0`.
fn check_result_code(l: &Localizer, value: &serde_json::Value) -> Result<()> {
    let code = value.get("resultCode").and_then(|v| v.as_i64()).unwrap_or(-1);
    if code == 0 {
        return Ok(());
    }
    let message = value.get("message").and_then(|v| v.as_str()).unwrap_or("");
    Err(Error::Operation(format!("{} {}: {}", l.tr("install-error"), code, message)))
}

/// Ask the connected camera to reconnect as a mass-storage device.
///
/// This is the only camera mode change that can be triggered over USB and it
/// only works from MTP mode. It is also one-way: no command switches a
/// mass-storage camera back, but the USB mode selected in the camera's own menu
/// is untouched, so a power cycle or replug restores it.
///
/// Already being in mass-storage mode is success, so a front-end can call this
/// without first re-checking the mode it saw when the operation failed.
pub fn switch_to_mass_storage(l: &Localizer, sink: &mut dyn ProgressSink) -> Result<()> {
    let camera = discovery::find_single_camera()?;
    match camera.mode {
        CameraMode::MassStorage => return Ok(()),
        CameraMode::Mtp => {}
        mode => return Err(Error::WrongMode { message: l.tr("mode-switch-impossible"), mode }),
    }

    sink.emit(ProgressEvent::step(ProgressKind::Switching, l.tr("mode-switching")));
    discovery::switch_to_mass_storage(&camera)?;

    sink.emit(ProgressEvent::step(ProgressKind::Waiting, wait_message(l, "mode-switch-wait")));
    discovery::wait_for_mode_or_choice(
        CameraMode::MassStorage,
        MODE_SWITCH_ATTEMPTS,
        MODE_SWITCH_DELAY,
        &mut choice_hint(l, sink),
    )
    .map_err(|e| timeout_or(l, e, "mode-switch-timeout"))?;

    sink.emit(ProgressEvent::step(ProgressKind::Done, l.tr("mode-switch-done")));
    Ok(())
}

/// Switch a command-mode camera into application-install mode and wait for it
/// to re-enumerate.
fn switch_to_app_install(
    l: &Localizer,
    camera: &discovery::DetectedCamera,
    sink: &mut dyn ProgressSink,
) -> Result<pmca_usb::app_install::AppInstallCamera<discovery::AppInstallTransport>> {
    use pmca_usb::sony_extcmd::SonyExtCmdCamera;

    sink.emit(ProgressEvent::step(ProgressKind::Switching, l.tr("install-switching")));
    let transport = discovery::open_extcmd_transport(camera)?;
    let mut cam = SonyExtCmdCamera::new(transport);
    // A camera whose `usbAppInstaller` setting is off rejects this command
    // outright, and its raw wording ("unsupported", "ext-cmd write failed")
    // sends the user looking for a broken cable or an unsupported model. The
    // real cause is a setting the tweaks view can turn back on, so say so.
    cam.switch_to_app_installer().map_err(|_| Error::Operation(l.tr("install-disabled")))?;
    drop(cam.into_transport());

    sink.emit(ProgressEvent::step(ProgressKind::Waiting, wait_message(l, "install-switch-wait")));
    // Only a camera that never showed up is a timeout. One that appeared but
    // refused to open has a specific cause (typically a driver binding) that
    // must reach the user unaltered.
    discovery::wait_for_app_install(MODE_SWITCH_ATTEMPTS, MODE_SWITCH_DELAY).map_err(|e| match e {
        pmca_usb::Error::NoDevice => Error::Operation(l.tr("install-switch-timeout")),
        other => other.into(),
    })
}

/// Render an unparsable camera payload for a diagnostic message.
///
/// Both forms are needed: the text shows a plain-text error the camera may have
/// sent instead of JSON, the hex shows whether the bytes are framing debris.
fn describe_payload(body: &[u8]) -> String {
    const LIMIT: usize = 96;
    let shown = &body[..body.len().min(LIMIT)];
    let text: String =
        shown.iter().map(|&b| if (0x20..0x7f).contains(&b) { b as char } else { '.' }).collect();
    let hex: Vec<String> = shown.iter().map(|b| format!("{b:02x}")).collect();
    format!("payload ({} bytes): \"{text}\" [{}]", body.len(), hex.join(" "))
}

/// Bridges the USB install observer onto a [`ProgressSink`].
struct SinkObserver<'a> {
    l: &'a Localizer,
    sink: &'a mut dyn ProgressSink,
}

impl pmca_usb::app_install::InstallObserver for SinkObserver<'_> {
    fn on_start(&mut self, body: &[u8]) -> pmca_usb::Result<()> {
        let value: serde_json::Value = serde_json::from_slice(body).map_err(|_| {
            // The payload is the only evidence of what the camera disagreed
            // about, and the error type cannot carry it, so report it here.
            self.sink.emit(ProgressEvent::step(ProgressKind::Info, describe_payload(body)));
            pmca_usb::Error::Protocol("invalid install start response")
        })?;
        let code = value.get("resultCode").and_then(|v| v.as_i64()).unwrap_or(-1);
        if code != 0 {
            let message = value.get("message").and_then(|v| v.as_str()).unwrap_or("");
            self.sink.emit(ProgressEvent::step(
                ProgressKind::Info,
                format!("{} {}: {}", self.l.tr("install-error"), code, message),
            ));
            return Err(pmca_usb::Error::Protocol("install start rejected"));
        }
        Ok(())
    }

    fn on_progress(&mut self, body: &[u8]) {
        let Ok(value) = serde_json::from_slice::<serde_json::Value>(body) else {
            return;
        };
        let percent = value.get("percent").and_then(|v| v.as_i64()).unwrap_or(0);
        let message = value.get("status text").and_then(|v| v.as_str()).unwrap_or("");
        let percent = u8::try_from(percent.clamp(0, 100)).unwrap_or(0);
        self.sink.emit(ProgressEvent::progress(message, percent));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn localizer() -> Localizer {
        Localizer::new("en-US")
    }

    #[test]
    fn closure_sinks_collect_events() {
        let mut seen = Vec::new();
        {
            let mut sink = |e: ProgressEvent| seen.push(e);
            sink.emit(ProgressEvent::step(ProgressKind::Info, "a"));
            sink.emit(ProgressEvent::progress("b", 50));
        }
        assert_eq!(seen.len(), 2);
        assert_eq!(seen[1].percent, Some(50));
    }

    #[test]
    fn null_sink_discards_events() {
        NullSink.emit(ProgressEvent::step(ProgressKind::Done, "x"));
    }

    #[test]
    fn accepts_a_zero_result_code() {
        let v = serde_json::json!({ "resultCode": 0 });
        assert!(check_result_code(&localizer(), &v).is_ok());
    }

    #[test]
    fn rejects_a_nonzero_result_code_with_the_camera_message() {
        let v = serde_json::json!({ "resultCode": 7, "message": "boom" });
        let err = check_result_code(&localizer(), &v).unwrap_err();
        let text = err.to_string();
        assert!(text.contains('7'), "{text}");
        assert!(text.contains("boom"), "{text}");
    }

    #[test]
    fn rejects_a_payload_without_a_result_code() {
        let v = serde_json::json!({});
        assert!(check_result_code(&localizer(), &v).is_err());
    }

    /// One error per variant, including every variant of the wrapped crates.
    ///
    /// Adding a variant does not break this list automatically, so keep it in
    /// step with the exhaustive `match` in [`Error::code`].
    fn one_error_per_variant() -> Vec<Error> {
        let usb = [
            pmca_usb::Error::Transfer("x".into()),
            pmca_usb::Error::Driver("x".into()),
            pmca_usb::Error::Permission("x".into()),
            pmca_usb::Error::Busy("x".into()),
            pmca_usb::Error::NeedsRoot,
            pmca_usb::Error::NoMassStorageVolume,
            pmca_usb::Error::AppInstallMode,
            pmca_usb::Error::Stall,
            pmca_usb::Error::Protocol("x"),
            pmca_usb::Error::MscStatus { status: 3 },
            pmca_usb::Error::MscSense { key: 3, asc: 0x11, ascq: 0 },
            pmca_usb::Error::PtpResponse { phase: "write", code: 0x2002 },
            pmca_usb::Error::Unsupported,
            pmca_usb::Error::UpdaterStatus("x".into()),
            pmca_usb::Error::Senser("x".into()),
            pmca_usb::Error::NoDevice,
            pmca_usb::Error::CameraNotReady,
            pmca_usb::Error::TooManyDevices,
        ];
        let market = [
            pmca_market::Error::Core(pmca_core::Error::Invalid("x")),
            pmca_market::Error::Json("x".into()),
            pmca_market::Error::MissingField("x"),
            pmca_market::Error::NoApk,
            pmca_market::Error::Http("x".into()),
            pmca_market::Error::Io("x".into()),
            pmca_market::Error::Tls("x".into()),
        ];
        let mut all: Vec<Error> = usb.into_iter().map(Error::Usb).collect();
        all.extend(market.into_iter().map(Error::Market));
        all.push(Error::Io(std::io::Error::other("x")));
        all.push(Error::Operation("already translated".into()));
        all.push(Error::DirectRefused("already translated".into()));
        all
    }

    #[test]
    fn every_error_code_is_distinct_from_the_english_message() {
        for error in one_error_per_variant() {
            let code = error.code();
            assert!(!code.is_empty(), "{error:?} has an empty code");
            assert!(
                code.chars().all(|c| c.is_ascii_lowercase() || c == '-'),
                "code {code:?} is not a stable kebab-case identifier"
            );
        }
    }

    /// The point of the codes is that a front-end can translate them, which
    /// only works if a translation actually exists in *every* shipped language.
    /// A missing key would otherwise surface as untranslated English at the one
    /// moment the user most needs to understand what happened.
    #[test]
    fn every_error_code_has_a_translation_in_every_language() {
        for locale in Localizer::new("en-US").available() {
            let catalogue = pmca_i18n::locale_source(&locale).expect("catalogue for listed locale");
            for error in one_error_per_variant() {
                // `operation` carries a message that the producing layer has
                // already localized, so it must *not* be translated again.
                if error.code() == "operation" {
                    continue;
                }
                let key = format!("err-{}", error.code());
                assert!(
                    catalogue.lines().any(|l| l.trim_start().starts_with(&format!("{key} ="))),
                    "{locale} is missing {key}"
                );
            }
        }
    }

    #[test]
    fn the_usb_mode_choice_hint_is_translated() {
        let l = Localizer::new("en-US");
        assert_ne!(l.tr("camera-choose-usb-mode"), "camera-choose-usb-mode");
        let waiting = timeout_or(&l, pmca_usb::Error::CameraNotReady, "mode-switch-timeout");
        assert_eq!(waiting.code(), "usb-camera-not-ready");
        let gone = timeout_or(&l, pmca_usb::Error::NoDevice, "mode-switch-timeout");
        assert_eq!(gone.code(), "operation");
    }

    #[test]
    fn an_already_localized_operation_error_is_passed_through_untouched() {
        let error = Error::Operation("Kamera nicht gefunden".into());
        assert_eq!(error.code(), "operation");
        assert_eq!(error.detail(), None);
        assert_eq!(error.to_string(), "Kamera nicht gefunden");
    }

    #[test]
    fn a_single_value_refusal_offers_the_whole_image() {
        let error = Error::DirectRefused("Kamera hat abgelehnt".into());
        assert_eq!(error.code(), "operation");
        assert_eq!(error.detail(), None);
        assert_eq!(error.recovery(), Some(RECOVERY_WHOLE_IMAGE));
        assert_eq!(error.to_string(), "Kamera hat abgelehnt");
        assert_eq!(Error::Operation("x".into()).recovery(), None);
    }

    #[test]
    fn a_wrapped_transport_error_keeps_its_own_code_and_detail() {
        let error = Error::Usb(pmca_usb::Error::MscStatus { status: 5 });
        assert_eq!(error.code(), "usb-msc-status");
        assert_eq!(error.detail().as_deref(), Some("5"));
    }
}
