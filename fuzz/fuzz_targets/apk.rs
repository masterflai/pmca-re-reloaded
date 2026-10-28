#![no_main]
//! APK files chosen by the user or downloaded from the app store, and the
//! binary XML manifest inside them.

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = pmca_apk::read(data);
    let _ = pmca_apk::verify_signature(data);
    let _ = pmca_apk::axml::parse(data);
});
