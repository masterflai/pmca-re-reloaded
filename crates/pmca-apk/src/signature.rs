//! Checks an APK's v1 (JAR) signature the way Android does before installing.
//!
//! Every file outside `META-INF/` has to be listed in `META-INF/MANIFEST.MF`
//! with a matching digest, the signature file (`*.SF`) has to match the
//! manifest, and the PKCS#7 block (`*.RSA`) has to hold an RSA signature over
//! the signature file made with the certificate it names. Only then does the
//! certificate say who built the archive; a file copied out of someone else's
//! APK proves nothing on its own.

use std::collections::BTreeMap;
use std::io::Read;

use num_bigint::BigUint;
use sha2::Digest;

use crate::der::{Element, element, expect};
use crate::{MAX_SIGNATURE_SIZE, Result, is_signature_block};

/// What an APK's v1 signature says.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Signature {
    /// The archive carries no signature block.
    Unsigned,
    /// The signature holds for every file; the signer's certificate (DER).
    Valid(Vec<u8>),
    /// Signed in a way this check does not cover (DSA, EC, an unknown digest);
    /// the certificate the block names (DER), not verified.
    Unsupported(Vec<u8>),
    /// The archive does not match its signature.
    Invalid(&'static str),
}

/// Upper bound on `MANIFEST.MF` and the signature file.
const MAX_MANIFEST_SIZE: u64 = 16 * 1024 * 1024;

/// Upper bound on all files hashed together, guarding against a zip bomb.
const MAX_CONTENT_SIZE: u64 = 1024 * 1024 * 1024;

/// Larger RSA keys are refused rather than spending minutes on them.
const MAX_RSA_BITS: u64 = 8192;

/// `1.2.840.113549.1.7.2`, PKCS#7 signed data.
const SIGNED_DATA: &[u8] = &[0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x07, 0x02];
/// `1.2.840.113549.1.1`, the PKCS#1 family (`rsaEncryption`, `sha256WithRSAEncryption`, …).
const PKCS1: &[u8] = &[0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x01];
/// `1.2.840.113549.1.1.1`, `rsaEncryption`.
const RSA_ENCRYPTION: &[u8] = &[0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x01, 0x01];
/// `1.2.840.113549.1.9.4`, the `messageDigest` signed attribute.
const MESSAGE_DIGEST: &[u8] = &[0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x09, 0x04];

/// The digests JAR signing uses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    Sha1,
    Sha256,
    Sha384,
    Sha512,
}

impl Algorithm {
    /// From the prefix of a manifest attribute such as `SHA-256-Digest`.
    fn from_manifest_name(name: &str) -> Option<Self> {
        match name.to_ascii_uppercase().as_str() {
            "SHA1" | "SHA-1" => Some(Self::Sha1),
            "SHA-256" => Some(Self::Sha256),
            "SHA-384" => Some(Self::Sha384),
            "SHA-512" => Some(Self::Sha512),
            _ => None,
        }
    }

    fn oid(self) -> &'static [u8] {
        match self {
            Self::Sha1 => &[0x2b, 0x0e, 0x03, 0x02, 0x1a],
            Self::Sha256 => &[0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x01],
            Self::Sha384 => &[0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x02],
            Self::Sha512 => &[0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x03],
        }
    }

    fn from_oid(oid: &[u8]) -> Option<Self> {
        [Self::Sha1, Self::Sha256, Self::Sha384, Self::Sha512].into_iter().find(|a| a.oid() == oid)
    }

    fn hasher(self) -> Hasher {
        match self {
            Self::Sha1 => Hasher::Sha1(sha1::Sha1::new()),
            Self::Sha256 => Hasher::Sha256(sha2::Sha256::new()),
            Self::Sha384 => Hasher::Sha384(sha2::Sha384::new()),
            Self::Sha512 => Hasher::Sha512(sha2::Sha512::new()),
        }
    }

    fn digest(self, data: &[u8]) -> Vec<u8> {
        let mut hasher = self.hasher();
        hasher.update(data);
        hasher.finish()
    }
}

enum Hasher {
    Sha1(sha1::Sha1),
    Sha256(sha2::Sha256),
    Sha384(sha2::Sha384),
    Sha512(sha2::Sha512),
}

impl Hasher {
    fn update(&mut self, data: &[u8]) {
        match self {
            Self::Sha1(h) => h.update(data),
            Self::Sha256(h) => h.update(data),
            Self::Sha384(h) => h.update(data),
            Self::Sha512(h) => h.update(data),
        }
    }

    fn finish(self) -> Vec<u8> {
        match self {
            Self::Sha1(h) => h.finalize().to_vec(),
            Self::Sha256(h) => h.finalize().to_vec(),
            Self::Sha384(h) => h.finalize().to_vec(),
            Self::Sha512(h) => h.finalize().to_vec(),
        }
    }
}

/// Check the v1 signature of a complete APK.
///
/// Signed by several signers, the first signature block by name counts, as
/// for [`crate::read`]'s `signed` flag; an archive does not get to pick by its
/// order which one that is.
pub fn verify_signature(data: &[u8]) -> Result<Signature> {
    let mut archive = zip::ZipArchive::new(std::io::Cursor::new(data))?;
    let names: Vec<String> =
        archive.file_names().filter_map(|n| n.ok().map(|n| n.into_owned())).collect();
    let mut blocks: Vec<&String> = names.iter().filter(|n| is_signature_block(n)).collect();
    blocks.sort();
    let Some(block_name) = blocks.first() else { return Ok(Signature::Unsigned) };

    let Some(block) = read_entry(&mut archive, block_name, MAX_SIGNATURE_SIZE)? else {
        return Ok(Signature::Invalid("signature block unreadable"));
    };
    let Some(signed) = signed_data(&block) else {
        return Ok(Signature::Invalid("signature block unreadable"));
    };
    let Some(certificate) = signed.certificates.iter().copied().find(|c| signed.signer.names(c))
    else {
        return Ok(Signature::Invalid("signer certificate missing"));
    };
    let algorithm = Algorithm::from_oid(signed.signer.digest);
    let key = rsa_key(certificate);
    let (Some(algorithm), Some((modulus, exponent)), true) =
        (algorithm, key, signed.signer.encryption.starts_with(PKCS1))
    else {
        return Ok(Signature::Unsupported(certificate.to_vec()));
    };

    // `META-INF/CERT.RSA` signs `META-INF/CERT.SF`.
    let stem = &block_name[..block_name.rfind('.').unwrap_or(block_name.len())];
    let Some(signature_file) = read_entry(&mut archive, &format!("{stem}.SF"), MAX_MANIFEST_SIZE)?
    else {
        return Ok(Signature::Invalid("signature file missing"));
    };
    let Some(signed_bytes) = signed.signer.signed_bytes(algorithm, &signature_file) else {
        return Ok(Signature::Invalid("signed attributes do not match the signature file"));
    };
    if !rsa_verifies(
        modulus,
        exponent,
        signed.signer.signature,
        algorithm,
        &algorithm.digest(&signed_bytes),
    ) {
        return Ok(Signature::Invalid("signature does not verify"));
    }

    let Some(manifest) = read_entry(&mut archive, "META-INF/MANIFEST.MF", MAX_MANIFEST_SIZE)?
    else {
        return Ok(Signature::Invalid("manifest missing"));
    };
    if !signature_file_matches(&signature_file, &manifest) {
        return Ok(Signature::Invalid("manifest does not match the signature file"));
    }
    if let Err(reason) = files_match(&mut archive, &names, &manifest)? {
        return Ok(Signature::Invalid(reason));
    }
    Ok(Signature::Valid(certificate.to_vec()))
}

/// An entry's bytes, at most `limit` of them; `None` when the archive lacks it.
fn read_entry<R: Read + std::io::Seek>(
    archive: &mut zip::ZipArchive<R>,
    name: &str,
    limit: u64,
) -> Result<Option<Vec<u8>>> {
    let entry = match archive.by_name(name) {
        Ok(entry) => entry,
        Err(zip::result::ZipError::FileNotFound) => return Ok(None),
        Err(e) => return Err(e.into()),
    };
    let mut data = Vec::new();
    entry.take(limit).read_to_end(&mut data)?;
    Ok(Some(data))
}

/// Every file outside `META-INF/` listed with matching digests, and every
/// listed file present.
fn files_match<R: Read + std::io::Seek>(
    archive: &mut zip::ZipArchive<R>,
    names: &[String],
    manifest: &[u8],
) -> Result<std::result::Result<(), &'static str>> {
    let mut entries: BTreeMap<String, Vec<(Algorithm, Vec<u8>)>> = BTreeMap::new();
    for section in sections(manifest).iter().skip(1) {
        let Some(name) = section.attribute("Name") else { continue };
        if entries.insert(name.to_string(), section.digests("-Digest")).is_some() {
            return Ok(Err("manifest names a file twice"));
        }
    }
    let exempt = |name: &String| name.ends_with('/') || name.starts_with("META-INF/");
    if names.iter().any(|name| !exempt(name) && !entries.contains_key(name)) {
        return Ok(Err("a file is not in the manifest"));
    }

    let mut budget = MAX_CONTENT_SIZE;
    let mut buffer = vec![0; 64 * 1024];
    for (name, digests) in &entries {
        if digests.is_empty() {
            return Ok(Err("a manifest entry has no known digest"));
        }
        let mut entry = match archive.by_name(name) {
            Ok(entry) => entry,
            Err(zip::result::ZipError::FileNotFound) => {
                return Ok(Err("the manifest names a missing file"));
            }
            Err(e) => return Err(e.into()),
        };
        let mut hashers: Vec<Hasher> = digests.iter().map(|(a, _)| a.hasher()).collect();
        loop {
            let read = entry.read(&mut buffer)?;
            if read == 0 {
                break;
            }
            budget = match budget.checked_sub(read as u64) {
                Some(left) => left,
                None => return Ok(Err("archive too large to check")),
            };
            hashers.iter_mut().for_each(|h| h.update(&buffer[..read]));
        }
        if hashers.into_iter().zip(digests).any(|(h, (_, expected))| h.finish() != *expected) {
            return Ok(Err("a file does not match the manifest"));
        }
    }
    Ok(Ok(()))
}

/// Whether the signature file vouches for `manifest`: as a whole, or section
/// by section for every entry, as Android falls back to.
fn signature_file_matches(signature_file: &[u8], manifest: &[u8]) -> bool {
    let signature_sections = sections(signature_file);
    let Some(main) = signature_sections.first() else { return false };
    let whole = main.digests("-Digest-Manifest");
    if !whole.is_empty() && whole.iter().all(|(a, d)| a.digest(manifest) == *d) {
        return true;
    }

    let manifest_sections = sections(manifest);
    let raw: BTreeMap<&str, &[u8]> = manifest_sections
        .iter()
        .skip(1)
        .filter_map(|s| s.attribute("Name").map(|name| (name, s.raw)))
        .collect();
    let mut vouched = 0;
    for section in signature_sections.iter().skip(1) {
        let digests = section.digests("-Digest");
        let Some(raw) = section.attribute("Name").and_then(|name| raw.get(name)) else {
            return false;
        };
        if digests.is_empty() || digests.iter().any(|(a, d)| a.digest(raw) != *d) {
            return false;
        }
        vouched += 1;
    }
    vouched == raw.len()
}

/// One section of a manifest or signature file.
struct Section<'a> {
    /// Its bytes, the blank line that ends it included, as they are digested.
    raw: &'a [u8],
    attributes: Vec<(String, String)>,
}

impl Section<'_> {
    fn attribute(&self, name: &str) -> Option<&str> {
        self.attributes.iter().find(|(k, _)| k.eq_ignore_ascii_case(name)).map(|(_, v)| v.as_str())
    }

    /// The digests in attributes named `<algorithm><suffix>`; unknown
    /// algorithms are skipped, undecodable values kept so they fail to match.
    fn digests(&self, suffix: &str) -> Vec<(Algorithm, Vec<u8>)> {
        self.attributes
            .iter()
            .filter_map(|(key, value)| {
                let cut = key.len().checked_sub(suffix.len())?;
                if !key.is_char_boundary(cut) || !key[cut..].eq_ignore_ascii_case(suffix) {
                    return None;
                }
                let algorithm = Algorithm::from_manifest_name(&key[..cut])?;
                Some((algorithm, base64(value.trim()).unwrap_or_default()))
            })
            .collect()
    }
}

/// Split a manifest into sections at blank lines; continuation lines start
/// with one space.
fn sections(text: &[u8]) -> Vec<Section<'_>> {
    let mut found = Vec::new();
    let mut start = 0;
    let mut lines: Vec<&[u8]> = Vec::new();
    let mut pos = 0;
    while pos < text.len() {
        let end = text[pos..].iter().position(|&b| b == b'\r' || b == b'\n').map(|i| pos + i);
        let (line, next) = match end {
            Some(i) if text[i] == b'\r' && text.get(i + 1) == Some(&b'\n') => {
                (&text[pos..i], i + 2)
            }
            Some(i) => (&text[pos..i], i + 1),
            None => (&text[pos..], text.len()),
        };
        pos = next;
        if line.is_empty() {
            if !lines.is_empty() {
                found.push(Section { raw: &text[start..pos], attributes: attributes(&lines) });
                lines.clear();
            }
            start = pos;
        } else {
            lines.push(line);
        }
    }
    if !lines.is_empty() {
        found.push(Section { raw: &text[start..], attributes: attributes(&lines) });
    }
    found
}

fn attributes(lines: &[&[u8]]) -> Vec<(String, String)> {
    // Joined as bytes first: a continuation may split a UTF-8 character.
    let mut raw: Vec<(Vec<u8>, Vec<u8>)> = Vec::new();
    for line in lines {
        if let Some(rest) = line.strip_prefix(b" ") {
            if let Some((_, value)) = raw.last_mut() {
                value.extend_from_slice(rest);
            }
        } else if let Some(i) = line.windows(2).position(|w| w == b": ") {
            raw.push((line[..i].to_vec(), line[i + 2..].to_vec()));
        }
    }
    raw.into_iter()
        .map(|(k, v)| {
            (String::from_utf8_lossy(&k).into_owned(), String::from_utf8_lossy(&v).into_owned())
        })
        .collect()
}

/// Standard base64 with padding.
fn base64(text: &str) -> Option<Vec<u8>> {
    let value = |c: u8| match c {
        b'A'..=b'Z' => Some(c - b'A'),
        b'a'..=b'z' => Some(c - b'a' + 26),
        b'0'..=b'9' => Some(c - b'0' + 52),
        b'+' => Some(62),
        b'/' => Some(63),
        _ => None,
    };
    let bytes = text.as_bytes();
    if !bytes.len().is_multiple_of(4) {
        return None;
    }
    let mut out = Vec::with_capacity(bytes.len() / 4 * 3);
    for (index, chunk) in bytes.chunks(4).enumerate() {
        let last = index == bytes.len() / 4 - 1;
        let padding = chunk.iter().rev().take_while(|&&c| c == b'=').count();
        if padding > 2 || (padding > 0 && !last) {
            return None;
        }
        let mut word = 0u32;
        for &c in &chunk[..4 - padding] {
            word = (word << 6) | u32::from(value(c)?);
        }
        word <<= 6 * padding;
        out.extend_from_slice(&word.to_be_bytes()[1..4 - padding]);
    }
    Some(out)
}

/// The parts of a PKCS#7 signature block the check needs.
struct SignedData<'a> {
    /// Whole DER of each certificate.
    certificates: Vec<&'a [u8]>,
    signer: SignerInfo<'a>,
}

struct SignerInfo<'a> {
    /// Whole DER of the issuer name and serial number of the signer's certificate.
    issuer: &'a [u8],
    serial: &'a [u8],
    /// OID contents.
    digest: &'a [u8],
    encryption: &'a [u8],
    /// Whole DER of the `[0]` signed attributes, if any.
    attributes: Option<&'a [u8]>,
    signature: &'a [u8],
}

impl SignerInfo<'_> {
    /// Whether `certificate` is the one this signer names.
    fn names(&self, certificate: &[u8]) -> bool {
        certificate_parts(certificate).is_some_and(|(serial, issuer, _)| {
            serial.whole == self.serial && issuer.whole == self.issuer
        })
    }

    /// What the signature covers: the signature file, or the signed attributes
    /// once their `messageDigest` matches it.
    fn signed_bytes(&self, algorithm: Algorithm, signature_file: &[u8]) -> Option<Vec<u8>> {
        let Some(attributes) = self.attributes else { return Some(signature_file.to_vec()) };
        let (mut list, _) = expect(attributes, 0xa0)?;
        let mut digest = None;
        while !list.is_empty() {
            let attribute = element(list)?;
            list = attribute.rest;
            let (oid, values) = expect(attribute.content, 0x06)?;
            if oid == MESSAGE_DIGEST {
                let (set, _) = expect(values, 0x31)?;
                digest = Some(expect(set, 0x04)?.0);
            }
        }
        if digest? != algorithm.digest(signature_file).as_slice() {
            return None;
        }
        // Signed as a SET, not under the implicit [0] tag it is stored with.
        let mut signed = attributes.to_vec();
        signed[0] = 0x31;
        Some(signed)
    }
}

/// `ContentInfo { signedData, [0] SignedData { version, digestAlgorithms,
/// contentInfo, [0] certificates, [1] crls, signerInfos } }`.
fn signed_data(block: &[u8]) -> Option<SignedData<'_>> {
    let (info, _) = expect(block, 0x30)?;
    let (oid, info) = expect(info, 0x06)?;
    if oid != SIGNED_DATA {
        return None;
    }
    let (wrapped, _) = expect(info, 0xa0)?;
    let (signed, _) = expect(wrapped, 0x30)?;
    let (_, rest) = expect(signed, 0x02)?;
    let (_, rest) = expect(rest, 0x31)?;
    let (_, rest) = expect(rest, 0x30)?;

    let mut next = element(rest)?;
    let mut certificates = Vec::new();
    if next.tag == 0xa0 {
        let mut list = next.content;
        while !list.is_empty() {
            let certificate = element(list)?;
            if certificate.tag == 0x30 {
                certificates.push(certificate.whole);
            }
            list = certificate.rest;
        }
        next = element(next.rest)?;
    }
    if next.tag == 0xa1 {
        next = element(next.rest)?;
    }
    if next.tag != 0x31 {
        return None;
    }
    let (first, _) = expect(next.content, 0x30)?;
    Some(SignedData { certificates, signer: signer_info(first)? })
}

/// `SignerInfo { version, issuerAndSerialNumber, digestAlgorithm,
/// [0] authenticatedAttributes, digestEncryptionAlgorithm, encryptedDigest, … }`.
fn signer_info(content: &[u8]) -> Option<SignerInfo<'_>> {
    let (_, rest) = expect(content, 0x02)?;
    let (names, rest) = expect(rest, 0x30)?;
    let issuer = element(names)?;
    let serial = element(issuer.rest)?;
    if issuer.tag != 0x30 || serial.tag != 0x02 {
        return None;
    }
    let (algorithm, rest) = expect(rest, 0x30)?;
    let (digest, _) = expect(algorithm, 0x06)?;
    let mut next = element(rest)?;
    let attributes = if next.tag == 0xa0 {
        let whole = next.whole;
        next = element(next.rest)?;
        Some(whole)
    } else {
        None
    };
    if next.tag != 0x30 {
        return None;
    }
    let (encryption, _) = expect(next.content, 0x06)?;
    let (signature, _) = expect(next.rest, 0x04)?;
    Some(SignerInfo {
        issuer: issuer.whole,
        serial: serial.whole,
        digest,
        encryption,
        attributes,
        signature,
    })
}

/// Serial number, issuer and the content of the public key info of a certificate.
fn certificate_parts(certificate: &[u8]) -> Option<(Element<'_>, Element<'_>, &[u8])> {
    let (outer, _) = expect(certificate, 0x30)?;
    let (tbs, _) = expect(outer, 0x30)?;
    let mut serial = element(tbs)?;
    if serial.tag == 0xa0 {
        serial = element(serial.rest)?;
    }
    let algorithm = element(serial.rest)?;
    let issuer = element(algorithm.rest)?;
    let validity = element(issuer.rest)?;
    let subject = element(validity.rest)?;
    let (key, _) = expect(subject.rest, 0x30)?;
    (serial.tag == 0x02 && issuer.tag == 0x30).then_some((serial, issuer, key))
}

/// Modulus and public exponent of an RSA certificate.
fn rsa_key(certificate: &[u8]) -> Option<(&[u8], &[u8])> {
    let (_, _, key) = certificate_parts(certificate)?;
    let (algorithm, rest) = expect(key, 0x30)?;
    if expect(algorithm, 0x06)?.0 != RSA_ENCRYPTION {
        return None;
    }
    let (bits, _) = expect(rest, 0x03)?;
    let (&0, key) = bits.split_first()? else { return None };
    let (key, _) = expect(key, 0x30)?;
    let (modulus, rest) = expect(key, 0x02)?;
    let (exponent, _) = expect(rest, 0x02)?;
    Some((modulus, exponent))
}

/// RSASSA-PKCS1-v1_5: `signature ^ exponent mod modulus` has to be
/// `00 01 FF… 00 DigestInfo(algorithm, digest)`.
fn rsa_verifies(
    modulus: &[u8],
    exponent: &[u8],
    signature: &[u8],
    algorithm: Algorithm,
    digest: &[u8],
) -> bool {
    let n = BigUint::from_bytes_be(modulus);
    let e = BigUint::from_bytes_be(exponent);
    let bits = n.bits();
    let size = bits.div_ceil(8) as usize;
    if !(512..=MAX_RSA_BITS).contains(&bits) || e.bits() > bits || signature.len() != size {
        return false;
    }
    let s = BigUint::from_bytes_be(signature);
    if s >= n {
        return false;
    }
    let m = s.modpow(&e, &n).to_bytes_be();
    let mut encoded = vec![0; size - m.len()];
    encoded.extend(m);

    let Some(rest) = encoded.strip_prefix(&[0x00, 0x01]) else { return false };
    let padding = rest.iter().take_while(|&&b| b == 0xff).count();
    let Some(info) = rest[padding..].strip_prefix(&[0x00]) else { return false };
    padding >= 8 && digest_info_matches(info, algorithm, digest)
}

/// `DigestInfo { AlgorithmIdentifier { oid, NULL? }, OCTET STRING }`, exactly.
fn digest_info_matches(info: &[u8], algorithm: Algorithm, digest: &[u8]) -> bool {
    let check = || -> Option<bool> {
        let (content, tail) = expect(info, 0x30)?;
        let (identifier, rest) = expect(content, 0x30)?;
        let (oid, parameters) = expect(identifier, 0x06)?;
        let (value, end) = expect(rest, 0x04)?;
        Some(
            tail.is_empty()
                && end.is_empty()
                && (parameters.is_empty() || parameters == [0x05, 0x00])
                && oid == algorithm.oid()
                && value == digest,
        )
    };
    check().unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use super::*;

    /// Wrap `content` in a DER element with `tag`.
    fn tlv(tag: u8, content: &[u8]) -> Vec<u8> {
        let mut out = vec![tag];
        let len = content.len();
        if len < 0x80 {
            out.push(len as u8);
        } else if len < 0x100 {
            out.extend([0x81, len as u8]);
        } else {
            out.extend([0x82, (len >> 8) as u8, len as u8]);
        }
        out.extend_from_slice(content);
        out
    }

    fn integer(value: &BigUint) -> Vec<u8> {
        let mut bytes = value.to_bytes_be();
        if bytes[0] & 0x80 != 0 {
            bytes.insert(0, 0);
        }
        tlv(0x02, &bytes)
    }

    fn base64_encode(data: &[u8]) -> String {
        const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let mut out = String::new();
        for chunk in data.chunks(3) {
            let mut word = [0u8; 3];
            word[..chunk.len()].copy_from_slice(chunk);
            let n = (u32::from(word[0]) << 16) | (u32::from(word[1]) << 8) | u32::from(word[2]);
            for i in 0..4 {
                if i <= chunk.len() {
                    out.push(ALPHABET[(n >> (18 - 6 * i) & 63) as usize] as char);
                } else {
                    out.push('=');
                }
            }
        }
        out
    }

    /// An RSA key from two Mersenne primes: weak, but enough to sign with.
    struct Key {
        n: BigUint,
        e: BigUint,
        d: BigUint,
    }

    fn key() -> Key {
        let one = BigUint::from(1u8);
        let p = (BigUint::from(1u8) << 521u32) - &one;
        let q = (BigUint::from(1u8) << 607u32) - &one;
        let e = BigUint::from(65537u32);
        let phi = (&p - &one) * (&q - &one);
        let d = e.modinv(&phi).unwrap();
        Key { n: p * q, e, d }
    }

    fn certificate(key: &Key) -> Vec<u8> {
        let name = tlv(
            0x30,
            &tlv(0x31, &tlv(0x30, &[tlv(0x06, &[0x55, 0x04, 0x03]), tlv(0x0c, b"Test")].concat())),
        );
        let spki = tlv(
            0x30,
            &[
                tlv(0x30, &[tlv(0x06, RSA_ENCRYPTION), vec![0x05, 0x00]].concat()),
                tlv(
                    0x03,
                    &[vec![0], tlv(0x30, &[integer(&key.n), integer(&key.e)].concat())].concat(),
                ),
            ]
            .concat(),
        );
        let tbs = tlv(
            0x30,
            &[
                tlv(0xa0, &tlv(0x02, &[2])),
                tlv(0x02, &[7]),
                tlv(0x30, &tlv(0x06, &[0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x01, 0x0b])),
                name.clone(),
                tlv(0x30, &[]),
                name,
                spki,
            ]
            .concat(),
        );
        tlv(0x30, &[tbs, tlv(0x30, &[]), tlv(0x03, &[0])].concat())
    }

    fn sign(key: &Key, data: &[u8]) -> Vec<u8> {
        let size = key.n.bits().div_ceil(8) as usize;
        let info = tlv(
            0x30,
            &[
                tlv(0x30, &[tlv(0x06, Algorithm::Sha256.oid()), vec![0x05, 0x00]].concat()),
                tlv(0x04, &Algorithm::Sha256.digest(data)),
            ]
            .concat(),
        );
        let mut encoded = vec![0x00, 0x01];
        encoded.resize(size - info.len() - 1, 0xff);
        encoded.push(0x00);
        encoded.extend(info);
        let s = BigUint::from_bytes_be(&encoded).modpow(&key.d, &key.n).to_bytes_be();
        [vec![0; size - s.len()], s].concat()
    }

    fn block(key: &Key, signature_file: &[u8]) -> Vec<u8> {
        let certificate = certificate(key);
        let (serial, issuer, _) = certificate_parts(&certificate).unwrap();
        let sha256 = tlv(0x30, &tlv(0x06, Algorithm::Sha256.oid()));
        let signer = tlv(
            0x30,
            &[
                tlv(0x02, &[1]),
                tlv(0x30, &[issuer.whole, serial.whole].concat()),
                sha256.clone(),
                tlv(0x30, &tlv(0x06, RSA_ENCRYPTION)),
                tlv(0x04, &sign(key, signature_file)),
            ]
            .concat(),
        );
        let signed = [
            tlv(0x02, &[1]),
            tlv(0x31, &sha256),
            tlv(0x30, &tlv(0x06, &[0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x07, 0x01])),
            tlv(0xa0, &certificate),
            tlv(0x31, &signer),
        ]
        .concat();
        tlv(0x30, &[tlv(0x06, SIGNED_DATA), tlv(0xa0, &tlv(0x30, &signed))].concat())
    }

    fn manifest(files: &[(&str, &[u8])]) -> Vec<u8> {
        let mut out = b"Manifest-Version: 1.0\r\n\r\n".to_vec();
        for (name, data) in files {
            let digest = base64_encode(&Algorithm::Sha256.digest(data));
            out.extend(format!("Name: {name}\r\nSHA-256-Digest: {digest}\r\n\r\n").bytes());
        }
        out
    }

    fn signature_file(manifest: &[u8]) -> Vec<u8> {
        let digest = base64_encode(&Algorithm::Sha256.digest(manifest));
        format!("Signature-Version: 1.0\r\nSHA-256-Digest-Manifest: {digest}\r\n\r\n").into_bytes()
    }

    fn zip(entries: &[(&str, &[u8])]) -> Vec<u8> {
        let mut writer = zip::ZipWriter::new(std::io::Cursor::new(Vec::new()));
        let options = zip::write::SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Stored);
        for (name, data) in entries {
            writer.start_file(*name, options).unwrap();
            writer.write_all(data).unwrap();
        }
        writer.finish().unwrap().into_inner()
    }

    const FILES: &[(&str, &[u8])] =
        &[("AndroidManifest.xml", b"manifest bytes"), ("classes.dex", b"dex bytes")];

    /// An APK signed over `signed` files that actually contains `contents`.
    fn apk(signed: &[(&str, &[u8])], contents: &[(&str, &[u8])]) -> Vec<u8> {
        let key = key();
        let manifest = manifest(signed);
        let signature_file = signature_file(&manifest);
        let block = block(&key, &signature_file);
        let mut entries: Vec<(&str, &[u8])> = vec![
            ("META-INF/MANIFEST.MF", &manifest),
            ("META-INF/CERT.SF", &signature_file),
            ("META-INF/CERT.RSA", &block),
        ];
        entries.extend_from_slice(contents);
        zip(&entries)
    }

    #[test]
    fn a_correctly_signed_archive_names_its_certificate() {
        let data = apk(FILES, FILES);
        assert_eq!(verify_signature(&data).unwrap(), Signature::Valid(certificate(&key())));
    }

    #[test]
    fn a_changed_added_or_missing_file_breaks_the_signature() {
        let changed: &[(&str, &[u8])] =
            &[("AndroidManifest.xml", b"manifest bytes"), ("classes.dex", b"evil bytes")];
        let added: &[(&str, &[u8])] = &[FILES[0], FILES[1], ("lib/armeabi/libx.so", b"extra")];
        for contents in [changed, added, &FILES[..1]] {
            assert!(matches!(
                verify_signature(&apk(FILES, contents)).unwrap(),
                Signature::Invalid(_)
            ));
        }
    }

    #[test]
    fn a_signature_over_other_content_is_refused() {
        let forged = signature_file(&manifest(&[("AndroidManifest.xml", b"other")]));
        let listed = manifest(FILES);
        let vouching = signature_file(&listed);
        let wrong_block = block(&key(), &forged);
        let entries: Vec<(&str, &[u8])> = vec![
            ("META-INF/MANIFEST.MF", &listed),
            ("META-INF/CERT.SF", &vouching),
            ("META-INF/CERT.RSA", &wrong_block),
            FILES[0],
            FILES[1],
        ];
        assert_eq!(
            verify_signature(&zip(&entries)).unwrap(),
            Signature::Invalid("signature does not verify")
        );
    }

    #[test]
    fn an_unsigned_archive_is_reported_as_such() {
        assert_eq!(verify_signature(&zip(FILES)).unwrap(), Signature::Unsigned);
        assert!(verify_signature(b"not a zip").is_err());
    }

    #[test]
    fn sections_keep_their_raw_bytes_and_join_continuations() {
        let text = b"Manifest-Version: 1.0\r\n\r\nName: a/very/long/na\r\n me.txt\r\nSHA1-Digest: AAAA\r\n\r\n";
        let found = sections(text);
        assert_eq!(found.len(), 2);
        assert_eq!(found[1].attribute("name"), Some("a/very/long/name.txt"));
        assert_eq!(found[1].raw, &text[25..]);
        assert_eq!(found[1].digests("-Digest"), [(Algorithm::Sha1, vec![0, 0, 0])]);
    }

    #[test]
    fn base64_decodes_padding_and_refuses_garbage() {
        assert_eq!(base64("TWFu").unwrap(), b"Man");
        assert_eq!(base64("TWE=").unwrap(), b"Ma");
        assert_eq!(base64("TQ==").unwrap(), b"M");
        assert_eq!(base64(&base64_encode(&[0xff; 32])).unwrap(), [0xff; 32]);
        assert!(base64("TQ=").is_none());
        assert!(base64("T===").is_none());
        assert!(base64("TQ==TWFu").is_none());
        assert!(base64("T!Fu").is_none());
    }
}
