//! macOS: tweak operations in an elevated copy of the program.
//!
//! macOS hands a mass-storage camera only to root, and the capture that does
//! so belongs to the process that made it. The window runs as the user, so it
//! cannot hold the camera itself. It starts its own executable with
//! administrator rights for the whole operation instead, through the system's
//! password dialog, which macOS does not show again for a few minutes. Progress
//! and the result come back through a file the window creates and reads while
//! the helper writes to it.

use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

use pmca_i18n::Localizer;
use serde::{Deserialize, Serialize};

use crate::dto::{ProgressEvent, TweakChange, TweakReport};
use crate::{Error, ProgressSink};

/// First argument that turns the executable into the elevated tweak helper.
pub const HELPER_FLAG: &str = "--pmca-elevated-tweaks";

/// What the helper is asked to do; mirrors the four tweak entry points.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "kebab-case")]
pub enum TweakOperation {
    /// [`crate::tweaks::read_tweaks`].
    Read,
    /// [`crate::tweaks::apply_tweaks`].
    Apply {
        /// The changes to make.
        changes: Vec<TweakChange>,
    },
    /// [`crate::tweaks::apply_tweaks_whole_image`].
    ApplyWholeImage {
        /// The changes to make.
        changes: Vec<TweakChange>,
    },
    /// [`crate::tweaks::undo_tweaks`].
    Undo,
}

/// A failure as the helper reported it, flattened to what a front-end shows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RemoteError {
    /// [`Error::code`].
    pub code: String,
    /// The error's own wording.
    pub message: String,
    /// [`Error::detail`].
    pub detail: Option<String>,
    /// [`Error::recovery`].
    pub recovery: Option<String>,
}

impl From<&Error> for RemoteError {
    fn from(e: &Error) -> Self {
        Self {
            code: e.code().to_string(),
            message: e.to_string(),
            detail: e.detail(),
            recovery: e.recovery().map(str::to_string),
        }
    }
}

/// How an elevated operation failed.
#[derive(Debug)]
pub enum ElevatedError {
    /// The helper ran and the operation failed there.
    Helper(RemoteError),
    /// The helper could not be started or did not answer.
    Local(Error),
}

/// One line of the progress file.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum Line {
    Progress(ProgressEvent),
    Done(TweakReport),
    Failed(RemoteError),
}

/// Run as the helper if `args` (without the program name) ask for it.
///
/// Returns the exit code then; `None` means a normal start.
pub fn run_helper_if_requested(args: &[String]) -> Option<i32> {
    if args.first().map(String::as_str) != Some(HELPER_FLAG) {
        return None;
    }
    // A malformed request must not fall through to starting the window as root.
    let [_, locale, path, operation] = args else { return Some(2) };
    Some(run_helper(locale, Path::new(path), operation))
}

fn run_helper(locale: &str, path: &Path, operation: &str) -> i32 {
    let _ownership = crate::ownership::OwnershipGuard::new();
    let Some(mut out) = open_progress_file(path) else { return 2 };
    crate::keys::load_from_disk();
    let l = Localizer::new(locale);

    let outcome = match serde_json::from_str::<TweakOperation>(operation) {
        Ok(operation) => {
            let mut sink = |event: ProgressEvent| write_line(&mut out, &Line::Progress(event));
            perform(&l, operation, &mut sink)
        }
        Err(e) => Err(Error::Json(e)),
    };
    let last = match outcome {
        Ok(report) => Line::Done(report),
        Err(e) => Line::Failed((&e).into()),
    };
    write_line(&mut out, &last);
    0
}

/// The file the window created, and only that: never a link, and owned by the
/// user the helper was started for.
fn open_progress_file(path: &Path) -> Option<std::fs::File> {
    let meta = std::fs::symlink_metadata(path).ok()?;
    if !meta.file_type().is_file() {
        return None;
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        let owner = std::env::var("SUDO_UID").ok()?.parse::<u32>().ok()?;
        if meta.uid() != owner {
            return None;
        }
    }
    std::fs::OpenOptions::new().append(true).open(path).ok()
}

fn write_line(out: &mut std::fs::File, line: &Line) {
    if let Ok(mut text) = serde_json::to_string(line) {
        text.push('\n');
        let _ = out.write_all(text.as_bytes());
    }
}

fn perform(
    l: &Localizer,
    operation: TweakOperation,
    sink: &mut dyn ProgressSink,
) -> crate::Result<TweakReport> {
    match operation {
        TweakOperation::Read => crate::tweaks::read_tweaks(l, sink),
        TweakOperation::Apply { changes } => crate::tweaks::apply_tweaks(l, &changes, sink),
        TweakOperation::ApplyWholeImage { changes } => {
            crate::tweaks::apply_tweaks_whole_image(l, &changes, sink)
        }
        TweakOperation::Undo => crate::tweaks::undo_tweaks(l, sink),
    }
}

/// Run `operation` in the elevated helper and relay its progress to `sink`.
#[cfg(target_os = "macos")]
pub fn run_tweak_operation(
    l: &Localizer,
    locale: &str,
    operation: &TweakOperation,
    sink: &mut dyn ProgressSink,
) -> Result<TweakReport, ElevatedError> {
    use std::os::unix::fs::MetadataExt;

    let local = |e: std::io::Error| ElevatedError::Local(Error::Io(e));
    let exe = std::env::current_exe().map_err(local)?;
    let home = std::env::var_os("HOME")
        .map(PathBuf::from)
        .ok_or_else(|| local(std::io::Error::other("HOME is not set")))?;
    let owner = std::fs::metadata(&home).map_err(local)?;
    let request = serde_json::to_string(operation).map_err(|e| ElevatedError::Local(e.into()))?;

    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or_default();
    let file =
        std::env::temp_dir().join(format!("pmca-elevated-{}-{stamp}.jsonl", std::process::id()));
    std::fs::File::create(&file).map_err(local)?;

    let args = osascript_args(&Helper {
        home: &home,
        uid: owner.uid(),
        gid: owner.gid(),
        prompt: &l.tr("macos-admin-prompt"),
        exe: &exe,
        locale,
        file: &file,
        request: &request,
    });
    let spawned = std::process::Command::new("/usr/bin/osascript")
        .args(args)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::piped())
        .spawn();
    let mut child = match spawned {
        Ok(child) => child,
        Err(e) => {
            let _ = std::fs::remove_file(&file);
            return Err(local(e));
        }
    };

    let mut tail = Tail::new(&file);
    let mut outcome = None;
    loop {
        let finished = child.try_wait().map_err(local)?.is_some();
        for line in tail.read_lines() {
            match serde_json::from_str::<Line>(&line) {
                Ok(Line::Progress(event)) => sink.emit(event),
                Ok(Line::Done(report)) => outcome = Some(Ok(report)),
                Ok(Line::Failed(e)) => outcome = Some(Err(ElevatedError::Helper(e))),
                Err(_) => {}
            }
        }
        if finished {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(200));
    }
    let _ = std::fs::remove_file(&file);

    if let Some(outcome) = outcome {
        return outcome;
    }
    let mut stderr = String::new();
    if let Some(mut pipe) = child.stderr.take() {
        let _ = pipe.read_to_string(&mut stderr);
    }
    // osascript reports a cancelled password dialog as AppleScript error -128.
    let message = if stderr.contains("-128") {
        l.tr("macos-admin-declined")
    } else {
        format!("{} ({})", l.tr("macos-admin-failed"), stderr.trim())
    };
    Err(ElevatedError::Local(Error::Operation(message)))
}

/// Everything the elevated command line is built from.
#[cfg_attr(not(target_os = "macos"), allow(dead_code))]
struct Helper<'a> {
    home: &'a Path,
    uid: u32,
    gid: u32,
    prompt: &'a str,
    exe: &'a Path,
    locale: &'a str,
    file: &'a Path,
    request: &'a str,
}

/// The AppleScript that runs the helper as root. Every value arrives as an
/// argument and is quoted by AppleScript itself, so none can break out of the
/// shell command. A service-mode session can take minutes, longer than
/// AppleScript's default timeout.
const SCRIPT: [&str; 9] = [
    "on run argv",
    "set cmd to \"HOME=\" & quoted form of item 1 of argv & \" SUDO_UID=\" & quoted form of item 2 of argv & \" SUDO_GID=\" & quoted form of item 3 of argv",
    "repeat with i from 5 to count of argv",
    "set cmd to cmd & \" \" & quoted form of item i of argv",
    "end repeat",
    "with timeout of 3600 seconds",
    "do shell script cmd with prompt (item 4 of argv) with administrator privileges",
    "end timeout",
    "end run",
];

#[cfg_attr(not(target_os = "macos"), allow(dead_code))]
fn osascript_args(h: &Helper) -> Vec<std::ffi::OsString> {
    let mut args: Vec<std::ffi::OsString> = Vec::new();
    for line in SCRIPT {
        args.push("-e".into());
        args.push(line.into());
    }
    args.push(h.home.into());
    args.push(h.uid.to_string().into());
    args.push(h.gid.to_string().into());
    args.push(h.prompt.into());
    args.push(h.exe.into());
    args.push(HELPER_FLAG.into());
    args.push(h.locale.into());
    args.push(h.file.into());
    args.push(h.request.into());
    args
}

/// Reads the lines appended to a file since the last call.
#[cfg_attr(not(target_os = "macos"), allow(dead_code))]
struct Tail {
    path: PathBuf,
    offset: u64,
    partial: String,
}

#[cfg_attr(not(target_os = "macos"), allow(dead_code))]
impl Tail {
    fn new(path: &Path) -> Self {
        Self { path: path.to_path_buf(), offset: 0, partial: String::new() }
    }

    /// Complete lines written since the last call; a line still being written
    /// is kept for the next one.
    fn read_lines(&mut self) -> Vec<String> {
        let mut text = String::new();
        if let Ok(mut file) = std::fs::File::open(&self.path)
            && file.seek(SeekFrom::Start(self.offset)).is_ok()
            && let Ok(read) = file.read_to_string(&mut text)
        {
            self.offset += read as u64;
        }
        self.partial.push_str(&text);
        let Some(end) = self.partial.rfind('\n') else { return Vec::new() };
        let complete: String = self.partial.drain(..=end).collect();
        complete.lines().filter(|l| !l.is_empty()).map(str::to_string).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dto::ProgressKind;

    #[test]
    fn a_line_survives_the_file() {
        let lines = [
            Line::Progress(ProgressEvent::step(ProgressKind::Info, "reading")),
            Line::Done(TweakReport::default()),
            Line::Failed((&Error::DirectRefused("refused".into())).into()),
        ];
        for line in lines {
            let text = serde_json::to_string(&line).unwrap();
            assert!(!text.contains('\n'));
            assert_eq!(serde_json::from_str::<Line>(&text).unwrap(), line);
        }
    }

    #[test]
    fn a_refusal_keeps_its_recovery_across_the_helper() {
        let remote: RemoteError = (&Error::DirectRefused("refused".into())).into();
        assert_eq!(remote.recovery.as_deref(), Some(crate::RECOVERY_WHOLE_IMAGE));
        assert_eq!(remote.code, "operation");
    }

    #[test]
    fn only_the_flag_makes_a_helper_and_a_bad_request_never_starts_the_window() {
        let args = |a: &[&str]| a.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        assert_eq!(run_helper_if_requested(&args(&[])), None);
        assert_eq!(run_helper_if_requested(&args(&["--lang", "de-DE"])), None);
        assert_eq!(run_helper_if_requested(&args(&[HELPER_FLAG])), Some(2));
        assert_eq!(run_helper_if_requested(&args(&[HELPER_FLAG, "a", "b", "c", "d"])), Some(2));
    }

    #[test]
    fn every_value_is_an_argument_not_part_of_the_script() {
        let helper = Helper {
            home: Path::new("/Users/ann"),
            uid: 501,
            gid: 20,
            prompt: "Allow?",
            exe: Path::new("/Applications/X.app/Contents/MacOS/x"),
            locale: "de-DE",
            file: Path::new("/tmp/p.jsonl"),
            request: r#"{"op":"read"}"#,
        };
        let args = osascript_args(&helper);
        let script: Vec<_> = args.iter().skip(1).step_by(2).take(SCRIPT.len()).collect();
        assert!(script.iter().zip(SCRIPT).all(|(a, s)| *a == s));
        let rest: Vec<_> = args[SCRIPT.len() * 2..].iter().map(|a| a.to_string_lossy()).collect();
        assert_eq!(
            rest,
            [
                "/Users/ann",
                "501",
                "20",
                "Allow?",
                "/Applications/X.app/Contents/MacOS/x",
                HELPER_FLAG,
                "de-DE",
                "/tmp/p.jsonl",
                r#"{"op":"read"}"#,
            ]
        );
    }

    #[test]
    fn the_tail_returns_only_complete_lines() {
        let path = std::env::temp_dir().join(format!("pmca-tail-test-{}", std::process::id()));
        std::fs::write(&path, "one\ntw").unwrap();
        let mut tail = Tail::new(&path);
        assert_eq!(tail.read_lines(), ["one"]);
        let mut file = std::fs::OpenOptions::new().append(true).open(&path).unwrap();
        file.write_all(b"o\nthree\n").unwrap();
        assert_eq!(tail.read_lines(), ["two", "three"]);
        assert!(tail.read_lines().is_empty());
        let _ = std::fs::remove_file(&path);
    }
}
