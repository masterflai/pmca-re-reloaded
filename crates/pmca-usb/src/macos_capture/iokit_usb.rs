//! macOS: capture a mass-storage camera through IOKit's IOUSBLib, the way the
//! Python original and libusb reach it.
//!
//! The IOUSBHost device capture in the parent module stops every driver of a
//! device at once, which macOS 26 allows to root — but on that release opening
//! the freed *interface* through IOUSBHost hit a wall (measured 01.11. and
//! 04.11.2026). A libusb probe showed the older IOUSBLib path still works there
//! as root, reaching an ILCE-5000's mass storage end to end. This module ports
//! exactly that path, so no libusb (LGPL) enters the build:
//!
//!  1. open the device (`USBDeviceOpenSeize`),
//!  2. `USBDeviceReEnumerate` with the *capture* flag, which detaches the
//!     kernel's drivers **without** re-enumerating the bus (so the device keeps
//!     its IORegistry entry and does not drop off),
//!  3. close and re-open the device, which capture requires,
//!  4. claim the mass-storage interface (`USBInterfaceOpen`) and move bulk data
//!     through its pipes (`ReadPipeTO`/`WritePipeTO`).
//!
//! Dropping the capture re-enumerates the device without the flag, which lets
//! the kernel re-match its drivers so the camera's card comes back.
//!
//! The vtable layouts and the plugin/UUID machinery below follow nusb's macOS
//! backend (`iokit_c.rs`/`iokit_usb.rs`, based on Kate Temkin's `usrs`, MIT OR
//! Apache-2.0), which this crate already pulls in; they are reproduced here only
//! because `io-kit-sys` does not expose them. The device vtable is kept through
//! `USBDeviceReEnumerate` and the interface vtable through `ClearPipeStallBothEnds`
//! — every function this module calls lies within those prefixes, and a prefix of
//! a `#[repr(C)]` vtable keeps the offsets of the functions it does declare.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]
// The scalar aliases below keep the names of the C headers (`UInt32`, `HRESULT`,
// `LPVOID`, …) so the vtable transcription reads against them one to one.
#![allow(clippy::upper_case_acronyms)]

use std::collections::HashMap;
use std::ffi::c_void;
use std::ptr;
use std::time::Duration;

use core_foundation_sys::base::{CFAllocatorRef, SInt32, kCFAllocatorSystemDefault, mach_port_t};
use core_foundation_sys::runloop::CFRunLoopSourceRef;
use core_foundation_sys::uuid::{CFUUIDBytes, CFUUIDRef};
use io_kit_sys::ret::{IOReturn, kIOReturnSuccess};
use io_kit_sys::types::{io_iterator_t, io_service_t};
use io_kit_sys::{IOAsyncCallback1, IOIteratorNext, IOObjectRelease};

use crate::{Error, Result};

// --- plain scalar aliases, matching <IOKit/usb/IOUSBLib.h> through nusb ---
type UInt8 = std::os::raw::c_uchar;
type UInt16 = std::os::raw::c_ushort;
type UInt32 = std::os::raw::c_uint;
type UInt64 = std::os::raw::c_ulonglong;
type ULONG = std::os::raw::c_ulong;
type Boolean = std::os::raw::c_uchar;
type kern_return_t = std::os::raw::c_int;
type USBDeviceAddress = UInt16;
type HRESULT = SInt32;
type REFIID = CFUUIDBytes;
type LPVOID = *mut c_void;
type AbsoluteTime = UnsignedWide;

/// `kUSBReEnumerateCaptureDeviceMask`: detach the kernel's drivers and reserve
/// the device for this process, without a bus re-enumeration. Root (or the
/// `com.apple.vm.device-access` entitlement) is required.
const CAPTURE_DEVICE_MASK: UInt32 = 0x4000_0000;

/// `kIOUSBPipeStalled` and `kUSBHostReturnPipeStalled`: a halted endpoint the
/// mass-storage layer recovers from, as opposed to a transport fault.
const STALLED: [IOReturn; 2] = [0xe000_404f_u32 as IOReturn, 0xe000_5000_u32 as IOReturn];

#[repr(C, packed(2))]
#[derive(Clone, Copy)]
struct UnsignedWide {
    lo: UInt32,
    hi: UInt32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct NumVersion {
    nonRelRev: UInt8,
    stage: UInt8,
    minorAndBugRev: UInt8,
    majorRev: UInt8,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct IOUSBDevRequest {
    bmRequestType: UInt8,
    bRequest: UInt8,
    wValue: UInt16,
    wIndex: UInt16,
    wLength: UInt16,
    pData: *mut c_void,
    wLenDone: UInt32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct IOUSBDevRequestTO {
    bmRequestType: UInt8,
    bRequest: UInt8,
    wValue: UInt16,
    wIndex: UInt16,
    wLength: UInt16,
    pData: *mut c_void,
    wLenDone: UInt32,
    noDataTimeout: UInt32,
    completionTimeout: UInt32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct IOUSBFindInterfaceRequest {
    bInterfaceClass: UInt16,
    bInterfaceSubClass: UInt16,
    bInterfaceProtocol: UInt16,
    bAlternateSetting: UInt16,
}

/// `kIOUSBFindInterfaceDontCare`: match every interface in the iterator.
const FIND_INTERFACE_DONT_CARE: UInt16 = 0xFFFF;

#[repr(C, packed)]
#[derive(Clone, Copy)]
struct IOUSBConfigurationDescriptor {
    bLength: u8,
    bDescriptorType: u8,
    wTotalLength: u16,
    bNumInterfaces: u8,
    bConfigurationValue: u8,
    iConfiguration: u8,
    bmAttributes: u8,
    MaxPower: u8,
}
type IOUSBConfigurationDescriptorPtr = *mut IOUSBConfigurationDescriptor;

#[repr(C)]
#[derive(Clone, Copy)]
struct IOUSBIsocFrame {
    frStatus: IOReturn,
    frReqCount: UInt16,
    frActCount: UInt16,
}

type Fn0<R> = Option<unsafe extern "C" fn(*mut c_void) -> R>;

/// `IOCFPlugInInterface`, the intermediate the device/interface interface is
/// queried from.
#[repr(C)]
struct IOCFPlugInInterface {
    _reserved: *mut c_void,
    QueryInterface: Option<unsafe extern "C" fn(*mut c_void, REFIID, *mut LPVOID) -> HRESULT>,
    AddRef: Fn0<ULONG>,
    Release: Fn0<ULONG>,
    version: UInt16,
    revision: UInt16,
    Probe: Option<
        unsafe extern "C" fn(*mut c_void, *const c_void, io_service_t, *mut SInt32) -> IOReturn,
    >,
    Start: Option<unsafe extern "C" fn(*mut c_void, *const c_void, io_service_t) -> IOReturn>,
    Stop: Fn0<IOReturn>,
}

/// `IOUSBDeviceInterface650`, kept through `USBDeviceReEnumerate`.
#[repr(C)]
struct IOUSBDeviceInterface {
    _reserved: *mut c_void,
    QueryInterface: Option<unsafe extern "C" fn(*mut c_void, REFIID, *mut LPVOID) -> HRESULT>,
    AddRef: Fn0<ULONG>,
    Release: Fn0<ULONG>,
    CreateDeviceAsyncEventSource:
        Option<unsafe extern "C" fn(*mut c_void, *mut CFRunLoopSourceRef) -> IOReturn>,
    GetDeviceAsyncEventSource: Option<unsafe extern "C" fn(*mut c_void) -> CFRunLoopSourceRef>,
    CreateDeviceAsyncPort: Option<unsafe extern "C" fn(*mut c_void, *mut mach_port_t) -> IOReturn>,
    GetDeviceAsyncPort: Option<unsafe extern "C" fn(*mut c_void) -> mach_port_t>,
    USBDeviceOpen: Fn0<IOReturn>,
    USBDeviceClose: Fn0<IOReturn>,
    GetDeviceClass: Option<unsafe extern "C" fn(*mut c_void, *mut UInt8) -> IOReturn>,
    GetDeviceSubClass: Option<unsafe extern "C" fn(*mut c_void, *mut UInt8) -> IOReturn>,
    GetDeviceProtocol: Option<unsafe extern "C" fn(*mut c_void, *mut UInt8) -> IOReturn>,
    GetDeviceVendor: Option<unsafe extern "C" fn(*mut c_void, *mut UInt16) -> IOReturn>,
    GetDeviceProduct: Option<unsafe extern "C" fn(*mut c_void, *mut UInt16) -> IOReturn>,
    GetDeviceReleaseNumber: Option<unsafe extern "C" fn(*mut c_void, *mut UInt16) -> IOReturn>,
    GetDeviceAddress: Option<unsafe extern "C" fn(*mut c_void, *mut USBDeviceAddress) -> IOReturn>,
    GetDeviceBusPowerAvailable: Option<unsafe extern "C" fn(*mut c_void, *mut UInt32) -> IOReturn>,
    GetDeviceSpeed: Option<unsafe extern "C" fn(*mut c_void, *mut UInt8) -> IOReturn>,
    GetNumberOfConfigurations: Option<unsafe extern "C" fn(*mut c_void, *mut UInt8) -> IOReturn>,
    GetLocationID: Option<unsafe extern "C" fn(*mut c_void, *mut UInt32) -> IOReturn>,
    GetConfigurationDescriptorPtr: Option<
        unsafe extern "C" fn(*mut c_void, UInt8, *mut IOUSBConfigurationDescriptorPtr) -> IOReturn,
    >,
    GetConfiguration: Option<unsafe extern "C" fn(*mut c_void, *mut UInt8) -> IOReturn>,
    SetConfiguration: Option<unsafe extern "C" fn(*mut c_void, UInt8) -> IOReturn>,
    GetBusFrameNumber:
        Option<unsafe extern "C" fn(*mut c_void, *mut UInt64, *mut AbsoluteTime) -> IOReturn>,
    ResetDevice: Fn0<IOReturn>,
    DeviceRequest: Option<unsafe extern "C" fn(*mut c_void, *mut IOUSBDevRequest) -> IOReturn>,
    DeviceRequestAsync: Option<
        unsafe extern "C" fn(
            *mut c_void,
            *mut IOUSBDevRequest,
            IOAsyncCallback1,
            *mut c_void,
        ) -> IOReturn,
    >,
    CreateInterfaceIterator: Option<
        unsafe extern "C" fn(
            *mut c_void,
            *mut IOUSBFindInterfaceRequest,
            *mut io_iterator_t,
        ) -> IOReturn,
    >,
    USBDeviceOpenSeize: Fn0<IOReturn>,
    DeviceRequestTO: Option<unsafe extern "C" fn(*mut c_void, *mut IOUSBDevRequestTO) -> IOReturn>,
    DeviceRequestAsyncTO: Option<
        unsafe extern "C" fn(
            *mut c_void,
            *mut IOUSBDevRequestTO,
            IOAsyncCallback1,
            *mut c_void,
        ) -> IOReturn,
    >,
    USBDeviceSuspend: Option<unsafe extern "C" fn(*mut c_void, Boolean) -> IOReturn>,
    USBDeviceAbortPipeZero: Fn0<IOReturn>,
    USBGetManufacturerStringIndex:
        Option<unsafe extern "C" fn(*mut c_void, *mut UInt8) -> IOReturn>,
    USBGetProductStringIndex: Option<unsafe extern "C" fn(*mut c_void, *mut UInt8) -> IOReturn>,
    USBGetSerialNumberStringIndex:
        Option<unsafe extern "C" fn(*mut c_void, *mut UInt8) -> IOReturn>,
    USBDeviceReEnumerate: Option<unsafe extern "C" fn(*mut c_void, UInt32) -> IOReturn>,
}

/// `IOUSBInterfaceInterface700`, kept through `ClearPipeStallBothEnds`.
#[repr(C)]
struct IOUSBInterfaceInterface {
    _reserved: *mut c_void,
    QueryInterface: Option<unsafe extern "C" fn(*mut c_void, REFIID, *mut LPVOID) -> HRESULT>,
    AddRef: Fn0<ULONG>,
    Release: Fn0<ULONG>,
    CreateInterfaceAsyncEventSource:
        Option<unsafe extern "C" fn(*mut c_void, *mut CFRunLoopSourceRef) -> IOReturn>,
    GetInterfaceAsyncEventSource: Option<unsafe extern "C" fn(*mut c_void) -> CFRunLoopSourceRef>,
    CreateInterfaceAsyncPort:
        Option<unsafe extern "C" fn(*mut c_void, *mut mach_port_t) -> IOReturn>,
    GetInterfaceAsyncPort: Option<unsafe extern "C" fn(*mut c_void) -> mach_port_t>,
    USBInterfaceOpen: Fn0<IOReturn>,
    USBInterfaceClose: Fn0<IOReturn>,
    GetInterfaceClass: Option<unsafe extern "C" fn(*mut c_void, *mut UInt8) -> IOReturn>,
    GetInterfaceSubClass: Option<unsafe extern "C" fn(*mut c_void, *mut UInt8) -> IOReturn>,
    GetInterfaceProtocol: Option<unsafe extern "C" fn(*mut c_void, *mut UInt8) -> IOReturn>,
    GetDeviceVendor: Option<unsafe extern "C" fn(*mut c_void, *mut UInt16) -> IOReturn>,
    GetDeviceProduct: Option<unsafe extern "C" fn(*mut c_void, *mut UInt16) -> IOReturn>,
    GetDeviceReleaseNumber: Option<unsafe extern "C" fn(*mut c_void, *mut UInt16) -> IOReturn>,
    GetConfigurationValue: Option<unsafe extern "C" fn(*mut c_void, *mut UInt8) -> IOReturn>,
    GetInterfaceNumber: Option<unsafe extern "C" fn(*mut c_void, *mut UInt8) -> IOReturn>,
    GetAlternateSetting: Option<unsafe extern "C" fn(*mut c_void, *mut UInt8) -> IOReturn>,
    GetNumEndpoints: Option<unsafe extern "C" fn(*mut c_void, *mut UInt8) -> IOReturn>,
    GetLocationID: Option<unsafe extern "C" fn(*mut c_void, *mut UInt32) -> IOReturn>,
    GetDevice: Option<unsafe extern "C" fn(*mut c_void, *mut io_service_t) -> IOReturn>,
    SetAlternateInterface: Option<unsafe extern "C" fn(*mut c_void, UInt8) -> IOReturn>,
    GetBusFrameNumber:
        Option<unsafe extern "C" fn(*mut c_void, *mut UInt64, *mut AbsoluteTime) -> IOReturn>,
    ControlRequest:
        Option<unsafe extern "C" fn(*mut c_void, UInt8, *mut IOUSBDevRequest) -> IOReturn>,
    ControlRequestAsync: Option<
        unsafe extern "C" fn(
            *mut c_void,
            UInt8,
            *mut IOUSBDevRequest,
            IOAsyncCallback1,
            *mut c_void,
        ) -> IOReturn,
    >,
    GetPipeProperties: Option<
        unsafe extern "C" fn(
            *mut c_void,
            UInt8,
            *mut UInt8,
            *mut UInt8,
            *mut UInt8,
            *mut UInt16,
            *mut UInt8,
        ) -> IOReturn,
    >,
    GetPipeStatus: Option<unsafe extern "C" fn(*mut c_void, UInt8) -> IOReturn>,
    AbortPipe: Option<unsafe extern "C" fn(*mut c_void, UInt8) -> IOReturn>,
    ResetPipe: Option<unsafe extern "C" fn(*mut c_void, UInt8) -> IOReturn>,
    ClearPipeStall: Option<unsafe extern "C" fn(*mut c_void, UInt8) -> IOReturn>,
    ReadPipe:
        Option<unsafe extern "C" fn(*mut c_void, UInt8, *mut c_void, *mut UInt32) -> IOReturn>,
    WritePipe: Option<unsafe extern "C" fn(*mut c_void, UInt8, *mut c_void, UInt32) -> IOReturn>,
    ReadPipeAsync: Option<
        unsafe extern "C" fn(
            *mut c_void,
            UInt8,
            *mut c_void,
            UInt32,
            IOAsyncCallback1,
            *mut c_void,
        ) -> IOReturn,
    >,
    WritePipeAsync: Option<
        unsafe extern "C" fn(
            *mut c_void,
            UInt8,
            *mut c_void,
            UInt32,
            IOAsyncCallback1,
            *mut c_void,
        ) -> IOReturn,
    >,
    ReadIsochPipeAsync: Option<
        unsafe extern "C" fn(
            *mut c_void,
            UInt8,
            *mut c_void,
            UInt64,
            UInt32,
            *mut IOUSBIsocFrame,
            IOAsyncCallback1,
            *mut c_void,
        ) -> IOReturn,
    >,
    WriteIsochPipeAsync: Option<
        unsafe extern "C" fn(
            *mut c_void,
            UInt8,
            *mut c_void,
            UInt64,
            UInt32,
            *mut IOUSBIsocFrame,
            IOAsyncCallback1,
            *mut c_void,
        ) -> IOReturn,
    >,
    ControlRequestTO:
        Option<unsafe extern "C" fn(*mut c_void, UInt8, *mut IOUSBDevRequestTO) -> IOReturn>,
    ControlRequestAsyncTO: Option<
        unsafe extern "C" fn(
            *mut c_void,
            UInt8,
            *mut IOUSBDevRequestTO,
            IOAsyncCallback1,
            *mut c_void,
        ) -> IOReturn,
    >,
    ReadPipeTO: Option<
        unsafe extern "C" fn(
            *mut c_void,
            UInt8,
            *mut c_void,
            *mut UInt32,
            UInt32,
            UInt32,
        ) -> IOReturn,
    >,
    WritePipeTO: Option<
        unsafe extern "C" fn(*mut c_void, UInt8, *mut c_void, UInt32, UInt32, UInt32) -> IOReturn,
    >,
    ReadPipeAsyncTO: Option<
        unsafe extern "C" fn(
            *mut c_void,
            UInt8,
            *mut c_void,
            UInt32,
            UInt32,
            UInt32,
            IOAsyncCallback1,
            *mut c_void,
        ) -> IOReturn,
    >,
    WritePipeAsyncTO: Option<
        unsafe extern "C" fn(
            *mut c_void,
            UInt8,
            *mut c_void,
            UInt32,
            UInt32,
            UInt32,
            IOAsyncCallback1,
            *mut c_void,
        ) -> IOReturn,
    >,
    USBInterfaceGetStringIndex: Option<unsafe extern "C" fn(*mut c_void, *mut UInt8) -> IOReturn>,
    USBInterfaceOpenSeize: Fn0<IOReturn>,
    ClearPipeStallBothEnds: Option<unsafe extern "C" fn(*mut c_void, UInt8) -> IOReturn>,
}

// IOKit's documentation says these interfaces may be called from any thread;
// only one thread ever owns the backend that holds them here.
unsafe impl Send for IOUSBDeviceInterface {}
unsafe impl Send for IOUSBInterfaceInterface {}

// Declared in IOKit; the frameworks are linked through `io-kit-sys` and
// `core-foundation-sys`, exactly as the parent module relies on them.
unsafe extern "C" {
    fn CFUUIDGetUUIDBytes(uuid: CFUUIDRef) -> CFUUIDBytes;

    fn IOCreatePlugInInterfaceForService(
        service: io_service_t,
        pluginType: CFUUIDRef,
        interfaceType: CFUUIDRef,
        theInterface: *mut *mut *mut IOCFPlugInInterface,
        theScore: *mut SInt32,
    ) -> kern_return_t;

    #[allow(clippy::too_many_arguments)]
    fn CFUUIDGetConstantUUIDWithBytes(
        alloc: CFAllocatorRef,
        b0: UInt8,
        b1: UInt8,
        b2: UInt8,
        b3: UInt8,
        b4: UInt8,
        b5: UInt8,
        b6: UInt8,
        b7: UInt8,
        b8: UInt8,
        b9: UInt8,
        b10: UInt8,
        b11: UInt8,
        b12: UInt8,
        b13: UInt8,
        b14: UInt8,
        b15: UInt8,
    ) -> CFUUIDRef;
}

/// Calls a function pointer in a `**Interface` vtable with the vtable as `self`.
macro_rules! call {
    ($ptr:expr, $function:ident($($args:expr),* $(,)?)) => {{
        let func = (**$ptr).$function.expect("IOKit vtable entry was null");
        func($ptr as *mut c_void, $($args),*)
    }};
}

fn plugin_interface_id() -> CFUUIDRef {
    // SAFETY: a constant UUID the framework interns; no preconditions.
    unsafe {
        CFUUIDGetConstantUUIDWithBytes(
            ptr::null(),
            0xC2,
            0x44,
            0xE8,
            0x58,
            0x10,
            0x9C,
            0x11,
            0xD4,
            0x91,
            0xD4,
            0x00,
            0x50,
            0xE4,
            0xC6,
            0x42,
            0x6F,
        )
    }
}

fn device_user_client_type_id() -> CFUUIDRef {
    // SAFETY: a constant UUID the framework interns; no preconditions.
    unsafe {
        CFUUIDGetConstantUUIDWithBytes(
            ptr::null(),
            0x9d,
            0xc7,
            0xb7,
            0x80,
            0x9e,
            0xc0,
            0x11,
            0xD4,
            0xa5,
            0x4f,
            0x00,
            0x0a,
            0x27,
            0x05,
            0x28,
            0x61,
        )
    }
}

fn interface_user_client_type_id() -> CFUUIDRef {
    // SAFETY: a constant UUID the framework interns; no preconditions.
    unsafe {
        CFUUIDGetConstantUUIDWithBytes(
            ptr::null(),
            0x2d,
            0x97,
            0x86,
            0xc6,
            0x9e,
            0xf3,
            0x11,
            0xD4,
            0xad,
            0x51,
            0x00,
            0x0a,
            0x27,
            0x05,
            0x28,
            0x61,
        )
    }
}

fn device_interface_id() -> CFUUIDRef {
    // SAFETY: a constant UUID the framework interns; no preconditions.
    unsafe {
        CFUUIDGetConstantUUIDWithBytes(
            kCFAllocatorSystemDefault,
            0x4A,
            0xAC,
            0x1B,
            0x2E,
            0x24,
            0xC2,
            0x47,
            0x6A,
            0x96,
            0x4D,
            0x91,
            0x33,
            0x35,
            0x34,
            0xF2,
            0xCC,
        )
    }
}

fn interface_interface_id() -> CFUUIDRef {
    // SAFETY: a constant UUID the framework interns; no preconditions.
    unsafe {
        CFUUIDGetConstantUUIDWithBytes(
            kCFAllocatorSystemDefault,
            0x17,
            0xF9,
            0xE5,
            0x9C,
            0xB0,
            0xA1,
            0x40,
            0x1D,
            0x9A,
            0xC0,
            0x8D,
            0xE2,
            0x7A,
            0xC6,
            0x04,
            0x7E,
        )
    }
}

/// A camera captured and claimed through IOUSBLib: the device held open while
/// its interface carries bulk transfers.
pub(crate) struct CapturedUsbInterface {
    device: *mut *mut IOUSBDeviceInterface,
    interface: *mut *mut IOUSBInterfaceInterface,
    /// Pipe reference for each endpoint address, as IOKit numbers them (1-based).
    pipes: HashMap<u8, u8>,
    registry_id: u64,
    device_open: bool,
    captured: bool,
}

// The IOKit interfaces are only touched from the thread owning the backend.
unsafe impl Send for CapturedUsbInterface {}

impl CapturedUsbInterface {
    /// Capture the device with IORegistry entry `registry_id`, detaching its
    /// kernel drivers, then claim interface `interface_number`. `note` records
    /// each step for a trace. A failure resets the device on drop so its drivers
    /// return.
    pub(crate) fn capture(
        registry_id: u64,
        interface_number: u8,
        note: &mut dyn FnMut(String),
    ) -> std::result::Result<Self, String> {
        if !super::is_root() {
            return Err("iokit-usb: root required".into());
        }
        let device = device_interface(registry_id)
            .ok_or_else(|| "iokit-usb: could not open device interface".to_string())?;
        let mut me = Self {
            device,
            interface: ptr::null_mut(),
            pipes: HashMap::new(),
            registry_id,
            device_open: false,
            captured: false,
        };

        // SAFETY: `device` is a live IOUSBDeviceInterface for the duration of
        // `me`; every call goes through its vtable with it as `self`.
        unsafe {
            // Seizing may still report exclusive access while the disk driver
            // holds an interface; libusb proceeds regardless, and the capture
            // below frees it either way.
            let seize = call!(device, USBDeviceOpenSeize());
            me.device_open = seize == kIOReturnSuccess;
            note(format!("iokit-usb: seize {}", ret_name(seize)));

            let capture = call!(device, USBDeviceReEnumerate(CAPTURE_DEVICE_MASK));
            if capture != kIOReturnSuccess {
                return Err(format!("iokit-usb: capture refused {}", ret_name(capture)));
            }
            me.captured = true;
            note("iokit-usb: captured via USBDeviceReEnumerate".into());

            // Capture does not re-enumerate the bus, but it does require the
            // device to be re-opened before interfaces can be claimed.
            if me.device_open {
                call!(device, USBDeviceClose());
            }
            let reopen = call!(device, USBDeviceOpenSeize());
            me.device_open = reopen == kIOReturnSuccess;
            note(format!("iokit-usb: reopen {}", ret_name(reopen)));

            me.open_interface(interface_number, note)?;
            me.map_pipes(note)?;
        }
        Ok(me)
    }

    /// Find interface `number`, open the IOUSBLib interface for it and claim it.
    ///
    /// # Safety
    ///
    /// `self.device` must be a live, open IOUSBDeviceInterface.
    unsafe fn open_interface(
        &mut self,
        number: u8,
        note: &mut dyn FnMut(String),
    ) -> std::result::Result<(), String> {
        // SAFETY: guaranteed by the caller.
        let service = unsafe { interface_service(self.device, number) }
            .ok_or_else(|| format!("iokit-usb: interface {number} not found"))?;
        let interface = unsafe { query_interface_interface(service) };
        // SAFETY: the service was retained by the iterator; release our ref.
        unsafe { IOObjectRelease(service) };
        let interface =
            interface.ok_or_else(|| "iokit-usb: could not open interface interface".to_string())?;
        self.interface = interface;
        // SAFETY: `interface` is a live IOUSBInterfaceInterface.
        let opened = unsafe { call!(interface, USBInterfaceOpen()) };
        if opened != kIOReturnSuccess {
            return Err(format!("iokit-usb: USBInterfaceOpen {}", ret_name(opened)));
        }
        note(format!("iokit-usb: interface {number} claimed"));
        Ok(())
    }

    /// Build the endpoint-address → pipe-reference map IOKit addresses transfers
    /// by.
    ///
    /// # Safety
    ///
    /// `self.interface` must be a live, open IOUSBInterfaceInterface.
    unsafe fn map_pipes(
        &mut self,
        note: &mut dyn FnMut(String),
    ) -> std::result::Result<(), String> {
        let interface = self.interface;
        // SAFETY: guaranteed by the caller.
        unsafe {
            let mut count: UInt8 = 0;
            let got = call!(interface, GetNumEndpoints(&mut count));
            if got != kIOReturnSuccess {
                return Err(format!("iokit-usb: GetNumEndpoints {}", ret_name(got)));
            }
            // Pipe 0 is the control pipe and is not counted; data pipes are 1-based.
            for pipe_ref in 1..=count {
                let mut direction: UInt8 = 0;
                let mut endpoint: UInt8 = 0;
                let mut transfer_type: UInt8 = 0;
                let mut max_packet: UInt16 = 0;
                let mut interval: UInt8 = 0;
                let props = call!(
                    interface,
                    GetPipeProperties(
                        pipe_ref,
                        &mut direction,
                        &mut endpoint,
                        &mut transfer_type,
                        &mut max_packet,
                        &mut interval,
                    )
                );
                if props != kIOReturnSuccess {
                    continue;
                }
                let address = endpoint | (u8::from(direction != 0) << 7);
                self.pipes.insert(address, pipe_ref);
            }
        }
        note(format!("iokit-usb: {} pipe(s) mapped", self.pipes.len()));
        if self.pipes.is_empty() {
            return Err("iokit-usb: no bulk pipes".into());
        }
        Ok(())
    }

    fn pipe(&self, ep: u8) -> Result<u8> {
        self.pipes.get(&ep).copied().ok_or(Error::Protocol("endpoint not on the interface"))
    }

    /// Read up to `length` bytes from IN endpoint `ep`.
    pub(crate) fn read(&mut self, ep: u8, length: usize, timeout: Duration) -> Result<Vec<u8>> {
        let pipe = self.pipe(ep)?;
        let ms = timeout_ms(timeout);
        let mut buffer = vec![0u8; length];
        let mut size = u32::try_from(length).map_err(|_| Error::Protocol("read too long"))?;
        // SAFETY: `self.interface` is live; the buffer outlives the call and is
        // written only up to `size` bytes, which IOKit caps at `length`.
        let code = unsafe {
            call!(self.interface, ReadPipeTO(pipe, buffer.as_mut_ptr().cast(), &mut size, ms, ms))
        };
        if code != kIOReturnSuccess {
            return Err(io_error("read", code));
        }
        buffer.truncate(size as usize);
        Ok(buffer)
    }

    /// Write `data` to OUT endpoint `ep`.
    pub(crate) fn write(&mut self, ep: u8, data: &[u8], timeout: Duration) -> Result<()> {
        let pipe = self.pipe(ep)?;
        let ms = timeout_ms(timeout);
        let size = u32::try_from(data.len()).map_err(|_| Error::Protocol("write too long"))?;
        // SAFETY: `self.interface` is live; `data` outlives the call and IOKit
        // only reads `size` bytes from it.
        let code = unsafe {
            call!(self.interface, WritePipeTO(pipe, data.as_ptr() as *mut c_void, size, ms, ms))
        };
        if code != kIOReturnSuccess {
            return Err(io_error("write", code));
        }
        Ok(())
    }

    /// Clear the halt of endpoint `ep`, on both the host and the device.
    pub(crate) fn clear_halt(&mut self, ep: u8) -> Result<()> {
        let pipe = self.pipe(ep)?;
        // SAFETY: `self.interface` is live.
        let code = unsafe { call!(self.interface, ClearPipeStallBothEnds(pipe)) };
        if code != kIOReturnSuccess {
            return Err(io_error("clear halt", code));
        }
        Ok(())
    }

    /// Send a host-to-device control transfer through the captured device.
    pub(crate) fn control_out(
        &mut self,
        request_type: u8,
        request: u8,
        value: u16,
        index: u16,
        data: &[u8],
        timeout: Duration,
    ) -> Result<()> {
        let ms = timeout_ms(timeout);
        let length =
            u16::try_from(data.len()).map_err(|_| Error::Protocol("control data too long"))?;
        let mut req = IOUSBDevRequestTO {
            bmRequestType: request_type,
            bRequest: request,
            wValue: value,
            wIndex: index,
            wLength: length,
            pData: data.as_ptr() as *mut c_void,
            wLenDone: 0,
            noDataTimeout: ms,
            completionTimeout: ms,
        };
        // SAFETY: `self.device` is live and open; `req` (and the buffer it
        // points at) outlives the call.
        let code = unsafe { call!(self.device, DeviceRequestTO(&mut req)) };
        if code != kIOReturnSuccess {
            return Err(io_error("control transfer", code));
        }
        Ok(())
    }
}

impl Drop for CapturedUsbInterface {
    fn drop(&mut self) {
        // SAFETY: each interface is live until released here, and released once.
        unsafe {
            if !self.interface.is_null() {
                call!(self.interface, USBInterfaceClose());
                call!(self.interface, Release());
            }
            if !self.device.is_null() {
                if self.captured {
                    // Re-enumerate without the capture flag so the kernel
                    // re-matches the device's drivers and the card comes back.
                    call!(self.device, USBDeviceReEnumerate(0));
                }
                if self.device_open {
                    call!(self.device, USBDeviceClose());
                }
                call!(self.device, Release());
            }
        }
        let _ = self.registry_id;
    }
}

/// Open the IOUSBLib device interface for the service with IORegistry entry
/// `registry_id`.
fn device_interface(registry_id: u64) -> Option<*mut *mut IOUSBDeviceInterface> {
    let service = super::service_by_id(registry_id)?;
    let mut plugin: *mut *mut IOCFPlugInInterface = ptr::null_mut();
    let mut score: SInt32 = 0;
    // SAFETY: the service is live until released; the plugin is released once
    // the device interface is queried, following IOKit's create rule.
    unsafe {
        let rc = IOCreatePlugInInterfaceForService(
            service,
            device_user_client_type_id(),
            plugin_interface_id(),
            &mut plugin,
            &mut score,
        );
        IOObjectRelease(service);
        if rc != kIOReturnSuccess || plugin.is_null() {
            return None;
        }
        let mut device: *mut *mut IOUSBDeviceInterface = ptr::null_mut();
        call!(
            plugin,
            QueryInterface(
                CFUUIDGetUUIDBytes(device_interface_id()),
                (&mut device as *mut *mut *mut IOUSBDeviceInterface).cast(),
            )
        );
        call!(plugin, Release());
        (!device.is_null()).then_some(device)
    }
}

/// The IOUSBLib interface service numbered `number` below the captured device.
///
/// # Safety
///
/// `device` must be a live IOUSBDeviceInterface.
unsafe fn interface_service(
    device: *mut *mut IOUSBDeviceInterface,
    number: u8,
) -> Option<io_service_t> {
    let mut request = IOUSBFindInterfaceRequest {
        bInterfaceClass: FIND_INTERFACE_DONT_CARE,
        bInterfaceSubClass: FIND_INTERFACE_DONT_CARE,
        bInterfaceProtocol: FIND_INTERFACE_DONT_CARE,
        bAlternateSetting: FIND_INTERFACE_DONT_CARE,
    };
    let mut iterator: io_iterator_t = 0;
    // SAFETY: guaranteed by the caller; the iterator and every unmatched child
    // are released.
    unsafe {
        let created = call!(device, CreateInterfaceIterator(&mut request, &mut iterator));
        if created != kIOReturnSuccess {
            return None;
        }
        let mut found = None;
        loop {
            let child = IOIteratorNext(iterator);
            if child == 0 {
                break;
            }
            if found.is_none()
                && super::number_property(child, c"bInterfaceNumber") == Some(i32::from(number))
            {
                found = Some(child);
            } else {
                IOObjectRelease(child);
            }
        }
        IOObjectRelease(iterator);
        found
    }
}

/// Query an IOUSBLib interface interface from the interface `service`.
///
/// # Safety
///
/// `service` must be a live IOUSBInterface service.
unsafe fn query_interface_interface(
    service: io_service_t,
) -> Option<*mut *mut IOUSBInterfaceInterface> {
    let mut plugin: *mut *mut IOCFPlugInInterface = ptr::null_mut();
    let mut score: SInt32 = 0;
    // SAFETY: guaranteed by the caller; the plugin is released after the query.
    unsafe {
        let rc = IOCreatePlugInInterfaceForService(
            service,
            interface_user_client_type_id(),
            plugin_interface_id(),
            &mut plugin,
            &mut score,
        );
        if rc != kIOReturnSuccess || plugin.is_null() {
            return None;
        }
        let mut interface: *mut *mut IOUSBInterfaceInterface = ptr::null_mut();
        call!(
            plugin,
            QueryInterface(
                CFUUIDGetUUIDBytes(interface_interface_id()),
                (&mut interface as *mut *mut *mut IOUSBInterfaceInterface).cast(),
            )
        );
        call!(plugin, Release());
        (!interface.is_null()).then_some(interface)
    }
}

/// Milliseconds for IOKit's pipe timeouts, saturated into its 32-bit field.
fn timeout_ms(timeout: Duration) -> UInt32 {
    timeout.as_millis().try_into().unwrap_or(UInt32::MAX)
}

/// Classify a failed IOUSBLib call, keeping a recoverable stall apart from a
/// transport fault.
fn io_error(step: &str, code: IOReturn) -> Error {
    if STALLED.contains(&code) {
        Error::Stall
    } else {
        Error::Transfer(format!("iokit-usb {step}: {}", ret_name(code)))
    }
}

/// A short name for the IOReturn codes this path meets, for a trace.
fn ret_name(code: IOReturn) -> String {
    let name = match code as u32 {
        0x0000_0000 => "ok",
        0xe000_02c5 => "exclusive access",
        0xe000_02c0 => "no device",
        0xe000_02cd => "not open",
        0xe000_02e2 => "not permitted",
        0xe000_02be => "no resources",
        0xe000_404f | 0xe000_5000 => "pipe stalled",
        0xe000_02d6 => "timeout",
        _ => return format!("0x{:08x}", code as u32),
    };
    format!("{name} (0x{:08x})", code as u32)
}
