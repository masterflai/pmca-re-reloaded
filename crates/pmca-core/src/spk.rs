//! SPK container handling – a faithful, memory-safe port of `pmca/spk`.
//!
//! An SPK file is the Sony app-store package format. It wraps an APK payload
//! that is AES-128/ECB encrypted in 1 MiB chunks (each chunk PKCS#7-padded to
//! a 16-byte boundary). The 128-bit AES key is transported as a raw-RSA
//! encrypted blob at the front of the container.
//!
//! The RSA operation is the *public-key* "textbook" transform `m^e mod n`
//! (matching the original `rsa._encrypt` call), used here only to unwrap the
//! sample key shipped inside every store package — no confidentiality claim is
//! made and none is relied upon.
//!
//! # Security hardening vs. the original
//!
//! * Every offset/length read from the container is bounds-checked before use.
//! * PKCS#7 unpadding rejects malformed pad lengths instead of silently
//!   truncating (which in Python could remove attacker-controlled amounts).
//! * No `unsafe`, no panics on malformed input.

use aes::cipher::{BlockCipherDecrypt, BlockCipherEncrypt, KeyInit};
use aes::{Aes128, Block};
use num_bigint::BigUint;

use crate::keystore::{self, KeyClass, KeyId};
use crate::util::{dump32le, parse32le};
use crate::{Error, Result};

/// SPK-related constants (port of `pmca/spk/constants.py`).
///
/// The RSA modulus and the sample wrapped key are **not** here: they are Sony
/// key material and are supplied at run time through [`crate::keystore`].
pub mod constants {
    /// MIME type advertised for SPK packages.
    pub const MIME_TYPE: &str = "application/vnd.sony.spk.package-archive";
    /// Canonical SPK file extension.
    pub const EXTENSION: &str = ".1.spk";
    /// Plaintext chunk size fed to AES on encryption (1 MiB).
    pub const BLOCK_SIZE: usize = 0x10_0000;
    /// PKCS#7 padding block size appended to every plaintext chunk.
    pub const PADDING_SIZE: usize = 16;
    /// RSA public exponent used to unwrap the AES key.
    pub const RSA_EXPONENT: u32 = 65537;
}

const SPK_MAGIC: &[u8; 4] = b"1spk";

/// Returns `true` if `data` starts with the SPK magic.
pub fn is_spk(data: &[u8]) -> bool {
    data.len() >= 8 && &data[..4] == SPK_MAGIC
}

/// Split an SPK container into `(encrypted_key, encrypted_data)`.
fn parse_container(data: &[u8]) -> Result<(&[u8], &[u8])> {
    if !is_spk(data) {
        return Err(Error::BadMagic { expected: "1spk" });
    }
    // Header: magic[4] + keyOffset(u32 LE)
    let key_offset = parse32le(&data[4..8])? as usize;
    let key_header_offset = 8usize
        .checked_add(key_offset)
        .ok_or(Error::OutOfBounds { context: "spk keyHeaderOffset" })?;
    // Key header: keySize(u32 LE)
    let key_size_bytes = data
        .get(key_header_offset..key_header_offset + 4)
        .ok_or(Error::OutOfBounds { context: "spk keyHeader" })?;
    let key_size = parse32le(key_size_bytes)? as usize;
    let key_start = key_header_offset + 4;
    let data_start =
        key_start.checked_add(key_size).ok_or(Error::OutOfBounds { context: "spk dataOffset" })?;
    let encrypted_key =
        data.get(key_start..data_start).ok_or(Error::OutOfBounds { context: "spk key" })?;
    let encrypted_data =
        data.get(data_start..).ok_or(Error::OutOfBounds { context: "spk data" })?;
    Ok((encrypted_key, encrypted_data))
}

/// Build an SPK container from an encrypted key and encrypted payload.
fn dump_container(encrypted_key: &[u8], encrypted_data: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(8 + 4 + encrypted_key.len() + encrypted_data.len());
    out.extend_from_slice(SPK_MAGIC);
    out.extend_from_slice(&dump32le(0)); // keyOffset = 0
    out.extend_from_slice(&dump32le(encrypted_key.len() as u32));
    out.extend_from_slice(encrypted_key);
    out.extend_from_slice(encrypted_data);
    out
}

/// Unwrap the AES key via the raw RSA public transform `m^e mod n`.
///
/// The modulus is Sony key material supplied through the key store; without it
/// the operation is refused.
fn decrypt_key(encrypted_key: &[u8]) -> Result<Vec<u8>> {
    let modulus_bytes = keystore::get(KeyId::SpkModulus)
        .ok_or(Error::KeyUnavailable { class: KeyClass::Spk.slug() })?;
    let modulus = BigUint::from_bytes_be(&modulus_bytes);
    let exponent = BigUint::from(constants::RSA_EXPONENT);
    let c = BigUint::from_bytes_be(encrypted_key);
    let m = c.modpow(&exponent, &modulus);
    Ok(m.to_bytes_be()) // minimal big-endian, matching Python long_to_bytes
}

/// Remove PKCS#7 padding, rejecting malformed pad lengths.
fn unpad(block: &[u8]) -> Result<&[u8]> {
    let n = *block.last().ok_or(Error::Invalid("empty AES block"))? as usize;
    if n == 0 || n > block.len() {
        return Err(Error::Invalid("invalid PKCS#7 padding"));
    }
    Ok(&block[..block.len() - n])
}

/// Apply PKCS#7 padding to `block.len() + (size - len % size)`.
fn pad(block: &[u8], size: usize) -> Vec<u8> {
    let n = size - block.len() % size;
    let mut out = Vec::with_capacity(block.len() + n);
    out.extend_from_slice(block);
    out.extend(std::iter::repeat_n(n as u8, n));
    out
}

/// Decrypt the AES-128/ECB payload (1 MiB + 16 chunking) into the raw APK.
fn decrypt_data(key: &[u8], encrypted: &[u8]) -> Result<Vec<u8>> {
    let cipher = build_cipher(key)?;
    let chunk = constants::BLOCK_SIZE + constants::PADDING_SIZE;
    let mut out = Vec::with_capacity(encrypted.len());
    for c in encrypted.chunks(chunk) {
        if c.len() % 16 != 0 {
            return Err(Error::Invalid("SPK ciphertext not block-aligned"));
        }
        let mut buf = c.to_vec();
        for block in buf.as_chunks_mut::<16>().0 {
            let ga = <&mut Block>::try_from(&mut block[..]).expect("chunk is one AES block");
            cipher.decrypt_block(ga);
        }
        out.extend_from_slice(unpad(&buf)?);
    }
    Ok(out)
}

/// Encrypt raw APK data into the AES-128/ECB payload (1 MiB chunking).
fn encrypt_data(key: &[u8], data: &[u8]) -> Result<Vec<u8>> {
    let cipher = build_cipher(key)?;
    let mut out = Vec::with_capacity(data.len() + data.len() / constants::BLOCK_SIZE + 16);
    // Match Python: iterate in BLOCK_SIZE chunks; an empty input still yields
    // one padded (all-padding) block via range(0, 0, step) -> no chunk. To be
    // byte-identical, empty input produces empty output, like Python's join.
    for c in data.chunks(constants::BLOCK_SIZE) {
        let mut buf = pad(c, constants::PADDING_SIZE);
        for block in buf.as_chunks_mut::<16>().0 {
            let ga = <&mut Block>::try_from(&mut block[..]).expect("chunk is one AES block");
            cipher.encrypt_block(ga);
        }
        out.extend_from_slice(&buf);
    }
    Ok(out)
}

fn build_cipher(key: &[u8]) -> Result<Aes128> {
    let k: [u8; 16] = key.try_into().map_err(|_| Error::Invalid("AES key is not 16 bytes"))?;
    Ok(Aes128::new(&k.into()))
}

/// Parse an SPK file and return the contained APK bytes.
pub fn parse(data: &[u8]) -> Result<Vec<u8>> {
    let (encrypted_key, encrypted_data) = parse_container(data)?;
    let key = decrypt_key(encrypted_key)?;
    decrypt_data(&key, encrypted_data)
}

/// Build an SPK file wrapping the given APK bytes.
///
/// Needs the sample wrapped key and the RSA modulus from the key store.
pub fn dump(apk: &[u8]) -> Result<Vec<u8>> {
    let encrypted_key = keystore::get(KeyId::SpkSampleKey)
        .ok_or(Error::KeyUnavailable { class: KeyClass::Spk.slug() })?;
    let key = decrypt_key(&encrypted_key)?;
    let encrypted_data = encrypt_data(&key, apk)?;
    Ok(dump_container(&encrypted_key, &encrypted_data))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aes_payload_round_trips() {
        // The AES chunking and container are tested without any key material:
        // a fixed 16-byte key stands in for the unwrapped SPK key.
        let key = [0x11u8; 16];
        let apk = b"PK\x03\x04 pretend this is an apk \x00\x01\x02".to_vec();
        let encrypted = encrypt_data(&key, &apk).unwrap();
        let container = dump_container(b"wrapped", &encrypted);
        assert!(is_spk(&container));
        let (_, data) = parse_container(&container).unwrap();
        assert_eq!(decrypt_data(&key, data).unwrap(), apk);
    }

    #[test]
    fn aes_payload_round_trips_across_a_chunk_boundary() {
        let key = [0x22u8; 16];
        let apk: Vec<u8> =
            (0..(constants::BLOCK_SIZE + 12345)).map(|i| (i * 7 + 3) as u8).collect();
        let encrypted = encrypt_data(&key, &apk).unwrap();
        assert_eq!(decrypt_data(&key, &encrypted).unwrap(), apk);
    }

    #[test]
    fn dump_without_keys_is_refused() {
        let _g = crate::keystore::test_guard();
        crate::keystore::clear();
        assert!(matches!(dump(b"apk"), Err(Error::KeyUnavailable { .. })));
    }

    #[test]
    fn rejects_wrong_magic() {
        assert!(matches!(parse(b"XXXX\0\0\0\0"), Err(Error::BadMagic { .. })));
    }

    #[test]
    fn rejects_truncated_container() {
        assert!(parse(b"1spk").is_err());
    }
}
