//! Windows mass-storage transport via `IOCTL_SCSI_PASS_THROUGH_DIRECT`.
//!
//! Byte-faithful port of `pmca/usb/driver/windows/msc.py`. Instead of driving
//! the Bulk-Only Transport ourselves over WinUSB (as [`crate::msc`] does), this
//! hands a SCSI command block directly to the Windows USBSTOR mass-storage
//! stack, which performs the CBW/CSW framing in the kernel.
//!
//! This path is required for Sony cameras **in firmware-updater mode**: their
//! bulk OUT endpoint STALLs on the first raw WinUSB Command Block Wrapper even
//! after a full Bulk-Only reset, whereas the OS mass-storage stack handles the
//! device correctly. It only works when the device is bound to the default
//! `USBSTOR` driver (i.e. it shows up as a removable drive), not when a WinUSB
//! filter driver (e.g. installed via Zadig) has replaced it.
//!
//! This module is one of the three that contain `unsafe` code (with [`crate::wpd`]
//! and [`crate::winusb_install`]) — here a handful of Win32 FFI calls
//! (`CreateFileW`, `DeviceIoControl`, `CloseHandle`, `GetLogicalDrives`,
//! `GetDriveTypeW`) — deliberately confined so the rest of the crate stays
//! `#![deny(unsafe_code)]`.
#![allow(unsafe_code)]

use core::ffi::c_void;
use core::ptr;
use std::time::Duration;

use crate::command::{ScsiExtCmdChannel, SonyExtCmdTransport};
use crate::msc::{SENSE_OK, Sense};
use crate::{Error, Result};

// --- Win32 FFI ------------------------------------------------------------

/// Win32 `HANDLE` (pointer-sized).
type Handle = isize;

const INVALID_HANDLE_VALUE: Handle = -1;

const GENERIC_READ: u32 = 0x8000_0000;
const GENERIC_WRITE: u32 = 0x4000_0000;
const FILE_SHARE_READ: u32 = 0x0000_0001;
const FILE_SHARE_WRITE: u32 = 0x0000_0002;
const OPEN_EXISTING: u32 = 3;
const DRIVE_REMOVABLE: u32 = 2;

/// `IOCTL_SCSI_PASS_THROUGH_DIRECT`.
const IOCTL_SCSI_PASS_THROUGH_DIRECT: u32 = 0x4d014;

/// SCSI passthrough data directions.
const SCSI_IOCTL_DATA_OUT: u8 = 0;
const SCSI_IOCTL_DATA_IN: u8 = 1;
const SCSI_IOCTL_DATA_UNSPECIFIED: u8 = 2;

#[link(name = "kernel32")]
unsafe extern "system" {
    fn CreateFileW(
        lp_file_name: *const u16,
        dw_desired_access: u32,
        dw_share_mode: u32,
        lp_security_attributes: *mut c_void,
        dw_creation_disposition: u32,
        dw_flags_and_attributes: u32,
        h_template_file: Handle,
    ) -> Handle;

    fn CloseHandle(h_object: Handle) -> i32;

    fn DeviceIoControl(
        h_device: Handle,
        dw_io_control_code: u32,
        lp_in_buffer: *mut c_void,
        n_in_buffer_size: u32,
        lp_out_buffer: *mut c_void,
        n_out_buffer_size: u32,
        lp_bytes_returned: *mut u32,
        lp_overlapped: *mut c_void,
    ) -> i32;

    fn GetLogicalDrives() -> u32;

    fn GetDriveTypeW(lp_root_path_name: *const u16) -> u32;
}

/// `SCSI_PASS_THROUGH_DIRECT` (natural C layout, matching the reference
/// `ctypes.Structure`; the pointer field forces 8-byte alignment on x64).
#[repr(C)]
struct ScsiPassThroughDirect {
    length: u16,
    scsi_status: u8,
    path_id: u8,
    target_id: u8,
    lun: u8,
    cdb_length: u8,
    sense_info_length: u8,
    data_in: u8,
    data_transfer_length: u32,
    time_out_value: u32,
    data_buffer: *mut c_void,
    sense_info_offset: u32,
    cdb: [u8; 16],
}

/// `SCSI_PASS_THROUGH_DIRECT_WITH_BUFFER`: the passthrough header plus an
/// inline auto-sense buffer, used as the single in/out IOCTL buffer.
#[repr(C)]
struct ScsiPassThroughDirectWithBuffer {
    sptd: ScsiPassThroughDirect,
    filler: u32,
    sense_buf: [u8; 32],
}

/// Parses the SCSI sense triple `(key, asc, ascq)` from an auto-sense buffer.
fn parse_sense(buf: &[u8]) -> Sense {
    let key = buf.get(2).copied().unwrap_or(0) & 0x0f;
    let asc = buf.get(12).copied().unwrap_or(0);
    let ascq = buf.get(13).copied().unwrap_or(0);
    (key, asc, ascq)
}

/// Encodes a Rust string as a NUL-terminated UTF-16 path for the Win32 `W` API.
fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

// --- Device ---------------------------------------------------------------

/// A Sony mass-storage device reached through the Windows USBSTOR stack.
///
/// The device is addressed by its `\\.\X:` path; a fresh handle is opened and
/// closed around every SCSI command, mirroring the reference driver.
pub struct ScsiPassThroughDevice {
    path: Vec<u16>,
    /// Human-readable form of `path`, for diagnostics.
    display: String,
}

impl ScsiPassThroughDevice {
    /// Issues one SCSI command via `IOCTL_SCSI_PASS_THROUGH_DIRECT`.
    ///
    /// `data` is the direct-mapped transfer buffer: read into for
    /// [`SCSI_IOCTL_DATA_IN`], sent from for [`SCSI_IOCTL_DATA_OUT`], and empty
    /// otherwise. Returns the sense triple; a zero SCSI status maps to
    /// [`SENSE_OK`].
    fn send_scsi(&self, cdb: &[u8], direction: u8, data: &mut [u8]) -> Result<Sense> {
        if cdb.is_empty() || cdb.len() > 16 {
            return Err(Error::Protocol("invalid SCSI command length"));
        }

        let mut wb: ScsiPassThroughDirectWithBuffer = unsafe { core::mem::zeroed() };

        // Offset of the auto-sense buffer, computed from the real layout rather
        // than assumed (offset_of! is not available on the crate's MSRV).
        let sense_offset = {
            let base = &wb as *const _ as usize;
            let sense = &wb.sense_buf as *const _ as usize;
            (sense - base) as u32
        };

        wb.sptd.length = core::mem::size_of::<ScsiPassThroughDirect>() as u16;
        wb.sptd.data_in = direction;
        wb.sptd.data_transfer_length = data.len() as u32;
        wb.sptd.data_buffer =
            if data.is_empty() { ptr::null_mut() } else { data.as_mut_ptr() as *mut c_void };
        wb.sptd.cdb_length = cdb.len() as u8;
        wb.sptd.cdb[..cdb.len()].copy_from_slice(cdb);
        wb.sptd.time_out_value = 5;
        wb.sptd.sense_info_length = wb.sense_buf.len() as u8;
        wb.sptd.sense_info_offset = sense_offset;

        let handle = unsafe {
            CreateFileW(
                self.path.as_ptr(),
                GENERIC_READ | GENERIC_WRITE,
                FILE_SHARE_READ | FILE_SHARE_WRITE,
                ptr::null_mut(),
                OPEN_EXISTING,
                0,
                0,
            )
        };
        if handle == INVALID_HANDLE_VALUE {
            return Err(Error::Transfer(format!("cannot open {}", self.display)));
        }

        let size = core::mem::size_of::<ScsiPassThroughDirectWithBuffer>() as u32;
        let mut returned = 0u32;
        let buf_ptr = &mut wb as *mut ScsiPassThroughDirectWithBuffer as *mut c_void;
        // The SPTD header is METHOD_BUFFERED: the same buffer serves as input
        // and output, receiving the updated status and auto-sense in place. The
        // transfer payload travels separately through `data_buffer` (DIRECT).
        let ok = unsafe {
            DeviceIoControl(
                handle,
                IOCTL_SCSI_PASS_THROUGH_DIRECT,
                buf_ptr,
                size,
                buf_ptr,
                size,
                &mut returned,
                ptr::null_mut(),
            )
        };
        unsafe {
            CloseHandle(handle);
        }

        if ok == 0 {
            return Err(Error::Transfer("SCSI pass-through ioctl failed".into()));
        }

        if wb.sptd.scsi_status != 0 {
            let sense = parse_sense(&wb.sense_buf);
            if sense == SENSE_OK {
                // Non-zero status but empty sense: an unexpected hard error.
                return Err(Error::MscStatus { status: wb.sptd.scsi_status });
            }
            return Ok(sense);
        }
        Ok(SENSE_OK)
    }

    /// Sends a SCSI command with an OUT data phase.
    pub fn send_write_command(&mut self, command: &[u8], data: &[u8]) -> Result<Sense> {
        let mut buf = data.to_vec();
        self.send_scsi(command, SCSI_IOCTL_DATA_OUT, &mut buf)
    }

    /// Sends a SCSI command and reads `size` bytes in the IN data phase.
    pub fn send_read_command(&mut self, command: &[u8], size: usize) -> Result<(Sense, Vec<u8>)> {
        let mut buf = vec![0u8; size];
        let sense = self.send_scsi(command, SCSI_IOCTL_DATA_IN, &mut buf)?;
        Ok((sense, buf))
    }

    /// Sends a SCSI command with no data phase.
    pub fn send_command(&mut self, command: &[u8]) -> Result<Sense> {
        self.send_scsi(command, SCSI_IOCTL_DATA_UNSPECIFIED, &mut [])
    }

    /// Reads the SCSI INQUIRY vendor identification string, if any.
    fn inquiry_vendor(&self) -> Result<Option<String>> {
        // Standard 36-byte INQUIRY.
        let cdb = [0x12u8, 0, 0, 0, 36, 0];
        let mut buf = vec![0u8; 36];
        let sense = self.send_scsi(&cdb, SCSI_IOCTL_DATA_IN, &mut buf)?;
        if sense != SENSE_OK || buf.len() < 16 {
            return Ok(None);
        }
        let vendor: String = buf[8..16].iter().map(|&b| b as char).collect();
        Ok(Some(vendor.trim_matches(|c| c == ' ' || c == '\0').to_string()))
    }
}

impl ScsiExtCmdChannel for ScsiPassThroughDevice {
    fn scsi_write(&mut self, command: &[u8], data: &[u8]) -> Result<Sense> {
        self.send_write_command(command, data)
    }
    fn scsi_read(&mut self, command: &[u8], size: usize) -> Result<(Sense, Vec<u8>)> {
        self.send_read_command(command, size)
    }
}

impl SonyExtCmdTransport for ScsiPassThroughDevice {
    fn send_sony_ext_command(
        &mut self,
        cmd: u32,
        data: &[u8],
        buffer_size: usize,
    ) -> Result<Vec<u8>> {
        crate::command::send_msc_ext_command(self, cmd, data, buffer_size)
    }
}

/// Locates the first Sony mass-storage device reachable via SCSI pass-through.
///
/// Scans removable logical drives and probes each with a SCSI INQUIRY, matching
/// the reference `_listDevices` vendor filter. Returns `Ok(None)` when no Sony
/// mass-storage drive is present — typically because the camera is bound to a
/// WinUSB filter driver instead of USBSTOR, in which case the caller should
/// fall back to the raw Bulk-Only transport.
pub fn find_sony_scsi_device() -> Result<Option<ScsiPassThroughDevice>> {
    let mask = unsafe { GetLogicalDrives() };
    for i in 0..26u32 {
        if mask & (1 << i) == 0 {
            continue;
        }
        let letter = (b'A' + i as u8) as char;
        let root = wide(&format!("{letter}:\\"));
        let drive_type = unsafe { GetDriveTypeW(root.as_ptr()) };
        if drive_type != DRIVE_REMOVABLE {
            continue;
        }
        let display = format!("\\\\.\\{letter}:");
        let device = ScsiPassThroughDevice { path: wide(&display), display };
        if let Ok(Some(vendor)) = device.inquiry_vendor()
            && vendor.eq_ignore_ascii_case("sony")
        {
            return Ok(Some(device));
        }
    }
    Ok(None)
}

/// How many times [`wait_for_sony_scsi_device`] rescans the drive letters.
const MOUNT_ATTEMPTS: u32 = 20;

/// Pause between rescans.
const MOUNT_DELAY: Duration = Duration::from_millis(250);

/// Like [`find_sony_scsi_device`], but gives Windows time to mount the volume.
///
/// `USBSTOR` claims the camera as soon as it is plugged in, but the disk and
/// volume stacks assign drive letters a moment later — and a camera that is
/// still waking up delays that further. A scan started in between finds
/// nothing, and the caller would then fall back to raw endpoint access, which
/// USBSTOR refuses: an ordinary race turns into a driver complaint. Only call
/// this when `USBSTOR` is known to be the bound driver, since with WinUSB there
/// is no volume to wait for.
pub fn wait_for_sony_scsi_device() -> Result<Option<ScsiPassThroughDevice>> {
    for attempt in 0..MOUNT_ATTEMPTS {
        if let Some(device) = find_sony_scsi_device()? {
            return Ok(Some(device));
        }
        if attempt + 1 < MOUNT_ATTEMPTS {
            std::thread::sleep(MOUNT_DELAY);
        }
    }
    Ok(None)
}
