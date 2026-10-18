//! Experiment: can a camera in MTP mode be switched into service mode directly?
//!
//! Service mode is entered from mass storage today: a vendor start request,
//! then a short handshake on the mass-storage bulk endpoints. macOS never
//! releases a mass-storage interface, but it does release the PTP one. This
//! probe sends the same request and handshake on the PTP endpoints and reports
//! whether the camera comes back in service mode.
//!
//! On Windows the PTP interface needs WinUSB first; the hardware id is printed
//! by this probe when the binding is missing:
//!
//! ```text
//! pmca-console driver bind --id "USB\VID_054C&PID_xxxx"   (elevated)
//! cargo run -p pmca-usb --example probe_senser_mtp
//! pmca-console driver unbind                              (elevated)
//! ```
//!
//! Needs no service-mode keys: the handshake before the switch is the short
//! variant. A camera that reaches service mode and is reachable is asked to
//! leave it again; one that is only visible as a driverless node leaves it on a
//! power cycle.

use std::time::Duration;

use pmca_usb::discovery::{self, CameraMode};
use pmca_usb::nusb_backend::NusbBackend;
use pmca_usb::senser::SenserAuthDevice;
use pmca_usb::sony::{SONY_ID_PRODUCT_SENSER, SONY_ID_VENDOR};
use pmca_usb::{class, winusb_install};

const POLLS: usize = 40;
const DELAY: Duration = Duration::from_millis(500);

fn main() {
    let cameras = match discovery::list_cameras() {
        Ok(c) => c,
        Err(e) => return eprintln!("enumeration failed: {e}"),
    };
    let Some(camera) = cameras.into_iter().find(|c| c.mode == CameraMode::Mtp) else {
        return eprintln!("no camera in MTP mode; set the camera's USB connection to MTP");
    };
    let id =
        format!(r"USB\VID_{:04X}&PID_{:04X}", camera.device.vendor_id, camera.device.product_id);
    println!("camera:   {id}, driver {:?}", camera.device.os_driver());

    let (backend, iface) = match NusbBackend::open_class(&camera.device, class::PTP) {
        Ok(opened) => opened,
        Err(e) => {
            eprintln!("cannot open the PTP interface: {e}");
            if cfg!(windows) {
                eprintln!("bind WinUSB first: pmca-console driver bind --id \"{id}\"");
            }
            return;
        }
    };
    println!(
        "opened:   interface {}, bulk in {:#04x}, out {:#04x}",
        iface.interface_number, iface.ep_in, iface.ep_out
    );

    let mut auth =
        SenserAuthDevice::new(backend, iface.ep_in, iface.ep_out, camera.device.product_id);
    auth.start();
    println!("start:    request sent");
    match auth.authenticate() {
        Ok(()) => println!("handshake: accepted"),
        Err(e) => println!("handshake: {e}"),
    }
    drop(auth);

    println!(
        "waiting:  up to {} s for service mode",
        POLLS as u64 * DELAY.as_millis() as u64 / 1000
    );
    for poll in 0..POLLS {
        std::thread::sleep(DELAY);
        if let Ok(cameras) = discovery::list_cameras()
            && let Some(senser) = cameras.into_iter().find(|c| c.mode == CameraMode::Senser)
        {
            println!("RESULT:   service mode reached after {} s (reachable)", seconds(poll));
            let left = discovery::leave_service_mode(&senser);
            println!("leave:    stop request sent ({left:?})");
            return;
        }
        // Windows has no driver for the service-mode device, so it only shows
        // up as a node until something binds one.
        let node = SONY_ID_PRODUCT_SENSER
            .iter()
            .map(|p| format!(r"USB\VID_{SONY_ID_VENDOR:04X}&PID_{p:04X}"))
            .find(|n| winusb_install::device_present(n));
        if let Some(node) = node {
            println!("RESULT:   service mode reached after {} s, node {node}", seconds(poll));
            println!("leave:    switch the camera off and on again");
            return;
        }
    }
    let modes: Vec<CameraMode> =
        discovery::list_cameras().unwrap_or_default().into_iter().map(|c| c.mode).collect();
    println!("RESULT:   no service mode; cameras now: {modes:?}");
}

fn seconds(poll: usize) -> f32 {
    (poll + 1) as f32 * DELAY.as_secs_f32()
}
