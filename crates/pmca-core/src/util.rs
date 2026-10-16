//! Endian-safe integer (de)serialization helpers.
//!
//! Faithful port of `pmca/util/__init__.py`. Every reader is bounds-checked
//! and returns a [`Result`] instead of panicking on short input, which is the
//! main memory-safety improvement over the original `struct.unpack` calls.

use crate::{Error, Result};

/// Read a little-endian `u32` from the first 4 bytes of `data`.
pub fn parse32le(data: &[u8]) -> Result<u32> {
    let b: [u8; 4] = data
        .get(..4)
        .and_then(|s| s.try_into().ok())
        .ok_or(Error::OutOfBounds { context: "parse32le" })?;
    Ok(u32::from_le_bytes(b))
}

/// Serialize `value` as 4 little-endian bytes.
pub fn dump32le(value: u32) -> [u8; 4] {
    value.to_le_bytes()
}

/// Read a big-endian `u32` from the first 4 bytes of `data`.
pub fn parse32be(data: &[u8]) -> Result<u32> {
    let b: [u8; 4] = data
        .get(..4)
        .and_then(|s| s.try_into().ok())
        .ok_or(Error::OutOfBounds { context: "parse32be" })?;
    Ok(u32::from_be_bytes(b))
}

/// Serialize `value` as 4 big-endian bytes.
pub fn dump32be(value: u32) -> [u8; 4] {
    value.to_be_bytes()
}

/// Read a little-endian `u16` from the first 2 bytes of `data`.
pub fn parse16le(data: &[u8]) -> Result<u16> {
    let b: [u8; 2] = data
        .get(..2)
        .and_then(|s| s.try_into().ok())
        .ok_or(Error::OutOfBounds { context: "parse16le" })?;
    Ok(u16::from_le_bytes(b))
}

/// Serialize `value` as 2 little-endian bytes.
pub fn dump16le(value: u16) -> [u8; 2] {
    value.to_le_bytes()
}

/// Read the first byte of `data`.
pub fn parse8(data: &[u8]) -> Result<u8> {
    data.first().copied().ok_or(Error::OutOfBounds { context: "parse8" })
}

/// Decode a fixed-size, NUL-padded byte field as a lossless Latin-1 string,
/// trailing NUL bytes removed. Latin-1 maps every byte 1:1 to a `char`, so
/// this never fails and never loses information.
pub fn latin1_trim_nul(data: &[u8]) -> String {
    let end = data.iter().rposition(|&b| b != 0).map_or(0, |p| p + 1);
    data[..end].iter().map(|&b| b as char).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_u32() {
        assert_eq!(parse32le(&dump32le(0x11223344)).unwrap(), 0x11223344);
        assert_eq!(parse32be(&dump32be(0x11223344)).unwrap(), 0x11223344);
    }

    #[test]
    fn endianness() {
        assert_eq!(dump32le(0x11223344), [0x44, 0x33, 0x22, 0x11]);
        assert_eq!(dump32be(0x11223344), [0x11, 0x22, 0x33, 0x44]);
    }

    #[test]
    fn short_input_is_error_not_panic() {
        assert!(parse32le(&[1, 2, 3]).is_err());
        assert!(parse16le(&[1]).is_err());
        assert!(parse8(&[]).is_err());
    }

    #[test]
    fn latin1_trims_trailing_nul() {
        assert_eq!(latin1_trim_nul(b"EU_\0\0\0"), "EU_");
        assert_eq!(latin1_trim_nul(&[0xe9, 0x00]), "é");
    }
}
