//! macOS: take a mass-storage camera away from the system's disk driver.
//!
//! Since macOS 26 the system refuses every IOUSBLib user client for an
//! interface one of its own drivers holds, so `nusb` (like libusb) cannot reach
//! a camera in mass-storage mode at all, root or not. The IOUSBHost framework
//! still can: opening the device with its capture option ends every driver of
//! the device and its interfaces, which Apple allows to root.
//! [`HostInterface`] does that and then moves data through IOUSBHost's own
//! interface and pipe objects. Destroying the device resets it and lets the
//! system match its drivers again, so the card comes back.
//!
//! IOUSBHost is an Objective-C framework. It is driven through the Objective-C
//! runtime directly, which keeps the dependency list as it was.
//!
//! The module also holds what prepares a capture and explains a failed one:
//! the root check, unmounting the camera's card and a trace of the drivers
//! that hold the device.

use std::collections::HashMap;
use std::ffi::{CStr, c_char, c_void};
use std::ptr;
use std::time::{Duration, Instant};

use core_foundation_sys::base::{CFGetTypeID, CFIndex, CFRelease, kCFAllocatorDefault};
use core_foundation_sys::number::{CFNumberGetTypeID, CFNumberGetValue, kCFNumberSInt32Type};
use core_foundation_sys::string::{
    CFStringCreateWithCString, CFStringGetCString, CFStringGetTypeID, kCFStringEncodingUTF8,
};
use io_kit_sys::keys::kIOServicePlane;
use io_kit_sys::ret::kIOReturnSuccess;
use io_kit_sys::types::{io_iterator_t, io_registry_entry_t, io_service_t};
use io_kit_sys::{
    IOIteratorNext, IOObjectConformsTo, IOObjectGetClass, IOObjectRelease,
    IORegistryEntryCreateCFProperty, IORegistryEntryGetChildIterator, IORegistryEntryIDMatching,
    IORegistryEntrySearchCFProperty, IOServiceGetMatchingService, kIORegistryIterateRecursively,
};

use crate::{Error, Result};

/// `kIOMainPortDefault`, which is defined as 0.
const MAIN_PORT_DEFAULT: u32 = 0;

/// `IOUSBHostObjectInitOptionsDeviceCapture`.
const INIT_DEVICE_CAPTURE: usize = 1 << 0;

/// `IOUSBHostObjectDestroyOptionsDeviceSurrender`: no reset, no re-matching.
const DESTROY_SURRENDER: usize = 1 << 0;

/// `kIOReturnError`, reported when IOUSBHost gives no error object.
const IO_RETURN_ERROR: i32 = 0xe000_02bc_u32 as i32;

/// The IOReturn codes of a stalled endpoint: `kIOUSBPipeStalled` and
/// `kUSBHostReturnPipeStalled`.
const STALLED: [i32; 2] = [0xe000_404f_u32 as i32, 0xe000_5000_u32 as i32];

/// How long the captured device's interface may stay unavailable while macOS
/// ends its drivers.
const INTERFACE_TIMEOUT: Duration = Duration::from_secs(10);

/// How often the interface is looked for meanwhile.
const INTERFACE_POLL: Duration = Duration::from_millis(100);

type Id = *mut c_void;
type Sel = *const c_void;

#[link(name = "objc")]
unsafe extern "C" {
    fn objc_getClass(name: *const c_char) -> Id;
    fn sel_registerName(name: *const c_char) -> Sel;
    fn objc_msgSend();
    fn objc_autoreleasePoolPush() -> *mut c_void;
    fn objc_autoreleasePoolPop(pool: *mut c_void);
}

#[link(name = "IOUSBHost", kind = "framework")]
unsafe extern "C" {
    static IOUSBHostVersionNumber: f64;
}

unsafe extern "C" {
    fn geteuid() -> u32;
}

/// Sends an Objective-C message: `send!(Ret; receiver, c"selector:", arg => Type, ...)`.
///
/// `objc_msgSend` must be called through a pointer of the method's exact type.
macro_rules! send {
    ($ret:ty; $receiver:expr, $selector:expr $(, $arg:expr => $ty:ty)* $(,)?) => {{
        let f = std::mem::transmute::<unsafe extern "C" fn(), unsafe extern "C" fn(Id, Sel $(, $ty)*) -> $ret>(
            objc_msgSend as unsafe extern "C" fn(),
        );
        f($receiver, sel_registerName(CStr::as_ptr($selector)) $(, $arg)*)
    }};
}

/// Whether this process runs as root, which capturing requires.
pub(crate) fn is_root() -> bool {
    // SAFETY: geteuid has no preconditions and cannot fail.
    unsafe { geteuid() == 0 }
}

/// An autorelease pool for the duration of one call: IOUSBHost hands out its
/// error objects autoreleased, and this thread has no pool of its own.
struct Pool(*mut c_void);

impl Pool {
    fn new() -> Self {
        // SAFETY: push has no preconditions; the pool is popped on drop.
        Self(unsafe { objc_autoreleasePoolPush() })
    }
}

impl Drop for Pool {
    fn drop(&mut self) {
        // SAFETY: pops the pool pushed in `new`, on the same thread.
        unsafe { objc_autoreleasePoolPop(self.0) }
    }
}

/// `IOUSBDeviceRequest`, the setup packet of a control transfer.
#[repr(C, packed)]
#[derive(Clone, Copy)]
struct DeviceRequest {
    request_type: u8,
    request: u8,
    value: u16,
    index: u16,
    length: u16,
}

/// A captured camera's interface, reached through IOUSBHost.
pub(crate) struct HostInterface {
    device: Id,
    interface: Id,
    /// Pipes opened so far, keyed by endpoint address.
    pipes: HashMap<u8, Id>,
    registry_id: u64,
}

// The objects are only used from the thread that owns the backend holding them.
unsafe impl Send for HostInterface {}

impl HostInterface {
    /// Capture the device with IORegistry entry `registry_id` and open its
    /// interface `interface_number`. `note` records each step for a trace.
    pub(crate) fn capture(
        registry_id: u64,
        interface_number: u8,
        note: &mut dyn FnMut(String),
    ) -> std::result::Result<Self, String> {
        let _pool = Pool::new();
        // SAFETY: every object created here is owned by `host`, whose drop
        // destroys and releases it; services are released after use.
        unsafe {
            note(format!("IOUSBHost {}", IOUSBHostVersionNumber));
            let service = service_by_id(registry_id).ok_or("device not in the IORegistry")?;
            let device = init_object(c"IOUSBHostDevice", service, INIT_DEVICE_CAPTURE);
            IOObjectRelease(service);
            let device = device.map_err(|code| format!("capture refused (0x{code:08x})"))?;
            let mut host =
                Self { device, interface: ptr::null_mut(), pipes: HashMap::new(), registry_id };
            note(format!("captured, drivers: {}", driver_tree(registry_id)));

            let deadline = Instant::now() + INTERFACE_TIMEOUT;
            let mut configured = false;
            loop {
                match interface_service(registry_id, interface_number) {
                    Some(service) => {
                        let opened = init_object(c"IOUSBHostInterface", service, 0);
                        IOObjectRelease(service);
                        match opened {
                            Ok(interface) => {
                                host.interface = interface;
                                return Ok(host);
                            }
                            Err(code) => note(format!("interface refused (0x{code:08x})")),
                        }
                    }
                    // Without its drivers the device may have dropped its configuration.
                    None if !configured => {
                        configured = true;
                        note(format!("configure: 0x{:08x}", host.configure()));
                    }
                    None => note("no interface".into()),
                }
                if Instant::now() >= deadline {
                    return Err("interface not available".into());
                }
                std::thread::sleep(INTERFACE_POLL);
            }
        }
    }

    /// Select the device's first configuration; returns the IOReturn code.
    fn configure(&self) -> i32 {
        // SAFETY: `self.device` is a live IOUSBHostDevice; the descriptor
        // pointer IOUSBHost returns stays valid while the device does.
        unsafe {
            let mut error: Id = ptr::null_mut();
            let descriptor = send!(*const u8; self.device, c"configurationDescriptorWithIndex:error:",
                0usize => usize, &mut error => *mut Id);
            let value = if descriptor.is_null() { 1 } else { *descriptor.add(5) };
            let mut error: Id = ptr::null_mut();
            let ok = send!(i8; self.device, c"configureWithValue:matchInterfaces:error:",
                usize::from(value) => usize, 1i8 => i8, &mut error => *mut Id);
            if ok != 0 { kIOReturnSuccess } else { error_code(error) }
        }
    }

    /// The pipe for endpoint `ep`, opened on first use.
    fn pipe(&mut self, ep: u8) -> Result<Id> {
        if let Some(&pipe) = self.pipes.get(&ep) {
            return Ok(pipe);
        }
        // SAFETY: `self.interface` is a live IOUSBHostInterface; the pipe is
        // released on drop.
        let pipe = unsafe {
            let mut error: Id = ptr::null_mut();
            let pipe = send!(Id; self.interface, c"copyPipeWithAddress:error:",
                usize::from(ep) => usize, &mut error => *mut Id);
            if pipe.is_null() {
                return Err(io_error("open pipe", error_code(error)));
            }
            pipe
        };
        self.pipes.insert(ep, pipe);
        Ok(pipe)
    }

    /// Run one bulk transfer on `ep` with `data` as its buffer; returns the
    /// bytes transferred.
    ///
    /// # Safety
    ///
    /// `data` must be a live `NSMutableData` or null.
    unsafe fn transfer(&mut self, ep: u8, data: Id, timeout: Duration) -> Result<usize> {
        let pipe = self.pipe(ep)?;
        let mut done = 0usize;
        let mut error: Id = ptr::null_mut();
        // SAFETY: `pipe` is live; `data` is guaranteed by the caller; `error`
        // is an NSError or null.
        unsafe {
            let ok = send!(i8; pipe, c"sendIORequestWithData:bytesTransferred:completionTimeout:error:",
                data => Id, &mut done => *mut usize, timeout.as_secs_f64() => f64,
                &mut error => *mut Id);
            if ok != 0 { Ok(done) } else { Err(io_error("bulk transfer", error_code(error))) }
        }
    }

    /// Read up to `length` bytes from IN endpoint `ep`.
    pub(crate) fn read(&mut self, ep: u8, length: usize, timeout: Duration) -> Result<Vec<u8>> {
        let _pool = Pool::new();
        // SAFETY: the buffer is created here, read only up to the bytes the
        // transfer reports, and released on every path.
        unsafe {
            let data = send!(Id; send!(Id; objc_getClass(c"NSMutableData".as_ptr()), c"alloc"),
                c"initWithLength:", length => usize);
            let result = self.transfer(ep, data, timeout).map(|done| {
                let bytes = send!(*const u8; data, c"mutableBytes");
                std::slice::from_raw_parts(bytes, done.min(length)).to_vec()
            });
            send!((); data, c"release");
            result
        }
    }

    /// Write `data` to OUT endpoint `ep`.
    pub(crate) fn write(&mut self, ep: u8, data: &[u8], timeout: Duration) -> Result<()> {
        let _pool = Pool::new();
        // SAFETY: the buffer copies `data` and is released on every path.
        unsafe {
            let buffer = new_data(data);
            let result = self.transfer(ep, buffer, timeout).map(|_| ());
            release(buffer);
            result
        }
    }

    /// Clear the halt of endpoint `ep`.
    pub(crate) fn clear_halt(&mut self, ep: u8) -> Result<()> {
        let _pool = Pool::new();
        let pipe = self.pipe(ep)?;
        // SAFETY: `pipe` is live.
        unsafe {
            let mut error: Id = ptr::null_mut();
            let ok = send!(i8; pipe, c"clearStallWithError:", &mut error => *mut Id);
            if ok != 0 { Ok(()) } else { Err(io_error("clear halt", error_code(error))) }
        }
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
        let _pool = Pool::new();
        let length = u16::try_from(data.len())
            .map_err(|_| Error::Protocol("control transfer data too long"))?;
        let setup = DeviceRequest { request_type, request, value, index, length };
        // SAFETY: `self.device` is live; the buffer is released on every path.
        unsafe {
            let buffer = new_data(data);
            let mut error: Id = ptr::null_mut();
            let ok = send!(i8; self.device,
                c"sendDeviceRequest:data:bytesTransferred:completionTimeout:error:",
                setup => DeviceRequest, buffer => Id, ptr::null_mut() => *mut usize,
                timeout.as_secs_f64() => f64, &mut error => *mut Id);
            let code = error_code(error);
            release(buffer);
            if ok != 0 { Ok(()) } else { Err(io_error("control transfer", code)) }
        }
    }
}

impl Drop for HostInterface {
    fn drop(&mut self) {
        let _pool = Pool::new();
        // SAFETY: each object is live until released here, and released once.
        unsafe {
            for &pipe in self.pipes.values() {
                release(pipe);
            }
            if !self.interface.is_null() {
                send!((); self.interface, c"destroy");
                release(self.interface);
            }
            // A camera that switched mode is a new device by now; a reset
            // would hit that one instead.
            let present = service_by_id(self.registry_id).map(|s| IOObjectRelease(s)).is_some();
            if present {
                send!((); self.device, c"destroy");
            } else {
                send!((); self.device, c"destroyWithOptions:", DESTROY_SURRENDER => usize);
            }
            release(self.device);
        }
    }
}

/// `[[class alloc] initWithIOService:service options:options queue:nil error:&e interestHandler:nil]`.
///
/// # Safety
///
/// `service` must be a live service of the kind `class` represents.
unsafe fn init_object(
    class: &CStr,
    service: io_service_t,
    options: usize,
) -> std::result::Result<Id, i32> {
    // SAFETY: guaranteed by the caller; a failed init releases the allocation.
    unsafe {
        let allocated = send!(Id; objc_getClass(class.as_ptr()), c"alloc");
        let mut error: Id = ptr::null_mut();
        let object = send!(Id; allocated, c"initWithIOService:options:queue:error:interestHandler:",
            service => io_service_t, options => usize, ptr::null_mut() => Id,
            &mut error => *mut Id, ptr::null_mut() => Id);
        if object.is_null() { Err(error_code(error)) } else { Ok(object) }
    }
}

/// A new `NSMutableData` holding a copy of `bytes`, or nil when empty.
///
/// # Safety
///
/// The result must be released with [`release`].
unsafe fn new_data(bytes: &[u8]) -> Id {
    if bytes.is_empty() {
        return ptr::null_mut();
    }
    // SAFETY: copies `bytes`, which outlive the call.
    unsafe {
        let allocated = send!(Id; objc_getClass(c"NSMutableData".as_ptr()), c"alloc");
        send!(Id; allocated, c"initWithBytes:length:",
            bytes.as_ptr() => *const u8, bytes.len() => usize)
    }
}

/// # Safety
///
/// `object` must be an owned Objective-C object or null.
unsafe fn release(object: Id) {
    if !object.is_null() {
        // SAFETY: guaranteed by the caller.
        unsafe { send!((); object, c"release") }
    }
}

/// The IOReturn code inside an `NSError` from IOUSBHost.
///
/// # Safety
///
/// `error` must be an `NSError` or null.
unsafe fn error_code(error: Id) -> i32 {
    if error.is_null() {
        return IO_RETURN_ERROR;
    }
    // SAFETY: guaranteed by the caller. IOReturn codes are 32-bit.
    unsafe { send!(isize; error, c"code") as i32 }
}

/// Classifies a failed IOUSBHost call.
fn io_error(step: &str, code: i32) -> Error {
    if STALLED.contains(&code) {
        Error::Stall
    } else {
        Error::Transfer(format!("{step} failed (0x{code:08x})"))
    }
}

/// The service with IORegistry entry `registry_id`, to be released by the caller.
fn service_by_id(registry_id: u64) -> Option<io_service_t> {
    // SAFETY: the matching dictionary is consumed by IOServiceGetMatchingService.
    let service = unsafe {
        IOServiceGetMatchingService(
            MAIN_PORT_DEFAULT,
            IORegistryEntryIDMatching(registry_id).cast_const(),
        )
    };
    (service != 0).then_some(service)
}

/// The IOUSBHostInterface service numbered `number` below the device with
/// IORegistry entry `registry_id`, to be released by the caller.
fn interface_service(registry_id: u64, number: u8) -> Option<io_service_t> {
    let device = service_by_id(registry_id)?;
    let mut iterator: io_iterator_t = 0;
    // SAFETY: the iterator and every child that is not returned are released.
    unsafe {
        let listed = IORegistryEntryGetChildIterator(device, kIOServicePlane, &mut iterator);
        IOObjectRelease(device);
        if listed != kIOReturnSuccess {
            return None;
        }
        let mut found = None;
        loop {
            let child = IOIteratorNext(iterator);
            if child == 0 {
                break;
            }
            if found.is_none()
                && IOObjectConformsTo(child, c"IOUSBHostInterface".as_ptr()) != 0
                && number_property(child, c"bInterfaceNumber") == Some(i32::from(number))
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

/// A numeric property of a registry entry.
///
/// # Safety
///
/// `entry` must be a live registry entry.
unsafe fn number_property(entry: io_registry_entry_t, key: &CStr) -> Option<i32> {
    // SAFETY: guaranteed by the caller; the key and the value are released.
    unsafe {
        let key =
            CFStringCreateWithCString(kCFAllocatorDefault, key.as_ptr(), kCFStringEncodingUTF8);
        let value = IORegistryEntryCreateCFProperty(entry, key, kCFAllocatorDefault, 0);
        CFRelease(key.cast());
        if value.is_null() {
            return None;
        }
        let mut number = 0i32;
        let read = CFGetTypeID(value) == CFNumberGetTypeID()
            && CFNumberGetValue(value.cast(), kCFNumberSInt32Type, (&raw mut number).cast());
        CFRelease(value);
        read.then_some(number)
    }
}

/// The BSD name of the disk below the device with IORegistry entry
/// `registry_id`, such as `disk4`.
pub(crate) fn bsd_disk(registry_id: u64) -> Option<String> {
    let service = service_by_id(registry_id)?;
    // SAFETY: the service, the key and the value are released on every path.
    unsafe {
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

/// The macOS version, for a trace.
pub(crate) fn os_version() -> String {
    std::process::Command::new("/usr/bin/sw_vers")
        .arg("-productVersion")
        .output()
        .map(|out| String::from_utf8_lossy(&out.stdout).trim().to_string())
        .unwrap_or_else(|e| e.to_string())
}

/// The IOKit classes below the device with IORegistry entry `registry_id`,
/// three levels deep, for a trace: which drivers hold its interfaces.
pub(crate) fn driver_tree(registry_id: u64) -> String {
    let Some(service) = service_by_id(registry_id) else {
        return "device gone".into();
    };
    // SAFETY: `service` is live until released below.
    unsafe {
        let tree = children(service, 3).join(" ");
        IOObjectRelease(service);
        tree
    }
}

/// # Safety
///
/// `entry` must be a live registry entry.
unsafe fn children(entry: io_registry_entry_t, depth: u32) -> Vec<String> {
    let mut iterator: io_iterator_t = 0;
    // SAFETY: guaranteed by the caller; the iterator and every child are released.
    unsafe {
        if depth == 0
            || IORegistryEntryGetChildIterator(entry, kIOServicePlane, &mut iterator)
                != kIOReturnSuccess
        {
            return Vec::new();
        }
        let mut out = Vec::new();
        loop {
            let child = IOIteratorNext(iterator);
            if child == 0 {
                break;
            }
            let mut name = [0 as c_char; 128];
            IOObjectGetClass(child, name.as_mut_ptr());
            let name = CStr::from_ptr(name.as_ptr()).to_string_lossy().into_owned();
            let below = children(child, depth - 1);
            out.push(if below.is_empty() { name } else { format!("{name}[{}]", below.join(",")) });
            IOObjectRelease(child);
        }
        IOObjectRelease(iterator);
        out
    }
}
