//! `pmca-console` – command-line front-end for the modernized toolkit.
//!
//! This binary exposes the same subcommands as the original Python
//! `pmca-console.py`. Their logic lives in `pmca-app`, which is shared with the
//! GUI; this crate parses arguments and renders results. That holds for the
//! file-only tools (`apk2spk`, `spk2apk`, `print-backup`, `print-usagelog`,
//! `guess-firmware`, `xpd-checksum`, via `pmca_app::tools`), the community
//! store listing (`apps`) and every camera command.
//!
//! Two parts reach into `pmca-usb` directly, because they exist only in a
//! terminal: `driver`, which must run as a separate elevated process, and the
//! interactive updater-shell REPL with its stdin/TCP tunnel, which runs inside
//! the firmware-write callback.

#![warn(missing_docs)]

use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::ExitCode;
use std::sync::mpsc::{Receiver, TryRecvError};
use std::sync::{Mutex, OnceLock};

use clap::{Parser, Subcommand};
use pmca_app::tools::ToolRequest;
use pmca_i18n::Localizer;

mod commands;
mod console;
mod shell_io;

use commands::*;
use console::*;
use shell_io::*;

/// PMCA-RE RELOADED — a toolkit for Sony cameras (command line).
#[derive(Parser)]
#[command(name = "pmca-console", version, about)]
#[command(after_help = concat!(
    "Ported from Sony-PMCA-RE by ma1co (https://github.com/ma1co/Sony-PMCA-RE),\n",
    "published under the same MIT license. Run `pmca-console license` for the\n",
    "acknowledgements, the reason for the port and the license in full.",
))]
struct Cli {
    /// UI language (BCP-47, e.g. `de-DE`). Defaults to the system locale.
    #[arg(long, global = true)]
    lang: Option<String>,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Show the acknowledgements, the origin of this port and the license.
    #[command(alias = "about")]
    License {
        /// Print every dependency's license and copyright notice instead.
        #[arg(long)]
        third_party: bool,
    },
    /// Convert an APK file to an SPK package.
    Apk2spk { input: PathBuf, output: PathBuf },
    /// Convert an SPK package back to an APK file.
    Spk2apk { input: PathBuf, output: PathBuf },
    /// Print the contents of a `Backup.bin` file.
    PrintBackup { file: PathBuf },
    /// Print the contents of a `Usagelog.bin` downloaded with `usage-log`.
    ///
    /// Sony does not document what the event numbers mean, so they are shown
    /// as numbers; the header, the timestamps and how often each number occurs
    /// are read out in full.
    ///
    /// Named after the file it reads, the way `print-backup` is named after
    /// `Backup.bin`; clap's default would have spelled it `print-usage-log`.
    #[command(name = "print-usagelog")]
    PrintUsageLog { file: PathBuf },
    /// Locate the firmware payload (FDAT) inside a `.dat` file.
    GuessFirmware { file: PathBuf },
    /// Compute the CIC (HMAC-SHA256) checksum of a file.
    XpdChecksum { file: PathBuf },

    // --- Hardware commands (delegated to the USB layer) ---
    /// Display information about the connected camera.
    Info,
    /// Download the camera's usage log ("KikiLog").
    ///
    /// The format is undocumented, so the log is written out verbatim. The
    /// camera is only read from.
    UsageLog {
        /// Write the log here instead of `Usagelog.bin`.
        #[arg(long)]
        out: Option<PathBuf>,
    },
    /// Install an app on the connected camera.
    ///
    /// To investigate a failed TLS handshake, set the environment variable
    /// `PMCA_TLS_DUMP` to a file name: the camera's ClientHello is saved there.
    Install {
        /// Install a local APK file instead of a store app.
        #[arg(long, conflicts_with = "app")]
        apk: Option<PathBuf>,
        /// Install an app from the community store by package name.
        #[arg(long, value_name = "PACKAGE")]
        app: Option<String>,
        /// Write the camera's install result (JSON) to this file.
        #[arg(long)]
        out: Option<PathBuf>,
        /// Present this PEM certificate and key instead of generating a
        /// throwaway pair. Only useful for decrypting a capture.
        #[arg(long, value_name = "FILE")]
        cert: Option<PathBuf>,
    },
    /// List the apps offered by the community app store.
    Apps,
    /// Launch the firmware-updater debug shell.
    ///
    /// Switching to updater mode and writing the payload modifies the camera
    /// and is therefore only performed when `--flash` is given.
    Updatershell {
        /// Override the detected camera model (selects the bundled firmware).
        #[arg(long)]
        model: Option<String>,
        /// Use a specific firmware image file instead of a bundled one.
        #[arg(long)]
        fdat_file: Option<PathBuf>,
        /// Directory holding the bundled updater firmware images.
        #[arg(long, default_value = "updatershell/bodyfs")]
        fdat_dir: PathBuf,
        /// Confirm the destructive switch-to-updater + payload write.
        #[arg(long)]
        flash: bool,
    },
    /// Launch the service-mode shell.
    ///
    /// Set the camera's USB connection mode to Auto, and choose Mass Storage
    /// (MSC) when it asks after a session: with Mass Storage set permanently
    /// the camera often refuses the next switch to service mode.
    Serviceshell,
    /// Assign or release the WinUSB driver for the camera (Windows only).
    ///
    /// Service mode needs raw USB access, which Windows does not grant through
    /// the mass-storage driver. This does the job Zadig used to be needed for.
    ///
    /// It exists as a command because assigning a driver requires
    /// administrator rights: an unelevated application cannot do it in-process
    /// and has to re-launch this one command elevated instead. Everything else
    /// keeps running without elevation.
    Driver {
        #[command(subcommand)]
        action: DriverAction,
    },
    /// List or change the connected camera's tweakable settings.
    ///
    /// Without `--enable` / `--disable` / `--set` the current state is printed
    /// and the camera is not modified. Changing a tweak rewrites the camera's
    /// backup memory and therefore requires `--confirm`.
    ///
    /// Tweaks go through service mode: set the camera's USB connection mode to
    /// Auto, and choose Mass Storage (MSC) when it asks after a session.
    ///
    /// For testing, the environment variable `PMCA_TWEAKS_DIRECT` makes every
    /// camera write single values, and `PMCA_TWEAKS_IMAGE` makes every camera
    /// write the whole image, whatever the backup size. With both set,
    /// `PMCA_TWEAKS_DIRECT` wins.
    Tweaks {
        /// Enable a tweak by id (repeatable).
        #[arg(long, value_name = "ID")]
        enable: Vec<String>,
        /// Disable a tweak by id (repeatable).
        #[arg(long, value_name = "ID")]
        disable: Vec<String>,
        /// Select one of a tweak's named states, as `ID=VALUE` (repeatable).
        ///
        /// For settings that are a choice rather than a switch, where
        /// "enabled" would name nothing: `--set palNtsc=PAL`.
        #[arg(long, value_name = "ID=VALUE")]
        set: Vec<String>,
        /// Confirm writing the changes to the camera.
        #[arg(long)]
        confirm: bool,
        /// Also enable settings the toolkit leaves off by default because
        /// this camera cannot fully support them.
        ///
        /// Currently only affects `language` on Japanese cameras, where two
        /// of the languages restart the camera's language menu. Applies to
        /// every `--enable` in the same run.
        #[arg(long)]
        accept_caution: bool,
        /// Write back the values saved before the first per-property change
        /// (cameras whose backup cannot be read in one piece). Needs `--confirm`.
        #[arg(long, conflicts_with_all = ["enable", "disable", "set"])]
        undo: bool,
        /// Write the changes through the whole backup image, for a camera that
        /// refuses them one value at a time. The image read beforehand is
        /// saved first. Needs `--confirm`.
        #[arg(long, conflicts_with = "undo")]
        whole_image: bool,
    },
    /// Find out which backup value a menu setting is stored in (experimental).
    ///
    /// The complete backup is read once per menu value, in a short service
    /// session each; between the reads you change the setting on the camera.
    /// Without `--write-test` the camera is only read from. The result is
    /// saved as a report without serial number or backup contents.
    ///
    /// Use it on your own camera only: service mode itself is an intervention
    /// Sony does not intend.
    Analyze {
        /// The menu setting, as the camera names it.
        #[arg(long, required_unless_present = "restore")]
        setting: Option<String>,
        /// The menu values, comma-separated. The first is the one set now.
        #[arg(long, value_delimiter = ',', required_unless_present = "restore")]
        values: Vec<String>,
        /// Afterwards write the found value as a test and restore it.
        ///
        /// This CHANGES the camera. Only on your own camera, at your own risk.
        /// Asks for confirmation twice; there is no way to skip that.
        #[arg(long)]
        write_test: bool,
        /// Also save the report to this file.
        #[arg(long)]
        out: Option<PathBuf>,
        /// Write back the value an interrupted write test left on the camera.
        #[arg(long, conflicts_with_all = ["setting", "values", "write_test", "out"])]
        restore: bool,
    },
    /// Upload GPS assist data to the connected camera.
    ///
    /// Without `--file` the current data is downloaded from Sony over HTTPS.
    Gps {
        /// Use a local assist-data file instead of downloading it.
        #[arg(long)]
        file: Option<PathBuf>,
    },
    /// Read or write the access points stored for live streaming.
    ///
    /// This is *not* the camera's general Wi-Fi configuration: the settings
    /// live in the camera's live-streaming plug-in, which bodies without a
    /// streaming feature do not implement at all. Without `--write` the
    /// current settings are printed as JSON and the camera is not modified.
    Wifi {
        /// Write the access points from this JSON file to the camera.
        #[arg(long, value_name = "FILE")]
        write: Option<PathBuf>,
        /// Write the settings read from the camera to this file.
        #[arg(long, conflicts_with = "write")]
        out: Option<PathBuf>,
        /// Use the multi-access-point list instead of the single one.
        #[arg(long)]
        multi: bool,
    },
    /// Read or write the camera's live-streaming settings.
    ///
    /// Without `--write` the current settings are printed as JSON and the
    /// camera is not modified.
    Stream {
        /// Write the settings from this JSON file to the camera.
        #[arg(long, value_name = "FILE")]
        write: Option<PathBuf>,
        /// Write the settings read from the camera to this file.
        #[arg(long, conflicts_with = "write")]
        out: Option<PathBuf>,
    },
    /// Flash an official firmware image (`.dat`) onto the camera.
    ///
    /// This overwrites the camera's firmware. The image is validated
    /// non-destructively first; writing it requires both `--flash` and
    /// `--confirm`.
    Firmware {
        /// The firmware `.dat` file to flash.
        file: PathBuf,
        /// Perform the destructive switch-to-updater and firmware write.
        #[arg(long)]
        flash: bool,
        /// Confirm that the camera's firmware may be overwritten.
        #[arg(long)]
        confirm: bool,
    },
    /// Import or inspect the Sony-derived key material the program does not ship.
    ///
    /// The service-mode keys, the CIC key and the SPK key material are not part
    /// of this program. Supply them from a source of your own choosing. The
    /// program keeps only their digests and recognises the right bytes in
    /// whatever you point it at. Observe the laws that apply where you are and
    /// work only on hardware you own.
    Keys {
        #[command(subcommand)]
        action: KeysAction,
    },
    /// Install, remove, list and run plugins.
    ///
    /// A plugin is a `.pmcaplugin` file from your own disk; there is no plugin
    /// store. It can add languages, switches for camera settings, rows for the
    /// compatibility overview, tools and diagnostics. Its code runs in a
    /// sandbox without access to files, network or key material. Changes take
    /// effect at the next start.
    Plugins {
        #[command(subcommand)]
        action: PluginsAction,
    },
}

/// What [`Command::Plugins`] should do.
#[derive(Subcommand)]
enum PluginsAction {
    /// List the installed plugins.
    List,
    /// Check a plugin file and show what it would do, without installing it.
    Inspect { file: PathBuf },
    /// Install or update a plugin from a file.
    Install { file: PathBuf },
    /// Remove an installed plugin.
    Remove { id: String },
    /// Run a plugin tool (with `--input`) or diagnostic (without).
    Run {
        /// Plugin id.
        plugin: String,
        /// Tool or diagnostic id within the plugin.
        entry: String,
        /// The file a tool should read.
        #[arg(long)]
        input: Option<PathBuf>,
        /// Where to write a tool's output (default: beside the input).
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Build a plugin file from a folder, checked as installing would check it.
    Pack { dir: PathBuf, file: PathBuf },
}

/// What [`Command::Keys`] should do.
#[derive(Subcommand)]
enum KeysAction {
    /// Show which key material is present and which is still missing.
    Status,
    /// Import key material from one or more sources.
    Import {
        /// A URL to a repository, archive or file that holds the key material.
        #[arg(value_name = "URL", required = true)]
        url: Vec<String>,
    },
}

/// What [`Command::Driver`] should do.
#[derive(Subcommand)]
enum DriverAction {
    /// Assign WinUSB to one device.
    Bind {
        /// Hardware id, for example `USB\VID_054C&PID_0336`.
        ///
        /// Note for interactive use: the `&` needs quoting in most shells.
        #[arg(long, value_name = "ID")]
        id: String,
    },
    /// Release every assignment this program made and restore the in-box
    /// drivers.
    ///
    /// The same sweep runs automatically when a service session ends and again
    /// at every program start, so this is only needed to tidy up by hand.
    Unbind,
    /// List driver packages Zadig or a similar tool installed for Sony devices.
    ///
    /// The original Sony-PMCA-RE had them installed by hand. Windows gives the
    /// camera to them before this program can assign its own driver, which
    /// breaks service mode and can hide the camera's drive.
    Zadig {
        /// Remove the packages listed; the devices return to the drivers
        /// Windows ships. Asks for administrator rights.
        #[arg(long)]
        remove: bool,
    },
}

/// Renders an error in the user's language where a translation exists.
///
/// An unknown key makes [`Localizer::tr`] echo the key back, which would put
/// `err-usb-stall` on screen; that case falls back to the English `Display`
/// text, as does any error that carries no structured code at all (file and
/// argument handling).
fn error_text(error: &(dyn std::error::Error + 'static), l: &Localizer) -> String {
    let Some((code, detail)) = structured_error(error) else {
        return error.to_string();
    };
    let key = format!("err-{code}");
    let translated = l.tr(&key);
    if translated == key {
        return error.to_string();
    }
    match detail {
        Some(detail) => format!("{translated} ({detail})"),
        None => translated,
    }
}

/// The stable code and variable detail of an error that carries them.
///
/// Commands reach the camera on two routes: the orchestrated ones go through
/// `pmca-app`, the console-only updater-shell REPL calls `pmca-usb` directly
/// and boxes those errors as they are.
/// The three types carry the same pair but are unrelated, so each has to be
/// tried in turn — recognizing only `pmca_app::Error` left the direct route
/// printing English next to a translated prefix.
fn structured_error(
    error: &(dyn std::error::Error + 'static),
) -> Option<(&'static str, Option<String>)> {
    if let Some(e) = error.downcast_ref::<pmca_app::Error>() {
        return Some((e.code(), e.detail()));
    }
    if let Some(e) = error.downcast_ref::<pmca_usb::Error>() {
        return Some((e.code(), e.detail()));
    }
    if let Some(e) = error.downcast_ref::<pmca_market::Error>() {
        return Some((e.code(), e.detail()));
    }
    None
}

fn main() -> ExitCode {
    // Started by this very program as its elevated driver helper: no command
    // line parsing, no console output, just the helper loop.
    let raw: Vec<String> = std::env::args().skip(1).collect();
    if let Some(code) = pmca_app::drivers::run_helper_if_requested(&raw) {
        return ExitCode::from(u8::try_from(code).unwrap_or(1));
    }
    // Parsed first: `--help`, `--version` and argument errors make clap exit
    // the process, which would skip the guard's destructor and leave the
    // console switched. Those paths print ASCII only, so they need no guard.
    let cli = Cli::parse();
    // Held for the rest of the run; restores the code page on the way out.
    let _console = ConsoleCodePage::switch_to_utf8();
    // Load whatever key material the user has already imported, so the
    // protected commands find it. Missing keys are not an error here.
    pmca_app::keys::load_from_disk();
    // Before the first Localizer: plugins may add languages and messages.
    pmca_app::plugins::load();
    let lang = cli
        .lang
        .clone()
        .or_else(system_locale)
        .unwrap_or_else(|| pmca_i18n::DEFAULT_LOCALE.to_string());
    let l = Localizer::new(&lang);

    // A run that is killed part-way through — power loss, task manager, a
    // panic — cannot undo its own driver binding, nor tell the camera to leave
    // service mode. A binding left in place stops the camera's mass-storage and
    // updater modes from working at all, and a camera left in service mode
    // stalls the next session's authentication. Sweeping at start rather than
    // only at exit is what makes that self-healing: whatever state the last run
    // died in, this one begins from the delivery state. Silent when there was
    // nothing to find, which is the normal case.
    //
    // `driver` is exempt. `driver bind` would have its own package swept away
    // moments before staging a new one, and `driver unbind` would find nothing
    // left to release and report zero — telling the user nothing happened when
    // in fact everything did.
    if !matches!(cli.command, Command::Driver { .. }) {
        let swept = pmca_app::service_mode::sweep_leftovers();
        if swept.packages > 0 {
            eprintln!("{}", l.tr_arg("driver-swept", "count", &swept.packages.to_string()));
        }
        if swept.camera_restored {
            eprintln!("{}", l.tr("driver-swept-camera"));
        }
        if pmca_usb::winusb_install::pending_packages() > 0 {
            eprintln!("{}", l.tr("driver-leftover-needs-elevation"));
        }
        let foreign = pmca_app::drivers::foreign_drivers();
        if !foreign.is_empty() {
            let packages = pmca_app::drivers::describe(&foreign);
            eprintln!("{}", l.tr_arg("foreign-drivers-hint", "packages", &packages));
        }
    }

    match run(&cli.command, &l) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{}: {}", l.tr("cli-error-prefix"), error_text(e.as_ref(), &l));
            ExitCode::FAILURE
        }
    }
}

fn run(cmd: &Command, l: &Localizer) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        Command::License { third_party } => {
            license_command(l, *third_party);
        }
        Command::Apk2spk { input, output } => {
            tool_command(
                l,
                ToolRequest::ApkToSpk { input: input.clone(), output: output.clone() },
            )?;
        }
        Command::Spk2apk { input, output } => {
            tool_command(
                l,
                ToolRequest::SpkToApk { input: input.clone(), output: output.clone() },
            )?;
        }
        Command::PrintBackup { file } => {
            tool_command(l, ToolRequest::PrintBackup { input: file.clone() })?;
        }
        Command::PrintUsageLog { file } => {
            tool_command(l, ToolRequest::PrintUsageLog { input: file.clone() })?;
        }
        Command::GuessFirmware { file } => {
            tool_command(l, ToolRequest::GuessFirmware { input: file.clone() })?;
        }
        Command::XpdChecksum { file } => {
            tool_command(l, ToolRequest::XpdChecksum { input: file.clone() })?;
        }
        Command::Info => {
            info_command(l)?;
        }
        Command::UsageLog { out } => {
            usage_log_command(l, out.as_deref())?;
        }
        Command::Updatershell { model, fdat_file, fdat_dir, flash } => {
            updatershell_command(l, model.as_deref(), fdat_file.as_deref(), fdat_dir, *flash)?;
        }
        Command::Install { apk, app, out, cert } => {
            install_command(l, apk.as_deref(), app.as_deref(), out.as_deref(), cert.as_deref())?;
        }
        Command::Apps => {
            apps_command(l)?;
        }
        Command::Serviceshell => {
            serviceshell_command(l)?;
        }
        Command::Driver { action } => {
            driver_command(l, action)?;
        }
        Command::Keys { action } => {
            keys_command(l, action)?;
        }
        Command::Plugins { action } => {
            plugins_command(l, action)?;
        }
        Command::Tweaks { enable, disable, set, confirm, accept_caution, undo, whole_image } => {
            let write = match (undo, whole_image) {
                (true, _) => TweakWrite::Undo,
                (_, true) => TweakWrite::WholeImage,
                _ => TweakWrite::Default,
            };
            tweaks_command(l, enable, disable, set, *confirm, *accept_caution, write)?;
        }
        Command::Analyze { setting, values, write_test, out, restore } => {
            if *restore {
                analyze_restore_command(l)?;
            } else {
                let setting = setting.as_deref().unwrap_or_default();
                analyze_command(l, setting, values, *write_test, out.as_deref())?;
            }
        }
        Command::Gps { file } => {
            gps_command(l, file.as_deref())?;
        }
        Command::Wifi { write, out, multi } => {
            wifi_command(l, write.as_deref(), out.as_deref(), *multi)?;
        }
        Command::Stream { write, out } => {
            stream_command(l, write.as_deref(), out.as_deref())?;
        }
        Command::Firmware { file, flash, confirm } => {
            firmware_command(l, file, *flash, *confirm)?;
        }
    }
    Ok(())
}

/// Run one of the file operations and print what it produced.
///
/// The work itself lives in [`pmca_app::tools`] so that the window can offer
/// the same five operations; all that is left here is deciding where the text
/// goes. The report — a backup dump, a payload offset — is printed after the
/// summary line, because a summary that scrolls off the top of a long dump is
/// no summary at all.
fn tool_command(l: &Localizer, request: ToolRequest) -> Result<(), Box<dyn std::error::Error>> {
    let outcome = pmca_app::tools::run(l, &request)?;
    println!("{}", outcome.message);
    if let Some(report) = outcome.report {
        print!("{report}");
    }
    Ok(())
}

/// Print the acknowledgements, the origin of this port and the license.
///
/// The paragraphs come from the shared catalogue, so this prints what the GUI
/// shows. Lines are left unwrapped: the terminal knows its own width, and
/// breaking the text here would break it in the wrong places for the languages
/// that do not put spaces between words.
///
/// The dependency notices are behind `--third-party` rather than appended,
/// because there are several hundred of them: printed unasked they would push
/// everything above them out of the scrollback, which would hide the licence
/// in the act of showing it.
fn license_command(l: &Localizer, third_party: bool) {
    let about = pmca_app::about::info();

    if third_party {
        print!("{}", about.third_party_text);
        return;
    }

    for section in about.sections {
        println!("{}", l.tr(section.heading));
        for key in section.body {
            println!("  {}", l.tr(key));
        }
        if !section.people.is_empty() {
            println!("  {}", section.people.join(", "));
        }
        println!();
    }

    println!("{}", l.tr("about-license-full"));
    for line in about.license_text.lines() {
        println!("  {line}");
    }
    println!();

    println!("{}", l.tr("about-links-heading"));
    for link in about.links {
        println!("  {}", l.tr(link.label));
        println!("    {}", link.url);
    }
    println!();

    println!("{}", l.tr("about-third-party-heading"));
    println!("  {}", l.tr("about-third-party-body"));
    println!();

    println!("{}: {} ({})", l.tr("about-version"), about.version, about.license);
}

/// Write `value` as pretty JSON to `path`, or to stdout when no path is given.
fn emit_json<T: serde::Serialize>(
    value: &T,
    path: Option<&std::path::Path>,
) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(value)?;
    match path {
        Some(path) => std::fs::write(path, json)?,
        None => println!("{json}"),
    }
    Ok(())
}

/// Print `label: value` when a value is present, otherwise skip the line.
fn print_field(l: &Localizer, key: &str, value: Option<&str>) {
    if let Some(v) = value
        && !v.is_empty()
    {
        println!("{}: {}", l.tr(key), v);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn posix_locales_become_language_tags() {
        assert_eq!(posix_locale_tag("de_DE.UTF-8").as_deref(), Some("de-DE"));
        assert_eq!(posix_locale_tag("pt_BR").as_deref(), Some("pt-BR"));
        assert_eq!(posix_locale_tag("sr_RS@latin").as_deref(), Some("sr-RS"));
        assert_eq!(posix_locale_tag("de-DE").as_deref(), Some("de-DE"));
        assert_eq!(posix_locale_tag("C.UTF-8"), None);
        assert_eq!(posix_locale_tag("POSIX"), None);
        assert_eq!(posix_locale_tag(""), None);
    }

    #[cfg(windows)]
    #[test]
    fn windows_reports_a_user_locale() {
        assert!(windows_user_locale().is_some_and(|l| l.contains('-')));
    }

    #[test]
    fn numbered_steps_get_lines_of_their_own() {
        assert_eq!(
            numbered_lines("NOW: 1. Unplug. 2. Set “A 2.0”. 10. Done."),
            "NOW:\n1. Unplug.\n2. Set “A 2.0”.\n10. Done."
        );
        assert_eq!(numbered_lines("no steps here"), "no steps here");
    }

    #[test]
    fn shell_lines_lose_the_windows_carriage_return() {
        assert_eq!(shell_line_bytes("ls /\r\n"), b"ls /\n");
        assert_eq!(shell_line_bytes("exit\r\n"), b"exit\n");
        // Unix input must survive unchanged.
        assert_eq!(shell_line_bytes("ls /\n"), b"ls /\n");
        // A line without any terminator still needs one, or the remote shell
        // waits forever.
        assert_eq!(shell_line_bytes("ls /"), b"ls /\n");
        // An empty line is a bare newline, not end of input.
        assert_eq!(shell_line_bytes("\r\n"), b"\n");
        // Only the terminator goes; a CR inside the line is the user's own.
        assert_eq!(shell_line_bytes("echo a\rb\r\n"), b"echo a\rb\n");
    }

    #[test]
    fn transport_errors_are_translated_on_every_route() {
        let l = Localizer::new("de-DE");

        // Straight from pmca-usb, as the updater shell boxes it.
        let usb: Box<dyn std::error::Error> = Box::new(pmca_usb::Error::NoDevice);
        let text = error_text(usb.as_ref(), &l);
        assert_ne!(text, pmca_usb::Error::NoDevice.to_string(), "still English");
        assert_eq!(text, l.tr("err-usb-no-device"));

        // The same condition wrapped by pmca-app must read identically.
        let app: Box<dyn std::error::Error> =
            Box::new(pmca_app::Error::from(pmca_usb::Error::NoDevice));
        assert_eq!(error_text(app.as_ref(), &l), text);

        // Straight from pmca-market.
        let market: Box<dyn std::error::Error> = Box::new(pmca_market::Error::NoApk);
        assert_eq!(error_text(market.as_ref(), &l), l.tr("err-market-no-apk"));
    }

    #[test]
    fn errors_without_a_code_keep_their_english_text() {
        let l = Localizer::new("de-DE");
        let io: Box<dyn std::error::Error> = Box::new(std::io::Error::other("something specific"));
        assert_eq!(error_text(io.as_ref(), &l), "something specific");
    }
}
