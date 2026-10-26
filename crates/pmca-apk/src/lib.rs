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
//! The signature is not verified. [`signer_certificate`] only lifts the first
//! certificate out of the signature block, so that a later download can be
//! told apart from one signed by someone else; a full ASN.1/CMS stack would be
//! needed for more.

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

/// Upper bound on a signature block; real ones are a few kilobytes.
const MAX_SIGNATURE_SIZE: u64 = 1024 * 1024;

/// The DER bytes of the first certificate in the APK's v1 signature block.
///
/// `None` when the archive is unsigned or the block is not the PKCS#7 layout
/// `jarsigner` and `apksigner` write. The first block by name is used, so the
/// answer does not depend on the order of the archive.
pub fn signer_certificate(data: &[u8]) -> Result<Option<Vec<u8>>> {
    let mut archive = zip::ZipArchive::new(std::io::Cursor::new(data))?;
    let mut blocks: Vec<String> = archive
        .file_names()
        .filter_map(|n| n.ok().map(|n| n.into_owned()))
        .filter(|n| is_signature_block(n))
        .collect();
    blocks.sort();
    let Some(name) = blocks.first() else { return Ok(None) };
    let mut block = Vec::new();
    archive.by_name(name)?.take(MAX_SIGNATURE_SIZE).read_to_end(&mut block)?;
    Ok(der::first_certificate(&block).map(<[u8]>::to_vec))
}

/// Just enough DER to walk a PKCS#7 `SignedData` to its certificates.
mod der {
    /// One DER element.
    struct Element<'a> {
        tag: u8,
        /// Header and content together.
        whole: &'a [u8],
        content: &'a [u8],
        /// What follows the element.
        rest: &'a [u8],
    }

    fn element(input: &[u8]) -> Option<Element<'_>> {
        let (&tag, rest) = input.split_first()?;
        let (&first, rest) = rest.split_first()?;
        let (len, rest) = if first < 0x80 {
            (usize::from(first), rest)
        } else {
            // Indefinite (0x80) and lengths beyond 4 bytes are not DER here.
            let count = usize::from(first & 0x7f);
            if count == 0 || count > 4 || rest.len() < count {
                return None;
            }
            let len = rest[..count].iter().fold(0usize, |acc, &b| (acc << 8) | usize::from(b));
            (len, &rest[count..])
        };
        if rest.len() < len {
            return None;
        }
        let header = input.len() - rest.len();
        Some(Element {
            tag,
            whole: &input[..header + len],
            content: &rest[..len],
            rest: &rest[len..],
        })
    }

    /// The content of an element with `tag`, and what follows it.
    fn expect(input: &[u8], tag: u8) -> Option<(&[u8], &[u8])> {
        let found = element(input)?;
        (found.tag == tag).then_some((found.content, found.rest))
    }

    /// `ContentInfo { oid, [0] SignedData { version, digests, content, [0] certs } }`.
    pub fn first_certificate(block: &[u8]) -> Option<&[u8]> {
        let (info, _) = expect(block, 0x30)?;
        let (_, info) = expect(info, 0x06)?;
        let (wrapped, _) = expect(info, 0xa0)?;
        let (signed, _) = expect(wrapped, 0x30)?;
        let (_, signed) = expect(signed, 0x02)?;
        let (_, signed) = expect(signed, 0x31)?;
        let (_, signed) = expect(signed, 0x30)?;
        let (certificates, _) = expect(signed, 0xa0)?;
        let first = element(certificates)?;
        (first.tag == 0x30).then_some(first.whole)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        /// Wrap `content` in a DER element with `tag`.
        fn tlv(tag: u8, content: &[u8]) -> Vec<u8> {
            let mut out = vec![tag];
            if content.len() < 0x80 {
                out.push(content.len() as u8);
            } else {
                out.extend([0x82, (content.len() >> 8) as u8, content.len() as u8]);
            }
            out.extend_from_slice(content);
            out
        }

        fn block(certificates: &[u8]) -> Vec<u8> {
            let signed = [
                tlv(0x02, &[1]),
                tlv(0x31, &tlv(0x30, &[0x06, 0x01, 0x00])),
                tlv(0x30, &tlv(0x06, &[0x2a])),
                tlv(0xa0, certificates),
                tlv(0x31, &[]),
            ]
            .concat();
            tlv(0x30, &[tlv(0x06, &[0x2a, 0x86]), tlv(0xa0, &tlv(0x30, &signed))].concat())
        }

        #[test]
        fn finds_the_first_certificate() {
            let first = tlv(0x30, &[0xab; 200]);
            let second = tlv(0x30, &[0xcd; 3]);
            let data = block(&[first.clone(), second].concat());
            assert_eq!(first_certificate(&data), Some(first.as_slice()));
        }

        #[test]
        fn refuses_what_is_not_signed_data() {
            assert_eq!(first_certificate(&[]), None);
            assert_eq!(first_certificate(&[0x30, 0x05, 0x01]), None);
            assert_eq!(first_certificate(&block(&tlv(0x04, &[1]))), None);
            assert_eq!(first_certificate(&[0x30, 0x80, 0x00, 0x00]), None);
        }
    }
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
