//! Example diagnostic: which system this is and which Sony cameras are
//! connected in which mode. Uses the host access its manifest declares.

use pmca_plugin_sdk::{Request, Response, host, plugin};

fn run(request: Request, _input: &[u8]) -> Response {
    let german = request.locale.starts_with("de");
    let mut report = Vec::new();

    let os = host("os");
    if let Some(error) = os.get("error") {
        return Response::error(error.to_string());
    }
    report.push(format!(
        "{} / {}",
        os["os"].as_str().unwrap_or("?"),
        os["arch"].as_str().unwrap_or("?")
    ));

    let found = host("cameras");
    if let Some(error) = found.get("error") {
        return Response::warning(format!("{}\n{error}", report.join("\n")));
    }
    let cameras = found["cameras"].as_array().cloned().unwrap_or_default();
    if cameras.is_empty() {
        report.push(
            if german { "Keine Kamera verbunden." } else { "No camera connected." }.to_string(),
        );
        return Response::warning(report.join("\n"));
    }
    for camera in &cameras {
        report.push(format!(
            "{:04x}:{:04x}  {}",
            camera["vendorId"].as_u64().unwrap_or(0),
            camera["productId"].as_u64().unwrap_or(0),
            camera["mode"].as_str().unwrap_or("?"),
        ));
    }
    Response::ok(report.join("\n"))
}

plugin!(run);
