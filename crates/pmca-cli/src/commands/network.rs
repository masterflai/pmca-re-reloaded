//! `gps`, `wifi` and `stream`.

use crate::*;

/// Upload GPS assist data to the connected camera.
pub(crate) fn gps_command(
    l: &Localizer,
    file: Option<&std::path::Path>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut sink = |event: pmca_app::ProgressEvent| println!("{}", event.message);
    let local = file.map(std::fs::read).transpose()?;
    warn_unsupported(l, pmca_app::devices::Capability::Gps);
    pmca_app::network::update_gps(l, local.as_deref(), &mut sink)?;
    Ok(())
}

/// Read or write the access points stored for live streaming.
///
/// The printed JSON contains the network passphrases in clear text, so the
/// warning is written to stderr and stays out of a redirected settings file.
pub(crate) fn wifi_command(
    l: &Localizer,
    write: Option<&std::path::Path>,
    out: Option<&std::path::Path>,
    multi: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(path) = write {
        let aps: Vec<pmca_app::dto::WifiAccessPoint> =
            serde_json::from_slice(&std::fs::read(path)?)?;
        pmca_app::network::write_wifi(l, multi, &aps)?;
        println!("{}", l.tr("wifi-written"));
        return Ok(());
    }

    eprintln!("{}", l.tr("wifi-read"));
    let aps = pmca_app::network::read_wifi(l, multi)?;
    eprintln!("{}", l.tr("net-secret-warning"));
    emit_json(&aps, out)
}

/// Read or write the camera's live-streaming settings.
pub(crate) fn stream_command(
    l: &Localizer,
    write: Option<&std::path::Path>,
    out: Option<&std::path::Path>,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(path) = write {
        let settings: pmca_app::dto::StreamingSettings =
            serde_json::from_slice(&std::fs::read(path)?)?;
        pmca_app::network::write_streaming(l, &settings)?;
        println!("{}", l.tr("stream-written"));
        return Ok(());
    }

    eprintln!("{}", l.tr("stream-read"));
    let settings = pmca_app::network::read_streaming(l)?;
    eprintln!("{}", l.tr("net-secret-warning"));
    emit_json(&settings, out)
}
