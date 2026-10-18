//! Flashing an official Sony firmware image onto the attached camera.
//!
//! This is the most destructive operation the toolkit offers: it overwrites the
//! camera's firmware, and an interrupted or mismatched write can leave the
//! device unusable. The module is therefore built around two ideas.
//!
//! First, **validation is separable from writing**. [`FlashRequest::flash`] is
//! off by default, and with it off the camera is only opened, asked whether it
//! accepts the image, and queried for its version pair. Nothing is written, and
//! the camera is not even rebooted. A front-end can use this to show the user
//! the exact version transition before asking for confirmation.
//!
//! Second, **the camera decides compatibility**, not this code. The image is
//! submitted to the updater's guard check, and a rejection is surfaced with the
//! camera's own status code. Guessing from model names would be a worse test
//! than the one the device already performs on itself.

use std::path::Path;
use std::time::Duration;

use pmca_i18n::Localizer;
use pmca_usb::command::SonyExtCmdTransport;
use pmca_usb::discovery::{self, CameraMode, DetectedCamera};
use pmca_usb::sony_updater::SonyUpdaterCamera;

use crate::dto::{FirmwareOutcome, ProgressEvent, ProgressKind};
use crate::{Error, ProgressSink, Result};

/// An updater-mode camera reached over a boxed, mode-agnostic transport.
///
/// Boxing keeps the type identical whether the camera was reached over MSC or
/// MTP, so the flow below does not have to be generic over the transport.
pub type BoxedUpdater = SonyUpdaterCamera<Box<dyn SonyExtCmdTransport>>;

/// How many times to poll for the camera returning in updater mode.
const UPDATER_POLL_ATTEMPTS: usize = 60;

/// Delay between updater-mode polls.
const UPDATER_POLL_DELAY: Duration = Duration::from_millis(500);

/// What to do with a firmware file.
#[derive(Debug, Clone, Copy)]
pub struct FlashRequest<'a> {
    /// The official `.dat` firmware file.
    pub file: &'a Path,
    /// Switch the camera into updater mode and write the payload.
    ///
    /// With `false` the image is only validated, leaving the camera untouched.
    pub flash: bool,
    /// Explicit acknowledgement that the camera's firmware will be overwritten.
    pub confirm: bool,
}

/// Validate an official firmware image against the camera, and flash it when
/// [`FlashRequest::flash`] and [`FlashRequest::confirm`] are both set.
///
/// Only the `FDAT` payload embedded in the `.dat` container is sent, exactly as
/// the reference implementation does.
pub fn flash_firmware(
    l: &Localizer,
    request: &FlashRequest<'_>,
    sink: &mut dyn ProgressSink,
) -> Result<FirmwareOutcome> {
    // Refuse before the camera is opened. Entering updater mode already reboots
    // the device, so an unconfirmed request must not get that far.
    if request.flash && !request.confirm {
        return Err(Error::Operation(l.tr("firmware-confirm-required")));
    }

    let data = std::fs::read(request.file)?;
    let loc = pmca_core::firmware::read_dat(&data)
        .map_err(|e| Error::Operation(format!("{}: {e}", l.tr("firmware-invalid"))))?;
    let payload = data
        .get(loc.offset..loc.offset + loc.size)
        .ok_or_else(|| Error::Operation(l.tr("firmware-truncated")))?;

    let camera = discovery::find_single_camera()?;
    let already_updater = camera.mode == CameraMode::Updater;
    let transport = discovery::open_extcmd_transport(&camera)?;

    let mut updater = SonyUpdaterCamera::new(transport);
    updater.init()?;
    updater.check_guard(payload)?;
    sink.emit(ProgressEvent::step(ProgressKind::Info, l.tr("updater-guard-ok")));

    let versions = updater.get_firmware_version()?;
    sink.emit(ProgressEvent::step(
        ProgressKind::Info,
        format!("{} {} -> {}", l.tr("firmware-updating"), versions.old, versions.new),
    ));
    let outcome = FirmwareOutcome {
        current_version: versions.old,
        new_version: versions.new,
        flashed: false,
    };

    let Some(mut updater) =
        enter_updater_mode(l, updater, payload, already_updater, request.flash, sink)?
    else {
        return Ok(outcome);
    };

    sink.emit(ProgressEvent::step(ProgressKind::Progress, l.tr("updater-flashing")));
    updater.write_firmware(payload, None)?;
    updater.complete()?;
    sink.emit(ProgressEvent::step(ProgressKind::Done, l.tr("msg-done")));

    Ok(FirmwareOutcome { flashed: true, ..outcome })
}

/// Bring the camera into firmware-updater mode, ready to receive `payload`.
///
/// Returns `Ok(None)` when the camera would have to be switched but `flash` was
/// not given: rebooting into updater mode and writing the payload is
/// destructive, so it never happens without an explicit opt-in.
///
/// Shared with the updater-shell flow, which writes a different payload but
/// must reach updater mode in exactly the same way.
pub fn enter_updater_mode(
    l: &Localizer,
    mut updater: BoxedUpdater,
    payload: &[u8],
    already_updater: bool,
    flash: bool,
    sink: &mut dyn ProgressSink,
) -> Result<Option<BoxedUpdater>> {
    if already_updater {
        // The payload can be written straight away on the current connection.
        sink.emit(ProgressEvent::step(ProgressKind::Info, l.tr("updater-already-updater")));
        return Ok(Some(updater));
    }

    if !flash {
        sink.emit(ProgressEvent::step(ProgressKind::Info, l.tr("updater-flash-required")));
        return Ok(None);
    }

    // Updater mode is mass storage; refuse before the reboot, not after it.
    if discovery::mass_storage_needs_root() {
        return Err(Error::Usb(pmca_usb::Error::NeedsRoot));
    }
    sink.emit(ProgressEvent::step(ProgressKind::Switching, l.tr("updater-switch-wait")));
    updater.switch_mode()?;
    // Release the rebooting device before polling for its updater-mode identity.
    drop(updater.into_transport());

    let camera =
        wait_for_updater_mode().ok_or_else(|| Error::Operation(l.tr("updater-switch-timeout")))?;
    let mut updater = SonyUpdaterCamera::new(discovery::open_extcmd_transport(&camera)?);
    updater.init()?;
    // Re-check against the rebooted camera: that is the one about to be written.
    updater.check_guard(payload)?;
    Ok(Some(updater))
}

/// Poll for the camera re-appearing in firmware-updater mode after a switch.
///
/// Exactly one updater-mode camera must be present. With several attached there
/// is no way to tell which one was just rebooted, and writing firmware to the
/// wrong device is not recoverable.
fn wait_for_updater_mode() -> Option<DetectedCamera> {
    for _ in 0..UPDATER_POLL_ATTEMPTS {
        std::thread::sleep(UPDATER_POLL_DELAY);
        if let Ok(cameras) = discovery::list_cameras() {
            let mut updaters = cameras.into_iter().filter(|c| c.mode == CameraMode::Updater);
            if let Some(first) = updaters.next()
                && updaters.next().is_none()
            {
                return Some(first);
            }
        }
    }
    None
}
