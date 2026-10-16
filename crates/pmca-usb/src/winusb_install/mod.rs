//! Windows: bind WinUSB to a camera without third-party tools.
//!
//! Sony cameras expose two modes that need raw USB access — mass storage on
//! the way into service mode, and service (senser) mode itself. Windows binds
//! `USBSTOR` to the first and nothing at all to the second, and neither can
//! carry the vendor control transfers those modes require. Until now the
//! answer was to ask the user to download Zadig and rebind the device by hand,
//! twice.
//!
//! This module does the same job in-process. It generates a minimal INF that
//! maps one hardware id onto the in-box `winusb.sys`, signs it with a
//! certificate that exists for the duration of the install, stages it, and
//! binds it.
//!
//! ## What this does and does not ship
//!
//! The generated package contains **no driver code**. It is a few hundred
//! bytes of `Include`/`Needs` directives pointing at Windows' own
//! `winusb.inf`, so the driver that ends up loaded is Microsoft's, signed by
//! Microsoft. What we sign is the mapping — the statement "this device may use
//! WinUSB" — which is the only part Windows requires a third party to youch
//! for.
//!
//! ## Cleanup
//!
//! Every package this module stages is removable, and removal is not optional
//! bookkeeping: while WinUSB is bound, the camera's mass-storage and
//! firmware-updater paths do not work, because those need `USBSTOR` back. So
//! there are two cleanup paths, and both are needed:
//!
//! * [`unbind_all`] at the end of a session, on every exit path.
//! * [`sweep`] at process start, which catches whatever a crashed or
//!   force-killed previous run left behind.
//!
//! The sweep is what makes the design safe to reason about: no matter how a
//! run ends, the next start returns the machine to its delivery state.
//!
//! Everything here needs administrator rights and is Windows-only.

#![allow(unsafe_code)]

pub mod helper_pipe;
mod setup;
mod signing;

use std::path::PathBuf;

/// Marker comment written into every INF this module generates.
///
/// The sweep identifies our packages by this string rather than by remembering
/// names in a file, so it works even for packages staged by a previous build,
/// a previous version, or a run that died before it could record anything.
const PACKAGE_MARKER: &str = "; pmca-winusb-managed-package";

/// Provider shown in the driver properties dialog.
///
/// This names *us* as the publisher of the generated package, so it must not
/// carry the camera manufacturer's name: Windows presents the value as the
/// origin of the installed driver.
const PROVIDER: &str = "PMCA-RE RELOADED";

/// Errors from driver installation.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The operation needs administrator rights.
    ///
    /// Separate from [`Error::Install`] because it is the one failure with an
    /// action the user can take, and the GUI has to be able to offer that
    /// action rather than print a message.
    #[error("changing driver assignments requires administrator rights")]
    NeedsElevation,
    /// The hardware id is not a plain USB vendor/product id.
    #[error("not a usable USB hardware id: {0}")]
    BadHardwareId(String),
    /// A Windows driver-installation call failed.
    #[error("{0}")]
    Install(String),
    /// A temporary file could not be written.
    #[error("could not prepare the driver package: {0}")]
    Io(#[from] std::io::Error),
}

impl Error {
    fn last_os(context: &str) -> Self {
        Self::Install(format!("{context}: {}", std::io::Error::last_os_error()))
    }
}

/// Result alias for this module.
pub type Result<T> = std::result::Result<T, Error>;

/// Encodes a string as a NUL-terminated UTF-16 buffer for Win32.
fn wide(text: &str) -> Vec<u16> {
    text.encode_utf16().chain(std::iter::once(0)).collect()
}

/// What a cleanup pass removed.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Cleanup {
    /// Published names (`oem<n>.inf`) of the packages removed.
    pub packages: Vec<String>,
    /// Number of leftover signing certificates removed.
    pub certificates: usize,
}

impl Cleanup {
    /// Whether anything was actually found. Lets callers stay quiet in the
    /// overwhelmingly common case of a clean machine.
    pub fn is_empty(&self) -> bool {
        self.packages.is_empty() && self.certificates == 0
    }
}

/// Binds WinUSB to `hardware_id`, e.g. `USB\VID_054C&PID_0336`.
///
/// Returns the published package name, which [`unbind_all`] would remove
/// again. Safe to call when the binding already exists: staging an identical
/// package is a no-op and re-binding is idempotent.
pub fn bind(hardware_id: &str) -> Result<String> {
    let hardware_id = validate_hardware_id(hardware_id)?;

    let workspace = Workspace::new()?;
    let inf = workspace.path.join("pmca_winusb.inf");
    let catalog = workspace.path.join("pmca_winusb.cat");

    std::fs::write(&inf, generate_inf(&hardware_id))?;

    // Scope the certificate as tightly as the language allows: it is dropped —
    // and with it removed from the machine trust stores — the moment staging
    // is done, before the device is touched. Verified on hardware: the binding
    // below succeeds with zero certificates present, because trust for a
    // staged package lives in the DriverStore from then on.
    let published = {
        let certificate = signing::TempCertificate::create()?;
        signing::create_signed_catalog(&inf, &catalog, &certificate)?;
        setup::stage_package(&inf)?
    };

    setup::install_package(&inf)?;
    Ok(published)
}

/// Whether a device matching `hardware_id` is plugged in right now, whatever
/// driver it has — including none.
///
/// The complement to the USB backend's own enumeration, which lists only
/// devices Windows already has a driver for. Service mode is a fresh device
/// with a product id no in-box driver claims, so between the mode switch and
/// the binding the camera is plugged in and invisible; waiting for the backend
/// to see it would wait forever. This is what turns that deadlock into a
/// device that can be bound.
pub fn device_present(hardware_id: &str) -> bool {
    match validate_hardware_id(hardware_id) {
        Ok(id) => setup::device_present(&id),
        Err(_) => false,
    }
}

/// Whether a device has to be rebound before raw USB transfers reach it.
///
/// `current_driver` is the lower-cased OS driver name, as
/// `DiscoveredDevice::os_driver` reports it. Only `winusb` needs nothing done
/// — `usbstor` and "no driver at all" both have to be rebound, and so does a
/// libusb-style driver a user installed by hand, because the raw-transfer
/// backend speaks to WinUSB and nothing else.
///
/// Split out from [`bind`] so a caller can tell the user what is about to
/// happen before it happens: assigning a driver takes a moment and may raise a
/// UAC prompt, and a silent pause there reads as a hang.
pub fn needs_binding(current_driver: Option<&str>) -> bool {
    current_driver != Some("winusb")
}

/// Removes every driver package this module has ever staged and restores the
/// in-box drivers.
///
/// Also sweeps leftover certificates, so a single call is enough to undo
/// everything regardless of how far a previous attempt got.
pub fn unbind_all() -> Result<Cleanup> {
    let mut cleanup = Cleanup::default();

    for package in staged_packages() {
        setup::remove_package(&package)?;
        cleanup.packages.push(package);
    }
    if !cleanup.packages.is_empty() {
        // Without this the devices stay driverless until the next replug, and
        // the camera would look broken to the user right after we told them we
        // had tidied up.
        setup::rescan_devices();
    }

    cleanup.certificates = signing::remove_certificates().unwrap_or(0);
    Ok(cleanup)
}

/// Startup sweep: undoes anything a previous run left behind.
///
/// Deliberately infallible. This runs before the user has asked for anything,
/// so a failure here — most likely "not running as administrator" — must not
/// stop the program from starting. Whatever could not be removed is simply
/// found again by the next sweep, or by an explicit [`unbind_all`].
pub fn sweep() -> Cleanup {
    let mut cleanup = Cleanup::default();
    for package in staged_packages() {
        if setup::remove_package(&package).is_ok() {
            cleanup.packages.push(package);
        }
    }
    if !cleanup.packages.is_empty() {
        setup::rescan_devices();
    }
    cleanup.certificates = signing::remove_certificates().unwrap_or(0);
    cleanup
}

/// How many driver packages of ours are still staged, e.g. because the last
/// cleanup ran without administrator rights.
pub fn pending_packages() -> usize {
    staged_packages().len()
}

/// Whether this process runs elevated and may therefore assign drivers.
pub fn can_assign_drivers() -> bool {
    // SAFETY: takes no arguments and only inspects the calling process's token.
    unsafe { windows::Win32::UI::Shell::IsUserAnAdmin().as_bool() }
}

/// Starts the current executable again with administrator rights and returns
/// the new process's id.
///
/// Used to start the driver helper, the one part of the program that needs
/// those rights. A user who dismisses the UAC prompt gets
/// [`Error::NeedsElevation`] back: nothing was started, nothing has to be
/// undone.
///
/// `arguments` is passed through verbatim. It is *not* a shell command line:
/// `ShellExecuteExW` hands it to the new process, so no quoting rules of a
/// shell apply and no interpreter is involved. `parent` is the raw handle of
/// the window that asked; without it Windows may open the consent prompt in
/// the background, as a flashing taskbar button.
pub fn launch_elevated(arguments: &str, parent: Option<isize>) -> Result<u32> {
    use windows::Win32::Foundation::{CloseHandle, ERROR_CANCELLED, HWND};
    use windows::Win32::System::Threading::GetProcessId;
    use windows::Win32::UI::Shell::{
        SEE_MASK_NOASYNC, SEE_MASK_NOCLOSEPROCESS, SHELLEXECUTEINFOW, ShellExecuteExW,
    };
    use windows::Win32::UI::WindowsAndMessaging::SW_HIDE;
    use windows::core::PCWSTR;

    let exe = std::env::current_exe()?;
    let exe = wide(&exe.to_string_lossy());
    let verb = wide("runas");
    let arguments = wide(arguments);
    let mut info = SHELLEXECUTEINFOW {
        cbSize: std::mem::size_of::<SHELLEXECUTEINFOW>() as u32,
        fMask: SEE_MASK_NOCLOSEPROCESS | SEE_MASK_NOASYNC,
        hwnd: HWND(parent.unwrap_or(0) as *mut core::ffi::c_void),
        lpVerb: PCWSTR(verb.as_ptr()),
        lpFile: PCWSTR(exe.as_ptr()),
        lpParameters: PCWSTR(arguments.as_ptr()),
        nShow: SW_HIDE.0,
        ..Default::default()
    };

    // SAFETY: `info` and every buffer it points to outlive the call; a null
    // or stale window handle only loses the prompt its owner.
    if let Err(e) = unsafe { ShellExecuteExW(&mut info) } {
        if e.code() == ERROR_CANCELLED.to_hresult() {
            return Err(Error::NeedsElevation);
        }
        return Err(Error::Install(format!("the elevated helper could not be started: {e}")));
    }
    // SAFETY: SEE_MASK_NOCLOSEPROCESS hands over a process handle that this
    // function owns and closes exactly once.
    let pid = unsafe {
        let pid = GetProcessId(info.hProcess);
        let _ = CloseHandle(info.hProcess);
        pid
    };
    if pid == 0 {
        return Err(Error::last_os("the elevated helper has no process id"));
    }
    Ok(pid)
}

/// Lists the published names of the packages this module staged.
///
/// Reads `%SystemRoot%\INF\oem*.inf` directly. The alternative — enumerating
/// through SetupAPI — returns the same information but needs a driver-store
/// enumeration handle and several more FFI calls, for a set of files we can
/// simply read.
fn staged_packages() -> Vec<String> {
    let Some(dir) = inf_directory() else {
        return Vec::new();
    };
    let Ok(entries) = std::fs::read_dir(dir) else {
        return Vec::new();
    };

    let mut found = Vec::new();
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().to_ascii_lowercase();
        if !name.starts_with("oem") || !name.ends_with(".inf") {
            continue;
        }
        // Read as bytes: an INF written by someone else may not be UTF-8, and
        // a decoding error must not hide a package of ours further down the
        // directory.
        if let Ok(contents) = std::fs::read(entry.path())
            && contents.windows(PACKAGE_MARKER.len()).any(|w| w == PACKAGE_MARKER.as_bytes())
        {
            found.push(name);
        }
    }
    found
}

fn inf_directory() -> Option<PathBuf> {
    std::env::var_os("SystemRoot").map(|root| PathBuf::from(root).join("INF"))
}

/// Rejects anything that is not a plain `USB\VID_xxxx&PID_xxxx` hardware id.
///
/// The value is interpolated into a generated INF, so this is an injection
/// boundary: a caller-supplied string containing a newline could otherwise
/// append arbitrary sections — including ones that install a real driver — to
/// a file we are about to sign and hand to Windows. Only the exact shape the
/// caller legitimately needs is allowed through.
fn validate_hardware_id(id: &str) -> Result<String> {
    let bad = || Error::BadHardwareId(id.to_string());

    let rest = id.strip_prefix(r"USB\VID_").ok_or_else(bad)?;
    let (vid, rest) = rest.split_at_checked(4).ok_or_else(bad)?;
    let pid = rest.strip_prefix("&PID_").ok_or_else(bad)?;

    if pid.len() != 4 {
        return Err(bad());
    }
    if !vid.bytes().chain(pid.bytes()).all(|b| b.is_ascii_hexdigit()) {
        return Err(bad());
    }
    Ok(format!(r"USB\VID_{}&PID_{}", vid.to_uppercase(), pid.to_uppercase()))
}

/// Builds the INF for one hardware id.
///
/// `CatalogFile` has to be present before the catalog is generated, because
/// the catalog hashes this file: adding the directive afterwards would
/// invalidate the very signature it refers to.
fn generate_inf(hardware_id: &str) -> String {
    // The decoration Windows uses for the architecture-specific model section.
    let arch = if cfg!(target_arch = "aarch64") { "NTARM64" } else { "NTamd64" };

    format!(
        "{PACKAGE_MARKER}\r\n\
         ; Generated automatically. Removed again when the session ends.\r\n\
         [Version]\r\n\
         Signature   = \"$Windows NT$\"\r\n\
         Class       = USBDevice\r\n\
         ClassGuid   = {{88BAE032-5A81-49F0-BC3D-A4FF138216D6}}\r\n\
         Provider    = %ProviderName%\r\n\
         CatalogFile = pmca_winusb.cat\r\n\
         DriverVer   = 01/01/2020,1.0.0.0\r\n\
         \r\n\
         [Manufacturer]\r\n\
         %ProviderName% = Devices,{arch}\r\n\
         \r\n\
         [Devices.{arch}]\r\n\
         %DeviceName% = USB_Install, {hardware_id}\r\n\
         \r\n\
         ; No driver binaries of our own: these three sections pull in the\r\n\
         ; service, copy and registry work from Windows' own winusb.inf, so the\r\n\
         ; driver that loads is Microsoft's.\r\n\
         [USB_Install]\r\n\
         Include = winusb.inf\r\n\
         Needs   = WINUSB.NT\r\n\
         \r\n\
         [USB_Install.Services]\r\n\
         Include = winusb.inf\r\n\
         Needs   = WINUSB.NT.Services\r\n\
         \r\n\
         [USB_Install.HW]\r\n\
         Include = winusb.inf\r\n\
         Needs   = WINUSB.NT.HW\r\n\
         \r\n\
         [Strings]\r\n\
         ProviderName = \"{PROVIDER}\"\r\n\
         DeviceName   = \"Sony Camera (WinUSB)\"\r\n"
    )
}

/// A temporary directory that deletes itself.
struct Workspace {
    path: PathBuf,
}

impl Workspace {
    fn new() -> Result<Self> {
        // The name only has to be unique within this machine's temp directory;
        // process id plus a clock reading is enough and avoids pulling in a
        // random-number dependency for a directory name.
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.subsec_nanos())
            .unwrap_or(0);
        let path = std::env::temp_dir().join(format!("pmca-winusb-{}-{nonce}", std::process::id()));
        std::fs::create_dir_all(&path)?;
        Ok(Self { path })
    }
}

impl Drop for Workspace {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_a_plain_usb_hardware_id() {
        assert_eq!(
            validate_hardware_id(r"USB\VID_054c&PID_0336").unwrap(),
            r"USB\VID_054C&PID_0336"
        );
    }

    #[test]
    fn rejects_ids_that_could_inject_inf_sections() {
        // The whole point of the validator: a newline would let a caller append
        // sections to a file we are about to sign.
        for bad in [
            "USB\\VID_054C&PID_0336\r\n[Devices.NTamd64]",
            r"USB\VID_054C&PID_033",
            r"USB\VID_054C&PID_033G",
            r"USB\VID_054C",
            r"PCI\VEN_054C&DEV_0336",
            "",
        ] {
            assert!(validate_hardware_id(bad).is_err(), "should have been rejected: {bad:?}");
        }
    }

    #[test]
    fn rejects_compatible_ids_which_would_match_far_too_much() {
        // Binding WinUSB to a class-level compatible id would capture every
        // mass-storage device on the machine, not just the camera.
        assert!(validate_hardware_id(r"USB\Class_08&SubClass_06").is_err());
    }

    #[test]
    fn generated_inf_carries_the_marker_the_sweep_looks_for() {
        let inf = generate_inf(r"USB\VID_054C&PID_0336");
        assert!(inf.starts_with(PACKAGE_MARKER));
        assert!(inf.contains(r"USB\VID_054C&PID_0336"));
        // Without this directive the package is refused outright, and the
        // catalog would be built over an INF that does not reference it.
        assert!(inf.contains("CatalogFile = pmca_winusb.cat"));
    }

    #[test]
    fn generated_inf_ships_no_driver_of_its_own() {
        let inf = generate_inf(r"USB\VID_054C&PID_0336");
        assert!(inf.contains("Include = winusb.inf"));
        assert!(!inf.contains("CopyFiles"));
        assert!(!inf.contains(".sys"));
    }

    #[test]
    fn cleanup_reports_emptiness() {
        assert!(Cleanup::default().is_empty());
        assert!(!Cleanup { packages: vec!["oem140.inf".into()], certificates: 0 }.is_empty());
    }
}
