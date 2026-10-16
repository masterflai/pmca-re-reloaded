//! The sandbox plugin code runs in.
//!
//! A tool or diagnostic is a WebAssembly module run by an interpreter. It sees
//! nothing of this computer except what the host functions below hand it: no
//! files, no network, no key store. What it may ask the host for is limited to
//! the operations its manifest declares, and it runs under a fuel budget and a
//! memory limit, so a faulty plugin ends with an error instead of hanging or
//! exhausting the machine.
//!
//! # Interface (version 1)
//!
//! The module exports `memory`, `pmca_alloc(len: i32) -> i32` and
//! `pmca_run(req_ptr, req_len, data_ptr, data_len: i32) -> i64`. The request is
//! JSON, the data the input file (empty for a diagnostic). The result packs a
//! pointer and a length as `ptr << 32 | len`; the bytes there are a 4-byte
//! little-endian JSON length, the JSON (`status`, `report`, optional
//! `outputExtension`) and, after it, the output file. The module may import
//! `pmca.log(ptr, len)` and `pmca.host_call(ptr, len) -> i64`, the latter taking
//! and returning JSON (`{"op": "os"}`).

use serde::Deserialize;
use serde_json::{Value, json};
use wasmi::{
    Caller, Config, Engine, Extern, Linker, Memory, Module, Store, StoreLimits, StoreLimitsBuilder,
    TrapCode, TypedFunc,
};

/// Interpreter fuel for one call: generous for real work, finite for a loop.
pub(crate) const FUEL: u64 = 5_000_000_000;
/// Linear memory a plugin may grow to.
const MEMORY_LIMIT: usize = 512 << 20;
/// Upper bound for a request or message passed through a host function.
const MAX_HOST_MESSAGE: usize = 1 << 20;
const MAX_LOG: usize = 64 << 10;
const MAX_REPORT: usize = 1 << 20;

/// What a plugin call produced.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Response {
    pub status: String,
    pub report: String,
    pub output_extension: Option<String>,
    pub output: Vec<u8>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Wire {
    status: String,
    #[serde(default)]
    report: String,
    #[serde(default)]
    output_extension: Option<String>,
}

struct Host {
    limits: StoreLimits,
    log: String,
    permitted: Vec<String>,
}

fn engine() -> Engine {
    let mut config = Config::default();
    config.consume_fuel(true);
    Engine::new(&config)
}

fn describe(e: &wasmi::Error) -> String {
    if e.as_trap_code() == Some(TrapCode::OutOfFuel) {
        "the plugin used up its time budget".to_string()
    } else {
        e.to_string()
    }
}

/// Check that `bytes` is a module this host can run, without running it.
pub(crate) fn check(bytes: &[u8]) -> Result<(), String> {
    if !bytes.starts_with(b"\0asm") {
        return Err("not a WebAssembly module".to_string());
    }
    let module = Module::new(&engine(), bytes).map_err(|e| e.to_string())?;
    for name in ["memory", "pmca_alloc", "pmca_run"] {
        if module.get_export(name).is_none() {
            return Err(format!("does not export `{name}`"));
        }
    }
    for import in module.imports() {
        if import.module() != "pmca" || !["log", "host_call"].contains(&import.name()) {
            return Err(format!(
                "imports `{}.{}`, which the host does not provide",
                import.module(),
                import.name()
            ));
        }
    }
    Ok(())
}

/// Copy `len` bytes at `ptr` out of the guest's memory, bounds-checked.
fn read(memory: &[u8], ptr: i64, len: i64, max: usize) -> Result<Vec<u8>, String> {
    let (Ok(ptr), Ok(len)) = (usize::try_from(ptr), usize::try_from(len)) else {
        return Err("negative pointer or length".to_string());
    };
    if len > max {
        return Err("message too large".to_string());
    }
    let end = ptr.checked_add(len).filter(|&e| e <= memory.len()).ok_or("out of bounds")?;
    Ok(memory[ptr..end].to_vec())
}

fn guest_memory(caller: &Caller<'_, Host>) -> Result<Memory, wasmi::Error> {
    caller
        .get_export("memory")
        .and_then(Extern::into_memory)
        .ok_or_else(|| wasmi::Error::new("module has no memory"))
}

fn pack(ptr: i32, len: usize) -> i64 {
    ((u64::from(ptr as u32) << 32) | len as u64) as i64
}

/// Answer one `host_call` request, honouring the plugin's declared access.
pub(crate) fn host_call(permitted: &[String], request: &[u8]) -> Value {
    let op = serde_json::from_slice::<Value>(request)
        .ok()
        .and_then(|v| v.get("op").and_then(Value::as_str).map(str::to_string));
    let Some(op) = op else {
        return json!({ "error": "request must be JSON with an `op`" });
    };
    if !permitted.contains(&op) {
        return json!({ "error": format!("`{op}` is not among this plugin's declared host access") });
    }
    match op.as_str() {
        "os" => json!({
            "os": std::env::consts::OS,
            "arch": std::env::consts::ARCH,
            "family": std::env::consts::FAMILY,
        }),
        "cameras" => match pmca_usb::discovery::list_cameras() {
            Ok(cameras) => json!({
                "cameras": cameras
                    .iter()
                    .map(|c| json!({
                        "mode": c.mode.id(),
                        "vendorId": c.device.vendor_id,
                        "productId": c.device.product_id,
                    }))
                    .collect::<Vec<_>>(),
            }),
            Err(e) => json!({ "error": e.to_string() }),
        },
        _ => json!({ "error": format!("unknown operation `{op}`") }),
    }
}

/// Copy `bytes` into fresh guest memory and return where they went.
fn place(
    store: &mut Store<Host>,
    memory: &Memory,
    alloc: &TypedFunc<i32, i32>,
    bytes: &[u8],
) -> Result<(i32, i32), String> {
    let len = i32::try_from(bytes.len()).map_err(|_| "input too large".to_string())?;
    let ptr = alloc.call(&mut *store, len).map_err(|e| describe(&e))?;
    let offset = usize::try_from(ptr).map_err(|_| "allocator returned a negative pointer")?;
    memory
        .write(&mut *store, offset, bytes)
        .map_err(|_| "allocator returned memory out of bounds")?;
    Ok((ptr, len))
}

fn linker(engine: &Engine) -> Result<Linker<Host>, wasmi::Error> {
    let mut linker = Linker::<Host>::new(engine);
    linker.func_wrap(
        "pmca",
        "log",
        |mut caller: Caller<'_, Host>, ptr: i32, len: i32| -> Result<(), wasmi::Error> {
            let memory = guest_memory(&caller)?;
            let bytes = read(memory.data(&caller), ptr.into(), len.into(), MAX_HOST_MESSAGE)
                .map_err(wasmi::Error::new)?;
            let text = String::from_utf8_lossy(&bytes).into_owned();
            let log = &mut caller.data_mut().log;
            if log.len() + text.len() < MAX_LOG {
                log.push_str(&text);
                log.push('\n');
            }
            Ok(())
        },
    )?;
    linker.func_wrap(
        "pmca",
        "host_call",
        |mut caller: Caller<'_, Host>, ptr: i32, len: i32| -> Result<i64, wasmi::Error> {
            let memory = guest_memory(&caller)?;
            let request = read(memory.data(&caller), ptr.into(), len.into(), MAX_HOST_MESSAGE)
                .map_err(wasmi::Error::new)?;
            let reply = host_call(&caller.data().permitted, &request).to_string().into_bytes();
            let alloc = caller
                .get_export("pmca_alloc")
                .and_then(Extern::into_func)
                .ok_or_else(|| wasmi::Error::new("module has no allocator"))?
                .typed::<i32, i32>(&caller)?;
            let reply_len =
                i32::try_from(reply.len()).map_err(|_| wasmi::Error::new("reply too large"))?;
            let at = alloc.call(&mut caller, reply_len)?;
            let offset = usize::try_from(at).map_err(|_| wasmi::Error::new("negative pointer"))?;
            memory
                .write(&mut caller, offset, &reply)
                .map_err(|_| wasmi::Error::new("allocator returned memory out of bounds"))?;
            Ok(pack(at, reply.len()))
        },
    )?;
    Ok(linker)
}

/// Run a module's `pmca_run` with `request` and `data`.
pub(crate) fn run(
    module: &[u8],
    request: &Value,
    data: &[u8],
    permitted: &[String],
    fuel: u64,
) -> Result<Response, String> {
    let engine = engine();
    let module = Module::new(&engine, module).map_err(|e| e.to_string())?;
    let host = Host {
        limits: StoreLimitsBuilder::new().memory_size(MEMORY_LIMIT).build(),
        log: String::new(),
        permitted: permitted.to_vec(),
    };
    let mut store = Store::new(&engine, host);
    store.limiter(|host| &mut host.limits);
    store.set_fuel(fuel).map_err(|e| e.to_string())?;

    let linker = linker(&engine).map_err(|e| e.to_string())?;
    let instance = linker.instantiate_and_start(&mut store, &module).map_err(|e| describe(&e))?;
    let memory = instance.get_memory(&store, "memory").ok_or("module has no memory")?;
    let alloc =
        instance.get_typed_func::<i32, i32>(&store, "pmca_alloc").map_err(|e| e.to_string())?;
    let entry = instance
        .get_typed_func::<(i32, i32, i32, i32), i64>(&store, "pmca_run")
        .map_err(|e| e.to_string())?;

    let request = request.to_string().into_bytes();
    let (req_ptr, req_len) = place(&mut store, &memory, &alloc, &request)?;
    let (data_ptr, data_len) = place(&mut store, &memory, &alloc, data)?;
    let packed =
        entry.call(&mut store, (req_ptr, req_len, data_ptr, data_len)).map_err(|e| describe(&e))?;

    let packed = packed as u64;
    let bytes = read(
        memory.data(&store),
        (packed >> 32) as i64,
        (packed & 0xffff_ffff) as i64,
        MEMORY_LIMIT,
    )?;
    let log = std::mem::take(&mut store.data_mut().log);
    parse(&bytes, log)
}

/// Split and check the bytes `pmca_run` pointed at.
fn parse(bytes: &[u8], log: String) -> Result<Response, String> {
    let header: [u8; 4] =
        bytes.get(..4).and_then(|h| h.try_into().ok()).ok_or("response is empty")?;
    let json_len = u32::from_le_bytes(header) as usize;
    let json_end = 4usize
        .checked_add(json_len)
        .filter(|&e| e <= bytes.len())
        .ok_or("response is truncated")?;
    let wire: Wire =
        serde_json::from_slice(&bytes[4..json_end]).map_err(|e| format!("response: {e}"))?;
    if !["ok", "warning", "error"].contains(&wire.status.as_str()) {
        return Err(format!("response: unknown status `{}`", wire.status));
    }
    if wire.report.len() > MAX_REPORT {
        return Err("response: report too long".to_string());
    }
    if let Some(ext) = &wire.output_extension
        && (ext.is_empty() || ext.len() > 8 || !ext.chars().all(|c| c.is_ascii_alphanumeric()))
    {
        return Err(format!("response: `{ext}` is not a file extension"));
    }
    let output = bytes[json_end..].to_vec();
    if wire.output_extension.is_none() && !output.is_empty() {
        return Err("response: output without an outputExtension".to_string());
    }
    let report = match (log.is_empty(), wire.report.is_empty()) {
        (true, _) => wire.report,
        (false, true) => log.trim_end().to_string(),
        (false, false) => format!("{}\n{}", log.trim_end(), wire.report),
    };
    Ok(Response { status: wire.status, report, output_extension: wire.output_extension, output })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A module that logs once and answers with a fixed response plus payload.
    fn answering_module(json: &str, payload: &str) -> String {
        let total = 4 + json.len() + payload.len();
        let header: String =
            (json.len() as u32).to_le_bytes().iter().map(|b| format!("\\{b:02x}")).collect();
        format!(
            r#"(module
                (import "pmca" "log" (func $log (param i32 i32)))
                (memory (export "memory") 1)
                (global $next (mut i32) (i32.const 4096))
                (data (i32.const 16) "hello")
                (data (i32.const 256) "{header}{json}{payload}")
                (func (export "pmca_alloc") (param $n i32) (result i32)
                    (local $p i32)
                    (local.set $p (global.get $next))
                    (global.set $next (i32.add (global.get $next) (local.get $n)))
                    (local.get $p))
                (func (export "pmca_run") (param i32 i32 i32 i32) (result i64)
                    (call $log (i32.const 16) (i32.const 5))
                    (i64.or (i64.shl (i64.const 256) (i64.const 32)) (i64.const {total}))))"#,
            json = json.replace('"', "\\\""),
        )
    }

    #[test]
    fn a_module_answers_with_a_report_and_an_output_file() {
        let wat = answering_module(
            r#"{"status":"ok","report":"done","outputExtension":"txt"}"#,
            "PAYLOAD",
        );
        let response = run(wat.as_bytes(), &json!({"kind": "tool"}), b"input", &[], FUEL).unwrap();
        assert_eq!(response.status, "ok");
        assert_eq!(response.report, "hello\ndone");
        assert_eq!(response.output_extension.as_deref(), Some("txt"));
        assert_eq!(response.output, b"PAYLOAD");
    }

    #[test]
    fn output_without_an_extension_is_rejected() {
        let wat = answering_module(r#"{"status":"ok"}"#, "STRAY");
        let err = run(wat.as_bytes(), &json!({}), b"", &[], FUEL).unwrap_err();
        assert!(err.contains("outputExtension"), "{err}");
    }

    #[test]
    fn an_endless_loop_ends_with_an_error() {
        let wat = r#"(module
            (memory (export "memory") 1)
            (func (export "pmca_alloc") (param i32) (result i32) (i32.const 1024))
            (func (export "pmca_run") (param i32 i32 i32 i32) (result i64)
                (loop $l (br $l))
                (i64.const 0)))"#;
        let err = run(wat.as_bytes(), &json!({}), b"", &[], 100_000).unwrap_err();
        assert!(err.contains("time budget"), "{err}");
    }

    #[test]
    fn host_access_is_limited_to_what_the_manifest_declares() {
        let denied = host_call(&[], br#"{"op":"os"}"#);
        assert!(denied["error"].as_str().unwrap().contains("declared"));
        let allowed = host_call(&["os".to_string()], br#"{"op":"os"}"#);
        assert_eq!(allowed["os"], std::env::consts::OS);
        let garbage = host_call(&["os".to_string()], b"not json");
        assert!(garbage.get("error").is_some());
    }

    #[test]
    fn modules_that_reach_for_anything_else_are_refused() {
        let wat = r#"(module
            (import "wasi_snapshot_preview1" "fd_write" (func (param i32 i32 i32 i32) (result i32)))
            (memory (export "memory") 1)
            (func (export "pmca_alloc") (param i32) (result i32) (i32.const 0))
            (func (export "pmca_run") (param i32 i32 i32 i32) (result i64) (i64.const 0)))"#;
        let binary = wat::parse_str(wat).unwrap();
        assert!(check(&binary).unwrap_err().contains("wasi_snapshot_preview1"));
        assert!(check(b"(module)").unwrap_err().contains("not a WebAssembly module"));
        let incomplete = wat::parse_str(r#"(module (memory (export "memory") 1))"#).unwrap();
        assert!(check(&incomplete).unwrap_err().contains("pmca_alloc"));
    }
}
