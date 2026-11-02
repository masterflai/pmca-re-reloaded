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

use std::path::{Path, PathBuf};
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

/// How often to try the first command on the camera back in updater mode.
const REOPEN_ATTEMPTS: usize = 5;

/// Delay between those tries.
const REOPEN_DELAY: Duration = Duration::from_secs(2);

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
    /// Raise the image's declared version to just one step above the installed
    /// one before sending, so the camera accepts the same version again (or a
    /// lower one). The genuine version is restored over service mode afterwards,
    /// so the reported version ends up correct. Requires the chip's FDAT key.
    pub reflash: bool,
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

    // Re-flash restores the genuine version over service mode afterwards; without
    // the service-mode keys that second step cannot run and would strand the
    // camera on the raised placeholder version, so refuse before writing anything.
    if request.reflash
        && request.flash
        && !pmca_core::keystore::class_present(pmca_core::keystore::KeyClass::Senser)
    {
        return Err(Error::MissingKeys(l.tr("firmware-reflash-needs-service-keys")));
    }

    let data = std::fs::read(request.file)?;
    let loc = pmca_core::firmware::read_dat(&data)
        .map_err(|e| Error::Operation(format!("{}: {e}", l.tr("firmware-invalid"))))?;
    let raw = data
        .get(loc.offset..loc.offset + loc.size)
        .ok_or_else(|| Error::Operation(l.tr("firmware-truncated")))?;

    let camera = discovery::find_single_camera()?;
    let already_updater = camera.mode == CameraMode::Updater;
    let transport = discovery::open_extcmd_transport(&camera)?;

    let mut updater = SonyUpdaterCamera::new(transport);
    updater.init()?;

    // Decide what to send. For a re-flash the installed version is read first so
    // the image's declared version can be raised to just one step above it — the
    // smallest value that clears the camera's version gate. Keeping it minimal
    // means an interrupted version-restore leaves the camera one notch above its
    // real version (still open to genuine updates) rather than pinned at an
    // unreachable maximum.
    let patched: Vec<u8>;
    let real_version: Option<(u8, u8)>;
    let current_version: String;
    let new_version: String;
    let payload: &[u8] = if request.reflash {
        let installed = updater.get_firmware_version()?;
        let (major, minor) = parse_version(&installed.old)
            .ok_or_else(|| Error::Operation(l.tr("firmware-invalid")))?;
        let (raised_major, raised_minor) = next_version(major, minor);
        let image = crate::fdat::force_version(l, raw, raised_major, raised_minor)?;
        real_version = Some((image.real_version_major, image.real_version_minor));
        patched = image.payload;
        sink.emit(ProgressEvent::step(ProgressKind::Info, l.tr("firmware-reflash-applied")));
        updater.check_guard(&patched)?;
        current_version = installed.old;
        new_version = version_string(raised_major, raised_minor);
        &patched
    } else {
        real_version = None;
        updater.check_guard(raw)?;
        let versions = updater.get_firmware_version()?;
        current_version = versions.old;
        new_version = versions.new;
        raw
    };
    sink.emit(ProgressEvent::step(ProgressKind::Info, l.tr("updater-guard-ok")));

    sink.emit(ProgressEvent::step(
        ProgressKind::Info,
        format!("{} {} -> {}", l.tr("firmware-updating"), current_version, new_version),
    ));
    let outcome = FirmwareOutcome { current_version, new_version, flashed: false };

    let Some(mut updater) =
        enter_updater_mode(l, updater, payload, already_updater, request.flash, sink)?
    else {
        return Ok(outcome);
    };

    sink.emit(ProgressEvent::step(ProgressKind::Progress, l.tr("updater-flashing")));
    updater.write_firmware(payload, None)?;
    updater.complete()?;
    sink.emit(ProgressEvent::step(ProgressKind::Done, l.tr("msg-done")));

    // The raised version also becomes the version the camera reports, which
    // would block later updates and collide with the next re-flash. Restore the
    // genuine version so the camera reports it and stays re-flashable.
    if let Some((major, minor)) = real_version {
        restore_reported_version(l, major, minor, sink);
    }

    Ok(FirmwareOutcome { flashed: true, ..outcome })
}

/// The register whose value the camera reports as its firmware version.
const VERSION_REGISTER_PATH: &str = "/setting/updater/dat4";

/// How many times to poll for the camera to return after the flash reboot.
const POST_FLASH_ATTEMPTS: usize = 90;

/// Delay between those polls.
const POST_FLASH_DELAY: Duration = Duration::from_secs(1);

/// Write `major.minor` to the camera's version register over service mode.
///
/// Shared by the post-flash restore and the standalone Plan-B commands. It waits
/// for the camera to be back in a normal mode first, so it works both right
/// after a flash reboot and on a camera already sitting idle.
fn write_version_via_service(
    l: &Localizer,
    major: u8,
    minor: u8,
    sink: &mut dyn ProgressSink,
) -> Result<()> {
    wait_for_post_flash_camera()?;
    let mut session = crate::service::ServiceSession::open(l, sink)?;
    // dat4 holds [minor, major]; the camera shows it as major.minor.
    let written = session.write_camera_file(VERSION_REGISTER_PATH, &[minor, major]);
    session.close();
    written
}

/// Restore the genuine version after a sentinel re-flash (step 2 of two).
///
/// Best-effort: the firmware is already written, so a failure here records the
/// owed version (see [`pending_version`]) and points the user at
/// [`restore_pending_version`] rather than failing the whole operation.
fn restore_reported_version(l: &Localizer, major: u8, minor: u8, sink: &mut dyn ProgressSink) {
    sink.emit(ProgressEvent::step(ProgressKind::Info, l.tr("firmware-reflash-restoring")));
    match write_version_via_service(l, major, minor, sink) {
        Ok(()) => {
            clear_pending_version();
            sink.emit(ProgressEvent::step(
                ProgressKind::Done,
                l.tr_arg("firmware-reflash-restored", "version", &version_string(major, minor)),
            ));
        }
        Err(e) => {
            save_pending_version(major, minor);
            sink.emit(ProgressEvent::step(
                ProgressKind::Notice,
                format!("{}: {e}", l.tr("firmware-reflash-restore-failed")),
            ));
            sink.emit(ProgressEvent::step(
                ProgressKind::Notice,
                l.tr_arg("firmware-reflash-pending", "version", &version_string(major, minor)),
            ));
        }
    }
}

/// Formats a version pair the way the camera displays it (`major.minor`).
fn version_string(major: u8, minor: u8) -> String {
    format!("{major:x}.{minor:02x}")
}

/// Parses a `major.minor` version (hex, e.g. `1.10`) into its two bytes.
fn parse_version(s: &str) -> Option<(u8, u8)> {
    let (major, minor) = s.split_once('.')?;
    Some((u8::from_str_radix(major.trim(), 16).ok()?, u8::from_str_radix(minor.trim(), 16).ok()?))
}

/// The smallest version strictly above `major.minor`, carrying into the major
/// byte when the minor byte is already at its maximum. This is the least the
/// re-flash has to declare to clear the camera's version gate.
fn next_version(major: u8, minor: u8) -> (u8, u8) {
    match minor.checked_add(1) {
        Some(next) => (major, next),
        None => (major.saturating_add(1), 0),
    }
}

/// Record of the genuine version a failed re-flash still owes the camera.
#[derive(serde::Serialize, serde::Deserialize)]
struct PendingVersion {
    major: u8,
    minor: u8,
}

/// Path of the pending-version record under the user's data folder.
fn pending_version_path() -> Option<PathBuf> {
    Some(pmca_i18n::catalog::user_data_dir()?.join("pending-version.json"))
}

fn save_pending_version(major: u8, minor: u8) {
    let Some(path) = pending_version_path() else { return };
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Ok(json) = serde_json::to_string(&PendingVersion { major, minor }) {
        let _ = std::fs::write(path, json);
    }
}

fn load_pending_version() -> Option<(u8, u8)> {
    let text = std::fs::read_to_string(pending_version_path()?).ok()?;
    let p: PendingVersion = serde_json::from_str(&text).ok()?;
    Some((p.major, p.minor))
}

fn clear_pending_version() {
    if let Some(path) = pending_version_path() {
        let _ = std::fs::remove_file(path);
    }
}

/// The genuine version a failed re-flash still owes the camera, if any.
///
/// A front-end shows this and offers [`restore_pending_version`] when the camera
/// is left reporting the `9.99` sentinel from an interrupted re-flash.
#[must_use]
pub fn pending_version() -> Option<String> {
    let (major, minor) = load_pending_version()?;
    Some(version_string(major, minor))
}

/// Finish a re-flash whose version restore did not complete (Plan B).
///
/// Returns the version written, so a front-end can report it.
pub fn restore_pending_version(l: &Localizer, sink: &mut dyn ProgressSink) -> Result<String> {
    let (major, minor) =
        load_pending_version().ok_or_else(|| Error::Operation(l.tr("firmware-restore-none")))?;
    write_version_via_service(l, major, minor, sink)?;
    clear_pending_version();
    let version = version_string(major, minor);
    sink.emit(ProgressEvent::step(
        ProgressKind::Done,
        l.tr_arg("firmware-reflash-restored", "version", &version),
    ));
    Ok(version)
}

/// Write an arbitrary reported version by hand over service mode (Plan B).
///
/// Returns the normalised version written, so a front-end can report it.
pub fn set_firmware_version(
    l: &Localizer,
    version: &str,
    sink: &mut dyn ProgressSink,
) -> Result<String> {
    let (major, minor) = parse_version(version)
        .ok_or_else(|| Error::Operation(l.tr("firmware-set-version-invalid")))?;
    write_version_via_service(l, major, minor, sink)?;
    clear_pending_version();
    let written = version_string(major, minor);
    sink.emit(ProgressEvent::step(
        ProgressKind::Done,
        l.tr_arg("firmware-reflash-restored", "version", &written),
    ));
    Ok(written)
}

/// Poll for the camera to re-enumerate in a normal mode after the flash reboot,
/// so the service-mode restore can reach it.
fn wait_for_post_flash_camera() -> Result<()> {
    for attempt in 0..POST_FLASH_ATTEMPTS {
        if let Ok(camera) = discovery::find_single_camera()
            && camera.mode != CameraMode::Updater
        {
            return Ok(());
        }
        if attempt + 1 < POST_FLASH_ATTEMPTS {
            std::thread::sleep(POST_FLASH_DELAY);
        }
    }
    Err(Error::Usb(pmca_usb::Error::NoDevice))
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
    let mut updater = open_rebooted(&camera)?;
    // Re-check against the rebooted camera: that is the one about to be written.
    updater.check_guard(payload)?;
    Ok(Some(updater))
}

/// Open and initialise the camera that has just come back in updater mode.
///
/// Right after it re-appears, the first command can fail in transit while the
/// drive is still being set up; INIT writes nothing, so it is simply retried.
fn open_rebooted(camera: &DetectedCamera) -> Result<BoxedUpdater> {
    let mut attempt = 1;
    loop {
        let result = discovery::open_extcmd_transport(camera).and_then(|transport| {
            let mut updater = SonyUpdaterCamera::new(transport);
            updater.init().map(|()| updater)
        });
        match result {
            Err(pmca_usb::Error::Transfer(_)) if attempt < REOPEN_ATTEMPTS => {
                attempt += 1;
                std::thread::sleep(REOPEN_DELAY);
            }
            other => return Ok(other?),
        }
    }
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
