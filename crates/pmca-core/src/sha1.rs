//! SHA-1, including the length-truncating variant used by Sony's senser code.
//!
//! Port of `pmca/usb/crypto.py`. A stock SHA-1 is provided ([`sha1`]) together
//! with [`sha1_faulty`], which reproduces a bug in Sony's on-camera senser
//! implementation: the bit-length appended during padding is derived from
//! `len(message) & 0x1f` rather than the real message length.
//!
//! Reproducing that bug is required to pass the camera's service-mode
//! authentication challenge; it is deliberately *not* a general-purpose hash.
//!
//! # Security note
//!
//! SHA-1 is cryptographically broken and is used here only because the camera
//! protocol mandates it. Do not use these functions for anything else.

/// Length of a SHA-1 digest in bytes.
pub const DIGEST_SIZE: usize = 20;

/// SHA-1 round constants.
const K: [u32; 4] = [0x5a82_7999, 0x6ed9_eba1, 0x8f1b_bcdc, 0xca62_c1d6];

/// Initial state (h0..h4).
const INIT: [u32; 5] = [0x6745_2301, 0xefcd_ab89, 0x98ba_dcfe, 0x1032_5476, 0xc3d2_e1f0];

/// Computes the SHA-1 digest of `message`.
pub fn sha1(message: &[u8]) -> [u8; DIGEST_SIZE] {
    sha1_with_length(message, message.len() as u64)
}

/// Computes SHA-1 over `message` but appends `length` (in bytes) as the padded
/// bit count instead of the real message length.
///
/// With `length == message.len()` this is standard SHA-1; other values
/// reproduce non-conforming implementations such as Sony's.
pub fn sha1_with_length(message: &[u8], length: u64) -> [u8; DIGEST_SIZE] {
    // Padding: 0x80, then zeros up to a 56 (mod 64) boundary, then the bit count.
    let mut padded = Vec::with_capacity(message.len() + 72);
    padded.extend_from_slice(message);
    padded.push(0x80);
    padded.resize(padded.len() + (56usize.wrapping_sub(padded.len()) % 64), 0);
    padded.extend_from_slice(&length.wrapping_mul(8).to_be_bytes());

    let mut h = INIT;
    for block in padded.as_chunks::<64>().0 {
        let mut w = [0u32; 80];
        for (i, word) in block.as_chunks::<4>().0.iter().enumerate() {
            w[i] = u32::from_be_bytes(*word);
        }
        for i in 16..80 {
            w[i] = (w[i - 3] ^ w[i - 8] ^ w[i - 14] ^ w[i - 16]).rotate_left(1);
        }

        let [mut a, mut b, mut c, mut d, mut e] = h;
        for (i, wi) in w.iter().enumerate() {
            let (f, k) = match i {
                0..=19 => (d ^ (b & (c ^ d)), K[0]),
                20..=39 => (b ^ c ^ d, K[1]),
                40..=59 => ((b & c) | (b & d) | (c & d), K[2]),
                _ => (b ^ c ^ d, K[3]),
            };
            let tmp =
                a.rotate_left(5).wrapping_add(f).wrapping_add(e).wrapping_add(k).wrapping_add(*wi);
            e = d;
            d = c;
            c = b.rotate_left(30);
            b = a;
            a = tmp;
        }

        for (hi, vi) in h.iter_mut().zip([a, b, c, d, e]) {
            *hi = hi.wrapping_add(vi);
        }
    }

    let mut out = [0u8; DIGEST_SIZE];
    for (chunk, hi) in out.as_chunks_mut::<4>().0.iter_mut().zip(h) {
        *chunk = hi.to_be_bytes();
    }
    out
}

/// SHA-1 as (mis)implemented in Sony's senser component.
///
/// The appended bit count is computed from `message.len() & 0x1f`, so messages
/// of 32 bytes or more hash differently from standard SHA-1.
pub fn sha1_faulty(message: &[u8]) -> [u8; DIGEST_SIZE] {
    sha1_with_length(message, (message.len() & 0x1f) as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hex(digest: &[u8]) -> String {
        digest.iter().map(|b| format!("{b:02x}")).collect()
    }

    #[test]
    fn matches_known_vectors() {
        assert_eq!(hex(&sha1(b"")), "da39a3ee5e6b4b0d3255bfef95601890afd80709");
        assert_eq!(hex(&sha1(b"abc")), "a9993e364706816aba3e25717850c26c9cd0d89d");
        assert_eq!(
            hex(&sha1(b"The quick brown fox jumps over the lazy dog")),
            "2fd4e1c67a2d28fced849ee1bb76e7391b93eb12"
        );
    }

    #[test]
    fn handles_multi_block_input() {
        // 64 bytes forces exactly one extra padding block.
        let msg = vec![b'a'; 64];
        assert_eq!(hex(&sha1(&msg)), "0098ba824b5c16427bd7a1122a5a442a25ec644d");
    }

    #[test]
    fn faulty_matches_standard_below_32_bytes() {
        // For len < 0x20 the truncated length equals the real one.
        for len in [0usize, 1, 4, 31] {
            let msg = vec![0xabu8; len];
            assert_eq!(sha1_faulty(&msg), sha1(&msg), "len {len}");
        }
    }

    #[test]
    fn faulty_differs_from_standard_at_32_bytes() {
        // len 0x20 & 0x1f == 0, so the appended bit count is wrong.
        let msg = vec![0xabu8; 32];
        assert_ne!(sha1_faulty(&msg), sha1(&msg));
        assert_eq!(sha1_faulty(&msg), sha1_with_length(&msg, 0));
    }

    #[test]
    fn faulty_truncates_length_modulo_32() {
        let msg = vec![7u8; 0x212];
        assert_eq!(sha1_faulty(&msg), sha1_with_length(&msg, 0x12));
    }
}
