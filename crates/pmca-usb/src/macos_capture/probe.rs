//! Diagnostic run for macOS: tries every known way into a Sony camera's USB
//! interfaces, in each USB mode the camera is set to, and writes every step to
//! a report. The `macos_probe` example starts it as root; the program itself
//! never calls it.

use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, Write as _};
use std::time::{Duration, Instant};

use nusb::MaybeFuture;

use super::*;
use crate::UsbBackend;
use crate::discovery::{self, CameraMode};
use crate::nusb_backend::NusbBackend;

/// Identifies the probe's logic in a report.
const REVISION: &str = "probe 1";

/// The report, written to the current directory.
const REPORT: &str = "pmca-macos-probe.txt";

/// How long each way to an interface is retried.
const OPEN_WAIT: Duration = Duration::from_secs(3);

/// Timeout of a single test transfer.
const IO_TIMEOUT: Duration = Duration::from_secs(5);

/// How long the mass-storage driver may take to return after a reset.
const DRIVER_SETTLE: Duration = Duration::from_secs(8);

/// `IOUSBHostObjectInitOptionsDeviceSeize`.
const INIT_DEVICE_SEIZE: usize = 1 << 1;

#[repr(C)]
struct Timespec {
    sec: u32,
    nsec: i32,
}

unsafe extern "C" {
    fn IOServiceWaitQuiet(service: io_service_t, wait: *mut Timespec) -> i32;
    fn dispatch_queue_create(label: *const c_char, attr: *const c_void) -> Id;
}

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
    // SAFETY: creates a serial queue that lives as long as the process.
    let queue = unsafe { dispatch_queue_create(c"pmca.probe".as_ptr(), ptr::null()) };

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
            suite(&mut r, crate::class::MSC, false, queue);
        } else if interface_of(&info, crate::class::PTP).is_some() {
            suite(&mut r, crate::class::PTP, false, queue);
            r.mode = format!("{english}, switched to mass storage");
            r.line(format!("########## {}", r.mode));
            suite(&mut r, crate::class::MSC, true, queue);
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
    r.block(&format!("ioreg {label} ({name})"), &tree, 800, false);
    r.line(format!("ioreg {label} written to the report"));
}

/// What one experiment works on.
struct Target {
    id: u64,
    location: u32,
    number: u8,
    class: u8,
    ep_in: u8,
    ep_out: u8,
}

type Body<'a> = &'a mut dyn FnMut(&mut Report, &Target) -> String;
type Prepare<'a> = &'a mut dyn FnMut(&mut Report, &mut HostInterface);

/// One experiment: wait for the camera, unmount its card, run `body`, then
/// record the outcome and what the system logged meanwhile.
fn experiment(r: &mut Report, title: &str, class: u8, switch: bool, eps: (u8, u8), body: Body<'_>) {
    r.line(format!("=== {} / {title}", r.mode));
    let Some((info, number)) = wait_for(r, class, switch) else {
        r.result(title, "skipped, no camera".into());
        return;
    };
    let target = Target {
        id: info.registry_entry_id(),
        location: info.location_id(),
        number,
        class,
        ep_in: eps.0,
        ep_out: eps.1,
    };
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

fn suite(r: &mut Report, class: u8, switch: bool, queue: Id) {
    let Some((info, _)) = wait_for(r, class, switch) else { return };
    let eps = endpoints(&info, class).unwrap_or_else(|e| {
        r.line(format!("endpoints: {e}, assuming in 0x81 / out 0x02"));
        (0x81, 0x02)
    });
    r.line(format!("bulk endpoints: in 0x{:02x}, out 0x{:02x}", eps.0, eps.1));
    dump_ioreg(r, info.registry_entry_id(), "before");
    drop(info);
    let mode = if class == crate::class::MSC { CameraMode::MassStorage } else { CameraMode::Mtp };
    let cap = INIT_DEVICE_CAPTURE;
    let nil: Id = ptr::null_mut();

    experiment(r, "app path (rc.8): read camera info", class, switch, eps, &mut |_, _| {
        production(mode)
    });
    experiment(r, "no capture: IOUSBHost interface, then nusb", class, switch, eps, &mut |r, t| {
        without_capture(r, t)
    });
    experiment(r, "capture: IOUSBHost interface", class, switch, eps, &mut |r, t| {
        via_host(r, t, cap, nil, &[0], &mut |r, host| dump_ioreg(r, host.registry_id, "captured"))
    });
    experiment(r, "capture, wait quiet: IOUSBHost interface", class, switch, eps, &mut |r, t| {
        via_host(r, t, cap, nil, &[0], &mut |r, host| {
            let quiet = wait_quiet(host.registry_id, t.number);
            r.line(format!("wait quiet: {quiet}"));
        })
    });
    experiment(r, "capture + seize: IOUSBHost interface", class, switch, eps, &mut |r, t| {
        via_host(r, t, cap | INIT_DEVICE_SEIZE, nil, &[0], &mut nothing)
    });
    experiment(r, "capture with queue: IOUSBHost interface", class, switch, eps, &mut |r, t| {
        via_host(r, t, cap, queue, &[0], &mut nothing)
    });
    experiment(r, "capture: interface options 1, 2", class, switch, eps, &mut |r, t| {
        via_host(r, t, cap, nil, &[1, 2], &mut nothing)
    });
    experiment(r, "capture, reconfigure: IOUSBHost interface", class, switch, eps, &mut |r, t| {
        via_host(r, t, cap, nil, &[0], &mut |r, host| {
            r.line(format!("reconfigure: {}", host.reconfigure()));
        })
    });
    experiment(r, "capture, device reset: IOUSBHost interface", class, switch, eps, &mut |r, t| {
        via_host(r, t, cap, nil, &[0], &mut |r, host| reset_device(r, host, t.location))
    });
    experiment(r, "capture: nusb claim", class, switch, eps, &mut |r, t| {
        via_nusb(r, t, &mut nothing)
    });
    experiment(r, "capture, reconfigure: nusb claim", class, switch, eps, &mut |r, t| {
        via_nusb(r, t, &mut |r, host| r.line(format!("reconfigure: {}", host.reconfigure())))
    });
    experiment(r, "capture, surrender: IOUSBHost interface", class, switch, eps, &mut |r, t| {
        via_host(r, t, cap, nil, &[0], &mut surrender)
    });
    experiment(r, "capture, surrender: nusb claim", class, switch, eps, &mut |r, t| {
        via_nusb(r, t, &mut surrender)
    });
    experiment(
        r,
        "capture, configure with matching: IOUSBHost interface",
        class,
        switch,
        eps,
        &mut |r, t| via_host(r, t, cap, nil, &[0], &mut configure_matching),
    );
}

fn nothing(_: &mut Report, _: &mut HostInterface) {}

fn surrender(r: &mut Report, host: &mut HostInterface) {
    host.surrender();
    std::thread::sleep(Duration::from_millis(500));
    r.line(format!("surrendered; drivers: {}", driver_tree(host.registry_id)));
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

/// Capture, `prepare`, then open the interface through IOUSBHost with each of
/// `interface_options` in turn and move data on it.
fn via_host(
    r: &mut Report,
    t: &Target,
    capture_options: usize,
    queue: Id,
    interface_options: &[usize],
    prepare: Prepare<'_>,
) -> String {
    let mut host = match capture(t.id, capture_options, queue) {
        Ok(host) => host,
        Err(e) => return e,
    };
    r.line(format!("captured; drivers: {}", driver_tree(t.id)));
    prepare(r, &mut host);
    let mut refused = Vec::new();
    for &options in interface_options {
        match open_interface(&mut host, t.number, options, queue) {
            Ok(opened) => {
                let test = exchange(t, &mut host);
                refused.push(format!("options {options}: {opened}; {test}"));
                break;
            }
            Err(e) => {
                r.line(format!("interface options {options}: {e}"));
                refused.push(format!("options {options}: {e}"));
            }
        }
    }
    refused.join(" | ")
}

/// Capture, `prepare`, then claim the interface through `nusb` and move data.
fn via_nusb(r: &mut Report, t: &Target, prepare: Prepare<'_>) -> String {
    let mut host = match capture(t.id, INIT_DEVICE_CAPTURE, ptr::null_mut()) {
        Ok(host) => host,
        Err(e) => return e,
    };
    r.line(format!("captured; drivers: {}", driver_tree(t.id)));
    prepare(r, &mut host);
    let outcome = match nusb_claim(host.registry_id, t.number) {
        Ok(mut backend) => format!("nusb claimed; {}", exchange(t, &mut backend)),
        Err(e) => format!("nusb {e}"),
    };
    drop(host);
    outcome
}

fn without_capture(r: &mut Report, t: &Target) -> String {
    if t.class == crate::class::PTP {
        crate::nusb_backend::evict_macos_ptp_daemons();
        std::thread::sleep(Duration::from_millis(300));
    }
    let mut host = HostInterface {
        device: ptr::null_mut(),
        interface: ptr::null_mut(),
        pipes: HashMap::new(),
        registry_id: t.id,
    };
    let host_outcome = match open_interface(&mut host, t.number, 0, ptr::null_mut()) {
        Ok(opened) => format!("{opened}; {}", exchange(t, &mut host)),
        Err(e) => e,
    };
    // Nothing was captured, so the drop must not reset the device.
    host.registry_id = 0;
    drop(host);
    r.line(format!("IOUSBHost: {host_outcome}"));
    if t.class == crate::class::PTP {
        crate::nusb_backend::evict_macos_ptp_daemons();
        std::thread::sleep(Duration::from_millis(300));
    }
    let nusb_outcome = match nusb_claim(t.id, t.number) {
        Ok(mut backend) => format!("claimed; {}", exchange(t, &mut backend)),
        Err(e) => e,
    };
    if let Some(disk) = bsd_disk(t.id) {
        r.line(format!("mount {disk}: {}", diskutil(&["mountDisk"], &disk)));
    }
    format!("IOUSBHost: {host_outcome} | nusb: {nusb_outcome}")
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
}

impl Bulk for HostInterface {
    fn put(&mut self, ep: u8, data: &[u8]) -> Result<()> {
        self.write(ep, data, IO_TIMEOUT)
    }

    fn get(&mut self, ep: u8, length: usize) -> Result<Vec<u8>> {
        self.read(ep, length, IO_TIMEOUT)
    }
}

impl Bulk for NusbBackend {
    fn put(&mut self, ep: u8, data: &[u8]) -> Result<()> {
        self.write(ep, data)
    }

    fn get(&mut self, ep: u8, length: usize) -> Result<Vec<u8>> {
        self.read(ep, length)
    }
}

/// One harmless request that proves data flows: TEST UNIT READY on mass
/// storage, GetDeviceInfo on PTP.
fn exchange(t: &Target, bulk: &mut dyn Bulk) -> String {
    if t.class == crate::class::MSC {
        let mut cbw = [0u8; 31];
        cbw[..4].copy_from_slice(b"USBC");
        cbw[4..8].copy_from_slice(&0x5052_4f42_u32.to_le_bytes());
        cbw[14] = 6;
        if let Err(e) = bulk.put(t.ep_out, &cbw) {
            return format!("TEST UNIT READY write failed: {e:?}");
        }
        return match bulk.get(t.ep_in, 512) {
            Ok(csw) if csw.len() == 13 && csw.starts_with(b"USBS") => {
                format!("DATA OK: TEST UNIT READY answered, status {}", csw[12])
            }
            Ok(other) => format!("TEST UNIT READY: unexpected reply {other:02x?}"),
            Err(e) => format!("TEST UNIT READY read failed: {e:?}"),
        };
    }
    let command = crate::mtp::pack_container(1, 0x1001, 0, &[]);
    if let Err(e) = bulk.put(t.ep_out, &command) {
        return format!("GetDeviceInfo write failed: {e:?}");
    }
    let mut data = match bulk.get(t.ep_in, 4096) {
        Ok(data) => data,
        Err(e) => return format!("GetDeviceInfo read failed: {e:?}"),
    };
    let header = |b: &[u8]| -> Option<(u32, u16, u16)> {
        Some((
            u32::from_le_bytes(b.get(..4)?.try_into().ok()?),
            u16::from_le_bytes(b.get(4..6)?.try_into().ok()?),
            u16::from_le_bytes(b.get(6..8)?.try_into().ok()?),
        ))
    };
    let Some((length, kind, _)) = header(&data) else {
        return format!("GetDeviceInfo: short reply {data:02x?}");
    };
    if kind == 2 {
        let mut reads = 0;
        while data.len() < length as usize && reads < 16 {
            match bulk.get(t.ep_in, 4096) {
                Ok(more) => data.extend(more),
                Err(e) => return format!("GetDeviceInfo data read failed: {e:?}"),
            }
            reads += 1;
        }
        let response = match bulk.get(t.ep_in, 512) {
            Ok(response) => response,
            Err(e) => return format!("GetDeviceInfo response read failed: {e:?}"),
        };
        return match header(&response) {
            Some((_, 3, code)) => {
                format!("DATA OK: GetDeviceInfo {} bytes, response 0x{code:04x}", data.len())
            }
            _ => format!("GetDeviceInfo: unexpected response {response:02x?}"),
        };
    }
    format!("GetDeviceInfo: unexpected container {:02x?}", &data[..data.len().min(16)])
}

fn wait_quiet(registry_id: u64, number: u8) -> String {
    let services = [
        ("device", service_by_id(registry_id)),
        ("interface", interface_service(registry_id, number)),
    ];
    let mut out = Vec::new();
    for (label, service) in services {
        match service {
            Some(service) => {
                let mut wait = Timespec { sec: 5, nsec: 0 };
                // SAFETY: the service is live until released.
                let code = unsafe {
                    let code = IOServiceWaitQuiet(service, &mut wait);
                    IOObjectRelease(service);
                    code
                };
                out.push(format!("{label} {}", code_name(code)));
            }
            None => out.push(format!("{label} missing")),
        }
    }
    out.join(", ")
}

fn reset_device(r: &mut Report, host: &mut HostInterface, location: u32) {
    let _pool = Pool::new();
    let mut error: Id = ptr::null_mut();
    // SAFETY: the captured device is live; `error` is an NSError or null.
    let code = unsafe {
        let ok = send!(i8; host.device, c"resetWithError:", &mut error => *mut Id);
        if ok != 0 { kIOReturnSuccess } else { error_code(error) }
    };
    std::thread::sleep(Duration::from_secs(2));
    let now = sony_devices().into_iter().find(|d| d.location_id() == location);
    r.line(format!(
        "reset: {}; now {}",
        code_name(code),
        now.as_ref().map(describe).unwrap_or_else(|| "gone".into())
    ));
    if let Some(device) = now {
        host.registry_id = device.registry_entry_id();
    }
    r.line(format!("drivers: {}", driver_tree(host.registry_id)));
}

fn configure_matching(r: &mut Report, host: &mut HostInterface) {
    let _pool = Pool::new();
    // SAFETY: the captured device is live; the descriptor pointer stays valid
    // while it is.
    let (off, value, on) = unsafe {
        let mut error: Id = ptr::null_mut();
        let descriptor = send!(*const u8; host.device, c"configurationDescriptorWithIndex:error:",
            0usize => usize, &mut error => *mut Id);
        let value = if descriptor.is_null() { 1 } else { *descriptor.add(5) };
        let configure = |value: u8, matching: bool| {
            let mut error: Id = ptr::null_mut();
            let ok = send!(i8; host.device, c"configureWithValue:matchInterfaces:error:",
                usize::from(value) => usize, i8::from(matching) => i8, &mut error => *mut Id);
            if ok != 0 { kIOReturnSuccess } else { error_code(error) }
        };
        (configure(0, false), value, configure(value, true))
    };
    std::thread::sleep(Duration::from_secs(1));
    r.line(format!(
        "configure 0: {}, {value} with matching: {}; drivers: {}",
        code_name(off),
        code_name(on),
        driver_tree(host.registry_id)
    ));
}
