//! `updatershell` and `firmware`: the firmware-update route and its shells.

use crate::*;

/// Drive the firmware-updater flow and hand off to the interactive shell.
///
/// The camera is inspected and the payload is validated non-destructively
/// (init + guard check + version query). Switching to updater mode and writing
/// the payload alters the camera and only happens when `flash` is `true` (or the
/// camera is already in updater mode).
///
/// Everything up to that point lives in [`pmca_app::updater`], shared with the
/// window; what stays here is the part a window cannot do — a prompt, and a
/// terminal to hand to the `shell` tunnel.
pub(crate) fn updatershell_command(
    l: &Localizer,
    model: Option<&str>,
    fdat_file: Option<&std::path::Path>,
    fdat_dir: &std::path::Path,
    flash: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let request = pmca_app::updater::OpenRequest {
        model: model.map(str::to_string),
        fdat_file: fdat_file.map(std::path::Path::to_path_buf),
        fdat_dir: fdat_dir.to_path_buf(),
        flash,
    };
    let mut sink = |event: pmca_app::ProgressEvent| println!("{}", event.message);
    let (payload, updater, already_updater) = pmca_app::updater::prepare(l, &request, &mut sink)?;

    match pmca_app::firmware::enter_updater_mode(
        l,
        updater,
        &payload,
        already_updater,
        flash,
        &mut sink,
    )? {
        Some(mut updater) => write_firmware_and_run_shell(l, &mut updater, &payload),
        None => Ok(()),
    }
}

/// Flash an official firmware image onto the connected camera.
///
/// This overwrites the camera's firmware and is the most destructive operation
/// the tool offers, so it needs both `--flash` and `--confirm`. The actual work
/// lives in [`pmca_app::firmware`], shared with the GUI.
pub(crate) fn firmware_command(
    l: &Localizer,
    file: &std::path::Path,
    flash: bool,
    confirm: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut sink = |event: pmca_app::ProgressEvent| println!("{}", event.message);

    // On stderr so it stays visible even when the run is being logged.
    eprintln!("{}", l.tr("firmware-warning"));
    eprintln!("{}", l.tr("firmware-untested"));
    pmca_app::firmware::flash_firmware(
        l,
        &pmca_app::firmware::FlashRequest { file, flash, confirm },
        &mut sink,
    )?;
    Ok(())
}

/// Write the payload and, once it is running, launch the interactive shell.
///
/// The completion callback fires when the final firmware chunk has been sent;
/// at that point the payload is live and answers the platform-shell protocol
/// over the same transport, so the REPL runs before the write loop finalises
/// (mirroring the reference `updaterShellCommand`).
pub(crate) fn write_firmware_and_run_shell(
    l: &Localizer,
    updater: &mut pmca_app::firmware::BoxedUpdater,
    fdat_bytes: &[u8],
) -> Result<(), Box<dyn std::error::Error>> {
    use pmca_usb::platform_shell::PlatformShell;

    println!("{}", l.tr("updater-flashing"));
    let mut shell_started = false;
    updater.write_firmware(
        fdat_bytes,
        Some(&mut |u| {
            let mut shell = PlatformShell::new(u.transport_mut());
            shell.start()?;
            shell_started = true;
            println!("{}", l.tr("updater-shell-ready"));
            run_platform_shell(l, &mut shell)?;
            shell.stop()?;
            Ok(())
        }),
    )?;
    updater.complete()?;
    if shell_started {
        println!("{}", l.tr("updater-shell-done"));
    }
    Ok(())
}

/// Interactive REPL over a live platform shell.
///
/// Reads commands from standard input until `exit` or EOF. The words are given
/// their meaning by [`pmca_app::updater::parse`], the same parser the window
/// uses, so a command cannot mean one thing here and another there.
///
/// Individual command failures are printed and the loop continues; only a
/// transport-level failure aborts the session, because the payload is only
/// reachable while this function is on the stack and there is no getting back.
pub(crate) fn run_platform_shell<T: pmca_usb::command::SonyExtCmdTransport>(
    l: &Localizer,
    shell: &mut pmca_usb::platform_shell::PlatformShell<T>,
) -> pmca_usb::Result<()> {
    use pmca_app::updater::UpdaterRequest;
    use std::io::Write;

    loop {
        print!("pmca> ");
        let _ = std::io::stdout().flush();

        // The same reader the tunnel uses, so neither can steal the other's
        // input.
        let Some(line) = read_stdin_line() else { break };
        let request = match pmca_app::updater::parse(&line, l) {
            Ok(Some(request)) => request,
            Ok(None) => continue,
            Err(e) => {
                eprintln!("{}: {e}", l.tr("cli-error-prefix"));
                continue;
            }
        };

        // The tunnel is the console's alone: it needs a local endpoint that can
        // be polled without blocking, which a terminal has and a window does
        // not. `execute` therefore refuses it, and it is intercepted here.
        if let UpdaterRequest::Shell { tcp } = request {
            if let Err(e) = run_camera_shell(shell, tcp) {
                eprintln!("{}: {e}", l.tr("cli-error-prefix"));
            }
            continue;
        }

        match pmca_app::updater::execute(shell, l, &request) {
            // Reported by the caller, once the updater has completed.
            Ok(outcome) if outcome.finished => break,
            Ok(outcome) => {
                println!("{}", outcome.message);
                if let Some(dump) = outcome.dump {
                    print!("{dump}");
                }
            }
            Err(e) => eprintln!("{}: {e}", l.tr("cli-error-prefix")),
        }
    }
    Ok(())
}

/// Bridge stdin/stdout, or a loopback TCP port, to a shell on the camera.
pub(crate) fn run_camera_shell<T: pmca_usb::command::SonyExtCmdTransport>(
    shell: &mut pmca_usb::platform_shell::PlatformShell<T>,
    tcp: Option<u16>,
) -> Result<(), Box<dyn std::error::Error>> {
    match tcp {
        None => {
            println!("entering camera shell, terminate the remote shell to return");
            shell.interactive_shell(&mut StdioEndpoint::new())?;
        }
        Some(port) => {
            // This is an unauthenticated root shell on the camera, so it is
            // bound to the loopback interface only - never to all interfaces.
            let listener = std::net::TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, port))?;
            println!("waiting for a connection on 127.0.0.1:{port} ...");
            let (stream, _) = listener.accept()?;
            let mut endpoint = TcpEndpoint::new(stream)?;
            shell.interactive_shell(&mut endpoint)?;
        }
    }
    println!("camera shell closed");
    Ok(())
}
