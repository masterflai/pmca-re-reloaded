//! The Sony-derived key material the program does not ship.
//!
//! The keys are supplied by the user from a source of their own choosing. This
//! module loads them from the user's profile at start-up, imports them from a
//! URL the user enters, verifies every byte against the digests in
//! [`pmca_core::keystore`], and reports what is present. No source is ever
//! suggested, and nothing is written back to any network.

use std::io::{Cursor, Read};
use std::path::{Path, PathBuf};

use pmca_core::keystore::{self, KeyClass};
use pmca_i18n::Localizer;
use serde::Serialize;

use crate::dto::{ProgressEvent, ProgressKind};
use crate::{Error, ProgressSink, Result};

/// Upper bound for a single downloaded source.
const MAX_IMPORT_BYTES: u64 = 64 * 1024 * 1024;

/// Upper bound for one file read out of an archive.
const MAX_ENTRY_BYTES: u64 = 8 * 1024 * 1024;

/// Whether one class of keys is loaded.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct KeyClassStatus {
    /// Stable slug of the class (`senser`, `cic`, `spk`).
    pub slug: String,
    /// Whether every key in the class is present.
    pub present: bool,
}

/// What the program holds of the key material.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct KeyStatus {
    /// Whether every class is present.
    pub complete: bool,
    /// One entry per class, in a stable order.
    pub classes: Vec<KeyClassStatus>,
}

/// The current state of the key store.
#[must_use]
pub fn status() -> KeyStatus {
    KeyStatus {
        complete: keystore::complete(),
        classes: KeyClass::all()
            .into_iter()
            .map(|c| KeyClassStatus {
                slug: c.slug().to_string(),
                present: keystore::class_present(c),
            })
            .collect(),
    }
}

/// Where imported keys are kept, under the user's data folder.
#[must_use]
pub fn key_dir() -> Option<PathBuf> {
    Some(pmca_i18n::catalog::user_data_dir()?.join("keys"))
}

/// Load every key the profile holds, verifying each against its digest.
///
/// Infallible on purpose: a missing folder, an unreadable file or a tampered
/// key simply leaves that key absent, which the rest of the program already
/// handles. Called once at start-up.
pub fn load_from_disk() {
    let Some(dir) = key_dir() else { return };
    for id in keystore::ALL {
        let path = dir.join(format!("{}.key", id.slug()));
        if let Ok(bytes) = std::fs::read(&path) {
            // `install` checks the digest; a file that does not match is ignored.
            keystore::install(&bytes);
        }
    }
}

/// Import key material from the sources the user entered, then persist it.
///
/// A source is an https URL, downloaded, or the path of a local file. Either is
/// scanned for key material (as source code, as a decimal integer, or as a
/// binary file or archive), and every match is verified and kept. Returns the
/// state afterwards.
pub fn import_from_urls(
    urls: &[String],
    l: &Localizer,
    sink: &mut dyn ProgressSink,
) -> Result<KeyStatus> {
    for url in urls {
        let url = url.trim();
        if url.is_empty() {
            continue;
        }
        sink.emit(ProgressEvent::step(ProgressKind::Info, l.tr("keys-fetching")));
        let local = std::path::Path::new(url);
        let data = if local.is_file() {
            if std::fs::metadata(local)?.len() > MAX_IMPORT_BYTES {
                return Err(Error::Io(std::io::Error::other("file too large for key import")));
            }
            std::fs::read(local)?
        } else {
            pmca_market::appstore::download(&normalize(url), MAX_IMPORT_BYTES)?
        };
        sink.emit(ProgressEvent::step(ProgressKind::Info, l.tr("keys-scanning")));
        scan_blob(&data);
    }
    persist().map_err(Error::Io)?;
    let state = status();
    let key = if state.complete { "keys-complete" } else { "keys-incomplete" };
    sink.emit(ProgressEvent::step(ProgressKind::Done, l.tr(key)));
    Ok(state)
}

/// Rewrite a bare repository URL to its default-branch archive; leave anything
/// else untouched so a direct file or archive URL works as given.
fn normalize(url: &str) -> String {
    let trimmed = url.trim_end_matches('/');
    if let Some(rest) = trimmed.strip_prefix("https://github.com/") {
        let rest = rest.strip_suffix(".git").unwrap_or(rest);
        let parts: Vec<&str> = rest.split('/').collect();
        if parts.len() == 2 {
            return format!("https://codeload.github.com/{}/{}/zip/HEAD", parts[0], parts[1]);
        }
    }
    if let Some(rest) = trimmed.strip_prefix("https://gitlab.com/") {
        let rest = rest.strip_suffix(".git").unwrap_or(rest);
        let parts: Vec<&str> = rest.split('/').collect();
        if parts.len() == 2 {
            return format!(
                "https://gitlab.com/{0}/{1}/-/archive/HEAD/{1}-HEAD.zip",
                parts[0], parts[1]
            );
        }
    }
    url.to_string()
}

/// Scan a downloaded blob: every file of a zip archive, or the bytes as they
/// are. Installs every key whose digest matches.
fn scan_blob(data: &[u8]) {
    if data.starts_with(b"PK\x03\x04") {
        if let Ok(mut archive) = zip::ZipArchive::new(Cursor::new(data)) {
            for i in 0..archive.len() {
                let Ok(entry) = archive.by_index(i) else { continue };
                if !entry.is_file() {
                    continue;
                }
                let mut buf = Vec::new();
                if entry.take(MAX_ENTRY_BYTES).read_to_end(&mut buf).is_ok() {
                    keystore::scan(&buf);
                }
                if keystore::complete() {
                    return;
                }
            }
        }
        return;
    }
    keystore::scan(data);
}

/// Write every loaded key to the profile.
fn persist() -> std::io::Result<()> {
    let Some(dir) = key_dir() else { return Ok(()) };
    std::fs::create_dir_all(&dir)?;
    for id in keystore::ALL {
        if let Some(bytes) = keystore::get(id) {
            write_private(&dir.join(format!("{}.key", id.slug())), &bytes)?;
        }
    }
    Ok(())
}

/// Write `bytes` to `path`, readable by the owner only where the platform
/// supports it.
fn write_private(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    std::fs::write(path, bytes)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_bare_github_url_becomes_a_zip_archive() {
        assert_eq!(
            normalize("https://github.com/owner/repo"),
            "https://codeload.github.com/owner/repo/zip/HEAD"
        );
        assert_eq!(
            normalize("https://github.com/owner/repo.git/"),
            "https://codeload.github.com/owner/repo/zip/HEAD"
        );
    }

    #[test]
    fn a_direct_url_is_left_alone() {
        let url = "https://example.com/keys/constants.py";
        assert_eq!(normalize(url), url);
        // A URL that points deeper than owner/repo is not a bare repo.
        let deep = "https://github.com/owner/repo/blob/main/x.py";
        assert_eq!(normalize(deep), deep);
    }

    #[test]
    fn status_lists_every_class() {
        let _g = keystore::test_guard();
        keystore::clear();
        let s = status();
        assert!(!s.complete);
        assert_eq!(s.classes.len(), 4);
        assert!(s.classes.iter().all(|c| !c.present));
    }
}
