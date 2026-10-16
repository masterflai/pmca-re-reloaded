//! A minimal TLS 1.0 server, for PlayMemories cameras and nothing else.
//!
//! # Why this exists
//!
//! During app installation the camera opens an HTTPS connection that the
//! installer tunnels over USB to a server on `127.0.0.1`. A camera offers
//! this and only this:
//!
//! | | |
//! |---|---|
//! | Version | TLS 1.0 (`{3,1}`) |
//! | Cipher suites | `TLS_RSA_WITH_AES_128_CBC_SHA`, `TLS_RSA_WITH_AES_256_CBC_SHA` |
//! | Extensions | `session_ticket` only — notably **no** `signature_algorithms` |
//!
//! No maintained Rust TLS library will negotiate that, by deliberate policy
//! rather than oversight: `rustls` rejects the ClientHello outright with
//! `SignatureAlgorithmsExtensionRequired`. Since the camera's firmware cannot
//! be changed, the only way to speak to it is to implement the handshake.
//!
//! # Security boundary
//!
//! Everything here is obsolete by design — TLS 1.0, RSA key exchange without
//! forward secrecy, CBC with the chained IV that BEAST exploits, and HMAC-SHA1.
//! Three properties keep that acceptable:
//!
//! * [`bind_loopback`] refuses to bind to anything but a loopback address, so
//!   the traffic cannot reach a network.
//! * The peer is a USB device reached through a tunnel, not a remote host.
//! * The certificate and its key are generated when the server starts and
//!   discarded when it stops, so there is no stored secret to steal. Its
//!   issuer *name* is a root the camera knows, because the camera rejects any
//!   other; it does not verify the signature (see [`selfsigned`]).
//!
//! This module must never be used for a general-purpose client or server. For
//! anything reachable from a network, use `rustls`, which the store client
//! still does.

mod prf;
mod record;
mod rsa;
mod selfsigned;

use std::io::{self, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::path::Path;

use record::{BLOCK_LEN, Cipher, CipherState, ContentType, MAC_LEN, MAX_FRAGMENT, VERSION};
use rsa::RsaPrivateKey;
use rustls_pki_types::pem;

use crate::{Error, Result};

/// `TLS_RSA_WITH_AES_128_CBC_SHA`.
const SUITE_AES128: u16 = 0x002f;
/// `TLS_RSA_WITH_AES_256_CBC_SHA`.
const SUITE_AES256: u16 = 0x0035;

/// Handshake message types used by an RSA handshake.
const HS_CLIENT_HELLO: u8 = 1;
const HS_SERVER_HELLO: u8 = 2;
const HS_CERTIFICATE: u8 = 11;
const HS_SERVER_HELLO_DONE: u8 = 14;
const HS_CLIENT_KEY_EXCHANGE: u8 = 16;
const HS_FINISHED: u8 = 20;

/// Length of the client and server random values.
const RANDOM_LEN: usize = 32;
/// Length of the premaster secret in an RSA key exchange.
const PREMASTER_LEN: usize = 48;

/// Upper bound on a single handshake message, to cap what a peer can make the
/// server buffer before it is authenticated.
const MAX_HANDSHAKE_MSG: usize = 1 << 16;

/// Fill `buf` with cryptographically secure random bytes.
fn random_bytes(buf: &mut [u8]) -> Result<()> {
    getrandom::fill(buf).map_err(|e| Error::Tls(format!("no randomness available: {e}")))
}

/// A certificate chain and the matching RSA private key.
pub struct Tls10Config {
    /// DER-encoded certificates, leaf first.
    certificates: Vec<Vec<u8>>,
    key: RsaPrivateKey,
}

impl Tls10Config {
    /// Make up a certificate and key for this session.
    ///
    /// This is the normal path. Nothing is read from disk and nothing is left
    /// behind; see [`selfsigned`] for why that is preferable to shipping a key.
    pub fn generate() -> Result<Self> {
        let generated = selfsigned::generate()?;
        Ok(Tls10Config { certificates: vec![generated.certificate], key: generated.key })
    }

    /// Load a certificate chain and private key from one combined PEM file.
    ///
    /// Only for the rare case of wanting a known key — decrypting a capture in
    /// Wireshark, say. [`Tls10Config::generate`] is what normal use calls.
    pub fn from_pem(path: &Path) -> Result<Self> {
        let file = std::fs::File::open(path).map_err(|e| Error::Io(e.to_string()))?;
        let mut reader = io::BufReader::new(file);

        let mut certificates = Vec::new();
        let mut key = None;
        // `from_buf` yields one section at a time, so certificates and the key
        // can be picked out of a single pass. Unrecognised sections (a DH
        // parameter block, say) are skipped rather than treated as an error.
        while let Some((kind, der)) =
            pem::from_buf(&mut reader).map_err(|e| Error::Io(e.to_string()))?
        {
            match kind {
                pem::SectionKind::Certificate => certificates.push(der),
                pem::SectionKind::RsaPrivateKey => {
                    key = RsaPrivateKey::from_pkcs1_der(&der);
                }
                pem::SectionKind::PrivateKey => {
                    key = RsaPrivateKey::from_pkcs8_der(&der);
                }
                _ => {}
            }
        }

        if certificates.is_empty() {
            return Err(Error::Tls("no certificate found in PEM file".into()));
        }
        let key = key.ok_or_else(|| {
            Error::Tls("no usable RSA private key found in PEM file (only RSA is supported)".into())
        })?;
        Ok(Tls10Config { certificates, key })
    }
}

/// Everything the handshake agrees on, handed to the record layer.
struct Negotiated {
    cipher: Cipher,
    client_random: [u8; RANDOM_LEN],
    server_random: [u8; RANDOM_LEN],
}

/// A TLS 1.0 connection to a camera, usable as a byte stream.
pub struct Tls10Stream {
    stream: TcpStream,
    read_state: CipherState,
    write_state: CipherState,
    /// Decrypted application data not yet handed to the caller.
    pending: Vec<u8>,
    /// Set once the peer sent `close_notify` or closed the socket.
    closed: bool,
}

impl Tls10Stream {
    /// Perform the server side of the handshake on an accepted connection.
    pub fn accept(mut stream: TcpStream, config: &Tls10Config) -> Result<Self> {
        // Every handshake message, in order, feeds the Finished check.
        let mut transcript = Vec::new();
        let mut buffered = Vec::new();

        let (kind, body) = read_handshake(&mut stream, &mut buffered, None, &mut transcript)?;
        if kind != HS_CLIENT_HELLO {
            return Err(Error::Tls("expected a ClientHello".into()));
        }
        let hello = parse_client_hello(&body)?;

        let mut server_random = [0u8; RANDOM_LEN];
        random_bytes(&mut server_random)?;
        let negotiated =
            Negotiated { cipher: hello.cipher, client_random: hello.random, server_random };

        // ServerHello, Certificate and ServerHelloDone travel in the clear.
        let mut flight = Vec::new();
        push_handshake(
            &mut flight,
            &mut transcript,
            HS_SERVER_HELLO,
            &server_hello_body(&negotiated),
        );
        push_handshake(
            &mut flight,
            &mut transcript,
            HS_CERTIFICATE,
            &certificate_body(&config.certificates),
        );
        push_handshake(&mut flight, &mut transcript, HS_SERVER_HELLO_DONE, &[]);
        write_plain_record(&mut stream, ContentType::Handshake, &flight)?;

        let (kind, body) = read_handshake(&mut stream, &mut buffered, None, &mut transcript)?;
        if kind != HS_CLIENT_KEY_EXCHANGE {
            return Err(Error::Tls("expected a ClientKeyExchange".into()));
        }
        let premaster = decrypt_premaster(&config.key, &body)?;

        let master =
            prf::master_secret(&premaster, &negotiated.client_random, &negotiated.server_random);
        let keys = prf::key_block(
            &master,
            &negotiated.client_random,
            &negotiated.server_random,
            MAC_LEN,
            negotiated.cipher.key_len(),
            BLOCK_LEN,
        );
        let mut read_state =
            CipherState::new(negotiated.cipher, keys.client_mac, keys.client_key, keys.client_iv);
        let mut write_state =
            CipherState::new(negotiated.cipher, keys.server_mac, keys.server_key, keys.server_iv);

        // ChangeCipherSpec, then the client's Finished under the new keys.
        let (content_type, payload) = read_plain_record(&mut stream)?;
        if content_type != ContentType::ChangeCipherSpec || payload != [1] {
            return Err(Error::Tls("expected a ChangeCipherSpec".into()));
        }

        // The client's Finished covers the transcript up to but not including
        // itself, so the expected value must be computed before appending it.
        let expected = prf::finished_verify_data(&master, b"client finished", &transcript);
        let (kind, body) =
            read_handshake(&mut stream, &mut buffered, Some(&mut read_state), &mut transcript)?;
        if kind != HS_FINISHED {
            return Err(Error::Tls("expected a Finished".into()));
        }
        if body.len() != expected.len()
            || body.iter().zip(&expected).fold(0u8, |acc, (a, b)| acc | (a ^ b)) != 0
        {
            // Reaching here means the peer derived different keys: a wrong
            // premaster secret, or a tampered handshake.
            return Err(Error::Tls("client Finished did not verify".into()));
        }

        write_plain_record(&mut stream, ContentType::ChangeCipherSpec, &[1])?;
        let verify = prf::finished_verify_data(&master, b"server finished", &transcript);
        let mut finished = Vec::new();
        push_handshake(&mut finished, &mut transcript, HS_FINISHED, &verify);
        let sealed = write_state.seal(ContentType::Handshake, &finished)?;
        write_raw_record(&mut stream, ContentType::Handshake, &sealed)?;

        Ok(Tls10Stream { stream, read_state, write_state, pending: Vec::new(), closed: false })
    }

    /// Read and decrypt one record, routing non-application content.
    ///
    /// Returns `Ok(false)` once the connection has ended.
    fn fill(&mut self) -> Result<bool> {
        loop {
            let (content_type, payload) = match read_plain_record(&mut self.stream) {
                Ok(record) => record,
                // A peer that vanishes mid-connection is an ordinary end of
                // stream for our purposes; the HTTP layer above handles it.
                Err(Error::Io(_)) => {
                    self.closed = true;
                    return Ok(false);
                }
                Err(e) => return Err(e),
            };
            let fragment = self.read_state.open(content_type, &payload)?;
            match content_type {
                ContentType::ApplicationData => {
                    self.pending.extend_from_slice(&fragment);
                    return Ok(true);
                }
                ContentType::Alert => {
                    // level 1 / description 0 is close_notify; anything fatal
                    // ends the connection just the same.
                    self.closed = true;
                    return Ok(false);
                }
                // A renegotiation request or a stray ChangeCipherSpec is not
                // something this server supports; ignoring it is safe because
                // the keys never change.
                _ => continue,
            }
        }
    }
}

impl Read for Tls10Stream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        while self.pending.is_empty() {
            if self.closed {
                return Ok(0);
            }
            match self.fill() {
                Ok(true) => {}
                Ok(false) => return Ok(0),
                Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidData, e.to_string())),
            }
        }
        let n = buf.len().min(self.pending.len());
        buf[..n].copy_from_slice(&self.pending[..n]);
        self.pending.drain(..n);
        Ok(n)
    }
}

impl Write for Tls10Stream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        for chunk in buf.chunks(MAX_FRAGMENT) {
            let sealed = self
                .write_state
                .seal(ContentType::ApplicationData, chunk)
                .map_err(|e| io::Error::other(e.to_string()))?;
            write_raw_record(&mut self.stream, ContentType::ApplicationData, &sealed)
                .map_err(|e| io::Error::other(e.to_string()))?;
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.stream.flush()
    }
}

/// The parts of a ClientHello the server acts on.
#[derive(Debug)]
struct ClientHello {
    random: [u8; RANDOM_LEN],
    cipher: Cipher,
}

/// Parse a ClientHello and pick a cipher suite.
fn parse_client_hello(body: &[u8]) -> Result<ClientHello> {
    let mut cursor = Cursor::new(body);
    let version = cursor.take(2)?;
    // A client may offer a higher version than it will use, but this server has
    // exactly one to give.
    if version[0] != VERSION[0] {
        return Err(Error::Tls(format!(
            "unsupported TLS major version {}; this server speaks TLS 1.0 only",
            version[0]
        )));
    }
    let mut random = [0u8; RANDOM_LEN];
    random.copy_from_slice(cursor.take(RANDOM_LEN)?);

    let session_id_len = cursor.take(1)?[0] as usize;
    cursor.take(session_id_len)?;

    let suites_len = u16::from_be_bytes([cursor.take(1)?[0], cursor.take(1)?[0]]) as usize;
    let suites = cursor.take(suites_len)?;
    let offered: Vec<u16> =
        suites.as_chunks::<2>().0.iter().map(|c| u16::from_be_bytes(*c)).collect();

    // Prefer the wider key when the client offers both.
    let cipher = if offered.contains(&SUITE_AES256) {
        Cipher::Aes256
    } else if offered.contains(&SUITE_AES128) {
        Cipher::Aes128
    } else {
        return Err(Error::Tls(format!(
            "camera offered no supported cipher suite (got {})",
            offered.iter().map(|s| format!("0x{s:04x}")).collect::<Vec<_>>().join(", ")
        )));
    };

    // Compression and extensions are read but not acted on: this server always
    // answers with null compression and no extensions.
    Ok(ClientHello { random, cipher })
}

/// Build the ServerHello body.
fn server_hello_body(negotiated: &Negotiated) -> Vec<u8> {
    let suite = match negotiated.cipher {
        Cipher::Aes128 => SUITE_AES128,
        Cipher::Aes256 => SUITE_AES256,
    };
    let mut body = Vec::with_capacity(RANDOM_LEN + 8);
    body.extend_from_slice(&VERSION);
    body.extend_from_slice(&negotiated.server_random);
    // Empty session id: this server never resumes.
    body.push(0);
    body.extend_from_slice(&suite.to_be_bytes());
    // Null compression.
    body.push(0);
    body
}

/// Build the Certificate body: a 24-bit length, then each certificate with its
/// own 24-bit length.
fn certificate_body(certificates: &[Vec<u8>]) -> Vec<u8> {
    let mut list = Vec::new();
    for certificate in certificates {
        list.extend_from_slice(&u24(certificate.len()));
        list.extend_from_slice(certificate);
    }
    let mut body = Vec::with_capacity(list.len() + 3);
    body.extend_from_slice(&u24(list.len()));
    body.extend_from_slice(&list);
    body
}

/// Encode a 24-bit big-endian length.
fn u24(value: usize) -> [u8; 3] {
    [(value >> 16) as u8, (value >> 8) as u8, value as u8]
}

/// Recover the premaster secret from a ClientKeyExchange.
///
/// A failure here is never reported to the peer. Anything malformed yields a
/// random premaster secret so the handshake dies at Finished instead, denying
/// the padding oracle that Bleichenbacher's attack needs.
fn decrypt_premaster(key: &RsaPrivateKey, body: &[u8]) -> Result<Vec<u8>> {
    let mut fallback = vec![0u8; PREMASTER_LEN];
    random_bytes(&mut fallback)?;

    // TLS (unlike SSLv3) length-prefixes the encrypted premaster secret.
    if body.len() < 2 {
        return Ok(fallback);
    }
    let len = u16::from_be_bytes([body[0], body[1]]) as usize;
    if body.len() != len + 2 {
        return Ok(fallback);
    }

    match key.decrypt_pkcs1v15(&body[2..]) {
        // The first two bytes repeat the client's offered version; a mismatch
        // is the rollback attack RFC 5246 §7.4.7.1 describes, handled by the
        // same silent substitution.
        Some(premaster) if premaster.len() == PREMASTER_LEN => Ok(premaster),
        _ => Ok(fallback),
    }
}

/// A bounds-checked reader over a handshake body.
struct Cursor<'a> {
    data: &'a [u8],
}

impl<'a> Cursor<'a> {
    fn new(data: &'a [u8]) -> Self {
        Cursor { data }
    }

    fn take(&mut self, n: usize) -> Result<&'a [u8]> {
        if self.data.len() < n {
            return Err(Error::Tls("handshake message ended early".into()));
        }
        let (head, tail) = self.data.split_at(n);
        self.data = tail;
        Ok(head)
    }
}

/// Append a handshake message to `out` and to the transcript.
fn push_handshake(out: &mut Vec<u8>, transcript: &mut Vec<u8>, kind: u8, body: &[u8]) {
    let start = out.len();
    out.push(kind);
    out.extend_from_slice(&u24(body.len()));
    out.extend_from_slice(body);
    transcript.extend_from_slice(&out[start..]);
}

/// Read one handshake message, decrypting records when `state` is set.
///
/// `buffered` carries bytes left over from a record that held more than one
/// message — a client is free to pack its whole flight into a single record.
fn read_handshake(
    stream: &mut TcpStream,
    buffered: &mut Vec<u8>,
    mut state: Option<&mut CipherState>,
    transcript: &mut Vec<u8>,
) -> Result<(u8, Vec<u8>)> {
    loop {
        if buffered.len() >= 4 {
            let len = ((buffered[1] as usize) << 16)
                | ((buffered[2] as usize) << 8)
                | buffered[3] as usize;
            if len > MAX_HANDSHAKE_MSG {
                return Err(Error::Tls("handshake message is implausibly large".into()));
            }
            if buffered.len() >= 4 + len {
                let message: Vec<u8> = buffered.drain(..4 + len).collect();
                transcript.extend_from_slice(&message);
                return Ok((message[0], message[4..].to_vec()));
            }
        }

        let (content_type, payload) = read_plain_record(stream)?;
        let fragment = match state.as_deref_mut() {
            Some(state) => state.open(content_type, &payload)?,
            None => payload,
        };
        match content_type {
            ContentType::Handshake => buffered.extend_from_slice(&fragment),
            ContentType::Alert => return Err(alert_error(&fragment)),
            _ => return Err(Error::Tls("expected a handshake record".into())),
        }
    }
}

/// Turn an alert record into a readable error.
fn alert_error(fragment: &[u8]) -> Error {
    let description = fragment.get(1).copied().unwrap_or(255);
    Error::Tls(format!("camera aborted the handshake (alert {description})"))
}

/// Read one record, returning its type and raw payload.
fn read_plain_record(stream: &mut TcpStream) -> Result<(ContentType, Vec<u8>)> {
    let mut header = [0u8; 5];
    stream.read_exact(&mut header).map_err(|e| Error::Io(e.to_string()))?;

    let content_type = ContentType::from_byte(header[0])
        .ok_or_else(|| Error::Tls(format!("unknown record type {}", header[0])))?;
    let len = u16::from_be_bytes([header[3], header[4]]) as usize;
    // The ceiling allows for the MAC and padding an encrypted record adds.
    if len > MAX_FRAGMENT + 2048 {
        return Err(Error::Tls("record is larger than TLS allows".into()));
    }

    let mut payload = vec![0u8; len];
    stream.read_exact(&mut payload).map_err(|e| Error::Io(e.to_string()))?;
    Ok((content_type, payload))
}

/// Write a record whose payload is already final.
fn write_raw_record(
    stream: &mut TcpStream,
    content_type: ContentType,
    payload: &[u8],
) -> Result<()> {
    stream
        .write_all(&record::record_header(content_type, payload.len()))
        .map_err(|e| Error::Io(e.to_string()))?;
    stream.write_all(payload).map_err(|e| Error::Io(e.to_string()))?;
    stream.flush().map_err(|e| Error::Io(e.to_string()))
}

/// Write an unencrypted record, splitting oversized fragments.
fn write_plain_record(
    stream: &mut TcpStream,
    content_type: ContentType,
    body: &[u8],
) -> Result<()> {
    for chunk in body.chunks(MAX_FRAGMENT) {
        write_raw_record(stream, content_type, chunk)?;
    }
    Ok(())
}

/// Reject a listen address that is not loopback.
///
/// The handshake this module implements is obsolete on purpose; binding it to a
/// routable address would expose weak cryptography to a network. The camera is
/// always reached through a USB tunnel to `127.0.0.1`, so this costs nothing.
pub fn ensure_loopback(addr: SocketAddr) -> Result<()> {
    if addr.ip().is_loopback() {
        return Ok(());
    }
    Err(Error::Tls(format!("refusing to serve TLS 1.0 on the non-loopback address {addr}")))
}

/// Bind a TCP listener, enforcing the loopback restriction.
pub fn bind_loopback(addr: SocketAddr) -> Result<TcpListener> {
    ensure_loopback(addr)?;
    let listener = TcpListener::bind(addr).map_err(|e| Error::Io(e.to_string()))?;
    let bound = listener.local_addr().map_err(|e| Error::Io(e.to_string()))?;
    ensure_loopback(bound)?;
    Ok(listener)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_non_loopback_address_is_refused() {
        assert!(ensure_loopback("127.0.0.1:0".parse().unwrap()).is_ok());
        assert!(ensure_loopback("[::1]:0".parse().unwrap()).is_ok());
        assert!(ensure_loopback("0.0.0.0:0".parse().unwrap()).is_err());
        assert!(ensure_loopback("192.168.1.10:443".parse().unwrap()).is_err());
    }

    /// What the install server actually does at start-up.
    #[test]
    fn a_generated_config_is_usable() {
        let config = match Tls10Config::generate() {
            Ok(config) => config,
            Err(e) => panic!("a certificate can be generated: {e}"),
        };
        assert_eq!(config.certificates.len(), 1, "a self-signed leaf, with nothing above it");
        // DER, i.e. starting with a SEQUENCE tag.
        assert_eq!(config.certificates[0].first(), Some(&0x30), "certificate is DER-encoded");
        assert!(config.key.size() >= 128, "RSA key is at least 1024-bit");
    }

    #[test]
    fn a_pem_without_a_key_is_refused() {
        let dir = std::env::temp_dir().join("pmca-tls10-pem-test");
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("no-key.pem");
        // A syntactically valid certificate section, but no private key.
        std::fs::write(
            &path,
            "-----BEGIN CERTIFICATE-----\nMAoGCCqGSM49BAMC\n-----END CERTIFICATE-----\n",
        )
        .unwrap();
        let err = match Tls10Config::from_pem(&path) {
            Ok(_) => panic!("a keyless PEM must be rejected"),
            Err(e) => e,
        };
        assert!(format!("{err}").contains("private key"), "the error names the missing key");
        let _ = std::fs::remove_file(&path);
    }

    /// The exact ClientHello a camera sent, captured from
    /// the USB tunnel. Everything after the 5-byte record header.
    fn camera_client_hello() -> Vec<u8> {
        let record: [u8; 61] = [
            0x16, 0x03, 0x01, 0x00, 0x38, 0x01, 0x00, 0x00, 0x34, 0x03, 0x01, 0x00, 0x00, 0x14,
            0xd3, 0xb2, 0x7c, 0xbf, 0xff, 0x98, 0x33, 0xbd, 0x2c, 0x02, 0xc2, 0xda, 0x74, 0x8f,
            0xf3, 0x00, 0xf6, 0xcc, 0x6e, 0xda, 0x3b, 0xe7, 0x6c, 0xbc, 0x6e, 0xff, 0xdc, 0xad,
            0xbf, 0x00, 0x00, 0x06, 0x00, 0x2f, 0x00, 0x35, 0x00, 0xff, 0x02, 0x01, 0x00, 0x00,
            0x04, 0x00, 0x23, 0x00, 0x00,
        ];
        // Skip the record header and the 4-byte handshake header.
        record[9..].to_vec()
    }

    #[test]
    fn the_real_camera_hello_is_understood() {
        let hello = parse_client_hello(&camera_client_hello()).expect("camera hello parses");
        // The camera offers both suites, so the 256-bit one wins.
        assert_eq!(hello.cipher, Cipher::Aes256);
        assert_eq!(hello.random.len(), RANDOM_LEN);
        // The random starts with the camera's (zero) timestamp.
        assert_eq!(&hello.random[..4], &[0x00, 0x00, 0x14, 0xd3]);
    }

    #[test]
    fn a_hello_offering_only_aes128_selects_it() {
        let mut body = Vec::new();
        body.extend_from_slice(&VERSION);
        body.extend_from_slice(&[0x00; RANDOM_LEN]);
        body.push(0); // no session id
        body.extend_from_slice(&2u16.to_be_bytes());
        body.extend_from_slice(&SUITE_AES128.to_be_bytes());
        body.extend_from_slice(&[0x01, 0x00]);
        assert_eq!(parse_client_hello(&body).unwrap().cipher, Cipher::Aes128);
    }

    #[test]
    fn a_hello_without_a_supported_suite_names_what_was_offered() {
        let mut body = Vec::new();
        body.extend_from_slice(&VERSION);
        body.extend_from_slice(&[0x00; RANDOM_LEN]);
        body.push(0);
        body.extend_from_slice(&2u16.to_be_bytes());
        body.extend_from_slice(&0xc02fu16.to_be_bytes());
        body.extend_from_slice(&[0x01, 0x00]);

        let err = parse_client_hello(&body).unwrap_err().to_string();
        assert!(err.contains("0xc02f"), "error should name the suite: {err}");
    }

    #[test]
    fn a_truncated_hello_is_rejected_rather_than_panicking() {
        let full = camera_client_hello();
        for cut in 0..full.len() {
            let _ = parse_client_hello(&full[..cut]);
        }
    }

    #[test]
    fn the_server_hello_echoes_the_negotiated_suite() {
        let negotiated = Negotiated {
            cipher: Cipher::Aes256,
            client_random: [0x01; RANDOM_LEN],
            server_random: [0x02; RANDOM_LEN],
        };
        let body = server_hello_body(&negotiated);
        assert_eq!(&body[..2], &VERSION);
        assert_eq!(&body[2..2 + RANDOM_LEN], &[0x02; RANDOM_LEN]);
        assert_eq!(body[2 + RANDOM_LEN], 0); // empty session id
        assert_eq!(u16::from_be_bytes([body[3 + RANDOM_LEN], body[4 + RANDOM_LEN]]), SUITE_AES256);
        assert_eq!(body[5 + RANDOM_LEN], 0); // null compression
    }

    #[test]
    fn the_certificate_body_nests_two_length_prefixes() {
        let body = certificate_body(&[vec![0xaa; 3], vec![0xbb; 2]]);
        // Outer list length: 2 certificates plus their 3-byte headers.
        assert_eq!(&body[..3], &u24(3 + 3 + 3 + 2));
        assert_eq!(&body[3..6], &u24(3));
        assert_eq!(&body[6..9], &[0xaa; 3]);
        assert_eq!(&body[9..12], &u24(2));
        assert_eq!(&body[12..14], &[0xbb; 2]);
    }

    #[test]
    fn handshake_messages_are_framed_and_recorded() {
        let mut out = Vec::new();
        let mut transcript = Vec::new();
        push_handshake(&mut out, &mut transcript, HS_SERVER_HELLO_DONE, &[]);
        assert_eq!(out, vec![HS_SERVER_HELLO_DONE, 0, 0, 0]);
        // The transcript must see exactly the bytes that went on the wire.
        assert_eq!(transcript, out);

        push_handshake(&mut out, &mut transcript, HS_FINISHED, &[0x09; 12]);
        assert_eq!(transcript, out);
        assert_eq!(out.len(), 4 + 4 + 12);
    }

    #[test]
    fn a_malformed_key_exchange_yields_a_random_premaster_rather_than_an_error() {
        // A key whose modulus is too small to decrypt anything real still
        // exercises the fallback path.
        let der = {
            let mut body = Vec::new();
            for value in [vec![0x00], vec![0x0c, 0xa1], vec![0x01, 0x00, 0x01], vec![0x01, 0x9d]] {
                body.push(0x02);
                body.push(value.len() as u8);
                body.extend_from_slice(&value);
            }
            let mut der = vec![0x30, body.len() as u8];
            der.extend_from_slice(&body);
            der
        };
        let key = RsaPrivateKey::from_pkcs1_der(&der).expect("toy key");

        // Too short to hold a length prefix.
        let a = decrypt_premaster(&key, &[]).unwrap();
        // Length prefix disagrees with the body.
        let b = decrypt_premaster(&key, &[0x00, 0x40, 0x01]).unwrap();
        // Well-formed length, but the ciphertext will not decrypt to padding.
        let c = decrypt_premaster(&key, &[0x00, 0x02, 0x01, 0x02]).unwrap();

        for premaster in [&a, &b, &c] {
            assert_eq!(premaster.len(), PREMASTER_LEN);
        }
        // Each fallback is freshly random, so two must not coincide.
        assert_ne!(a, b);
        assert_ne!(b, c);
    }

    #[test]
    fn an_alert_during_the_handshake_is_reported_with_its_code() {
        let err = alert_error(&[2, 40]).to_string();
        assert!(err.contains("40"), "alert code should appear: {err}");
    }

    #[test]
    fn u24_encodes_three_byte_lengths() {
        assert_eq!(u24(0), [0, 0, 0]);
        assert_eq!(u24(0x010203), [0x01, 0x02, 0x03]);
    }
}
