//! TCP relay used to tunnel the camera's proxied TLS session to a local server.
//!
//! The camera opens a "socket" through the MTP app-install proxy; every write
//! must be forwarded to a real TCP connection and every byte the server sends
//! back must be polled without blocking, because the same thread also drives
//! the USB transfers.

use std::io::{Read, Write};
use std::net::TcpStream;

use pmca_usb::app_install::{ProxySocket, SocketConnector};

/// Maximum number of bytes read from the server per poll.
const READ_CHUNK: usize = 16 * 1024;

/// Environment variable naming a file to receive the camera's first payload.
///
/// That payload is the TLS ClientHello. Cameras predate the current TLS
/// baseline, so when a handshake fails the offered version, cipher suites and
/// extensions are the only way to tell what the server would have to accept.
const HELLO_DUMP_VAR: &str = "PMCA_TLS_DUMP";

/// A non-blocking TCP socket exposed to the app-install proxy.
struct TcpProxySocket {
    stream: TcpStream,
    /// Cleared once the first payload has been dumped.
    dump_next: bool,
}

impl ProxySocket for TcpProxySocket {
    fn send(&mut self, data: &[u8]) -> pmca_usb::Result<()> {
        if self.dump_next {
            self.dump_next = false;
            if let Ok(path) = std::env::var(HELLO_DUMP_VAR) {
                // Diagnostics must never break an install that would otherwise
                // succeed, so a failed write is ignored.
                let _ = std::fs::write(path, data);
            }
        }
        self.stream.write_all(data).map_err(|e| pmca_usb::Error::Transfer(e.to_string()))
    }

    fn try_recv(&mut self) -> pmca_usb::Result<Option<Vec<u8>>> {
        let mut buf = [0u8; READ_CHUNK];
        match self.stream.read(&mut buf) {
            // A zero-length read means the peer closed the connection.
            Ok(0) => Ok(Some(Vec::new())),
            Ok(n) => Ok(Some(buf[..n].to_vec())),
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => Ok(None),
            Err(e) => Err(pmca_usb::Error::Transfer(e.to_string())),
        }
    }
}

/// Connects proxied sockets to a TCP endpoint.
pub struct TcpConnector;

impl SocketConnector for TcpConnector {
    fn connect(&mut self, host: &str, port: u16) -> pmca_usb::Result<Box<dyn ProxySocket>> {
        let stream = TcpStream::connect((host, port))
            .map_err(|e| pmca_usb::Error::Transfer(e.to_string()))?;
        stream.set_nonblocking(true).map_err(|e| pmca_usb::Error::Transfer(e.to_string()))?;
        Ok(Box::new(TcpProxySocket { stream, dump_next: true }))
    }
}
