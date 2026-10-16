//! Windows Portable Devices (WPD) transport for PTP/MTP cameras.
//!
//! Port of `pmca/usb/driver/windows/wpd.py`.
//!
//! # Why this exists
//!
//! On Windows a Sony camera in MTP or application-installation mode is claimed
//! by the in-box `WUDFWpdMtp` class driver. That driver owns the USB interface,
//! so the raw bulk-endpoint transport in [`crate::mtp`] cannot open it — the
//! reference workflow therefore required the user to replace the driver with
//! WinUSB using a third-party tool (Zadig). Replacing an in-box driver is
//! invasive, needs administrator rights, and breaks normal photo import until
//! it is undone.
//!
//! WPD is the supported way to talk to such a device. It is *not* a
//! [`UsbBackend`](crate::UsbBackend): it exposes no endpoints and does its own
//! PTP framing. What it does expose is the MTP extension command set, which
//! offers exactly the four PTP command shapes this crate needs. So this module
//! sits at the [`PtpCommands`] seam rather than under it, and everything built
//! on that trait (Sony external commands, the application-install proxy) works
//! unchanged.
//!
//! # Lifetime rules
//!
//! COM is apartment-scoped and reference-counted. Two rules are load-bearing
//! here, and violating either one crashes the process at exit:
//!
//! 1. Every interface pointer must be released *before* `CoUninitialize` runs.
//!    [`WpdDevice`] therefore declares its interfaces before its apartment
//!    guard, as Rust drops struct fields in declaration order.
//! 2. The apartment must be initialised on the thread that uses the
//!    interfaces, and [`WpdDevice`] must not move between threads. It is
//!    deliberately neither [`Send`] nor [`Sync`] (the raw COM pointers make it
//!    `!Send` automatically); this is what keeps rule 1 enforceable.

#![allow(unsafe_code)]

use windows::Win32::Devices::PortableDevices::{
    IPortableDevice, IPortableDeviceManager, IPortableDevicePropVariantCollection,
    IPortableDeviceValues, PortableDevice, PortableDeviceManager,
    PortableDevicePropVariantCollection, PortableDeviceValues,
    WPD_COMMAND_MTP_EXT_END_DATA_TRANSFER, WPD_COMMAND_MTP_EXT_EXECUTE_COMMAND_WITH_DATA_TO_READ,
    WPD_COMMAND_MTP_EXT_EXECUTE_COMMAND_WITH_DATA_TO_WRITE,
    WPD_COMMAND_MTP_EXT_EXECUTE_COMMAND_WITHOUT_DATA_PHASE, WPD_COMMAND_MTP_EXT_READ_DATA,
    WPD_COMMAND_MTP_EXT_WRITE_DATA, WPD_PROPERTY_COMMON_COMMAND_CATEGORY,
    WPD_PROPERTY_COMMON_COMMAND_ID, WPD_PROPERTY_COMMON_HRESULT,
    WPD_PROPERTY_MTP_EXT_OPERATION_CODE, WPD_PROPERTY_MTP_EXT_OPERATION_PARAMS,
    WPD_PROPERTY_MTP_EXT_RESPONSE_CODE, WPD_PROPERTY_MTP_EXT_TRANSFER_CONTEXT,
    WPD_PROPERTY_MTP_EXT_TRANSFER_DATA, WPD_PROPERTY_MTP_EXT_TRANSFER_NUM_BYTES_TO_READ,
    WPD_PROPERTY_MTP_EXT_TRANSFER_NUM_BYTES_TO_WRITE,
    WPD_PROPERTY_MTP_EXT_TRANSFER_TOTAL_DATA_SIZE,
};
use windows::Win32::Foundation::{PROPERTYKEY, RPC_E_CHANGED_MODE};
use windows::Win32::System::Com::StructuredStorage::PROPVARIANT;
use windows::Win32::System::Com::{
    CLSCTX_INPROC_SERVER, COINIT_MULTITHREADED, CoCreateInstance, CoInitializeEx, CoTaskMemFree,
    CoUninitialize,
};
use windows::Win32::System::Variant::VT_UI4;
use windows::core::PCWSTR;

use crate::app_install::AppInstallDevice;
use crate::command::SonyExtCmdTransport;
use crate::mtp::PtpCommands;
use crate::sony::{MtpDeviceInfo, is_sony_mtp_app_install_camera, is_sony_mtp_camera};
use crate::{Error, Result};

/// Sony's USB vendor id.
const VENDOR_SONY: u16 = 0x054c;

fn com_err(context: &'static str, e: windows::core::Error) -> Error {
    Error::Transfer(format!("{context}: {e}"))
}

/// A COM apartment held for as long as WPD interfaces are alive.
///
/// The multi-threaded apartment is used deliberately: WPD's proxy does not
/// require a message pump, and an STA would deadlock on any thread that is not
/// running one.
struct Apartment {
    /// Whether *this* guard incremented the apartment's reference count.
    ///
    /// `CoInitializeEx` answers `RPC_E_CHANGED_MODE` when the thread already
    /// belongs to an apartment of a different type. That is not fatal — the
    /// existing apartment is used — but it does *not* take a reference, so
    /// calling `CoUninitialize` in that case would tear down an apartment this
    /// guard never owned.
    owned: bool,
}

impl Apartment {
    fn enter() -> Result<Self> {
        // SAFETY: plain COM initialisation; the matching CoUninitialize runs in
        // `Drop`, and `Apartment` is not `Send`, so both calls stay on one
        // thread.
        let hr = unsafe { CoInitializeEx(None, COINIT_MULTITHREADED) };
        if hr == RPC_E_CHANGED_MODE {
            return Ok(Self { owned: false });
        }
        if hr.is_err() {
            return Err(Error::Transfer(format!("CoInitializeEx failed: {hr:?}")));
        }
        Ok(Self { owned: true })
    }
}

impl Drop for Apartment {
    fn drop(&mut self) {
        if self.owned {
            // SAFETY: balances the CoInitializeEx in `enter`, on the same thread.
            unsafe { CoUninitialize() };
        }
    }
}

/// Extracts `(vendor, product)` from a WPD device id.
///
/// WPD device ids embed the USB ids, e.g.
/// `\\?\usb#vid_054c&pid_08b7#0123456789ab#{6ac27878-...}`. Devices that are
/// not USB-backed (MTP-over-IP, for instance) carry no such fields and yield
/// `None`.
pub fn parse_device_id(id: &str) -> Option<(u16, u16)> {
    fn field(lower: &str, name: &str) -> Option<u16> {
        let at = lower.find(name)?;
        let start = at + name.len();
        let digits = lower.get(start..start + 4)?;
        if !digits.bytes().all(|b| b.is_ascii_hexdigit()) {
            return None;
        }
        u16::from_str_radix(digits, 16).ok()
    }
    let lower = id.to_ascii_lowercase();
    Some((field(&lower, "vid_")?, field(&lower, "pid_")?))
}

/// A device found by the WPD device manager.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WpdDeviceId {
    /// The opaque WPD device id, passed back to `IPortableDevice::Open`.
    pub id: String,
    /// USB vendor id, when the id encodes one.
    pub vendor: Option<u16>,
    /// USB product id, when the id encodes one.
    pub product: Option<u16>,
}

/// Lists every device the WPD subsystem currently exposes.
pub fn list_devices() -> Result<Vec<WpdDeviceId>> {
    let _apartment = Apartment::enter()?;
    // SAFETY: the two-call GetDevices idiom — first with a null buffer to learn
    // the count, then with a buffer of exactly that many slots. Each returned
    // string is allocated by COM and freed below.
    unsafe {
        let manager: IPortableDeviceManager =
            CoCreateInstance(&PortableDeviceManager, None, CLSCTX_INPROC_SERVER)
                .map_err(|e| com_err("PortableDeviceManager", e))?;

        let mut count: u32 = 0;
        manager
            .GetDevices(std::ptr::null_mut(), &mut count)
            .map_err(|e| com_err("GetDevices (count)", e))?;
        if count == 0 {
            return Ok(Vec::new());
        }

        let mut raw = vec![windows::core::PWSTR::null(); count as usize];
        manager.GetDevices(raw.as_mut_ptr(), &mut count).map_err(|e| com_err("GetDevices", e))?;

        let mut out = Vec::with_capacity(count as usize);
        for entry in raw.iter().take(count as usize) {
            if entry.is_null() {
                continue;
            }
            let id = entry.to_string().unwrap_or_default();
            CoTaskMemFree(Some(entry.0 as *const _));
            let (vendor, product) = match parse_device_id(&id) {
                Some((v, p)) => (Some(v), Some(p)),
                None => (None, None),
            };
            out.push(WpdDeviceId { id, vendor, product });
        }
        Ok(out)
    }
}

/// Lists the Sony devices WPD currently exposes.
pub fn list_sony_devices() -> Result<Vec<WpdDeviceId>> {
    Ok(list_devices()?.into_iter().filter(|d| d.vendor == Some(VENDOR_SONY)).collect())
}

/// A PTP/MTP device driven through the WPD MTP extension commands.
///
/// Field order is deliberate: `device` must drop before `_apartment`.
pub struct WpdDevice {
    device: IPortableDevice,
    /// Held purely for its `Drop`; see the module documentation.
    _apartment: Apartment,
}

impl WpdDevice {
    /// Opens the WPD device with the given id.
    pub fn open(id: &str) -> Result<Self> {
        let _apartment = Apartment::enter()?;
        // SAFETY: standard WPD open sequence. `wide` outlives the Open call,
        // which copies the string.
        let device = unsafe {
            let device: IPortableDevice =
                CoCreateInstance(&PortableDevice, None, CLSCTX_INPROC_SERVER)
                    .map_err(|e| com_err("PortableDevice", e))?;
            let client_info: IPortableDeviceValues =
                CoCreateInstance(&PortableDeviceValues, None, CLSCTX_INPROC_SERVER)
                    .map_err(|e| com_err("PortableDeviceValues", e))?;
            let wide: Vec<u16> = id.encode_utf16().chain(std::iter::once(0)).collect();
            device
                .Open(PCWSTR(wide.as_ptr()), &client_info)
                .map_err(|e| com_err("IPortableDevice::Open", e))?;
            device
        };
        Ok(Self { device, _apartment })
    }

    /// Builds the parameter set common to every MTP extension command.
    ///
    /// # Safety
    /// Must run inside the apartment this device was opened in.
    unsafe fn command_values(
        &self,
        command: &PROPERTYKEY,
        context: Option<&PCWSTR>,
    ) -> Result<IPortableDeviceValues> {
        // SAFETY: every statement below is a COM call on interfaces created in
        // the caller's apartment; the caller upholds that precondition.
        unsafe {
            let params: IPortableDeviceValues =
                CoCreateInstance(&PortableDeviceValues, None, CLSCTX_INPROC_SERVER)
                    .map_err(|e| com_err("PortableDeviceValues", e))?;
            params
                .SetGuidValue(&WPD_PROPERTY_COMMON_COMMAND_CATEGORY, &command.fmtid)
                .map_err(|e| com_err("SetGuidValue(category)", e))?;
            params
                .SetUnsignedIntegerValue(&WPD_PROPERTY_COMMON_COMMAND_ID, command.pid)
                .map_err(|e| com_err("SetUnsignedIntegerValue(id)", e))?;
            if let Some(ctx) = context {
                params
                    .SetStringValue(&WPD_PROPERTY_MTP_EXT_TRANSFER_CONTEXT, *ctx)
                    .map_err(|e| com_err("SetStringValue(context)", e))?;
            }
            Ok(params)
        }
    }

    /// Builds an initial command: opcode plus its parameter words.
    ///
    /// # Safety
    /// Must run inside the apartment this device was opened in.
    unsafe fn initial_command(
        &self,
        command: &PROPERTYKEY,
        code: u16,
        args: &[u32],
    ) -> Result<IPortableDeviceValues> {
        // SAFETY: COM calls in the caller's apartment, plus one raw variant
        // write that is justified inline below.
        unsafe {
            let params = self.command_values(command, None)?;
            params
                .SetUnsignedIntegerValue(&WPD_PROPERTY_MTP_EXT_OPERATION_CODE, u32::from(code))
                .map_err(|e| com_err("SetUnsignedIntegerValue(opcode)", e))?;

            let collection: IPortableDevicePropVariantCollection =
                CoCreateInstance(&PortableDevicePropVariantCollection, None, CLSCTX_INPROC_SERVER)
                    .map_err(|e| com_err("PortableDevicePropVariantCollection", e))?;
            for arg in args {
                // PTP parameters are 32-bit words, so each one goes in as VT_UI4
                // -- the same variant the reference implementation builds by
                // hand.
                //
                // The `windows` crate dropped its safe `PROPVARIANT` wrapper
                // after 0.58, leaving only the raw C union, so the variant is
                // filled in field by field. `value` is zeroed by `Default` and
                // VT_UI4 is exactly the arm being written, so tag and payload
                // stay consistent and no previously-set arm can leak. VT_UI4
                // stores its payload inline, so nothing needs `PropVariantClear`.
                let mut value = PROPVARIANT::default();
                let inner = &mut *value.Anonymous.Anonymous;
                inner.vt = VT_UI4;
                inner.Anonymous.ulVal = *arg;
                collection.Add(&value).map_err(|e| com_err("PropVariantCollection::Add", e))?;
            }
            params
                .SetIPortableDevicePropVariantCollectionValue(
                    &WPD_PROPERTY_MTP_EXT_OPERATION_PARAMS,
                    &collection,
                )
                .map_err(|e| com_err("SetIPortableDevicePropVariantCollectionValue", e))?;
            Ok(params)
        }
    }

    /// Issues one command and checks the embedded HRESULT.
    ///
    /// WPD reports two independent failures: `SendCommand` itself can fail, and
    /// the command can fail while `SendCommand` succeeds — the latter is
    /// carried in `WPD_PROPERTY_COMMON_HRESULT`. Both must be checked.
    ///
    /// # Safety
    /// Must run inside the apartment this device was opened in.
    unsafe fn send(&self, params: &IPortableDeviceValues) -> Result<IPortableDeviceValues> {
        // SAFETY: COM calls on this device's interfaces, in the caller's
        // apartment.
        unsafe {
            let result =
                self.device.SendCommand(0, params).map_err(|e| com_err("SendCommand", e))?;
            let hr = result
                .GetErrorValue(&WPD_PROPERTY_COMMON_HRESULT)
                .map_err(|e| com_err("GetErrorValue", e))?;
            if hr.is_err() {
                return Err(Error::Transfer(format!("WPD MTP command failed: {hr:?}")));
            }
            Ok(result)
        }
    }

    /// Ends a data transfer and returns the PTP response code.
    ///
    /// # Safety
    /// Must run inside the apartment this device was opened in.
    unsafe fn end_transfer(&self, context: &PCWSTR) -> Result<u16> {
        // SAFETY: COM calls in the caller's apartment.
        unsafe {
            let params =
                self.command_values(&WPD_COMMAND_MTP_EXT_END_DATA_TRANSFER, Some(context))?;
            let result = self.send(&params)?;
            response_code(&result)
        }
    }
}

/// Reads the PTP response code out of a command result.
///
/// # Safety
/// Must run inside the apartment the result was produced in.
unsafe fn response_code(result: &IPortableDeviceValues) -> Result<u16> {
    // SAFETY: COM call in the apartment the result was produced in.
    let code = unsafe { result.GetUnsignedIntegerValue(&WPD_PROPERTY_MTP_EXT_RESPONSE_CODE) }
        .map_err(|e| com_err("GetUnsignedIntegerValue(response)", e))?;
    // PTP response codes are 16-bit; a wider value would mean the driver broke
    // its own contract, and silently truncating it would hide that.
    u16::try_from(code).map_err(|_| Error::Protocol("WPD returned an out-of-range PTP response"))
}

/// Reads the transfer context handle out of a command result.
///
/// The returned [`PCWSTR`] borrows COM-allocated memory that must be freed with
/// [`CoTaskMemFree`] once the caller is done with it.
///
/// # Safety
/// Must run inside the apartment the result was produced in.
unsafe fn transfer_context(result: &IPortableDeviceValues) -> Result<windows::core::PWSTR> {
    // SAFETY: COM call in the apartment the result was produced in.
    unsafe { result.GetStringValue(&WPD_PROPERTY_MTP_EXT_TRANSFER_CONTEXT) }
        .map_err(|e| com_err("GetStringValue(context)", e))
}

/// Frees a COM-allocated wide string.
///
/// # Safety
/// `s` must be a COM-allocated string that is not used afterwards.
unsafe fn free_pwstr(s: windows::core::PWSTR) {
    if !s.is_null() {
        // SAFETY: the caller guarantees `s` is COM-allocated and unused after
        // this point.
        unsafe { CoTaskMemFree(Some(s.0 as *const _)) };
    }
}

impl PtpCommands for WpdDevice {
    fn send_command(&mut self, code: u16, args: &[u32]) -> Result<u16> {
        // SAFETY: `self` can only exist inside its apartment (see the module
        // docs); every COM allocation below is freed on all paths.
        unsafe {
            let params = self.initial_command(
                &WPD_COMMAND_MTP_EXT_EXECUTE_COMMAND_WITHOUT_DATA_PHASE,
                code,
                args,
            )?;
            let result = self.send(&params)?;
            response_code(&result)
        }
    }

    fn send_write_command(&mut self, code: u16, args: &[u32], data: &[u8]) -> Result<u16> {
        // SAFETY: see `send_command`.
        unsafe {
            let params = self.initial_command(
                &WPD_COMMAND_MTP_EXT_EXECUTE_COMMAND_WITH_DATA_TO_WRITE,
                code,
                args,
            )?;
            params
                .SetUnsignedLargeIntegerValue(
                    &WPD_PROPERTY_MTP_EXT_TRANSFER_TOTAL_DATA_SIZE,
                    data.len() as u64,
                )
                .map_err(|e| com_err("SetUnsignedLargeIntegerValue(total)", e))?;
            let result = self.send(&params)?;
            let context = transfer_context(&result)?;

            let outcome = (|| -> Result<u16> {
                let ctx = PCWSTR(context.0);
                let write = self.command_values(&WPD_COMMAND_MTP_EXT_WRITE_DATA, Some(&ctx))?;
                write
                    .SetUnsignedLargeIntegerValue(
                        &WPD_PROPERTY_MTP_EXT_TRANSFER_NUM_BYTES_TO_WRITE,
                        data.len() as u64,
                    )
                    .map_err(|e| com_err("SetUnsignedLargeIntegerValue(to write)", e))?;
                write
                    .SetBufferValue(&WPD_PROPERTY_MTP_EXT_TRANSFER_DATA, data)
                    .map_err(|e| com_err("SetBufferValue", e))?;
                self.send(&write)?;
                self.end_transfer(&ctx)
            })();

            free_pwstr(context);
            outcome
        }
    }

    fn send_read_command(&mut self, code: u16, args: &[u32]) -> Result<(u16, Vec<u8>)> {
        // SAFETY: see `send_command`. `GetBufferValue` hands back a
        // COM-allocated buffer plus its length; it is copied and then freed.
        unsafe {
            let params = self.initial_command(
                &WPD_COMMAND_MTP_EXT_EXECUTE_COMMAND_WITH_DATA_TO_READ,
                code,
                args,
            )?;
            let result = self.send(&params)?;
            let context = transfer_context(&result)?;

            let outcome = (|| -> Result<(u16, Vec<u8>)> {
                let ctx = PCWSTR(context.0);
                let length = result
                    .GetUnsignedIntegerValue(&WPD_PROPERTY_MTP_EXT_TRANSFER_TOTAL_DATA_SIZE)
                    .map_err(|e| com_err("GetUnsignedIntegerValue(total)", e))?;

                // A zero-length data phase still needs ending, but asking the
                // driver to read zero bytes is not a valid transfer.
                if length == 0 {
                    return Ok((self.end_transfer(&ctx)?, Vec::new()));
                }

                let zeros = vec![0u8; length as usize];
                let read = self.command_values(&WPD_COMMAND_MTP_EXT_READ_DATA, Some(&ctx))?;
                read.SetUnsignedLargeIntegerValue(
                    &WPD_PROPERTY_MTP_EXT_TRANSFER_NUM_BYTES_TO_READ,
                    u64::from(length),
                )
                .map_err(|e| com_err("SetUnsignedLargeIntegerValue(to read)", e))?;
                read.SetBufferValue(&WPD_PROPERTY_MTP_EXT_TRANSFER_DATA, &zeros)
                    .map_err(|e| com_err("SetBufferValue", e))?;
                let read_result = self.send(&read)?;

                let mut buffer: *mut u8 = std::ptr::null_mut();
                let mut got: u32 = 0;
                read_result
                    .GetBufferValue(&WPD_PROPERTY_MTP_EXT_TRANSFER_DATA, &mut buffer, &mut got)
                    .map_err(|e| com_err("GetBufferValue", e))?;
                let data = if buffer.is_null() {
                    Vec::new()
                } else {
                    let copy = std::slice::from_raw_parts(buffer, got as usize).to_vec();
                    CoTaskMemFree(Some(buffer as *const _));
                    copy
                };

                Ok((self.end_transfer(&ctx)?, data))
            })();

            free_pwstr(context);
            outcome
        }
    }

    fn send_read_command_opt(&mut self, code: u16, args: &[u32]) -> Result<(u16, Vec<u8>)> {
        // Unlike the bulk transport there is no container to inspect: WPD
        // performs the data phase itself and simply reports a total size of
        // zero when the device sends none. `send_read_command` already handles
        // that, so the "optional data phase" case needs no special path.
        self.send_read_command(code, args)
    }
}

impl SonyExtCmdTransport for WpdDevice {
    fn send_sony_ext_command(
        &mut self,
        cmd: u32,
        data: &[u8],
        buffer_size: usize,
    ) -> Result<Vec<u8>> {
        crate::command::send_ptp_ext_command(self, cmd, data, buffer_size)
    }
}

impl AppInstallDevice for WpdDevice {
    fn send_message(&mut self, msg_type: u16, data: &[u8]) -> Result<()> {
        crate::app_install::proxy_send_message(self, msg_type, data)
    }

    fn receive_message(&mut self) -> Result<Option<(u16, Vec<u8>)>> {
        crate::app_install::proxy_receive_message(self)
    }
}

impl WpdDevice {
    /// Fetches and parses the device info descriptor.
    pub fn get_device_info(&mut self) -> Result<MtpDeviceInfo> {
        let (code, data) = self.send_read_command(crate::mtp::ptp::OC_GET_DEVICE_INFO, &[])?;
        if code != crate::mtp::ptp::RC_OK {
            return Err(Error::Protocol("MTP GetDeviceInfo failed"));
        }
        crate::mtp::parse_device_info(&data)
    }

    /// Opens an MTP session (accepting an already-open session).
    pub fn open_session(&mut self, id: u32) -> Result<()> {
        let code = self.send_command(crate::mtp::ptp::OC_OPEN_SESSION, &[id])?;
        if code == crate::mtp::ptp::RC_OK || code == crate::mtp::ptp::RC_SESSION_ALREADY_OPENED {
            Ok(())
        } else {
            Err(Error::Protocol("MTP OpenSession failed"))
        }
    }
}

/// How an opened WPD camera identified itself.
pub struct WpdCamera {
    /// The opened transport.
    pub device: WpdDevice,
    /// The device info descriptor that was read to classify it.
    pub info: MtpDeviceInfo,
}

/// Finds and opens the first Sony camera reachable through WPD whose device
/// info satisfies `accept`.
///
/// Returns `Ok(None)` when no such device is present, so callers can fall back
/// to the raw bulk transport (used when the driver *has* been replaced with
/// WinUSB). An MTP session is opened before returning, as the Sony vendor
/// operations require one.
///
/// Errors from individual candidates are remembered but only reported when no
/// candidate worked out — one unopenable stray Sony device (a headset, say)
/// must not mask the camera next to it.
pub fn open_camera(accept: fn(&MtpDeviceInfo) -> bool) -> Result<Option<WpdCamera>> {
    let mut last_error = None;
    for candidate in list_sony_devices()? {
        let mut device = match WpdDevice::open(&candidate.id) {
            Ok(d) => d,
            Err(e) => {
                last_error = Some(e);
                continue;
            }
        };
        let info = match device.get_device_info() {
            Ok(i) => i,
            Err(e) => {
                last_error = Some(e);
                continue;
            }
        };
        if !accept(&info) {
            continue;
        }
        device.open_session(1)?;
        return Ok(Some(WpdCamera { device, info }));
    }
    match last_error {
        Some(e) => Err(e),
        None => Ok(None),
    }
}

/// Finds and opens a Sony camera that is in application-installation mode.
pub fn open_app_install() -> Result<Option<WpdDevice>> {
    Ok(open_camera(is_sony_mtp_app_install_camera)?.map(|c| c.device))
}

/// Finds and opens a Sony camera that is in MTP command mode.
pub fn open_command_camera() -> Result<Option<WpdCamera>> {
    open_camera(is_sony_mtp_camera)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_a_usb_backed_device_id() {
        let id = r"\\?\usb#vid_054c&pid_08b7#0123456789ab#{6ac27878-a6fa-4155-ba85-f98f491d4f33}";
        assert_eq!(parse_device_id(id), Some((0x054c, 0x08b7)));
    }

    #[test]
    fn parsing_is_case_insensitive() {
        let id = r"\\?\USB#VID_054C&PID_06A8#SERIAL#{GUID}";
        assert_eq!(parse_device_id(id), Some((0x054c, 0x06a8)));
    }

    #[test]
    fn parses_ids_using_backslash_separators() {
        assert_eq!(parse_device_id(r"USB\VID_054C&PID_1234\0123456789AB"), Some((0x054c, 0x1234)));
    }

    #[test]
    fn rejects_ids_without_usb_fields() {
        // MTP-over-IP and similar transports have no USB ids to report.
        assert_eq!(parse_device_id(r"\\?\mtp#ip_192.168.0.2#{guid}"), None);
    }

    #[test]
    fn rejects_truncated_and_non_hex_fields() {
        assert_eq!(parse_device_id("usb#vid_05"), None);
        assert_eq!(parse_device_id("usb#vid_zzzz&pid_08b7"), None);
        assert_eq!(parse_device_id("usb#vid_054c&pid_zzzz"), None);
    }

    #[test]
    fn requires_both_fields() {
        assert_eq!(parse_device_id("usb#vid_054c#no-product"), None);
        assert_eq!(parse_device_id("usb#pid_08b7#no-vendor"), None);
    }
}
