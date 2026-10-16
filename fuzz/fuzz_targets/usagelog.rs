#![no_main]
//! The camera's usage log.

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = pmca_core::usagelog::parse(data);
});
