//! # pmca-apk
//!
//! Reads the metadata the toolkit needs out of an Android APK: the package
//! name, version and minimum SDK level, plus whether the archive carries a
//! v1 (JAR) signature.
//!
//! This is a Rust port of `pmca/apk`, which relied on `axmlparserpy` and
//! `asn1crypto`. An APK is untrusted input — it is downloaded from a third
//! party and handed straight to a camera — so the decoder here validates every
//! offset ([`axml`]) and caps how much data it will decompress.
//!
//! The certificate itself is **not** parsed: the original code only used it to
//! print a warning when it was unreadable, which [`ApkInfo::signed`] covers
//! without pulling in an ASN.1/CMS stack.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod axml;

use std::io::Read;

/// Path of the manifest inside every APK.
const MANIFEST_PATH: &str = "AndroidManifest.xml";

/// Upper bound on the decompressed manifest, guarding against a zip bomb.
const MAX_MANIFEST_SIZE: u64 = 4 * 1024 * 1024;

/// Errors produced while inspecting an APK.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The file is not a readable ZIP archive.
    #[error("invalid apk archive: {0}")]
    Zip(String),
    /// The archive has no `AndroidManifest.xml`.
    #[error("apk contains no AndroidManifest.xml")]
    NoManifest,
    /// The binary manifest could not be decoded.
    #[error("invalid AndroidManifest.xml: {0}")]
    Axml(&'static str),
    /// The manifest decoded but lacks the `<manifest package="…">` attribute.
    #[error("AndroidManifest.xml declares no package name")]
    NoPackage,
    /// Reading the archive failed.
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

/// Convenience alias.
pub type Result<T> = std::result::Result<T, Error>;

impl From<zip::result::ZipError> for Error {
    fn from(e: zip::result::ZipError) -> Self {
        Error::Zip(e.to_string())
    }
}

/// The metadata of an APK.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApkInfo {
    /// Application package name, e.g. `com.github.ma1co.pmcademo.app`.
    pub package: String,
    /// `android:versionCode`, if declared.
    pub version_code: Option<u32>,
    /// `android:versionName`, if declared.
    pub version_name: Option<String>,
    /// `<uses-sdk android:minSdkVersion>`, if declared.
    pub min_sdk_version: Option<u32>,
    /// Whether the archive contains a v1 (JAR) signature block.
    pub signed: bool,
}

/// Read the metadata of a complete APK file.
pub fn read(data: &[u8]) -> Result<ApkInfo> {
    let mut archive = zip::ZipArchive::new(std::io::Cursor::new(data))?;

    let signed = archive.file_names().any(|n| n.is_ok_and(|n| is_signature_block(&n)));

    let mut manifest = Vec::new();
    match archive.by_name(MANIFEST_PATH) {
        Ok(entry) => {
            entry.take(MAX_MANIFEST_SIZE).read_to_end(&mut manifest)?;
        }
        Err(zip::result::ZipError::FileNotFound) => return Err(Error::NoManifest),
        Err(e) => return Err(e.into()),
    }

    let mut info = read_manifest(&manifest)?;
    info.signed = signed;
    Ok(info)
}

/// Read the metadata out of an already-extracted binary `AndroidManifest.xml`.
pub fn read_manifest(manifest: &[u8]) -> Result<ApkInfo> {
    let elements = axml::parse(manifest)?;
    let root = elements
        .iter()
        .find(|e| e.name == "manifest")
        .ok_or(Error::Axml("no <manifest> element"))?;

    let package = root
        .attr("package")
        .and_then(axml::Value::as_str)
        .filter(|s| !s.is_empty())
        .ok_or(Error::NoPackage)?
        .to_string();

    Ok(ApkInfo {
        package,
        version_code: root.attr("versionCode").and_then(axml::Value::as_int),
        version_name: root.attr("versionName").and_then(axml::Value::as_str).map(str::to_string),
        min_sdk_version: elements
            .iter()
            .find(|e| e.name == "uses-sdk")
            .and_then(|e| e.attr("minSdkVersion"))
            .and_then(axml::Value::as_int),
        signed: false,
    })
}

/// A `META-INF/*.RSA|DSA|EC` entry marks a v1-signed archive.
fn is_signature_block(name: &str) -> bool {
    name.starts_with("META-INF/")
        && (name.ends_with(".RSA") || name.ends_with(".DSA") || name.ends_with(".EC"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_metadata_from_a_manifest() {
        let info = read_manifest(&axml::tests::sample(false)).unwrap();
        assert_eq!(info.package, "com.example.app");
        assert_eq!(info.version_code, Some(42));
        assert_eq!(info.version_name.as_deref(), Some("1.2.3"));
        assert_eq!(info.min_sdk_version, Some(10));
        assert!(!info.signed);
    }

    #[test]
    fn a_garbage_manifest_is_rejected() {
        assert!(read_manifest(b"not xml at all").is_err());
    }

    #[test]
    fn a_non_zip_file_is_rejected() {
        assert!(matches!(read(b"PK-not-really"), Err(Error::Zip(_))));
    }

    #[test]
    fn signature_blocks_are_recognized() {
        assert!(is_signature_block("META-INF/CERT.RSA"));
        assert!(is_signature_block("META-INF/CERT.EC"));
        assert!(!is_signature_block("META-INF/MANIFEST.MF"));
        assert!(!is_signature_block("res/CERT.RSA"));
    }
}
