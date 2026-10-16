//! XPD file handling – port of `pmca/xpd`.
//!
//! An XPD file is an INI-style document with a single `[DrmTCD]` section. It is
//! encoded as Latin-1 (every byte is one code point), which we handle
//! losslessly on both read and write. The CIC checksum used to authenticate
//! XPD files is an HMAC-SHA256 over the payload with a fixed key.
//!
//! # Encoding safety
//!
//! Text is converted to/from bytes exclusively through Latin-1, never assuming
//! UTF-8. This guarantees round-trip fidelity for the byte values Sony's tools
//! emit and prevents silent corruption of non-ASCII bytes.

use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha256;

use crate::{Error, Result};

/// XPD-related constants (port of `pmca/xpd/constants.py`).
///
/// The CIC HMAC key is **not** here: it is Sony key material and is supplied at
/// run time through [`crate::keystore`].
pub mod constants {
    /// MIME type advertised for XPD files.
    pub const MIME_TYPE: &str = "application/x-psn-dstartup2";
    /// The single INI section name used in XPD files.
    pub const SECTION_NAME: &str = "DrmTCD";
}

type HmacSha256 = Hmac<Sha256>;

/// Encode a byte buffer as Latin-1 text (lossless, infallible).
fn latin1_decode(data: &[u8]) -> String {
    data.iter().map(|&b| b as char).collect()
}

/// Encode a `str` back to Latin-1 bytes. Returns an error if a character is
/// outside the Latin-1 range instead of silently replacing it.
fn latin1_encode(s: &str) -> Result<Vec<u8>> {
    s.chars()
        .map(|c| {
            let cp = c as u32;
            if cp <= 0xff {
                Ok(cp as u8)
            } else {
                Err(Error::Invalid("non-latin1 character in XPD value"))
            }
        })
        .collect()
}

/// Parse an XPD file into ordered `(key, value)` pairs from the `DrmTCD`
/// section. Comment lines and blank lines are ignored.
pub fn parse(data: &[u8]) -> Result<Vec<(String, String)>> {
    let text = latin1_decode(data);
    let mut items = Vec::new();
    let mut in_section = false;
    for raw in text.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with(&['#', ';'][..]) {
            continue;
        }
        if let Some(section) = line.strip_prefix('[').and_then(|l| l.strip_suffix(']')) {
            in_section = section == constants::SECTION_NAME;
            continue;
        }
        if in_section && let Some((k, v)) = line.split_once('=') {
            items.push((k.trim().to_string(), v.trim().to_string()));
        }
    }
    Ok(items)
}

/// Look up a key in a parsed XPD item list.
pub fn get<'a>(items: &'a [(String, String)], key: &str) -> Option<&'a str> {
    items.iter().find(|(k, _)| k == key).map(|(_, v)| v.as_str())
}

/// Serialize `(key, value)` pairs into an XPD (INI) byte buffer.
pub fn dump(items: &[(String, String)]) -> Result<Vec<u8>> {
    let mut s = String::new();
    s.push('[');
    s.push_str(constants::SECTION_NAME);
    s.push_str("]\n");
    for (k, v) in items {
        s.push_str(k);
        s.push_str(" = ");
        s.push_str(v);
        s.push('\n');
    }
    s.push('\n');
    latin1_encode(&s)
}

/// Compute the CIC checksum (HMAC-SHA256) of `data` as a lowercase hex string.
///
/// Needs the CIC key from the key store; without it the operation is refused.
pub fn calculate_checksum(data: &[u8]) -> Result<String> {
    let key = crate::keystore::get(crate::keystore::KeyId::Cic)
        .ok_or(Error::KeyUnavailable { class: crate::keystore::KeyClass::Cic.slug() })?;
    let mut mac = HmacSha256::new_from_slice(&key).expect("HMAC accepts any key size");
    mac.update(data);
    let bytes = mac.finalize().into_bytes();
    Ok(bytes.iter().map(|b| format!("{b:02x}")).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_reads_section_items() {
        let xpd = b"[DrmTCD]\nFNAME = app.spk\nOUS = https://example/app\n\n";
        let items = parse(xpd).unwrap();
        assert_eq!(get(&items, "FNAME"), Some("app.spk"));
        assert_eq!(get(&items, "OUS"), Some("https://example/app"));
    }

    #[test]
    fn parse_ignores_other_sections_and_comments() {
        let xpd = b"# comment\n[Other]\nX = 1\n[DrmTCD]\nA = b\n";
        let items = parse(xpd).unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(get(&items, "A"), Some("b"));
    }

    #[test]
    fn dump_then_parse_roundtrip() {
        let items = vec![
            ("FNAME".to_string(), "x.spk".to_string()),
            ("OUS".to_string(), "https://h/x".to_string()),
        ];
        let bytes = dump(&items).unwrap();
        assert_eq!(parse(&bytes).unwrap(), items);
    }

    #[test]
    fn checksum_needs_the_cic_key() {
        let _g = crate::keystore::test_guard();
        crate::keystore::clear();
        assert!(matches!(calculate_checksum(b"hello"), Err(Error::KeyUnavailable { .. })));
    }

    #[test]
    fn checksum_is_stable_hex_of_correct_length() {
        let _g = crate::keystore::test_guard();
        // A fabricated CIC key, so the test carries no real material.
        crate::keystore::install_unchecked(crate::keystore::KeyId::Cic, vec![0x42; 90]);
        let c = calculate_checksum(b"hello").unwrap();
        assert_eq!(c.len(), 64); // SHA-256 -> 32 bytes -> 64 hex chars
        assert!(c.chars().all(|c| c.is_ascii_hexdigit()));
        // Deterministic:
        assert_eq!(c, calculate_checksum(b"hello").unwrap());
        crate::keystore::clear();
    }
}
