//! Example tool: a hex dump of the file the user picks, as a report and as a
//! text file to save.

use pmca_plugin_sdk::{Request, Response, log, plugin};

/// At most this many bytes go into the on-screen report.
const PREVIEW: usize = 256;

fn dump(data: &[u8]) -> String {
    let mut out = String::new();
    for (row, chunk) in data.chunks(16).enumerate() {
        let hex: Vec<String> = chunk.iter().map(|b| format!("{b:02x}")).collect();
        let text: String =
            chunk.iter().map(|&b| if b.is_ascii_graphic() { b as char } else { '.' }).collect();
        out.push_str(&format!("{:08x}  {:<47}  {text}\n", row * 16, hex.join(" ")));
    }
    out
}

fn run(request: Request, input: &[u8]) -> Response {
    let name = request.file_name.unwrap_or_else(|| "input".to_string());
    if input.is_empty() {
        return Response::warning(format!("{name} is empty."));
    }
    log(&format!("{name}: {} bytes", input.len()));
    let preview = dump(&input[..input.len().min(PREVIEW)]);
    Response::ok(preview).with_output("txt", dump(input).into_bytes())
}

plugin!(run);
