//! Version information and icon for `pmca-console.exe`.
//!
//! Code signing requires product name and version in the executable, the same
//! values the window's executable carries from `tauri.conf.json`.

fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let mut res = winresource::WindowsResource::new();
    res.set_icon("../pmca-gui/src-tauri/icons/icon.ico")
        .set("ProductName", "PMCA-RE RELOADED")
        .set("FileDescription", "PMCA-RE RELOADED command line")
        .set("CompanyName", "masterflai")
        .set("LegalCopyright", "Copyright (c) 2015 ma1co, (c) 2026 masterflai. MIT License.")
        .set("OriginalFilename", "pmca-console.exe");
    if let Err(e) = res.compile() {
        panic!("embedding the Windows version resource failed: {e}");
    }
}
