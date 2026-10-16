//! Generating the install server's certificate when the server starts.
//!
//! # Why this is not a file
//!
//! For years this project shipped a wildcard certificate for `*.localtest.me`
//! whose private key had been published by that project. It worked, but it was
//! a stranger's key, it had expired in 2013, and every secret scanner that
//! looked at the repository found it.
//!
//! Making one up at run time removes all three problems at once. Nothing is
//! stored, so there is nothing to leak, nothing to expire, and nothing that
//! belongs to anybody else. The key lives for one install session.
//!
//! # What the camera checks
//!
//! Not nothing, as this module once assumed — that assumption shipped untested
//! and every install then failed with the camera's own `201: Error completed
//! (SSL error)`. Measured on a camera (2026-10-02):
//!
//! | Certificate presented | Result |
//! |---|---|
//! | Self-signed, any subject | rejected |
//! | Issuer named after an intermediate the camera does not know | rejected |
//! | Issuer named after a root the camera knows, signature **invalid** | accepted |
//!
//! So the camera looks the issuer's *name* up among its built-in roots and
//! stops there: it never verifies the signature, and ignores both validity
//! dates and host names. This certificate therefore names a root that camera
//! firmware of that era carries, [`ISSUER`], and is still signed with its own
//! throwaway key. Nothing of that root is used except its name — no key, no
//! certificate bytes — and the signature is invalid for anyone who checks.
//!
//! # What this is not
//!
//! A general-purpose certificate generator. It writes exactly the structure
//! this one peer accepts and no more. In particular there is no attempt at
//! constant-time arithmetic: the key is thrown away minutes later, the peer is
//! a USB device on the other end of a loopback socket, and the surrounding
//! protocol is TLS 1.0, which offers no forward secrecy anyway. If any of that
//! stops being true, this module is the wrong tool.

use std::time::{SystemTime, UNIX_EPOCH};

use num_bigint::{BigInt, BigUint};
use sha2::{Digest, Sha256};

use super::random_bytes;
use super::rsa::RsaPrivateKey;
use crate::{Error, Result};

/// Bits in the modulus.
///
/// 2048 rather than something smaller because the number is what an auditor
/// reads first, and because a few hundred milliseconds once per install session
/// is not a cost anybody notices.
const MODULUS_BITS: usize = 2048;

/// The public exponent, F4. Small enough to verify quickly, and the value every
/// implementation has been exercised against.
const PUBLIC_EXPONENT: u32 = 65537;

/// Trial division limit. Composites with a factor below this are discarded
/// before the expensive test runs, which is the great majority of them.
const SIEVE_LIMIT: usize = 4096;

/// A freshly generated key together with the certificate that carries it.
pub struct SelfSigned {
    /// The DER-encoded certificate, ready for the Certificate handshake
    /// message.
    pub certificate: Vec<u8>,
    pub key: RsaPrivateKey,
}

/// Generate a 2048-bit RSA key and a certificate for it, signed with that same
/// key but naming [`ISSUER`] as its issuer.
pub fn generate() -> Result<SelfSigned> {
    let sieve = small_primes(SIEVE_LIMIT);
    let half = MODULUS_BITS / 2;

    // Two distinct primes. Equal primes would make the modulus a square and the
    // key trivially breakable; drawing the same 1024-bit prime twice will not
    // happen, but the check costs one comparison.
    let p = generate_prime(half, &sieve)?;
    let q = loop {
        let candidate = generate_prime(half, &sieve)?;
        if candidate != p {
            break candidate;
        }
    };

    let one = BigUint::from(1u32);
    let n = &p * &q;
    let e = BigUint::from(PUBLIC_EXPONENT);
    let phi = (&p - &one) * (&q - &one);
    let d = modinv(&e, &phi)
        .ok_or_else(|| Error::Tls("generated key has no private exponent".into()))?;

    let certificate = build_certificate(&n, &e, &d)?;
    let key = RsaPrivateKey::from_components(n, d)
        .ok_or_else(|| Error::Tls("generated key is unusable".into()))?;
    Ok(SelfSigned { certificate, key })
}

// ---------------------------------------------------------------------------
// Prime generation
// ---------------------------------------------------------------------------

/// Every prime below `limit`, by sieve of Eratosthenes.
fn small_primes(limit: usize) -> Vec<u32> {
    let mut is_prime = vec![true; limit];
    is_prime[0] = false;
    if limit > 1 {
        is_prime[1] = false;
    }
    let mut i = 2;
    while i * i < limit {
        if is_prime[i] {
            let mut multiple = i * i;
            while multiple < limit {
                is_prime[multiple] = false;
                multiple += i;
            }
        }
        i += 1;
    }
    is_prime.iter().enumerate().filter(|&(_, &p)| p).map(|(i, _)| i as u32).collect()
}

/// Draw a random odd number of exactly `bits` bits, with the top two bits set.
///
/// Setting the second bit as well is the standard trick that makes the product
/// of two such primes land on exactly `2 * bits` bits, so the modulus length is
/// known in advance instead of being one bit short half the time.
fn random_candidate(bits: usize) -> Result<BigUint> {
    let mut buf = vec![0u8; bits / 8];
    random_bytes(&mut buf)?;
    buf[0] |= 0b1100_0000;
    let last = buf.len() - 1;
    buf[last] |= 1;
    Ok(BigUint::from_bytes_be(&buf))
}

/// Search for a prime of `bits` bits that is usable with [`PUBLIC_EXPONENT`].
fn generate_prime(bits: usize, sieve: &[u32]) -> Result<BigUint> {
    let zero = BigUint::from(0u32);
    let one = BigUint::from(1u32);
    loop {
        let candidate = random_candidate(bits)?;

        // Cheap rejection first. Roughly nine out of ten composites die here.
        if sieve.iter().any(|&small| &candidate % small == zero) {
            continue;
        }
        // `e` must be invertible modulo `p - 1`, which for a prime `e` means
        // `e` must not divide it.
        if (&candidate - &one) % PUBLIC_EXPONENT == zero {
            continue;
        }
        if is_probable_prime(&candidate, sieve) {
            return Ok(candidate);
        }
    }
}

/// Miller-Rabin, with the first 16 primes as bases.
///
/// Fixed bases are sound here because the candidate is drawn at random rather
/// than chosen by an adversary. A number that is a strong pseudoprime to all 16
/// bases exists, but the chance of stumbling onto one by drawing 1024 random
/// bits is far below the chance of the machine miscomputing the test.
fn is_probable_prime(n: &BigUint, sieve: &[u32]) -> bool {
    let one = BigUint::from(1u32);
    let two = BigUint::from(2u32);
    if n <= &two {
        return n == &two;
    }

    // n - 1 = d * 2^s with d odd.
    let n_minus_one = n - &one;
    let mut d = n_minus_one.clone();
    let mut s = 0u32;
    while &d % 2u32 == BigUint::from(0u32) {
        d >>= 1;
        s += 1;
    }

    'base: for &base in sieve.iter().take(16) {
        let a = BigUint::from(base);
        if &a >= n {
            continue;
        }
        let mut x = a.modpow(&d, n);
        if x == one || x == n_minus_one {
            continue;
        }
        for _ in 1..s {
            x = x.modpow(&two, n);
            if x == n_minus_one {
                continue 'base;
            }
        }
        return false;
    }
    true
}

/// The inverse of `a` modulo `m`, by the extended Euclidean algorithm.
fn modinv(a: &BigUint, m: &BigUint) -> Option<BigUint> {
    let zero = BigInt::from(0u32);
    let (mut old_r, mut r) = (BigInt::from(a.clone()), BigInt::from(m.clone()));
    let (mut old_s, mut s) = (BigInt::from(1u32), zero.clone());

    while r != zero {
        let quotient = &old_r / &r;
        let next_r = &old_r - &quotient * &r;
        old_r = std::mem::replace(&mut r, next_r);
        let next_s = &old_s - &quotient * &s;
        old_s = std::mem::replace(&mut s, next_s);
    }

    if old_r != BigInt::from(1u32) {
        return None;
    }
    let modulus = BigInt::from(m.clone());
    (((old_s % &modulus) + &modulus) % &modulus).to_biguint()
}

// ---------------------------------------------------------------------------
// DER
// ---------------------------------------------------------------------------

/// Encode a DER length, short form below 128 and long form above.
fn der_len(len: usize) -> Vec<u8> {
    if len < 0x80 {
        return vec![len as u8];
    }
    let bytes = len.to_be_bytes();
    let first = bytes.iter().position(|&b| b != 0).expect("a length of zero took the short form");
    let body = &bytes[first..];
    let mut out = Vec::with_capacity(body.len() + 1);
    out.push(0x80 | body.len() as u8);
    out.extend_from_slice(body);
    out
}

/// Wrap `body` in a tag-length-value triple.
fn tlv(tag: u8, body: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(body.len() + 6);
    out.push(tag);
    out.extend_from_slice(&der_len(body.len()));
    out.extend_from_slice(body);
    out
}

/// A DER SEQUENCE built from already-encoded parts.
fn seq(parts: &[&[u8]]) -> Vec<u8> {
    tlv(0x30, &parts.concat())
}

/// A non-negative INTEGER.
///
/// DER integers are signed, so a value whose top bit is set needs a leading
/// zero byte or it would be read as negative.
fn der_uint(value: &BigUint) -> Vec<u8> {
    let mut bytes = value.to_bytes_be();
    if bytes.is_empty() {
        bytes.push(0);
    }
    if bytes[0] & 0x80 != 0 {
        bytes.insert(0, 0);
    }
    tlv(0x02, &bytes)
}

/// `1.2.840.113549.1.1.1` — rsaEncryption.
const OID_RSA_ENCRYPTION: &[u8] =
    &[0x06, 0x09, 0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x01, 0x01];
/// `1.2.840.113549.1.1.11` — sha256WithRSAEncryption.
const OID_SHA256_WITH_RSA: &[u8] =
    &[0x06, 0x09, 0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x01, 0x0b];
/// `2.5.4.3` — commonName.
const OID_COMMON_NAME: &[u8] = &[0x06, 0x03, 0x55, 0x04, 0x03];
/// `2.5.4.6` — countryName.
const OID_COUNTRY: &[u8] = &[0x06, 0x03, 0x55, 0x04, 0x06];
/// `2.5.4.7` — localityName.
const OID_LOCALITY: &[u8] = &[0x06, 0x03, 0x55, 0x04, 0x07];
/// `2.5.4.8` — stateOrProvinceName.
const OID_STATE: &[u8] = &[0x06, 0x03, 0x55, 0x04, 0x08];
/// `2.5.4.10` — organizationName.
const OID_ORGANIZATION: &[u8] = &[0x06, 0x03, 0x55, 0x04, 0x0a];

/// The issuer name, in order: the subject of the "COMODO Certification
/// Authority" root, which the camera accepts as an issuer (see the module
/// documentation for why the name alone is enough).
///
/// Encoded as PrintableString like the root itself, so the bytes match exactly.
/// The camera also accepted the same text as UTF8String, i.e. it compares
/// text; a byte-identical name is what keeps a firmware that compares bytes
/// happy as well.
const ISSUER: [(&[u8], &str); 5] = [
    (OID_COUNTRY, "GB"),
    (OID_STATE, "Greater Manchester"),
    (OID_LOCALITY, "Salford"),
    (OID_ORGANIZATION, "COMODO CA Limited"),
    (OID_COMMON_NAME, "COMODO Certification Authority"),
];
/// `2.5.29.17` — subjectAltName.
const OID_SUBJECT_ALT_NAME: &[u8] = &[0x06, 0x03, 0x55, 0x1d, 0x11];
/// `2.5.29.19` — basicConstraints.
const OID_BASIC_CONSTRAINTS: &[u8] = &[0x06, 0x03, 0x55, 0x1d, 0x13];
/// ASN.1 NULL, the parameter every RSA algorithm identifier carries.
const DER_NULL: &[u8] = &[0x05, 0x00];

/// The DigestInfo prefix for SHA-256, as required by EMSA-PKCS1-v1_5.
const SHA256_DIGEST_PREFIX: &[u8] = &[
    0x30, 0x31, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x01, 0x05,
    0x00, 0x04, 0x20,
];

// ---------------------------------------------------------------------------
// Certificate
// ---------------------------------------------------------------------------

/// Build the certificate for the given key, signed with that key.
fn build_certificate(n: &BigUint, e: &BigUint, d: &BigUint) -> Result<Vec<u8>> {
    let algorithm = seq(&[OID_SHA256_WITH_RSA, DER_NULL]);
    let tbs = build_tbs_certificate(n, e, &algorithm)?;

    let digest = Sha256::digest(&tbs);
    let modulus_len = n.bits().div_ceil(8) as usize;
    let signature = sign_pkcs1v15(&digest, modulus_len, n, d)?;

    // A BIT STRING is prefixed by its count of unused trailing bits, which for
    // a whole number of bytes is zero.
    let mut bits = vec![0u8];
    bits.extend_from_slice(&signature);

    Ok(seq(&[&tbs, &algorithm, &tlv(0x03, &bits)]))
}

/// The TBSCertificate: everything the signature covers.
fn build_tbs_certificate(n: &BigUint, e: &BigUint, algorithm: &[u8]) -> Result<Vec<u8>> {
    // [0] EXPLICIT version, v3.
    let version = tlv(0xa0, &tlv(0x02, &[0x02]));

    // A random serial, with the top bit cleared so it stays positive.
    let mut serial = [0u8; 16];
    random_bytes(&mut serial)?;
    serial[0] &= 0x7f;
    let serial = tlv(0x02, &serial);

    let issuer = build_issuer();
    let subject = seq(&[&tlv(0x31, &seq(&[OID_COMMON_NAME, &tlv(0x0c, b"127.0.0.1")]))]);

    let validity = build_validity()?;
    let public_key = build_public_key_info(n, e);
    let extensions = build_extensions();

    Ok(seq(&[&version, &serial, algorithm, &issuer, &validity, &subject, &public_key, &extensions]))
}

/// The issuer Name: one RDN per [`ISSUER`] attribute, each a PrintableString.
fn build_issuer() -> Vec<u8> {
    let rdns: Vec<Vec<u8>> = ISSUER
        .iter()
        .map(|(oid, value)| tlv(0x31, &seq(&[oid, &tlv(0x13, value.as_bytes())])))
        .collect();
    let parts: Vec<&[u8]> = rdns.iter().map(Vec::as_slice).collect();
    seq(&parts)
}

/// SubjectPublicKeyInfo for an RSA key.
fn build_public_key_info(n: &BigUint, e: &BigUint) -> Vec<u8> {
    let rsa_key = seq(&[&der_uint(n), &der_uint(e)]);
    let mut bits = vec![0u8];
    bits.extend_from_slice(&rsa_key);
    seq(&[&seq(&[OID_RSA_ENCRYPTION, DER_NULL]), &tlv(0x03, &bits)])
}

/// The extensions: not a CA, and valid for the loopback host only.
fn build_extensions() -> Vec<u8> {
    // BasicConstraints with cA absent, which DER requires because FALSE is the
    // default and defaults are not encoded. Marked critical, as RFC 5280 says.
    let basic = seq(&[OID_BASIC_CONSTRAINTS, &tlv(0x01, &[0xff]), &tlv(0x04, &seq(&[]))]);

    // GeneralName choices are context tags: [2] dNSName, [7] iPAddress.
    let names = seq(&[
        &tlv(0x82, b"localhost"),
        &tlv(0x87, &[127, 0, 0, 1]),
        &tlv(0x87, &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]),
    ]);
    let alt_name = seq(&[OID_SUBJECT_ALT_NAME, &tlv(0x04, &names)]);

    tlv(0xa3, &seq(&[&basic, &alt_name]))
}

/// Validity, starting an hour ago and running for a year.
///
/// The backdating absorbs a peer whose clock is slow, which a camera's very
/// well may be. The end date is almost decorative: the key is discarded when
/// the process exits, so this certificate cannot outlive the session that made
/// it.
fn build_validity() -> Result<Vec<u8>> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| Error::Tls("the system clock is set before 1970".into()))?
        .as_secs() as i64;
    Ok(seq(&[&der_time(now - 3600), &der_time(now + 365 * 86400)]))
}

/// Encode a Unix timestamp the way RFC 5280 asks: UTCTime through 2049,
/// GeneralizedTime from 2050 on.
fn der_time(unix: i64) -> Vec<u8> {
    let days = unix.div_euclid(86400);
    let seconds = unix.rem_euclid(86400);
    let (year, month, day) = civil_from_days(days);
    let (hour, minute, second) = (seconds / 3600, (seconds / 60) % 60, seconds % 60);

    if (1950..2050).contains(&year) {
        let text = format!(
            "{:02}{month:02}{day:02}{hour:02}{minute:02}{second:02}Z",
            year.rem_euclid(100)
        );
        tlv(0x17, text.as_bytes())
    } else {
        let text = format!("{year:04}{month:02}{day:02}{hour:02}{minute:02}{second:02}Z");
        tlv(0x18, text.as_bytes())
    }
}

/// Turn a count of days since 1970-01-01 into a calendar date.
///
/// Howard Hinnant's `civil_from_days`, which shifts the year to start in March
/// so that the leap day lands at the end and the month lengths fall into a
/// repeating pattern.
fn civil_from_days(days: i64) -> (i64, i64, i64) {
    let shifted = days + 719_468;
    let era = if shifted >= 0 { shifted } else { shifted - 146_096 } / 146_097;
    let day_of_era = shifted - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_position = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_position + 2) / 5 + 1;
    let month = if month_position < 10 { month_position + 3 } else { month_position - 9 };
    (if month <= 2 { year + 1 } else { year }, month, day)
}

/// Sign a digest with RSASSA-PKCS1-v1_5.
fn sign_pkcs1v15(digest: &[u8], modulus_len: usize, n: &BigUint, d: &BigUint) -> Result<Vec<u8>> {
    let mut block = Vec::with_capacity(modulus_len);
    let payload = SHA256_DIGEST_PREFIX.len() + digest.len();
    // 0x00 0x01, then 0xff padding, then 0x00, then the DigestInfo. At least
    // eight padding bytes are required, which a 2048-bit modulus leaves room
    // for many times over.
    let padding = modulus_len
        .checked_sub(payload + 3)
        .filter(|&p| p >= 8)
        .ok_or_else(|| Error::Tls("the key is too small to sign a certificate".into()))?;
    block.push(0x00);
    block.push(0x01);
    block.extend(std::iter::repeat_n(0xffu8, padding));
    block.push(0x00);
    block.extend_from_slice(SHA256_DIGEST_PREFIX);
    block.extend_from_slice(digest);

    let signature = BigUint::from_bytes_be(&block).modpow(d, n).to_bytes_be();
    // The integer representation drops the leading zero the block starts with.
    let mut out = vec![0u8; modulus_len - signature.len()];
    out.extend_from_slice(&signature);
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Generating a key is slow enough that the tests should do it once.
    fn shared() -> &'static SelfSigned {
        static ONCE: std::sync::OnceLock<SelfSigned> = std::sync::OnceLock::new();
        ONCE.get_or_init(|| generate().expect("a key can be generated"))
    }

    #[test]
    fn the_sieve_finds_the_primes_everyone_knows() {
        let primes = small_primes(30);
        assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
    }

    #[test]
    fn primality_agrees_with_the_sieve_below_the_limit() {
        let primes = small_primes(2000);
        for candidate in 2u32..2000 {
            let expected = primes.contains(&candidate);
            let actual = is_probable_prime(&BigUint::from(candidate), &primes);
            assert_eq!(actual, expected, "{candidate} is prime: {expected}");
        }
    }

    #[test]
    fn a_carmichael_number_is_not_mistaken_for_a_prime() {
        // 561, 1105 and 1729 pass a naive Fermat test to many bases; Miller-
        // Rabin is what tells them apart from primes.
        let sieve = small_primes(64);
        for composite in [561u32, 1105, 1729, 2465, 6601, 8911] {
            assert!(!is_probable_prime(&BigUint::from(composite), &sieve), "{composite}");
        }
    }

    #[test]
    fn the_inverse_undoes_the_multiplication() {
        let m = BigUint::from(3_120u32);
        let a = BigUint::from(17u32);
        let inverse = modinv(&a, &m).expect("17 is invertible modulo 3120");
        assert_eq!(&a * &inverse % &m, BigUint::from(1u32));
        // 2 shares a factor with 3120, so it has no inverse.
        assert!(modinv(&BigUint::from(2u32), &m).is_none());
    }

    #[test]
    fn lengths_switch_to_the_long_form_at_128() {
        assert_eq!(der_len(0), vec![0x00]);
        assert_eq!(der_len(127), vec![0x7f]);
        assert_eq!(der_len(128), vec![0x81, 0x80]);
        assert_eq!(der_len(256), vec![0x82, 0x01, 0x00]);
    }

    #[test]
    fn an_integer_with_a_high_top_bit_gains_a_leading_zero() {
        assert_eq!(der_uint(&BigUint::from(0x7fu32)), vec![0x02, 0x01, 0x7f]);
        assert_eq!(der_uint(&BigUint::from(0x80u32)), vec![0x02, 0x02, 0x00, 0x80]);
    }

    #[test]
    fn dates_match_known_days() {
        assert_eq!(civil_from_days(0), (1970, 1, 1));
        assert_eq!(civil_from_days(59), (1970, 3, 1));
        // 2000-02-29, the leap day the hundred-year rule would have skipped.
        assert_eq!(civil_from_days(11_016), (2000, 2, 29));
    }

    #[test]
    fn times_before_2050_use_utctime() {
        // 2026-04-16 12:34:56 UTC.
        let stamp = der_time(1_776_342_896);
        assert_eq!(stamp[0], 0x17, "UTCTime");
        assert_eq!(&stamp[2..], b"260416123456Z");
    }

    #[test]
    fn times_from_2050_use_generalizedtime() {
        // 2050-01-01 00:00:00 UTC.
        let stamp = der_time(2_524_608_000);
        assert_eq!(stamp[0], 0x18, "GeneralizedTime");
        assert_eq!(&stamp[2..], b"20500101000000Z");
    }

    #[test]
    fn the_generated_key_is_the_requested_size() {
        assert_eq!(shared().key.size(), MODULUS_BITS / 8);
    }

    #[test]
    fn the_certificate_is_a_der_sequence_of_its_own_length() {
        let der = &shared().certificate;
        assert_eq!(der.first(), Some(&0x30), "a SEQUENCE");
        // Long form, two length bytes: a 2048-bit certificate is over 256 and
        // under 65536 bytes.
        assert_eq!(der[1], 0x82);
        let declared = ((der[2] as usize) << 8) | der[3] as usize;
        assert_eq!(declared + 4, der.len(), "the declared length matches the buffer");
    }

    /// The point of the whole module: the key in the certificate must be the
    /// key the server decrypts with. If these ever drift apart the handshake
    /// fails with nothing to explain it.
    #[test]
    fn the_certificate_carries_the_key_that_decrypts() {
        let generated = shared();
        // Encrypt a PKCS#1 block under the public key taken from the wire
        // format, then decrypt it with the private key the server holds.
        let (n, e) = public_key_from_certificate(&generated.certificate);
        let mut block = vec![0x00, 0x02];
        block.extend(std::iter::repeat_n(0xabu8, generated.key.size() - 3 - 4));
        block.push(0x00);
        block.extend_from_slice(b"pmca");
        let cipher = BigUint::from_bytes_be(&block).modpow(&e, &n).to_bytes_be();
        let mut padded = vec![0u8; generated.key.size() - cipher.len()];
        padded.extend_from_slice(&cipher);

        let plain = generated.key.decrypt_pkcs1v15(&padded).expect("the block decrypts");
        assert_eq!(plain, b"pmca");
    }

    /// The camera rejects any certificate whose issuer is not a root it knows,
    /// so the issuer must stay byte-for-byte the subject of the COMODO root.
    /// The expected bytes are the issuer field of the real EssentialSSL CA
    /// intermediate (`openssl asn1parse`), not a re-encoding of [`ISSUER`].
    #[test]
    fn the_issuer_is_the_root_the_camera_knows() {
        const COMODO_ROOT_SUBJECT: &str = "308181310b300906035504061302474231\
            1b30190603550408131247726561746572204d616e636865737465723110300e\
            0603550407130753616c666f7264311a3018060355040a1311434f4d4f444f20\
            4341204c696d69746564312730250603550403131e434f4d4f444f2043657274\
            696669636174696f6e20417574686f72697479";
        let expected: Vec<u8> = (0..COMODO_ROOT_SUBJECT.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&COMODO_ROOT_SUBJECT[i..i + 2], 16).expect("hex"))
            .collect();

        let (_, certificate, _) = split(&shared().certificate);
        let (_, tbs, _) = split(certificate);
        // version, serial, signature, then the issuer.
        let mut rest = tbs;
        for _ in 0..3 {
            let (_, _, tail) = split(rest);
            rest = tail;
        }
        let (_, _, after_issuer) = split(rest);
        let issuer = &rest[..rest.len() - after_issuer.len()];
        assert_eq!(issuer, expected.as_slice());
    }

    /// Return the value of the tag-length-value at `data`, and the rest.
    fn split(data: &[u8]) -> (u8, &[u8], &[u8]) {
        let tag = data[0];
        let (len, header) = if data[1] < 0x80 {
            (data[1] as usize, 2)
        } else {
            let count = (data[1] & 0x7f) as usize;
            let len = data[2..2 + count].iter().fold(0usize, |a, &b| (a << 8) | b as usize);
            (len, 2 + count)
        };
        (tag, &data[header..header + len], &data[header + len..])
    }

    /// Walk the certificate far enough to reach the RSA modulus and exponent.
    fn public_key_from_certificate(der: &[u8]) -> (BigUint, BigUint) {
        let (_, certificate, _) = split(der);
        let (_, tbs, _) = split(certificate);
        // version, serial, signature, issuer, validity, subject, then the key.
        let mut rest = tbs;
        for _ in 0..6 {
            let (_, _, tail) = split(rest);
            rest = tail;
        }
        let (_, spki, _) = split(rest);
        let (_, _, after_algorithm) = split(spki);
        let (_, bits, _) = split(after_algorithm);
        // Drop the unused-bits count that prefixes a BIT STRING.
        let (_, rsa_key, _) = split(&bits[1..]);
        let (_, modulus, after_modulus) = split(rsa_key);
        let (_, exponent, _) = split(after_modulus);
        (BigUint::from_bytes_be(modulus), BigUint::from_bytes_be(exponent))
    }
}
