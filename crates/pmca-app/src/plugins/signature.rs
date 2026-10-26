//! Plugin signatures: Ed25519 over the SHA-256 of every other file in the
//! archive.
//!
//! The signature travels inside the plugin file as [`FILE`], together with the
//! public key, the way an APK carries its certificate. That proves nothing
//! about the author on its own; what it allows is telling a later version
//! signed with the same key from one signed with another.

use ring::rand::SystemRandom;
use ring::signature::{ED25519, Ed25519KeyPair, KeyPair, UnparsedPublicKey};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// Name of the signature inside the archive.
pub const FILE: &str = "signature.json";

/// Prefix of the signed message, so a signature cannot be reused elsewhere.
const CONTEXT: &str = "pmca-plugin-signature-v1\n";
const ALGORITHM: &str = "ed25519";

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Signature {
    algorithm: String,
    /// 32-byte public key, hex.
    public_key: String,
    /// 64-byte signature, hex.
    signature: String,
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn unhex(text: &str) -> Option<Vec<u8>> {
    if !text.len().is_multiple_of(2) {
        return None;
    }
    (0..text.len()).step_by(2).map(|i| u8::from_str_radix(text.get(i..i + 2)?, 16).ok()).collect()
}

/// What is signed: every file except the signature, by name, with its SHA-256.
fn message(files: &[(String, Vec<u8>)]) -> Vec<u8> {
    let mut lines: Vec<String> = files
        .iter()
        .filter(|(name, _)| name != FILE)
        .map(|(name, data)| format!("{name}\t{}\n", hex(&Sha256::digest(data))))
        .collect();
    lines.sort();
    let mut out = CONTEXT.as_bytes().to_vec();
    for line in lines {
        out.extend_from_slice(line.as_bytes());
    }
    out
}

/// Check the signature over `files`; the signer's public key (hex) if it holds.
pub fn verify(signature: &[u8], files: &[(String, Vec<u8>)]) -> Result<String, String> {
    let parsed: Signature =
        serde_json::from_slice(signature).map_err(|e| format!("`{FILE}`: {e}"))?;
    if parsed.algorithm != ALGORITHM {
        return Err(format!("`{FILE}`: unknown algorithm `{}`", parsed.algorithm));
    }
    let key = unhex(&parsed.public_key).filter(|k| k.len() == 32);
    let sig = unhex(&parsed.signature).filter(|s| s.len() == 64);
    let (Some(key), Some(sig)) = (key, sig) else {
        return Err(format!("`{FILE}` is malformed"));
    };
    UnparsedPublicKey::new(&ED25519, &key)
        .verify(&message(files), &sig)
        .map_err(|_| "the signature does not match the contents".to_string())?;
    Ok(hex(&key))
}

/// Sign `files` with a PKCS#8 key; the contents of [`FILE`] and the public key (hex).
pub fn sign(pkcs8: &[u8], files: &[(String, Vec<u8>)]) -> Result<(Vec<u8>, String), String> {
    let pair = Ed25519KeyPair::from_pkcs8_maybe_unchecked(pkcs8)
        .map_err(|_| "not an Ed25519 key in PKCS#8 form".to_string())?;
    let public_key = hex(pair.public_key().as_ref());
    let signature = Signature {
        algorithm: ALGORITHM.to_string(),
        public_key: public_key.clone(),
        signature: hex(pair.sign(&message(files)).as_ref()),
    };
    let json = serde_json::to_vec_pretty(&signature).map_err(|e| e.to_string())?;
    Ok((json, public_key))
}

/// A new key pair: the PKCS#8 document to keep secret, and the public key (hex).
pub fn generate() -> Result<(Vec<u8>, String), String> {
    let pkcs8 = Ed25519KeyPair::generate_pkcs8(&SystemRandom::new())
        .map_err(|_| "no random numbers available".to_string())?;
    let pair = Ed25519KeyPair::from_pkcs8(pkcs8.as_ref()).map_err(|e| e.to_string())?;
    Ok((pkcs8.as_ref().to_vec(), hex(pair.public_key().as_ref())))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn files() -> Vec<(String, Vec<u8>)> {
        vec![
            ("plugin.json".to_string(), b"{}".to_vec()),
            ("locales/en-US.ftl".to_string(), b"plg-x-a = A\n".to_vec()),
        ]
    }

    #[test]
    fn a_signature_verifies_and_names_its_key() {
        let (key, public) = generate().unwrap();
        let (signature, signed_by) = sign(&key, &files()).unwrap();
        assert_eq!(signed_by, public);
        assert_eq!(verify(&signature, &files()).unwrap(), public);
    }

    #[test]
    fn the_order_of_the_archive_does_not_matter() {
        let (key, _) = generate().unwrap();
        let (signature, _) = sign(&key, &files()).unwrap();
        let mut reversed = files();
        reversed.reverse();
        assert!(verify(&signature, &reversed).is_ok());
    }

    #[test]
    fn a_changed_added_or_renamed_file_breaks_it() {
        let (key, _) = generate().unwrap();
        let (signature, _) = sign(&key, &files()).unwrap();

        let mut changed = files();
        changed[1].1 = b"plg-x-a = B\n".to_vec();
        assert!(verify(&signature, &changed).is_err());

        let mut added = files();
        added.push(("extra.wasm".to_string(), vec![0]));
        assert!(verify(&signature, &added).is_err());

        let mut renamed = files();
        renamed[1].0 = "locales/de-DE.ftl".to_string();
        assert!(verify(&signature, &renamed).is_err());
    }

    #[test]
    fn the_signature_file_itself_is_not_signed() {
        let (key, _) = generate().unwrap();
        let (signature, _) = sign(&key, &files()).unwrap();
        let mut with_itself = files();
        with_itself.push((FILE.to_string(), signature.clone()));
        assert!(verify(&signature, &with_itself).is_ok());
    }

    #[test]
    fn malformed_signatures_are_refused() {
        assert!(verify(b"not json", &files()).is_err());
        let other = br#"{"algorithm":"rsa","publicKey":"00","signature":"00"}"#;
        assert!(verify(other, &files()).is_err());
        let short = br#"{"algorithm":"ed25519","publicKey":"00","signature":"00"}"#;
        assert!(verify(short, &files()).is_err());
        assert!(sign(b"not a key", &files()).is_err());
    }
}
