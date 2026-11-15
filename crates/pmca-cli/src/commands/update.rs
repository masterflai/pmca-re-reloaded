//! `update`: download, verify and install a newer release of this program.
//!
//! Deliberately explicit and never automatic. It fetches the release built for
//! this platform from GitHub, checks its detached minisign signature against a
//! public key compiled into the binary, and only then replaces the running
//! executable in place. The signature is verified before anything from the
//! download is unpacked or run, so a tampered or man-in-the-middled file is
//! rejected rather than executed.
//!
//! Only the downloadable archive builds update themselves. A copy put in place
//! by a package manager (cargo, Homebrew, a distribution package) or sitting in
//! a directory this user cannot write is left to whoever installed it.

use std::io::Read;

use crate::*;

/// The raw minisign public key line of our release-signing key, the same key
/// the desktop app's updater verifies against (key id `0DD71D113B53A14C`).
const UPDATE_PUBKEY: &str = "RWRMoVM7ER3XDcCBN6gKQoRMYu2wiJHpjWXwctIVKIoocjmYwYktrOPl";

/// GitHub always serves the newest release's assets from here, so the version
/// to fetch never has to be resolved by hand.
const LATEST_DOWNLOAD: &str =
    "https://github.com/masterflai/pmca-re-reloaded/releases/latest/download";

/// A console build is a few megabytes; this is a generous ceiling.
const MAX_ARCHIVE_BYTES: u64 = 64 * 1024 * 1024;

/// A minisign signature file is a few hundred bytes.
const MAX_SIGNATURE_BYTES: u64 = 8 * 1024;

/// The release asset built for the running platform.
#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
const ASSET: Option<&str> = Some("pmca-console-x86_64-windows.zip");
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
const ASSET: Option<&str> = Some("pmca-console-x86_64-linux.tar.gz");
#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
const ASSET: Option<&str> = Some("pmca-console-x86_64-macos.tar.gz");
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
const ASSET: Option<&str> = Some("pmca-console-aarch64-macos.tar.gz");
#[cfg(not(any(
    all(target_os = "windows", target_arch = "x86_64"),
    all(target_os = "linux", target_arch = "x86_64"),
    all(target_os = "macos", target_arch = "x86_64"),
    all(target_os = "macos", target_arch = "aarch64"),
)))]
const ASSET: Option<&str> = None;

/// The program's file name inside the release archive.
#[cfg(windows)]
const BINARY_NAME: &str = "pmca-console.exe";
#[cfg(not(windows))]
const BINARY_NAME: &str = "pmca-console";

/// Check for a newer release and, with the user's go-ahead, install it.
pub(crate) fn update_command(l: &Localizer, yes: bool) -> Result<(), Box<dyn std::error::Error>> {
    let Some(asset) = ASSET else {
        return Err(l.tr("update-unsupported-platform").into());
    };
    let exe = std::env::current_exe().map_err(|e| e.to_string())?;
    if let Some(reason) = managed_install_reason(&exe) {
        return Err(l.tr(reason).into());
    }

    println!("{}", l.tr("update-checking"));
    let Some(update) = pmca_app::update::check()? else {
        println!("{}", l.tr_arg("update-none", "version", env!("CARGO_PKG_VERSION")));
        return Ok(());
    };
    println!(
        "{}",
        l.tr_args(
            "update-available-cli",
            &[("current", &update.current), ("latest", &update.latest)]
        )
    );

    if !yes && !confirmed(l) {
        println!("{}", l.tr("update-cancelled"));
        return Ok(());
    }

    println!("{}", l.tr("update-downloading"));
    let archive =
        pmca_market::appstore::download(&format!("{LATEST_DOWNLOAD}/{asset}"), MAX_ARCHIVE_BYTES)?;
    let signature = pmca_market::appstore::download(
        &format!("{LATEST_DOWNLOAD}/{asset}.sig"),
        MAX_SIGNATURE_BYTES,
    )?;

    println!("{}", l.tr("update-verifying"));
    verify(&archive, &signature).map_err(|_| l.tr("update-verify-failed"))?;

    let binary = extract_binary(&archive).ok_or_else(|| l.tr("update-extract-failed"))?;

    println!("{}", l.tr("update-applying"));
    apply(&exe, &binary)?;
    println!("{}", l.tr_arg("update-done", "version", &update.latest));
    Ok(())
}

/// Ask once before replacing the program; `y`, `yes` or the localized word count.
fn confirmed(l: &Localizer) -> bool {
    prompt(&l.tr("update-confirm"));
    read_stdin_line().is_some_and(|line| {
        let typed = line.trim().to_lowercase();
        typed == l.tr("update-yes-word").to_lowercase() || typed == "yes" || typed == "y"
    })
}

/// Verify the archive against the detached signature with the built-in key.
///
/// The `.sig` asset is the base64 of a minisign signature file, matching what
/// the desktop app's updater consumes.
fn verify(archive: &[u8], signature: &[u8]) -> std::result::Result<(), String> {
    use base64::prelude::*;
    use minisign_verify::{PublicKey, Signature};

    let decoded = BASE64_STANDARD.decode(trim(signature)).map_err(|e| e.to_string())?;
    let text = std::str::from_utf8(&decoded).map_err(|e| e.to_string())?;
    let signature = Signature::decode(text).map_err(|e| e.to_string())?;
    let public_key = PublicKey::from_base64(UPDATE_PUBKEY).map_err(|e| e.to_string())?;
    public_key.verify(archive, &signature, true).map_err(|e| e.to_string())
}

/// Drop surrounding ASCII whitespace so the base64 decoder sees only the payload.
fn trim(bytes: &[u8]) -> &[u8] {
    let start = bytes.iter().position(|b| !b.is_ascii_whitespace()).unwrap_or(0);
    let end = bytes.iter().rposition(|b| !b.is_ascii_whitespace()).map_or(start, |i| i + 1);
    &bytes[start..end]
}

/// Pull the program out of the verified archive.
#[cfg(windows)]
fn extract_binary(archive: &[u8]) -> Option<Vec<u8>> {
    let mut zip = zip::ZipArchive::new(std::io::Cursor::new(archive)).ok()?;
    for i in 0..zip.len() {
        let mut file = zip.by_index(i).ok()?;
        let name = file.name().ok()?.to_string();
        if base_name(&name) == BINARY_NAME {
            let mut buf = Vec::new();
            file.read_to_end(&mut buf).ok()?;
            return Some(buf);
        }
    }
    None
}

/// Pull the program out of the verified archive.
#[cfg(not(windows))]
fn extract_binary(archive: &[u8]) -> Option<Vec<u8>> {
    let decoder = flate2::read::GzDecoder::new(std::io::Cursor::new(archive));
    let mut tar = tar::Archive::new(decoder);
    for entry in tar.entries().ok()? {
        let mut entry = entry.ok()?;
        let name = entry.path().ok()?.file_name()?.to_str()?.to_string();
        if name == BINARY_NAME {
            let mut buf = Vec::new();
            entry.read_to_end(&mut buf).ok()?;
            return Some(buf);
        }
    }
    None
}

/// The last path component, with either separator.
#[cfg(windows)]
fn base_name(path: &str) -> &str {
    path.rsplit(['/', '\\']).next().unwrap_or(path)
}

/// Write the new program beside the current one and swap it in.
fn apply(exe: &std::path::Path, binary: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    // Same directory as the running program, so the swap is a rename on one
    // filesystem rather than a copy that could half-finish.
    let staged = exe.with_file_name(".pmca-console-update");
    std::fs::write(&staged, binary)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&staged, std::fs::Permissions::from_mode(0o755))?;
    }
    let result = self_replace::self_replace(&staged);
    let _ = std::fs::remove_file(&staged);
    result?;
    Ok(())
}

/// A key for the message to show, when this copy must not replace itself.
fn managed_install_reason(exe: &std::path::Path) -> Option<&'static str> {
    let lower = exe.to_string_lossy().to_lowercase();
    // Package managers put the binary under a path of their own.
    if [".cargo", "homebrew", "cellar", "linuxbrew", "scoop"].iter().any(|n| lower.contains(n)) {
        return Some("update-managed-install");
    }
    #[cfg(unix)]
    if ["/usr/", "/opt/", "/bin/"].iter().any(|n| lower.starts_with(n)) {
        return Some("update-managed-install");
    }
    match exe.parent() {
        Some(dir) if directory_is_writable(dir) => None,
        _ => Some("update-not-writable"),
    }
}

/// Whether a file can be created in `dir`, so the swap would succeed.
fn directory_is_writable(dir: &std::path::Path) -> bool {
    let probe = dir.join(".pmca-console-write-probe");
    match std::fs::File::create(&probe) {
        Ok(_) => {
            let _ = std::fs::remove_file(&probe);
            true
        }
        Err(_) => false,
    }
}
