//! Saving the camera's whole `Backup.bin` to a file and writing one back.
//!
//! This is the raw image, not a firmware `.dat`: the same bytes the tweak and
//! analysis paths read, written out in one piece so a user can keep a copy
//! before changing anything and put it back if a change goes wrong.
//!
//! # Limits
//!
//! On cameras whose backup image is larger than about 1 MB, `PDT_READ` stalls
//! and takes the USB session with it (see [`crate::direct`]), so a whole image
//! can be neither read nor written there. Both operations refuse such a camera
//! with a clear message rather than risk the stall.
//!
//! # Safety of the write path
//!
//! Restoring reuses the tweak layer's guard: the image is refused unless it
//! came from the very camera that is connected (same model and serial), the
//! current image is saved as a safety copy first, and after the write the image
//! is read back and compared before it is persisted to flash.

use std::path::Path;

use pmca_i18n::Localizer;
use pmca_platform::BackupPatch;
use pmca_platform::direct::full_read_too_large;
use pmca_usb::nusb_backend::NusbBackend;
use pmca_usb::senser_camera::SenserCamera;
use serde::Serialize;

use crate::dto::{ProgressEvent, ProgressKind};
use crate::service_mode::{leave_service_mode, open_service_camera};
use crate::{Error, ProgressSink, Result, direct, tweaks};

/// What a successful save wrote.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct BackupSaveOutcome {
    /// Where the image was written.
    pub path: String,
    /// The image size in bytes.
    pub bytes: usize,
}

/// What a successful restore did.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
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
/// Refuses a camera whose image is too large to read in one piece.
pub fn save(l: &Localizer, sink: &mut dyn ProgressSink, path: &Path) -> Result<BackupSaveOutcome> {
    let image = {
        let mut cam = Service::open(l, sink)?;
        sink.emit(ProgressEvent::step(ProgressKind::Info, l.tr("tweaks-reading")));
        if cam.0.get_backup_preset_data_status().ok().as_deref().is_some_and(full_read_too_large) {
            return Err(Error::Operation(l.tr("backup-too-large")));
        }
        cam.0.get_backup_preset_data(true).map_err(|e| {
            Error::Operation(l.tr_arg("tweaks-full-read-failed", "detail", &e.to_string()))
        })?
    };
    std::fs::write(path, &image)?;
    sink.emit(ProgressEvent::step(ProgressKind::Done, l.tr("backup-saved")));
    Ok(BackupSaveOutcome { path: path.display().to_string(), bytes: image.len() })
}

/// Write the backup image in `path` back to the connected camera.
///
/// Refused unless the image came from the connected camera and the camera can
/// take a whole image. The current image is saved first, the written image is
/// read back and compared, and only then is it persisted to flash.
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
    if cam.0.get_backup_preset_data_status().ok().as_deref().is_some_and(full_read_too_large) {
        return Err(Error::Operation(l.tr("backup-restore-too-large")));
    }
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
