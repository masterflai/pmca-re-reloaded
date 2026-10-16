//! Capability probe: which Sony external-command plug-ins does this camera have?
//!
//! The external-command set is split into plug-ins (`libInfra*.so` on the
//! camera) addressed by the first half of a [`SonyCmd`]. A camera that lacks a
//! plug-in refuses **every** sub-command of it, including the capability query
//! itself — which is exactly what separates "this model cannot do that" from
//! "we framed the command wrongly". Run this before concluding that a command
//! is broken:
//!
//! ```text
//! cargo run --release -p pmca-usb --example probe_extcmd
//! ```
//!
//! Read-only: every command listed here is a getter.

use pmca_usb::discovery;
use pmca_usb::sony_extcmd::{DEFAULT_BUFFER_SIZE, SonyCmd, SonyExtCmdCamera};

/// Getters worth asking about, as `(command, label)`.
///
/// Sub-command 1 of a plug-in is its `GetSupportedCommandIds`, so probing that
/// alone establishes whether the plug-in exists at all.
const PROBES: &[(SonyCmd, &str)] = &[
    ((1, 1), "DevInfoSender.GetModelInfo"),
    ((1, 2), "DevInfoSender.GetSupportedCommandIds"),
    ((2, 1), "KikiLogSender.InitKikiLog"),
    ((4, 1), "ExtBackupCommunicator.GetSupportedCommandIds"),
    ((5, 1), "ScalarExtCmdPlugIn.GetSupportedCommandIds"),
    ((6, 1), "LensCommunicator.GetSupportedCommandIds"),
    ((7, 1), "NetworkServiceInfo.GetSupportedCommandIds"),
    ((7, 3), "NetworkServiceInfo.GetLiveStreamingServiceInfo"),
    ((7, 5), "NetworkServiceInfo.GetLiveStreamingSNSInfo"),
    ((7, 7), "NetworkServiceInfo.GetWifiAPInfo"),
    ((7, 10), "NetworkServiceInfo.GetMultiWifiAPInfo"),
];

/// Bytes of each response to show; enough for a count plus its id list.
const PREVIEW: usize = 24;

fn main() {
    let camera = match discovery::find_single_camera() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("no camera: {e}");
            return;
        }
    };
    println!("mode: {:?}", camera.mode);

    let transport = match discovery::open_extcmd_transport(&camera) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("cannot open transport: {e}");
            return;
        }
    };
    let mut cam = SonyExtCmdCamera::new(transport);

    for (cmd, label) in PROBES {
        // A refusal is a result, not a reason to stop: the point is the whole
        // map, so one missing plug-in must not hide the rest.
        match cam.send_command(*cmd, &[], DEFAULT_BUFFER_SIZE, DEFAULT_BUFFER_SIZE) {
            Ok(data) => {
                let preview: Vec<String> =
                    data.iter().take(PREVIEW).map(|b| format!("{b:02x}")).collect();
                println!(
                    "({}, {:2}) {label:50} OK  {} bytes: {}",
                    cmd.0,
                    cmd.1,
                    data.len(),
                    preview.join(" ")
                );
            }
            Err(e) => println!("({}, {:2}) {label:50} ERR {e}", cmd.0, cmd.1),
        }
    }
}
