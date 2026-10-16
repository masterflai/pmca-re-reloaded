#![no_main]
//! Backup images as the function analysis reads them.

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = pmca_platform::analysis::Snapshot::parse("fuzz", data);
});
