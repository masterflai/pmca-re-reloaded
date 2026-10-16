//! Service-mode ("senser") transport, authentication and command layer.
//!
//! Byte-faithful port of `SonySenserDevice`, `SonySenserAuthDevice` and
//! `SonySenserCamera` from `pmca/usb/sony.py`.
//!
//! Cameras expose a vendor-specific USB interface once they have been switched
//! into service mode. Before any command works the host must pass a
//! challenge/response handshake ([`SenserAuthDevice::authenticate`]) that uses
//! the service-mode keys from [`pmca_core::keystore`] together with SHA-256 or
//! Sony's non-conforming SHA-1 variant ([`pmca_core::sha1::sha1_faulty`]). The
//! keys are supplied by the user, not shipped; without them authentication is
//! refused.
//!
//! # Safety / risk note
//!
//! Service mode grants raw access to the camera's memory, filesystem and
//! calibration backup. Write operations here can permanently damage a device;
//! they are ported for parity with the original tool and are expected to be
//! gated behind explicit user confirmation at the UI level.

use std::io::Write;

use pmca_core::keystore::{self, KeyId};
use pmca_core::sha1::sha1_faulty;
use sha2::{Digest, Sha256};

use crate::{Error, Result, UsbBackend};

/// Length of a single service-mode authentication key.
pub const SENSER_KEY_SIZE: usize = 0x200;

/// Number of authentication rounds (one key per round).
pub const SENSER_ROUNDS: usize = 3;

/// The service-mode key for one round and hash algorithm, from the key store.
fn senser_key(use_sha1: bool, round: usize) -> Result<Vec<u8>> {
    let id = match (use_sha1, round) {
        (true, 0) => KeyId::SenserSha1A,
        (true, 1) => KeyId::SenserSha1B,
        (true, 2) => KeyId::SenserSha1C,
        (false, 0) => KeyId::SenserSha256A,
        (false, 1) => KeyId::SenserSha256B,
        (false, 2) => KeyId::SenserSha256C,
        _ => return Err(Error::Senser("invalid authentication round".into())),
    };
    keystore::get(id).ok_or(Error::MissingKeys)
}

/// Size of the senser packet header in bytes.
pub const SENSER_HEADER_SIZE: usize = 12;

/// Transfer granularity the camera pads its responses to.
pub const SENSER_MIN_SIZE: usize = 0x200;

/// Maximum number of bytes read before the camera inserts a padding packet.
pub const SENSER_CHUNK_SIZE: usize = 0x8000;

/// Maximum payload carried by a single senser packet.
pub const SENSER_MAX_SIZE: usize = 0x100000;

/// Product id of the camera generation that uses the extended key handshake and
/// does not pad its senser responses.
pub const PID_SENSER_EXTENDED: u16 = 0x0336;

/// Vendor control request that starts the senser subsystem: `(request, value, index)`.
pub const VENDOR_REQUEST_START_SENSER: (u8, u16, u16) = (1, 0x37ff, 0xd7aa);

/// Vendor control request that stops the senser subsystem: `(request, value, index)`.
pub const VENDOR_REQUEST_STOP_SENSER: (u8, u16, u16) = (1, 0xc800, 0x2855);

/// Size of an authentication packet (2 + 2 + 0x200 bytes, big-endian).
pub const AUTH_PACKET_SIZE: usize = 4 + SENSER_KEY_SIZE;

/// Chunk size used for the terminal (serial console) endpoints.
pub const TERMINAL_CHUNK_SIZE: usize = 0x40;

/// Consecutive zero-length transfers after which a trace gives up.
const TRACE_EMPTY_LIMIT: usize = 64;

/// A senser reply as it arrived on the wire; see [`SenserDevice::trace_packet`].
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct WireTrace {
    /// Length of every bulk transfer in order; `0` is a zero-length packet.
    pub transfers: Vec<usize>,
    /// All received bytes, concatenated.
    pub data: Vec<u8>,
    /// The transfer error that ended the trace, if one did.
    pub end: Option<String>,
}

/// Header prefixed to every senser packet (little-endian).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SenserPacketHeader {
    /// Number of payload bytes still to follow (may exceed [`SENSER_MAX_SIZE`]).
    pub size: u32,
    /// Target function / subsystem selector.
    pub p_func: u16,
    /// Request sequence number; the reply must echo it.
    pub sequence: u16,
    /// Protocol version (always `0`).
    pub version: u8,
    /// Micon type selector (always `0`).
    pub micon_type: u8,
    /// Offset type selector (always `0`).
    pub offset_type: u8,
    /// Result code, only meaningful in replies.
    pub response: u8,
}

impl SenserPacketHeader {
    /// Serializes the header.
    pub fn pack(&self) -> [u8; SENSER_HEADER_SIZE] {
        let mut out = [0u8; SENSER_HEADER_SIZE];
        out[0..4].copy_from_slice(&self.size.to_le_bytes());
        out[4..6].copy_from_slice(&self.p_func.to_le_bytes());
        out[6..8].copy_from_slice(&self.sequence.to_le_bytes());
        out[8] = self.version;
        out[9] = self.micon_type;
        out[10] = self.offset_type;
        out[11] = self.response;
        out
    }

    /// Parses a header from the start of `data`.
    pub fn unpack(data: &[u8]) -> Result<Self> {
        if data.len() < SENSER_HEADER_SIZE {
            return Err(Error::Protocol("senser packet shorter than its header"));
        }
        Ok(Self {
            size: u32::from_le_bytes([data[0], data[1], data[2], data[3]]),
            p_func: u16::from_le_bytes([data[4], data[5]]),
            sequence: u16::from_le_bytes([data[6], data[7]]),
            version: data[8],
            micon_type: data[9],
            offset_type: data[10],
            response: data[11],
        })
    }
}

/// Bulk endpoint addresses of the vendor-specific service interface.
///
/// The interface exposes two bulk pairs: the first carries senser packets, the
/// second an optional debug terminal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SenserEndpoints {
    /// Bulk IN endpoint for senser packets.
    pub ep_in: u8,
    /// Bulk OUT endpoint for senser packets.
    pub ep_out: u8,
    /// Bulk IN endpoint of the debug terminal, if present.
    pub terminal_in: Option<u8>,
    /// Bulk OUT endpoint of the debug terminal, if present.
    pub terminal_out: Option<u8>,
}

/// Packet-level senser transport.
pub struct SenserDevice<B: UsbBackend> {
    backend: B,
    endpoints: SenserEndpoints,
    product_id: u16,
    sequence: u16,
}

impl<B: UsbBackend> SenserDevice<B> {
    /// Wraps `backend`; `product_id` selects the response padding behaviour.
    pub fn new(backend: B, endpoints: SenserEndpoints, product_id: u16) -> Self {
        Self { backend, endpoints, product_id, sequence: 1 }
    }

    /// Returns the wrapped backend.
    pub fn into_inner(self) -> B {
        self.backend
    }

    /// Asks the camera to leave service mode.
    ///
    /// This must be sent when a session ends. Without it the camera stays in
    /// service mode, and the next session's authentication runs against an
    /// already-started subsystem, which answers with an endpoint stall.
    /// Errors are ignored: the camera re-enumerates while handling the request.
    pub fn stop(&mut self) {
        let (request, value, index) = VENDOR_REQUEST_STOP_SENSER;
        let _ = self.backend.vendor_request_out(request, value, index, &[]);
    }

    /// Padding size the camera aligns its replies to (`0` on newer models).
    fn min_size(&self) -> usize {
        if self.product_id == PID_SENSER_EXTENDED { 0 } else { SENSER_MIN_SIZE }
    }

    /// Reads exactly `n` bytes, looping over short transfers.
    fn read_all(&mut self, n: usize) -> Result<Vec<u8>> {
        let mut data = Vec::with_capacity(n);
        while data.len() < n {
            let chunk = self.backend.read(self.endpoints.ep_in, n - data.len())?;
            if chunk.is_empty() {
                return Err(Error::Protocol("senser read returned no data"));
            }
            data.extend_from_slice(&chunk);
        }
        data.truncate(n);
        Ok(data)
    }

    /// Sends a senser packet and streams the reply into `out`.
    ///
    /// Returns the reply's [`SenserPacketHeader::response`] code.
    pub fn send_packet(&mut self, p_func: u16, data: &[u8], out: &mut dyn Write) -> Result<u8> {
        self.write_request(p_func, data)?;

        let min_size = self.min_size();
        let response = loop {
            let padding = min_size.saturating_sub(SENSER_HEADER_SIZE);
            let d = self.read_all(SENSER_HEADER_SIZE + padding)?;
            let header = SenserPacketHeader::unpack(&d)?;
            if header.sequence != self.sequence {
                return Err(Error::Protocol("wrong senser sequence"));
            }
            out.write_all(&d[SENSER_HEADER_SIZE..]).map_err(|e| Error::Senser(e.to_string()))?;

            let mut done = padding;
            let data_len = (header.size as usize).min(SENSER_MAX_SIZE);
            let mut chunk_len = SENSER_CHUNK_SIZE - min_size;
            while done < data_len {
                let l = (data_len - done).min(chunk_len);
                let chunk = self.read_all(l)?;
                out.write_all(&chunk).map_err(|e| Error::Senser(e.to_string()))?;
                done += l;
                // A chunk that is a multiple of the transfer granularity is
                // followed by a padding packet which must be drained.
                if l & (SENSER_MIN_SIZE - 1) == 0 {
                    self.backend.read(self.endpoints.ep_in, SENSER_MIN_SIZE)?;
                }
                chunk_len = SENSER_CHUNK_SIZE;
            }

            if (header.size as usize) <= SENSER_MAX_SIZE {
                break header.response;
            }
        };

        self.sequence = self.sequence.wrapping_add(1);
        Ok(response)
    }

    fn write_request(&mut self, p_func: u16, data: &[u8]) -> Result<()> {
        let mut rest = data;
        while !rest.is_empty() {
            let take = rest.len().min(SENSER_MAX_SIZE);
            let header = SenserPacketHeader {
                size: rest.len() as u32,
                p_func,
                sequence: self.sequence,
                ..Default::default()
            };
            let mut frame = Vec::with_capacity(SENSER_HEADER_SIZE + take);
            frame.extend_from_slice(&header.pack());
            frame.extend_from_slice(&rest[..take]);
            self.backend.write(self.endpoints.ep_out, &frame)?;
            rest = &rest[take..];
        }
        Ok(())
    }

    /// Diagnostic: sends a packet and records the reply exactly as the bulk
    /// transfers arrive, without the chunk and padding assumptions of
    /// [`send_packet`](Self::send_packet).
    ///
    /// Reading stops at the first transfer error, normally the timeout once
    /// the camera falls silent, or after `limit` bytes. The camera may be left
    /// mid-reply, so the session should be closed afterwards.
    pub fn trace_packet(&mut self, p_func: u16, data: &[u8], limit: usize) -> Result<WireTrace> {
        self.write_request(p_func, data)?;
        let mut trace = WireTrace::default();
        let mut empty = 0;
        while trace.data.len() < limit && empty < TRACE_EMPTY_LIMIT {
            match self.backend.read(self.endpoints.ep_in, SENSER_CHUNK_SIZE) {
                Ok(chunk) => {
                    empty = if chunk.is_empty() { empty + 1 } else { 0 };
                    trace.transfers.push(chunk.len());
                    trace.data.extend_from_slice(&chunk);
                }
                Err(e) => {
                    trace.end = Some(e.to_string());
                    break;
                }
            }
        }
        self.sequence = self.sequence.wrapping_add(1);
        Ok(trace)
    }

    /// Convenience wrapper around [`send_packet`](Self::send_packet) that
    /// collects the reply in memory.
    pub fn send_packet_buffered(&mut self, p_func: u16, data: &[u8]) -> Result<(u8, Vec<u8>)> {
        let mut out = Vec::new();
        let response = self.send_packet(p_func, data, &mut out)?;
        Ok((response, out))
    }

    /// Reads pending debug-terminal output. Returns an empty vector if the
    /// terminal is absent or nothing is available (transfer errors are
    /// swallowed, matching the reference behaviour of a short poll).
    pub fn read_terminal(&mut self) -> Vec<u8> {
        match self.endpoints.terminal_in {
            Some(ep) => self.backend.read(ep, TERMINAL_CHUNK_SIZE).unwrap_or_default(),
            None => Vec::new(),
        }
    }

    /// Writes `data` to the debug terminal in 0x40-byte chunks.
    pub fn write_terminal(&mut self, data: &[u8]) -> Result<()> {
        let ep = self
            .endpoints
            .terminal_out
            .ok_or(Error::Protocol("device has no terminal endpoint"))?;
        for chunk in data.chunks(TERMINAL_CHUNK_SIZE) {
            self.backend.write(ep, chunk)?;
        }
        Ok(())
    }
}

/// Authentication handshake performed before the senser interface accepts
/// commands.
pub struct SenserAuthDevice<B: UsbBackend> {
    backend: B,
    ep_in: u8,
    ep_out: u8,
    product_id: u16,
}

impl<B: UsbBackend> SenserAuthDevice<B> {
    /// Wraps `backend`; `product_id` selects the challenge variant.
    pub fn new(backend: B, ep_in: u8, ep_out: u8, product_id: u16) -> Self {
        Self { backend, ep_in, ep_out, product_id }
    }

    /// Returns the wrapped backend.
    pub fn into_inner(self) -> B {
        self.backend
    }

    /// Requests that the camera enter senser mode.
    ///
    /// The camera usually re-enumerates while handling this request, so a
    /// transfer error is expected and ignored.
    pub fn start(&mut self) {
        let (request, value, index) = VENDOR_REQUEST_START_SENSER;
        let _ = self.backend.vendor_request_out(request, value, index, &[]);
    }

    /// Requests that the camera leave senser mode. Errors are ignored for the
    /// same reason as in [`start`](Self::start).
    pub fn stop(&mut self) {
        let (request, value, index) = VENDOR_REQUEST_STOP_SENSER;
        let _ = self.backend.vendor_request_out(request, value, index, &[]);
    }

    /// Sends one authentication packet and returns `(ret, payload)`.
    fn send_auth_packet(&mut self, cmd: u16, data: &[u8]) -> Result<(i32, Vec<u8>)> {
        if data.len() > SENSER_KEY_SIZE {
            return Err(Error::Protocol("auth payload exceeds packet size"));
        }
        let mut frame = vec![0u8; AUTH_PACKET_SIZE];
        frame[0..2].copy_from_slice(&(!cmd).to_be_bytes());
        // salt is always zero on the host side; bytes 2..4 stay cleared.
        frame[4..4 + data.len()].copy_from_slice(data);
        self.backend.write(self.ep_out, &frame)?;

        let response = self.read_auth_packet()?;
        let resp_cmd = u16::from_be_bytes([response[0], response[1]]);
        let salt = u16::from_be_bytes([response[2], response[3]]);
        let ret = i32::from(!resp_cmd) - i32::from(salt);
        Ok((ret, response[4..].to_vec()))
    }

    /// Reads exactly one authentication packet.
    fn read_auth_packet(&mut self) -> Result<Vec<u8>> {
        let mut data = Vec::with_capacity(AUTH_PACKET_SIZE);
        while data.len() < AUTH_PACKET_SIZE {
            let chunk = self.backend.read(self.ep_in, AUTH_PACKET_SIZE - data.len())?;
            if chunk.is_empty() {
                return Err(Error::Protocol("auth read returned no data"));
            }
            data.extend_from_slice(&chunk);
        }
        data.truncate(AUTH_PACKET_SIZE);
        Ok(data)
    }

    /// Runs the three-round challenge/response handshake.
    ///
    /// Each round the camera nominates a hash algorithm (`2` = Sony's faulty
    /// SHA-1, `8` = SHA-256); the host answers with the digest of the challenge
    /// combined with the round's key.
    pub fn authenticate(&mut self) -> Result<()> {
        for round in 0..SENSER_ROUNDS {
            let (ret, challenge) = self.send_auth_packet(1, &[])?;
            let use_sha1 = match ret {
                2 => true,
                8 => false,
                other => {
                    return Err(Error::Senser(format!("invalid authentication response {other}")));
                }
            };

            let mut message = Vec::new();
            if self.product_id == PID_SENSER_EXTENDED {
                message.extend_from_slice(&challenge);
                message.extend_from_slice(&senser_key(use_sha1, round)?);
            } else {
                if challenge.len() < 4 {
                    return Err(Error::Protocol("authentication challenge too short"));
                }
                message.extend_from_slice(&challenge[..4]);
            }

            let mut reply = vec![1u8];
            if use_sha1 {
                reply.extend_from_slice(&sha1_faulty(&message));
            } else {
                reply.extend_from_slice(Sha256::digest(&message).as_slice());
            }

            let (ret, _) = self.send_auth_packet(3, &reply)?;
            if ret != 4 {
                return Err(Error::Senser(format!(
                    "authentication rejected in round {round} (response {ret})"
                )));
            }
        }

        let (ret, data) = self.send_auth_packet(5, &[])?;
        if ret != 6 {
            return Err(Error::Senser(format!("invalid authentication response {ret}")));
        }
        match data.first() {
            Some(1) => Ok(()),
            Some(&res) => Err(Error::Senser(format!("senser error {res}"))),
            None => Err(Error::Protocol("empty authentication result")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::MockBackend;

    const EP_IN: u8 = 0x81;
    const EP_OUT: u8 = 0x02;

    fn endpoints() -> SenserEndpoints {
        SenserEndpoints {
            ep_in: EP_IN,
            ep_out: EP_OUT,
            terminal_in: Some(0x83),
            terminal_out: Some(0x04),
        }
    }

    /// Builds a reply frame: header (with `size` payload bytes) padded to
    /// `min_size`, as an old-generation camera would send it.
    fn reply(sequence: u16, response: u8, payload: &[u8], min_size: usize) -> Vec<u8> {
        let header = SenserPacketHeader {
            size: payload.len() as u32,
            p_func: 0,
            sequence,
            response,
            ..Default::default()
        };
        let mut d = header.pack().to_vec();
        d.extend_from_slice(payload);
        d.resize(min_size.max(SENSER_HEADER_SIZE), 0);
        d
    }

    #[test]
    fn header_round_trips() {
        let h = SenserPacketHeader {
            size: 0x1234_5678,
            p_func: 0xff01,
            sequence: 7,
            version: 1,
            micon_type: 2,
            offset_type: 3,
            response: 4,
        };
        assert_eq!(SenserPacketHeader::unpack(&h.pack()).unwrap(), h);
    }

    #[test]
    fn header_rejects_short_input() {
        assert!(SenserPacketHeader::unpack(&[0u8; 11]).is_err());
    }

    #[test]
    fn sends_header_and_payload_and_reads_padded_reply() {
        let mut backend = MockBackend::new();
        // Whole reply fits into the 0x200-byte padding area.
        let mut r = reply(1, 1, b"\xaa\xbb", SENSER_MIN_SIZE);
        r.truncate(SENSER_MIN_SIZE);
        backend.queue_read(EP_IN, r);

        let mut dev = SenserDevice::new(backend, endpoints(), 0x02a9);
        let (res, data) = dev.send_packet_buffered(0x10, b"\x01\x02\x03").unwrap();
        assert_eq!(res, 1);
        // The padded remainder is returned verbatim, as in the reference.
        assert_eq!(data.len(), SENSER_MIN_SIZE - SENSER_HEADER_SIZE);
        assert_eq!(&data[..2], b"\xaa\xbb");

        let backend = dev.into_inner();
        assert_eq!(backend.writes.len(), 1);
        assert_eq!(backend.writes[0].ep, EP_OUT);
        let expected_header =
            SenserPacketHeader { size: 3, p_func: 0x10, sequence: 1, ..Default::default() };
        assert_eq!(&backend.writes[0].data[..SENSER_HEADER_SIZE], &expected_header.pack());
        assert_eq!(&backend.writes[0].data[SENSER_HEADER_SIZE..], b"\x01\x02\x03");
    }

    #[test]
    fn unpadded_model_reads_payload_in_a_second_transfer() {
        // Header announces 5 payload bytes that arrive in a separate transfer.
        let header = SenserPacketHeader { size: 5, sequence: 1, response: 1, ..Default::default() };
        let mut backend = MockBackend::new();
        backend.queue_read(EP_IN, header.pack().to_vec());
        backend.queue_read(EP_IN, b"hello".to_vec());

        let mut dev = SenserDevice::new(backend, endpoints(), PID_SENSER_EXTENDED);
        let (res, data) = dev.send_packet_buffered(0x10, b"x").unwrap();
        assert_eq!(res, 1);
        assert_eq!(data, b"hello");
    }

    #[test]
    fn rejects_mismatched_sequence() {
        let mut backend = MockBackend::new();
        backend.queue_read(EP_IN, reply(99, 0, b"", 0));
        let mut dev = SenserDevice::new(backend, endpoints(), PID_SENSER_EXTENDED);
        assert!(matches!(dev.send_packet_buffered(0x10, b"x"), Err(Error::Protocol(_))));
    }

    #[test]
    fn sequence_increments_between_packets() {
        let mut backend = MockBackend::new();
        backend.queue_read(EP_IN, reply(1, 1, b"", 0));
        backend.queue_read(EP_IN, reply(2, 1, b"", 0));
        let mut dev = SenserDevice::new(backend, endpoints(), PID_SENSER_EXTENDED);
        dev.send_packet_buffered(0x10, b"a").unwrap();
        dev.send_packet_buffered(0x10, b"b").unwrap();
        let backend = dev.into_inner();
        assert_eq!(backend.writes[1].data[6..8], 2u16.to_le_bytes());
    }

    #[test]
    fn a_reply_over_one_mebibyte_arrives_in_packets_with_their_own_headers() {
        // The wire layout a camera used for its 2,390,264-byte
        // backup: a header per MiB, each 32 KiB block closed by a zero-length packet.
        let total = 0x2478f8usize;
        let payload: Vec<u8> = (0..total).map(|i| (i % 251) as u8).collect();
        let mut backend = MockBackend::new();
        for (n, packet) in payload.chunks(SENSER_MAX_SIZE).enumerate() {
            let size = (total - n * SENSER_MAX_SIZE) as u32;
            let header = SenserPacketHeader { size, sequence: 1, ..Default::default() };
            backend.queue_read(EP_IN, header.pack().to_vec());
            for block in packet.chunks(SENSER_CHUNK_SIZE) {
                backend.queue_read(EP_IN, block.to_vec());
                if block.len() % SENSER_MIN_SIZE == 0 {
                    backend.queue_read(EP_IN, Vec::new());
                }
            }
        }

        let mut dev = SenserDevice::new(backend, endpoints(), PID_SENSER_EXTENDED);
        let (_, data) = dev.send_packet_buffered(0x40, b"x").unwrap();
        assert_eq!(data, payload);
    }

    #[test]
    fn a_request_over_one_mebibyte_is_framed_like_the_cameras_own_replies() {
        // A whole backup image plus the 8-byte PDT_WRITE prefix.
        let request = vec![0x5a; 0x2478f8 + 8];
        let mut backend = MockBackend::new();
        backend.queue_read(EP_IN, reply(1, 0, b"", 0));
        let mut dev = SenserDevice::new(backend, endpoints(), PID_SENSER_EXTENDED);
        dev.send_packet_buffered(0x40, &request).unwrap();

        let writes = dev.into_inner().writes;
        let frames: Vec<(u32, usize)> = writes
            .iter()
            .map(|w| (SenserPacketHeader::unpack(&w.data).unwrap().size, w.data.len()))
            .collect();
        assert_eq!(
            frames,
            vec![
                (0x247900, SENSER_HEADER_SIZE + SENSER_MAX_SIZE),
                (0x147900, SENSER_HEADER_SIZE + SENSER_MAX_SIZE),
                (0x47900, SENSER_HEADER_SIZE + 0x47900),
            ]
        );
        let payload: Vec<u8> =
            writes.iter().flat_map(|w| w.data[SENSER_HEADER_SIZE..].to_vec()).collect();
        assert_eq!(payload, request);
    }

    #[test]
    fn trace_records_transfers_until_the_camera_falls_silent() {
        let header = SenserPacketHeader { size: 3, sequence: 1, ..Default::default() };
        let mut backend = MockBackend::new();
        backend.queue_read(EP_IN, header.pack().to_vec());
        backend.queue_read(EP_IN, b"abc".to_vec());
        backend.queue_read(EP_IN, Vec::new());

        let mut dev = SenserDevice::new(backend, endpoints(), PID_SENSER_EXTENDED);
        let trace = dev.trace_packet(0x40, b"x", 1 << 20).unwrap();
        assert_eq!(trace.transfers, vec![SENSER_HEADER_SIZE, 3, 0]);
        assert_eq!(&trace.data[SENSER_HEADER_SIZE..], b"abc");
        // The mock's "no scripted response" stands in for the timeout.
        assert!(trace.end.is_some());
        assert_eq!(dev.sequence, 2);
    }

    #[test]
    fn terminal_writes_are_chunked() {
        let mut dev = SenserDevice::new(MockBackend::new(), endpoints(), PID_SENSER_EXTENDED);
        dev.write_terminal(&[0x41u8; 0x41]).unwrap();
        let backend = dev.into_inner();
        assert_eq!(backend.writes.len(), 2);
        assert_eq!(backend.writes[0].ep, 0x04);
        assert_eq!(backend.writes[0].data.len(), 0x40);
        assert_eq!(backend.writes[1].data.len(), 1);
    }

    /// Builds an authentication reply frame with the given `ret` value.
    fn auth_reply(ret: u16, payload: &[u8]) -> Vec<u8> {
        let mut d = vec![0u8; AUTH_PACKET_SIZE];
        d[0..2].copy_from_slice(&(!ret).to_be_bytes());
        d[4..4 + payload.len()].copy_from_slice(payload);
        d
    }

    #[test]
    fn start_and_stop_issue_vendor_requests() {
        let mut dev = SenserAuthDevice::new(MockBackend::new(), EP_IN, EP_OUT, 0x02a9);
        dev.start();
        dev.stop();
        let backend = dev.into_inner();
        assert_eq!(backend.vendor_requests.len(), 2);
        assert_eq!(
            (
                backend.vendor_requests[0].request,
                backend.vendor_requests[0].value,
                backend.vendor_requests[0].index
            ),
            VENDOR_REQUEST_START_SENSER
        );
        assert_eq!(
            (
                backend.vendor_requests[1].request,
                backend.vendor_requests[1].value,
                backend.vendor_requests[1].index
            ),
            VENDOR_REQUEST_STOP_SENSER
        );
    }

    #[test]
    fn authenticate_answers_sha1_challenges() {
        let challenge = [0x11u8, 0x22, 0x33, 0x44, 0x55];
        let mut backend = MockBackend::new();
        for _ in 0..SENSER_ROUNDS {
            backend.queue_read(EP_IN, auth_reply(2, &challenge));
            backend.queue_read(EP_IN, auth_reply(4, &[]));
        }
        backend.queue_read(EP_IN, auth_reply(6, &[1]));

        let mut dev = SenserAuthDevice::new(backend, EP_IN, EP_OUT, 0x02a9);
        dev.authenticate().unwrap();

        let backend = dev.into_inner();
        // 3 challenges + 3 replies + 1 finish
        assert_eq!(backend.writes.len(), 7);
        // Older models hash only the first four challenge bytes.
        let expected = sha1_faulty(&challenge[..4]);
        assert_eq!(backend.writes[1].data[4], 1);
        assert_eq!(&backend.writes[1].data[5..5 + expected.len()], &expected);
    }

    /// Installs six fabricated service-mode keys and returns them, so a test
    /// can compute the expected response without any real key material.
    fn install_fake_senser_keys() -> [Vec<u8>; 6] {
        let mk = |seed: u8| -> Vec<u8> {
            (0..SENSER_KEY_SIZE).map(|i| (i as u8).wrapping_add(seed)).collect()
        };
        let ids = [
            KeyId::SenserSha1A,
            KeyId::SenserSha1B,
            KeyId::SenserSha1C,
            KeyId::SenserSha256A,
            KeyId::SenserSha256B,
            KeyId::SenserSha256C,
        ];
        let keys: [Vec<u8>; 6] =
            std::array::from_fn(|i| mk(u8::try_from(i).expect("six keys") + 1));
        for (id, key) in ids.into_iter().zip(keys.iter()) {
            keystore::install_unchecked(id, key.clone());
        }
        keys
    }

    #[test]
    fn authenticate_answers_sha256_challenges_with_keys() {
        let _g = keystore::test_guard();
        let keys = install_fake_senser_keys();
        let challenge = vec![0x5au8; SENSER_KEY_SIZE];
        let mut backend = MockBackend::new();
        for _ in 0..SENSER_ROUNDS {
            backend.queue_read(EP_IN, auth_reply(8, &challenge));
            backend.queue_read(EP_IN, auth_reply(4, &[]));
        }
        backend.queue_read(EP_IN, auth_reply(6, &[1]));

        let mut dev = SenserAuthDevice::new(backend, EP_IN, EP_OUT, PID_SENSER_EXTENDED);
        dev.authenticate().unwrap();

        let backend = dev.into_inner();
        let mut message = challenge.clone();
        // Round 0 uses the first SHA-256 key.
        message.extend_from_slice(&keys[3]);
        let expected = Sha256::digest(&message);
        assert_eq!(&backend.writes[1].data[5..5 + expected.len()], &expected[..]);
        keystore::clear();
    }

    #[test]
    fn authenticate_without_keys_is_refused() {
        let _g = keystore::test_guard();
        keystore::clear();
        let challenge = vec![0x5au8; SENSER_KEY_SIZE];
        let mut backend = MockBackend::new();
        backend.queue_read(EP_IN, auth_reply(8, &challenge));
        let mut dev = SenserAuthDevice::new(backend, EP_IN, EP_OUT, PID_SENSER_EXTENDED);
        assert!(matches!(dev.authenticate(), Err(Error::MissingKeys)));
    }

    #[test]
    fn authenticate_rejects_unknown_algorithm() {
        let mut backend = MockBackend::new();
        backend.queue_read(EP_IN, auth_reply(3, &[0; 8]));
        let mut dev = SenserAuthDevice::new(backend, EP_IN, EP_OUT, 0x02a9);
        assert!(matches!(dev.authenticate(), Err(Error::Senser(_))));
    }

    #[test]
    fn authenticate_reports_final_error_code() {
        let challenge = [0u8; 8];
        let mut backend = MockBackend::new();
        for _ in 0..SENSER_ROUNDS {
            backend.queue_read(EP_IN, auth_reply(2, &challenge));
            backend.queue_read(EP_IN, auth_reply(4, &[]));
        }
        backend.queue_read(EP_IN, auth_reply(6, &[7]));

        let mut dev = SenserAuthDevice::new(backend, EP_IN, EP_OUT, 0x02a9);
        let err = dev.authenticate().unwrap_err();
        assert!(matches!(err, Error::Senser(ref m) if m.contains('7')));
    }
}
