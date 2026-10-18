//! Entry point of the PMCA-RE RELOADED window; everything else is in the
//! library half of this crate.

// Hide the console window on Windows release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![warn(missing_docs)]

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Some(code) = pmca_app::drivers::run_helper_if_requested(&args) {
        std::process::exit(code);
    }
    if let Some(socket) = pmca_app::broker::broker_socket(&args) {
        std::process::exit(pmca_gui_tauri::run_broker(socket));
    }
    pmca_gui_tauri::run()
}
