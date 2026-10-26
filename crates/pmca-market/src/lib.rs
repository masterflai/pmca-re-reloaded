//! # pmca-market
//!
//! The PlayMemories "market" layer of PMCA-RE RELOADED. It
//! is a Rust port of `pmca/marketserver` and has two independent halves:
//!
//! * [`server`] – the **local install server**. During on-camera app
//!   installation the camera opens an HTTPS connection (tunneled over USB by
//!   the [`pmca_usb`](../pmca_usb/index.html) installer flow) and talks a small
//!   JSON/XPD protocol. This module builds the exact responses the camera
//!   expects and, behind the `server` feature, runs a TLS server that serves
//!   them.
//! * [`appstore`] – the **app sources**. Reads the YAML source files users
//!   add from GitHub, resolves their entries to downloadable APKs and plugins,
//!   and describes and searches source repositories.
//!
//! Sony's own PlayMemories Camera Apps store has been shut down, so the port of
//! `pmca/marketclient` that talked to it has been removed.
//!
//! ## Testability without network or hardware
//!
//! The security-sensitive parts – building the camera-facing responses,
//! parsing the (untrusted) data the camera POSTs, and parsing the community
//! store's app lists – are pure functions with no I/O. They are always
//! compiled and fully unit-tested. Only the actual socket work (the TLS server
//! and the `ureq` client) lives behind the `server`/`client` features.
//!
//! ## Encoding & safety
//!
//! The camera speaks Latin-1, matching the original Python. Byte buffers are
//! decoded losslessly via Latin-1 before JSON parsing, and every field access
//! is fallible (no panics on malformed input from the camera or the network).
//!
//! ## TLS compatibility note
//!
//! PlayMemories cameras speak TLS 1.0 with RSA key exchange and AES-CBC-SHA1,
//! and send no `signature_algorithms` extension. No maintained TLS library will
//! negotiate that, so the install server implements the handshake itself in
//! [`tls10`], restricted to loopback. The community-store client is unaffected
//! and still uses `rustls` over the public internet.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod appstore;
pub mod server;
#[cfg(feature = "server")]
pub mod tls10;

/// Errors produced by the market layer.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An error bubbled up from [`pmca_core`] (SPK packing, XPD handling).
    #[error(transparent)]
    Core(#[from] pmca_core::Error),
    /// The JSON exchanged with the camera or a download server was malformed.
    #[error("invalid json: {0}")]
    Json(String),
    /// A required field was missing from a response.
    #[error("missing field: {0}")]
    MissingField(&'static str),
    /// No APK has been staged, so the requested resource cannot be produced.
    #[error("no apk staged for installation")]
    NoApk,
    /// A network or protocol error occurred while downloading (only reachable
    /// with the `client` feature).
    #[error("http error: {0}")]
    Http(String),
    /// An I/O error occurred in the local server (only reachable with the
    /// `server` feature).
    #[error("io error: {0}")]
    Io(String),
    /// A TLS configuration or handshake error occurred (only reachable with the
    /// `server` feature).
    #[error("tls error: {0}")]
    Tls(String),
}

impl Error {
    /// A stable identifier for this kind of failure.
    ///
    /// Front-ends map the code onto a translated message, so it must stay
    /// stable even when the English wording changes.
    pub fn code(&self) -> &'static str {
        match self {
            Error::Core(_) => "market-core",
            Error::Json(_) => "market-json",
            Error::MissingField(_) => "market-missing-field",
            Error::NoApk => "market-no-apk",
            Error::Http(_) => "market-http",
            Error::Io(_) => "market-io",
            Error::Tls(_) => "market-tls",
        }
    }

    /// The variable part of the message, if this variant carries one.
    pub fn detail(&self) -> Option<String> {
        match self {
            Error::Core(e) => Some(e.to_string()),
            Error::Json(s) | Error::Http(s) | Error::Io(s) | Error::Tls(s) => Some(s.clone()),
            Error::MissingField(f) => Some((*f).to_string()),
            Error::NoApk => None,
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Json(e.to_string())
    }
}

/// Convenience alias used throughout the crate.
pub type Result<T> = std::result::Result<T, Error>;

/// Decode a byte buffer as Latin-1 text (lossless, infallible) – the encoding
/// the cameras use.
pub(crate) fn latin1_decode(data: &[u8]) -> String {
    data.iter().map(|&b| b as char).collect()
}
