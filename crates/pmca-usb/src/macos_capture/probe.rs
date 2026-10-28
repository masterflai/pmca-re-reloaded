//! Diagnostic run for macOS: reaches a Sony camera's mass-storage interface
//! through a capture, in each USB mode the camera is set to, and sends it the
//! program's real camera-info commands both through `nusb` and through
//! IOUSBHost, writing every step to a report. The `macos_probe` example starts
//! it as root; the program itself never calls it.

use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, Write as _};
use std::time::{Duration, Instant};

use nusb::MaybeFuture;

use super::*;
use crate::UsbBackend;
use crate::discovery::{self, CameraMode};
use crate::msc::MscDevice;
use crate::nusb_backend::NusbBackend;
use crate::sony_extcmd::SonyExtCmdCamera;

/// Identifies the probe's logic in a report.
const REVISION: &str = "probe 2";

/// The report, written to the current directory.
const REPORT: &str = "pmca-macos-probe.txt";

/// How long each way to an interface is retried.
const OPEN_WAIT: Duration = Duration::from_secs(3);

/// Timeout of a single test transfer.
const IO_TIMEOUT: Duration = Duration::from_secs(5);

/// How long the mass-storage driver may take to return after a reset.
const DRIVER_SETTLE: Duration = Duration::from_secs(8);

/// Runs the probe interactively; returns the process exit code.
pub fn run() -> i32 {
    if !is_root() {
        eprintln!("Bitte mit sudo starten / please run with sudo:  sudo ./pmca-macos-probe");
        return 2;
    }
    let mut r = Report::new();
    r.line(format!("PMCA-RE RELOADED macOS probe {} ({REVISION})", env!("CARGO_PKG_VERSION")));
    if r.file.is_none() {
        r.line("!!! report file not writable here; please copy this terminal output instead");
    }
    environment(&mut r);

    let steps = [("Auto", "Auto"), ("Massenspeicher", "Mass Storage"), ("MTP", "MTP")];
    for (index, (german, english)) in steps.iter().enumerate() {
        match ask(index + 1, steps.len(), german, english) {
            Answer::Quit => break,
            Answer::Skip => {
                r.line(format!("step {english}: skipped by the tester"));
                continue;
            }
            Answer::Go => {}
        }
        r.mode = (*english).to_string();
        r.line(format!("########## USB connection set to: {english}"));
        let Some(info) = wait_for_any(&mut r) else { continue };
        r.line(format!("found: {}", describe(&info)));
        if interface_of(&info, crate::class::MSC).is_some() {
            suite(&mut r, crate::class::MSC, false);
        } else if interface_of(&info, crate::class::PTP).is_some() {
            suite(&mut r, crate::class::PTP, false);
            r.mode = format!("{english}, switched to mass storage");
            r.line(format!("########## {}", r.mode));
            suite(&mut r, crate::class::MSC, true);
        } else {
            r.line("the camera offers neither mass storage nor PTP");
        }
    }
    summary(&mut r);
    r.hand_over();
    0
}

enum Answer {
    Go,
    Skip,
    Quit,
}

fn ask(step: usize, steps: usize, german: &str, english: &str) -> Answer {
    println!();
    println!("------------------------------------------------------------------");
    println!("Schritt {step}/{steps}: An der Kamera MENU > Einstellungen > USB-Verbindung");
    println!("auf \"{german}\" stellen. Kamera einschalten, per USB-Datenkabel anschliessen,");
    println!("dann Enter druecken. (s + Enter = Schritt ueberspringen, q + Enter = beenden)");
    println!("Step {step}/{steps}: set MENU > Setup > USB Connection to \"{english}\",");
    println!("switch the camera on, connect it, then press Enter. (s = skip, q = quit)");
    println!("------------------------------------------------------------------");
    let mut answer = String::new();
    match std::io::stdin().lock().read_line(&mut answer) {
        Ok(0) | Err(_) => Answer::Quit,
        Ok(_) => match answer.trim().to_ascii_lowercase().as_str() {
            "q" => Answer::Quit,
            "s" => Answer::Skip,
            _ => Answer::Go,
        },
    }
}

/// The report: every line goes to the terminal and the file, long blocks to
/// the file only.
struct Report {
    file: Option<File>,
    start: Instant,
    mode: String,
    results: Vec<(String, String, String)>,
}

impl Report {
    fn new() -> Self {
        Self {
            file: File::create(REPORT).ok(),
            start: Instant::now(),
            mode: "-".into(),
            results: Vec::new(),
        }
    }

    fn write(&mut self, text: &str) {
        if let Some(file) = &mut self.file {
            let _ = writeln!(file, "{text}");
            let _ = file.flush();
        }
    }

    fn line(&mut self, text: impl AsRef<str>) {
        let text = format!("[{:7.1}s] {}", self.start.elapsed().as_secs_f32(), text.as_ref());
        println!("{text}");
        self.write(&text);
    }

    /// `text` into the file, at most `max` lines: the first ones, or the last
    /// ones if `tail`.
    fn block(&mut self, title: &str, text: &str, max: usize, tail: bool) {
        let lines: Vec<&str> = text.lines().collect();
        let omitted = lines.len().saturating_sub(max);
        let shown = if tail { &lines[omitted..] } else { &lines[..lines.len() - omitted] };
        self.write(&format!("--- {title}: {} lines, {omitted} omitted ---", lines.len()));
        for line in shown {
            self.write(&format!("  | {line}"));
        }
    }

    fn result(&mut self, way: &str, outcome: String) {
        self.line(format!("RESULT {way}: {outcome}"));
        self.results.push((self.mode.clone(), way.to_string(), outcome));
    }

    /// Give the report to the user who started `sudo`, and say where it is.
    fn hand_over(&mut self) {
        self.file = None;
        let uid = std::env::var("SUDO_UID").ok().and_then(|v| v.parse().ok());
        let gid = std::env::var("SUDO_GID").ok().and_then(|v| v.parse().ok());
        let _ = std::os::unix::fs::chown(REPORT, uid, gid);
        let path =
            std::env::current_dir().map(|d| d.join(REPORT)).unwrap_or_else(|_| REPORT.into());
        println!();
        println!("Fertig. Bitte diese Datei zurueckschicken / done, please send back:");
        println!("  {}", path.display());
    }
}

fn summary(r: &mut Report) {
    r.line("########## SUMMARY");
    let results = std::mem::take(&mut r.results);
    for (mode, way, outcome) in &results {
        r.line(format!("{mode} | {way} | {outcome}"));
    }
}

fn shell(program: &str, args: &[&str]) -> String {
    match std::process::Command::new(program).args(args).output() {
        Ok(out) => format!(
            "{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        )
        .trim_end()
        .to_string(),
        Err(e) => format!("{program}: {e}"),
    }
}

fn environment(r: &mut Report) {
    r.line(format!("macOS: {}", shell("/usr/bin/sw_vers", &[]).replace('\n', ", ")));
    r.line(format!(
        "machine: {}",
        shell("/usr/sbin/sysctl", &["-n", "hw.model", "machdep.cpu.brand_string"])
            .replace('\n', ", ")
    ));
    r.line(format!("kernel: {}", shell("/usr/bin/uname", &["-a"])));
    r.line(format!("SIP: {}", shell("/usr/bin/csrutil", &["status"])));
    // SAFETY: a constant the framework exports.
    let version = unsafe { IOUSBHostVersionNumber };
    r.line(format!("IOUSBHost framework {version}"));
    r.line(format!("SUDO_USER: {}", std::env::var("SUDO_USER").unwrap_or_default()));
    let own = std::process::id().to_string();
    let running: Vec<String> = shell("/usr/bin/pgrep", &["-il", "pmca"])
        .lines()
        .filter(|l| !l.starts_with(&own))
        .map(str::to_string)
        .collect();
    if !running.is_empty() {
        r.line(format!("WARNING, still running: {}", running.join("; ")));
        println!("!!! Bitte PMCA-RE RELOADED beenden / please quit PMCA-RE RELOADED !!!");
    }
    let devices: Vec<String> = sony_devices().iter().map(describe).collect();
    r.line(format!("Sony devices now: {}", devices.join(" | ")));
}

fn sony_devices() -> Vec<nusb::DeviceInfo> {
    nusb::list_devices()
        .wait()
        .map(|all| all.filter(|d| d.vendor_id() == crate::sony::SONY_ID_VENDOR).collect())
        .unwrap_or_default()
}

fn interface_of(device: &nusb::DeviceInfo, class: u8) -> Option<u8> {
    device.interfaces().find(|i| i.class() == class).map(|i| i.interface_number())
}

fn describe(device: &nusb::DeviceInfo) -> String {
    let interfaces: Vec<String> = device
        .interfaces()
        .map(|i| format!("{}:0x{:02x}", i.interface_number(), i.class()))
        .collect();
    format!(
        "pid {:04x} \"{}\" id 0x{:x} location 0x{:08x} interfaces [{}]",
        device.product_id(),
        device.product_string().unwrap_or("?"),
        device.registry_entry_id(),
        device.location_id(),
        interfaces.join(" ")
    )
}

/// Names of the IOReturn codes seen so far, for a readable report.
fn code_name(code: i32) -> String {
    let name = match code as u32 {
        0 => "success",
        0xe000_02bc => "error",
        0xe000_02bd => "no memory",
        0xe000_02be => "no resources",
        0xe000_02c0 => "no device",
        0xe000_02c1 => "not privileged",
        0xe000_02c2 => "bad argument",
        0xe000_02c5 => "exclusive access",
        0xe000_02c7 => "unsupported",
        0xe000_02c9 => "internal error",
        0xe000_02ca => "I/O error",
        0xe000_02cd => "not open",
        0xe000_02d6 => "timeout",
        0xe000_02e2 => "not permitted",
        _ => "",
    };
    format!("0x{:08x} {name}", code as u32).trim_end().to_string()
}

fn wait_for_any(r: &mut Report) -> Option<nusb::DeviceInfo> {
    let start = Instant::now();
    let mut warned = false;
    loop {
        if let Some(device) = sony_devices().into_iter().find(|d| {
            interface_of(d, crate::class::MSC).is_some()
                || interface_of(d, crate::class::PTP).is_some()
        }) {
            return Some(device);
        }
        if !warned && start.elapsed() > Duration::from_secs(15) {
            warned = true;
            r.line("no camera yet / noch keine Kamera: eingeschaltet? Datenkabel? USB-Verbindung?");
        }
        if start.elapsed() > Duration::from_secs(90) {
            r.line("no camera found, step skipped");
            return None;
        }
        std::thread::sleep(Duration::from_millis(300));
    }
}

/// Waits for a camera offering an interface of `class`, then for the
/// mass-storage driver to settle. A camera in MTP mode is first switched to
/// mass storage if `switch_from_mtp`.
fn wait_for(r: &mut Report, class: u8, switch_from_mtp: bool) -> Option<(nusb::DeviceInfo, u8)> {
    let start = Instant::now();
    let mut switched = false;
    let mut warned = false;
    loop {
        let devices = sony_devices();
        if let Some((info, number)) =
            devices.iter().find_map(|d| interface_of(d, class).map(|n| (d.clone(), n)))
        {
            if class == crate::class::MSC {
                let seen = Instant::now();
                while seen.elapsed() < DRIVER_SETTLE
                    && !driver_tree(info.registry_entry_id()).contains("MassStorage")
                {
                    std::thread::sleep(Duration::from_millis(200));
                }
            }
            std::thread::sleep(Duration::from_secs(1));
            return Some((info, number));
        }
        if switch_from_mtp
            && !switched
            && devices.iter().any(|d| interface_of(d, crate::class::PTP).is_some())
        {
            switched = true;
            r.line(format!("camera is in MTP, switching to mass storage: {}", switch_to_msc()));
            continue;
        }
        if !warned && start.elapsed() > Duration::from_secs(20) {
            warned = true;
            r.line(
                "waiting for the camera / Kamera fehlt: falls sie nicht wiederkommt, \
                 Kabel kurz ab- und wieder anstecken",
            );
        }
        if start.elapsed() > Duration::from_secs(120) {
            r.line("the camera did not come back");
            return None;
        }
        std::thread::sleep(Duration::from_millis(300));
    }
}

fn switch_to_msc() -> String {
    match discovery::list_cameras() {
        Ok(cameras) => match cameras.into_iter().find(|c| c.mode == CameraMode::Mtp) {
            Some(camera) => match discovery::switch_to_mass_storage(&camera) {
                Ok(()) => "requested".into(),
                Err(e) => format!("failed: {e:?}"),
            },
            None => "no MTP camera".into(),
        },
        Err(e) => format!("listing failed: {e:?}"),
    }
}

fn endpoints(info: &nusb::DeviceInfo, class: u8) -> std::result::Result<(u8, u8), String> {
    let device = info.open().wait().map_err(|e| format!("open: {e}"))?;
    let config = device.active_configuration().map_err(|e| format!("configuration: {e}"))?;
    crate::nusb_backend::find_bulk_interface(&config, class)
        .map(|b| (b.ep_in, b.ep_out))
        .ok_or_else(|| "no bulk pair".into())
}

fn dump_ioreg(r: &mut Report, registry_id: u64, label: &str) {
    let Some(service) = service_by_id(registry_id) else {
        r.line(format!("ioreg {label}: device gone"));
        return;
    };
    let mut name = [0 as c_char; 128];
    // SAFETY: the service is live until released; the buffer is an io_name_t.
    let name = unsafe {
        io_kit_sys::IORegistryEntryGetName(service, name.as_mut_ptr());
        IOObjectRelease(service);
        CStr::from_ptr(name.as_ptr()).to_string_lossy().into_owned()
    };
    let tree = shell("/usr/sbin/ioreg", &["-l", "-w0", "-r", "-n", &name]);
    // The camera's serial number has no place in a report that gets mailed around.
    let tree: Vec<String> = tree
        .lines()
        .map(|line| match line.split_once('=') {
            Some((key, _)) if key.to_ascii_lowercase().contains("serial") => {
                format!("{key}= <removed>")
            }
            _ => line.to_string(),
        })
        .collect();
    r.block(&format!("ioreg {label} ({name})"), &tree.join("\n"), 800, false);
    r.line(format!("ioreg {label} written to the report"));
}

/// What one experiment works on.
struct Target {
    id: u64,
    number: u8,
    ep_in: u8,
    ep_out: u8,
}

type Body<'a> = &'a mut dyn FnMut(&mut Report, &Target) -> String;
type Test<'a> = &'a mut dyn FnMut(&Target, &mut dyn Bulk) -> String;

/// One experiment: wait for the camera, unmount its card, run `body`, then
/// record the outcome and what the system logged meanwhile.
fn experiment(r: &mut Report, title: &str, class: u8, switch: bool, eps: (u8, u8), body: Body<'_>) {
    r.line(format!("=== {} / {title}", r.mode));
    let Some((info, number)) = wait_for(r, class, switch) else {
        r.result(title, "skipped, no camera".into());
        return;
    };
    let target = Target { id: info.registry_entry_id(), number, ep_in: eps.0, ep_out: eps.1 };
    r.line(format!("device: {}", describe(&info)));
    drop(info);
    r.line(format!("drivers: {}", driver_tree(target.id)));
    if let Some(disk) = bsd_disk(target.id) {
        r.line(format!("unmount {disk}: {}", diskutil(&["unmountDisk", "force"], &disk)));
    }
    let since = shell("/bin/date", &["+%Y-%m-%d %H:%M:%S"]);
    let outcome = body(r, &target);
    r.result(title, outcome);
    std::thread::sleep(Duration::from_secs(2));
    let after: Vec<String> = sony_devices().iter().map(describe).collect();
    r.line(format!("afterwards: {}", after.join(" | ")));
    let predicate = format!(
        "(processID == 0 AND (senderImagePath CONTAINS[c] \"usb\" OR eventMessage CONTAINS[c] \"usb\")) \
         OR processID == {}",
        std::process::id()
    );
    let log = shell(
        "/usr/bin/log",
        &[
            "show",
            "--style",
            "compact",
            "--info",
            "--debug",
            "--start",
            &since,
            "--predicate",
            &predicate,
        ],
    );
    r.block("system log", &log, 200, true);
}

fn suite(r: &mut Report, class: u8, switch: bool) {
    let Some((info, _)) = wait_for(r, class, switch) else { return };
    let eps = endpoints(&info, class).unwrap_or_else(|e| {
        r.line(format!("endpoints: {e}, assuming in 0x81 / out 0x02"));
        (0x81, 0x02)
    });
    r.line(format!("bulk endpoints: in 0x{:02x}, out 0x{:02x}", eps.0, eps.1));
    dump_ioreg(r, info.registry_entry_id(), "before");
    drop(info);
    let mode = if class == crate::class::MSC { CameraMode::MassStorage } else { CameraMode::Mtp };

    experiment(r, "app path (rc.9): read camera info", class, switch, eps, &mut |_, _| {
        production(mode)
    });
    if class != crate::class::MSC {
        return;
    }
    experiment(r, "capture, nusb: INQUIRY + camera info", class, switch, eps, &mut |r, t| {
        via_nusb(r, t, &mut |t, bulk| camera_info(t, bulk, None, false))
    });
    experiment(r, "capture, IOUSBHost: INQUIRY + camera info", class, switch, eps, &mut |r, t| {
        via_host(r, t, &mut |t, bulk| camera_info(t, bulk, None, false))
    });
    experiment(
        r,
        "capture, IOUSBHost: camera info, writes in 512-byte pieces",
        class,
        switch,
        eps,
        &mut |r, t| via_host(r, t, &mut |t, bulk| camera_info(t, bulk, Some(512), false)),
    );
    experiment(
        r,
        "capture, IOUSBHost: bulk-only reset, then camera info",
        class,
        switch,
        eps,
        &mut |r, t| via_host(r, t, &mut |t, bulk| camera_info(t, bulk, None, true)),
    );
}

/// The program's own way, as the GUI's broker runs it.
fn production(mode: CameraMode) -> String {
    let cameras = match discovery::list_cameras() {
        Ok(cameras) => cameras,
        Err(e) => return format!("listing failed: {e:?}"),
    };
    let Some(camera) = cameras.into_iter().find(|c| c.mode == mode) else {
        return format!("no camera in mode {}", mode.id());
    };
    match discovery::read_camera_info(&camera) {
        Ok(info) => format!(
            "OK, model {}, firmware {}",
            info.model_name.unwrap_or_default(),
            info.firmware.unwrap_or_default()
        ),
        Err(e) => format!("failed: {e:?}"),
    }
}

/// `[[class alloc] initWithIOService:options:queue:error:interestHandler:nil]`.
///
/// # Safety
///
/// `service` must be a live service of the kind `class` represents; `queue` a
/// dispatch queue or null.
unsafe fn open_object(
    class: &CStr,
    service: io_service_t,
    options: usize,
    queue: Id,
) -> std::result::Result<Id, i32> {
    // SAFETY: guaranteed by the caller.
    unsafe {
        let allocated = send!(Id; objc_getClass(class.as_ptr()), c"alloc");
        let mut error: Id = ptr::null_mut();
        let object = send!(Id; allocated, c"initWithIOService:options:queue:error:interestHandler:",
            service => io_service_t, options => usize, queue => Id,
            &mut error => *mut Id, ptr::null_mut() => Id);
        if object.is_null() { Err(error_code(error)) } else { Ok(object) }
    }
}

fn capture(
    registry_id: u64,
    options: usize,
    queue: Id,
) -> std::result::Result<HostInterface, String> {
    let _pool = Pool::new();
    let service = service_by_id(registry_id).ok_or("device gone")?;
    // SAFETY: the service is live until released; the device is owned by the
    // returned host, whose drop resets it.
    let device = unsafe {
        let device = open_object(c"IOUSBHostDevice", service, options, queue);
        IOObjectRelease(service);
        device
    };
    device
        .map(|device| HostInterface {
            device,
            interface: ptr::null_mut(),
            pipes: HashMap::new(),
            registry_id,
        })
        .map_err(|code| format!("capture (options {options}) refused: {}", code_name(code)))
}

fn open_interface(
    host: &mut HostInterface,
    number: u8,
    options: usize,
    queue: Id,
) -> std::result::Result<String, String> {
    let _pool = Pool::new();
    let start = Instant::now();
    let mut refusals: BTreeMap<String, u32> = BTreeMap::new();
    loop {
        match interface_service(host.registry_id, number) {
            Some(service) => {
                // SAFETY: the service is live until released; an opened
                // interface is owned by `host`, whose drop destroys it.
                let (id, busy, opened) = unsafe {
                    let (id, busy) = service_state(service);
                    let opened = open_object(c"IOUSBHostInterface", service, options, queue);
                    IOObjectRelease(service);
                    (id, busy, opened)
                };
                match opened {
                    Ok(interface) => {
                        host.interface = interface;
                        return Ok(format!(
                            "interface 0x{id:x} opened after {:.1}s",
                            start.elapsed().as_secs_f32()
                        ));
                    }
                    Err(code) => {
                        *refusals
                            .entry(format!("interface 0x{id:x} busy {busy}: {}", code_name(code)))
                            .or_default() += 1;
                    }
                }
            }
            None => *refusals.entry("no interface service".into()).or_default() += 1,
        }
        if start.elapsed() >= OPEN_WAIT {
            let list: Vec<String> = refusals.iter().map(|(k, n)| format!("{k} x{n}")).collect();
            return Err(format!("refused: {}", list.join("; ")));
        }
        std::thread::sleep(INTERFACE_POLL);
    }
}

/// Capture, reconfigure as the program does, open the interface through
/// IOUSBHost and run `test` on it.
fn via_host(r: &mut Report, t: &Target, test: Test<'_>) -> String {
    let mut host = match capture(t.id, INIT_DEVICE_CAPTURE, ptr::null_mut()) {
        Ok(host) => host,
        Err(e) => return e,
    };
    r.line(format!("captured; drivers: {}", driver_tree(t.id)));
    r.line(format!("reconfigure: {}", host.reconfigure()));
    match open_interface(&mut host, t.number, 0, ptr::null_mut()) {
        Ok(opened) => format!("{opened}; {}", test(t, &mut host)),
        Err(e) => e,
    }
}

/// Capture, reconfigure as the program does, claim the interface through
/// `nusb` and run `test` on it.
fn via_nusb(r: &mut Report, t: &Target, test: Test<'_>) -> String {
    let host = match capture(t.id, INIT_DEVICE_CAPTURE, ptr::null_mut()) {
        Ok(host) => host,
        Err(e) => return e,
    };
    r.line(format!("captured; drivers: {}", driver_tree(t.id)));
    r.line(format!("reconfigure: {}", host.reconfigure()));
    let outcome = match nusb_claim(host.registry_id, t.number) {
        Ok(mut backend) => format!("nusb claimed; {}", test(t, &mut backend)),
        Err(e) => format!("nusb {e}"),
    };
    drop(host);
    outcome
}

/// Open and claim through `nusb`, retrying for [`OPEN_WAIT`].
fn nusb_claim(registry_id: u64, number: u8) -> std::result::Result<NusbBackend, String> {
    let start = Instant::now();
    let mut errors: BTreeMap<String, u32> = BTreeMap::new();
    loop {
        let attempt = || -> std::result::Result<NusbBackend, String> {
            let info = nusb::list_devices()
                .wait()
                .map_err(|e| format!("list: {e}"))?
                .find(|d| d.registry_entry_id() == registry_id)
                .ok_or("device gone")?;
            let device = info.open().wait().map_err(|e| format!("open: {e}"))?;
            let interface =
                device.claim_interface(number).wait().map_err(|e| format!("claim: {e}"))?;
            Ok(NusbBackend::from_interface(interface))
        };
        match attempt() {
            Ok(backend) => return Ok(backend),
            Err(e) => *errors.entry(e).or_default() += 1,
        }
        if start.elapsed() >= OPEN_WAIT {
            let list: Vec<String> = errors.iter().map(|(k, n)| format!("{k} x{n}")).collect();
            return Err(format!("refused: {}", list.join("; ")));
        }
        std::thread::sleep(INTERFACE_POLL);
    }
}

/// Bulk transfers through either way.
trait Bulk {
    fn put(&mut self, ep: u8, data: &[u8]) -> Result<()>;
    fn get(&mut self, ep: u8, length: usize) -> Result<Vec<u8>>;
    fn halt(&mut self, ep: u8) -> Result<()>;
    /// Bulk-Only Mass Storage Reset on interface `number`.
    fn bot_reset(&mut self, number: u8) -> Result<()>;
}

impl Bulk for HostInterface {
    fn put(&mut self, ep: u8, data: &[u8]) -> Result<()> {
        self.write(ep, data, IO_TIMEOUT)
    }

    fn get(&mut self, ep: u8, length: usize) -> Result<Vec<u8>> {
        self.read(ep, length, IO_TIMEOUT)
    }

    fn halt(&mut self, ep: u8) -> Result<()> {
        self.clear_halt(ep)
    }

    fn bot_reset(&mut self, number: u8) -> Result<()> {
        self.control_out(0x21, 0xFF, 0, u16::from(number), &[], IO_TIMEOUT)
    }
}

impl Bulk for NusbBackend {
    fn put(&mut self, ep: u8, data: &[u8]) -> Result<()> {
        self.write(ep, data)
    }

    fn get(&mut self, ep: u8, length: usize) -> Result<Vec<u8>> {
        self.read(ep, length)
    }

    fn halt(&mut self, ep: u8) -> Result<()> {
        self.clear_halt(ep)
    }

    fn bot_reset(&mut self, _number: u8) -> Result<()> {
        self.reset()
    }
}

/// A [`Bulk`] way as the backend the protocol code expects, optionally cutting
/// every write into `pieces`-byte transfers.
struct Pieces<'a> {
    bulk: &'a mut dyn Bulk,
    pieces: Option<usize>,
}

impl UsbBackend for Pieces<'_> {
    fn read(&mut self, ep: u8, length: usize) -> Result<Vec<u8>> {
        self.bulk.get(ep, length)
    }

    fn write(&mut self, ep: u8, data: &[u8]) -> Result<()> {
        match self.pieces {
            Some(size) if data.len() > size => {
                data.chunks(size).try_for_each(|piece| self.bulk.put(ep, piece))
            }
            _ => self.bulk.put(ep, data),
        }
    }

    fn clear_halt(&mut self, ep: u8) -> Result<()> {
        self.bulk.halt(ep)
    }
}

/// SCSI INQUIRY, then the program's own camera-info command (an 8 KiB
/// data-out phase and an 8 KiB data-in phase), each reported on its own.
fn camera_info(t: &Target, bulk: &mut dyn Bulk, pieces: Option<usize>, reset: bool) -> String {
    let mut steps = Vec::new();
    if reset {
        let reset = bulk.bot_reset(t.number);
        let out = bulk.halt(t.ep_out);
        let r#in = bulk.halt(t.ep_in);
        steps.push(format!("bulk-only reset {reset:?}, clear halts {out:?} {:?}", r#in));
    }
    let mut msc = MscDevice::new(Pieces { bulk, pieces }, t.ep_in, t.ep_out);
    let started = Instant::now();
    steps.push(match msc.get_device_info() {
        Ok(info) => format!("INQUIRY OK: {} {}", info.manufacturer, info.model),
        Err(e) => format!("INQUIRY failed: {e:?}"),
    });
    let mut camera = SonyExtCmdCamera::new(msc);
    steps.push(match camera.get_camera_info() {
        Ok(info) => format!(
            "DATA OK: camera info {} {}, after {:.1}s",
            info.model_name,
            info.model_code,
            started.elapsed().as_secs_f32()
        ),
        Err(e) => {
            format!("camera info failed after {:.1}s: {e:?}", started.elapsed().as_secs_f32())
        }
    });
    steps.join("; ")
}
