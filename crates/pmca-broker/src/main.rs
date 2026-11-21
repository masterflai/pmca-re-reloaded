//! The macOS window's elevated helper: started through `sudo`, it answers the
//! window's requests that need root (see `pmca_app::broker`). `sudo` also runs
//! it as its password helper, which shows the password dialog.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Some(socket) = pmca_app::broker::broker_socket(&args) {
        std::process::exit(pmca_app::elevated::serve_window(socket));
    }
    #[cfg(unix)]
    if let Some(code) = pmca_app::broker::run_askpass_if_requested() {
        std::process::exit(code);
    }
    eprintln!("pmca-broker is started by the PMCA-RE RELOADED window, not by hand.");
    std::process::exit(2);
}
