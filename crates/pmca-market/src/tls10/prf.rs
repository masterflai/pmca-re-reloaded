//! The TLS 1.0 pseudo-random function and the key material derived from it.
//!
//! TLS 1.0 (RFC 2246 §5) splits the secret in half and runs two independent
//! expansions — one over MD5, one over SHA-1 — combining them with XOR. The
//! design predates the single-hash PRF of TLS 1.2 and exists so that the
//! construction stays secure as long as *either* hash does. MD5 and SHA-1 are
//! both broken for collision resistance today, which is why this function
//! cannot be reused outside the loopback camera connection.

use hmac::digest::Digest;
use hmac::digest::block_api::BlockSizeUser;
use hmac::{KeyInit, Mac, SimpleHmac};
use md5::Md5;
use sha1::Sha1;

/// `P_hash` (RFC 2246 §5): expands `secret` and `seed` to fill `out`.
///
/// `A(0) = seed`, `A(i) = HMAC(secret, A(i-1))`, and the output is the
/// concatenation of `HMAC(secret, A(i) + seed)`.
fn p_hash<D: Digest + BlockSizeUser + Clone>(secret: &[u8], seed: &[u8], out: &mut [u8]) {
    let hmac = |key: &[u8], parts: &[&[u8]]| -> Vec<u8> {
        let mut mac =
            <SimpleHmac<D>>::new_from_slice(key).expect("hmac accepts keys of any length");
        for part in parts {
            mac.update(part);
        }
        mac.finalize().into_bytes().to_vec()
    };

    let mut a = hmac(secret, &[seed]);
    let mut pos = 0;
    while pos < out.len() {
        let block = hmac(secret, &[&a, seed]);
        let n = (out.len() - pos).min(block.len());
        out[pos..pos + n].copy_from_slice(&block[..n]);
        pos += n;
        a = hmac(secret, &[&a]);
    }
}

/// The TLS 1.0 PRF: `P_MD5(S1, label + seed) XOR P_SHA-1(S2, label + seed)`.
///
/// `S1` and `S2` are the two halves of `secret`. For an odd length they share
/// their middle byte, as the RFC requires.
pub fn prf(secret: &[u8], label: &[u8], seed: &[u8], out: &mut [u8]) {
    let half = secret.len().div_ceil(2);
    let s1 = &secret[..half];
    let s2 = &secret[secret.len() - half..];

    let mut label_and_seed = Vec::with_capacity(label.len() + seed.len());
    label_and_seed.extend_from_slice(label);
    label_and_seed.extend_from_slice(seed);

    let mut from_md5 = vec![0u8; out.len()];
    p_hash::<Md5>(s1, &label_and_seed, &mut from_md5);
    p_hash::<Sha1>(s2, &label_and_seed, out);
    for (byte, mask) in out.iter_mut().zip(from_md5) {
        *byte ^= mask;
    }
}

/// Length of the master secret in bytes.
pub const MASTER_SECRET_LEN: usize = 48;

/// Derive the master secret from the premaster secret and the two randoms.
pub fn master_secret(premaster: &[u8], client_random: &[u8], server_random: &[u8]) -> Vec<u8> {
    let mut seed = Vec::with_capacity(client_random.len() + server_random.len());
    seed.extend_from_slice(client_random);
    seed.extend_from_slice(server_random);

    let mut out = vec![0u8; MASTER_SECRET_LEN];
    prf(premaster, b"master secret", &seed, &mut out);
    out
}

/// The six secrets a CBC cipher suite needs, carved out of the key block.
pub struct KeyBlock {
    /// MAC key for records the client sends.
    pub client_mac: Vec<u8>,
    /// MAC key for records the server sends.
    pub server_mac: Vec<u8>,
    /// Encryption key for records the client sends.
    pub client_key: Vec<u8>,
    /// Encryption key for records the server sends.
    pub server_key: Vec<u8>,
    /// Initial CBC IV for records the client sends.
    pub client_iv: Vec<u8>,
    /// Initial CBC IV for records the server sends.
    pub server_iv: Vec<u8>,
}

/// Expand the master secret into the key block (RFC 2246 §6.3).
///
/// Note the seed order: key expansion puts the **server** random first, the
/// opposite of the master-secret derivation.
pub fn key_block(
    master: &[u8],
    client_random: &[u8],
    server_random: &[u8],
    mac_len: usize,
    key_len: usize,
    iv_len: usize,
) -> KeyBlock {
    let mut seed = Vec::with_capacity(server_random.len() + client_random.len());
    seed.extend_from_slice(server_random);
    seed.extend_from_slice(client_random);

    let mut block = vec![0u8; 2 * (mac_len + key_len + iv_len)];
    prf(master, b"key expansion", &seed, &mut block);

    let mut take = {
        let mut pos = 0;
        move |n: usize| {
            let start = pos;
            pos += n;
            start..pos
        }
    };
    let client_mac = block[take(mac_len)].to_vec();
    let server_mac = block[take(mac_len)].to_vec();
    let client_key = block[take(key_len)].to_vec();
    let server_key = block[take(key_len)].to_vec();
    let client_iv = block[take(iv_len)].to_vec();
    let server_iv = block[take(iv_len)].to_vec();

    KeyBlock { client_mac, server_mac, client_key, server_key, client_iv, server_iv }
}

/// Length of the `verify_data` in a Finished message.
pub const VERIFY_DATA_LEN: usize = 12;

/// Compute the `verify_data` of a Finished message (RFC 2246 §7.4.9).
///
/// `label` is `"client finished"` or `"server finished"`; `handshake` is every
/// handshake message exchanged so far, which is what makes Finished an
/// end-to-end check on the whole negotiation.
pub fn finished_verify_data(master: &[u8], label: &[u8], handshake: &[u8]) -> Vec<u8> {
    let mut seed = Vec::with_capacity(16 + 20);
    seed.extend_from_slice(&Md5::digest(handshake));
    seed.extend_from_slice(&Sha1::digest(handshake));

    let mut out = vec![0u8; VERIFY_DATA_LEN];
    prf(master, label, &seed, &mut out);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Published TLS 1.0 PRF test vector (IETF TLS working group, 2006):
    /// a 48-byte secret of `0xab`, the label `PRF Testvector`, and a 64-byte
    /// seed of `0xcd`.
    ///
    /// This is the one check that is independent of our own code — every other
    /// derived value here would be self-consistent with a wrong PRF.
    #[test]
    fn prf_matches_the_published_test_vector() {
        let secret = [0xabu8; 48];
        let seed = [0xcdu8; 64];
        let mut out = [0u8; 104];
        prf(&secret, b"PRF Testvector", &seed, &mut out);

        let expected: [u8; 104] = [
            0xd3, 0xd4, 0xd1, 0xe3, 0x49, 0xb5, 0xd5, 0x15, 0x04, 0x46, 0x66, 0xd5, 0x1d, 0xe3,
            0x2b, 0xab, 0x25, 0x8c, 0xb5, 0x21, 0xb6, 0xb0, 0x53, 0x46, 0x3e, 0x35, 0x48, 0x32,
            0xfd, 0x97, 0x67, 0x54, 0x44, 0x3b, 0xcf, 0x9a, 0x29, 0x65, 0x19, 0xbc, 0x28, 0x9a,
            0xbc, 0xbc, 0x11, 0x87, 0xe4, 0xeb, 0xd3, 0x1e, 0x60, 0x23, 0x53, 0x77, 0x6c, 0x40,
            0x8a, 0xaf, 0xb7, 0x4c, 0xbc, 0x85, 0xef, 0xf6, 0x92, 0x55, 0xf9, 0x78, 0x8f, 0xaa,
            0x18, 0x4c, 0xbb, 0x95, 0x7a, 0x98, 0x19, 0xd8, 0x4a, 0x5d, 0x7e, 0xb0, 0x06, 0xeb,
            0x45, 0x9d, 0x3a, 0xe8, 0xde, 0x98, 0x10, 0x45, 0x4b, 0x8b, 0x2d, 0x8f, 0x1a, 0xfb,
            0xc6, 0x55, 0xa8, 0xc9, 0xa0, 0x13,
        ];
        assert_eq!(out, expected);
    }

    #[test]
    fn prf_output_is_independent_of_the_requested_length() {
        let secret = [0x5au8; 48];
        let seed = [0xa5u8; 32];
        let mut long = [0u8; 96];
        let mut short = [0u8; 20];
        prf(&secret, b"key expansion", &seed, &mut long);
        prf(&secret, b"key expansion", &seed, &mut short);
        // A shorter request must be a prefix of a longer one, otherwise the
        // block-stitching in `p_hash` is off.
        assert_eq!(&long[..short.len()], &short[..]);
    }

    #[test]
    fn a_different_label_changes_the_output() {
        let secret = [0x11u8; 48];
        let seed = [0x22u8; 32];
        let mut client = [0u8; 12];
        let mut server = [0u8; 12];
        prf(&secret, b"client finished", &seed, &mut client);
        prf(&secret, b"server finished", &seed, &mut server);
        assert_ne!(client, server);
    }

    #[test]
    fn key_block_partitions_in_the_documented_order() {
        let master = [0x01u8; 48];
        let client_random = [0x02u8; 32];
        let server_random = [0x03u8; 32];
        let block = key_block(&master, &client_random, &server_random, 20, 16, 16);

        assert_eq!(block.client_mac.len(), 20);
        assert_eq!(block.server_mac.len(), 20);
        assert_eq!(block.client_key.len(), 16);
        assert_eq!(block.server_key.len(), 16);
        assert_eq!(block.client_iv.len(), 16);
        assert_eq!(block.server_iv.len(), 16);

        // Reproduce the raw expansion and check the slices line up with it.
        let mut seed = Vec::new();
        seed.extend_from_slice(&server_random);
        seed.extend_from_slice(&client_random);
        let mut raw = vec![0u8; 2 * (20 + 16 + 16)];
        prf(&master, b"key expansion", &seed, &mut raw);
        assert_eq!(block.client_mac, raw[0..20]);
        assert_eq!(block.server_mac, raw[20..40]);
        assert_eq!(block.client_key, raw[40..56]);
        assert_eq!(block.server_key, raw[56..72]);
        assert_eq!(block.client_iv, raw[72..88]);
        assert_eq!(block.server_iv, raw[88..104]);
    }

    #[test]
    fn master_secret_seed_order_differs_from_key_expansion() {
        let premaster = [0x07u8; 48];
        let a = [0x08u8; 32];
        let b = [0x09u8; 32];
        // Swapping the randoms must change the result; if it did not, the
        // client/server order would be unverifiable.
        assert_ne!(master_secret(&premaster, &a, &b), master_secret(&premaster, &b, &a));
        assert_eq!(master_secret(&premaster, &a, &b).len(), MASTER_SECRET_LEN);
    }

    #[test]
    fn finished_verify_data_covers_every_handshake_byte() {
        let master = [0x33u8; 48];
        let first = finished_verify_data(&master, b"client finished", b"handshake-bytes");
        let second = finished_verify_data(&master, b"client finished", b"handshake-byteS");
        assert_eq!(first.len(), VERIFY_DATA_LEN);
        assert_ne!(first, second);
    }
}
