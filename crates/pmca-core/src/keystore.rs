//! Runtime key store for the Sony-derived material this program does not ship.
//!
//! The service-mode authentication keys, the CIC HMAC key and the SPK key
//! material are not part of this source tree. The user supplies them from a
//! source of their own choosing; the program keeps only the **SHA-256 digest**
//! and the length of each key, which reveal nothing about the key itself, and
//! uses them to recognise the right bytes in whatever the user points it at and
//! to verify that nothing else slipped in.
//!
//! Keys imported this way live only in this module's memory for the session and
//! are written to the user's profile by the application layer, never into the
//! program directory and never back out to any network.

use std::sync::{LazyLock, RwLock};

use sha2::{Digest, Sha256};

/// A single key the program needs but does not contain.
///
/// The discriminant doubles as the slot index in the store, so the order must
/// not change once digests are published.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyId {
    /// Service-mode key for the SHA-1 challenge, round 1.
    SenserSha1A,
    /// Service-mode key for the SHA-1 challenge, round 2.
    SenserSha1B,
    /// Service-mode key for the SHA-1 challenge, round 3.
    SenserSha1C,
    /// Service-mode key for the SHA-256 challenge, round 1.
    SenserSha256A,
    /// Service-mode key for the SHA-256 challenge, round 2.
    SenserSha256B,
    /// Service-mode key for the SHA-256 challenge, round 3.
    SenserSha256C,
    /// The CIC HMAC key used to checksum the XPD file.
    Cic,
    /// The RSA-wrapped AES key taken from a store SPK.
    SpkSampleKey,
    /// The SPK RSA public modulus, as its big-endian bytes.
    SpkModulus,
    /// FDAT image key for the CXD4105 / MB8AC102 chips (1st gen).
    FdatCxd4105,
    /// FDAT image key for the CXD4115 chip (1st gen).
    FdatCxd4115,
    /// FDAT image key for the CXD4115 interchangeable-lens chip (1st gen).
    FdatCxd4115Ilc,
    /// FDAT image key for the CXD4120 chip (1st gen).
    FdatCxd4120,
    /// FDAT image key for the CXD4132 chip (2nd gen, also the gen-3 outer key).
    FdatCxd4132,
    /// FDAT image key for the CXD90014 chip (3rd gen inner key).
    FdatCxd90014,
}

/// Number of keys the store tracks.
pub const KEY_COUNT: usize = 15;

/// Every key, in slot order.
pub const ALL: [KeyId; KEY_COUNT] = [
    KeyId::SenserSha1A,
    KeyId::SenserSha1B,
    KeyId::SenserSha1C,
    KeyId::SenserSha256A,
    KeyId::SenserSha256B,
    KeyId::SenserSha256C,
    KeyId::Cic,
    KeyId::SpkSampleKey,
    KeyId::SpkModulus,
    KeyId::FdatCxd4105,
    KeyId::FdatCxd4115,
    KeyId::FdatCxd4115Ilc,
    KeyId::FdatCxd4120,
    KeyId::FdatCxd4132,
    KeyId::FdatCxd90014,
];

/// The class a key belongs to, i.e. the group of features it unlocks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyClass {
    /// Service-mode authentication: service shell, tweaks, function analysis.
    Senser,
    /// The XPD checksum key: app installation and `xpd-checksum`.
    Cic,
    /// The SPK key material: app installation and `apk2spk`/`spk2apk`.
    Spk,
    /// The FDAT image keys: generating the updater-shell firmware payload.
    Fdat,
}

impl KeyClass {
    /// A stable slug the front-ends map to a catalogue key.
    #[must_use]
    pub fn slug(self) -> &'static str {
        match self {
            KeyClass::Senser => "senser",
            KeyClass::Cic => "cic",
            KeyClass::Spk => "spk",
            KeyClass::Fdat => "fdat",
        }
    }

    /// Every class, in a stable order.
    #[must_use]
    pub fn all() -> [KeyClass; 4] {
        [KeyClass::Senser, KeyClass::Cic, KeyClass::Spk, KeyClass::Fdat]
    }

    /// The keys that make up this class.
    #[must_use]
    pub fn keys(self) -> &'static [KeyId] {
        match self {
            KeyClass::Senser => &[
                KeyId::SenserSha1A,
                KeyId::SenserSha1B,
                KeyId::SenserSha1C,
                KeyId::SenserSha256A,
                KeyId::SenserSha256B,
                KeyId::SenserSha256C,
            ],
            KeyClass::Cic => &[KeyId::Cic],
            KeyClass::Spk => &[KeyId::SpkSampleKey, KeyId::SpkModulus],
            KeyClass::Fdat => &[
                KeyId::FdatCxd4105,
                KeyId::FdatCxd4115,
                KeyId::FdatCxd4115Ilc,
                KeyId::FdatCxd4120,
                KeyId::FdatCxd4132,
                KeyId::FdatCxd90014,
            ],
        }
    }
}

impl KeyId {
    fn slot(self) -> usize {
        self as usize
    }

    /// The class this key belongs to.
    #[must_use]
    pub fn class(self) -> KeyClass {
        match self {
            KeyId::SenserSha1A
            | KeyId::SenserSha1B
            | KeyId::SenserSha1C
            | KeyId::SenserSha256A
            | KeyId::SenserSha256B
            | KeyId::SenserSha256C => KeyClass::Senser,
            KeyId::Cic => KeyClass::Cic,
            KeyId::SpkSampleKey | KeyId::SpkModulus => KeyClass::Spk,
            KeyId::FdatCxd4105
            | KeyId::FdatCxd4115
            | KeyId::FdatCxd4115Ilc
            | KeyId::FdatCxd4120
            | KeyId::FdatCxd4132
            | KeyId::FdatCxd90014 => KeyClass::Fdat,
        }
    }

    /// A stable identifier used as the file name in the key store.
    #[must_use]
    pub fn slug(self) -> &'static str {
        match self {
            KeyId::SenserSha1A => "senser-sha1-1",
            KeyId::SenserSha1B => "senser-sha1-2",
            KeyId::SenserSha1C => "senser-sha1-3",
            KeyId::SenserSha256A => "senser-sha256-1",
            KeyId::SenserSha256B => "senser-sha256-2",
            KeyId::SenserSha256C => "senser-sha256-3",
            KeyId::Cic => "cic",
            KeyId::SpkSampleKey => "spk-sample-key",
            KeyId::SpkModulus => "spk-modulus",
            KeyId::FdatCxd4105 => "fdat-cxd4105",
            KeyId::FdatCxd4115 => "fdat-cxd4115",
            KeyId::FdatCxd4115Ilc => "fdat-cxd4115-ilc",
            KeyId::FdatCxd4120 => "fdat-cxd4120",
            KeyId::FdatCxd4132 => "fdat-cxd4132",
            KeyId::FdatCxd90014 => "fdat-cxd90014",
        }
    }

    /// Resolve a slug back to a key id.
    #[must_use]
    pub fn from_slug(slug: &str) -> Option<KeyId> {
        ALL.into_iter().find(|id| id.slug() == slug)
    }

    /// The length in bytes of the real key.
    #[must_use]
    pub fn length(self) -> usize {
        LENGTHS[self.slot()]
    }

    /// The SHA-256 digest of the real key.
    #[must_use]
    pub fn digest(self) -> [u8; 32] {
        DIGESTS[self.slot()]
    }
}

/// Expected byte length of each key, in slot order.
const LENGTHS: [usize; KEY_COUNT] =
    [512, 512, 512, 512, 512, 512, 90, 256, 256, 40, 40, 40, 40, 16, 16];

/// SHA-256 digest of each key, in slot order. The keys themselves are not here.
const DIGESTS: [[u8; 32]; KEY_COUNT] = [
    hx("91a02111b8734848782a5b344c64c48a38ff4e9bbff0f1a1a86f09019e3e9eae"),
    hx("736909cecc41ce0d5079603de3aa400e009666fcc91b7c0062fb2df8e5c741bb"),
    hx("a32e826a50b69f7040722418881fca44a795b4ddb67aadb3dc38458de7057569"),
    hx("8f2320bd115f0515a2a962ae3e68018b1531d74f3675ee0bf602f359a8903f78"),
    hx("255e23be43f0e29abb1efd531ad946c25cba48c1ac8b81e350449e1172adbd82"),
    hx("5ad14fec28814175188194b89aa254363fd7fe08abf6a6393267c8587c106de3"),
    hx("313fcfa7233ddfd0f186b8e2e2f46114551bb74d12fdb232010e88e3b251013e"),
    hx("ab48ea4ed96206dd5b4aa4aeb748cc37fb37b2c89e3bb184d5affe0c5fc795c9"),
    hx("6fa56be73e29ecd004f7325eba8f0e24878340668660aa90aace539693b77e0e"),
    hx("475a2a57ab49ee49da0919565cce6cf7b7960465212ca7a7e2c6928468e716e4"),
    hx("c3063d799aa266a52b8d12dbf9a990dab76cf208ccfbb3cf7793099c63e7bf29"),
    hx("56e028827a68e55951e8a93097f6a47098c019d61c908600727e5f1a9043714b"),
    hx("66584b663f6e677163eb84eaa628df1b497cfe13079a268f46f0c5b25bb24da5"),
    hx("2646036bb22781536be710245c8cbb04b92842c587f845996ce8ab542592a25e"),
    hx("830f3140dac57930eab647fe885fe76a87e807b74af8d3ad3ee90faa0d644f8a"),
];

/// Decode a 64-character hex string to 32 bytes at compile time.
const fn hx(s: &str) -> [u8; 32] {
    let bytes = s.as_bytes();
    let mut out = [0u8; 32];
    let mut i = 0;
    while i < 32 {
        out[i] = nibble(bytes[i * 2]) << 4 | nibble(bytes[i * 2 + 1]);
        i += 1;
    }
    out
}

const fn nibble(c: u8) -> u8 {
    match c {
        b'0'..=b'9' => c - b'0',
        b'a'..=b'f' => c - b'a' + 10,
        _ => 0,
    }
}

/// The keys loaded for this session. A slot is `Some` once its key is present.
static STORE: LazyLock<RwLock<[Option<Vec<u8>>; KEY_COUNT]>> =
    LazyLock::new(|| RwLock::new(Default::default()));

/// Whether a key is loaded.
#[must_use]
pub fn is_present(id: KeyId) -> bool {
    STORE.read().map(|s| s[id.slot()].is_some()).unwrap_or(false)
}

/// A copy of a loaded key, or `None` when it has not been supplied.
#[must_use]
pub fn get(id: KeyId) -> Option<Vec<u8>> {
    STORE.read().ok().and_then(|s| s[id.slot()].clone())
}

/// Whether every key of a class is loaded.
#[must_use]
pub fn class_present(class: KeyClass) -> bool {
    class.keys().iter().all(|&id| is_present(id))
}

/// The classes with at least one key still missing.
#[must_use]
pub fn missing_classes() -> Vec<KeyClass> {
    KeyClass::all().into_iter().filter(|&c| !class_present(c)).collect()
}

/// Whether every key the program needs is loaded.
#[must_use]
pub fn complete() -> bool {
    missing_classes().is_empty()
}

/// Store `bytes` if their digest matches a known key. Returns the key it was.
///
/// A key already present is left as it is and its id is still returned, so a
/// re-import is idempotent.
pub fn install(bytes: &[u8]) -> Option<KeyId> {
    let digest: [u8; 32] = Sha256::digest(bytes).into();
    let id = ALL.into_iter().find(|id| id.length() == bytes.len() && id.digest() == digest)?;
    if let Ok(mut store) = STORE.write()
        && store[id.slot()].is_none()
    {
        store[id.slot()] = Some(bytes.to_vec());
    }
    Some(id)
}

/// Remove every loaded key. For tests and for a user who wants to start over.
pub fn clear() {
    if let Ok(mut store) = STORE.write() {
        *store = Default::default();
    }
}

/// Store `bytes` for `id` **without** checking the digest.
///
/// Only for tests, which cannot carry the real key material: it lets a test
/// stand in a fabricated key so the self-consistent paths (an HMAC compared
/// with itself, an encrypt/decrypt round-trip) can run. Gated behind `test`
/// and the `test-keys` feature so it never reaches a release build.
#[cfg(any(test, feature = "test-keys"))]
pub fn install_unchecked(id: KeyId, bytes: Vec<u8>) {
    if let Ok(mut store) = STORE.write() {
        store[id.slot()] = Some(bytes);
    }
}

/// Install a full set of fabricated keys that are self-consistent, so the
/// self-checking paths (the HMAC, the SPK unwrap/round-trip, the senser
/// handshake) run in tests without any real key material.
///
/// The SPK pair is chosen so the RSA unwrap yields a fixed 16-byte AES key:
/// with modulus `2^128` and wrapped value `2^128 - 1` (which is `-1` mod the
/// modulus), `(-1)^65537 = -1`, i.e. `0xFF * 16`.
#[cfg(any(test, feature = "test-keys"))]
pub fn install_test_keys() {
    let senser = [
        KeyId::SenserSha1A,
        KeyId::SenserSha1B,
        KeyId::SenserSha1C,
        KeyId::SenserSha256A,
        KeyId::SenserSha256B,
        KeyId::SenserSha256C,
    ];
    for (i, id) in senser.into_iter().enumerate() {
        let seed = u8::try_from(i).unwrap_or(0) + 1;
        install_unchecked(id, (0..512).map(|j| (j as u8).wrapping_add(seed)).collect());
    }
    install_unchecked(KeyId::Cic, vec![0x42; 90]);
    install_unchecked(KeyId::SpkSampleKey, vec![0xff; 16]);
    let mut modulus = vec![0x01];
    modulus.extend(std::iter::repeat_n(0u8, 16));
    install_unchecked(KeyId::SpkModulus, modulus);
    for id in [KeyId::FdatCxd4105, KeyId::FdatCxd4115, KeyId::FdatCxd4115Ilc, KeyId::FdatCxd4120] {
        install_unchecked(id, vec![0x55; 40]);
    }
    install_unchecked(KeyId::FdatCxd4132, vec![0x66; 16]);
    install_unchecked(KeyId::FdatCxd90014, vec![0x77; 16]);
}

/// Serialises tests that touch the shared store, since they run in parallel
/// threads of one process. Poisoning is ignored: a panicking test still
/// releases the lock for the next.
#[cfg(any(test, feature = "test-keys"))]
pub fn test_guard() -> std::sync::MutexGuard<'static, ()> {
    static GUARD: std::sync::Mutex<()> = std::sync::Mutex::new(());
    GUARD.lock().unwrap_or_else(|e| e.into_inner())
}

/// Extract candidate byte sequences from `data` and install every one whose
/// digest matches a key. Returns the keys found this call.
///
/// `data` may be source code (the keys as `b'...'` literals or a decimal
/// integer), a text dump, or a binary file; all three are covered.
pub fn scan(data: &[u8]) -> Vec<KeyId> {
    let mut found = Vec::new();
    for candidate in candidates(data) {
        if let Some(id) = install(&candidate)
            && !found.contains(&id)
        {
            found.push(id);
        }
    }
    found
}

/// Candidate byte sequences the scanner tries, in rough order of likelihood.
fn candidates(data: &[u8]) -> Vec<Vec<u8>> {
    let mut out = Vec::new();
    if let Ok(text) = std::str::from_utf8(data) {
        out.extend(byte_string_literals(text));
        out.extend(decimal_integers(text));
        out.extend(hex_byte_arrays(text));
    }
    out.extend(raw_windows(data));
    out
}

/// Decode every `b'...'` / `b"..."` byte-string literal (Python and Rust) in
/// `text`, handling `\xNN`, the common escapes and a literal backslash.
fn byte_string_literals(text: &str) -> Vec<Vec<u8>> {
    let bytes = text.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i + 1 < bytes.len() {
        if bytes[i] == b'b' && (bytes[i + 1] == b'\'' || bytes[i + 1] == b'"') {
            let quote = bytes[i + 1];
            if let Some((decoded, end)) = decode_literal(bytes, i + 2, quote) {
                out.push(decoded);
                i = end;
                continue;
            }
        }
        i += 1;
    }
    out
}

/// Decode one byte-string body starting at `start`, up to the closing `quote`.
fn decode_literal(bytes: &[u8], start: usize, quote: u8) -> Option<(Vec<u8>, usize)> {
    let mut out = Vec::new();
    let mut i = start;
    while i < bytes.len() {
        match bytes[i] {
            b'\\' => {
                let next = *bytes.get(i + 1)?;
                match next {
                    b'x' => {
                        let hi = hex_val(*bytes.get(i + 2)?)?;
                        let lo = hex_val(*bytes.get(i + 3)?)?;
                        out.push(hi << 4 | lo);
                        i += 4;
                    }
                    b'n' => {
                        out.push(b'\n');
                        i += 2;
                    }
                    b'r' => {
                        out.push(b'\r');
                        i += 2;
                    }
                    b't' => {
                        out.push(b'\t');
                        i += 2;
                    }
                    b'0' => {
                        out.push(0);
                        i += 2;
                    }
                    b'\\' | b'\'' | b'"' => {
                        out.push(next);
                        i += 2;
                    }
                    // A line continuation inside a long literal.
                    b'\n' => i += 2,
                    _ => return None,
                }
            }
            c if c == quote => return Some((out, i + 1)),
            // A bare newline ends a single-quoted literal in both languages.
            b'\n' => return None,
            c => {
                out.push(c);
                i += 1;
            }
        }
    }
    None
}

/// Every run of 100+ decimal digits, as its big-endian bytes (the RSA modulus).
fn decimal_integers(text: &str) -> Vec<Vec<u8>> {
    let mut out = Vec::new();
    let bytes = text.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i].is_ascii_digit() {
            let start = i;
            while i < bytes.len() && bytes[i].is_ascii_digit() {
                i += 1;
            }
            // A digit run touching a letter is part of an identifier, not a number.
            let touches_letter = (start > 0 && (bytes[start - 1] as char).is_alphabetic())
                || bytes.get(i).is_some_and(|&c| (c as char).is_alphabetic());
            if !touches_letter && i - start >= 100 {
                out.push(decimal_to_bytes(&text[start..i]));
            }
        } else {
            i += 1;
        }
    }
    out
}

/// Every array of `0xNN` bytes, as C and Rust sources write key tables.
///
/// A run is broken by anything but commas and whitespace, so each inner array
/// of a nested table comes out on its own. Longer runs also yield windows of
/// the expected key lengths.
fn hex_byte_arrays(text: &str) -> Vec<Vec<u8>> {
    let bytes = text.as_bytes();
    let mut out = Vec::new();
    let mut run: Vec<u8> = Vec::new();
    let mut flush = |run: &mut Vec<u8>| {
        if run.len() >= 16 {
            for &len in LENGTHS.iter() {
                if run.len() > len {
                    out.extend(run.windows(len).map(<[u8]>::to_vec));
                }
            }
            out.push(std::mem::take(run));
        }
        run.clear();
    };
    let mut i = 0;
    while i < bytes.len() {
        let token = bytes.get(i..i + 4).filter(|t| t[0] == b'0' && (t[1] | 0x20) == b'x');
        let hex = token.and_then(|t| Some(hex_val(t[2])? << 4 | hex_val(t[3])?));
        let ends = bytes.get(i + 4).is_none_or(|c| !c.is_ascii_hexdigit());
        match hex {
            Some(value) if ends && (i == 0 || !bytes[i - 1].is_ascii_alphanumeric()) => {
                run.push(value);
                i += 4;
            }
            _ => {
                if !(bytes[i] == b',' || bytes[i].is_ascii_whitespace()) {
                    flush(&mut run);
                }
                i += 1;
            }
        }
    }
    flush(&mut run);
    out
}

/// Convert a decimal string to its minimal big-endian byte representation.
fn decimal_to_bytes(digits: &str) -> Vec<u8> {
    let mut bytes: Vec<u8> = vec![0];
    for d in digits.bytes().map(|b| b - b'0') {
        let mut carry = u16::from(d);
        for byte in bytes.iter_mut().rev() {
            let v = u16::from(*byte) * 10 + carry;
            *byte = (v & 0xff) as u8;
            carry = v >> 8;
        }
        while carry > 0 {
            bytes.insert(0, (carry & 0xff) as u8);
            carry >>= 8;
        }
    }
    while bytes.len() > 1 && bytes[0] == 0 {
        bytes.remove(0);
    }
    bytes
}

/// Sliding windows of each expected key length over the raw bytes.
///
/// The fallback for a binary key file. Capped so a large archive does not turn
/// into a quadratic scan.
fn raw_windows(data: &[u8]) -> Vec<Vec<u8>> {
    /// Beyond this, the literal and integer paths are relied on instead.
    const MAX_RAW_SCAN: usize = 4 * 1024 * 1024;
    if data.len() > MAX_RAW_SCAN {
        return Vec::new();
    }
    let mut lengths: Vec<usize> = LENGTHS.to_vec();
    lengths.sort_unstable();
    lengths.dedup();
    let mut out = Vec::new();
    for len in lengths {
        if data.len() >= len {
            for window in data.windows(len) {
                out.push(window.to_vec());
            }
        }
    }
    out
}

fn hex_val(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'a'..=b'f' => Some(c - b'a' + 10),
        b'A'..=b'F' => Some(c - b'A' + 10),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // A fabricated key of the right length, so the tests carry no real material.
    fn fake(id: KeyId) -> Vec<u8> {
        (0..id.length()).map(|i| (i as u8).wrapping_mul(31).wrapping_add(7)).collect()
    }

    // Overriding the digest table is not possible from a test, so these tests
    // verify the plumbing with the real lengths and fabricated bytes whose
    // digest will not match; `install` therefore returns `None`, which is the
    // correct behaviour for wrong bytes and is what we assert.

    #[test]
    fn wrong_bytes_are_refused() {
        let _g = test_guard();
        clear();
        assert_eq!(install(&fake(KeyId::Cic)), None);
        assert!(!is_present(KeyId::Cic));
    }

    #[test]
    fn a_python_literal_is_decoded() {
        let text = "cicKey = b'0011223344556677889900aabbccddeeff'";
        // 32 ASCII bytes here, not any real key, so no install; but the decoder
        // must have produced exactly those 32 bytes as a candidate.
        let cands = byte_string_literals(text);
        assert_eq!(cands.len(), 1);
        assert_eq!(cands[0], b"0011223344556677889900aabbccddeeff");
    }

    #[test]
    fn an_escaped_literal_is_decoded() {
        let cands = byte_string_literals(r"k = b'\x7e\x29\x35\x14'");
        assert_eq!(cands, vec![vec![0x7e, 0x29, 0x35, 0x14]]);
    }

    #[test]
    fn a_long_decimal_becomes_big_endian_bytes() {
        assert_eq!(decimal_to_bytes("255"), vec![0xff]);
        assert_eq!(decimal_to_bytes("256"), vec![0x01, 0x00]);
        assert_eq!(decimal_to_bytes("65535"), vec![0xff, 0xff]);
    }

    #[test]
    fn a_short_decimal_is_not_a_candidate() {
        assert!(decimal_integers("version = 65537").is_empty());
    }

    #[test]
    fn nested_hex_arrays_come_out_one_per_inner_array() {
        let a: Vec<String> = (0u8..16).map(|b| format!("0x{b:02x}")).collect();
        let b: Vec<String> = (16u8..32).map(|b| format!("0x{b:02X}")).collect();
        let text = format!(
            "static K: [[u8; 16]; 2] = [\n  [{}],\n  [{}],\n];",
            a.join(", "),
            b.join(",\n ")
        );
        let found = hex_byte_arrays(&text);
        assert!(found.contains(&(0u8..16).collect::<Vec<_>>()));
        assert!(found.contains(&(16u8..32).collect::<Vec<_>>()));
        // A size constant and hex inside an identifier are not key bytes.
        assert!(hex_byte_arrays("const SIZE: usize = 0x200; let a0x41 = 1;").is_empty());
    }

    #[test]
    fn classes_cover_every_key() {
        let mut seen: Vec<KeyId> =
            KeyClass::all().into_iter().flat_map(|c| c.keys().to_vec()).collect();
        seen.sort_by_key(|id| id.slot());
        assert_eq!(seen, ALL.to_vec());
    }

    #[test]
    fn slugs_round_trip() {
        for id in ALL {
            assert_eq!(KeyId::from_slug(id.slug()), Some(id));
        }
    }
}
