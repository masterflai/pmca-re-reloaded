//! Reaching service mode and leaving it again.
//!
//! Shared by every service-mode user — tweaks, the service shell and the
//! function analysis. Covers the mode switch with its two driver assignments,
//! the way back, and the clean-up after a run that never got to the way back.

use pmca_i18n::Localizer;
use pmca_usb::discovery::{self, CameraMode};
use pmca_usb::nusb_backend::NusbBackend;
use pmca_usb::senser_camera::SenserCamera;

use crate::drivers;
use crate::dto::{ProgressEvent, ProgressKind};
use crate::{Error, ProgressSink, Result};

/// How long to wait for the camera to re-enumerate after a mode switch.
///
/// On Windows this covers more than the camera's own switch: the device
/// reappears under a different product id, which the OS has to enumerate and
/// bind to WinUSB before it becomes visible at all. Measured at roughly 15 s on
/// a first-generation binding, so 10 s was not enough.
const SWITCH_ATTEMPTS: usize = 60;
const SWITCH_DELAY: std::time::Duration = std::time::Duration::from_millis(500);

/// How long the startup sweep waits for a camera it had to bind itself.
///
/// Shorter than [`SWITCH_ATTEMPTS`] on purpose: this runs before the user has
/// asked for anything, so it must not turn into a visible delay. Ten seconds
/// covers a driver assignment and the re-enumeration that follows it; a camera
/// that needs longer is found by the next start.
const RESTORE_ATTEMPTS: usize = 20;

/// End a service-mode session and undo what was needed to reach it.
///
/// Two things have to be given back, and neither is optional:
///
/// * The camera has to be told to leave service mode. One that is left there
///   stalls the next session's authentication.
/// * On Windows, the WinUSB binding that made the session possible has to go.
///   While it is in place the camera's mass-storage and firmware-updater paths
///   do not work, because those need the in-box `USBSTOR` driver back — so a
///   binding left behind does not merely litter, it breaks the next thing the
///   user tries.
///
/// Called on every exit path, successful or not, which is why it takes no
/// result and returns none: there is nothing a caller could usefully do about
/// a failure here, and a cleanup that only ran after success would be no
/// cleanup at all. A binding that could not be removed is picked up instead by
/// the sweep at the next program start.
pub(crate) fn leave_service_mode(cam: &mut SenserCamera<NusbBackend>) {
    cam.stop();
    let _ = drivers::unbind_all();
}

/// Release driver bindings an earlier session left behind, before the camera
/// is used again.
///
/// Such a binding keeps WinUSB on the camera's mass-storage side, where the
/// camera then reports "check the connected device" and every mass-storage
/// operation fails. It also lets a service session start without the rights
/// it would need to remove the binding again at its end, which is how a
/// process without administrator rights got into this state.
/// Removing needs those rights; the driver helper is started for it if need be.
pub fn release_leftovers(l: &Localizer) -> Result<()> {
    if pmca_usb::winusb_install::pending_packages() == 0 {
        return Ok(());
    }
    let _ = drivers::sweep();
    if pmca_usb::winusb_install::pending_packages() == 0 {
        return Ok(());
    }
    Err(Error::NeedsElevation(l.tr("driver-leftover-needs-elevation")))
}

/// What a startup sweep found and undid.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SweepReport {
    /// Driver packages removed, and with them the assignments they made.
    pub packages: usize,
    /// Whether a camera was found stranded in service mode and asked to leave.
    pub camera_restored: bool,
}

impl SweepReport {
    /// Whether anything at all had to be undone.
    pub fn is_empty(&self) -> bool {
        self.packages == 0 && !self.camera_restored
    }
}

/// Undo anything a previous run left behind. Call once at program start.
///
/// The counterpart to [`leave_service_mode`], for the case where that never
/// got to run: a crash, a power loss, or the process being killed. Two things
/// are left behind then, and undoing only one of them is not enough:
///
/// * The camera is bound to WinUSB, which silently disables its mass-storage
///   and firmware-updater modes.
/// * The camera is still *in* service mode, which stalls the next session's
///   authentication — so releasing the driver alone leaves the user with a
///   camera that is visible again but refuses every tweak until it is
///   power-cycled.
///
/// Neither state is one the user has any way of recognising as ours, and
/// neither has an obvious way out. Sweeping at start rather than only at exit
/// is what turns that from a permanent breakage into a lost session.
///
/// The order below is not free to change: the stop request is a USB transfer,
/// so it needs a driver binding in place, and removing the packages is exactly
/// what takes that binding away. The camera therefore goes first, and any
/// binding it needed for itself is swept away by the line after it.
///
/// Infallible on purpose: this runs before the user has asked for anything, so
/// a failure — most likely missing administrator rights — must not stop the
/// program from starting. Whatever survives is found again next time.
pub fn sweep_leftovers() -> SweepReport {
    let camera_restored = restore_camera_mode();
    SweepReport { packages: pmca_usb::winusb_install::sweep().packages.len(), camera_restored }
}

/// Ask a camera stranded in service mode to leave it, if there is one.
///
/// No authentication is attempted. The session that stranded the camera
/// already started its senser subsystem, and a second handshake against a
/// started subsystem is answered with a stalled endpoint — the stop request,
/// being a plain vendor transfer, is the one thing that still works here.
fn restore_camera_mode() -> bool {
    let Some(camera) = find_stranded_camera(drivers::bind_quietly) else { return false };
    discovery::leave_service_mode(&camera).is_ok()
}

/// Locate a camera left in service mode, making it visible first if need be.
///
/// Starts with the node list, and on Windows that decision is final: a device
/// the configuration manager does not report cannot be enumerated either, so
/// two cheap queries are enough to establish that there is nothing to do. That
/// matters because this runs at *every* program start, including commands that
/// never touch a camera — they should not pay for a USB enumeration. Other
/// platforms have no node list and are asked the ordinary way; they also have
/// no driver bindings to go wrong, so enumeration sees everything there.
///
/// The binding at the end is for the case the plain enumeration cannot reach:
/// a camera whose binding was already released — by an explicit `driver
/// unbind`, or by a sweep from a version that did not yet look at the camera —
/// is in service mode *and* driverless, and a driverless device is invisible.
/// The binding exists only to carry the one request that ends the mode; the
/// package it stages is removed by the sweep that follows. `bind` decides
/// whether that may ask for administrator rights.
fn find_stranded_camera(
    bind: fn(&str) -> pmca_usb::winusb_install::Result<String>,
) -> Option<discovery::DetectedCamera> {
    let nodes = senser_nodes();
    if cfg!(windows) && nodes.is_empty() {
        return None;
    }

    if let Ok(cameras) = discovery::list_cameras()
        && let Some(camera) = cameras.into_iter().find(|c| c.mode == CameraMode::Senser)
    {
        return Some(camera);
    }

    for id in nodes {
        if bind(&id).is_err() {
            continue;
        }
        if let Ok(camera) = discovery::wait_for_senser(RESTORE_ATTEMPTS, SWITCH_DELAY) {
            return Some(camera);
        }
    }

    None
}

/// The hardware ids of every service-mode product id currently on the bus.
fn senser_nodes() -> Vec<String> {
    pmca_usb::sony::SONY_ID_PRODUCT_SENSER
        .iter()
        .map(|product| format!(r"USB\VID_{:04X}&PID_{product:04X}", pmca_usb::sony::SONY_ID_VENDOR))
        .filter(|id| pmca_usb::winusb_install::device_present(id))
        .collect()
}

/// Rescue a camera an earlier run stranded in service mode, on the way in.
///
/// The startup sweep handles this too, but it only runs once, at start. A
/// camera plugged in after it — or switched on after it — is still stranded
/// and nothing has looked at it since. Left alone it fails in a way that
/// explains nothing: its senser subsystem is already started, so the
/// authentication below answers with a stalled endpoint, and the user is told
/// their camera is in the wrong mode while it sits there in exactly the mode
/// the tool was trying to reach.
///
/// Ending the stranded session is enough. The camera then returns in whatever
/// mode its own USB menu selects, and the ordinary path takes it from there —
/// which is why this hands back a camera rather than a service-mode session.
///
/// `None` means there was nothing stranded to find, and the caller keeps
/// whatever it was going to report anyway.
fn recover_stranded_camera(
    l: &Localizer,
    sink: &mut dyn ProgressSink,
) -> Option<discovery::DetectedCamera> {
    let camera = find_stranded_camera(drivers::bind)?;

    sink.emit(ProgressEvent::step(ProgressKind::Switching, l.tr("service-stranded-reset")));
    discovery::leave_service_mode(&camera).ok()?;

    // Any binding made just above to reach the camera is left in place. The
    // mass-storage side needs one moments later anyway, and the session's own
    // clean-up removes every package regardless of who staged it.
    sink.emit(ProgressEvent::step(
        ProgressKind::Waiting,
        crate::wait_message(l, "mode-switch-wait"),
    ));
    wait_for_normal_mode()
}

/// Wait for a camera to come back in a mode a session can be started from.
///
/// Which one it will be is the camera's decision, not ours: leaving service
/// mode returns it to the USB mode selected in its own menu, so both
/// mass storage and MTP are correct outcomes here.
fn wait_for_normal_mode() -> Option<discovery::DetectedCamera> {
    for _ in 0..SWITCH_ATTEMPTS {
        std::thread::sleep(SWITCH_DELAY);
        if let Ok(cameras) = discovery::list_cameras()
            && let Some(camera) = cameras
                .into_iter()
                .find(|c| matches!(c.mode, CameraMode::MassStorage | CameraMode::Mtp))
        {
            return Some(camera);
        }
    }
    None
}

/// Bring the attached camera into service mode and authenticate.
///
/// Service mode is entered from mass storage, so an MTP camera is asked to
/// reconnect first. That step cannot be left to the user: leaving service mode
/// puts the camera back into the USB mode configured in its own menu, so a
/// camera that is normally MTP is MTP again after every single tweak
/// operation — reading the tweaks would work and applying them would not.
///
/// A failure part-way through releases whatever was bound so far. Without
/// that, a camera that does not reach service mode is left on WinUSB with no
/// session to close it — mass storage gone, updater mode gone, and no visible
/// cause. The sweep at the next program start would find it eventually, but
/// "eventually" is the wrong answer when the user is holding the camera.
pub(crate) fn open_service_camera(
    l: &Localizer,
    sink: &mut dyn ProgressSink,
) -> Result<SenserCamera<NusbBackend>> {
    let result = enter_service_mode(l, sink);
    if result.is_err() {
        release_drivers(l, sink);
    }
    result
}

/// How long releasing the drivers may take before the user is asked to replug.
const RELEASE_PATIENCE: std::time::Duration = std::time::Duration::from_secs(15);

/// [`pmca_usb::winusb_install::unbind_all`], asking for the cable to be
/// replugged when it stalls.
///
/// Removing the package waits until Windows has finished setting the camera up
/// again, and a camera that refused the switch to service mode never finishes:
/// measured: only a replug ended the
/// wait after four minutes. The release itself still runs to completion.
fn release_drivers(l: &Localizer, sink: &mut dyn ProgressSink) {
    let (done, finished) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let _ = done.send(drivers::unbind_all());
    });
    if finished.recv_timeout(RELEASE_PATIENCE).is_err() {
        sink.emit(ProgressEvent::step(ProgressKind::Waiting, l.tr("driver-release-replug")));
        let _ = finished.recv();
    }
}

fn enter_service_mode(
    l: &Localizer,
    sink: &mut dyn ProgressSink,
) -> Result<SenserCamera<NusbBackend>> {
    release_leftovers(l)?;
    let camera = match discovery::find_single_camera() {
        // Already in service mode, and not because we put it there — this
        // function has not done anything yet. So an earlier run stranded it
        // and the startup sweep missed it, which happens when the camera was
        // plugged in after the sweep had already run.
        Ok(camera) if camera.mode == CameraMode::Senser => recover_stranded_camera(l, sink)
            .ok_or_else(|| Error::Operation(l.tr("service-stranded-failed")))?,
        Ok(camera) => camera,
        // The same situation wearing a different mask: a stranded camera whose
        // binding is gone is driverless, and a driverless device is invisible
        // to enumeration, so "no camera at all" is what it looks like from
        // here. Worth one look at the node list before believing it.
        Err(e) => recover_stranded_camera(l, sink).ok_or(Error::from(e))?,
    };

    let camera = match camera.mode {
        CameraMode::Mtp => {
            sink.emit(ProgressEvent::step(ProgressKind::Switching, l.tr("mode-switching")));
            discovery::switch_to_mass_storage(&camera)?;

            sink.emit(ProgressEvent::step(
                ProgressKind::Waiting,
                crate::wait_message(l, "mode-switch-wait"),
            ));
            discovery::wait_for_mode_or_choice(
                CameraMode::MassStorage,
                SWITCH_ATTEMPTS,
                SWITCH_DELAY,
                &mut crate::choice_hint(l, sink),
            )
            .map_err(|e| crate::timeout_or(l, e, "mode-switch-timeout"))?
        }
        _ => camera,
    };

    let camera = match camera.mode {
        CameraMode::MassStorage => {
            // The switch command below is a vendor control transfer, which the
            // mass-storage stack will not carry.
            let camera = ensure_raw_access(l, camera, sink)?;

            sink.emit(ProgressEvent::step(ProgressKind::Switching, l.tr("service-switching")));
            let mut auth =
                open_when_started(&camera, discovery::open_senser_auth_from_mass_storage)?;
            auth.start();
            auth.authenticate()?;

            sink.emit(ProgressEvent::step(
                ProgressKind::Waiting,
                crate::wait_message(l, "service-switch-wait"),
            ));
            await_service_mode(l, sink)?
        }
        mode => return Err(Error::WrongMode { message: l.tr("service-bad-mode"), mode }),
    };

    // Service mode is a different USB device with a different product id, and
    // Windows has no driver for it at all — so it needs its own binding, even
    // when the mass-storage side was already bound above.
    let camera = ensure_raw_access(l, camera, sink)?;

    sink.emit(ProgressEvent::step(ProgressKind::Authenticating, l.tr("service-authenticating")));
    Ok(SenserCamera::new(open_when_started(&camera, discovery::open_senser)?))
}

/// How often an open refused by a driver that is still starting is retried.
const DRIVER_START_ATTEMPTS: usize = 20;

/// Open `camera`, retrying while Windows refuses because the driver it was
/// just given has not started yet.
///
/// The driver name already reads WinUSB at that point, so only the open itself
/// tells; in a measurement the open came two seconds
/// too early. Each retry looks the camera up afresh.
fn open_when_started<T>(
    camera: &discovery::DetectedCamera,
    open: impl Fn(&discovery::DetectedCamera) -> pmca_usb::Result<T>,
) -> Result<T> {
    let mut result = open(camera);
    for _ in 0..DRIVER_START_ATTEMPTS {
        if !matches!(result, Err(pmca_usb::Error::Driver(_))) {
            break;
        }
        std::thread::sleep(SWITCH_DELAY);
        if let Ok(cameras) = discovery::list_cameras()
            && let Some(fresh) = cameras.into_iter().find(|c| c.mode == camera.mode)
        {
            result = open(&fresh);
        }
    }
    Ok(result?)
}

/// Wait for the camera to come back in service mode, assigning WinUSB to it
/// the moment Windows notices it.
///
/// Simply polling the USB backend does not work, and the reason is a
/// chicken-and-egg: service mode is a new USB device with a product id no
/// in-box driver claims, and the backend only enumerates devices that already
/// have a driver. So the camera sits on the bus in an error state, the backend
/// reports nothing, and the wait runs to its timeout while the device it is
/// waiting for is plugged in the whole time. Measured: the
/// node appears about six seconds after the switch and never becomes visible.
///
/// The configuration manager has no such blind spot, so it is asked instead —
/// and once it says the device is there, binding makes it real for the backend
/// too. Binding is attempted once per product id: a repeat every half second
/// would stage the same package over and over while a genuinely failing bind
/// keeps failing.
fn await_service_mode(
    l: &Localizer,
    sink: &mut dyn ProgressSink,
) -> Result<discovery::DetectedCamera> {
    let mut bound: Vec<u16> = Vec::new();
    // A camera that refuses the switch is back in mass storage when the wait
    // ends; it may drop off the bus briefly before that.
    let mut mass_storage = false;

    for _ in 0..SWITCH_ATTEMPTS {
        std::thread::sleep(SWITCH_DELAY);

        if let Ok(cameras) = discovery::list_cameras() {
            if let Some(camera) = cameras.iter().find(|c| c.mode == CameraMode::Senser) {
                return Ok(camera.clone());
            }
            mass_storage = cameras.iter().any(|c| c.mode == CameraMode::MassStorage);
        }

        for &product in pmca_usb::sony::SONY_ID_PRODUCT_SENSER {
            let id = format!(r"USB\VID_{:04X}&PID_{product:04X}", pmca_usb::sony::SONY_ID_VENDOR);
            if bound.contains(&product) || !pmca_usb::winusb_install::device_present(&id) {
                continue;
            }

            sink.emit(ProgressEvent::step(ProgressKind::Switching, l.tr("driver-binding")));
            drivers::bind(&id).map_err(|e| drivers::localize_error(l, e))?;
            bound.push(product);
            sink.emit(ProgressEvent::step(
                ProgressKind::Waiting,
                crate::wait_message(l, "driver-bind-wait"),
            ));
        }
    }

    if mass_storage {
        return Err(Error::Operation(l.tr("service-switch-refused")));
    }
    Err(Error::Operation(format!("{} {}", l.tr("service-switch-timeout"), l.tr("usb-mode-advice"))))
}

/// Make sure the camera can be reached with raw USB transfers, assigning
/// WinUSB to it if it cannot.
///
/// This is what replaces the manual Zadig step. It is deliberately placed at
/// both ends of the service path rather than once at the start: mass storage
/// and service mode are separate USB devices to Windows, each with its own
/// driver assignment.
///
/// A fresh [`discovery::DetectedCamera`] is returned because binding
/// re-enumerates the device: the handle passed in refers to a device node that
/// no longer exists, and using it would fail in a way that looks like a
/// disconnected camera.
fn ensure_raw_access(
    l: &Localizer,
    camera: discovery::DetectedCamera,
    sink: &mut dyn ProgressSink,
) -> Result<discovery::DetectedCamera> {
    let driver = camera.device.os_driver();
    if !pmca_usb::winusb_install::needs_binding(driver.as_deref()) {
        return Ok(camera);
    }

    let id =
        format!(r"USB\VID_{:04X}&PID_{:04X}", camera.device.vendor_id, camera.device.product_id);
    let mode = camera.mode;

    sink.emit(ProgressEvent::step(ProgressKind::Switching, l.tr("driver-binding")));
    drivers::bind(&id).map_err(|e| drivers::localize_error(l, e))?;

    sink.emit(ProgressEvent::step(
        ProgressKind::Waiting,
        crate::wait_message(l, "driver-bind-wait"),
    ));
    discovery::wait_for_mode_or_choice(
        mode,
        SWITCH_ATTEMPTS,
        SWITCH_DELAY,
        &mut crate::choice_hint(l, sink),
    )
    .map_err(|e| crate::timeout_or(l, e, "driver-bind-timeout"))
}
