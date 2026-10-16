//! Reading an RSA private key and decrypting the premaster secret.
//!
//! TLS 1.0's RSA key exchange has the client pick the premaster secret and send
//! it encrypted under the server's public key. The server therefore needs a raw
//! PKCS#1 v1.5 *decryption*, which the modern Rust TLS stack does not expose
//! because no current cipher suite uses it.
//!
//! The key handled here belongs to the install server's throwaway loopback
//! certificate, which [`super::selfsigned`] makes up when the server starts.

use num_bigint::BigUint;

/// An RSA private key, reduced to what decryption needs.
pub struct RsaPrivateKey {
    modulus: BigUint,
    private_exponent: BigUint,
    /// Byte length of the modulus; ciphertexts and padded plaintexts use it.
    size: usize,
}

/// A cursor over DER-encoded data.
struct Der<'a> {
    data: &'a [u8],
}

impl<'a> Der<'a> {
    fn new(data: &'a [u8]) -> Self {
        Der { data }
    }

    /// Read one tag-length-value triple, requiring `tag`, and return its value.
    fn take(&mut self, tag: u8) -> Option<&'a [u8]> {
        let (&actual, rest) = self.data.split_first()?;
        if actual != tag {
            return None;
        }
        let (&first, rest) = rest.split_first()?;
        let (len, rest) = if first < 0x80 {
            (first as usize, rest)
        } else {
            // Long form: the low bits count the length bytes that follow.
            let count = (first & 0x7f) as usize;
            // A length needing more than 4 bytes cannot describe a key we can
            // hold in memory, and `usize` may be 32-bit.
            if count == 0 || count > 4 || rest.len() < count {
                return None;
            }
            let (bytes, rest) = rest.split_at(count);
            let len = bytes.iter().fold(0usize, |acc, &b| (acc << 8) | b as usize);
            (len, rest)
        };
        if rest.len() < len {
            return None;
        }
        let (value, rest) = rest.split_at(len);
        self.data = rest;
        Some(value)
    }

    /// Read an INTEGER as an unsigned big-endian number.
    fn take_uint(&mut self) -> Option<BigUint> {
        Some(BigUint::from_bytes_be(self.take(0x02)?))
    }
}

impl RsaPrivateKey {
    /// Parse a PKCS#1 `RSAPrivateKey` DER structure.
    pub fn from_pkcs1_der(der: &[u8]) -> Option<Self> {
        let mut outer = Der::new(der);
        let body = outer.take(0x30)?;
        let mut fields = Der::new(body);

        let version = fields.take_uint()?;
        // Version 1 means multi-prime, which our modulus/exponent-only
        // decryption cannot represent.
        if version != BigUint::from(0u32) {
            return None;
        }
        let modulus = fields.take_uint()?;
        let _public_exponent = fields.take_uint()?;
        let private_exponent = fields.take_uint()?;

        let size = modulus.bits().div_ceil(8) as usize;
        if size == 0 {
            return None;
        }
        Some(RsaPrivateKey { modulus, private_exponent, size })
    }

    /// Parse a PKCS#8 `PrivateKeyInfo` wrapping an RSA key.
    pub fn from_pkcs8_der(der: &[u8]) -> Option<Self> {
        let mut outer = Der::new(der);
        let body = outer.take(0x30)?;
        let mut fields = Der::new(body);

        let _version = fields.take_uint()?;
        let _algorithm = fields.take(0x30)?;
        let inner = fields.take(0x04)?;
        Self::from_pkcs1_der(inner)
    }

    /// Build a key from a modulus and private exponent that were just
    /// generated, rather than parsed.
    ///
    /// Decryption needs nothing else. The primes are deliberately not kept:
    /// this implementation does not use the Chinese remainder speed-up, so
    /// holding on to them would only widen what a memory dump reveals.
    pub fn from_components(modulus: BigUint, private_exponent: BigUint) -> Option<Self> {
        let size = modulus.bits().div_ceil(8) as usize;
        if size == 0 {
            return None;
        }
        Some(RsaPrivateKey { modulus, private_exponent, size })
    }

    /// Size of the modulus in bytes, which is also the ciphertext length.
    ///
    /// Decryption reads the field directly, so this exists only for the tests
    /// that check a key came out the size it was asked for.
    #[cfg(test)]
    pub fn size(&self) -> usize {
        self.size
    }

    /// Decrypt a PKCS#1 v1.5 encrypted block, returning the payload.
    ///
    /// Returns `None` whenever anything is wrong. Callers must **not** turn
    /// that into a distinguishable response: revealing whether the padding was
    /// well-formed is the Bleichenbacher oracle, so the caller substitutes a
    /// random premaster secret and lets the handshake fail later at Finished
    /// (RFC 5246 §7.4.7.1, whose advice applies unchanged to TLS 1.0).
    pub fn decrypt_pkcs1v15(&self, ciphertext: &[u8]) -> Option<Vec<u8>> {
        if ciphertext.len() != self.size {
            return None;
        }
        let c = BigUint::from_bytes_be(ciphertext);
        if c >= self.modulus {
            return None;
        }
        let m = c.modpow(&self.private_exponent, &self.modulus);

        // Left-pad to the modulus length: the leading zero of an EME-PKCS1
        // block is dropped by the integer representation.
        let raw = m.to_bytes_be();
        if raw.len() > self.size {
            return None;
        }
        let mut block = vec![0u8; self.size - raw.len()];
        block.extend_from_slice(&raw);

        // Expect 0x00 0x02 <at least 8 non-zero bytes> 0x00 <payload>.
        if block.len() < 11 || block[0] != 0x00 || block[1] != 0x02 {
            return None;
        }
        let separator = block[2..].iter().position(|&b| b == 0x00)? + 2;
        if separator < 10 {
            return None;
        }
        Some(block[separator + 1..].to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A tiny RSA key built by hand, so the tests need no key file.
    ///
    /// p = 61, q = 53, n = 3233, e = 17, d = 413 — the textbook example. It is
    /// far too small to encrypt anything real, but it exercises the modular
    /// arithmetic and the block handling exactly like a 2048-bit key.
    fn toy_key() -> RsaPrivateKey {
        RsaPrivateKey {
            modulus: BigUint::from(3233u32),
            private_exponent: BigUint::from(413u32),
            size: 2,
        }
    }

    /// Encrypt with the toy key's public exponent.
    fn toy_encrypt(block: &[u8]) -> Vec<u8> {
        let m = BigUint::from_bytes_be(block);
        let c = m.modpow(&BigUint::from(17u32), &BigUint::from(3233u32));
        let raw = c.to_bytes_be();
        let mut out = vec![0u8; 2 - raw.len()];
        out.extend_from_slice(&raw);
        out
    }

    #[test]
    fn toy_key_round_trips_through_modular_exponentiation() {
        let plain = [0x0cu8, 0x1e];
        let cipher = toy_encrypt(&plain);
        let key = toy_key();
        let m = BigUint::from_bytes_be(&cipher)
            .modpow(&key.private_exponent, &key.modulus)
            .to_bytes_be();
        assert_eq!(m, plain);
    }

    #[test]
    fn a_ciphertext_of_the_wrong_length_is_rejected() {
        let key = toy_key();
        assert!(key.decrypt_pkcs1v15(&[0x00]).is_none());
        assert!(key.decrypt_pkcs1v15(&[0x00, 0x01, 0x02]).is_none());
    }

    #[test]
    fn a_ciphertext_at_or_above_the_modulus_is_rejected() {
        let key = toy_key();
        // 3233 == 0x0CA1, exactly the modulus.
        assert!(key.decrypt_pkcs1v15(&[0x0c, 0xa1]).is_none());
    }

    /// Builds a PKCS#1 DER key so the parser can be tested end to end.
    fn der_uint(value: &[u8]) -> Vec<u8> {
        let mut body = Vec::new();
        // DER integers are signed, so a high bit needs a leading zero.
        if value.first().is_some_and(|&b| b >= 0x80) {
            body.push(0x00);
        }
        body.extend_from_slice(value);
        let mut out = vec![0x02];
        out.extend_from_slice(&der_len(body.len()));
        out.extend_from_slice(&body);
        out
    }

    fn der_len(len: usize) -> Vec<u8> {
        if len < 0x80 {
            vec![len as u8]
        } else if len <= 0xff {
            vec![0x81, len as u8]
        } else {
            vec![0x82, (len >> 8) as u8, len as u8]
        }
    }

    fn der_seq(body: &[u8]) -> Vec<u8> {
        let mut out = vec![0x30];
        out.extend_from_slice(&der_len(body.len()));
        out.extend_from_slice(body);
        out
    }

    fn pkcs1_der(modulus: &[u8], private_exponent: &[u8]) -> Vec<u8> {
        let mut body = Vec::new();
        body.extend_from_slice(&der_uint(&[0x00])); // version
        body.extend_from_slice(&der_uint(modulus));
        body.extend_from_slice(&der_uint(&[0x01, 0x00, 0x01])); // e
        body.extend_from_slice(&der_uint(private_exponent));
        body.extend_from_slice(&der_uint(&[0x02])); // p
        body.extend_from_slice(&der_uint(&[0x03])); // q
        der_seq(&body)
    }

    #[test]
    fn pkcs1_keys_are_parsed_into_modulus_and_exponent() {
        let der = pkcs1_der(&[0x0c, 0xa1], &[0x01, 0x9d]);
        let key = RsaPrivateKey::from_pkcs1_der(&der).expect("valid pkcs#1 key");
        assert_eq!(key.modulus, BigUint::from(3233u32));
        assert_eq!(key.private_exponent, BigUint::from(413u32));
        assert_eq!(key.size(), 2);
    }

    #[test]
    fn pkcs8_keys_are_unwrapped_before_parsing() {
        let inner = pkcs1_der(&[0x0c, 0xa1], &[0x01, 0x9d]);
        let mut octet_string = vec![0x04];
        octet_string.extend_from_slice(&der_len(inner.len()));
        octet_string.extend_from_slice(&inner);

        let mut body = Vec::new();
        body.extend_from_slice(&der_uint(&[0x00]));
        body.extend_from_slice(&der_seq(&[])); // AlgorithmIdentifier
        body.extend_from_slice(&octet_string);

        let key = RsaPrivateKey::from_pkcs8_der(&der_seq(&body)).expect("valid pkcs#8 key");
        assert_eq!(key.modulus, BigUint::from(3233u32));
    }

    #[test]
    fn a_long_form_length_is_decoded() {
        // 200 bytes of payload forces the 0x81 long form.
        let modulus = vec![0x01u8; 200];
        let der = pkcs1_der(&modulus, &[0x01, 0x9d]);
        let key = RsaPrivateKey::from_pkcs1_der(&der).expect("valid key");
        assert_eq!(key.size(), 200);
    }

    #[test]
    fn truncated_der_is_rejected_rather_than_panicking() {
        let der = pkcs1_der(&[0x0c, 0xa1], &[0x01, 0x9d]);
        for cut in 0..der.len() {
            // Any prefix must be refused; none may panic.
            assert!(RsaPrivateKey::from_pkcs1_der(&der[..cut]).is_none(), "accepted {cut} bytes");
        }
    }

    #[test]
    fn a_wrong_outer_tag_is_rejected() {
        let mut der = pkcs1_der(&[0x0c, 0xa1], &[0x01, 0x9d]);
        der[0] = 0x31; // SET instead of SEQUENCE
        assert!(RsaPrivateKey::from_pkcs1_der(&der).is_none());
    }

    #[test]
    fn multi_prime_keys_are_refused() {
        let mut body = Vec::new();
        body.extend_from_slice(&der_uint(&[0x01])); // version 1
        body.extend_from_slice(&der_uint(&[0x0c, 0xa1]));
        body.extend_from_slice(&der_uint(&[0x01, 0x00, 0x01]));
        body.extend_from_slice(&der_uint(&[0x01, 0x9d]));
        assert!(RsaPrivateKey::from_pkcs1_der(&der_seq(&body)).is_none());
    }
}
