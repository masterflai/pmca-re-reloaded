//! Firmware `.dat` container parsing – port of `pmca/firmware`.
//!
//! A firmware update file starts with an 8-byte magic followed by a sequence
//! of big-endian `(size: u32, type: [u8;4])` chunk headers. The payload we
//! care about is the `FDAT` chunk. This parser locates it without ever
//! trusting a length field past the end of the buffer.

use crate::util::parse32be;
use crate::{Error, Result};

const DAT_MAGIC: [u8; 8] = [0x89, 0x55, 0x46, 0x55, 0x0d, 0x0a, 0x1a, 0x0a];

/// Location of the firmware payload inside a `.dat` file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FdatLocation {
    /// Byte offset of the `FDAT` payload within the file.
    pub offset: usize,
    /// Length of the `FDAT` payload in bytes.
    pub size: usize,
}

/// Locate the `FDAT` chunk inside a firmware `.dat` buffer.
///
/// Returns the offset and size of the firmware payload. All arithmetic is
/// checked so a corrupt `size` field can never cause an out-of-bounds slice.
pub fn read_dat(data: &[u8]) -> Result<FdatLocation> {
    if data.len() < 8 || data[..8] != DAT_MAGIC {
        return Err(Error::BadMagic { expected: "UFU dat header" });
    }
    let mut pos = 8usize;
    // Each chunk header is size(u32 BE) + type[4].
    while let Some(header) = data.get(pos..pos + 8) {
        let size = parse32be(&header[..4])? as usize;
        let ty = &header[4..8];
        let body = pos + 8;
        if ty == b"FDAT" {
            // Validate the payload actually fits.
            let end = body.checked_add(size).ok_or(Error::OutOfBounds { context: "FDAT size" })?;
            if end > data.len() {
                return Err(Error::OutOfBounds { context: "FDAT payload" });
            }
            return Ok(FdatLocation { offset: body, size });
        }
        pos = body.checked_add(size).ok_or(Error::OutOfBounds { context: "chunk size" })?;
        if pos > data.len() {
            break;
        }
    }
    Err(Error::Invalid("FDAT chunk not found"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::dump32be;

    fn build(chunks: &[(&[u8; 4], &[u8])]) -> Vec<u8> {
        let mut v = DAT_MAGIC.to_vec();
        for (ty, body) in chunks {
            v.extend_from_slice(&dump32be(body.len() as u32));
            v.extend_from_slice(*ty);
            v.extend_from_slice(body);
        }
        v
    }

    #[test]
    fn finds_fdat_after_other_chunks() {
        let dat = build(&[(b"BLOB", &[1, 2, 3, 4]), (b"FDAT", &[9, 8, 7])]);
        let loc = read_dat(&dat).unwrap();
        assert_eq!(loc.size, 3);
        assert_eq!(&dat[loc.offset..loc.offset + loc.size], &[9, 8, 7]);
    }

    #[test]
    fn rejects_bad_magic() {
        assert!(matches!(read_dat(&[0u8; 16]), Err(Error::BadMagic { .. })));
    }

    #[test]
    fn rejects_oversize_fdat() {
        let mut dat = DAT_MAGIC.to_vec();
        dat.extend_from_slice(&dump32be(0xffff_ffff)); // lies about size
        dat.extend_from_slice(b"FDAT");
        assert!(read_dat(&dat).is_err());
    }
}
