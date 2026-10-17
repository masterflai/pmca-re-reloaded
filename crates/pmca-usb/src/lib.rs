//! # pmca-usb
//!
//! USB transport abstraction for PMCA-RE RELOADED.
//!
//! This crate is the hardware-integration layer that sits on top of the
//! hardware-independent [`pmca_core`]. It is a Rust port of the driver layer
//! in `pmca/usb/driver` plus the Sony device detection in `pmca/usb/sony.py`.
//!
//! ## Design
//!
//! * [`UsbBackend`] is the minimal transport trait: bulk `read`/`write` on an
//!   endpoint plus `reset`. Everything else is built on top of it, so the same
//!   protocol code runs against real hardware and against the in-memory
//!   [`mock::MockBackend`] used for tests.
//! * [`msc`] implements the USB Mass-Storage Bulk-Only Transport (CBW/CSW),
//!   the framing Sony cameras use in mass-storage mode.
//! * [`sony`] holds the vendor/product constants and the detection helpers.
//! * [`app_install`] ports the MTP application-install proxy: it tunnels a
//!   small message protocol through four vendor PTP operations and relays the
//!   camera's TLS traffic to the local install server during app installation.
//! * The real hardware backend ([`nusb_backend`], behind the default
//!   `hardware` feature) wraps `nusb`.
//! * On Windows, [`scsi_passthrough`] provides an alternative mass-storage
//!   transport that talks to the OS USBSTOR stack via
//!   `IOCTL_SCSI_PASS_THROUGH_DIRECT`. It is required for cameras in
//!   firmware-updater mode, whose bulk endpoints reject raw WinUSB Bulk-Only
//!   transfers.
//! * Also on Windows, [`wpd`] provides an alternative MTP transport built on
//!   Windows Portable Devices. Cameras in MTP and application-installation
//!   mode are claimed by the in-box `WUDFWpdMtp` driver, which denies raw
//!   endpoint access; WPD is the supported way to reach them, and using it
//!   removes the need to replace that driver with WinUSB by hand.
//! * [`winusb_install`], Windows only, binds WinUSB to the camera for the two
//!   modes where no such alternative exists — mass storage on the way into
//!   service mode, and service mode itself. It replaces the manual Zadig step
//!   the toolkit used to ask for, and undoes itself again afterwards.
//!
//!   These three are the crate's only `unsafe` code (Windows FFI), confined to
//!   those modules; the rest of the crate is `#![deny(unsafe_code)]`. The one
//!   exception is a read-only service-manager query in [`host_services`].
//!
//! ## Testability without a camera
//!
//! All framing/detection logic is covered by unit tests driven by
//! [`mock::MockBackend`], which replays scripted endpoint responses. No
//! physical device or USB system library is required for `cargo test
//! --no-default-features`.

#![deny(unsafe_code)]
#![warn(missing_docs)]

pub mod app_install;
pub mod command;
#[cfg(feature = "hardware")]
pub mod discovery;
pub mod host_services;
pub mod mock;
pub mod msc;
pub mod mtp;
pub mod network;
#[cfg(feature = "hardware")]
pub mod nusb_backend;
pub mod platform_shell;
#[cfg(all(windows, feature = "hardware"))]
pub mod scsi_passthrough;
pub mod senser;
pub mod senser_camera;
pub mod sony;
pub mod sony_extcmd;
pub mod sony_updater;
#[cfg(all(windows, feature = "hardware"))]
pub mod winusb_install;
// Same API, empty behaviour, so the cleanup calls need no `cfg` at their call
// sites. See the module for why "nothing to do" is the right answer elsewhere.
#[cfg(not(all(windows, feature = "hardware")))]
#[path = "winusb_install/unsupported.rs"]
pub mod winusb_install;
#[cfg(all(windows, feature = "hardware"))]
pub mod wpd;

/// USB device class codes relevant to Sony cameras.
pub mod class {
    /// Still Image / PTP-MTP.
    pub const PTP: u8 = 6;
    /// Mass Storage.
    pub const MSC: u8 = 8;
    /// Vendor-specific (service / senser mode).
    pub const VENDOR_SPECIFIC: u8 = 255;
}

/// Transport-level errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// A bulk transfer failed at the backend level.
    #[error("usb transfer failed: {0}")]
    Transfer(String),
    /// The device is bound to a driver that cannot carry raw USB transfers.
    ///
    /// Distinct from [`Error::Transfer`] because the remedy is completely
    /// different: no amount of re-plugging or swapping cables helps, the OS
    /// simply has to hand the device to a driver that allows raw access.
    #[error("the device is claimed by a driver that does not allow raw USB access: {0}")]
    Driver(String),
    /// The OS refused to open the device for lack of permission.
    ///
    /// On Linux the raw USB node is root-only by default, so opening it as an
    /// ordinary user fails with `EACCES`. The remedy is a one-off udev rule,
    /// not a different cable, which is why this is kept apart from
    /// [`Error::Transfer`] whose message sends the user cable-hunting.
    #[error("permission to open the USB device was denied: {0}")]
    Permission(String),
    /// Another holder has claimed the interface.
    ///
    /// On Linux a userspace holder (a desktop media monitor, another program)
    /// keeps the interface, so claiming it fails with `EBUSY`. Kept apart from
    /// [`Error::Transfer`] whose message would send the user cable-hunting; the
    /// kernel-driver case is already resolved by claiming with a detach.
    #[error("the USB interface is held by another program: {0}")]
    Busy(String),
    /// A mass-storage camera is bound to `USBSTOR` but has no drive letter.
    ///
    /// Deliberately not folded into [`Error::Driver`], whose advice is to bind
    /// WinUSB: here that advice is actively harmful. `USBSTOR` is the *right*
    /// driver for this mode — the SCSI pass-through path needs it — and
    /// replacing it would break the very transport that is missing. The camera
    /// simply has not presented a volume, which usually means it is still
    /// waking up, is asking on its display which USB mode to use, or has no
    /// card inserted.
    #[error("the camera is bound to the mass-storage driver but exposes no drive")]
    NoMassStorageVolume,
    /// The camera is still in application-install mode.
    ///
    /// Some models stay in that mode after an
    /// installation until they are switched off. The mode offers only the
    /// install proxy and basic PTP object access — no external commands, no
    /// reconnect request — so nothing can be sent over USB to end it, and every
    /// other operation fails. Kept apart from [`Error::Driver`], whose advice
    /// (assign WinUSB) would send the user off in the wrong direction.
    #[error("the camera is still in application-install mode")]
    AppInstallMode,
    /// A bulk endpoint returned a STALL / halt condition.
    ///
    /// For mass-storage transfers this is an expected part of the Bulk-Only
    /// protocol (the device signals a short/omitted data phase); callers clear
    /// the halt and read the command status wrapper to obtain the real result.
    #[error("endpoint STALL condition")]
    Stall,
    /// The device returned a malformed or unexpected response.
    #[error("protocol error: {0}")]
    Protocol(&'static str),
    /// A mass-storage command failed with a non-zero CSW / SCSI status byte
    /// and no accompanying sense data.
    ///
    /// This is the transport-level failure: the device rejected the command
    /// without saying why. See [`Error::MscSense`] for the case where the
    /// device did explain itself.
    #[error("mass-storage command failed (SCSI status {status})")]
    MscStatus {
        /// The CSW / SCSI status byte returned by the device.
        status: u8,
    },
    /// A mass-storage command failed and the device returned sense data.
    ///
    /// Kept separate from [`Error::MscStatus`] because the two numbers come
    /// from different fields of the SCSI protocol and are not comparable; a
    /// single shared variant made a sense key indistinguishable from a status
    /// byte in the error message.
    #[error("mass-storage command failed (sense key {key:#04x}, ASC {asc:#04x}/{ascq:#04x})")]
    MscSense {
        /// SCSI sense key.
        key: u8,
        /// Additional sense code.
        asc: u8,
        /// Additional sense code qualifier.
        ascq: u8,
    },
    /// An MTP command failed with a non-OK PTP response code.
    ///
    /// The MTP counterpart of [`Error::MscStatus`]. The response code is the
    /// only thing the camera said about the failure, so it is carried here
    /// rather than folded into a generic protocol error — without it, every
    /// distinct rejection looks alike from the outside.
    #[error("MTP command failed in the {phase} phase (response code {code:#06x})")]
    PtpResponse {
        /// Which half of the exchange failed: `"write"` or `"read"`.
        phase: &'static str,
        /// The PTP response code returned by the device.
        code: u16,
    },
    /// The camera rejected the command as one it does not implement.
    ///
    /// Sony cameras answer an unknown external command with the SCSI
    /// "illegal request" sense key rather than a dedicated status, so a model
    /// that simply lacks a feature is indistinguishable from a real fault
    /// unless that key is given its own meaning here. In MTP mode the same
    /// refusal arrives as one of the PTP "not supported" response codes.
    #[error("the camera does not support this command")]
    Unsupported,
    /// The firmware updater reported a non-OK per-chunk status.
    ///
    /// Carries the human-readable meaning of the first offending status code
    /// (e.g. "invalid model", "AC adapter required"), matching the reference
    /// `_statusToStr` mapping.
    #[error("firmware updater rejected the operation: {0}")]
    UpdaterStatus(String),
    /// A service-mode (senser) operation failed or was rejected.
    #[error("senser error: {0}")]
    Senser(String),
    /// No matching device was found.
    #[error("no device found")]
    NoDevice,
    /// A camera is connected but in no USB data mode: switched off and
    /// charging, or waiting on its display for the USB mode to be chosen.
    #[error("the camera is connected but not in a USB data mode")]
    CameraNotReady,
    /// More than one candidate device was found.
    #[error("too many devices found")]
    TooManyDevices,
    /// The service-mode key material the user supplies is not loaded.
    #[error("the service-mode key material is not available")]
    MissingKeys,
}

impl Error {
    /// A stable identifier for this kind of failure.
    ///
    /// Front-ends map the code onto a translated message. It is deliberately
    /// decoupled from the [`Display`](std::fmt::Display) text so that rewording
    /// an English message cannot silently break a translation.
    pub fn code(&self) -> &'static str {
        match self {
            Error::Transfer(_) => "usb-transfer",
            Error::Driver(_) => "usb-driver",
            Error::Permission(_) => "usb-permission",
            Error::Busy(_) => "usb-busy",
            Error::NoMassStorageVolume => "usb-no-mass-storage-volume",
            Error::AppInstallMode => "usb-app-install-mode",
            Error::Stall => "usb-stall",
            Error::Protocol(_) => "usb-protocol",
            Error::MscStatus { .. } => "usb-msc-status",
            Error::MscSense { .. } => "usb-msc-sense",
            Error::PtpResponse { .. } => "usb-ptp-response",
            Error::Unsupported => "usb-unsupported",
            Error::UpdaterStatus(_) => "usb-updater-status",
            Error::Senser(_) => "usb-senser",
            Error::NoDevice => "usb-no-device",
            Error::CameraNotReady => "usb-camera-not-ready",
            Error::TooManyDevices => "usb-too-many-devices",
            Error::MissingKeys => "missing-keys",
        }
    }

    /// The variable part of the message, if this variant carries one.
    ///
    /// A translated message can only cover the fixed wording; the detail is
    /// appended verbatim so that diagnostics survive translation.
    pub fn detail(&self) -> Option<String> {
        match self {
            Error::Transfer(s)
            | Error::UpdaterStatus(s)
            | Error::Senser(s)
            | Error::Driver(s)
            | Error::Permission(s)
            | Error::Busy(s) => Some(s.clone()),
            Error::Protocol(s) => Some((*s).to_string()),
            Error::MscStatus { status } => Some(status.to_string()),
            Error::MscSense { key, asc, ascq } => {
                Some(format!("{key:#04x}/{asc:#04x}/{ascq:#04x}"))
            }
            Error::PtpResponse { phase, code } => Some(format!("{phase} {code:#06x}")),
            Error::Stall
            | Error::Unsupported
            | Error::NoDevice
            | Error::CameraNotReady
            | Error::NoMassStorageVolume
            | Error::AppInstallMode
            | Error::TooManyDevices
            | Error::MissingKeys => None,
        }
    }
}

/// Convenience alias.
pub type Result<T> = std::result::Result<T, Error>;

/// Standard USB endpoint direction bit (set = device-to-host).
pub const ENDPOINT_IN: u8 = 0x80;

/// Minimal bulk USB transport used by all higher-level protocol code.
///
/// Implementors move raw bytes on bulk endpoints; the semantics of those bytes
/// (CBW/CSW, PTP containers, …) live in the protocol modules above.
pub trait UsbBackend {
    /// Read up to `length` bytes from endpoint `ep` (an IN endpoint address).
    fn read(&mut self, ep: u8, length: usize) -> Result<Vec<u8>>;

    /// Write `data` to endpoint `ep` (an OUT endpoint address).
    fn write(&mut self, ep: u8, data: &[u8]) -> Result<()>;

    /// Clear a bulk endpoint's halt / STALL condition. Default: no-op.
    fn clear_halt(&mut self, _ep: u8) -> Result<()> {
        Ok(())
    }

    /// Reset / re-initialize the device state. Default: no-op.
    fn reset(&mut self) -> Result<()> {
        Ok(())
    }

    /// Issue a vendor-specific host-to-device control transfer.
    ///
    /// Used by the service-mode (senser) handshake to start and stop the
    /// camera's senser subsystem. Default: unsupported, since most transports
    /// (mass-storage, MTP) never need it.
    fn vendor_request_out(
        &mut self,
        _request: u8,
        _value: u16,
        _index: u16,
        _data: &[u8],
    ) -> Result<()> {
        Err(Error::Protocol("vendor control transfers are not supported by this backend"))
    }
}
