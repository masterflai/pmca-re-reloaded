//! Local install server – port of `pmca/marketserver`.
//!
//! The camera drives app installation by talking a tiny protocol over an
//! HTTPS connection to a "portal". During a local install that portal is this
//! server:
//!
//! 1. The installer hands the camera an **XPD** file ([`MarketServer::xpd`])
//!    pointing it at the portal URL.
//! 2. The camera **POST**s its device/account info. If an APK is staged the
//!    server answers with a `dlandinstall` action; otherwise with an empty
//!    action list. The POSTed body is captured as the [`InstallResult`].
//! 3. To fetch the app the camera **GET**s the SPK-packed APK.
//!
//! [`MarketServer`] implements this protocol as pure functions. Behind the
//! `server` feature, [`TlsMarketServer`] wraps it in a real `rustls` listener.

use serde_json::{Value, json};

use crate::{Error, Result, latin1_decode};

/// MIME type the store uses for the JSON exchange. Historically this is *not*
/// `application/json` – the camera only accepts the SPK MIME type here, so we
/// reproduce that quirk exactly.
pub const JSON_MIME: &str = "application/vnd.sony.spk.package-archive";
/// MIME type for the SPK download.
pub const SPK_MIME: &str = "application/vnd.sony.spk.package-archive";
/// MIME type advertised for the XPD startup file.
pub const XPD_MIME: &str = "application/x-psn-dstartup2";
/// Filename extension used for the served SPK.
pub const SPK_EXTENSION: &str = ".1.spk";
/// Cosmetic portal URL embedded in the XPD `TCD` field. The camera's traffic is
/// tunneled to this local server regardless of the host, so this only needs to
/// be a URL the camera accepts.
pub const PORTAL_URL: &str = "https://www.playmemoriescameraapps.com/";

/// Device information parsed out of the camera's install POST.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DeviceInfo {
    /// Human-readable model name (e.g. `ILCE-5100`).
    pub name: String,
    /// Sony product code.
    pub product_code: String,
    /// Device id / serial reported by the camera.
    pub device_id: String,
    /// Firmware version string.
    pub firmware_version: String,
}

/// The result the camera reports at the end of a task, as captured from its
/// POST body. [`raw`](Self::raw) keeps the full JSON for logging/output; the
/// typed [`device`](Self::device) exposes the commonly used fields.
#[derive(Debug, Clone)]
pub struct InstallResult {
    /// Parsed device information.
    pub device: DeviceInfo,
    /// The full JSON document the camera posted.
    pub raw: Value,
}

/// One HTTP reply produced by the protocol layer.
#[derive(Debug, Clone)]
pub struct HttpReply {
    /// `Content-Type` header value.
    pub content_type: &'static str,
    /// Response body bytes.
    pub body: Vec<u8>,
    /// Optional attachment filename (sets `Content-Disposition`).
    pub filename: Option<String>,
}

/// Parse the JSON body the camera POSTs to the portal.
///
/// The body is Latin-1 encoded (matching the original Python). Missing fields
/// default to empty strings rather than erroring, so partial device info from
/// a camera still yields a usable [`InstallResult`].
pub fn parse_post_data(body: &[u8]) -> Result<InstallResult> {
    let text = latin1_decode(body);
    let raw: Value = serde_json::from_str(&text)?;
    let field = |key: &str| -> String {
        raw.get("deviceinfo")
            .and_then(|d| d.get(key))
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string()
    };
    let device = DeviceInfo {
        name: field("name"),
        product_code: field("productcode"),
        device_id: field("deviceid"),
        firmware_version: field("fwversion"),
    };
    Ok(InstallResult { device, raw })
}

/// Build the JSON response that tells the camera to download and install the
/// staged app from `spk_url`.
fn install_response(app_name: &str, spk_url: &str) -> Vec<u8> {
    let value = json!({
        "actions": [{
            "command": "dlandinstall",
            "args": spk_url,
            "attrs": [{
                "attrname": "appname",
                "attrvalue": app_name,
            }],
        }],
    });
    serde_json::to_vec(&value).expect("serializing a fixed json value cannot fail")
}

/// Build the JSON response used when no further action is required.
fn empty_response() -> Vec<u8> {
    serde_json::to_vec(&json!({ "actions": [] }))
        .expect("serializing a fixed json value cannot fail")
}

/// The pure, I/O-free implementation of the camera install protocol.
///
/// It holds the (optional) staged APK and the [`InstallResult`] captured from
/// the camera, and turns incoming GET/POST requests into [`HttpReply`]s.
#[derive(Debug, Clone)]
pub struct MarketServer {
    apk: Option<Vec<u8>>,
    result: Option<InstallResult>,
    /// URL the camera is told to GET the SPK from (the local server).
    server_url: String,
    /// URL embedded in the XPD `TCD` field.
    portal_url: String,
}

impl MarketServer {
    /// Create a server whose `dlandinstall` action points the camera at
    /// `server_url` (e.g. `https://127.0.0.1/`).
    pub fn new(server_url: impl Into<String>) -> Self {
        MarketServer {
            apk: None,
            result: None,
            server_url: server_url.into(),
            portal_url: PORTAL_URL.to_string(),
        }
    }

    /// Stage an APK to be installed. Without this, POSTs get an empty action
    /// list (useful for the read-only `info` flow).
    pub fn set_apk(&mut self, apk: Vec<u8>) {
        self.apk = Some(apk);
    }

    /// The result captured from the camera, if the task has run.
    pub fn result(&self) -> Option<&InstallResult> {
        self.result.as_ref()
    }

    /// Build the XPD startup file that points the camera at the portal. This is
    /// what the installer sends to the camera to begin the task.
    pub fn xpd(&self) -> Result<Vec<u8>> {
        let items = vec![
            ("TCD".to_string(), self.portal_url.clone()),
            ("TKN".to_string(), "0".to_string()),
            ("CIC".to_string(), pmca_core::xpd::calculate_checksum(self.portal_url.as_bytes())?),
        ];
        Ok(pmca_core::xpd::dump(&items)?)
    }

    /// Handle a POST from the camera: decide the response *before* recording the
    /// posted body as the result (matching the original ordering), then capture
    /// the body.
    pub fn handle_post(&mut self, body: &[u8]) -> Result<HttpReply> {
        let response = if self.result.is_none() && self.apk.is_some() {
            install_response("app", &self.server_url)
        } else {
            empty_response()
        };
        self.result = Some(parse_post_data(body)?);
        Ok(HttpReply { content_type: JSON_MIME, body: response, filename: None })
    }

    /// Handle a GET from the camera: serve the staged APK, SPK-packed.
    pub fn handle_get(&self) -> Result<HttpReply> {
        let apk = self.apk.as_ref().ok_or(Error::NoApk)?;
        let spk = pmca_core::spk::dump(apk)?;
        Ok(HttpReply {
            content_type: SPK_MIME,
            body: spk,
            filename: Some(format!("app{SPK_EXTENSION}")),
        })
    }
}

#[cfg(feature = "server")]
mod tls {
    use std::io::{BufRead, BufReader, Write};
    use std::net::{SocketAddr, TcpListener, TcpStream};
    use std::path::Path;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::{Arc, Mutex};
    use std::thread::{self, JoinHandle};

    use super::{HttpReply, InstallResult, MarketServer};
    use crate::tls10::{Tls10Config, Tls10Stream};
    use crate::{Error, Result};

    /// A TLS 1.0 HTTPS server wrapping [`MarketServer`].
    ///
    /// It listens on a background thread and serves the install protocol over
    /// TLS. Shared state (staged APK, captured result) is guarded by a mutex so
    /// the caller can stage an APK and read the result while the server runs.
    ///
    /// The obsolete protocol version is not a choice — see [`crate::tls10`] for
    /// why cameras leave no alternative, and for the loopback restriction that
    /// contains it.
    pub struct TlsMarketServer {
        state: Arc<Mutex<MarketServer>>,
        config: Arc<Tls10Config>,
        addr: SocketAddr,
        listener: Option<TcpListener>,
        stop: Arc<AtomicBool>,
        handle: Option<JoinHandle<()>>,
        last_error: Arc<Mutex<Option<String>>>,
    }

    impl TlsMarketServer {
        /// Bind a server on `addr`.
        ///
        /// The certificate and key are generated on the spot unless `pem_path`
        /// names a combined PEM file to use instead, which only a debugging
        /// session should want.
        ///
        /// The `dlandinstall` action points the camera at
        /// `https://<host>/`, matching the original behaviour.
        ///
        /// `addr` must be a loopback address.
        pub fn bind(addr: SocketAddr, pem_path: Option<&Path>) -> Result<Self> {
            let config = match pem_path {
                Some(path) => Tls10Config::from_pem(path)?,
                None => Tls10Config::generate()?,
            };

            let listener = crate::tls10::bind_loopback(addr)?;
            let bound = listener.local_addr().map_err(|e| Error::Io(e.to_string()))?;
            let server_url = format!("https://{}/", bound.ip());

            Ok(TlsMarketServer {
                state: Arc::new(Mutex::new(MarketServer::new(server_url))),
                config: Arc::new(config),
                addr: bound,
                listener: Some(listener),
                stop: Arc::new(AtomicBool::new(false)),
                handle: None,
                last_error: Arc::new(Mutex::new(None)),
            })
        }

        /// The most recent error from a served connection, if any.
        ///
        /// Connections are handled on background threads, so their errors
        /// cannot be returned to the caller. A TLS handshake the peer rejects
        /// fails here and is visible to the relay only as a dropped socket,
        /// which says nothing about the cause.
        pub fn last_error(&self) -> Option<String> {
            self.last_error.lock().expect("mutex not poisoned").clone()
        }

        /// The address the server is bound to (host, port).
        pub fn local_addr(&self) -> SocketAddr {
            self.addr
        }

        /// Stage an APK to be installed on the camera.
        pub fn set_apk(&self, apk: Vec<u8>) {
            self.state.lock().expect("mutex not poisoned").set_apk(apk);
        }

        /// Build the XPD startup file for the installer to send to the camera.
        pub fn xpd(&self) -> Result<Vec<u8>> {
            self.state.lock().expect("mutex not poisoned").xpd()
        }

        /// Clone the result captured from the camera, if the task has run.
        pub fn result(&self) -> Option<InstallResult> {
            self.state.lock().expect("mutex not poisoned").result().cloned()
        }

        /// Start serving on a background thread.
        pub fn start(&mut self) -> Result<()> {
            let listener =
                self.listener.take().ok_or_else(|| Error::Io("server already started".into()))?;
            listener.set_nonblocking(true).map_err(|e| Error::Io(e.to_string()))?;

            let state = Arc::clone(&self.state);
            let config = Arc::clone(&self.config);
            let stop = Arc::clone(&self.stop);
            let last_error = Arc::clone(&self.last_error);

            self.handle = Some(thread::spawn(move || {
                while !stop.load(Ordering::Relaxed) {
                    match listener.accept() {
                        Ok((stream, _peer)) => {
                            let state = Arc::clone(&state);
                            let config = Arc::clone(&config);
                            let last_error = Arc::clone(&last_error);
                            let _ = stream.set_nonblocking(false);
                            thread::spawn(move || {
                                if let Err(e) = handle_connection(stream, config, state) {
                                    *last_error.lock().expect("mutex not poisoned") =
                                        Some(e.to_string());
                                }
                            });
                        }
                        Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                            thread::sleep(std::time::Duration::from_millis(20));
                        }
                        Err(_) => break,
                    }
                }
            }));
            Ok(())
        }

        /// Signal the accept loop to stop. Running connections finish on their
        /// own; this returns immediately.
        pub fn stop(&mut self) {
            self.stop.store(true, Ordering::Relaxed);
        }
    }

    impl Drop for TlsMarketServer {
        fn drop(&mut self) {
            self.stop();
        }
    }

    fn handle_connection(
        stream: TcpStream,
        config: Arc<Tls10Config>,
        state: Arc<Mutex<MarketServer>>,
    ) -> Result<()> {
        let mut tls = Tls10Stream::accept(stream, &config)?;
        // Keep-Alive: serve requests on this connection until the peer closes it
        // or a request cannot be read.
        loop {
            let mut reader = BufReader::new(&mut tls);
            let request = match read_request(&mut reader)? {
                Some(req) => req,
                None => return Ok(()), // connection closed
            };

            let reply = {
                let mut server = state.lock().expect("mutex not poisoned");
                match request.method.as_str() {
                    "POST" => server.handle_post(&request.body)?,
                    "GET" => server.handle_get()?,
                    _ => HttpReply { content_type: "text/plain", body: Vec::new(), filename: None },
                }
            };

            write_reply(&mut tls, &reply)?;
        }
    }

    struct HttpRequest {
        method: String,
        body: Vec<u8>,
    }

    /// Read a single HTTP/1.1 request (request line + headers + optional body).
    /// Returns `None` when the connection has been closed cleanly.
    fn read_request<R: BufRead>(reader: &mut R) -> Result<Option<HttpRequest>> {
        let mut request_line = String::new();
        let n = reader.read_line(&mut request_line).map_err(|e| Error::Io(e.to_string()))?;
        if n == 0 {
            return Ok(None);
        }
        let method = request_line.split_whitespace().next().unwrap_or("").to_string();

        let mut content_length = 0usize;
        loop {
            let mut line = String::new();
            let n = reader.read_line(&mut line).map_err(|e| Error::Io(e.to_string()))?;
            if n == 0 {
                return Ok(None);
            }
            let line = line.trim_end();
            if line.is_empty() {
                break;
            }
            if let Some((name, value)) = line.split_once(':')
                && name.trim().eq_ignore_ascii_case("content-length")
            {
                content_length = value.trim().parse().unwrap_or(0);
            }
        }

        let mut body = vec![0u8; content_length];
        if content_length > 0 {
            reader.read_exact(&mut body).map_err(|e| Error::Io(e.to_string()))?;
        }
        Ok(Some(HttpRequest { method, body }))
    }

    fn write_reply<W: Write>(writer: &mut W, reply: &HttpReply) -> Result<()> {
        let mut head = format!(
            "HTTP/1.1 200 OK\r\nConnection: Keep-Alive\r\nContent-Type: {}\r\nContent-Length: {}\r\n",
            reply.content_type,
            reply.body.len()
        );
        if let Some(name) = &reply.filename {
            head.push_str(&format!("Content-Disposition: attachment;filename=\"{name}\"\r\n"));
        }
        head.push_str("\r\n");
        writer.write_all(head.as_bytes()).map_err(|e| Error::Io(e.to_string()))?;
        writer.write_all(&reply.body).map_err(|e| Error::Io(e.to_string()))?;
        writer.flush().map_err(|e| Error::Io(e.to_string()))?;
        Ok(())
    }
}

#[cfg(feature = "server")]
pub use tls::TlsMarketServer;

#[cfg(test)]
mod tests {
    use super::*;

    fn post_body() -> &'static [u8] {
        br#"{"deviceinfo":{"name":"ILCE-5100","productcode":"1234","deviceid":"SN123","fwversion":"1.20"},"applications":[]}"#
    }

    #[test]
    fn parse_post_extracts_device_info() {
        let result = parse_post_data(post_body()).unwrap();
        assert_eq!(result.device.name, "ILCE-5100");
        assert_eq!(result.device.product_code, "1234");
        assert_eq!(result.device.device_id, "SN123");
        assert_eq!(result.device.firmware_version, "1.20");
        // The raw document is preserved for logging/output.
        assert!(result.raw.get("applications").is_some());
    }

    #[test]
    fn missing_device_fields_default_to_empty() {
        let result = parse_post_data(br#"{"session":{"correlationid":"x"}}"#).unwrap();
        assert_eq!(result.device, DeviceInfo::default());
    }

    #[test]
    fn first_post_with_apk_triggers_install_then_empty() {
        let mut server = MarketServer::new("https://127.0.0.1/");
        server.set_apk(b"not-a-real-apk".to_vec());

        // First POST: install action, referencing the local server url.
        let first = server.handle_post(post_body()).unwrap();
        let text = String::from_utf8(first.body).unwrap();
        assert!(text.contains("dlandinstall"));
        assert!(text.contains("https://127.0.0.1/"));
        assert!(text.contains("appname"));
        assert_eq!(first.content_type, JSON_MIME);
        // The result was captured from the POST body.
        assert_eq!(server.result().unwrap().device.name, "ILCE-5100");

        // Second POST: no further action.
        let second = server.handle_post(post_body()).unwrap();
        let text = String::from_utf8(second.body).unwrap();
        assert!(text.contains("\"actions\""));
        assert!(!text.contains("dlandinstall"));
    }

    #[test]
    fn post_without_apk_returns_empty_actions() {
        let mut server = MarketServer::new("https://127.0.0.1/");
        let reply = server.handle_post(post_body()).unwrap();
        let text = String::from_utf8(reply.body).unwrap();
        assert!(!text.contains("dlandinstall"));
        assert!(text.contains("actions"));
    }

    #[test]
    fn get_without_apk_is_rejected() {
        let server = MarketServer::new("https://127.0.0.1/");
        assert!(matches!(server.handle_get(), Err(Error::NoApk)));
    }

    #[test]
    fn get_serves_spk_packed_apk() {
        let _g = pmca_core::keystore::test_guard();
        pmca_core::keystore::install_test_keys();
        let mut server = MarketServer::new("https://127.0.0.1/");
        server.set_apk(b"payload-bytes".to_vec());
        let reply = server.handle_get().unwrap();
        assert_eq!(reply.content_type, SPK_MIME);
        assert_eq!(reply.filename.as_deref(), Some("app.1.spk"));
        // Round-trips back to the original apk via the core SPK parser.
        let apk = pmca_core::spk::parse(&reply.body).unwrap();
        assert_eq!(apk, b"payload-bytes");
        pmca_core::keystore::clear();
    }

    #[test]
    fn xpd_contains_portal_url_and_checksum() {
        let _g = pmca_core::keystore::test_guard();
        pmca_core::keystore::install_test_keys();
        let server = MarketServer::new("https://127.0.0.1/");
        let xpd = server.xpd().unwrap();
        let items = pmca_core::xpd::parse(&xpd).unwrap();
        assert_eq!(pmca_core::xpd::get(&items, "TCD"), Some(PORTAL_URL));
        assert_eq!(pmca_core::xpd::get(&items, "TKN"), Some("0"));
        let expected = pmca_core::xpd::calculate_checksum(PORTAL_URL.as_bytes()).unwrap();
        assert_eq!(pmca_core::xpd::get(&items, "CIC"), Some(expected.as_str()));
        pmca_core::keystore::clear();
    }
}
