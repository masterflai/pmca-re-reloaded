//! macOS diagnostic: `sudo ./pmca-macos-probe` with the camera connected. It
//! tries every known way into the camera's USB interfaces and writes a report.

fn main() {
    #[cfg(target_os = "macos")]
    std::process::exit(pmca_usb::macos_probe::run());
    #[cfg(not(target_os = "macos"))]
    eprintln!("This probe runs on macOS only.");
}
