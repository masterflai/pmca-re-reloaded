//! FDAT firmware-image (de)cryption, parsing and re-assembly.
//!
//! A Sony firmware update ships an encrypted `FDAT` image. The image is
//! processed in fixed-size blocks; each block is wrapped in a small frame
//! (a 16-bit checksum plus a size/last-block flag) and then run through a
//! generation-specific cipher:
//!
//! | Generation | Chip family            | Block | Cipher                               |
//! |-----------:|------------------------|------:|--------------------------------------|
//! | 1          | CXD4105 … CXD4120      | 1000  | SHA-1 key-stream XOR ([`Crypter::sha`]) |
//! | 2          | CXD4132                | 1024  | AES-128/ECB ([`Crypter::aes`])          |
//! | 3          | CXD90014               | 1024  | double AES-128/ECB ([`Crypter::double_aes`]) |
//!
//! Only these three generations can be *re-encrypted* by this code (and by the
//! upstream `fwtool`); the CBC-based 4th generation (CXD90045/90057) is
//! decrypt-only and intentionally not implemented here.
//!
//! # Key material
//!
//! Like the rest of the crate, this module never embeds Sony key bytes. The
//! per-generation keys are supplied by the caller (ultimately from
//! [`crate::keystore`]). All ciphers validate their key lengths up front.
//!
//! # Security hardening vs. the original
//!
//! * Every block length and header offset is bounds-checked before use.
//! * Frame checksums and the end-of-stream flag are verified on decryption.
//! * No `unsafe`, no panics on malformed input – malformed images return
//!   [`Error`] instead.

use aes::cipher::{BlockCipherDecrypt, BlockCipherEncrypt, KeyInit};
use aes::{Aes128, Block};

use crate::sha1::sha1;
use crate::util::{dump16le, dump32le, parse16le, parse32le};
use crate::{Error, Result};

/// Size of the decrypted FDAT header, in bytes.
pub const HEADER_SIZE: usize = 512;
/// Size of a single file-system sub-header inside the FDAT header.
const FS_HEADER_SIZE: usize = 16;
/// Number of file-system sub-headers reserved in the FDAT header.
const MAX_NUM_FILE_SYSTEMS: usize = 28;

const MAGIC: &[u8; 8] = b"UDTRFIRM";
const VERSION: &[u8; 4] = b"0100";

const UPDATE_MODE_USER: u8 = b'U';
const LUW_FLAG_NORMAL: u8 = b'N';
const FS_MODE_USER: u8 = b'U';
const FS_MODE_PROD: u8 = b'P';

// Byte offsets inside the 512-byte header.
const OFF_MAGIC: usize = 0;
const OFF_CHECKSUM: usize = 8;
const OFF_VERSION: usize = 12;
const OFF_MODE_TYPE: usize = 16;
const OFF_LUW_FLAG: usize = 20;
const OFF_VERSION_MINOR: usize = 32;
const OFF_VERSION_MAJOR: usize = 33;
const OFF_MODEL: usize = 36;
const OFF_REGION: usize = 40;
const OFF_FIRMWARE_OFFSET: usize = 48;
const OFF_FIRMWARE_SIZE: usize = 52;
const OFF_NUM_FILE_SYSTEMS: usize = 56;
const OFF_FILE_SYSTEM_HEADERS: usize = 64;

/// Parsed, decrypted FDAT header fields relevant to re-assembly.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FdatHeader {
    /// Numeric model identifier.
    pub model: u32,
    /// Numeric region identifier.
    pub region: u32,
    /// Major firmware version (BCD-ish, as stored).
    pub version_major: u8,
    /// Minor firmware version (BCD-ish, as stored).
    pub version_minor: u8,
    /// Offset of the firmware body within the decrypted image.
    pub firmware_offset: u32,
    /// Length of the firmware body within the decrypted image.
    pub firmware_size: u32,
}

/// Returns `true` if `data` looks like a *decrypted* FDAT image.
///
/// This mirrors `fwtool`'s `isFdat`: the magic must match and the reserved
/// file-system header area must end in four zero bytes (an unused slot).
#[must_use]
pub fn is_fdat(data: &[u8]) -> bool {
    data.len() >= HEADER_SIZE
        && &data[OFF_MAGIC..OFF_MAGIC + 8] == MAGIC
        && data[HEADER_SIZE - 4..HEADER_SIZE] == [0, 0, 0, 0]
}

/// CRC-32 (IEEE 802.3, as used by zlib / `binascii.crc32`).
#[must_use]
pub fn crc32(data: &[u8]) -> u32 {
    let mut crc: u32 = 0xFFFF_FFFF;
    for &byte in data {
        crc ^= u32::from(byte);
        for _ in 0..8 {
            let mask = (crc & 1).wrapping_neg();
            crc = (crc >> 1) ^ (0xEDB8_8320 & mask);
        }
    }
    !crc
}

/// Computes the header checksum over bytes `12..HEADER_SIZE`.
fn header_checksum(image: &[u8]) -> u32 {
    crc32(&image[OFF_VERSION..HEADER_SIZE])
}

/// Reads the relevant fields from a decrypted FDAT header.
pub fn read_header(image: &[u8]) -> Result<FdatHeader> {
    if image.len() < HEADER_SIZE {
        return Err(Error::OutOfBounds { context: "fdat header" });
    }
    if &image[OFF_MAGIC..OFF_MAGIC + 8] != MAGIC {
        return Err(Error::BadMagic { expected: "UDTRFIRM" });
    }
    if &image[OFF_VERSION..OFF_VERSION + 4] != VERSION {
        return Err(Error::Invalid("unsupported FDAT version"));
    }
    if header_checksum(image) != parse32le(&image[OFF_CHECKSUM..OFF_CHECKSUM + 4])? {
        return Err(Error::Checksum);
    }
    if image[OFF_MODE_TYPE] != UPDATE_MODE_USER {
        return Err(Error::Invalid("unsupported FDAT update mode"));
    }
    if image[OFF_LUW_FLAG] != LUW_FLAG_NORMAL {
        return Err(Error::Invalid("unsupported FDAT LUW flag"));
    }
    Ok(FdatHeader {
        model: parse32le(&image[OFF_MODEL..OFF_MODEL + 4])?,
        region: parse32le(&image[OFF_REGION..OFF_REGION + 4])?,
        version_major: image[OFF_VERSION_MAJOR],
        version_minor: image[OFF_VERSION_MINOR],
        firmware_offset: parse32le(&image[OFF_FIRMWARE_OFFSET..OFF_FIRMWARE_OFFSET + 4])?,
        firmware_size: parse32le(&image[OFF_FIRMWARE_SIZE..OFF_FIRMWARE_SIZE + 4])?,
    })
}

/// Overwrites the model, region and version in a decrypted FDAT header and
/// recomputes the header checksum in place.
///
/// This is the hook the runtime firmware builder uses to retarget a reference
/// image at a specific camera model. It does not touch the firmware body or
/// the file systems.
pub fn retarget(
    image: &mut [u8],
    model: u32,
    region: u32,
    version_major: u8,
    version_minor: u8,
) -> Result<()> {
    if image.len() < HEADER_SIZE {
        return Err(Error::OutOfBounds { context: "fdat header" });
    }
    if &image[OFF_MAGIC..OFF_MAGIC + 8] != MAGIC {
        return Err(Error::BadMagic { expected: "UDTRFIRM" });
    }
    image[OFF_MODEL..OFF_MODEL + 4].copy_from_slice(&dump32le(model));
    image[OFF_REGION..OFF_REGION + 4].copy_from_slice(&dump32le(region));
    image[OFF_VERSION_MAJOR] = version_major;
    image[OFF_VERSION_MINOR] = version_minor;
    image[OFF_CHECKSUM..OFF_CHECKSUM + 4].copy_from_slice(&dump32le(0));
    let checksum = header_checksum(image);
    image[OFF_CHECKSUM..OFF_CHECKSUM + 4].copy_from_slice(&dump32le(checksum));
    Ok(())
}

/// Assembles a decrypted FDAT image from a file system and firmware body.
///
/// Port of `fwtool`'s `writeFdat`: lays out `[header][fs][firmware]`, writes
/// the two file-system sub-headers (one `U`ser, one `P`rod) and finally the
/// header checksum.
pub fn assemble(
    fs: &[u8],
    firmware: &[u8],
    model: u32,
    region: u32,
    version_major: u8,
    version_minor: u8,
) -> Vec<u8> {
    let fs_offset = HEADER_SIZE;
    let firmware_offset = fs_offset + fs.len();
    let mut image = vec![0u8; firmware_offset + firmware.len()];
    image[fs_offset..firmware_offset].copy_from_slice(fs);
    image[firmware_offset..].copy_from_slice(firmware);

    image[OFF_MAGIC..OFF_MAGIC + 8].copy_from_slice(MAGIC);
    image[OFF_VERSION..OFF_VERSION + 4].copy_from_slice(VERSION);
    image[OFF_MODE_TYPE] = UPDATE_MODE_USER;
    image[OFF_LUW_FLAG] = LUW_FLAG_NORMAL;
    image[OFF_VERSION_MINOR] = version_minor;
    image[OFF_VERSION_MAJOR] = version_major;
    image[OFF_MODEL..OFF_MODEL + 4].copy_from_slice(&dump32le(model));
    image[OFF_REGION..OFF_REGION + 4].copy_from_slice(&dump32le(region));
    image[OFF_FIRMWARE_OFFSET..OFF_FIRMWARE_OFFSET + 4]
        .copy_from_slice(&dump32le(firmware_offset as u32));
    image[OFF_FIRMWARE_SIZE..OFF_FIRMWARE_SIZE + 4]
        .copy_from_slice(&dump32le(firmware.len() as u32));
    image[OFF_NUM_FILE_SYSTEMS..OFF_NUM_FILE_SYSTEMS + 4].copy_from_slice(&dump32le(2));

    write_fs_header(
        &mut image,
        0,
        FS_MODE_USER,
        fs_offset as u32,
        (firmware_offset - fs_offset) as u32,
    );
    write_fs_header(&mut image, 1, FS_MODE_PROD, fs_offset as u32, 0);

    let checksum = header_checksum(&image);
    image[OFF_CHECKSUM..OFF_CHECKSUM + 4].copy_from_slice(&dump32le(checksum));
    image
}

fn write_fs_header(image: &mut [u8], index: usize, mode: u8, offset: u32, size: u32) {
    let base = OFF_FILE_SYSTEM_HEADERS + index * FS_HEADER_SIZE;
    image[base] = mode;
    image[base + 4..base + 8].copy_from_slice(&dump32le(offset));
    image[base + 8..base + 12].copy_from_slice(&dump32le(size));
    debug_assert!(index < MAX_NUM_FILE_SYSTEMS);
}

/// A generation-specific block cipher plus the frame (un)packing shared by all
/// generations.
///
/// Construct one with [`Crypter::sha`], [`Crypter::aes`] or
/// [`Crypter::double_aes`], then call [`Crypter::decrypt`] / [`Crypter::encrypt`].
pub struct Crypter {
    cipher: Cipher,
}

enum Cipher {
    Sha { key: [u8; 40], digest: [u8; 20] },
    Aes { cipher: Box<Aes128> },
    DoubleAes { c1: Box<Aes128>, c2: Box<Aes128> },
}

impl Cipher {
    /// Decrypt-block size (the on-disk block size).
    fn block_size(&self) -> usize {
        match self {
            Cipher::Sha { .. } => 1000,
            Cipher::Aes { .. } | Cipher::DoubleAes { .. } => 1024,
        }
    }

    fn reset(&mut self) {
        if let Cipher::Sha { key, digest } = self {
            digest.copy_from_slice(&key[..20]);
        }
    }

    fn sha_stream(key: &[u8; 40], digest: &mut [u8; 20], block: &mut [u8]) {
        let mut produced = 0;
        while produced < block.len() {
            let mut input = [0u8; 40];
            input[..20].copy_from_slice(digest);
            input[20..].copy_from_slice(&key[20..40]);
            *digest = sha1(&input);
            let take = (block.len() - produced).min(20);
            for (b, k) in block[produced..produced + take].iter_mut().zip(&digest[..take]) {
                *b ^= *k;
            }
            produced += take;
        }
    }

    fn decrypt_block(&mut self, block: &mut [u8], first: bool) {
        match self {
            Cipher::Sha { key, digest } => {
                if first {
                    digest.copy_from_slice(&key[..20]);
                }
                let key = *key;
                Self::sha_stream(&key, digest, block);
            }
            Cipher::Aes { cipher } => aes_ecb_decrypt(cipher, block),
            Cipher::DoubleAes { c1, c2 } => {
                aes_ecb_decrypt(c1, block);
                if first {
                    // Only bytes 512.. are run through the second cipher.
                    let mut tail = block[512..].to_vec();
                    aes_ecb_decrypt(c2, &mut tail);
                    block[512..].copy_from_slice(&tail);
                } else {
                    aes_ecb_decrypt(c2, block);
                }
            }
        }
    }

    fn encrypt_block(&mut self, block: &mut [u8], first: bool) {
        match self {
            Cipher::Sha { key, digest } => {
                if first {
                    digest.copy_from_slice(&key[..20]);
                }
                let key = *key;
                Self::sha_stream(&key, digest, block);
            }
            Cipher::Aes { cipher } => aes_ecb_encrypt(cipher, block),
            Cipher::DoubleAes { c1, c2 } => {
                if first {
                    // Bytes 0..512 bypass the second cipher entirely.
                    let mut tail = block[512..].to_vec();
                    aes_ecb_encrypt(c2, &mut tail);
                    block[512..].copy_from_slice(&tail);
                } else {
                    aes_ecb_encrypt(c2, block);
                }
                aes_ecb_encrypt(c1, block);
            }
        }
    }
}

fn aes_ecb_decrypt(cipher: &Aes128, data: &mut [u8]) {
    for chunk in data.as_chunks_mut::<16>().0 {
        let block = <&mut Block>::try_from(&mut chunk[..]).expect("chunk is one AES block");
        cipher.decrypt_block(block);
    }
}

fn aes_ecb_encrypt(cipher: &Aes128, data: &mut [u8]) {
    for chunk in data.as_chunks_mut::<16>().0 {
        let block = <&mut Block>::try_from(&mut chunk[..]).expect("chunk is one AES block");
        cipher.encrypt_block(block);
    }
}

/// Sum of the little-endian 16-bit words of `data`, truncated to 16 bits.
fn checksum16(data: &[u8]) -> u16 {
    let mut sum: u32 = 0;
    for word in data.as_chunks::<2>().0 {
        sum = sum.wrapping_add(u32::from(u16::from_le_bytes(*word)));
    }
    (sum & 0xffff) as u16
}

impl Crypter {
    /// 1st-generation SHA-1 key-stream cipher. `key` must be 40 bytes.
    pub fn sha(key: &[u8]) -> Result<Self> {
        let key: [u8; 40] =
            key.try_into().map_err(|_| Error::Invalid("FDAT gen-1 key must be 40 bytes"))?;
        Ok(Self { cipher: Cipher::Sha { key, digest: [0; 20] } })
    }

    /// 2nd-generation single AES-128/ECB cipher. `key` must be 16 bytes.
    pub fn aes(key: &[u8]) -> Result<Self> {
        let cipher = aes128(key)?;
        Ok(Self { cipher: Cipher::Aes { cipher: Box::new(cipher) } })
    }

    /// 3rd-generation double AES-128/ECB cipher. Both keys must be 16 bytes.
    pub fn double_aes(key1: &[u8], key2: &[u8]) -> Result<Self> {
        Ok(Self {
            cipher: Cipher::DoubleAes { c1: Box::new(aes128(key1)?), c2: Box::new(aes128(key2)?) },
        })
    }

    /// Decrypts a full encrypted FDAT image into its plaintext form.
    pub fn decrypt(&mut self, data: &[u8]) -> Result<Vec<u8>> {
        let block_size = self.cipher.block_size();
        if data.is_empty() || !data.len().is_multiple_of(block_size) {
            return Err(Error::Invalid("FDAT size is not a whole number of blocks"));
        }
        self.cipher.reset();
        let num_blocks = data.len() / block_size;
        let mut out = Vec::with_capacity(data.len());
        for (index, chunk) in data.chunks_exact(block_size).enumerate() {
            let first = index == 0;
            let last = index == num_blocks - 1;
            let mut block = chunk.to_vec();
            self.cipher.decrypt_block(&mut block, first);
            out.extend_from_slice(&unpack_block(&block, last)?);
        }
        Ok(out)
    }

    /// Encrypts a plaintext FDAT image into its on-disk encrypted form.
    pub fn encrypt(&mut self, data: &[u8]) -> Result<Vec<u8>> {
        let block_size = self.cipher.block_size();
        let payload = block_size - 4;
        if data.is_empty() {
            return Err(Error::Invalid("cannot encrypt an empty FDAT"));
        }
        self.cipher.reset();
        let num_blocks = data.len().div_ceil(payload);
        let mut out = Vec::with_capacity(num_blocks * block_size);
        for (index, chunk) in data.chunks(payload).enumerate() {
            let first = index == 0;
            let last = index == num_blocks - 1;
            let mut block = pack_block(chunk, last, block_size);
            self.cipher.encrypt_block(&mut block, first);
            out.extend_from_slice(&block);
        }
        Ok(out)
    }
}

fn aes128(key: &[u8]) -> Result<Aes128> {
    let key: [u8; 16] =
        key.try_into().map_err(|_| Error::Invalid("FDAT AES key must be 16 bytes"))?;
    Ok(Aes128::new((&key).into()))
}

/// Verifies a decrypted block frame and returns its payload.
fn unpack_block(block: &[u8], last: bool) -> Result<Vec<u8>> {
    if block.len() < 4 {
        return Err(Error::OutOfBounds { context: "fdat block frame" });
    }
    let checksum = parse16le(&block[0..2])?;
    let size_and_flag = parse16le(&block[2..4])?;
    let size = (size_and_flag & 0x7fff) as usize;
    let end_flag = size_and_flag & 0x8000 != 0;
    if checksum16(&block[2..]) != checksum {
        return Err(Error::Checksum);
    }
    if end_flag != last {
        return Err(Error::Invalid("FDAT last-block flag mismatch"));
    }
    if 4 + size > block.len() {
        return Err(Error::OutOfBounds { context: "fdat block payload" });
    }
    Ok(block[4..4 + size].to_vec())
}

/// Builds an encrypted-but-not-yet-ciphered block frame around `data`.
fn pack_block(data: &[u8], last: bool, block_size: usize) -> Vec<u8> {
    let mut size_and_flag = data.len() as u16;
    if last {
        size_and_flag |= 0x8000;
    }
    // inner = size/flag(2) + data + 0xff padding, total block_size - 2 bytes.
    let mut inner = Vec::with_capacity(block_size - 2);
    inner.extend_from_slice(&dump16le(size_and_flag));
    inner.extend_from_slice(data);
    inner.resize(block_size - 2, 0xff);
    let checksum = checksum16(&inner);
    let mut block = Vec::with_capacity(block_size);
    block.extend_from_slice(&dump16le(checksum));
    block.extend_from_slice(&inner);
    block
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crc32_matches_known_vector() {
        // zlib.crc32(b"123456789") == 0xCBF43926
        assert_eq!(crc32(b"123456789"), 0xCBF4_3926);
        assert_eq!(crc32(b""), 0);
    }

    #[test]
    fn checksum16_sums_le_words() {
        assert_eq!(checksum16(&[0x01, 0x00, 0xff, 0xff]), 0x0000);
        assert_eq!(checksum16(&[0x34, 0x12, 0x01, 0x00]), 0x1235);
    }

    fn sample_image() -> Vec<u8> {
        // A small but valid decrypted FDAT: header + 32-byte fs + 48-byte body.
        let fs = vec![0xAA; 32];
        let firmware = vec![0x55; 48];
        assemble(&fs, &firmware, 0x0030_0000, 0x0001_0000, 9, 0x99)
    }

    #[test]
    fn assemble_produces_readable_header() {
        let image = sample_image();
        assert!(is_fdat(&image));
        let header = read_header(&image).unwrap();
        assert_eq!(header.model, 0x0030_0000);
        assert_eq!(header.region, 0x0001_0000);
        assert_eq!(header.version_major, 9);
        assert_eq!(header.version_minor, 0x99);
        assert_eq!(header.firmware_offset as usize, HEADER_SIZE + 32);
        assert_eq!(header.firmware_size, 48);
    }

    #[test]
    fn retarget_updates_fields_and_checksum() {
        let mut image = sample_image();
        retarget(&mut image, 0x0040_0000, 0x0002_0000, 1, 0x23).unwrap();
        let header = read_header(&image).unwrap();
        assert_eq!(header.model, 0x0040_0000);
        assert_eq!(header.region, 0x0002_0000);
        assert_eq!(header.version_major, 1);
        assert_eq!(header.version_minor, 0x23);
    }

    fn roundtrip(mut enc: Crypter, mut dec: Crypter) {
        let image = sample_image();
        let ciphertext = enc.encrypt(&image).unwrap();
        let plaintext = dec.decrypt(&ciphertext).unwrap();
        assert_eq!(plaintext, image);
    }

    #[test]
    fn sha_roundtrip() {
        let key = [0x11u8; 40];
        roundtrip(Crypter::sha(&key).unwrap(), Crypter::sha(&key).unwrap());
    }

    #[test]
    fn aes_roundtrip() {
        let key = [0x22u8; 16];
        roundtrip(Crypter::aes(&key).unwrap(), Crypter::aes(&key).unwrap());
    }

    #[test]
    fn double_aes_roundtrip() {
        let k1 = [0x33u8; 16];
        let k2 = [0x44u8; 16];
        roundtrip(Crypter::double_aes(&k1, &k2).unwrap(), Crypter::double_aes(&k1, &k2).unwrap());
    }

    #[test]
    fn wrong_key_lengths_are_rejected() {
        assert!(Crypter::sha(&[0u8; 20]).is_err());
        assert!(Crypter::aes(&[0u8; 8]).is_err());
        assert!(Crypter::double_aes(&[0u8; 16], &[0u8; 4]).is_err());
    }
}
