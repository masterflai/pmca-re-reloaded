//! Real hardware backend built on [`nusb`] (pure-Rust, cross-platform USB).
//!
//! This module is compiled only with the `hardware` feature. It provides:
//! * [`list_sony_devices`] — enumerate connected Sony USB devices with their
//!   descriptor class, so callers can pick MSC / PTP / vendor-specific modes.
//! * [`NusbBackend`] — an [`UsbBackend`] that performs blocking bulk transfers
//!   through `nusb`'s endpoint queues.
//!
//! `nusb` itself is pure Rust, so this backend contains no `unsafe` code. The
//! crate's `unsafe` lives in the three Windows-only FFI modules listed in the
//! crate documentation.

use nusb::MaybeFuture;
use nusb::descriptors::TransferType;
use nusb::transfer::{Buffer, Bulk, Direction, In, Out};
use std::collections::HashMap;
use std::time::Duration;

use crate::{Error, Result, UsbBackend};

/// Default per-transfer timeout, matching the reference tool's 5-second limit.
///
/// Without a timeout a stalled or unresponsive endpoint would block the
/// `block_on` call forever; the reference libusb backend used a 5000 ms default.
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(5);

/// A discovered Sony USB device, before it is opened.
#[derive(Debug, Clone)]
pub struct DiscoveredDevice {
    /// The underlying `nusb` descriptor.
    pub info: nusb::DeviceInfo,
    /// USB vendor id.
    pub vendor_id: u16,
    /// USB product id.
    pub product_id: u16,
    /// Device-level `bDeviceClass` (`0` = defined at interface level).
    pub class: u8,
}

/// A claimed bulk interface with its IN/OUT endpoint addresses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BulkInterface {
    /// `bInterfaceNumber` of the claimed interface.
    pub interface_number: u8,
    /// Bulk IN endpoint address (device-to-host, high bit set).
    pub ep_in: u8,
    /// Bulk OUT endpoint address (host-to-device).
    pub ep_out: u8,
}

/// All bulk endpoints of an interface, in descriptor order.
///
/// The vendor-specific service ("senser") interface exposes two bulk pairs: the
/// first carries senser packets, the second a debug terminal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BulkEndpoints {
    /// `bInterfaceNumber` of the claimed interface.
    pub interface_number: u8,
    /// Bulk IN endpoint addresses, in descriptor order.
    pub ep_in: Vec<u8>,
    /// Bulk OUT endpoint addresses, in descriptor order.
    pub ep_out: Vec<u8>,
}

/// Enumerates connected Sony USB devices (vendor `0x054c`).
///
/// Errors from the OS enumeration are surfaced as [`Error::Transfer`].
pub fn list_sony_devices() -> Result<Vec<DiscoveredDevice>> {
    let devices = nusb::list_devices().wait().map_err(|e| Error::Transfer(e.to_string()))?;
    Ok(devices
        .filter(|d| d.vendor_id() == crate::sony::SONY_ID_VENDOR)
        .map(|d| DiscoveredDevice {
            vendor_id: d.vendor_id(),
            product_id: d.product_id(),
            class: d.class(),
            info: d,
        })
        .collect())
}

/// Find, inside an active configuration, the first interface whose class equals
/// `class` and that exposes both a bulk IN and a bulk OUT endpoint.
fn find_bulk_interface(
    config: &nusb::descriptors::ConfigurationDescriptor<'_>,
    class: u8,
) -> Option<BulkInterface> {
    for group in config.interfaces() {
        for alt in group.alt_settings() {
            if alt.class() != class {
                continue;
            }
            let mut ep_in = None;
            let mut ep_out = None;
            for ep in alt.endpoints() {
                if ep.transfer_type() != TransferType::Bulk {
                    continue;
                }
                match ep.direction() {
                    Direction::In => {
                        ep_in.get_or_insert(ep.address());
                    }
                    Direction::Out => {
                        ep_out.get_or_insert(ep.address());
                    }
                }
            }
            if let (Some(ep_in), Some(ep_out)) = (ep_in, ep_out) {
                return Some(BulkInterface {
                    interface_number: alt.interface_number(),
                    ep_in,
                    ep_out,
                });
            }
        }
    }
    None
}

/// Collect *all* bulk endpoints of the first interface matching `class`.
fn find_all_bulk_endpoints(
    config: &nusb::descriptors::ConfigurationDescriptor<'_>,
    class: u8,
) -> Option<BulkEndpoints> {
    for group in config.interfaces() {
        for alt in group.alt_settings() {
            if alt.class() != class {
                continue;
            }
            let mut ep_in = Vec::new();
            let mut ep_out = Vec::new();
            for ep in alt.endpoints() {
                if ep.transfer_type() != TransferType::Bulk {
                    continue;
                }
                match ep.direction() {
                    Direction::In => ep_in.push(ep.address()),
                    Direction::Out => ep_out.push(ep.address()),
                }
            }
            if !ep_in.is_empty() && !ep_out.is_empty() {
                return Some(BulkEndpoints {
                    interface_number: alt.interface_number(),
                    ep_in,
                    ep_out,
                });
            }
        }
    }
    None
}

impl DiscoveredDevice {
    /// List the distinct interface classes advertised by the active
    /// configuration. Sony cameras expose their mode (MSC / PTP /
    /// vendor-specific) at the interface level, so this is used for detection
    /// even though the device-level [`class`](Self::class) is usually `0`.
    ///
    /// Reads cached descriptors; no bulk IO is performed.
    pub fn interface_classes(&self) -> Result<Vec<u8>> {
        let device = self.info.open().wait().map_err(|e| Error::Transfer(e.to_string()))?;
        let config = device.active_configuration().map_err(|e| Error::Transfer(e.to_string()))?;
        let mut classes = Vec::new();
        for group in config.interfaces() {
            for alt in group.alt_settings() {
                if !classes.contains(&alt.class()) {
                    classes.push(alt.class());
                }
            }
        }
        Ok(classes)
    }

    /// The interface classes the OS has cached, without opening the device.
    ///
    /// This is the fallback for [`Self::interface_classes`], which needs an
    /// open handle and therefore fails for devices owned by a class driver.
    /// Windows only fills this in for composite devices bound to `usbccgp`, so
    /// an empty result there means "unknown", not "no interfaces"; on Linux and
    /// macOS it is always populated.
    pub fn cached_interface_classes(&self) -> Vec<u8> {
        let mut classes = Vec::new();
        for iface in self.info.interfaces() {
            if !classes.contains(&iface.class()) {
                classes.push(iface.class());
            }
        }
        classes
    }

    /// The OS driver bound to the device, lower-cased, where the platform
    /// reports one.
    ///
    /// This is the only thing that separates a mass-storage camera from an
    /// unrelated Sony peripheral on Windows: both are claimed by a class
    /// driver, and both then look equally featureless.
    pub fn os_driver(&self) -> Option<String> {
        #[cfg(target_os = "windows")]
        {
            self.info.driver().map(|d| d.to_ascii_lowercase())
        }
        #[cfg(not(target_os = "windows"))]
        {
            None
        }
    }
}

/// Recognises an open/claim failure caused by the bound OS driver.
///
/// The wording is the backend's, not ours, and it has already changed once:
/// `nusb` 0.1 refused with `Device driver is "WUDFWpdMtp", not WinUSB`, while
/// 0.2 says `incompatible driver is installed for this device` (or
/// `… for this interface`) and names no driver at all. Matching only the older
/// phrasing turned a driver problem into [`Error::Transfer`], which tells the
/// user to go looking for a bad cable. Every known phrasing is therefore
/// matched here, and the check is a pure function so it can be tested without
/// hardware.
fn is_driver_refusal(message: &str) -> bool {
    message.contains("WinUSB")
        || message.contains("USBSTOR")
        || message.to_ascii_lowercase().contains("incompatible driver")
}

/// Classifies a device-open failure.
///
/// Windows binds cameras in mass-storage mode to `USBSTOR`, and cameras in MTP
/// mode to `WUDFWpdMtp`; both refuse the raw transfers the service and updater
/// modes need. That is not a transfer fault, and reporting it as one sends the
/// user looking for a bad cable, so it gets its own variant.
///
/// The detail carries the name of the driver actually bound rather than the
/// backend's sentence: it identifies the culprit, and being a Windows driver
/// name it reads the same in every language — the backend's sentence would
/// appear in English underneath a translated message.
fn map_open_error<E: std::fmt::Display>(device: &DiscoveredDevice, e: E) -> Error {
    let message = e.to_string();
    if is_driver_refusal(&message) {
        Error::Driver(device.os_driver().unwrap_or(message))
    } else {
        Error::Transfer(message)
    }
}

/// A blocking bulk transport over an opened `nusb` interface.
pub struct NusbBackend {
    interface: nusb::Interface,
    interface_number: u8,
    timeout: Duration,
    /// Opened IN endpoints, keyed by address.
    ///
    /// `nusb` 0.2 hands out exclusive [`nusb::Endpoint`] handles that own a
    /// transfer queue, so an address may only be opened once. They are cached
    /// here and reused for every transfer on that address.
    ep_in: HashMap<u8, nusb::Endpoint<Bulk, In>>,
    /// Opened OUT endpoints, keyed by address.
    ep_out: HashMap<u8, nusb::Endpoint<Bulk, Out>>,
}

impl NusbBackend {
    fn new(interface: nusb::Interface, interface_number: u8) -> Self {
        Self {
            interface,
            interface_number,
            timeout: DEFAULT_TIMEOUT,
            ep_in: HashMap::new(),
            ep_out: HashMap::new(),
        }
    }

    /// Opens `device` and claims interface number `interface_number`.
    pub fn open(device: &DiscoveredDevice, interface_number: u8) -> Result<Self> {
        let handle = device.info.open().wait().map_err(|e| map_open_error(device, e))?;
        let interface = handle
            .claim_interface(interface_number)
            .wait()
            .map_err(|e| map_open_error(device, e))?;
        Ok(Self::new(interface, interface_number))
    }

    /// Opens `device`, locates the bulk interface for the given USB `class`
    /// (e.g. [`crate::class::MSC`] or [`crate::class::PTP`]), claims it, and
    /// returns the ready backend together with its endpoint addresses.
    pub fn open_class(device: &DiscoveredDevice, class: u8) -> Result<(Self, BulkInterface)> {
        let handle = device.info.open().wait().map_err(|e| map_open_error(device, e))?;
        let config = handle.active_configuration().map_err(|e| Error::Transfer(e.to_string()))?;
        let iface = find_bulk_interface(&config, class).ok_or(Error::NoDevice)?;
        let interface = handle
            .claim_interface(iface.interface_number)
            .wait()
            .map_err(|e| map_open_error(device, e))?;
        Ok((Self::new(interface, iface.interface_number), iface))
    }

    /// Like [`open_class`](Self::open_class), but reports *every* bulk endpoint
    /// of the matched interface. Needed for the vendor-specific service
    /// interface, which carries a second bulk pair for the debug terminal.
    pub fn open_class_all_endpoints(
        device: &DiscoveredDevice,
        class: u8,
    ) -> Result<(Self, BulkEndpoints)> {
        let handle = device.info.open().wait().map_err(|e| map_open_error(device, e))?;
        let config = handle.active_configuration().map_err(|e| Error::Transfer(e.to_string()))?;
        let eps = find_all_bulk_endpoints(&config, class).ok_or(Error::NoDevice)?;
        let interface = handle
            .claim_interface(eps.interface_number)
            .wait()
            .map_err(|e| map_open_error(device, e))?;
        Ok((Self::new(interface, eps.interface_number), eps))
    }

    /// Wraps an already-claimed interface (useful for advanced callers/tests).
    pub fn from_interface(interface: nusb::Interface) -> Self {
        Self::new(interface, 0)
    }

    /// Overrides the per-transfer timeout.
    pub fn set_timeout(&mut self, timeout: Duration) {
        self.timeout = timeout;
    }

    /// The cached IN endpoint for `ep`, opening it on first use.
    fn endpoint_in(&mut self, ep: u8) -> Result<&mut nusb::Endpoint<Bulk, In>> {
        if !self.ep_in.contains_key(&ep) {
            let endpoint = self
                .interface
                .endpoint::<Bulk, In>(ep)
                .map_err(|e| Error::Transfer(e.to_string()))?;
            self.ep_in.insert(ep, endpoint);
        }
        Ok(self.ep_in.get_mut(&ep).expect("endpoint was just inserted"))
    }

    /// The cached OUT endpoint for `ep`, opening it on first use.
    fn endpoint_out(&mut self, ep: u8) -> Result<&mut nusb::Endpoint<Bulk, Out>> {
        if !self.ep_out.contains_key(&ep) {
            let endpoint = self
                .interface
                .endpoint::<Bulk, Out>(ep)
                .map_err(|e| Error::Transfer(e.to_string()))?;
            self.ep_out.insert(ep, endpoint);
        }
        Ok(self.ep_out.get_mut(&ep).expect("endpoint was just inserted"))
    }
}

impl UsbBackend for NusbBackend {
    fn read(&mut self, ep: u8, length: usize) -> Result<Vec<u8>> {
        if ep & crate::ENDPOINT_IN == 0 {
            return Err(Error::Protocol("read requested on a non-IN endpoint"));
        }
        if length == 0 {
            return Ok(Vec::new());
        }
        let timeout = self.timeout;
        let endpoint = self.endpoint_in(ep)?;
        // `nusb` 0.2 requires the requested length to be a non-zero multiple of
        // the maximum packet size. The protocols above ask for exact byte
        // counts, so the request is rounded up; the device ends the transfer
        // early with a short packet and the surplus is never delivered.
        let packet = endpoint.max_packet_size().max(1);
        let requested = length.div_ceil(packet) * packet;
        let completion = endpoint.transfer_blocking(Buffer::new(requested), timeout);
        let data = completion.into_result().map_err(map_transfer_error)?;
        if data.len() > length {
            return Err(Error::Protocol("device returned more data than requested"));
        }
        Ok(data.into_vec())
    }

    fn write(&mut self, ep: u8, data: &[u8]) -> Result<()> {
        if ep & crate::ENDPOINT_IN != 0 {
            return Err(Error::Protocol("write requested on a non-OUT endpoint"));
        }
        let timeout = self.timeout;
        let endpoint = self.endpoint_out(ep)?;
        let completion = endpoint.transfer_blocking(Buffer::from(data.to_vec()), timeout);
        completion.into_result().map(|_| ()).map_err(map_transfer_error)
    }

    fn clear_halt(&mut self, ep: u8) -> Result<()> {
        // In `nusb` 0.2 the halt is cleared on the endpoint, so the endpoint has
        // to be open. It always is by the time a stall is recovered from, but
        // opening it here keeps the call valid on its own.
        if ep & crate::ENDPOINT_IN != 0 {
            self.endpoint_in(ep)?.clear_halt().wait().map_err(|e| Error::Transfer(e.to_string()))
        } else {
            self.endpoint_out(ep)?.clear_halt().wait().map_err(|e| Error::Transfer(e.to_string()))
        }
    }

    fn reset(&mut self) -> Result<()> {
        // Bulk-Only Mass Storage Reset (USB MSC BBB, class request 0xFF):
        // re-synchronises the device's command/data/status state machine. This
        // matters on Windows/WinUSB after a previous session left a transfer
        // half-finished (e.g. a timed-out data phase), which otherwise makes the
        // next session's first CBW stall. Best-effort: not all stacks accept it.
        self.interface
            .control_out(
                nusb::transfer::ControlOut {
                    control_type: nusb::transfer::ControlType::Class,
                    recipient: nusb::transfer::Recipient::Interface,
                    request: 0xFF,
                    value: 0,
                    index: self.interface_number as u16,
                    data: &[],
                },
                self.timeout,
            )
            .wait()
            .map(|_| ())
            .map_err(map_transfer_error)
    }

    fn vendor_request_out(
        &mut self,
        request: u8,
        value: u16,
        index: u16,
        data: &[u8],
    ) -> Result<()> {
        // Vendor host-to-device control transfer, as used by the service-mode
        // (senser) start/stop requests. The recipient must be `Other`
        // (bmRequestType 0x43): Sony's firmware ignores the request when it is
        // addressed to the device, and the following bulk write then runs into
        // the still-active mass-storage endpoint and stalls.
        self.interface
            .control_out(
                nusb::transfer::ControlOut {
                    control_type: nusb::transfer::ControlType::Vendor,
                    recipient: nusb::transfer::Recipient::Other,
                    request,
                    value,
                    index,
                    data,
                },
                self.timeout,
            )
            .wait()
            .map(|_| ())
            .map_err(map_transfer_error)
    }
}

/// Translate an `nusb` transfer error, distinguishing a STALL (which the
/// mass-storage layer recovers from) from other failures.
fn map_transfer_error(e: nusb::transfer::TransferError) -> Error {
    match e {
        nusb::transfer::TransferError::Stall => Error::Stall,
        other => Error::Transfer(other.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::is_driver_refusal;

    #[test]
    fn a_driver_refusal_is_recognised_in_every_known_wording() {
        // nusb 0.1
        assert!(is_driver_refusal(r#"Device driver is "WUDFWpdMtp", not WinUSB"#));
        assert!(is_driver_refusal("device is bound to USBSTOR"));
        // nusb 0.2 — names no driver, so neither of the above matches
        assert!(is_driver_refusal("incompatible driver is installed for this device"));
        assert!(is_driver_refusal("incompatible driver is installed for this interface"));
    }

    #[test]
    fn a_genuine_transfer_fault_is_not_mistaken_for_one() {
        assert!(!is_driver_refusal("device disconnected"));
        assert!(!is_driver_refusal("timed out"));
        assert!(!is_driver_refusal("Access denied (insufficient permissions)"));
    }
}
