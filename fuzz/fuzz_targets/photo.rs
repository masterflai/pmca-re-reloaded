#![no_main]
//! Photos (JPEG, ARW, HEIF) read for the shutter count.

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = pmca_core::photo::read(data);
});
