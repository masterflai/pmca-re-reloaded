//! Reading and applying camera tweaks over a service-mode connection.
//!
//! The dangerous part — deciding which bytes of `Backup.bin` change — lives in
//! [`pmca_platform`] and is unit-tested there. This module only adds the
//! transport: put the camera into service mode, download the backup image,
//! hand it to the tweak layer, and write the resulting image(s) back.
//!
//! # Safety of the write path
//!
//! Every image written back is a full, checksummed `Backup.bin`. After each
//! write the image is read back and compared, mirroring the reference
//! implementation's `Cannot overwrite backup` guard: a silently rejected write
//! must never be reported as success, because the user would then believe a
//! setting was applied when it was not.

use pmca_i18n::Localizer;
use pmca_platform::{BackupPatch, Settings, tweaks};
use pmca_usb::UsbBackend;
use pmca_usb::nusb_backend::NusbBackend;
use pmca_usb::senser_camera::SenserCamera;

use crate::direct;
use crate::dto::{ProgressEvent, ProgressKind, TweakChange, TweakReport};
use crate::service_mode::{leave_service_mode, open_service_camera};
use crate::{Error, ProgressSink, Result};

/// Forces the per-property path, to test it on a camera that does not need it.
const FORCE_DIRECT_ENV: &str = "PMCA_TWEAKS_DIRECT";

/// Forces the whole-image path on a camera whose backup exceeds 1 MB, to test
/// writing such an image. [`FORCE_DIRECT_ENV`] wins if both are set.
const FORCE_IMAGE_ENV: &str = "PMCA_TWEAKS_IMAGE";

/// Protect mode the camera expects when the preset data is written back.
pub(crate) const BACKUP_PROTECT_MODE: u32 = 2;

/// Bytes at the start of the image that the camera regenerates (checksum,
/// counters); `bk rewrite` reports differences there separately.
pub(crate) const VERIFY_SKIP: usize = 0x100;

/// Report the tweaks the connected camera supports and their current state.
///
/// The camera's identifying values come from the same backup image and are
/// reported with them, so reading them costs no extra service-mode session.
pub fn read_tweaks(l: &Localizer, sink: &mut dyn ProgressSink) -> Result<TweakReport> {
    let mut session = TweakSession::open(l, sink)?;
    session.read(l, sink)
}

/// Turn a front-end's `ID=VALUE` request into the change the tweak layer takes.
///
/// Lives here rather than in the command line parser because the two names are
/// the tweak's own: a front-end should not have to know that "NTSC" is the
/// on-value of one particular property. Answering a typo with the accepted
/// names costs nothing here and saves a trip to the documentation.
///
/// The check is deliberately static — no camera is contacted — so a mistyped
/// argument is refused before anything is put into service mode.
pub fn resolve_choice(l: &Localizer, id: &str, value: &str) -> Result<bool> {
    match tweaks::presentation_of(id).map_err(|e| Error::Operation(e.to_string()))? {
        tweaks::Presentation::Choice { off, on } => {
            // Case-insensitive: these are proper nouns, and nobody should have
            // to remember which letters the toolkit capitalises.
            if value.eq_ignore_ascii_case(off) {
                Ok(false)
            } else if value.eq_ignore_ascii_case(on) {
                Ok(true)
            } else {
                Err(Error::Operation(l.tr_args(
                    "tweaks-set-unknown-value",
                    &[("value", value), ("options", &format!("{off}, {on}"))],
                )))
            }
        }
        tweaks::Presentation::Toggle => {
            Err(Error::Operation(l.tr_arg("tweaks-set-not-a-choice", "id", id)))
        }
    }
}

/// Reject an on/off request aimed at a tweak that is a choice.
///
/// A choice has no state that "enabled" names, so guessing at one would write
/// a value the user never asked for. Returns `Ok(())` for anything else,
/// including unknown ids: those are the tweak layer's to complain about, and
/// with more detail than this function has.
pub fn reject_toggle_of_choice(l: &Localizer, id: &str) -> Result<()> {
    if let Ok(tweaks::Presentation::Choice { off, on }) = tweaks::presentation_of(id) {
        return Err(Error::Operation(
            l.tr_args("tweaks-needs-set", &[("id", id), ("options", &format!("{off}, {on}"))]),
        ));
    }
    Ok(())
}

/// An authenticated service-mode connection kept open for tweak work.
///
/// Reaching service mode is expensive: a driver assignment, a mode switch, a
/// second driver assignment for the product id service mode enumerates under,
/// and two rounds of Windows re-enumeration between them — the better part of
/// a minute on the measured camera. A front-end that reads the tweak list and
/// then writes a change would pay that twice for one piece of work, with the
/// camera returned to mass storage in between only to be fetched back.
///
/// So the connection is a value the caller can hold. Whether that is worth
/// doing depends on the front-end: the console does one operation per process
/// and has nothing to reuse, while the window has two buttons over the same
/// camera and every reason to.
///
/// Holding one is not free. The camera stays in service mode and its mass
/// storage stays bound to WinUSB for as long as the session lives, so the
/// holder is responsible for [`close`](Self::close)-ing it when the user moves
/// on rather than when the program ends.
pub struct TweakSession {
    cam: SenserCamera<NusbBackend>,
    /// Cleared by [`Self::close`] so [`Drop`] does not send a second request.
    open: bool,
    /// Decided on first use; the backup's size does not change in a session.
    path: Option<BackupPath>,
    /// The per-property path comes from [`FORCE_DIRECT_ENV`], not the size.
    direct_forced: bool,
}

/// How a session reaches the camera's settings.
#[derive(Debug, Clone)]
enum BackupPath {
    /// The whole backup image, read and written in one piece.
    Image,
    /// One property at a time; carries the region from the status block.
    Direct(String),
}

impl TweakSession {
    /// Bring the attached camera into service mode and authenticate.
    pub fn open(l: &Localizer, sink: &mut dyn ProgressSink) -> Result<Self> {
        Ok(Self {
            cam: open_service_camera(l, sink)?,
            open: true,
            path: None,
            direct_forced: false,
        })
    }

    /// Asks the status block before anything else. Reading an image over
    /// 1 MB works, but writing one back is untested, so such a
    /// camera gets the per-property path unless [`FORCE_IMAGE_ENV`] is set.
    fn path(&mut self) -> BackupPath {
        if let Some(path) = &self.path {
            return path.clone();
        }
        let status = self.cam.get_backup_preset_data_status().ok();
        let region = status.as_deref().map(pmca_platform::direct::status_region);
        let large = status.as_deref().is_some_and(pmca_platform::direct::full_read_too_large);
        let forced = std::env::var_os(FORCE_DIRECT_ENV).is_some();
        let path = if (large && std::env::var_os(FORCE_IMAGE_ENV).is_none()) || forced {
            BackupPath::Direct(region.unwrap_or_default())
        } else {
            BackupPath::Image
        };
        self.direct_forced = forced && !large;
        self.path = Some(path.clone());
        path
    }

    /// Read the camera's tweakable settings, leaving them unmodified.
    pub fn read(&mut self, l: &Localizer, sink: &mut dyn ProgressSink) -> Result<TweakReport> {
        match self.path() {
            BackupPath::Image => {
                download(&mut self.cam, l, sink).map(|patch| report(&patch, false))
            }
            BackupPath::Direct(region) => {
                sink.emit(ProgressEvent::step(ProgressKind::Info, l.tr("tweaks-reading")));
                let listed = report(&direct::read(&mut self.cam, &region)?, true);
                Ok(TweakReport { direct_forced: self.direct_forced, ..listed })
            }
        }
    }

    /// Apply the changes through the whole backup image, whatever the camera's
    /// default path is.
    ///
    /// The answer to [`Error::DirectRefused`]: some models only take a
    /// read-only setting this way. The image read before the change is saved
    /// first, as for every whole-image write.
    pub fn apply_whole_image(
        &mut self,
        l: &Localizer,
        changes: &[TweakChange],
        sink: &mut dyn ProgressSink,
    ) -> Result<TweakReport> {
        apply_to_open_camera(&mut self.cam, l, changes, sink)
    }

    /// Apply the requested changes and report the resulting state.
    pub fn apply(
        &mut self,
        l: &Localizer,
        changes: &[TweakChange],
        sink: &mut dyn ProgressSink,
    ) -> Result<TweakReport> {
        match self.path() {
            BackupPath::Image => apply_to_open_camera(&mut self.cam, l, changes, sink),
            BackupPath::Direct(region) => {
                let mut patch = direct::read(&mut self.cam, &region)?;
                for change in changes {
                    tweaks::set_enabled(
                        &mut patch,
                        &change.id,
                        change.enabled,
                        change.accept_caution,
                    )
                    .map_err(|e| Error::Operation(e.to_string()))?;
                }
                let writes = patch.writes();
                if !writes.is_empty() {
                    // Saved before the first write: there is no full image to
                    // fall back on, so this record is the only way back.
                    let dir = direct::rollback_dir()
                        .ok_or_else(|| Error::Operation(l.tr("tweaks-rollback-unsaved")))?;
                    direct::remember(&dir, &direct::camera_key(&patch), &writes).map_err(|e| {
                        Error::Operation(format!("{} ({e})", l.tr("tweaks-rollback-unsaved")))
                    })?;
                    write_direct(&mut self.cam, l, &writes, sink)?;
                }
                self.read(l, sink)
            }
        }
    }

    /// Write back the values saved before the first per-property change.
    pub fn undo(&mut self, l: &Localizer, sink: &mut dyn ProgressSink) -> Result<TweakReport> {
        let region = match self.path() {
            BackupPath::Direct(region) => region,
            BackupPath::Image => self
                .cam
                .get_backup_preset_data_status()
                .map(|s| pmca_platform::direct::status_region(&s))
                .unwrap_or_default(),
        };
        let key = direct::camera_key(&direct::read(&mut self.cam, &region)?);
        let dir = direct::rollback_dir();
        let record = dir
            .as_deref()
            .and_then(|dir| direct::load(dir, &key))
            .ok_or_else(|| Error::Operation(l.tr("tweaks-undo-none")))?;
        let writes = direct::undo_writes(&mut self.cam, &record);
        write_direct(&mut self.cam, l, &writes, sink)?;
        if let Some(dir) = dir {
            direct::forget(&dir, &key);
        }
        self.read(l, sink)
    }

    /// Leave service mode and release the driver binding that reached it.
    ///
    /// Safe to call more than once: the paths that have to call it — the end
    /// of a one-shot operation, a failure part-way through, and the window
    /// being closed — cannot all know whether one of the others got there
    /// first.
    pub fn close(&mut self) {
        if std::mem::take(&mut self.open) {
            leave_service_mode(&mut self.cam);
        }
    }
}

impl Drop for TweakSession {
    fn drop(&mut self) {
        self.close();
    }
}

/// Run one tweak operation, reusing a held session where there is one.
///
/// The whole point of holding a session is that the second operation skips the
/// mode switch and both driver assignments — the check for "is the camera
/// already where we need it" is this function, and the answer is yes exactly
/// when `slot` still holds a live session.
///
/// A held session can also have gone stale, because the camera may have been
/// unplugged since it was opened and its handle then refers to a device that
/// is no longer there. That is indistinguishable from a genuine failure until
/// it is tried, so a *reused* session that fails is discarded and the
/// operation is attempted once more on a fresh one. A session opened by this
/// call gets no such second chance: there is nothing stale about it, and
/// retrying would only make the user wait twice for the same failure.
///
/// Retrying is safe for both operations. Reading changes nothing, and applying
/// re-downloads the image first and sets each tweak to the value asked for
/// rather than toggling it, so running it twice lands in the same place as
/// running it once.
pub fn with_session<T>(
    slot: &mut Option<TweakSession>,
    l: &Localizer,
    sink: &mut dyn ProgressSink,
    operation: impl Fn(&mut TweakSession, &mut dyn ProgressSink) -> Result<T>,
) -> Result<T> {
    if let Some(session) = slot.as_mut() {
        match operation(session, sink) {
            Ok(value) => return Ok(value),
            // Not reported: the camera is about to be reached again, and a
            // failure the retry recovers from is not one the user has to act
            // on. If the retry fails too, its error is the one that surfaces.
            Err(_) => *slot = None,
        }
    }

    let session = slot.insert(TweakSession::open(l, sink)?);
    operation(session, sink).inspect_err(|_| *slot = None)
}

/// Turn the camera's settings into the report every entry point returns.
fn report(settings: &impl Settings, direct: bool) -> TweakReport {
    let key = direct::camera_key(settings);
    TweakReport {
        properties: pmca_platform::identity::list(settings).into_iter().map(Into::into).collect(),
        tweaks: tweaks::list(settings).into_iter().map(Into::into).collect(),
        direct,
        direct_forced: false,
        can_undo: direct::rollback_dir().is_some_and(|dir| direct::load(&dir, &key).is_some()),
    }
}

/// Write single properties and translate a failure into what the user can do.
fn write_direct(
    cam: &mut SenserCamera<NusbBackend>,
    l: &Localizer,
    writes: &[pmca_platform::direct::DirectWrite],
    sink: &mut dyn ProgressSink,
) -> Result<()> {
    let total = writes.len().max(1);
    let message = l.tr("tweaks-writing");
    let mut progress = |done: usize| {
        sink.emit(ProgressEvent::progress(message.clone(), ((done * 100) / total) as u8));
    };
    direct::write(cam, writes, &mut progress).map_err(|e| match e {
        direct::WriteError::Refused { cause, unchanged: true } => {
            Error::DirectRefused(l.tr_arg("tweaks-direct-refused", "detail", &cause.to_string()))
        }
        direct::WriteError::Refused { cause, unchanged: false } => {
            Error::Operation(l.tr_arg("tweaks-direct-revert-failed", "detail", &cause.to_string()))
        }
        direct::WriteError::Usb(e) => Error::from(e),
    })?;
    sink.emit(ProgressEvent::step(ProgressKind::Done, l.tr("tweaks-direct-restart")));
    Ok(())
}

/// Apply the requested tweak changes to the connected camera.
///
/// Note that a change whose value already matches the camera's is **not** a
/// no-op: the whole backup image is rebuilt and written back regardless,
/// exactly as `BackupPatchDataInterface.apply` does. Only an empty change list
/// leaves the flash untouched.
pub fn apply_tweaks(
    l: &Localizer,
    changes: &[TweakChange],
    sink: &mut dyn ProgressSink,
) -> Result<TweakReport> {
    let mut session = TweakSession::open(l, sink)?;
    session.apply(l, changes, sink)
}

/// [`apply_tweaks`] through the whole backup image; see
/// [`TweakSession::apply_whole_image`].
pub fn apply_tweaks_whole_image(
    l: &Localizer,
    changes: &[TweakChange],
    sink: &mut dyn ProgressSink,
) -> Result<TweakReport> {
    let mut session = TweakSession::open(l, sink)?;
    session.apply_whole_image(l, changes, sink)
}

/// Write back the values saved before the first per-property change.
pub fn undo_tweaks(l: &Localizer, sink: &mut dyn ProgressSink) -> Result<TweakReport> {
    let mut session = TweakSession::open(l, sink)?;
    session.undo(l, sink)
}

/// Apply the changes to an already-authenticated service-mode camera.
fn apply_to_open_camera(
    cam: &mut SenserCamera<NusbBackend>,
    l: &Localizer,
    changes: &[TweakChange],
    sink: &mut dyn ProgressSink,
) -> Result<TweakReport> {
    let mut patch = download(cam, l, sink)?;

    for change in changes {
        tweaks::set_enabled(&mut patch, &change.id, change.enabled, change.accept_caution)
            .map_err(|e| Error::Operation(e.to_string()))?;
    }

    let images = patch.build_images(true).map_err(|e| Error::Operation(e.to_string()))?;

    if !images.is_empty() {
        let saved = save_original(&patch)
            .map_err(|e| Error::Operation(format!("{} ({e})", l.tr("tweaks-rollback-unsaved"))))?;
        sink.emit(ProgressEvent::step(
            ProgressKind::Info,
            l.tr_arg("tweaks-image-saved", "path", &saved.display().to_string()),
        ));
    }

    for (i, image) in images.iter().enumerate() {
        sink.emit(ProgressEvent::progress(
            l.tr("tweaks-writing"),
            ((i * 100) / images.len().max(1)) as u8,
        ));
        if let Err(e) = upload(cam, l, &patch, image) {
            // The first of two images clears read-only bits and only the second
            // puts them back, so a stop in between must not leave them cleared.
            if images.len() > 1 {
                let _ = restore_attributes(cam, patch.original());
            }
            return Err(e);
        }
    }

    // Persist to flash, otherwise the change is lost on the next power cycle.
    if !images.is_empty() {
        cam.save_backup(0)?;
        // The per-property path's record no longer holds the values from
        // before the first change once a whole image has been written.
        if let Some(dir) = direct::rollback_dir() {
            direct::forget(&dir, &direct::camera_key(&patch));
        }
    }

    sink.emit(ProgressEvent::step(ProgressKind::Done, l.tr("tweaks-done")));

    let patch = download(cam, l, sink)?;
    Ok(report(&patch, false))
}

/// How many images taken before a whole-image write are kept per camera.
const IMAGES_KEPT: usize = 5;

/// Save the image a whole-image write starts from, next to the per-property
/// rollback records, and drop the oldest beyond [`IMAGES_KEPT`].
pub(crate) fn save_original(patch: &BackupPatch) -> std::io::Result<std::path::PathBuf> {
    let dir = direct::rollback_dir().ok_or_else(|| std::io::Error::other("no user data folder"))?;
    std::fs::create_dir_all(&dir)?;
    let prefix = format!("{}-", direct::camera_key(patch));
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or_default();
    let path = dir.join(format!("{prefix}{stamp}.bin"));
    std::fs::write(&path, patch.original())?;

    let mut saved: Vec<std::path::PathBuf> = std::fs::read_dir(&dir)?
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .filter(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.starts_with(&prefix) && n.ends_with(".bin"))
        })
        .collect();
    // Same-length seconds since 1970, so the names sort by age.
    saved.sort();
    let excess = saved.len().saturating_sub(IMAGES_KEPT);
    for old in &saved[..excess] {
        let _ = std::fs::remove_file(old);
    }
    Ok(path)
}

/// Download the camera's backup image and parse it.
pub(crate) fn download<B: UsbBackend>(
    cam: &mut SenserCamera<B>,
    l: &Localizer,
    sink: &mut dyn ProgressSink,
) -> Result<BackupPatch> {
    sink.emit(ProgressEvent::step(ProgressKind::Info, l.tr("tweaks-reading")));
    // A stalled full read leaves the camera unresponsive until its battery is
    // pulled (upstream #744), and nothing in the raw error says so.
    let image = cam.get_backup_preset_data(true).map_err(|e| {
        Error::Operation(l.tr_arg("tweaks-full-read-failed", "detail", &e.to_string()))
    })?;
    BackupPatch::new(image).map_err(|e| Error::Operation(e.to_string()))
}

/// Write one complete backup image back and verify that the staged changes
/// stuck.
pub(crate) fn upload<B: UsbBackend>(
    cam: &mut SenserCamera<B>,
    l: &Localizer,
    patch: &BackupPatch,
    image: &[u8],
) -> Result<()> {
    cam.set_backup_preset_data(BACKUP_PROTECT_MODE, image)?;

    let written = cam.get_backup_preset_data(true)?;
    let stored = written.len() == image.len()
        && patch.stored(image, &written).map_err(|e| Error::Operation(e.to_string()))?;
    if !stored {
        return Err(Error::Operation(l.tr("tweaks-write-failed")));
    }
    Ok(())
}

/// Set every property attribute back to the one in `reference`, writing the
/// image only if one differs. Returns the changes as `(id, was, now)`.
pub(crate) fn restore_attributes<B: UsbBackend>(
    cam: &mut SenserCamera<B>,
    reference: &[u8],
) -> Result<Vec<pmca_platform::AttributeChange>> {
    let current = cam.get_backup_preset_data(true)?;
    let (image, changed) = pmca_platform::restore_attributes(&current, reference)
        .map_err(|e| Error::Operation(e.to_string()))?;
    if !changed.is_empty() {
        cam.set_backup_preset_data(BACKUP_PROTECT_MODE, &image)?;
        cam.save_backup(0)?;
    }
    Ok(changed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dto::TweakState;

    #[test]
    fn tweak_state_maps_ids_to_fluent_keys() {
        let state: TweakState = pmca_platform::tweaks::TweakStatus {
            id: "recLimit4k".into(),
            enabled: true,
            value: "9h 06m 07s".into(),
            presentation: pmca_platform::tweaks::Presentation::Toggle,
            caution: None,
        }
        .into();
        assert_eq!(state.label_key, "tweak-rec-limit-4k");
        assert!(!Localizer::new("en-US").tr(&state.label_key).is_empty());
        assert_eq!(state.caution_key, None);
        assert_eq!(state.choice, None);
        assert_eq!(state.note_key, None);
    }

    #[test]
    fn a_choice_arrives_with_both_alternatives_named() {
        // The front-end must be able to label the two options without knowing
        // that this particular tweak is about video systems.
        let state: TweakState = pmca_platform::tweaks::TweakStatus {
            id: "palNtsc".into(),
            enabled: false,
            value: "PAL".into(),
            presentation: pmca_platform::tweaks::Presentation::Choice { off: "PAL", on: "NTSC" },
            caution: None,
        }
        .into();
        let choice = state.choice.unwrap();
        let l = Localizer::new("en-US");
        assert_eq!(l.tr(&choice.off_key), "PAL");
        assert_eq!(l.tr(&choice.on_key), "NTSC");
        // Changing the video system makes the camera ask for the card to be
        // reformatted, which is worth knowing before the change rather than
        // after it. Nothing to accept, so it is a note and not a caution.
        assert!(!l.tr(&state.note_key.unwrap()).is_empty());
        assert_eq!(state.caution_key, None);
    }

    #[test]
    fn a_caution_is_carried_across_as_a_translatable_key() {
        // The front-end must never have to know which regions are affected;
        // it only has to render whatever key arrives.
        let state: TweakState = pmca_platform::tweaks::TweakStatus {
            id: "language".into(),
            enabled: false,
            value: "33 / 35 language slots enabled".into(),
            presentation: pmca_platform::tweaks::Presentation::Toggle,
            caution: Some(pmca_platform::tweaks::Caution::UnsupportedLanguages),
        }
        .into();
        let key = state.caution_key.unwrap();
        assert!(!Localizer::new("en-US").tr(&key).is_empty());
    }

    #[test]
    fn a_named_state_resolves_whatever_way_it_is_spelled() {
        let l = Localizer::new("en-US");
        assert!(!resolve_choice(&l, "palNtsc", "PAL").unwrap());
        assert!(resolve_choice(&l, "palNtsc", "ntsc").unwrap());
    }

    #[test]
    fn an_unknown_state_is_answered_with_the_accepted_ones() {
        // A refusal that does not say what would have worked sends the user
        // to the documentation for something the toolkit already knows.
        let message =
            resolve_choice(&Localizer::new("en-US"), "palNtsc", "SECAM").unwrap_err().to_string();
        assert!(message.contains("SECAM"), "{message}");
        assert!(message.contains("PAL") && message.contains("NTSC"), "{message}");
    }

    #[test]
    fn a_switch_cannot_be_set_and_a_choice_cannot_be_toggled() {
        let l = Localizer::new("en-US");
        assert!(resolve_choice(&l, "recLimit", "PAL").is_err());
        assert!(reject_toggle_of_choice(&l, "palNtsc").is_err());
        assert!(reject_toggle_of_choice(&l, "recLimit").is_ok());
        // Unknown ids belong to the tweak layer, which explains them better.
        assert!(reject_toggle_of_choice(&l, "nonesuch").is_ok());
    }
}
