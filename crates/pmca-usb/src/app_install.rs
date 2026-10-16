//! Sony application-installation proxy over MTP.
//!
//! Byte-faithful port of `SonyMtpAppInstallDevice`, `SonyAppInstallCamera`
//! (`pmca/usb/sony.py`) and the relay loop in `pmca/installer/__init__.py`.
//!
//! When a Sony camera is switched into application-installation mode it exposes
//! an MTP interface that tunnels a small message protocol through four
//! vendor-specific PTP operations. On top of that transport the camera speaks a
//! request/response protocol which lets the host:
//!
//! * negotiate protocols ([`AppInstallCamera::send_init`]),
//! * push a REST request that starts the installation
//!   ([`AppInstallCamera::send_request`]), and
//! * proxy the camera's TLS traffic to the local install server (the camera
//!   opens an "SSL socket", the host relays raw bytes to/from a TCP socket).
//!
//! The wire layout is split across two endianness conventions: the outer
//! `InfoMsgHeader`/message-type framing is little-endian, while every
//! [`AppInstallCamera`] sub-header is big-endian. This mirrors the reference
//! exactly.

use crate::mtp::{MtpDevice, PtpCommands};
use crate::{Error, Result, UsbBackend};

// --- Proxy transport (SonyMtpAppInstallDevice) -----------------------------

/// PTP operation code: read the info header describing a pending message.
const OC_GET_PROXY_MESSAGE_INFO: u16 = 0x9488;
/// PTP operation code: read a pending message body.
const OC_GET_PROXY_MESSAGE: u16 = 0x9489;
/// PTP operation code: write the info header describing an outgoing message.
const OC_SEND_PROXY_MESSAGE_INFO: u16 = 0x948c;
/// PTP operation code: write an outgoing message body.
const OC_SEND_PROXY_MESSAGE: u16 = 0x948d;

/// PTP response code: no message pending (accepted on `GetProxyMessage`).
const RC_NO_DATA: u16 = 0xa488;
/// PTP response code: the device is busy, retry the operation.
const RC_SONY_DEVICE_BUSY: u16 = 0xa489;

/// Magic constant carried in every [`InfoMsgHeader`].
const INFO_MSG_HEADER_MAGIC: u16 = 0xb481;
/// Serialized size of an [`InfoMsgHeader`].
const INFO_MSG_HEADER_SIZE: usize = 56;
/// Serialized size of the message-type prefix (`MsgHeader`).
const MSG_HEADER_SIZE: usize = 2;

/// Retry ceiling for the `SonyDeviceBusy` write loops (matches the reference's
/// effectively-unbounded spin, but bounded so a wedged device cannot hang).
const MAX_BUSY_RETRIES: usize = 100_000;

/// The 56-byte little-endian info header prefixing every proxy transfer.
struct InfoMsgHeader {
    magic: u16,
    data_size: u32,
}

impl InfoMsgHeader {
    /// Serializes an outgoing header (all reserved fields zero, as on write).
    fn pack(data_size: usize) -> Vec<u8> {
        let mut out = vec![0u8; INFO_MSG_HEADER_SIZE];
        out[4..6].copy_from_slice(&INFO_MSG_HEADER_MAGIC.to_le_bytes());
        out[8..12].copy_from_slice(&(data_size as u32).to_le_bytes());
        out
    }

    /// Parses an info header read from the camera.
    fn unpack(data: &[u8]) -> Result<Self> {
        if data.len() < INFO_MSG_HEADER_SIZE {
            return Err(Error::Protocol("short app-install info header"));
        }
        Ok(Self {
            magic: u16::from_le_bytes([data[4], data[5]]),
            data_size: u32::from_le_bytes([data[8], data[9], data[10], data[11]]),
        })
    }
}

/// A device speaking the application-install proxy message protocol.
///
/// This is the transport [`AppInstallCamera`] is built on; it is a trait so the
/// message layer and the relay loop can be exercised without hardware.
pub trait AppInstallDevice {
    /// Sends a typed message to the camera.
    fn send_message(&mut self, msg_type: u16, data: &[u8]) -> Result<()>;

    /// Receives the next message, or `None` when the camera has none pending.
    fn receive_message(&mut self) -> Result<Option<(u16, Vec<u8>)>>;
}

/// Writes a raw proxy payload (info header + body, each with a busy retry).
fn proxy_write<D: PtpCommands + ?Sized>(dev: &mut D, data: &[u8]) -> Result<()> {
    let info = InfoMsgHeader::pack(data.len());
    proxy_send_with_retry(dev, OC_SEND_PROXY_MESSAGE_INFO, &info)?;
    proxy_send_with_retry(dev, OC_SEND_PROXY_MESSAGE, data)?;
    Ok(())
}

/// Runs one `Send*` operation, retrying while the device reports busy.
fn proxy_send_with_retry<D: PtpCommands + ?Sized>(
    dev: &mut D,
    code: u16,
    data: &[u8],
) -> Result<()> {
    for _ in 0..MAX_BUSY_RETRIES {
        let rc = dev.send_write_command(code, &[], data)?;
        if rc == RC_SONY_DEVICE_BUSY {
            continue;
        }
        if rc == crate::mtp::ptp::RC_OK {
            return Ok(());
        }
        return Err(Error::Protocol("app-install proxy write rejected"));
    }
    Err(Error::Protocol("app-install proxy device busy timeout"))
}

/// Reads a raw proxy payload, or an empty vector when nothing is pending.
fn proxy_read<D: PtpCommands + ?Sized>(dev: &mut D) -> Result<Vec<u8>> {
    let (rc, info_data) = dev.send_read_command(OC_GET_PROXY_MESSAGE_INFO, &[0])?;
    if rc != crate::mtp::ptp::RC_OK {
        return Err(Error::Protocol("app-install GetProxyMessageInfo failed"));
    }
    let info = InfoMsgHeader::unpack(&info_data)?;
    if info.magic != INFO_MSG_HEADER_MAGIC {
        return Err(Error::Protocol("app-install info header magic mismatch"));
    }

    let (rc, data) = dev.send_read_command_opt(OC_GET_PROXY_MESSAGE, &[0])?;
    if rc != crate::mtp::ptp::RC_OK && rc != RC_NO_DATA {
        return Err(Error::Protocol("app-install GetProxyMessage failed"));
    }
    let end = (info.data_size as usize).min(data.len());
    Ok(data[..end].to_vec())
}

impl<B: UsbBackend> AppInstallDevice for MtpDevice<B> {
    fn send_message(&mut self, msg_type: u16, data: &[u8]) -> Result<()> {
        proxy_send_message(self, msg_type, data)
    }

    fn receive_message(&mut self) -> Result<Option<(u16, Vec<u8>)>> {
        proxy_receive_message(self)
    }
}

/// Forwarding impl so a type-erased transport (bulk MTP or Windows WPD, decided
/// at open time) can drive [`AppInstallCamera`] uniformly.
impl AppInstallDevice for Box<dyn AppInstallDevice> {
    fn send_message(&mut self, msg_type: u16, data: &[u8]) -> Result<()> {
        (**self).send_message(msg_type, data)
    }

    fn receive_message(&mut self) -> Result<Option<(u16, Vec<u8>)>> {
        (**self).receive_message()
    }
}

/// Frames and writes one proxy message over any PTP transport.
pub(crate) fn proxy_send_message<D: PtpCommands + ?Sized>(
    dev: &mut D,
    msg_type: u16,
    data: &[u8],
) -> Result<()> {
    let mut payload = Vec::with_capacity(MSG_HEADER_SIZE + data.len());
    payload.extend_from_slice(&msg_type.to_be_bytes());
    payload.extend_from_slice(data);
    proxy_write(dev, &payload)
}

/// Reads and unframes one proxy message over any PTP transport.
pub(crate) fn proxy_receive_message<D: PtpCommands + ?Sized>(
    dev: &mut D,
) -> Result<Option<(u16, Vec<u8>)>> {
    let data = proxy_read(dev)?;
    if data.is_empty() {
        return Ok(None);
    }
    if data.len() < MSG_HEADER_SIZE {
        return Err(Error::Protocol("short app-install message"));
    }
    let msg_type = u16::from_be_bytes([data[0], data[1]]);
    Ok(Some((msg_type, data[MSG_HEADER_SIZE..].to_vec())))
}

// --- Message layer (SonyAppInstallCamera) ----------------------------------

/// Outer message types (the [`AppInstallDevice`] message type field).
const MSG_COMMON: u16 = 0;
const MSG_TCP: u16 = 1;
const MSG_REST: u16 = 2;

/// `Common` sub-types.
const COMMON_START: u32 = 0x400;
const COMMON_HELLO: u32 = 0x401;
const COMMON_BYE: u32 = 0x402;

/// `Tcp` sub-types.
const TCP_PROXY_CONNECT: u32 = 0x501;
const TCP_PROXY_DISCONNECT: u32 = 0x502;
const TCP_PROXY_DATA: u32 = 0x503;
const TCP_PROXY_END: u32 = 0x504;

/// `Rest` sub-types.
const REST_IN: u16 = 0;
const REST_OUT: u16 = 2;

/// Version carried in every `CommonMsgHeader`.
const COMMON_MSG_VERSION: u16 = 1;
/// Serialized size of a `CommonMsgHeader`.
const COMMON_MSG_HEADER_SIZE: usize = 16;
/// Serialized size of a `TcpMsgHeader`.
const TCP_MSG_HEADER_SIZE: usize = 4;
/// Serialized size of a `RestMsgHeader`.
const REST_MSG_HEADER_SIZE: usize = 4;
/// Serialized size of a `ProxyConnectMsgHeader`.
const PROXY_CONNECT_HEADER_SIZE: usize = 6;
/// Serialized size of an `SslDataMsgHeader`.
const SSL_DATA_HEADER_SIZE: usize = 4;
/// Serialized size of a `ProtocolMsgHeader`.
const PROTOCOL_HEADER_SIZE: usize = 4;
/// Serialized size of a `ProtocolMsgProto` entry.
const PROTOCOL_PROTO_SIZE: usize = 6;

/// Protocols advertised by [`AppInstallCamera::send_init`], matching the
/// reference `ProtocolMsgProtos`.
const DEFAULT_PROTOCOLS: &[(&[u8; 4], u16)] = &[(b"TCPT", 0x01), (b"REST", 0x100)];

/// A parsed message from the camera.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProxyMessage {
    /// Response to an init: the protocols the camera accepted.
    InitResponse {
        /// The `(name, id)` pairs the camera kept.
        protocols: Vec<([u8; 4], u16)>,
    },
    /// The camera wants the host to open an SSL socket (TLS to the server).
    SslStart {
        /// Opaque socket identifier the camera assigns to this connection.
        connection_id: i32,
        /// Host the camera intends to reach (ignored; always relayed locally).
        host: String,
        /// Port the camera intends to reach.
        port: u16,
    },
    /// The camera produced raw SSL bytes to send over that socket.
    SslSendData {
        /// The connection the bytes belong to.
        connection_id: i32,
        /// The raw (encrypted) bytes to forward to the server.
        data: Vec<u8>,
    },
    /// The camera wants the SSL socket closed.
    SslEnd {
        /// The connection to close.
        connection_id: i32,
    },
    /// A REST request from the camera (installation progress / completion).
    Request {
        /// The raw request bytes.
        data: Vec<u8>,
    },
    /// A REST response from the camera (reply to [`AppInstallCamera::send_request`]).
    Response {
        /// The raw response bytes.
        data: Vec<u8>,
    },
}

/// Big-endian read helpers with bounds checking.
fn be_u16(data: &[u8], offset: usize) -> Result<u16> {
    data.get(offset..offset + 2)
        .map(|b| u16::from_be_bytes([b[0], b[1]]))
        .ok_or(Error::Protocol("app-install message truncated"))
}

fn be_u32(data: &[u8], offset: usize) -> Result<u32> {
    data.get(offset..offset + 4)
        .map(|b| u32::from_be_bytes([b[0], b[1], b[2], b[3]]))
        .ok_or(Error::Protocol("app-install message truncated"))
}

fn be_i32(data: &[u8], offset: usize) -> Result<i32> {
    be_u32(data, offset).map(|v| v as i32)
}

/// A camera in application-installation mode.
///
/// Wraps an [`AppInstallDevice`] transport and speaks the request/response
/// protocol the camera expects during installation.
pub struct AppInstallCamera<D: AppInstallDevice> {
    dev: D,
}

impl<D: AppInstallDevice> AppInstallCamera<D> {
    /// Wraps a proxy transport.
    pub fn new(dev: D) -> Self {
        Self { dev }
    }

    /// Consumes the camera, returning the underlying transport.
    pub fn into_inner(self) -> D {
        self.dev
    }

    /// Receives and parses the next message, or `None` when none is pending.
    pub fn receive(&mut self) -> Result<Option<ProxyMessage>> {
        let Some((msg_type, data)) = self.dev.receive_message()? else {
            return Ok(None);
        };
        match msg_type {
            MSG_COMMON => self.parse_common(&data).map(Some),
            MSG_TCP => self.parse_tcp(&data).map(Some),
            MSG_REST => self.parse_rest(&data).map(Some),
            _ => Err(Error::Protocol("unknown app-install message type")),
        }
    }

    fn parse_common(&self, data: &[u8]) -> Result<ProxyMessage> {
        let (sub_type, body) = split_common(data)?;
        match sub_type {
            COMMON_HELLO => {
                let n = be_u32(body, 0)? as usize;
                let mut protocols = Vec::with_capacity(n);
                for i in 0..n {
                    let off = PROTOCOL_HEADER_SIZE + i * PROTOCOL_PROTO_SIZE;
                    let name = body
                        .get(off..off + 4)
                        .ok_or(Error::Protocol("app-install protocol truncated"))?;
                    let id = be_u16(body, off + 4)?;
                    protocols.push(([name[0], name[1], name[2], name[3]], id));
                }
                Ok(ProxyMessage::InitResponse { protocols })
            }
            COMMON_BYE => Err(Error::Protocol("app-install: bye from camera")),
            _ => Err(Error::Protocol("unknown app-install common message")),
        }
    }

    fn parse_tcp(&self, data: &[u8]) -> Result<ProxyMessage> {
        let (sub_type, body) = split_common(data)?;
        let connection_id = be_i32(body, 0)?;
        let rest = &body[TCP_MSG_HEADER_SIZE..];
        match sub_type {
            TCP_PROXY_CONNECT => {
                let port = be_u16(rest, 0)?;
                let host_size = be_u32(rest, 2)? as usize;
                let host = rest
                    .get(PROXY_CONNECT_HEADER_SIZE..PROXY_CONNECT_HEADER_SIZE + host_size)
                    .ok_or(Error::Protocol("app-install host truncated"))?;
                let host = host.iter().map(|&b| b as char).collect::<String>();
                Ok(ProxyMessage::SslStart { connection_id, host, port })
            }
            TCP_PROXY_DISCONNECT => Ok(ProxyMessage::SslEnd { connection_id }),
            TCP_PROXY_DATA => {
                let size = be_u32(rest, 0)? as usize;
                let payload = rest
                    .get(SSL_DATA_HEADER_SIZE..SSL_DATA_HEADER_SIZE + size)
                    .ok_or(Error::Protocol("app-install ssl data truncated"))?;
                Ok(ProxyMessage::SslSendData { connection_id, data: payload.to_vec() })
            }
            _ => Err(Error::Protocol("unknown app-install tcp message")),
        }
    }

    fn parse_rest(&self, data: &[u8]) -> Result<ProxyMessage> {
        let sub_type = be_u16(data, 0)?;
        let size = be_u16(data, 2)? as usize;
        let body = data
            .get(REST_MSG_HEADER_SIZE..REST_MSG_HEADER_SIZE + size)
            .ok_or(Error::Protocol("app-install rest body truncated"))?
            .to_vec();
        match sub_type {
            REST_OUT => Ok(ProxyMessage::Response { data: body }),
            REST_IN => Ok(ProxyMessage::Request { data: body }),
            _ => Err(Error::Protocol("unknown app-install rest message")),
        }
    }

    /// Receives until a message of the expected variant arrives.
    fn receive_response(&mut self, want: fn(&ProxyMessage) -> bool) -> Result<ProxyMessage> {
        loop {
            if let Some(msg) = self.receive()? {
                if want(&msg) {
                    return Ok(msg);
                }
                return Err(Error::Protocol("unexpected app-install response"));
            }
        }
    }

    fn send_common_message(&mut self, sub_type: u32, data: &[u8]) -> Result<()> {
        self.send_common_typed(MSG_COMMON, sub_type, data)
    }

    fn send_common_typed(&mut self, msg_type: u16, sub_type: u32, data: &[u8]) -> Result<()> {
        let mut payload = Vec::with_capacity(COMMON_MSG_HEADER_SIZE + data.len());
        payload.extend_from_slice(&COMMON_MSG_VERSION.to_be_bytes());
        payload.extend_from_slice(&sub_type.to_be_bytes());
        payload.extend_from_slice(&((COMMON_MSG_HEADER_SIZE + data.len()) as u32).to_be_bytes());
        payload.extend_from_slice(&[0u8; 6]);
        payload.extend_from_slice(data);
        self.dev.send_message(msg_type, &payload)
    }

    fn send_tcp_message(&mut self, sub_type: u32, socket_fd: i32, data: &[u8]) -> Result<()> {
        let mut body = Vec::with_capacity(TCP_MSG_HEADER_SIZE + data.len());
        body.extend_from_slice(&socket_fd.to_be_bytes());
        body.extend_from_slice(data);
        self.send_common_typed(MSG_TCP, sub_type, &body)
    }

    fn send_rest_message(&mut self, sub_type: u16, data: &[u8]) -> Result<()> {
        let mut payload = Vec::with_capacity(REST_MSG_HEADER_SIZE + data.len());
        payload.extend_from_slice(&sub_type.to_be_bytes());
        payload.extend_from_slice(&(data.len() as u16).to_be_bytes());
        payload.extend_from_slice(data);
        self.dev.send_message(MSG_REST, &payload)
    }

    /// Receives and discards all currently pending messages.
    pub fn empty_buffer(&mut self) -> Result<()> {
        while self.receive()?.is_some() {}
        Ok(())
    }

    /// Sends an initialization message, returning the protocols the camera kept.
    pub fn send_init(&mut self) -> Result<Vec<([u8; 4], u16)>> {
        let mut data = Vec::with_capacity(
            PROTOCOL_HEADER_SIZE + DEFAULT_PROTOCOLS.len() * PROTOCOL_PROTO_SIZE,
        );
        data.extend_from_slice(&(DEFAULT_PROTOCOLS.len() as u32).to_be_bytes());
        for (name, id) in DEFAULT_PROTOCOLS {
            data.extend_from_slice(*name);
            data.extend_from_slice(&id.to_be_bytes());
        }
        self.send_common_message(COMMON_START, &data)?;
        match self.receive_response(|m| matches!(m, ProxyMessage::InitResponse { .. }))? {
            ProxyMessage::InitResponse { protocols } => Ok(protocols),
            _ => unreachable!("receive_response guarantees the variant"),
        }
    }

    /// Sends a REST request, returning the camera's response body.
    pub fn send_request(&mut self, data: &[u8]) -> Result<Vec<u8>> {
        self.send_rest_message(REST_OUT, data)?;
        match self.receive_response(|m| matches!(m, ProxyMessage::Response { .. }))? {
            ProxyMessage::Response { data } => Ok(data),
            _ => unreachable!("receive_response guarantees the variant"),
        }
    }

    /// Sends raw SSL bytes (received from the local server) to the camera.
    pub fn send_ssl_data(&mut self, connection_id: i32, data: &[u8]) -> Result<()> {
        let mut body = Vec::with_capacity(SSL_DATA_HEADER_SIZE + data.len());
        body.extend_from_slice(&(data.len() as u32).to_be_bytes());
        body.extend_from_slice(data);
        self.send_tcp_message(TCP_PROXY_DATA, connection_id, &body)
    }

    /// Tells the camera the SSL socket has been closed by the peer.
    pub fn send_ssl_end(&mut self, connection_id: i32) -> Result<()> {
        self.send_tcp_message(TCP_PROXY_END, connection_id, &three_value_msg(1, 1, 0))
    }

    /// Ends the communication with the camera.
    pub fn send_end(&mut self) -> Result<()> {
        self.send_common_message(COMMON_BYE, &three_value_msg(0, 0, 0))
    }
}

/// Splits a `Common`/`Tcp` payload into its sub-type and the body bounded by
/// the header's declared size.
fn split_common(data: &[u8]) -> Result<(u32, &[u8])> {
    let sub_type = be_u32(data, 2)?;
    let size = be_u32(data, 6)? as usize;
    let body = data
        .get(COMMON_MSG_HEADER_SIZE..size)
        .ok_or(Error::Protocol("app-install common body truncated"))?;
    Ok((sub_type, body))
}

/// Serializes a `ThreeValueMsg` (big-endian INT16, INT32, INT32).
fn three_value_msg(a: u16, b: u32, c: u32) -> Vec<u8> {
    let mut out = Vec::with_capacity(10);
    out.extend_from_slice(&a.to_be_bytes());
    out.extend_from_slice(&b.to_be_bytes());
    out.extend_from_slice(&c.to_be_bytes());
    out
}

// --- Installation relay loop -----------------------------------------------

/// A non-blocking byte stream to the local install server.
///
/// The relay reads and writes raw (already-encrypted) TLS bytes; TLS is
/// terminated between the camera and the server, so this is a plain byte pipe.
pub trait ProxySocket {
    /// Sends raw bytes to the server.
    fn send(&mut self, data: &[u8]) -> Result<()>;

    /// Attempts to read pending bytes without blocking.
    ///
    /// Returns `Ok(None)` when no data is currently available, `Ok(Some(v))`
    /// with `v` non-empty when bytes were read, and `Ok(Some(vec![]))` when the
    /// peer has closed the connection.
    fn try_recv(&mut self) -> Result<Option<Vec<u8>>>;
}

/// Opens [`ProxySocket`]s to a host/port on demand.
pub trait SocketConnector {
    /// Connects to the given host and port.
    fn connect(&mut self, host: &str, port: u16) -> Result<Box<dyn ProxySocket>>;
}

/// Builds a REST request understood by the camera's install endpoint.
pub fn build_rest_request(endpoint: &str, content_type: &str, data: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(b"POST ");
    out.extend_from_slice(endpoint.as_bytes());
    out.extend_from_slice(b" REST/1.0\r\nContent-type: ");
    out.extend_from_slice(content_type.as_bytes());
    out.extend_from_slice(b"\r\n\r\n");
    out.extend_from_slice(data);
    out
}

/// A minimal parsed HTTP message (request or response).
struct Http {
    first_line: String,
    body: Vec<u8>,
}

/// Splits an HTTP message into its first line and body (headers are ignored,
/// as the reference does for the fields it needs).
///
/// The body ends at the *next* `\r\n\r\n`, not at the end of the buffer: the
/// camera terminates its REST payloads with a second blank line and a NUL, and
/// those trailing bytes would otherwise be handed to the JSON parser. This
/// mirrors the reference's `data.split(b'\r\n\r\n')[:2]`, which keeps only the
/// segment between the first and second separator.
fn parse_http(data: &[u8]) -> Result<Http> {
    const SEP: &[u8] = b"\r\n\r\n";
    let sep = data
        .windows(SEP.len())
        .position(|w| w == SEP)
        .ok_or(Error::Protocol("app-install: malformed HTTP message"))?;
    let head = &data[..sep];
    let rest = &data[sep + SEP.len()..];
    let end = rest.windows(SEP.len()).position(|w| w == SEP).unwrap_or(rest.len());
    let body = rest[..end].to_vec();
    let first_line = head
        .split(|&b| b == b'\r' || b == b'\n')
        .next()
        .unwrap_or(&[])
        .iter()
        .map(|&b| b as char)
        .collect::<String>();
    Ok(Http { first_line, body })
}

/// The URL of a parsed REST request (the second whitespace-separated token).
fn request_url(data: &[u8]) -> Result<(String, Vec<u8>)> {
    let http = parse_http(data)?;
    let url = http
        .first_line
        .split(' ')
        .nth(1)
        .ok_or(Error::Protocol("app-install: malformed REST request"))?
        .to_string();
    Ok((url, http.body))
}

/// The body of a parsed REST response.
fn response_body(data: &[u8]) -> Result<Vec<u8>> {
    Ok(parse_http(data)?.body)
}

/// Callbacks the [`install`] loop invokes with raw (JSON) payloads, so the
/// caller can interpret them without this crate depending on a JSON library.
pub trait InstallObserver {
    /// Validates the `/task/start` response body; return `Err` to abort.
    fn on_start(&mut self, body: &[u8]) -> Result<()>;

    /// Reports a `/task/progress` body.
    fn on_progress(&mut self, body: &[u8]);
}

/// Drives a full application installation.
///
/// Sends `xpd_data` to the camera, relays its TLS traffic to the local install
/// server (reached through `connector` at `host:port`), and returns the raw
/// `/task/complete` body once the camera finishes.
pub fn install<D: AppInstallDevice>(
    camera: &mut AppInstallCamera<D>,
    connector: &mut dyn SocketConnector,
    host: &str,
    port: u16,
    xpd_data: &[u8],
    content_type: &str,
    observer: &mut dyn InstallObserver,
) -> Result<Vec<u8>> {
    camera.empty_buffer()?;
    camera.send_init()?;

    let response =
        camera.send_request(&build_rest_request("/task/start", content_type, xpd_data))?;
    observer.on_start(&response_body(&response)?)?;

    let mut connection_id: i32 = 0;
    let mut sock: Option<Box<dyn ProxySocket>> = None;

    loop {
        // Relay any server -> camera bytes waiting on the socket.
        if let Some(s) = sock.as_mut()
            && let Some(bytes) = s.try_recv()?
        {
            if bytes.is_empty() {
                camera.send_ssl_end(connection_id)?;
            } else {
                camera.send_ssl_data(connection_id, &bytes)?;
            }
        }

        let Some(message) = camera.receive()? else {
            continue;
        };

        match message {
            ProxyMessage::SslStart { connection_id: id, host: _host, port: _port } => {
                // The camera dictates the target; we always relay to our local
                // install server, matching the reference (host/port ignored).
                connection_id = id;
                sock = Some(connector.connect(host, port)?);
            }
            ProxyMessage::SslSendData { connection_id: id, data } if id == connection_id => {
                if let Some(s) = sock.as_mut() {
                    s.send(&data)?;
                }
            }
            ProxyMessage::SslEnd { connection_id: id } if id == connection_id => {
                sock = None;
            }
            ProxyMessage::Request { data } => {
                let (url, body) = request_url(&data)?;
                match url.as_str() {
                    "/task/progress" => observer.on_progress(&body),
                    "/task/complete" => {
                        camera.send_end()?;
                        return Ok(body);
                    }
                    _ => return Err(Error::Protocol("app-install: unknown request url")),
                }
            }
            _ => return Err(Error::Protocol("app-install: unexpected message")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    /// A scripted [`AppInstallDevice`]: records sent messages and replays a
    /// queue of incoming ones.
    ///
    /// A queued `None` models "the buffer is currently empty": [`receive`] then
    /// yields `None` (consuming the marker) instead of the next real message.
    /// This lets [`AppInstallCamera::empty_buffer`] stop where a real camera
    /// would, rather than draining scripted replies meant for later steps.
    #[derive(Default)]
    struct MockDevice {
        sent: Vec<(u16, Vec<u8>)>,
        incoming: VecDeque<Option<(u16, Vec<u8>)>>,
    }

    impl MockDevice {
        /// Queues a message to be received.
        fn push(&mut self, msg: (u16, Vec<u8>)) {
            self.incoming.push_back(Some(msg));
        }

        /// Queues an "empty buffer" marker (one `receive` returns `None`).
        fn push_empty(&mut self) {
            self.incoming.push_back(None);
        }
    }

    impl AppInstallDevice for MockDevice {
        fn send_message(&mut self, msg_type: u16, data: &[u8]) -> Result<()> {
            self.sent.push((msg_type, data.to_vec()));
            Ok(())
        }

        fn receive_message(&mut self) -> Result<Option<(u16, Vec<u8>)>> {
            // `pop_front` yields `None` when the queue is empty; a queued
            // `Some(None)` is the explicit "empty now" marker.
            Ok(self.incoming.pop_front().flatten())
        }
    }

    /// Builds a raw `Common`/`Tcp` message body (version, type, size, padding).
    fn common_body(sub_type: u32, inner: &[u8]) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(&COMMON_MSG_VERSION.to_be_bytes());
        out.extend_from_slice(&sub_type.to_be_bytes());
        out.extend_from_slice(&((COMMON_MSG_HEADER_SIZE + inner.len()) as u32).to_be_bytes());
        out.extend_from_slice(&[0u8; 6]);
        out.extend_from_slice(inner);
        out
    }

    #[test]
    fn info_header_round_trips() {
        let packed = InfoMsgHeader::pack(0x1234);
        assert_eq!(packed.len(), INFO_MSG_HEADER_SIZE);
        let parsed = InfoMsgHeader::unpack(&packed).unwrap();
        assert_eq!(parsed.magic, INFO_MSG_HEADER_MAGIC);
        assert_eq!(parsed.data_size, 0x1234);
    }

    #[test]
    fn parses_init_response() {
        let mut inner = Vec::new();
        inner.extend_from_slice(&2u32.to_be_bytes());
        inner.extend_from_slice(b"TCPT");
        inner.extend_from_slice(&0x01u16.to_be_bytes());
        inner.extend_from_slice(b"REST");
        inner.extend_from_slice(&0x100u16.to_be_bytes());

        let mut dev = MockDevice::default();
        dev.push((MSG_COMMON, common_body(COMMON_HELLO, &inner)));
        let mut cam = AppInstallCamera::new(dev);

        let msg = cam.receive().unwrap().unwrap();
        assert_eq!(
            msg,
            ProxyMessage::InitResponse { protocols: vec![(*b"TCPT", 0x01), (*b"REST", 0x100)] }
        );
    }

    #[test]
    fn parses_ssl_start_data_and_end() {
        let mut connect = Vec::new();
        connect.extend_from_slice(&42i32.to_be_bytes()); // socketFd
        connect.extend_from_slice(&443u16.to_be_bytes()); // port
        connect.extend_from_slice(&(4u32).to_be_bytes()); // hostSize
        connect.extend_from_slice(b"host");

        let mut data = Vec::new();
        data.extend_from_slice(&42i32.to_be_bytes());
        data.extend_from_slice(&(3u32).to_be_bytes());
        data.extend_from_slice(b"abc");

        let mut dev = MockDevice::default();
        dev.push((MSG_TCP, common_body(TCP_PROXY_CONNECT, &connect)));
        dev.push((MSG_TCP, common_body(TCP_PROXY_DATA, &data)));
        dev.push((MSG_TCP, common_body(TCP_PROXY_DISCONNECT, &42i32.to_be_bytes())));
        let mut cam = AppInstallCamera::new(dev);

        assert_eq!(
            cam.receive().unwrap().unwrap(),
            ProxyMessage::SslStart { connection_id: 42, host: "host".into(), port: 443 }
        );
        assert_eq!(
            cam.receive().unwrap().unwrap(),
            ProxyMessage::SslSendData { connection_id: 42, data: b"abc".to_vec() }
        );
        assert_eq!(cam.receive().unwrap().unwrap(), ProxyMessage::SslEnd { connection_id: 42 });
    }

    #[test]
    fn send_init_frames_protocols() {
        let mut dev = MockDevice::default();
        // Queue the Hello reply so send_init's wait resolves.
        let mut inner = Vec::new();
        inner.extend_from_slice(&1u32.to_be_bytes());
        inner.extend_from_slice(b"TCPT");
        inner.extend_from_slice(&0x01u16.to_be_bytes());
        dev.push((MSG_COMMON, common_body(COMMON_HELLO, &inner)));
        let mut cam = AppInstallCamera::new(dev);

        let protos = cam.send_init().unwrap();
        assert_eq!(protos, vec![(*b"TCPT", 0x01)]);

        let dev = cam.into_inner();
        assert_eq!(dev.sent.len(), 1);
        let (msg_type, payload) = &dev.sent[0];
        assert_eq!(*msg_type, MSG_COMMON);
        // version, COMMON_START, size, padding, then the protocol list.
        assert_eq!(u32::from_be_bytes(payload[2..6].try_into().unwrap()), COMMON_START);
    }

    #[test]
    fn build_rest_request_matches_reference() {
        let req = build_rest_request("/task/start", "application/x-psn-dstartup2", b"DATA");
        assert_eq!(
            req,
            b"POST /task/start REST/1.0\r\nContent-type: application/x-psn-dstartup2\r\n\r\nDATA"
        );
    }

    #[test]
    fn http_body_stops_at_the_cameras_trailing_separator() {
        // What a camera actually answers to `/task/start`: the JSON is
        // followed by a second blank line and a NUL. Including those made the
        // JSON parser reject an otherwise perfectly good response.
        let raw = b"REST/1.0 200 OK\r\n\r\n{\"resultCode\":0}\r\n\r\n\0";
        assert_eq!(parse_http(raw).unwrap().body, b"{\"resultCode\":0}");
    }

    #[test]
    fn http_body_without_a_trailing_separator_is_kept_whole() {
        let raw = b"REST/1.0 200 OK\r\n\r\n{\"resultCode\":0}";
        assert_eq!(parse_http(raw).unwrap().body, b"{\"resultCode\":0}");
    }

    #[test]
    fn http_first_line_survives_the_body_split() {
        let raw = b"POST /task/complete REST/1.0\r\nContent-type: x\r\n\r\nBODY\r\n\r\n";
        let http = parse_http(raw).unwrap();
        assert_eq!(http.first_line, "POST /task/complete REST/1.0");
        assert_eq!(http.body, b"BODY");
    }

    /// A connector whose sockets never yield server data (installation drives
    /// entirely from the scripted camera messages).
    struct NullConnector;
    struct NullSocket;

    impl ProxySocket for NullSocket {
        fn send(&mut self, _data: &[u8]) -> Result<()> {
            Ok(())
        }
        fn try_recv(&mut self) -> Result<Option<Vec<u8>>> {
            Ok(None)
        }
    }

    impl SocketConnector for NullConnector {
        fn connect(&mut self, _host: &str, _port: u16) -> Result<Box<dyn ProxySocket>> {
            Ok(Box::new(NullSocket))
        }
    }

    struct RecordingObserver {
        started: bool,
        progress: Vec<Vec<u8>>,
    }

    impl InstallObserver for RecordingObserver {
        fn on_start(&mut self, _body: &[u8]) -> Result<()> {
            self.started = true;
            Ok(())
        }
        fn on_progress(&mut self, body: &[u8]) {
            self.progress.push(body.to_vec());
        }
    }

    /// Wraps a REST body in the outer message framing the camera would send.
    fn rest_message(sub_type: u16, body: &[u8]) -> (u16, Vec<u8>) {
        let mut payload = Vec::new();
        payload.extend_from_slice(&sub_type.to_be_bytes());
        payload.extend_from_slice(&(body.len() as u16).to_be_bytes());
        payload.extend_from_slice(body);
        (MSG_REST, payload)
    }

    fn http_response(body: &[u8]) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(b"RESP 200 OK\r\n\r\n");
        out.extend_from_slice(body);
        out
    }

    fn http_request(url: &str, body: &[u8]) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(format!("POST {url} REST/1.0\r\n\r\n").as_bytes());
        out.extend_from_slice(body);
        out
    }

    #[test]
    fn install_runs_to_completion() {
        let mut dev = MockDevice::default();
        // empty_buffer at the start of install() sees an empty buffer.
        dev.push_empty();
        // send_init -> Hello
        let mut hello = Vec::new();
        hello.extend_from_slice(&0u32.to_be_bytes());
        dev.push((MSG_COMMON, common_body(COMMON_HELLO, &hello)));
        // send_request(/task/start) -> Response (REST_OUT)
        dev.push(rest_message(REST_OUT, &http_response(b"{\"resultCode\":0}")));
        // loop: a progress request, then completion.
        dev.push(rest_message(REST_IN, &http_request("/task/progress", b"{\"percent\":50}")));
        dev.push(rest_message(REST_IN, &http_request("/task/complete", b"{\"resultCode\":0}")));

        let mut cam = AppInstallCamera::new(dev);
        let mut connector = NullConnector;
        let mut observer = RecordingObserver { started: false, progress: Vec::new() };

        let result = install(
            &mut cam,
            &mut connector,
            "127.0.0.1",
            8080,
            b"XPD",
            "application/x-psn-dstartup2",
            &mut observer,
        )
        .unwrap();

        assert_eq!(result, b"{\"resultCode\":0}");
        assert!(observer.started);
        assert_eq!(observer.progress, vec![b"{\"percent\":50}".to_vec()]);
    }
}
