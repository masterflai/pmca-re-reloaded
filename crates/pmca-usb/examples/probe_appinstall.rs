//! Diagnostic: what does a camera in app-install mode offer, and can it be
//! made to leave that mode over USB?
//!
//! ```text
//! cargo run -p pmca-usb --example probe_appinstall                # list only
//! cargo run -p pmca-usb --example probe_appinstall -- 1003 [p..]  # no data phase
//! cargo run -p pmca-usb --example probe_appinstall -- r940a [p..] # IN data phase
//! ```
//!
//! The operation code is hex, the parameters decimal. Without an operation the
//! probe is read-only.

#[cfg(windows)]
use pmca_usb::mtp::PtpCommands;
#[cfg(windows)]
use pmca_usb::sony::is_sony_mtp_app_install_camera;

fn main() {
    #[cfg(windows)]
    {
        let camera = match pmca_usb::wpd::open_camera(is_sony_mtp_app_install_camera) {
            Ok(Some(c)) => c,
            Ok(None) => {
                eprintln!("no camera in app-install mode");
                return;
            }
            Err(e) => {
                eprintln!("cannot open: {e}");
                return;
            }
        };
        let info = &camera.info;
        println!("model: {} / {}", info.manufacturer, info.model);
        println!("vendor extension: {}", info.vendor_extension);
        let ops: Vec<String> =
            info.operations_supported.iter().map(|op| format!("{op:04x}")).collect();
        println!("operations ({}): {}", ops.len(), ops.join(" "));

        let mut device = camera.device;
        let args: Vec<String> = std::env::args().skip(1).collect();
        if let Some(arg) = args.first() {
            // `r` prefix: the operation has an IN data phase.
            let (read, hex) = match arg.strip_prefix('r') {
                Some(rest) => (true, rest),
                None => (false, arg.as_str()),
            };
            let Ok(code) = u16::from_str_radix(hex.trim_start_matches("0x"), 16) else {
                eprintln!("not a hex operation code: {arg}");
                return;
            };
            let params: Vec<u32> = args[1..].iter().filter_map(|p| p.parse().ok()).collect();
            if read {
                match device.send_read_command_opt(code, &params) {
                    Ok((rc, data)) => {
                        let hex: Vec<String> = data.iter().map(|b| format!("{b:02x}")).collect();
                        println!(
                            "op {code:04x} {params:?} -> response {rc:04x}, {} bytes: {}",
                            data.len(),
                            hex.join(" ")
                        );
                    }
                    Err(e) => println!("op {code:04x} {params:?} -> error {e}"),
                }
            } else {
                match device.send_command(code, &params) {
                    Ok(rc) => println!("op {code:04x} {params:?} -> response {rc:04x}"),
                    Err(e) => println!("op {code:04x} {params:?} -> error {e}"),
                }
            }
        }
    }
    #[cfg(not(windows))]
    eprintln!("this probe uses the Windows WPD transport");
}
