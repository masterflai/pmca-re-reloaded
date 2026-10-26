//! # pmca-plugin-sdk
//!
//! Extensions for PMCA-RE RELOADED are plugin files the user installs from
//! their own disk (Settings → Plugins, or `pmca-console plugins install`) or
//! from a source they added (see `docs/app-sources.md`). This crate is for
//! writing the parts of a plugin
//! that run code — tools and diagnostics — in Rust compiled to WebAssembly.
//! Plugins that only bring data need no code at all.
//!
//! ## The plugin file
//!
//! A `*.pmcaplugin` file is a ZIP archive. `pmca-console plugins pack <dir>
//! <file>` builds one from a folder and checks it the way installing would;
//! with `--key <keyfile>` (from `pmca-console plugins keygen`) it adds
//! `signature.json`, so an installed copy only accepts updates you signed.
//!
//! ```text
//! plugin.json            the manifest (required)
//! locales/en-US.ftl      English names of every entry (required if there are entries)
//! locales/<lang>.ftl     further translations, or a whole new language
//! *.wasm                 the modules tools and diagnostics run
//! ```
//!
//! `plugin.json`:
//!
//! ```json
//! {
//!   "api": 1,
//!   "id": "example",
//!   "name": "Example plugin",
//!   "version": "1.0.0",
//!   "author": "Jane Doe",
//!   "description": "What it is for.",
//!   "tweaks": [{
//!     "id": "my-switch", "property": "0x0107014a", "count": 1, "partSize": 1,
//!     "off": "00", "on": "01", "models": ["ILCE-5100"]
//!   }],
//!   "compatibility": [{
//!     "model": "ILCE-6400", "arch": "CXD90045", "apps": null,
//!     "service": true, "updater": false, "note": "Reported working"
//!   }],
//!   "tools": [{ "id": "hexdump", "module": "hexdump.wasm", "input": ["bin"] }],
//!   "diagnostics": [{ "id": "system", "module": "system-check.wasm", "host": ["os", "cameras"] }]
//! }
//! ```
//!
//! * **ids** are lower-case letters, digits and single hyphens. Every entry's
//!   name is the message `plg-<plugin id>-<entry id>` in `locales/en-US.ftl`;
//!   an optional `…-hint` message adds a one-line explanation. The optional
//!   messages `plg-<plugin id>-name` and `plg-<plugin id>-description`
//!   translate the manifest's `name` and `description`, which is why no entry
//!   may have the id `name` or `description`.
//! * **Languages.** For a language the program ships, a plugin may only add
//!   its own `plg-<plugin id>-…` messages. For any other language the file may
//!   be a complete translation of the program's catalogue; missing messages
//!   fall back to English. One message per line.
//! * **Tweaks** switch one backup setting between two exact values. The
//!   switch is offered only while the camera holds one of them (and, if
//!   `models` is given, only on those models), so it can never write into a
//!   setting that means something else on another camera. A camera's identity
//!   (model, serial number) cannot be changed by a plugin.
//! * **Compatibility** rows appear in their own table under the plugin's name.
//! * **Tools** read a file the user picks and may write one back; the user
//!   chooses where. **Diagnostics** take no input; `host` lists what they may
//!   ask the program for (`os`, `cameras`).
//!
//! Installing takes effect at the next start of the program.
//!
//! ## Code
//!
//! A module runs in a sandbox: no files, no network, no access to the key
//! material, a time budget and a memory limit. It talks to the program only
//! through [`log`] and [`host`].
//!
//! ```ignore
//! use pmca_plugin_sdk::{Request, Response, plugin};
//!
//! fn run(request: Request, input: &[u8]) -> Response {
//!     Response::ok(format!("{} has {} bytes", request.file_name.unwrap_or_default(), input.len()))
//! }
//!
//! plugin!(run);
//! ```
//!
//! Build with `crate-type = ["cdylib"]` for `wasm32-unknown-unknown`:
//! `cargo build --release --target wasm32-unknown-unknown`. The examples of
//! this crate are complete plugins.

use serde::Deserialize;

/// What the program asks the plugin to do.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    /// Interface version of the program.
    pub api: u32,
    /// `"tool"` or `"diagnostic"`.
    pub kind: String,
    /// Id of the entry being run, from the manifest.
    pub entry: String,
    /// The user's language, e.g. `de-DE`, for the plugin's own wording.
    pub locale: String,
    /// Name of the input file (tools only).
    #[serde(default)]
    pub file_name: Option<String>,
}

/// How the plugin judges its result; the program shows it accordingly.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// Done, nothing to worry about.
    Ok,
    /// Done, but something deserves attention.
    Warning,
    /// Could not do it.
    Error,
}

/// The answer to a [`Request`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Response {
    status: Status,
    report: String,
    output: Option<(String, Vec<u8>)>,
}

impl Response {
    /// A successful result with a report for the user.
    pub fn ok(report: impl Into<String>) -> Self {
        Self { status: Status::Ok, report: report.into(), output: None }
    }

    /// A result that deserves attention.
    pub fn warning(report: impl Into<String>) -> Self {
        Self { status: Status::Warning, report: report.into(), output: None }
    }

    /// A failure, with the reason.
    pub fn error(report: impl Into<String>) -> Self {
        Self { status: Status::Error, report: report.into(), output: None }
    }

    /// Attach an output file; the user picks where to save it.
    ///
    /// `extension` is the file extension without a dot, e.g. `"txt"`.
    pub fn with_output(mut self, extension: impl Into<String>, data: Vec<u8>) -> Self {
        self.output = Some((extension.into(), data));
        self
    }

    /// The bytes the program reads back: JSON length, JSON, output file.
    pub fn encode(&self) -> Vec<u8> {
        let status = match self.status {
            Status::Ok => "ok",
            Status::Warning => "warning",
            Status::Error => "error",
        };
        let mut head = serde_json::json!({ "status": status, "report": self.report });
        if let Some((ext, _)) = &self.output {
            head["outputExtension"] = serde_json::Value::String(ext.clone());
        }
        let json = head.to_string().into_bytes();
        let mut out = Vec::with_capacity(4 + json.len());
        out.extend_from_slice(&(json.len() as u32).to_le_bytes());
        out.extend_from_slice(&json);
        if let Some((_, data)) = &self.output {
            out.extend_from_slice(data);
        }
        out
    }
}

#[cfg(target_arch = "wasm32")]
mod ffi {
    #[link(wasm_import_module = "pmca")]
    unsafe extern "C" {
        pub fn log(ptr: *const u8, len: usize);
        pub fn host_call(ptr: *const u8, len: usize) -> i64;
    }
}

/// Add a line to the report the user sees, before the plugin's own report.
pub fn log(message: &str) {
    #[cfg(target_arch = "wasm32")]
    // SAFETY: the host only reads `len` bytes at `ptr`, which `message` owns.
    unsafe {
        ffi::log(message.as_ptr(), message.len());
    }
    #[cfg(not(target_arch = "wasm32"))]
    eprintln!("{message}");
}

/// Ask the program for something the manifest declared under `host`.
///
/// Returns the program's JSON answer; an `"error"` field says why a request
/// was refused. Outside WebAssembly (e.g. in unit tests) every request is
/// refused.
pub fn host(op: &str) -> serde_json::Value {
    #[cfg(target_arch = "wasm32")]
    {
        let request = serde_json::json!({ "op": op }).to_string();
        // SAFETY: the host reads the request, then writes its reply into memory
        // it obtained from `pmca_alloc`, whose ownership passes to us here.
        let reply = unsafe {
            let packed = ffi::host_call(request.as_ptr(), request.len()) as u64;
            __private::take((packed >> 32) as usize as *mut u8, (packed & 0xffff_ffff) as usize)
        };
        serde_json::from_slice(&reply)
            .unwrap_or_else(|_| serde_json::json!({ "error": "unreadable reply" }))
    }
    #[cfg(not(target_arch = "wasm32"))]
    serde_json::json!({ "error": format!("`{op}` is only available inside the program") })
}

/// Declare the function that handles every request of this module.
///
/// The handler has the signature `fn(Request, &[u8]) -> Response`; the slice
/// is the input file (empty for a diagnostic).
#[macro_export]
macro_rules! plugin {
    ($handler:path) => {
        #[unsafe(no_mangle)]
        pub extern "C" fn pmca_alloc(len: usize) -> *mut u8 {
            $crate::__private::alloc(len)
        }

        /// # Safety
        /// Called by the program only, with buffers it obtained from `pmca_alloc`.
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn pmca_run(
            req_ptr: *mut u8,
            req_len: usize,
            data_ptr: *mut u8,
            data_len: usize,
        ) -> i64 {
            // SAFETY: forwarded from the caller's contract above.
            unsafe { $crate::__private::run(req_ptr, req_len, data_ptr, data_len, $handler) }
        }
    };
}

#[doc(hidden)]
pub mod __private {
    use super::{Request, Response};

    /// Hand out a buffer of exactly `len` bytes; ownership passes to the caller.
    pub fn alloc(len: usize) -> *mut u8 {
        Box::into_raw(vec![0u8; len].into_boxed_slice()).cast()
    }

    /// Take back a buffer handed out by [`alloc`].
    ///
    /// # Safety
    /// `ptr` and `len` must come from one call to [`alloc`], not yet taken.
    pub unsafe fn take(ptr: *mut u8, len: usize) -> Vec<u8> {
        // SAFETY: guaranteed by the caller.
        unsafe { Box::from_raw(std::ptr::slice_from_raw_parts_mut(ptr, len)).into_vec() }
    }

    /// Decode the request, run the handler and leave the encoded response in
    /// memory, returning where it is.
    ///
    /// # Safety
    /// Both buffers must come from [`alloc`] and not have been taken.
    pub unsafe fn run(
        req_ptr: *mut u8,
        req_len: usize,
        data_ptr: *mut u8,
        data_len: usize,
        handler: fn(Request, &[u8]) -> Response,
    ) -> i64 {
        // SAFETY: guaranteed by the caller.
        let (request, data) = unsafe { (take(req_ptr, req_len), take(data_ptr, data_len)) };
        let response = match serde_json::from_slice::<Request>(&request) {
            Ok(request) => handler(request, &data),
            Err(e) => Response::error(format!("unreadable request: {e}")),
        };
        let encoded = response.encode();
        let len = encoded.len();
        let ptr = Box::into_raw(encoded.into_boxed_slice()) as *mut u8 as usize;
        (((ptr as u64) << 32) | len as u64) as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_response_encodes_as_the_program_reads_it() {
        let bytes = Response::warning("careful").with_output("txt", b"DATA".to_vec()).encode();
        let json_len = u32::from_le_bytes(bytes[..4].try_into().unwrap()) as usize;
        let head: serde_json::Value = serde_json::from_slice(&bytes[4..4 + json_len]).unwrap();
        assert_eq!(head["status"], "warning");
        assert_eq!(head["report"], "careful");
        assert_eq!(head["outputExtension"], "txt");
        assert_eq!(&bytes[4 + json_len..], b"DATA");
    }

    #[test]
    fn a_plain_response_has_no_output_extension() {
        let bytes = Response::ok("fine").encode();
        let json_len = u32::from_le_bytes(bytes[..4].try_into().unwrap()) as usize;
        let head: serde_json::Value = serde_json::from_slice(&bytes[4..4 + json_len]).unwrap();
        assert!(head.get("outputExtension").is_none());
        assert_eq!(bytes.len(), 4 + json_len);
    }

    #[test]
    fn a_request_decodes_with_and_without_a_file_name() {
        let tool: Request = serde_json::from_str(
            r#"{"api":1,"kind":"tool","entry":"x","locale":"de-DE","fileName":"a.bin"}"#,
        )
        .unwrap();
        assert_eq!(tool.file_name.as_deref(), Some("a.bin"));
        let diagnostic: Request = serde_json::from_str(
            r#"{"api":1,"kind":"diagnostic","entry":"y","locale":"en-US","fileName":null}"#,
        )
        .unwrap();
        assert!(diagnostic.file_name.is_none());
    }
}
