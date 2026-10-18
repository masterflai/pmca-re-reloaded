//! `serviceshell` and `driver`, plus the driver error hints.

use crate::*;

/// Switch a mass-storage camera into service mode, authenticate and run the
/// interactive service shell.
///
/// Service mode exposes raw memory, filesystem and calibration access, so the
/// session starts with an explicit warning. Raw USB access is required; on
/// Windows the device must be bound to WinUSB/libusb rather than the OS
/// mass-storage stack.
///
/// The commands themselves live in [`pmca_app::service`], which the window uses
/// as well: a line typed here and the same line typed there do the same thing.
pub(crate) fn serviceshell_command(l: &Localizer) -> Result<(), Box<dyn std::error::Error>> {
    use std::io::Write;

    let mut sink = |event: pmca_app::ProgressEvent| println!("{}", event.message);
    eprintln!("{}", l.tr("usb-mode-advice"));
    let mut session =
        pmca_app::service::ServiceSession::open(l, &mut sink).map_err(|e| app_driver_hint(l, e))?;

    eprintln!("{}", l.tr("service-warning"));
    println!("{}", l.tr("service-shell-ready"));

    let stdin = std::io::stdin();
    loop {
        print!("senser> ");
        let _ = std::io::stdout().flush();

        let mut line = String::new();
        match stdin.read_line(&mut line) {
            Ok(0) | Err(_) => break,
            Ok(_) => {}
        }
        if line.trim() == "term" {
            interactive_terminal(l, &mut session);
            continue;
        }
        match session.run_line(l, &line) {
            Ok(None) => {}
            // Reported below, once the session is actually closed.
            Ok(Some(outcome)) if outcome.finished => break,
            Ok(Some(outcome)) => {
                println!("{}", outcome.message);
                if let Some(dump) = outcome.dump {
                    print!("{dump}");
                }
            }
            Err(e) => eprintln!("{}: {e}", l.tr("cli-error-prefix")),
        }
    }

    // Leaving the camera in service mode stalls the next session's
    // authentication, so the stop request is sent even after an error. The
    // same call releases the driver binding the session needed.
    session.close();
    println!("{}", l.tr("service-shell-done"));
    Ok(())
}

/// Run the camera's interactive BusyBox serial console until the user types
/// `exit` (which returns to the `senser>` prompt, not out of service mode).
///
/// The console is polled on a background thread so output appears while the
/// user is not typing; a blocking `read_line` on the main thread cannot do
/// both. Terminal errors end the loop rather than abort the session, so the
/// `senser>` prompt — and the clean service-mode exit — is always reached.
fn interactive_terminal(l: &Localizer, session: &mut pmca_app::service::ServiceSession) {
    use std::io::Write;
    use std::sync::mpsc;

    if let Err(e) = session.enable_terminal() {
        eprintln!("{}: {e}", l.tr("cli-error-prefix"));
        return;
    }
    println!("{}", l.tr("service-terminal-interactive"));

    // Typed lines arrive from a reader thread; the main loop owns the camera.
    let (tx, rx) = mpsc::channel::<String>();
    let reader = std::thread::spawn(move || {
        let stdin = std::io::stdin();
        loop {
            let mut line = String::new();
            match stdin.read_line(&mut line) {
                Ok(0) | Err(_) => break,
                Ok(_) => {}
            }
            if tx.send(line).is_err() {
                break;
            }
        }
    });

    let out = std::io::stdout();
    loop {
        let chunk = session.read_terminal();
        if !chunk.is_empty() {
            let mut handle = out.lock();
            let _ = handle.write_all(&chunk);
            let _ = handle.flush();
        }
        match rx.try_recv() {
            Ok(line) => {
                if line.trim() == "exit" {
                    break;
                }
                if let Err(e) = session.write_terminal(line.as_bytes()) {
                    eprintln!("{}: {e}", l.tr("cli-error-prefix"));
                    break;
                }
            }
            Err(mpsc::TryRecvError::Empty) => {
                std::thread::sleep(std::time::Duration::from_millis(50));
            }
            Err(mpsc::TryRecvError::Disconnected) => break,
        }
    }

    let _ = session.disable_terminal();
    // The reader thread is parked on stdin; it ends with the process. Detaching
    // it keeps this function from blocking on a line that may never come.
    drop(reader);
}

/// Assign or release the WinUSB driver.
///
/// Both directions are one call into `pmca-usb`; what makes this worth a
/// command of its own is that it is the only part of the toolkit that needs
/// administrator rights, so it has to be reachable as a separate process.
pub(crate) fn driver_command(
    l: &Localizer,
    action: &DriverAction,
) -> Result<(), Box<dyn std::error::Error>> {
    match action {
        DriverAction::Bind { id } => {
            let published = pmca_usb::winusb_install::bind(id)
                .map_err(|e| -> Box<dyn std::error::Error> { driver_error(l, e) })?;
            println!("{}", l.tr_args("driver-bound", &[("id", id), ("package", &published)]));
        }
        DriverAction::Unbind => {
            let cleanup = pmca_usb::winusb_install::unbind_all()
                .map_err(|e| -> Box<dyn std::error::Error> { driver_error(l, e) })?;
            println!(
                "{}",
                l.tr_arg("driver-released", "count", &cleanup.packages.len().to_string())
            );
        }
        DriverAction::Zadig { remove } => zadig_command(l, *remove)?,
    }
    Ok(())
}

/// List the packages Zadig and similar tools left for Sony devices, and
/// remove them when asked to.
fn zadig_command(l: &Localizer, remove: bool) -> Result<(), Box<dyn std::error::Error>> {
    let found = pmca_app::drivers::foreign_drivers();
    if found.is_empty() {
        println!("{}", l.tr("driver-zadig-none"));
        return Ok(());
    }
    for driver in &found {
        println!(
            "{}",
            l.tr_args(
                "driver-zadig-entry",
                &[
                    ("package", &driver.published),
                    ("provider", &driver.provider),
                    ("devices", &driver.devices.join(", ")),
                ],
            )
        );
    }
    if !remove {
        println!("{}", l.tr("driver-zadig-remove-hint"));
        return Ok(());
    }

    let removed = pmca_app::drivers::remove_foreign_drivers(l)?;
    println!("{}", l.tr_arg("driver-zadig-removed", "count", &removed.len().to_string()));
    let left = pmca_app::drivers::foreign_drivers();
    if !left.is_empty() {
        let packages = pmca_app::drivers::describe(&left);
        return Err(l.tr_arg("foreign-drivers-remaining", "packages", &packages).into());
    }
    Ok(())
}

/// Turn a driver-installation failure into an actionable message.
///
/// Only the missing-rights case gets its own text, because it is the only one
/// the user can act on — and the action ("run this elevated") is not something
/// the raw Win32 message ever says.
pub(crate) fn driver_error(
    l: &Localizer,
    e: pmca_usb::winusb_install::Error,
) -> Box<dyn std::error::Error> {
    match e {
        pmca_usb::winusb_install::Error::NeedsElevation => l.tr("driver-needs-elevation").into(),
        other => other.to_string().into(),
    }
}

/// Turn a raw-USB access failure into an actionable driver hint.
///
/// The same advice as [`driver_hint`], for the errors that arrive already
/// wrapped by the orchestration layer.
pub(crate) fn app_driver_hint(l: &Localizer, e: pmca_app::Error) -> Box<dyn std::error::Error> {
    match e {
        pmca_app::Error::Usb(usb) => driver_hint(l, usb),
        other => other.into(),
    }
}

/// Turn a raw-USB access failure into an actionable driver hint.
pub(crate) fn driver_hint(l: &Localizer, e: pmca_usb::Error) -> Box<dyn std::error::Error> {
    match e {
        pmca_usb::Error::Transfer(_) | pmca_usb::Error::NoDevice => {
            format!("{}: {}", l.tr("service-driver-required"), e).into()
        }
        other => other.into(),
    }
}
