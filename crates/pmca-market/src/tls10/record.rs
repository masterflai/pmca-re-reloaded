//! The TLS 1.0 record layer for CBC cipher suites.
//!
//! Records carry a type, a version and a length, followed by a fragment that is
//! plaintext until the cipher is switched on and MAC-then-encrypt afterwards.
//!
//! TLS 1.0 chains the CBC initialisation vector: the IV of a record is the last
//! ciphertext block of the record before it. This is the flaw TLS 1.1 fixed
//! with explicit per-record IVs, and it is what makes BEAST possible against a
//! browser. It is reproduced here because the camera implements TLS 1.0 and
//! nothing else, and because the connection never leaves the loopback
//! interface.

use aes::cipher::{BlockModeDecrypt, BlockModeEncrypt, KeyIvInit, block_padding::NoPadding};
use aes::{Aes128, Aes256};
use hmac::{KeyInit, Mac, SimpleHmac};
use sha1::Sha1;

use super::{Error, Result};

/// AES block size, and therefore the CBC IV length.
pub const BLOCK_LEN: usize = 16;
/// Length of an HMAC-SHA1 tag.
pub const MAC_LEN: usize = 20;
/// Largest plaintext fragment a record may carry (RFC 2246 §6.2.1).
pub const MAX_FRAGMENT: usize = 16384;
/// The version on every record and handshake message: TLS 1.0 is `{3, 1}`.
pub const VERSION: [u8; 2] = [0x03, 0x01];

/// Record content types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentType {
    /// The one-byte message that switches the cipher on.
    ChangeCipherSpec,
    /// A warning or fatal alert.
    Alert,
    /// A handshake message.
    Handshake,
    /// Payload bytes handed to the application.
    ApplicationData,
}

impl ContentType {
    /// Map the wire byte to a content type.
    pub fn from_byte(value: u8) -> Option<Self> {
        match value {
            20 => Some(ContentType::ChangeCipherSpec),
            21 => Some(ContentType::Alert),
            22 => Some(ContentType::Handshake),
            23 => Some(ContentType::ApplicationData),
            _ => None,
        }
    }

    /// The wire byte for this content type.
    pub fn as_byte(self) -> u8 {
        match self {
            ContentType::ChangeCipherSpec => 20,
            ContentType::Alert => 21,
            ContentType::Handshake => 22,
            ContentType::ApplicationData => 23,
        }
    }
}

/// Which AES key size the negotiated suite uses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cipher {
    /// `TLS_RSA_WITH_AES_128_CBC_SHA`.
    Aes128,
    /// `TLS_RSA_WITH_AES_256_CBC_SHA`.
    Aes256,
}

impl Cipher {
    /// Key length in bytes.
    pub fn key_len(self) -> usize {
        match self {
            Cipher::Aes128 => 16,
            Cipher::Aes256 => 32,
        }
    }

    fn encrypt_cbc(self, key: &[u8], iv: &[u8], buf: &mut [u8]) -> Result<()> {
        let ok = match self {
            Cipher::Aes128 => cbc::Encryptor::<Aes128>::new_from_slices(key, iv)
                .map(|c| c.encrypt_padded::<NoPadding>(buf, buf.len()).is_ok()),
            Cipher::Aes256 => cbc::Encryptor::<Aes256>::new_from_slices(key, iv)
                .map(|c| c.encrypt_padded::<NoPadding>(buf, buf.len()).is_ok()),
        };
        match ok {
            Ok(true) => Ok(()),
            _ => Err(Error::Tls("record encryption failed".into())),
        }
    }

    fn decrypt_cbc(self, key: &[u8], iv: &[u8], buf: &mut [u8]) -> Result<()> {
        let ok = match self {
            Cipher::Aes128 => cbc::Decryptor::<Aes128>::new_from_slices(key, iv)
                .map(|c| c.decrypt_padded::<NoPadding>(buf).is_ok()),
            Cipher::Aes256 => cbc::Decryptor::<Aes256>::new_from_slices(key, iv)
                .map(|c| c.decrypt_padded::<NoPadding>(buf).is_ok()),
        };
        match ok {
            Ok(true) => Ok(()),
            _ => Err(Error::Tls("record decryption failed".into())),
        }
    }
}

/// Keys and running state for one direction of the connection.
pub struct CipherState {
    cipher: Cipher,
    mac_key: Vec<u8>,
    key: Vec<u8>,
    /// The IV for the next record: initially from the key block, thereafter the
    /// previous record's final ciphertext block.
    iv: Vec<u8>,
    /// Record counter fed into the MAC; it is never allowed to wrap.
    sequence: u64,
}

impl CipherState {
    /// Build the state for one direction.
    pub fn new(cipher: Cipher, mac_key: Vec<u8>, key: Vec<u8>, iv: Vec<u8>) -> Self {
        CipherState { cipher, mac_key, key, iv, sequence: 0 }
    }

    /// Compute the record MAC over the sequence number, header and fragment.
    ///
    /// The length in the MAC input is that of the *plaintext* fragment, not of
    /// the encrypted record.
    fn mac(&self, content_type: ContentType, fragment: &[u8]) -> Vec<u8> {
        let mut hmac = <SimpleHmac<Sha1>>::new_from_slice(&self.mac_key)
            .expect("hmac accepts keys of any length");
        hmac.update(&self.sequence.to_be_bytes());
        hmac.update(&[content_type.as_byte()]);
        hmac.update(&VERSION);
        hmac.update(&(fragment.len() as u16).to_be_bytes());
        hmac.update(fragment);
        hmac.finalize().into_bytes().to_vec()
    }

    /// Advance the record counter, refusing to wrap it.
    ///
    /// A repeated sequence number would let records be replayed, so exhaustion
    /// has to end the connection. At one record per byte this is unreachable in
    /// practice; the check costs nothing and removes the need to reason about
    /// it.
    fn advance(&mut self) -> Result<()> {
        self.sequence =
            self.sequence.checked_add(1).ok_or(Error::Tls("record sequence exhausted".into()))?;
        Ok(())
    }

    /// MAC, pad and encrypt a fragment into a record payload.
    pub fn seal(&mut self, content_type: ContentType, fragment: &[u8]) -> Result<Vec<u8>> {
        let mut buf = Vec::with_capacity(fragment.len() + MAC_LEN + BLOCK_LEN);
        buf.extend_from_slice(fragment);
        buf.extend_from_slice(&self.mac(content_type, fragment));

        // Pad to a whole number of blocks. Every padding byte, including the
        // trailing length byte, holds `pad_len - 1`.
        let pad_len = BLOCK_LEN - (buf.len() % BLOCK_LEN);
        buf.resize(buf.len() + pad_len, (pad_len - 1) as u8);

        self.cipher.encrypt_cbc(&self.key, &self.iv, &mut buf)?;
        // Chain the IV for the next record.
        self.iv = buf[buf.len() - BLOCK_LEN..].to_vec();
        self.advance()?;
        Ok(buf)
    }

    /// Decrypt a record payload, strip the padding and verify the MAC.
    pub fn open(&mut self, content_type: ContentType, payload: &[u8]) -> Result<Vec<u8>> {
        if payload.is_empty() || !payload.len().is_multiple_of(BLOCK_LEN) {
            return Err(Error::Tls("encrypted record has a bad length".into()));
        }
        let next_iv = payload[payload.len() - BLOCK_LEN..].to_vec();
        let mut buf = payload.to_vec();
        self.cipher.decrypt_cbc(&self.key, &self.iv, &mut buf)?;
        self.iv = next_iv;

        let pad_len = *buf.last().expect("payload is not empty") as usize;
        // The padding byte counts the bytes before the length byte itself.
        if pad_len + 1 + MAC_LEN > buf.len() {
            return Err(Error::Tls("record padding is inconsistent".into()));
        }
        let padding_start = buf.len() - pad_len - 1;
        if buf[padding_start..].iter().any(|&b| b as usize != pad_len) {
            return Err(Error::Tls("record padding is malformed".into()));
        }

        let mac_start = padding_start - MAC_LEN;
        let fragment = buf[..mac_start].to_vec();
        let expected = self.mac(content_type, &fragment);
        // Compare the whole tag: an early exit would time-leak how much of it
        // matched.
        let received = &buf[mac_start..padding_start];
        let differing = expected.iter().zip(received).fold(0u8, |acc, (a, b)| acc | (a ^ b));
        if differing != 0 {
            return Err(Error::Tls("record mac mismatch".into()));
        }

        self.advance()?;
        Ok(fragment)
    }
}

/// Serialise a record header for `content_type` and a payload length.
pub fn record_header(content_type: ContentType, len: usize) -> [u8; 5] {
    let len = (len as u16).to_be_bytes();
    [content_type.as_byte(), VERSION[0], VERSION[1], len[0], len[1]]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn state() -> CipherState {
        CipherState::new(Cipher::Aes128, vec![0x11; MAC_LEN], vec![0x22; 16], vec![0x33; BLOCK_LEN])
    }

    /// Seal on one state, open on a second one primed identically: this is
    /// exactly how the two ends of a real connection are set up.
    fn pair() -> (CipherState, CipherState) {
        (state(), state())
    }

    #[test]
    fn a_sealed_record_opens_back_to_the_original_fragment() {
        let (mut sender, mut receiver) = pair();
        let payload = sender.seal(ContentType::ApplicationData, b"hello camera").unwrap();
        let opened = receiver.open(ContentType::ApplicationData, &payload).unwrap();
        assert_eq!(opened, b"hello camera");
    }

    #[test]
    fn sealed_records_are_always_a_whole_number_of_blocks() {
        for len in 0..64usize {
            let mut sender = state();
            let fragment = vec![0x5a; len];
            let payload = sender.seal(ContentType::ApplicationData, &fragment).unwrap();
            assert_eq!(payload.len() % BLOCK_LEN, 0, "length {len} produced {}", payload.len());
            // Content plus MAC plus at least one padding byte.
            assert!(payload.len() > len + MAC_LEN);
        }
    }

    #[test]
    fn several_records_survive_the_chained_iv() {
        let (mut sender, mut receiver) = pair();
        for i in 0..8u8 {
            let fragment = vec![i; 10 + i as usize];
            let payload = sender.seal(ContentType::ApplicationData, &fragment).unwrap();
            assert_eq!(receiver.open(ContentType::ApplicationData, &payload).unwrap(), fragment);
        }
    }

    #[test]
    fn a_receiver_that_loses_iv_chaining_fails_on_the_second_record() {
        let mut sender = state();
        let mut receiver = state();
        let first = sender.seal(ContentType::ApplicationData, b"first").unwrap();
        let second = sender.seal(ContentType::ApplicationData, b"second").unwrap();
        receiver.open(ContentType::ApplicationData, &first).unwrap();

        // A receiver whose IV was not carried over decrypts the leading block
        // to garbage; the MAC or the padding then rejects it.
        let mut naive = state();
        naive.sequence = 1;
        assert!(naive.open(ContentType::ApplicationData, &second).is_err());
        // The correctly chained receiver accepts the same record.
        assert_eq!(receiver.open(ContentType::ApplicationData, &second).unwrap(), b"second");
    }

    #[test]
    fn a_tampered_record_is_rejected() {
        let (mut sender, mut receiver) = pair();
        let mut payload = sender.seal(ContentType::ApplicationData, b"transfer 100").unwrap();
        payload[0] ^= 0x01;
        assert!(receiver.open(ContentType::ApplicationData, &payload).is_err());
    }

    #[test]
    fn the_content_type_is_covered_by_the_mac() {
        let (mut sender, mut receiver) = pair();
        let payload = sender.seal(ContentType::Handshake, b"finished").unwrap();
        // Opening under a different type must fail: the type is a MAC input.
        assert!(receiver.open(ContentType::ApplicationData, &payload).is_err());
    }

    #[test]
    fn the_sequence_number_is_covered_by_the_mac() {
        let mut sender = state();
        let first = sender.seal(ContentType::ApplicationData, b"replay me").unwrap();
        // Replaying the first record against a receiver that already consumed
        // one record must fail.
        let mut receiver = state();
        receiver.open(ContentType::ApplicationData, &first).unwrap();
        assert!(receiver.open(ContentType::ApplicationData, &first).is_err());
    }

    #[test]
    fn a_record_of_a_non_block_length_is_rejected() {
        let mut receiver = state();
        assert!(receiver.open(ContentType::ApplicationData, &[0u8; 17]).is_err());
        assert!(receiver.open(ContentType::ApplicationData, &[]).is_err());
    }

    #[test]
    fn a_record_too_short_to_hold_a_mac_is_rejected() {
        let mut receiver = state();
        // One valid-looking block cannot contain a 20-byte MAC.
        assert!(receiver.open(ContentType::ApplicationData, &[0u8; BLOCK_LEN]).is_err());
    }

    #[test]
    fn aes256_round_trips_as_well() {
        let make = || {
            CipherState::new(
                Cipher::Aes256,
                vec![0x44; MAC_LEN],
                vec![0x55; 32],
                vec![0x66; BLOCK_LEN],
            )
        };
        let (mut sender, mut receiver) = (make(), make());
        let payload = sender.seal(ContentType::ApplicationData, b"wider key").unwrap();
        assert_eq!(receiver.open(ContentType::ApplicationData, &payload).unwrap(), b"wider key");
    }

    #[test]
    fn key_lengths_match_the_suite() {
        assert_eq!(Cipher::Aes128.key_len(), 16);
        assert_eq!(Cipher::Aes256.key_len(), 32);
    }

    #[test]
    fn content_types_round_trip_through_their_wire_bytes() {
        for kind in [
            ContentType::ChangeCipherSpec,
            ContentType::Alert,
            ContentType::Handshake,
            ContentType::ApplicationData,
        ] {
            assert_eq!(ContentType::from_byte(kind.as_byte()), Some(kind));
        }
        assert_eq!(ContentType::from_byte(99), None);
    }

    #[test]
    fn the_record_header_carries_type_version_and_length() {
        assert_eq!(record_header(ContentType::Handshake, 0x0134), [22, 3, 1, 0x01, 0x34]);
    }
}
