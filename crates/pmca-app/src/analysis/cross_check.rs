//! Cross-checking the candidates on another camera. Only reads.

use super::*;

impl AnalysisRun {
    /// Whether another camera can be checked: every snapshot taken, a
    /// candidate found, and no written value waiting to be restored.
    pub fn can_cross_check(&self) -> bool {
        self.complete() && !self.needs_restore() && !self.evaluation().candidates.is_empty()
    }

    /// Read the candidates from another camera. Only reads.
    ///
    /// The analysed camera itself is refused: its values are in the snapshots
    /// already, and listing it as another camera would mislead the report.
    pub fn cross_check(
        &mut self,
        l: &Localizer,
        sink: &mut dyn ProgressSink,
    ) -> Result<CrossCheckView> {
        if !self.complete() || self.needs_restore() {
            return Err(Error::Operation(l.tr("analysis-cross-not-now")));
        }
        let evaluation = self.evaluation();
        if evaluation.candidates.is_empty() {
            return Err(Error::Operation(l.tr("analysis-cross-none")));
        }
        let firmware = crate::camera_info().ok().and_then(|c| c.firmware);

        let mut cam = Service::open(l, sink)?;
        sink.emit(ProgressEvent::step(ProgressKind::Info, l.tr("tweaks-reading")));
        let identity = identify(&mut cam.0)?;
        if self.key.as_deref() == Some(direct::camera_key(&identity).as_str()) {
            return Err(Error::Operation(l.tr("analysis-cross-same-camera")));
        }
        let values =
            cross_values(&evaluation.candidates, &self.labels(), |id| cam.0.read_backup(id))?;
        drop(cam);

        let ids = pmca_platform::identity::list(&identity);
        let get = |id: &str| ids.iter().find(|v| v.id == id).map(|v| v.value.clone());
        let check = CrossCheckView {
            at: timestamp(),
            camera: CrossCamera {
                model_name: get("modelName"),
                model_code: get("modelCode"),
                region: identity.region().to_string(),
                platform_version: get("androidPlatformVersion"),
                firmware,
            },
            values,
        };
        let model = check.camera.model_name.clone();
        self.record(Access::Read, StepKind::CrossCheck, model.as_deref());
        self.cross_checks.push(check.clone());
        sink.emit(ProgressEvent::step(ProgressKind::Done, l.tr("analysis-snapshot-done")));
        Ok(check)
    }

    /// Refuse to write to any camera but the analysed one.
    pub(crate) fn ensure_same_camera(
        &self,
        l: &Localizer,
        cam: &mut SenserCamera<NusbBackend>,
    ) -> Result<()> {
        let key = direct::camera_key(&identify(cam)?);
        if self.key.as_deref() == Some(key.as_str()) {
            return Ok(());
        }
        let model = self.camera.as_ref().and_then(|c| c.model_name.clone()).unwrap_or_default();
        Err(Error::Operation(l.tr_arg("analysis-wrong-camera", "model", &model)))
    }
}

/// The camera's identifying settings, read one property at a time: fast, and
/// it works on every backup size.
pub(crate) fn identify(cam: &mut SenserCamera<NusbBackend>) -> Result<pmca_platform::DirectPatch> {
    let region = cam
        .get_backup_preset_data_status()
        .map(|s| pmca_platform::direct::status_region(&s))
        .unwrap_or_default();
    Ok(direct::read_ids(cam, &region, pmca_platform::DirectPatch::identity_ids())?)
}

/// Read every candidate through `read` and compare it with the values the
/// analysed camera stored. A senser error means the camera lacks the property.
pub(crate) fn cross_values(
    candidates: &[Candidate],
    labels: &[String],
    mut read: impl FnMut(u32) -> pmca_usb::Result<Vec<u8>>,
) -> Result<Vec<CrossValue>> {
    let mut out = Vec::with_capacity(candidates.len());
    for c in candidates {
        let data = match read(c.id) {
            Ok(data) => Some(data),
            Err(pmca_usb::Error::Senser(_)) => None,
            Err(e) => return Err(e.into()),
        };
        // The last snapshot repeats the first menu value.
        let menu = c.values.iter().zip(labels).take(labels.len().saturating_sub(1));
        out.push(CrossValue {
            id: format!("{:#010x}", c.id),
            present: data.is_some(),
            size: data.as_ref().map(Vec::len),
            hex: data.as_deref().filter(|d| d.len() <= MAX_WRITE_SIZE).map(hex),
            matches: data
                .as_ref()
                .and_then(|d| menu.clone().find(|(v, _)| *v == d).map(|(_, label)| label.clone())),
        });
    }
    Ok(out)
}
