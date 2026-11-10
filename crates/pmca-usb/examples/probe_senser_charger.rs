//! Experiment: can a camera that only enumerates its charging personality be
//! pushed into service mode?
//!
//! A Sony body sets its USB personality from the main application. When that
//! application cannot start (a boot loop after a damaged library, for example)
//! the camera never reaches mass storage or MTP; Windows only ever sees the
//! HID "USB Charger" node, typically `PID_0994`, often for about a second per
//! boot attempt.
//!
//! The service-mode start request is a *vendor control transfer* (bmRequestType
//! 0x43, recipient Other), not a bulk command, so it needs nothing but the
//! device's control endpoint. Whether Sony's firmware honours it outside mass
//! storage is unknown: an earlier probe on the PTP interface was ignored
//! (`probe_senser_mtp`). This probe answers the same question for the charging
//! personality, which is the only one a boot-looping camera still offers.
//!
//! The probe sends one request per appearance of the node and then watches for
//! the service-mode product id. It writes nothing and never flashes anything;
//! the worst case is a stalled control transfer.
//!
//! On Windows the node is claimed by HIDClass, so WinUSB has to be bound first:
//!
//! ```text
//! pmca-console driver bind --id "USB\VID_054C&PID_0994"   (elevated)
//! cargo run -p pmca-usb --example probe_senser_charger -- 0994
//! pmca-console driver unbind                              (elevated)
//! ```

use std::time::{Duration, Instant};

use pmca_usb::discovery::{self, CameraMode};
use pmca_usb::nusb_backend::NusbBackend;
use pmca_usb::senser::VENDOR_REQUEST_START_SENSER;
use pmca_usb::sony::{SONY_ID_PRODUCT_SENSER, SONY_ID_VENDOR};
use pmca_usb::{UsbBackend, winusb_install};

/// Charging personality of the bodies measured so far (ILCE-6000, ILCE-7CR).
const DEFAULT_PRODUCT_ID: u16 = 0x0994;
/// How long to keep trying; a boot loop offers a new window every few seconds.
const RUN_FOR: Duration = Duration::from_secs(120);
const POLL: Duration = Duration::from_millis(250);

fn main() {
    let product_id = match std::env::args().nth(1) {
        Some(arg) => match u16::from_str_radix(arg.trim_start_matches("0x"), 16) {
            Ok(pid) => pid,
            Err(_) => return eprintln!("usage: probe_senser_charger [product-id-in-hex]"),
        },
        None => DEFAULT_PRODUCT_ID,
    };
    let id = format!(r"USB\VID_{SONY_ID_VENDOR:04X}&PID_{product_id:04X}");
    println!("target:   {id}");
    println!("watching: {} s, one start request per appearance", RUN_FOR.as_secs());

    let started = Instant::now();
    let mut attempts = 0usize;
    let mut sent = 0usize;
    let mut last_error = None;
    while started.elapsed() < RUN_FOR {
        if let Some(senser) = service_mode_reached() {
            println!("RESULT:   service mode reached after {attempts} attempts ({sent} sent)");
            println!("found:    {senser}");
            return;
        }
        let cameras = discovery::list_cameras().unwrap_or_default();
        let Some(camera) = cameras.into_iter().find(|c| c.device.product_id == product_id) else {
            std::thread::sleep(POLL);
            continue;
        };
        attempts += 1;
        match NusbBackend::open(&camera.device, 0) {
            Ok(mut backend) => {
                let (request, value, index) = VENDOR_REQUEST_START_SENSER;
                // A stall here is not a failure: the camera may accept the
                // request and re-enumerate before it answers.
                let result = backend.vendor_request_out(request, value, index, &[]);
                sent += 1;
                println!("attempt {attempts}: start request sent ({result:?})");
            }
            Err(e) => {
                if last_error.as_deref() != Some(e.to_string().as_str()) {
                    println!("attempt {attempts}: cannot open the device: {e}");
                    if cfg!(windows) {
                        println!("          bind WinUSB first (elevated):");
                        println!("          pmca-console driver bind --id \"{id}\"");
                    }
                    last_error = Some(e.to_string());
                }
            }
        }
        std::thread::sleep(POLL);
    }

    match service_mode_reached() {
        Some(senser) => println!("RESULT:   service mode reached, {senser}"),
        None => println!(
            "RESULT:   no service mode after {attempts} attempts ({sent} requests sent); \
             the charging personality ignores the start request"
        ),
    }
}

/// Report the service-mode device, whether or not it has a driver bound.
fn service_mode_reached() -> Option<String> {
    if let Ok(cameras) = discovery::list_cameras()
        && cameras.iter().any(|c| c.mode == CameraMode::Senser)
    {
        return Some("service mode is open".to_owned());
    }
    // Windows registers no interface for a driverless device, so `list_cameras`
    // cannot see it; the configuration manager still lists the node.
    SONY_ID_PRODUCT_SENSER
        .iter()
        .map(|p| format!(r"USB\VID_{SONY_ID_VENDOR:04X}&PID_{p:04X}"))
        .find(|n| winusb_install::device_present(n))
}
