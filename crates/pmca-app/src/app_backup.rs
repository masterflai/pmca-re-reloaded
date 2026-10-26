//! Copying the apps installed on a camera to this computer.
//!
//! None of the camera's normal USB modes hands an installed app back. The
//! updater payload can: in updater mode the camera's Android data partition is
//! not in use, so it is mounted read-only and the packages in its `app/` folder
//! are copied with the payload's own file transfer. Restoring a backup is an
//! ordinary installation of the saved file.
//!
//! Getting there costs the same reboot and payload write as the updater shell,
//! which is why [`BackupRequest::confirm`] must be set. Nothing on the camera is
//! written, and the payload is gone after the next power cycle.

use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use pmca_i18n::Localizer;
use pmca_usb::command::SonyExtCmdTransport;
use pmca_usb::platform_shell::{PlatformShell, ShellEndpoint};
use sha2::{Digest, Sha256};

use crate::dto::{AppBackupOutcome, BackedUpApp, ProgressEvent, ProgressKind};
use crate::{Error, ProgressSink, Result};

/// Marks the lines the scripts below report through.
///
/// The scripts spell it `PM""CA`, so a shell that echoes its input cannot make
/// a command line look like a result.
const MARK: &str = "PMCA:";

/// Mount the data partition read-only on `/mnt` and list its packages.
///
/// The partition is `nflasha17` on the third camera generation and `nflashb2`
/// on the second, as the payload's own Android-data code has it. A candidate
/// counts only if it holds the folders of an Android data partition.
const LIST_SCRIPT: &str = "\
for d in /dev/nflasha17 /dev/nflashb2; do [ -b $d ] || continue; \
mount -t vfat -o ro $d /mnt 2>/dev/null || continue; \
if [ -d /mnt/app ] && [ -d /mnt/dalvik-cache ]; then echo \"PM\"\"CA:MOUNTED $d\"; break; fi; \
umount /mnt; done
for f in /mnt/app/*.apk; do [ -f \"$f\" ] && echo \"PM\"\"CA:APK $f\"; done
echo \"PM\"\"CA:END\"
exit
";

const UNMOUNT_SCRIPT: &str = "umount /mnt\nexit\n";

/// How long a script may run before its input is closed.
const SCRIPT_TIMEOUT: Duration = Duration::from_secs(60);

/// What to back up, and where to.
#[derive(Debug, Clone)]
pub struct BackupRequest {
    /// Directory holding the updater payload images.
    pub fdat_dir: PathBuf,
    /// Folder for the backups; `None` uses [`backups_dir`].
    pub out_dir: Option<PathBuf>,
    /// The user's agreement to the reboot into updater mode.
    pub confirm: bool,
}

/// Where backups go by default: `app-backups` in the user's data folder.
pub fn backups_dir() -> Option<PathBuf> {
    Some(pmca_i18n::catalog::user_data_dir()?.join("app-backups"))
}

/// Copy every app installed on the attached camera into a folder of its own.
///
/// The camera has to be in its normal mode: its model and serial number name
/// the folder, and a camera in updater mode reports neither.
pub fn backup_apps(
    l: &Localizer,
    request: &BackupRequest,
    sink: &mut dyn ProgressSink,
) -> Result<AppBackupOutcome> {
    // Refused before the camera is opened, so a missing confirmation costs nothing.
    if !request.confirm {
        return Err(Error::Operation(l.tr("app-backup-confirm-required")));
    }
    let root = request
        .out_dir
        .clone()
        .or_else(backups_dir)
        .ok_or_else(|| Error::Operation(l.tr("app-backup-no-folder")))?;

    let info = crate::camera_info()?;
    let model = info.model_name.ok_or_else(|| Error::Operation(l.tr("app-backup-no-model")))?;
    let folder = root.join(folder_name(&model, info.serial.as_deref()));

    let open = crate::updater::OpenRequest {
        model: Some(model),
        fdat_file: None,
        fdat_dir: request.fdat_dir.clone(),
        flash: true,
    };
    let (payload, updater, already_updater) = crate::updater::prepare(l, &open, sink)?;
    let mut updater =
        crate::firmware::enter_updater_mode(l, updater, &payload, already_updater, true, sink)?
            .ok_or_else(|| Error::Operation(l.tr("updater-flash-required")))?;

    sink.emit(ProgressEvent::step(ProgressKind::Progress, l.tr("updater-flashing")));
    let mut copied = None;
    updater.write_firmware(
        &payload,
        Some(&mut |camera| {
            let mut shell = PlatformShell::new(camera.transport_mut());
            shell.start()?;
            copied = Some(copy_apps(l, &mut shell, &folder, sink));
            shell.stop()
        }),
    )?;
    updater.complete()?;
    let apps = copied.ok_or_else(|| Error::Operation(l.tr("updater-flash-required")))??;

    let done = if apps.is_empty() {
        l.tr("app-backup-none")
    } else {
        l.tr_arg("app-backup-done", "count", &apps.len().to_string())
    };
    sink.emit(ProgressEvent::step(ProgressKind::Notice, l.tr("app-backup-power-cycle")));
    sink.emit(ProgressEvent::step(ProgressKind::Done, done));
    Ok(AppBackupOutcome { folder: folder.display().to_string(), apps })
}

/// Mount, copy and unmount, on a running payload.
fn copy_apps<T: SonyExtCmdTransport>(
    l: &Localizer,
    shell: &mut PlatformShell<T>,
    folder: &Path,
    sink: &mut dyn ProgressSink,
) -> Result<Vec<BackedUpApp>> {
    sink.emit(ProgressEvent::step(ProgressKind::Progress, l.tr("app-backup-mounting")));
    let listing = parse_listing(&run_script(shell, LIST_SCRIPT)?);
    if !listing.mounted {
        return Err(Error::Operation(l.tr("app-backup-no-partition")));
    }
    let copied = pull_all(l, shell, &listing.apks, folder, sink);
    // Also after a failed copy; switching the camera off drops the mount as well.
    let _ = run_script(shell, UNMOUNT_SCRIPT);
    copied
}

fn pull_all<T: SonyExtCmdTransport>(
    l: &Localizer,
    shell: &mut PlatformShell<T>,
    apks: &[String],
    folder: &Path,
    sink: &mut dyn ProgressSink,
) -> Result<Vec<BackedUpApp>> {
    if !apks.is_empty() {
        std::fs::create_dir_all(folder)?;
    }
    let mut saved = Vec::with_capacity(apks.len());
    for (i, remote) in apks.iter().enumerate() {
        let name = remote.rsplit('/').next().unwrap_or(remote);
        sink.emit(ProgressEvent::step(
            ProgressKind::Progress,
            l.tr_args(
                "app-backup-copying",
                &[
                    ("name", name),
                    ("current", &(i + 1).to_string()),
                    ("total", &apks.len().to_string()),
                ],
            ),
        ));
        let mut data = Vec::new();
        shell.read_file(remote, &mut data)?;
        saved.push(save(folder, name, &data)?);
    }
    Ok(saved)
}

/// Store one package under its package name and version.
///
/// A file of the same name is kept: identical content is reported as already
/// saved, different content goes to a name with the start of its checksum.
fn save(folder: &Path, camera_name: &str, data: &[u8]) -> Result<BackedUpApp> {
    let sha256: String = Sha256::digest(data).iter().map(|b| format!("{b:02x}")).collect();
    let apk = pmca_apk::read(data).ok();
    let version = apk
        .as_ref()
        .and_then(|a| a.version_name.clone().or_else(|| a.version_code.map(|c| c.to_string())));
    let package = apk.map(|a| a.package);
    let stem = match (&package, &version) {
        (Some(p), Some(v)) => format!("{p}-{v}"),
        (Some(p), None) => p.clone(),
        _ => camera_name.trim_end_matches(".apk").to_string(),
    };
    let stem = safe_name(&stem);

    let mut file = format!("{stem}.apk");
    let mut already_saved = false;
    if let Ok(existing) = std::fs::read(folder.join(&file)) {
        if existing == data {
            already_saved = true;
        } else {
            file = format!("{stem}-{}.apk", &sha256[..8]);
            already_saved = folder.join(&file).exists();
        }
    }
    if !already_saved {
        write_atomically(&folder.join(&file), data)?;
        write_atomically(
            &folder.join(format!("{file}.sha256")),
            format!("{sha256}  {file}\n").as_bytes(),
        )?;
    }
    Ok(BackedUpApp {
        package: package.unwrap_or_else(|| camera_name.to_string()),
        version,
        file,
        sha256,
        already_saved,
    })
}

/// Write via a temporary file, so an interrupted copy leaves no partial backup.
fn write_atomically(path: &Path, data: &[u8]) -> std::io::Result<()> {
    let mut part = path.as_os_str().to_owned();
    part.push(".part");
    let part = PathBuf::from(part);
    std::fs::write(&part, data)?;
    std::fs::rename(&part, path)
}

/// `<model>-<serial>`, reduced to characters every file system accepts.
fn folder_name(model: &str, serial: Option<&str>) -> String {
    match serial {
        Some(serial) if !serial.is_empty() => safe_name(&format!("{model}-{serial}")),
        _ => safe_name(model),
    }
}

fn safe_name(name: &str) -> String {
    name.chars()
        .map(|c| if c.is_ascii_alphanumeric() || matches!(c, '-' | '.' | '_') { c } else { '_' })
        .collect()
}

/// What [`LIST_SCRIPT`] reported.
#[derive(Debug, Default, PartialEq, Eq)]
struct Listing {
    mounted: bool,
    apks: Vec<String>,
}

fn parse_listing(output: &str) -> Listing {
    let mut listing = Listing::default();
    for line in output.lines() {
        let Some(at) = line.find(MARK) else { continue };
        let report = line[at + MARK.len()..].trim_end();
        if report.starts_with("MOUNTED ") {
            listing.mounted = true;
        } else if let Some(path) = report.strip_prefix("APK ") {
            listing.apks.push(path.to_string());
        }
    }
    listing
}

/// Run `script` in the camera's shell and return what it printed.
///
/// The script has to end with `exit`: the tunnel only returns once the camera
/// has closed its side.
fn run_script<T: SonyExtCmdTransport>(
    shell: &mut PlatformShell<T>,
    script: &str,
) -> Result<String> {
    let mut endpoint = Script::new(script);
    shell.interactive_shell(&mut endpoint)?;
    Ok(String::from_utf8_lossy(&endpoint.output).into_owned())
}

/// A shell endpoint that types a fixed script and keeps the output.
struct Script {
    input: Vec<u8>,
    sent: usize,
    output: Vec<u8>,
    deadline: Instant,
}

impl Script {
    fn new(script: &str) -> Self {
        Self {
            input: script.as_bytes().to_vec(),
            sent: 0,
            output: Vec::new(),
            deadline: Instant::now() + SCRIPT_TIMEOUT,
        }
    }
}

impl ShellEndpoint for Script {
    fn try_read(&mut self, buf: &mut [u8]) -> std::io::Result<Option<usize>> {
        if self.sent < self.input.len() {
            let n = (&self.input[self.sent..]).read(buf)?;
            self.sent += n;
            return Ok(Some(n));
        }
        // End of input makes the camera's shell exit should the script hang.
        Ok((Instant::now() > self.deadline).then_some(0))
    }

    fn try_write(&mut self, data: &[u8]) -> std::io::Result<usize> {
        self.output.write_all(data)?;
        Ok(data.len())
    }

    fn close(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_listing_is_read_from_marked_lines_only() {
        let output = "BusyBox v1.13.4\r\n/ # PMCA:MOUNTED /dev/nflasha17\r\n\
            / # PMCA:APK /mnt/app/com.example.app-1.apk\r\nPMCA:APK /mnt/app/b-2.apk\n\
            echo \"PM\"\"CA:APK $f\"\n/ # PMCA:END\n";
        assert_eq!(
            parse_listing(output),
            Listing {
                mounted: true,
                apks: vec!["/mnt/app/com.example.app-1.apk".into(), "/mnt/app/b-2.apk".into()],
            }
        );
        assert!(!parse_listing("/ # PMCA:END\n").mounted);
    }

    #[test]
    fn the_scripts_never_spell_the_mark() {
        assert!(!LIST_SCRIPT.contains(MARK));
        assert!(LIST_SCRIPT.trim_end().ends_with("exit"));
        assert!(UNMOUNT_SCRIPT.trim_end().ends_with("exit"));
    }

    #[test]
    fn folder_names_are_safe() {
        assert_eq!(folder_name("DSC-RX1RM2", Some("0a1b")), "DSC-RX1RM2-0a1b");
        assert_eq!(folder_name("ILCE 6000/x", None), "ILCE_6000_x");
        assert_eq!(folder_name("ILCE-6000", Some("")), "ILCE-6000");
    }

    #[test]
    fn a_backup_keeps_earlier_files() {
        let dir = std::env::temp_dir().join(format!("pmca-app-backup-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        let first = save(&dir, "x-1.apk", b"not an apk").unwrap();
        assert_eq!(first.file, "x-1.apk");
        assert_eq!(first.package, "x-1.apk");
        assert!(!first.already_saved);
        let checksum = std::fs::read_to_string(dir.join("x-1.apk.sha256")).unwrap();
        assert_eq!(checksum, format!("{}  x-1.apk\n", first.sha256));

        assert!(save(&dir, "x-1.apk", b"not an apk").unwrap().already_saved);

        let other = save(&dir, "x-1.apk", b"something else").unwrap();
        assert_eq!(other.file, format!("x-1-{}.apk", &other.sha256[..8]));
        assert!(!other.already_saved);
        assert_eq!(std::fs::read(dir.join("x-1.apk")).unwrap(), b"not an apk");

        std::fs::remove_dir_all(&dir).unwrap();
    }
}
