//! `info`, `usage-log`, `install` and `apps`.

use crate::*;

/// Import or report the Sony-derived key material.
pub(crate) fn keys_command(
    l: &Localizer,
    action: &KeysAction,
) -> Result<(), Box<dyn std::error::Error>> {
    match action {
        KeysAction::Status => print_key_status(l, &pmca_app::keys::status()),
        KeysAction::Import { url } => {
            let mut sink = |event: pmca_app::ProgressEvent| println!("{}", event.message);
            let status = pmca_app::keys::import_from_urls(url, l, &mut sink)?;
            print_key_status(l, &status);
        }
    }
    Ok(())
}

/// Print one line per key class and a summary.
fn print_key_status(l: &Localizer, status: &pmca_app::keys::KeyStatus) {
    for class in &status.classes {
        let state = if class.present { "keys-present" } else { "keys-missing" };
        println!("{}: {}", l.tr(&format!("keys-class-{}", class.slug)), l.tr(state));
    }
    println!("{}", l.tr(if status.complete { "keys-complete" } else { "keys-incomplete" }));
}

/// Enumerate a single attached Sony camera and print its identifying details.
pub(crate) fn info_command(l: &Localizer) -> Result<(), Box<dyn std::error::Error>> {
    let report = pmca_app::camera_info()?;

    println!("{}: {}", l.tr("info-mode"), l.tr(&format!("mode-{}", report.mode)));
    print_field(l, "info-model", report.model_name.as_deref());
    print_field(l, "info-product-code", report.model_code.as_deref());
    print_field(l, "info-serial", report.serial.as_deref());
    print_field(l, "info-firmware", report.firmware.as_deref());
    if let Some(lens) = report.lens.as_deref() {
        println!("{}: {}", l.tr("info-lens"), lens);
    }
    Ok(())
}

/// Download the camera's usage log to a file.
///
/// The log's structure is undocumented, so it is stored verbatim rather than
/// being rendered into a format that might misrepresent it.
pub(crate) fn usage_log_command(
    l: &Localizer,
    out: Option<&std::path::Path>,
) -> Result<(), Box<dyn std::error::Error>> {
    let default = std::path::Path::new("Usagelog.bin");
    let out = out.unwrap_or(default);
    println!("{}", l.tr("msg-connecting"));
    let log = pmca_app::usage_log()?;
    std::fs::write(out, &log)?;
    println!(
        "{} ({} bytes)",
        l.tr_arg("msg-usage-log-written", "path", &out.display().to_string()),
        log.len()
    );
    Ok(())
}

/// Locate a data file shipped alongside the executable.
///
/// Relative paths are tried against the working directory first, then next to
/// the binary. Without the second step a released copy would only work when
/// started from its own directory, and an absolute path baked in at build time
/// would only work on the machine that compiled it.
pub(crate) fn resolve_data_file(
    path: &std::path::Path,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    if path.is_absolute() || path.exists() {
        return Ok(path.to_path_buf());
    }
    if let Some(dir) = std::env::current_exe().ok().and_then(|exe| exe.parent().map(PathBuf::from))
    {
        let candidate = dir.join(path);
        if candidate.exists() {
            return Ok(candidate);
        }
    }
    Err(format!("{} not found", path.display()).into())
}

/// Install an application on the connected camera.
///
/// The orchestration (mode switch, local HTTPS server, proxy relay) lives in
/// `pmca-app` and is shared with the GUI; this only renders progress and the
/// final report.
pub(crate) fn install_command(
    l: &Localizer,
    apk_path: Option<&std::path::Path>,
    package: Option<&str>,
    out_path: Option<&std::path::Path>,
    cert: Option<&std::path::Path>,
) -> Result<(), Box<dyn std::error::Error>> {
    let cert = match cert {
        Some(path) => Some(resolve_data_file(path)?),
        None => None,
    };

    let mut sink = |event: pmca_app::ProgressEvent| match event.percent {
        Some(percent) => println!("{} {percent}%", event.message),
        None => println!("{}", event.message),
    };

    // Downloading happens before the camera session so a store or network
    // failure cannot leave the camera in app-install mode.
    let downloaded = match package {
        Some(package) => Some(pmca_app::store::download_community_app(l, package, &mut sink)?.data),
        None => None,
    };
    let apk = match (apk_path, downloaded.as_deref()) {
        (Some(path), _) => Some(pmca_app::ApkSource::File(path)),
        (None, Some(data)) => Some(pmca_app::ApkSource::Bytes(data)),
        (None, None) => None,
    };

    let request = pmca_app::InstallRequest { apk, cert: cert.as_deref() };
    let outcome = pmca_app::install_app(l, &request, &mut sink)?;

    if let Some(device) = outcome.device {
        if let (Some(path), Some(raw)) = (out_path, outcome.raw.as_ref()) {
            std::fs::write(path, serde_json::to_vec_pretty(raw)?)?;
        }
        print_field(l, "info-model", Some(&device.name));
        print_field(l, "info-product-code", Some(&device.product_code));
        print_field(l, "info-serial", Some(&device.device_id));
        print_field(l, "info-firmware", Some(&device.firmware_version));
    }
    Ok(())
}

/// List the apps published in the community app store.
///
/// Only the list itself is fetched; resolving each entry's newest build would
/// cost one API request per app, so that happens on install.
pub(crate) fn apps_command(l: &Localizer) -> Result<(), Box<dyn std::error::Error>> {
    let apps = pmca_app::store::community_apps()?;
    if apps.is_empty() {
        println!("{}", l.tr("msg-no-apps"));
        return Ok(());
    }
    for app in apps {
        let note = if app.installable {
            String::new()
        } else {
            format!(" ({})", l.tr("store-not-installable"))
        };
        println!("{:<44} {}{}", app.package, app.name, note);
        if let Some(desc) = app.description.filter(|d| !d.is_empty()) {
            println!("{:<44} {}", "", desc);
        }
    }
    Ok(())
}
