#![no_main]
//! Settings backup images, as read from the camera or loaded from disk.

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = pmca_core::backup::BackupFile::new(data);
});
