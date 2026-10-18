//! High-level camera discovery and identification.
//!
//! This module ties together the low-level [`nusb_backend`](crate::nusb_backend)
//! transport, the MSC/MTP protocol layers and the Sony command layers to offer a
//! single, hardware-facing entry point for the CLI and GUI. It mirrors the
//! behaviour of the legacy `pmca/commands/usb.py` device enumeration.
//!
//! All functions here perform real USB IO and are therefore only available with
//! the `hardware` feature enabled.

use crate::app_install::{AppInstallCamera, AppInstallDevice};
use crate::command::SonyExtCmdTransport;
use crate::msc::MscDevice;
use crate::mtp::MtpDevice;
use crate::nusb_backend::{DiscoveredDevice, NusbBackend, list_sony_devices};
use crate::senser::{SenserAuthDevice, SenserDevice, SenserEndpoints};
use crate::sony::{
    is_sony_msc_updater_camera, is_sony_mtp_app_install_camera, is_sony_mtp_camera,
    is_sony_senser_camera,
};
use crate::sony_extcmd::{CameraInfo, LensInfo, SonyExtCmdCamera};
use crate::sony_updater::{FirmwareVersions, SonyUpdaterCamera};
use crate::{Error, Result, UsbBackend, class};

/// The operating mode a connected Sony camera was detected in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CameraMode {
    /// Mass-storage camera speaking the Sony external-command protocol.
    MassStorage,
    /// PTP/MTP camera speaking the Sony external-command protocol.
    Mtp,
    /// Mass-storage device already in firmware-updater mode.
    Updater,
    /// PTP/MTP camera in application-install mode.
    AppInstall,
    /// Vendor-specific "senser" service mode.
    Senser,
    /// A Sony device whose mode could not be classified.
    Unknown,
}

impl CameraMode {
    /// A stable identifier; front-ends show it translated as `mode-<id>`.
    pub fn id(self) -> &'static str {
        match self {
            CameraMode::MassStorage => "mass-storage",
            CameraMode::Mtp => "mtp",
            CameraMode::Updater => "updater",
            CameraMode::AppInstall => "app-install",
            CameraMode::Senser => "service",
            CameraMode::Unknown => "unknown",
        }
    }
}

/// A discovered Sony device together with its detected mode.
#[derive(Debug, Clone)]
pub struct DetectedCamera {
    /// The underlying USB device.
    pub device: DiscoveredDevice,
    /// The detected operating mode.
    pub mode: CameraMode,
}

/// Identifying information gathered from a camera.
#[derive(Debug, Clone, Default)]
pub struct CameraInfoReport {
    /// Detected operating mode, as [`CameraMode::id`].
    pub mode: String,
    /// Human-readable model name (e.g. `ILCE-5100`), if available.
    pub model_name: Option<String>,
    /// Hex-encoded model code, if available.
    pub model_code: Option<String>,
    /// Hex-encoded serial number, if available.
    pub serial: Option<String>,
    /// Firmware version currently on the camera, if available.
    pub firmware: Option<String>,
    /// Mounted-lens description, if a lens with a non-zero model is present.
    pub lens: Option<String>,
}

/// Drivers a *camera* can sit behind while exposing no interface descriptors:
/// the Windows mass-storage stack, the composite parent, the raw drivers used
/// for service and updater mode, and the in-box MTP class driver that claims
/// cameras in MTP and application-installation mode (reached via
/// [`crate::wpd`], not through raw endpoints).
const CAMERA_CAPABLE_DRIVERS: [&str; 5] = ["usbstor", "usbccgp", "winusb", "libusbk", "wudfwpdmtp"];

/// How a camera in no data mode names itself, i.e. switched off and charging,
/// or showing the USB mode choice on its display. Matched by name because the
/// product id differs per model, while the names are "USB Bus-powered
/// Device" or "USB Charger". Game controllers and headsets
/// share the vendor id but not these names.
const WAITING_CAMERA_NAMES: [&str; 2] = ["USB Bus-powered Device", "USB Charger"];

fn is_waiting_camera_name(name: Option<&str>) -> bool {
    name.is_some_and(|n| WAITING_CAMERA_NAMES.iter().any(|w| n.trim().eq_ignore_ascii_case(w)))
}

fn is_waiting_camera(camera: &DetectedCamera) -> bool {
    camera.mode == CameraMode::Unknown
        && is_waiting_camera_name(camera.device.info.product_string())
}

/// Extra polls a wait grants while a camera sits at its USB mode choice, so a
/// user who is still reading the display is not timed out (two minutes at the
/// usual half-second delay).
const WAITING_PATIENCE: usize = 240;

/// Whether a device could be a camera at all.
///
/// Sharing Sony's vendor id is not enough: the same id covers headsets, game
/// controllers and mice, and one of those being plugged in must not make a
/// command report a camera "in the wrong mode" — or, worse, count towards
/// [`Error::TooManyDevices`] next to the real camera.
///
/// When the device advertises interfaces, we require a camera-ish class. When
/// it advertises none we have to fall back to the bound driver, because a
/// camera claimed by `USBSTOR` is just as featureless as a HID peripheral
/// claimed by `HidUsb`. Platforms that report no driver (Linux, macOS) always
/// populate the interface list, so the unknown-driver case stays permissive.
fn is_camera_candidate_from(classes: &[u8], driver: Option<&str>) -> bool {
    if !classes.is_empty() {
        return classes
            .iter()
            .any(|c| matches!(*c, class::MSC | class::PTP | class::VENDOR_SPECIFIC));
    }
    match driver {
        Some(driver) => CAMERA_CAPABLE_DRIVERS.iter().any(|d| driver.contains(d)),
        None => true,
    }
}

/// [`is_camera_candidate_from`] for a discovered device.
fn is_camera_candidate(device: &DiscoveredDevice, classes: &[u8]) -> bool {
    is_camera_candidate_from(classes, device.os_driver().as_deref())
}

/// Classify a discovered device by inspecting its interface classes and USB ids.
///
/// Sony cameras announce their mode at the interface level; the device-level
/// class is `0` for the composite MSC/MTP descriptors.
///
/// On Windows the interface classes are only visible while the device is bound
/// to the composite `usbccgp` driver (or WinUSB). A mass-storage device claimed
/// by the OS `USBSTOR` stack exposes no interface classes and cannot be opened
/// by `nusb`; in that case classification falls back to the product id, which
/// is sufficient to recognise updater and senser modes and to treat any other
/// interface-less Sony device as mass storage.
fn detect_mode(device: &DiscoveredDevice) -> Result<CameraMode> {
    // `interface_classes` needs an open handle, which a class driver refuses;
    // the cached descriptors are the next best source before giving up.
    let mut classes = device.interface_classes().unwrap_or_default();
    if classes.is_empty() {
        classes = device.cached_interface_classes();
    }
    if !is_camera_candidate(device, &classes) {
        return Ok(CameraMode::Unknown);
    }
    if classes.contains(&class::MSC) {
        if is_sony_msc_updater_camera(device.vendor_id, device.product_id) {
            return Ok(CameraMode::Updater);
        }
        return Ok(CameraMode::MassStorage);
    }
    if classes.contains(&class::PTP) {
        return Ok(CameraMode::Mtp);
    }
    if classes.contains(&class::VENDOR_SPECIFIC)
        && is_sony_senser_camera(device.vendor_id, device.product_id)
    {
        return Ok(CameraMode::Senser);
    }

    // No usable interface information (typically a Windows USBSTOR-bound device
    // that cannot be opened): classify by product id.
    if is_sony_msc_updater_camera(device.vendor_id, device.product_id) {
        return Ok(CameraMode::Updater);
    }
    if is_sony_senser_camera(device.vendor_id, device.product_id) {
        return Ok(CameraMode::Senser);
    }
    if classes.is_empty() {
        // A Sony device exposing no interfaces is, in practice, claimed by the
        // OS mass-storage stack; the SCSI-passthrough probe validates it.
        return Ok(CameraMode::MassStorage);
    }
    Ok(CameraMode::Unknown)
}

/// Enumerate all connected Sony devices and detect their modes.
pub fn list_cameras() -> Result<Vec<DetectedCamera>> {
    let mut out = Vec::new();
    for device in list_sony_devices()? {
        let mode = detect_mode(&device)?;
        out.push(DetectedCamera { device, mode });
    }
    Ok(out)
}

/// Locate exactly one connected Sony camera.
///
/// Devices classified as [`CameraMode::Unknown`] are skipped: they are Sony
/// peripherals rather than cameras, and counting them would turn an unrelated
/// headset into [`Error::TooManyDevices`].
///
/// Returns [`Error::NoDevice`] when none are present,
/// [`Error::CameraNotReady`] when the only one is switched off or waiting for
/// its USB mode to be chosen, and [`Error::TooManyDevices`] when more than one
/// is found.
pub fn find_single_camera() -> Result<DetectedCamera> {
    let all = list_cameras()?;
    let waiting = all.iter().any(is_waiting_camera);
    let mut cameras: Vec<DetectedCamera> =
        all.into_iter().filter(|c| c.mode != CameraMode::Unknown).collect();
    match cameras.len() {
        0 if waiting => Err(Error::CameraNotReady),
        0 => Err(Error::NoDevice),
        1 => Ok(cameras.pop().expect("length checked")),
        _ => Err(Error::TooManyDevices),
    }
}

/// Read external-command info (model, lens) and, best-effort, the firmware
/// version over any Sony external-command transport.
fn read_extcmd_report<T: SonyExtCmdTransport>(
    transport: T,
) -> Result<(CameraInfo, Option<LensInfo>, Option<FirmwareVersions>)> {
    let mut camera = SonyExtCmdCamera::new(transport);
    let info = camera.get_camera_info()?;
    let lens = camera.get_lens_info().ok();
    let transport = camera.into_transport();

    // The updater handshake shares the same transport; failures here are
    // non-fatal (some modes do not answer updater commands).
    let mut updater = SonyUpdaterCamera::new(transport);
    let firmware = match updater.init() {
        Ok(()) => updater.get_firmware_version().ok(),
        Err(_) => None,
    };
    Ok((info, lens, firmware))
}

/// Render a lens description if the lens carries a non-zero model id.
fn describe_lens(lens: &LensInfo) -> Option<String> {
    if lens.model == 0 {
        return None;
    }
    Some(format!(
        "Model 0x{:x} (Type 0x{:x}, Region 0x{:x}, Firmware {})",
        lens.model, lens.lens_type, lens.region, lens.version
    ))
}

/// Combine a [`CameraInfo`]/lens/firmware triple into a report.
fn build_report(
    mode: CameraMode,
    info: CameraInfo,
    lens: Option<LensInfo>,
    firmware: Option<FirmwareVersions>,
) -> CameraInfoReport {
    CameraInfoReport {
        mode: mode.id().to_string(),
        model_name: Some(info.model_name),
        model_code: Some(info.model_code),
        serial: Some(info.serial),
        firmware: firmware.map(|f| f.old),
        lens: lens.as_ref().and_then(describe_lens),
    }
}

/// Fails with [`Error::AppInstallMode`] when the MTP camera WPD can see is one
/// left in application-install mode.
///
/// Called once no command camera was found over WPD. Without it the caller
/// falls through to raw endpoints, which `WUDFWpdMtp` refuses, and reports a
/// driver problem whose advice — assign WinUSB — does not help: the camera
/// simply cannot take commands until it is switched off.
#[cfg(windows)]
fn reject_app_install_over_wpd() -> Result<()> {
    match crate::wpd::open_app_install() {
        Ok(Some(_)) => Err(Error::AppInstallMode),
        _ => Ok(()),
    }
}

/// Locate the camera's Windows mass-storage transport, waiting for it where
/// waiting is the right thing to do.
///
/// A camera bound to `USBSTOR` *will* get a drive letter, so a scan that comes
/// up empty means either that the volume stack has not caught up yet — worth
/// waiting for — or that the camera is not offering one, which no fallback can
/// fix. Falling through to raw endpoints would be wrong in both cases:
/// `USBSTOR` refuses them, so the report would be a driver complaint advising
/// WinUSB, and following that advice would remove the driver this path needs.
///
/// A camera bound to WinUSB, by contrast, will never get a drive letter, so the
/// scan must fail immediately and let the caller use raw endpoints.
#[cfg(windows)]
fn find_mass_storage_transport(
    device: &crate::nusb_backend::DiscoveredDevice,
) -> Result<Option<crate::scsi_passthrough::ScsiPassThroughDevice>> {
    if device.os_driver().as_deref() != Some("usbstor") {
        return crate::scsi_passthrough::find_sony_scsi_device();
    }
    match crate::scsi_passthrough::wait_for_sony_scsi_device()? {
        Some(device) => Ok(Some(device)),
        None => Err(Error::NoMassStorageVolume),
    }
}

/// Open a detected camera and read its identifying information.
///
/// * MSC / MTP external-command cameras yield model, serial and (best-effort)
///   firmware and lens data.
/// * Devices already in updater mode yield the firmware version only.
/// * App-install and senser modes are reported without further querying, as
///   they require the (not-yet-ported) proxy / service protocols.
/// * A camera left in app-install mode after an installation enumerates like an
///   MTP camera; it is reported as [`Error::AppInstallMode`].
pub fn read_camera_info(camera: &DetectedCamera) -> Result<CameraInfoReport> {
    match camera.mode {
        CameraMode::MassStorage => {
            // On Windows, prefer the OS mass-storage (USBSTOR) SCSI-passthrough
            // path when the camera is bound to it; fall back to raw WinUSB.
            #[cfg(windows)]
            if let Some(dev) = find_mass_storage_transport(&camera.device)? {
                let (info, lens, firmware) = read_extcmd_report(dev)?;
                return Ok(build_report(camera.mode, info, lens, firmware));
            }
            let (backend, iface) = NusbBackend::open_class(&camera.device, class::MSC)?;
            let mut msc = MscDevice::new(backend, iface.ep_in, iface.ep_out);
            msc.reset();
            let (info, lens, firmware) = read_extcmd_report(msc)?;
            Ok(build_report(camera.mode, info, lens, firmware))
        }
        CameraMode::Mtp => {
            // Prefer WPD on Windows; see `open_extcmd_transport`.
            #[cfg(windows)]
            if let Some(wpd_camera) = crate::wpd::open_command_camera()? {
                let (info, lens, firmware) = read_extcmd_report(wpd_camera.device)?;
                return Ok(build_report(camera.mode, info, lens, firmware));
            }
            #[cfg(windows)]
            reject_app_install_over_wpd()?;
            let (backend, iface) = NusbBackend::open_class(&camera.device, class::PTP)?;
            let mut mtp = MtpDevice::new(backend, iface.ep_in, iface.ep_out);
            let dev_info = mtp.get_device_info()?;
            if is_sony_mtp_app_install_camera(&dev_info) {
                return Err(Error::AppInstallMode);
            }
            if !is_sony_mtp_camera(&dev_info) {
                // Not a recognised Sony command camera; report what MTP told us.
                let mut report =
                    CameraInfoReport { mode: camera.mode.id().to_string(), ..Default::default() };
                report.model_name = Some(dev_info.model);
                report.serial = Some(dev_info.serial_number);
                return Ok(report);
            }
            let (info, lens, firmware) = read_extcmd_report(mtp)?;
            Ok(build_report(camera.mode, info, lens, firmware))
        }
        CameraMode::Updater => {
            #[cfg(windows)]
            if let Some(dev) = find_mass_storage_transport(&camera.device)? {
                let mut updater = SonyUpdaterCamera::new(dev);
                updater.init()?;
                let firmware = updater.get_firmware_version().ok().map(|f| f.old);
                return Ok(CameraInfoReport {
                    mode: camera.mode.id().to_string(),
                    firmware,
                    ..Default::default()
                });
            }
            let (backend, iface) = NusbBackend::open_class(&camera.device, class::MSC)?;
            let mut msc = MscDevice::new(backend, iface.ep_in, iface.ep_out);
            msc.reset();
            let mut updater = SonyUpdaterCamera::new(msc);
            updater.init()?;
            let firmware = updater.get_firmware_version().ok().map(|f| f.old);
            Ok(CameraInfoReport {
                mode: camera.mode.id().to_string(),
                firmware,
                ..Default::default()
            })
        }
        CameraMode::AppInstall | CameraMode::Senser | CameraMode::Unknown => {
            Ok(CameraInfoReport { mode: camera.mode.id().to_string(), ..Default::default() })
        }
    }
}

/// Opens a camera's Sony external-command transport, type-erased.
///
/// Resolves the right transport (mass-storage or MTP) for the camera's mode and
/// returns it boxed, so mode-agnostic orchestration (firmware updater, platform
/// shell) can share one code path. Only modes that speak the external-command
/// protocol are supported; app-install / senser / unknown modes are rejected.
pub fn open_extcmd_transport(camera: &DetectedCamera) -> Result<Box<dyn SonyExtCmdTransport>> {
    match camera.mode {
        CameraMode::MassStorage | CameraMode::Updater => {
            // Prefer the Windows USBSTOR SCSI-passthrough transport (required in
            // updater mode); fall back to the raw WinUSB Bulk-Only transport.
            #[cfg(windows)]
            if let Some(dev) = find_mass_storage_transport(&camera.device)? {
                return Ok(Box::new(dev));
            }
            let (backend, iface) = NusbBackend::open_class(&camera.device, class::MSC)?;
            let mut msc = MscDevice::new(backend, iface.ep_in, iface.ep_out);
            msc.reset();
            Ok(Box::new(msc))
        }
        CameraMode::Mtp => {
            // On Windows the camera is normally bound to the in-box
            // `WUDFWpdMtp` driver, which denies raw endpoint access; reach it
            // through WPD there and keep the bulk path as the fallback for a
            // camera whose driver has been replaced with WinUSB.
            #[cfg(windows)]
            if let Some(camera) = crate::wpd::open_command_camera()? {
                return Ok(Box::new(camera.device));
            }
            #[cfg(windows)]
            reject_app_install_over_wpd()?;
            let (backend, iface) = NusbBackend::open_class(&camera.device, class::PTP)?;
            let mut mtp = MtpDevice::new(backend, iface.ep_in, iface.ep_out);
            // Opening a session is required before ext-cmds flow over MTP.
            if is_sony_mtp_app_install_camera(&mtp.get_device_info()?) {
                return Err(Error::AppInstallMode);
            }
            Ok(Box::new(mtp))
        }
        CameraMode::AppInstall | CameraMode::Senser | CameraMode::Unknown => {
            Err(Error::Protocol("camera mode does not support the external-command transport"))
        }
    }
}

/// Reads a camera's model name over its external-command transport.
///
/// Consumes and returns the transport so the caller can continue using it (for
/// example to run the firmware-updater handshake on the same connection).
pub fn read_model_name(
    transport: Box<dyn SonyExtCmdTransport>,
) -> Result<(String, Box<dyn SonyExtCmdTransport>)> {
    let mut camera = SonyExtCmdCamera::new(transport);
    let info = camera.get_camera_info()?;
    Ok((info.model_name, camera.into_transport()))
}

/// The application-install transport, chosen at open time.
///
/// Two transports can carry the install proxy: the raw bulk-endpoint
/// [`MtpDevice`], and — on Windows — `wpd::WpdDevice`, which reaches a
/// camera still bound to the in-box `WUDFWpdMtp` driver. They share no type,
/// so the choice is erased here rather than leaking into every caller.
pub type AppInstallTransport = Box<dyn AppInstallDevice>;

/// Opens a camera that is in application-installation mode.
///
/// The camera must have re-enumerated as a Sony app-install MTP device (via
/// [`SonyExtCmdCamera::switch_to_app_installer`](crate::sony_extcmd::SonyExtCmdCamera::switch_to_app_installer)).
/// The returned [`AppInstallCamera`] drives the install proxy protocol.
///
/// On Windows the device is claimed by the in-box `WUDFWpdMtp` class driver,
/// which denies raw endpoint access, so `wpd` is tried first there.
/// The raw bulk transport remains the fallback for a camera whose driver has
/// been replaced with WinUSB.
pub fn open_app_install(camera: &DetectedCamera) -> Result<AppInstallCamera<AppInstallTransport>> {
    #[cfg(windows)]
    if let Some(device) = crate::wpd::open_app_install()? {
        return Ok(AppInstallCamera::new(Box::new(device)));
    }
    let (backend, iface) = NusbBackend::open_class(&camera.device, class::PTP)?;
    let mut mtp = MtpDevice::new(backend, iface.ep_in, iface.ep_out);
    let info = mtp.get_device_info()?;
    if !is_sony_mtp_app_install_camera(&info) {
        return Err(Error::Protocol("camera is not in application-installation mode"));
    }
    // The proxy operations run inside an MTP session.
    mtp.open_session(1)?;
    Ok(AppInstallCamera::new(Box::new(mtp)))
}

/// Reports whether a camera is in application-installation mode.
///
/// Descriptor-based mode detection cannot answer this: app-install and ordinary
/// command mode both enumerate as PTP-class devices, so the descriptors look
/// identical and [`CameraMode::AppInstall`] is never returned by
/// classification. The
/// difference only shows in the MTP `GetDeviceInfo` reply, which costs a device
/// open — too much for plain enumeration, but exactly what a caller waiting for
/// the mode to end needs.
///
/// A camera that cannot be opened is reported as *not* in app-install mode: the
/// probe exists to detect the end of that mode, and an unopenable device is
/// indistinguishable from an absent one for that purpose.
pub fn is_app_install(camera: &DetectedCamera) -> bool {
    if camera.mode != CameraMode::Mtp {
        return false;
    }

    // Prefer WPD on Windows; see `open_app_install`.
    #[cfg(windows)]
    if let Ok(Some(_)) = crate::wpd::open_app_install() {
        return true;
    }

    let Ok((backend, iface)) = NusbBackend::open_class(&camera.device, class::PTP) else {
        return false;
    };
    let mut mtp = MtpDevice::new(backend, iface.ep_in, iface.ep_out);
    matches!(mtp.get_device_info(), Ok(info) if is_sony_mtp_app_install_camera(&info))
}

/// Polls for a single camera in application-installation mode.
///
/// After switching a camera to app-install mode it re-enumerates; this retries
/// discovery a few times to give the OS time to rebind the device.
///
/// A camera that is found but cannot be opened reports *that* error rather
/// than [`Error::NoDevice`], because "no device" would send the user looking
/// for a cable fault instead of the real cause.
pub fn wait_for_app_install(
    attempts: usize,
    delay: std::time::Duration,
) -> Result<AppInstallCamera<AppInstallTransport>> {
    let mut last_error = None;
    for _ in 0..attempts {
        std::thread::sleep(delay);

        // Prefer WPD on Windows; see `open_app_install`.
        #[cfg(windows)]
        match crate::wpd::open_app_install() {
            Ok(Some(device)) => return Ok(AppInstallCamera::new(Box::new(device))),
            Ok(None) => {}
            Err(e) => last_error = Some(e),
        }

        let cameras = match list_cameras() {
            Ok(c) => c,
            Err(_) => continue,
        };
        for camera in cameras {
            if camera.mode == CameraMode::Mtp {
                match open_app_install(&camera) {
                    Ok(app) => return Ok(app),
                    Err(e) => last_error = Some(e),
                }
            }
        }
    }
    Err(last_error.unwrap_or(Error::NoDevice))
}

/// Opens the authentication endpoint of a camera that is still in mass-storage
/// mode, so it can be asked to switch into service ("senser") mode.
///
/// Requires a WinUSB/libusb-style driver binding: the OS mass-storage stack does
/// not allow the raw bulk and vendor control transfers used here.
pub fn open_senser_auth_from_mass_storage(
    camera: &DetectedCamera,
) -> Result<SenserAuthDevice<NusbBackend>> {
    let (backend, iface) = NusbBackend::open_class(&camera.device, class::MSC)?;
    Ok(SenserAuthDevice::new(backend, iface.ep_in, iface.ep_out, camera.device.product_id))
}

/// Opens a camera that is already in service ("senser") mode and authenticates.
///
/// The vendor-specific interface exposes two bulk pairs; the first carries
/// senser packets, the second the optional debug terminal. The same endpoints
/// are used for the authentication handshake, so the backend is handed from the
/// auth device to the resulting transport.
pub fn open_senser(camera: &DetectedCamera) -> Result<SenserDevice<NusbBackend>> {
    if camera.mode != CameraMode::Senser {
        return Err(Error::Protocol("camera is not in service mode"));
    }
    let (backend, eps) =
        NusbBackend::open_class_all_endpoints(&camera.device, class::VENDOR_SPECIFIC)?;
    let ep_in = *eps.ep_in.first().ok_or(Error::NoDevice)?;
    let ep_out = *eps.ep_out.first().ok_or(Error::NoDevice)?;

    let mut auth = SenserAuthDevice::new(backend, ep_in, ep_out, camera.device.product_id);
    auth.start();
    auth.authenticate()?;

    let endpoints = SenserEndpoints {
        ep_in,
        ep_out,
        terminal_in: eps.ep_in.get(1).copied(),
        terminal_out: eps.ep_out.get(1).copied(),
    };
    Ok(SenserDevice::new(auth.into_inner(), endpoints, camera.device.product_id))
}

/// Asks a camera that is already in service mode to leave it, without opening a
/// session first.
///
/// [`open_senser`] cannot be used for this. It runs the challenge/response
/// handshake, and a camera whose senser subsystem was started by a session that
/// then died answers a second handshake with a stalled endpoint — so the one
/// situation this is for is exactly the one where authenticating fails. The
/// stop request itself is a vendor control transfer that needs no session at
/// all, which is why it can be sent on its own.
///
/// The transfer result is ignored for the same reason as in
/// [`SenserDevice::stop`]: the camera drops off the bus while it handles the
/// request, so a failed transfer is the expected outcome rather than a sign
/// that the request did not arrive.
pub fn leave_service_mode(camera: &DetectedCamera) -> Result<()> {
    if camera.mode != CameraMode::Senser {
        return Err(Error::Protocol("camera is not in service mode"));
    }
    let (mut backend, _) =
        NusbBackend::open_class_all_endpoints(&camera.device, class::VENDOR_SPECIFIC)?;
    let (request, value, index) = crate::senser::VENDOR_REQUEST_STOP_SENSER;
    let _ = backend.vendor_request_out(request, value, index, &[]);
    Ok(())
}

/// Polls for a single camera in service ("senser") mode after a mode switch.
///
/// Returns the detected device, or [`Error::NoDevice`] if it never appears.
pub fn wait_for_senser(attempts: usize, delay: std::time::Duration) -> Result<DetectedCamera> {
    wait_for_mode(CameraMode::Senser, attempts, delay)
}

/// Polls for a single camera in `mode` after a mode switch.
///
/// Returns the detected device, or [`Error::NoDevice`] if it never appears.
pub fn wait_for_mode(
    mode: CameraMode,
    attempts: usize,
    delay: std::time::Duration,
) -> Result<DetectedCamera> {
    wait_for_mode_or_choice(mode, attempts, delay, &mut || {})
}

/// [`wait_for_mode`], telling the caller once when the camera shows its USB
/// mode choice and waiting up to `WAITING_PATIENCE` polls longer for it.
///
/// Ends with [`Error::CameraNotReady`] if the camera is still waiting then.
pub fn wait_for_mode_or_choice(
    mode: CameraMode,
    attempts: usize,
    delay: std::time::Duration,
    on_choice: &mut dyn FnMut(),
) -> Result<DetectedCamera> {
    let mut tries = 0;
    let mut patience = WAITING_PATIENCE;
    let mut waiting = false;
    while tries < attempts {
        std::thread::sleep(delay);
        let Ok(cameras) = list_cameras() else {
            tries += 1;
            continue;
        };
        let now_waiting = cameras.iter().any(is_waiting_camera);
        if let Some(camera) = cameras.into_iter().find(|c| c.mode == mode) {
            return Ok(camera);
        }
        if now_waiting && !waiting {
            on_choice();
        }
        waiting = now_waiting;
        if waiting && patience > 0 {
            patience -= 1;
        } else {
            tries += 1;
        }
    }
    Err(if waiting { Error::CameraNotReady } else { Error::NoDevice })
}

/// Asks an MTP-mode camera to reconnect as a mass-storage device.
///
/// Resolves the same two MTP transports as [`open_extcmd_transport`] — WPD on
/// Windows, raw bulk endpoints elsewhere — and sends Sony's reconnect request
/// over whichever one opens. The camera drops off the bus immediately, so the
/// caller must wait for it to re-enumerate (see [`wait_for_mode`]).
///
/// This is a one-way trip: no command switches a mass-storage camera back to
/// MTP. The mode selected in the camera's own USB menu is untouched, so a power
/// cycle or replug restores it.
pub fn switch_to_mass_storage(camera: &DetectedCamera) -> Result<()> {
    if camera.mode != CameraMode::Mtp {
        return Err(Error::Protocol("camera is not in MTP mode"));
    }

    #[cfg(windows)]
    if let Some(mut wpd) = crate::wpd::open_command_camera()? {
        return crate::mtp::switch_to_msc(&mut wpd.device);
    }
    #[cfg(windows)]
    reject_app_install_over_wpd()?;

    let (backend, iface) = NusbBackend::open_class(&camera.device, class::PTP)?;
    let mut mtp = MtpDevice::new(backend, iface.ep_in, iface.ep_out);
    if is_sony_mtp_app_install_camera(&mtp.get_device_info()?) {
        return Err(Error::AppInstallMode);
    }
    crate::mtp::switch_to_msc(&mut mtp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_camera_without_a_data_mode_is_recognised_by_its_name() {
        assert!(is_waiting_camera_name(Some("USB Bus-powered Device")));
        assert!(is_waiting_camera_name(Some("USB Charger")));
        // The data modes and other Sony devices keep their own names.
        assert!(!is_waiting_camera_name(Some("ILCE-7M3")));
        assert!(!is_waiting_camera_name(Some("Wireless Controller")));
        assert!(!is_waiting_camera_name(None));
    }

    #[test]
    fn interface_classes_decide_when_present() {
        // A camera, whatever driver happens to own it.
        assert!(is_camera_candidate_from(&[class::MSC], Some("winusb")));
        assert!(is_camera_candidate_from(&[class::PTP], None));
        assert!(is_camera_candidate_from(&[class::VENDOR_SPECIFIC], None));
        // A Sony HID peripheral: same vendor id, not a camera.
        assert!(!is_camera_candidate_from(&[3], Some("hidusb")));
        // Composite audio/HID headset.
        assert!(!is_camera_candidate_from(&[1, 3], None));
    }

    #[test]
    fn driver_decides_when_windows_hides_the_interfaces() {
        // The real regression: a HID device reports no interfaces once it is
        // bound to its class driver, which used to be read as "mass storage".
        assert!(!is_camera_candidate_from(&[], Some("hidusb")));
        // A mass-storage camera is equally featureless, but its driver differs.
        assert!(is_camera_candidate_from(&[], Some("USBSTOR".to_ascii_lowercase().as_str())));
        assert!(is_camera_candidate_from(&[], Some("usbccgp")));
        assert!(is_camera_candidate_from(&[], Some("winusb")));
        // Non-Windows platforms report no driver but always list interfaces,
        // so an empty list there must not exclude the device.
        assert!(is_camera_candidate_from(&[], None));
    }
}
