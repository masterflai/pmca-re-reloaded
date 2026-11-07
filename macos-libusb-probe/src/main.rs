//! Single-file diagnostic: does the *real* libusb open a Sony camera's
//! mass-storage interface as root on this Mac?
//!
//! Background: our own app reaches mass storage on macOS through IOUSBHost
//! device capture, which on macOS 26 hits an `IOUSBHostInterface` init wall.
//! A separate question is whether official libusb — whose darwin backend uses
//! its own `USBDeviceReEnumerate` capture path — can still do it there. The
//! Python original answers this with `pmca-console.py info -d libusb` under
//! Homebrew + Python + git, which is far too much for a non-technical tester.
//!
//! This binary bundles libusb (`rusb` "vendored", statically linked) and does
//! only that one thing the faithful way: detach the kernel/system driver from
//! the interface (libusb capture), claim it, then send a SCSI INQUIRY and the
//! Sony camera-info command. It is read-only — it changes nothing on the
//! camera. Run it with `sudo ./pmca-libusb-probe`.
//!
//! The report (`pmca-libusb-probe.txt`, written next to the binary) and the
//! final `RESULT:` line say whether libusb reached mass storage or hit the
//! same wall as our app.

use std::fmt::Write as _;
use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;

use pmca_usb::msc::MscDevice;
use pmca_usb::sony::SONY_ID_VENDOR;
use pmca_usb::sony_extcmd::SonyExtCmdCamera;
use pmca_usb::{Error, Result, UsbBackend, class};

/// Identifies this probe's logic in a report.
const REVISION: &str = "libusb probe 1";

/// Matches the Python original's `default_timeout = 5000` (ms).
const TIMEOUT: Duration = Duration::from_secs(5);

type Handle = rusb::DeviceHandle<rusb::GlobalContext>;
type UsbDevice = rusb::Device<rusb::GlobalContext>;

/// A [`UsbBackend`] that moves bulk bytes through real libusb.
struct Libusb {
    handle: Handle,
}

impl UsbBackend for Libusb {
    fn read(&mut self, ep: u8, length: usize) -> Result<Vec<u8>> {
        let mut buf = vec![0u8; length];
        match self.handle.read_bulk(ep, &mut buf, TIMEOUT) {
            Ok(n) => {
                buf.truncate(n);
                Ok(buf)
            }
            Err(rusb::Error::Pipe) => Err(Error::Stall),
            Err(e) => Err(Error::Transfer(format!("libusb read: {e}"))),
        }
    }

    fn write(&mut self, ep: u8, data: &[u8]) -> Result<()> {
        match self.handle.write_bulk(ep, data, TIMEOUT) {
            Ok(_) => Ok(()),
            Err(rusb::Error::Pipe) => Err(Error::Stall),
            Err(e) => Err(Error::Transfer(format!("libusb write: {e}"))),
        }
    }

    fn clear_halt(&mut self, ep: u8) -> Result<()> {
        self.handle.clear_halt(ep).map_err(|e| Error::Transfer(format!("libusb clear_halt: {e}")))
    }
}

fn main() {
    let mut report = String::new();
    let code = run(&mut report);

    let path = report_path();
    match std::fs::write(&path, &report) {
        Ok(()) => {
            println!("\nReport written to {}", path.display());
            restore_owner(&path);
        }
        Err(e) => eprintln!("could not write {}: {e}", path.display()),
    }
    std::process::exit(code);
}

/// Runs the probe, appending every step to `r`, and returns the process exit
/// code (0 only when libusb delivered camera info over mass storage).
fn run(r: &mut String) -> i32 {
    line(r, &format!("PMCA-RE RELOADED - libusb mass-storage probe ({REVISION})"));
    environment(r);

    let v = rusb::version();
    line(r, &format!("libusb version: {}.{}.{}.{}", v.major(), v.minor(), v.micro(), v.nano()));

    let devices = match rusb::devices() {
        Ok(d) => d,
        Err(e) => {
            line(r, &format!("cannot list USB devices: {e}"));
            return 2;
        }
    };

    let sony: Vec<UsbDevice> = devices
        .iter()
        .filter(|d| {
            d.device_descriptor().map(|dd| dd.vendor_id() == SONY_ID_VENDOR).unwrap_or(false)
        })
        .collect();

    if sony.is_empty() {
        line(r, "No Sony device found. Connect the camera with a data cable (no hub).");
        return 3;
    }

    // Prefer a device that exposes a mass-storage interface.
    if let Some((device, iface, ep_in, ep_out)) =
        sony.iter().find_map(|d| msc_interface(d).map(|(i, a, b)| (d, i, a, b)))
    {
        let pid = device.device_descriptor().map(|dd| dd.product_id()).unwrap_or(0);
        line(
            r,
            &format!(
                "Sony device {SONY_ID_VENDOR:04x}:{pid:04x}, mass-storage interface {iface}, bulk in 0x{ep_in:02x} / out 0x{ep_out:02x}"
            ),
        );
        return probe(r, device, iface, ep_in, ep_out);
    }

    if sony.iter().any(|d| has_interface_class(d, class::PTP)) {
        line(
            r,
            "The camera is in MTP mode. Set its USB connection to \"Mass Storage\" on the camera (MENU > Setup > USB Connection), switch it off and on, then run this again.",
        );
        return 4;
    }

    line(
        r,
        "A Sony device is present but offers no mass-storage interface. Set the camera's USB connection to \"Mass Storage\" and reconnect.",
    );
    4
}

/// Opens the device with libusb, detaches the system driver, claims the
/// interface and reads the camera's identity.
fn probe(r: &mut String, device: &UsbDevice, iface: u8, ep_in: u8, ep_out: u8) -> i32 {
    let handle = match device.open() {
        Ok(h) => h,
        Err(rusb::Error::Access) => {
            line(r, "open failed: access denied. Run it with sudo: sudo ./pmca-libusb-probe");
            return 5;
        }
        Err(e) => {
            line(r, &format!("open failed: {e}"));
            return 5;
        }
    };

    // The faithful libusb sequence the Python original uses: detach the
    // kernel/system driver (on macOS this is the USBDeviceReEnumerate capture),
    // then claim. `set_auto_detach_kernel_driver` asks libusb to do the detach
    // itself at claim time; the explicit detach below covers backends where
    // auto-detach is not wired up.
    let _ = handle.set_auto_detach_kernel_driver(true);
    match handle.kernel_driver_active(iface) {
        Ok(true) => {
            line(r, "a system driver holds the mass-storage interface (expected on macOS)");
            match handle.detach_kernel_driver(iface) {
                Ok(()) => line(r, "detach_kernel_driver (libusb capture): OK"),
                Err(e) => line(r, &format!("detach_kernel_driver (libusb capture): {e}")),
            }
        }
        Ok(false) => line(r, "no system driver reported on the interface"),
        Err(e) => line(r, &format!("kernel_driver_active: {e}")),
    }

    // The decisive step.
    match handle.claim_interface(iface) {
        Ok(()) => line(r, "claim_interface: OK  <<< libusb reached the mass-storage interface"),
        Err(e) => {
            line(r, &format!("claim_interface FAILED: {e}"));
            line(r, "");
            line(r, "RESULT: real libusb could NOT open mass storage as root on this macOS.");
            line(r, "        Same wall as our own app -> this macOS is blocked for direct");
            line(r, "        USB mass storage; the route is tweaks on the camera itself.");
            return 6;
        }
    }

    let mut msc = MscDevice::new(Libusb { handle }, ep_in, ep_out);
    match msc.get_device_info() {
        Ok(info) => line(r, &format!("INQUIRY OK: {} {}", info.manufacturer, info.model)),
        Err(e) => {
            line(r, &format!("INQUIRY failed: {e:?}"));
            line(r, "");
            line(r, "RESULT: libusb claimed the interface but the Bulk-Only transfer failed.");
            line(r, "        Copy this whole report back.");
            return 7;
        }
    }

    let mut camera = SonyExtCmdCamera::new(msc);
    match camera.get_camera_info() {
        Ok(info) => {
            // model_code is a product code, not the camera's serial number.
            line(
                r,
                &format!("CAMERA INFO OK: model {} (code {})", info.model_name, info.model_code),
            );
            line(r, "");
            line(r, "RESULT: real libusb CAN open mass storage as root on this macOS.");
            line(r, "        The limit is in our own code path, not in libusb -> port");
            line(r, "        libusb's capture path for the Mac mass-storage mode.");
            0
        }
        Err(e) => {
            line(r, &format!("camera info failed: {e:?}"));
            line(r, "");
            line(r, "RESULT: INQUIRY worked but the Sony camera-info command failed.");
            line(r, "        Copy this whole report back.");
            8
        }
    }
}

/// Returns the first mass-storage interface and its bulk in/out endpoints.
fn msc_interface(device: &UsbDevice) -> Option<(u8, u8, u8)> {
    let config = device.active_config_descriptor().ok()?;
    for interface in config.interfaces() {
        for desc in interface.descriptors() {
            if desc.class_code() != class::MSC {
                continue;
            }
            let mut ep_in = None;
            let mut ep_out = None;
            for ep in desc.endpoint_descriptors() {
                if ep.transfer_type() != rusb::TransferType::Bulk {
                    continue;
                }
                match ep.direction() {
                    rusb::Direction::In => ep_in = Some(ep.address()),
                    rusb::Direction::Out => ep_out = Some(ep.address()),
                }
            }
            if let (Some(i), Some(o)) = (ep_in, ep_out) {
                return Some((desc.interface_number(), i, o));
            }
        }
    }
    None
}

/// Whether the device exposes an interface of the given class.
fn has_interface_class(device: &UsbDevice, class: u8) -> bool {
    device
        .active_config_descriptor()
        .ok()
        .map(|config| config.interfaces().any(|i| i.descriptors().any(|d| d.class_code() == class)))
        .unwrap_or(false)
}

/// Appends the macOS and kernel version to the report, for the record.
fn environment(r: &mut String) {
    for (label, program, args) in
        [("macOS (sw_vers)", "sw_vers", &[][..]), ("kernel (uname -a)", "uname", &["-a"][..])]
    {
        match Command::new(program).args(args).output() {
            Ok(o) => line(
                r,
                &format!("=== {label} ===\n{}", String::from_utf8_lossy(&o.stdout).trim_end()),
            ),
            Err(e) => line(r, &format!("=== {label} === (not available: {e})")),
        }
    }
}

/// Where the report is written: next to the binary's working directory.
fn report_path() -> PathBuf {
    PathBuf::from("pmca-libusb-probe.txt")
}

/// Under `sudo` the report would be owned by root; hand it back to the user who
/// ran `sudo` so they can read and send it without more commands.
fn restore_owner(path: &PathBuf) {
    if let Ok(user) = std::env::var("SUDO_USER") {
        let _ = Command::new("chown").arg(&user).arg(path).status();
    }
}

/// Prints a line live and records it in the report.
fn line(r: &mut String, s: &str) {
    println!("{s}");
    let _ = writeln!(r, "{s}");
}
