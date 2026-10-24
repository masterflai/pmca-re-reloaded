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
//! Capturing does not re-enumerate: the drivers are ended in place, and the
//! device stays under its IORegistry entry. The capture lasts while the device
//! handle that asked for it stays open, so [`Capture`] keeps that handle,
//! seized, until it is dropped. A plain re-enumeration then gives the device
//! back; the kernel matches its drivers again and the volume reappears. The
//! IOKit sequence follows libusb's `darwin_detach_kernel_driver` and
//! `darwin_reenumerate_device`.

use std::ffi::{CStr, c_char, c_void};
use std::ptr;

use core_foundation_sys::base::{
    CFGetTypeID, CFIndex, CFRelease, SInt32, kCFAllocatorDefault, kCFAllocatorSystemDefault,
};
use core_foundation_sys::string::{
    CFStringCreateWithCString, CFStringGetCString, CFStringGetTypeID, kCFStringEncodingUTF8,
};
use core_foundation_sys::uuid::{
    CFUUIDBytes, CFUUIDGetConstantUUIDWithBytes, CFUUIDGetUUIDBytes, CFUUIDRef,
};
use io_kit_sys::keys::kIOServicePlane;
use io_kit_sys::ret::{IOReturn, kIOReturnNoResources, kIOReturnSuccess};
use io_kit_sys::types::io_service_t;
use io_kit_sys::{
    IOObjectRelease, IORegistryEntryIDMatching, IORegistryEntrySearchCFProperty,
    IOServiceGetMatchingService, kIORegistryIterateRecursively,
};

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
    _class_to_get_configuration: [Unused; 13],
    set_configuration: Option<unsafe extern "C" fn(this: *mut c_void, value: u8) -> IOReturn>,
    _frame_number_to_interface_iterator: [Unused; 5],
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

/// The BSD name of the disk below the device with IORegistry entry
/// `registry_id`, such as `disk4`.
pub(crate) fn bsd_disk(registry_id: u64) -> Option<String> {
    // SAFETY: the matching dictionary is consumed by IOServiceGetMatchingService;
    // the service, the key and the value are released on every path.
    unsafe {
        let service = IOServiceGetMatchingService(
            MAIN_PORT_DEFAULT,
            IORegistryEntryIDMatching(registry_id).cast_const(),
        );
        if service == 0 {
            return None;
        }
        let key = CFStringCreateWithCString(
            kCFAllocatorDefault,
            c"BSD Name".as_ptr(),
            kCFStringEncodingUTF8,
        );
        let value = IORegistryEntrySearchCFProperty(
            service,
            kIOServicePlane,
            key,
            kCFAllocatorDefault,
            kIORegistryIterateRecursively,
        );
        CFRelease(key.cast());
        IOObjectRelease(service);
        if value.is_null() {
            return None;
        }
        let mut buffer = [0 as c_char; 64];
        let name = (CFGetTypeID(value) == CFStringGetTypeID()
            && CFStringGetCString(
                value.cast(),
                buffer.as_mut_ptr(),
                buffer.len() as CFIndex,
                kCFStringEncodingUTF8,
            ) != 0)
            .then(|| CStr::from_ptr(buffer.as_ptr()).to_string_lossy().into_owned());
        CFRelease(value);
        name
    }
}

/// Run `diskutil` on `disk` and summarise the outcome in one line.
pub(crate) fn diskutil(args: &[&str], disk: &str) -> String {
    match std::process::Command::new("/usr/sbin/diskutil").args(args).arg(disk).output() {
        Ok(out) => {
            let text = if out.status.success() { &out.stdout } else { &out.stderr };
            let text = String::from_utf8_lossy(text);
            format!("{} {}", out.status, text.lines().next().unwrap_or("").trim())
        }
        Err(e) => e.to_string(),
    }
}

/// A captured device; dropping it gives the device back to its kernel drivers.
pub(crate) struct Capture {
    device: *mut *mut DeviceVtbl,
    open: bool,
    captured: bool,
}

// The handle is only used from the thread that owns the backend holding it.
unsafe impl Send for Capture {}

impl Capture {
    /// Capture the device with IORegistry entry `registry_id` and keep the
    /// capturing handle open. Also returns the open results, for a trace.
    pub(crate) fn take(registry_id: u64) -> Result<(Self, String), String> {
        let device = open_device_interface(registry_id)?;
        let mut capture = Self { device, open: false, captured: false };
        // SAFETY: `device` is a live IOUSBDeviceInterface650 owned by `capture`.
        unsafe {
            let seize = call0(device, (**device).open_seize);
            capture.open =
                seize == kIOReturnSuccess || call0(device, (**device).open) == kIOReturnSuccess;
            if !capture.open {
                return Err(format!("could not open the device to capture it (0x{seize:08x})"));
            }
            let rc = match (**device).re_enumerate {
                Some(f) => f(device.cast(), CAPTURE_DEVICE),
                None => kIOReturnNoResources,
            };
            if rc != kIOReturnSuccess {
                return Err(format!("capture refused (0x{rc:08x})"));
            }
            capture.captured = true;
            // libusb re-opens after capturing, and keeps the device open meanwhile.
            call0(device, (**device).close);
            let reseize = call0(device, (**device).open_seize);
            capture.open =
                reseize == kIOReturnSuccess || call0(device, (**device).open) == kIOReturnSuccess;
            Ok((capture, format!("open 0x{seize:08x}, reopen 0x{reseize:08x}")))
        }
    }

    /// Select configuration `value` through the capturing handle.
    pub(crate) fn set_configuration(&self, value: u8) -> IOReturn {
        // SAFETY: `self.device` is live until drop.
        unsafe {
            match (**self.device).set_configuration {
                Some(f) => f(self.device.cast(), value),
                None => kIOReturnNoResources,
            }
        }
    }
}

impl Drop for Capture {
    fn drop(&mut self) {
        let device = self.device;
        // SAFETY: `device` is live until the release below. Every call may fail
        // harmlessly: a camera that switched mode is no longer this device.
        unsafe {
            if self.captured {
                if !self.open {
                    self.open = call0(device, (**device).open_seize) == kIOReturnSuccess;
                }
                if let Some(f) = (**device).re_enumerate {
                    f(device.cast(), 0);
                }
            }
            if self.open {
                call0(device, (**device).close);
            }
            if let Some(release) = (**device).release {
                release(device.cast());
            }
        }
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
