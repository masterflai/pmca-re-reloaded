#![no_main]
//! Firmware update files (`.dat`, UFU container).

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = pmca_core::firmware::read_dat(data);
});
