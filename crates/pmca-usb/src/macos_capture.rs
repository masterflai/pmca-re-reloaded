//! macOS: take a USB device away from the kernel drivers that hold it.
//!
//! macOS cannot detach a kernel driver from a single interface. What it offers
//! instead is to *capture* the whole device: re-enumerating it with the capture
//! option ends every kernel driver bound to its interfaces and keeps them away
//! while this process holds the capture. That needs root (or an Apple
//! entitlement this program does not have). Only the mass-storage path asks for
//! it, because the system's disk driver is the one holder no ordinary process
//! can displace.
//!
//! Both directions re-enumerate: the device leaves the IORegistry and comes
//! back as a new entry, which has to be found and opened afresh. A plain
//! re-enumeration gives the device back; the kernel then matches its drivers
//! again and the volume reappears. The IOKit sequence follows what libusb does
//! for `libusb_detach_kernel_driver` on macOS.

use std::ffi::c_void;
use std::ptr;

use core_foundation_sys::base::{SInt32, kCFAllocatorSystemDefault};
use core_foundation_sys::uuid::{
    CFUUIDBytes, CFUUIDGetConstantUUIDWithBytes, CFUUIDGetUUIDBytes, CFUUIDRef,
};
use io_kit_sys::ret::{IOReturn, kIOReturnNoResources, kIOReturnSuccess};
use io_kit_sys::types::io_service_t;
use io_kit_sys::{IOObjectRelease, IORegistryEntryIDMatching, IOServiceGetMatchingService};

/// `kUSBReEnumerateCaptureDeviceMask` from `IOUSBLib.h`.
const CAPTURE_DEVICE: u32 = 1 << 30;

/// `kIOMainPortDefault`, which is defined as 0.
const MAIN_PORT_DEFAULT: u32 = 0;

type Fn0 = Option<unsafe extern "C" fn(this: *mut c_void) -> IOReturn>;
type Unused = *const c_void;

/// The leading part of `IOUSBDeviceStruct650`, up to `USBDeviceReEnumerate`.
///
/// Only the slots used here are typed; the order is the header's, and the
/// struct is only ever read through a pointer IOKit hands out, so the missing
/// tail does not matter.
#[repr(C)]
struct DeviceVtbl {
    _reserved: Unused,
    _query_interface: Unused,
    _add_ref: Unused,
    release: Option<unsafe extern "C" fn(this: *mut c_void) -> u32>,
    _async_and_port: [Unused; 4],
    open: Fn0,
    close: Fn0,
    _class_to_interface_iterator: [Unused; 19],
    open_seize: Fn0,
    _request_to_serial_index: [Unused; 7],
    re_enumerate: Option<unsafe extern "C" fn(this: *mut c_void, options: u32) -> IOReturn>,
}

/// The leading part of `IOCFPlugInInterfaceStruct`.
#[repr(C)]
struct PluginVtbl {
    _reserved: Unused,
    query_interface: Option<
        unsafe extern "C" fn(this: *mut c_void, iid: CFUUIDBytes, out: *mut *mut c_void) -> SInt32,
    >,
    _add_ref: Unused,
    release: Option<unsafe extern "C" fn(this: *mut c_void) -> u32>,
}

#[link(name = "IOKit", kind = "framework")]
unsafe extern "C" {
    fn IOCreatePlugInInterfaceForService(
        service: io_service_t,
        plugin_type: CFUUIDRef,
        interface_type: CFUUIDRef,
        out: *mut *mut *mut PluginVtbl,
        score: *mut SInt32,
    ) -> IOReturn;
}

unsafe extern "C" {
    fn geteuid() -> u32;
}

/// Whether this process runs as root, which capturing requires.
pub(crate) fn is_root() -> bool {
    // SAFETY: geteuid has no preconditions and cannot fail.
    unsafe { geteuid() == 0 }
}

fn uuid(bytes: [u8; 16]) -> CFUUIDRef {
    let [b0, b1, b2, b3, b4, b5, b6, b7, b8, b9, b10, b11, b12, b13, b14, b15] = bytes;
    // SAFETY: returns a constant, never-released UUID object.
    unsafe {
        CFUUIDGetConstantUUIDWithBytes(
            kCFAllocatorSystemDefault,
            b0,
            b1,
            b2,
            b3,
            b4,
            b5,
            b6,
            b7,
            b8,
            b9,
            b10,
            b11,
            b12,
            b13,
            b14,
            b15,
        )
    }
}

/// `kIOUSBDeviceUserClientTypeID`.
fn device_user_client() -> CFUUIDRef {
    uuid([
        0x9d, 0xc7, 0xb7, 0x80, 0x9e, 0xc0, 0x11, 0xd4, 0xa5, 0x4f, 0x00, 0x0a, 0x27, 0x05, 0x28,
        0x61,
    ])
}

/// `kIOCFPlugInInterfaceID`.
fn plugin_interface() -> CFUUIDRef {
    uuid([
        0xc2, 0x44, 0xe8, 0x58, 0x10, 0x9c, 0x11, 0xd4, 0x91, 0xd4, 0x00, 0x50, 0xe4, 0xc6, 0x42,
        0x6f,
    ])
}

/// `kIOUSBDeviceInterfaceID650`.
fn device_interface_650() -> CFUUIDRef {
    uuid([
        0x4a, 0xac, 0x1b, 0x2e, 0x24, 0xc2, 0x47, 0x6a, 0x96, 0x4d, 0x91, 0x33, 0x35, 0x34, 0xf2,
        0xcc,
    ])
}

/// Re-enumerate the device with IORegistry entry `registry_id` captured.
///
/// The entry goes away; the device returns under a new one, which the caller
/// has to find and then hold in a [`Capture`].
pub(crate) fn capture(registry_id: u64) -> Result<(), String> {
    re_enumerate(registry_id, CAPTURE_DEVICE).map_err(|rc| format!("capture refused (0x{rc:08x})"))
}

/// A device that came back captured; dropping it gives the device back.
pub(crate) struct Capture {
    registry_id: u64,
}

impl Capture {
    /// Hold the captured device with IORegistry entry `registry_id`.
    pub(crate) fn held(registry_id: u64) -> Self {
        Self { registry_id }
    }
}

impl Drop for Capture {
    fn drop(&mut self) {
        // May fail harmlessly: a camera that switched mode is no longer this entry.
        let _ = re_enumerate(self.registry_id, 0);
    }
}

/// Open the device seized and re-enumerate it with `options`.
fn re_enumerate(registry_id: u64, options: u32) -> Result<(), IOReturn> {
    let device = open_device_interface(registry_id).map_err(|_| kIOReturnNoResources)?;
    // SAFETY: `device` is a live IOUSBDeviceInterface650 until the release below.
    unsafe {
        let opened = call0(device, (**device).open_seize) == kIOReturnSuccess
            || call0(device, (**device).open) == kIOReturnSuccess;
        let rc = match (**device).re_enumerate {
            Some(f) if opened => f(device.cast(), options),
            _ => kIOReturnNoResources,
        };
        if opened {
            call0(device, (**device).close);
        }
        if let Some(release) = (**device).release {
            release(device.cast());
        }
        if rc == kIOReturnSuccess { Ok(()) } else { Err(rc) }
    }
}

/// # Safety
///
/// `device` must be a live device interface.
unsafe fn call0(device: *mut *mut DeviceVtbl, f: Fn0) -> IOReturn {
    match f {
        // SAFETY: guaranteed by the caller.
        Some(f) => unsafe { f(device.cast()) },
        None => -1,
    }
}

/// The IOUSBDeviceInterface650 for the device with IORegistry entry `registry_id`.
fn open_device_interface(registry_id: u64) -> Result<*mut *mut DeviceVtbl, String> {
    // SAFETY: the matching dictionary is consumed by IOServiceGetMatchingService;
    // the service and the plug-in are released on every path.
    unsafe {
        let service = IOServiceGetMatchingService(
            MAIN_PORT_DEFAULT,
            IORegistryEntryIDMatching(registry_id).cast_const(),
        );
        if service == 0 {
            return Err("device not found in the IORegistry".into());
        }

        let mut plugin: *mut *mut PluginVtbl = ptr::null_mut();
        let mut score: SInt32 = 0;
        let mut rc = kIOReturnNoResources;
        // A freshly enumerated device can answer "no resources" for a moment.
        for _ in 0..5 {
            rc = IOCreatePlugInInterfaceForService(
                service,
                device_user_client(),
                plugin_interface(),
                &mut plugin,
                &mut score,
            );
            if rc != kIOReturnNoResources {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
        IOObjectRelease(service);
        if rc != kIOReturnSuccess || plugin.is_null() {
            return Err(format!("no IOKit plug-in for the device (0x{rc:08x})"));
        }

        let mut device: *mut c_void = ptr::null_mut();
        let query = (**plugin).query_interface;
        let hr = match query {
            Some(f) => f(plugin.cast(), CFUUIDGetUUIDBytes(device_interface_650()), &mut device),
            None => -1,
        };
        if let Some(release) = (**plugin).release {
            release(plugin.cast());
        }
        if hr != 0 || device.is_null() {
            return Err(format!("no USB device interface (0x{hr:08x})"));
        }
        Ok(device.cast())
    }
}
