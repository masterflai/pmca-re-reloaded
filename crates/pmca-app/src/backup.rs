//! Saving the camera's whole `Backup.bin` to a file and writing one back.
//!
//! This is the raw image, not a firmware `.dat`: the same bytes the tweak and
//! analysis paths read, written out in one piece so a user can keep a copy
//! before changing anything and put it back if a change goes wrong.
//!
//! # Size
//!
//! Neither direction consults the image size. An ILCE-7CR moves 2,390,264
//! bytes both ways: read out and verified (hardware test 5.37), written back
//! unchanged (5.21) and written back with a change and reverted (5.22). The
//! cameras that cannot do this fail outright whatever their image measures,
//! and the size check that used to stand here refused working cameras without
//! helping the failing ones.
//!
//! # Safety of the write path
//!
//! Restoring reuses the tweak layer's guard: the image is refused unless it
//! came from the very camera that is connected (same model and serial), the
//! current image is saved as a safety copy first, and after the write the image
//! is read back and compared before it is persisted to flash. Until that last
//! step nothing has left the camera's memory, so a write that fails part way
//! leaves the stored settings as they were.

use std::path::Path;

use pmca_i18n::Localizer;
use pmca_platform::BackupPatch;
use pmca_usb::nusb_backend::NusbBackend;
use pmca_usb::senser_camera::SenserCamera;
use serde::{Deserialize, Serialize};

use crate::dto::{ProgressEvent, ProgressKind};
use crate::service_mode::{leave_service_mode, open_service_camera};
use crate::{Error, ProgressSink, Result, direct, tweaks};

/// What a successful save wrote.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct BackupSaveOutcome {
    /// Where the image was written.
    pub path: String,
    /// The image size in bytes.
    pub bytes: usize,
}

/// What a successful restore did.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct BackupRestoreOutcome {
    /// The image size written back, in bytes.
    pub bytes: usize,
    /// Where the camera's previous image was kept, if it could be saved.
    pub safety_copy: Option<String>,
}

/// A service session that ends when it goes out of scope, on every path.
struct Service(SenserCamera<NusbBackend>);

impl Service {
    fn open(l: &Localizer, sink: &mut dyn ProgressSink) -> Result<Self> {
        Ok(Self(open_service_camera(l, sink)?))
    }
}

impl Drop for Service {
    fn drop(&mut self) {
        leave_service_mode(&mut self.0);
    }
}

/// Read the camera's whole backup image and write it to `path`.
///
/// No size check: a camera that hands its image over is taken at its word, and
/// one that refuses fails outright whatever the size. That failure's message
/// says how to revive the session.
///
/// Attempted twice. The cameras that refuse this transfer take the USB session
/// down with them (upstream issues #733 and #744), so a second try inside the
/// same session could only fail again; the retry opens a fresh one. Reading
/// changes nothing on the camera, so the second attempt costs a wait and
/// nothing else.
pub fn save(l: &Localizer, sink: &mut dyn ProgressSink, path: &Path) -> Result<BackupSaveOutcome> {
    // The first failure is not reported: the camera is about to be reached
    // again, and a failure the retry recovers from is not one the user has to
    // act on. If the retry fails too, its error is the one that surfaces.
    let image = match read_whole_image(l, sink) {
        Ok(image) => image,
        Err(_) => read_whole_image(l, sink)?,
    };
    std::fs::write(path, &image)?;
    sink.emit(ProgressEvent::step(ProgressKind::Done, l.tr("backup-saved")));
    Ok(BackupSaveOutcome { path: path.display().to_string(), bytes: image.len() })
}

/// One attempt at the whole image, in a session of its own.
pub(crate) fn read_whole_image(l: &Localizer, sink: &mut dyn ProgressSink) -> Result<Vec<u8>> {
    let mut cam = Service::open(l, sink)?;
    sink.emit(ProgressEvent::step(ProgressKind::Info, l.tr("tweaks-reading")));
    cam.0.get_backup_preset_data(true).map_err(|e| {
        Error::Operation(l.tr_arg("tweaks-full-read-failed", "detail", &e.to_string()))
    })
}

/// Write the backup image in `path` back to the connected camera.
///
/// Refused unless the image came from the connected camera. The current image
/// is saved first, the written image is read back and compared, and only then
/// is it persisted to flash.
pub fn restore(
    l: &Localizer,
    sink: &mut dyn ProgressSink,
    path: &Path,
) -> Result<BackupRestoreOutcome> {
    let image = std::fs::read(path)?;
    let patch = BackupPatch::new(image.clone()).map_err(|e| Error::Operation(e.to_string()))?;
    let file_key = direct::camera_key(&patch);

    let mut cam = Service::open(l, sink)?;
    sink.emit(ProgressEvent::step(ProgressKind::Info, l.tr("tweaks-reading")));
    let current = cam.0.get_backup_preset_data(true).map_err(|e| {
        Error::Operation(l.tr_arg("tweaks-full-read-failed", "detail", &e.to_string()))
    })?;
    let current_patch = BackupPatch::new(current).map_err(|e| Error::Operation(e.to_string()))?;
    if direct::camera_key(&current_patch) != file_key {
        return Err(Error::Operation(l.tr_arg(
            "backup-other-camera",
            "model",
            &model_name(&patch),
        )));
    }
    let safety = tweaks::save_original(&current_patch).ok().map(|p| p.display().to_string());

    sink.emit(ProgressEvent::step(ProgressKind::Info, l.tr("tweaks-writing")));
    cam.0.set_backup_preset_data(tweaks::BACKUP_PROTECT_MODE, &image)?;
    let written = cam.0.get_backup_preset_data(true)?;
    if written.len() != image.len()
        || written[tweaks::VERIFY_SKIP..] != image[tweaks::VERIFY_SKIP..]
    {
        return Err(Error::Operation(l.tr("tweaks-write-failed")));
    }
    cam.0.save_backup(0)?;
    drop(cam);

    sink.emit(ProgressEvent::step(ProgressKind::Done, l.tr("backup-restored")));
    Ok(BackupRestoreOutcome { bytes: image.len(), safety_copy: safety })
}

/// The model name an image names itself with, for a refusal message.
fn model_name(patch: &BackupPatch) -> String {
    pmca_platform::identity::list(patch)
        .iter()
        .find(|v| v.id == "modelName")
        .map(|v| v.value.clone())
        .unwrap_or_default()
}
