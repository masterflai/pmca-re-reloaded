#![no_main]
//! Translation catalogues uploaded through the settings page.

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(text) = std::str::from_utf8(data) {
        let source = pmca_i18n::catalog::normalize(text);
        let _ = pmca_i18n::catalog::validate_for("en-US", &source);
    }
});
