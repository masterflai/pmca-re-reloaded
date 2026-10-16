//! # pmca-core
//!
//! Hardware-independent core logic for PMCA-RE RELOADED.
//!
//! This crate is a faithful, memory-safe Rust port of the security-critical,
//! camera-independent parts of the original Python implementation:
//!
//! * [`spk`] – SPK container packing/unpacking (RSA key unwrap + AES payload).
//! * [`xpd`] – XPD (INI-style) parsing/serialization and CIC HMAC checksum.
//! * [`firmware`] – Firmware `.dat` container parsing (locating the `FDAT` chunk).
//! * [`fdat`] – FDAT firmware-image (de)cryption, parsing and re-assembly.
//! * [`backup`] – Read-only parser for the camera `Backup.bin` settings file.
//! * [`usagelog`] – Read-only reader for the camera's usage log ("KikiLog").
//! * [`photo`] – Read-only reader for the shutter count and capture details
//!   Sony cameras record in their images.
//! * [`sha1`] – SHA-1 plus the length-truncating variant Sony's senser
//!   service-mode authentication requires.
//! * [`util`] – Endian-safe integer (de)serialization helpers.
//!
//! ## Security notes
//!
//! Unlike the original Python code, every parser here validates lengths and
//! offsets *before* indexing into buffers. All fallible operations return
//! [`Result`] instead of panicking, so malformed input from an untrusted
//! source (a file or a camera) can never cause out-of-bounds reads or aborts.
//!
//! Input/output encoding is handled explicitly: byte buffers are treated as
//! opaque `&[u8]`, and any text conversion (e.g. Latin-1 for XPD, region
//! strings) is performed through dedicated, lossless helpers rather than
//! assuming UTF-8.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod backup;
pub mod fdat;
pub mod firmware;
pub mod keystore;
pub mod photo;
pub mod sha1;
pub mod spk;
pub mod usagelog;
pub mod util;
pub mod xpd;

/// Crate-wide error type shared by all parsers.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// A magic number / signature did not match the expected value.
    #[error("invalid magic (expected {expected:?})")]
    BadMagic {
        /// The signature that was expected.
        expected: &'static str,
    },
    /// A length or offset field pointed outside the available buffer.
    #[error("truncated or out-of-bounds data at {context}")]
    OutOfBounds {
        /// Human-readable location of the failure.
        context: &'static str,
    },
    /// A structural invariant of the parsed format was violated.
    #[error("invalid data: {0}")]
    Invalid(&'static str),
    /// A checksum did not match.
    #[error("checksum mismatch")]
    Checksum,
    /// A Sony-derived key the user has not supplied is needed here.
    #[error("the {class} key material is not available")]
    KeyUnavailable {
        /// Slug of the missing key class (see [`keystore::KeyClass::slug`]).
        class: &'static str,
    },
}

/// Convenience alias used throughout the crate.
pub type Result<T> = std::result::Result<T, Error>;
