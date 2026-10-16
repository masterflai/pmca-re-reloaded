//! Running an analysis: snapshots, evaluation, write test, restore and
//! the service sessions they need.

use super::*;

impl AnalysisRun {
    /// Start a run. `values[0]` is the value the menu shows now.
    ///
    /// `write_test` records the user's consent to the write test; without it
    /// [`Self::write`] refuses.
    pub fn new(l: &Localizer, setting: &str, values: &[String], write_test: bool) -> Result<Self> {
        let setting = setting.trim();
        if setting.is_empty() {
            return Err(Error::Operation(l.tr("analysis-invalid-setting")));
        }
        let values: Vec<String> = values.iter().map(|v| v.trim().to_string()).collect();
        let distinct = values.iter().enumerate().all(|(i, v)| !values[..i].contains(v));
        if values.len() < 2
            || values.len() > MAX_VALUES
            || values.iter().any(String::is_empty)
            || !distinct
        {
            return Err(Error::Operation(l.tr_arg(
                "analysis-invalid-values",
                "max",
                &MAX_VALUES.to_string(),
            )));
        }
        let now = timestamp();
        Ok(Self {
            setting: setting.to_string(),
            values,
            write_consent: write_test.then(|| now.clone()),
            started: now,
            snapshots: Vec::new(),
            evaluation: std::cell::OnceCell::new(),
            camera: None,
            key: None,
            steps: Vec::new(),
            write: None,
            cross_checks: Vec::new(),
            saved: None,
        })
    }

    /// The menu value of every snapshot: each value once, then the first again.
    pub fn labels(&self) -> Vec<String> {
        let mut labels = self.values.clone();
        labels.push(self.values[0].clone());
        labels
    }

    /// The menu value the next snapshot expects, or `None` when all are taken.
    pub fn next_label(&self) -> Option<String> {
        self.labels().get(self.snapshots.len()).cloned()
    }

    /// How many snapshots have been taken.
    pub fn taken(&self) -> usize {
        self.snapshots.len()
    }

    /// Whether every snapshot has been taken.
    pub fn complete(&self) -> bool {
        self.next_label().is_none()
    }

    /// Whether the user agreed to the write test.
    pub fn write_requested(&self) -> bool {
        self.write_consent.is_some()
    }

    /// The menu value the write test writes.
    pub(crate) fn write_label(&self) -> &str {
        &self.values[1]
    }

    /// Every step of the run, with what it does to the camera.
    pub fn plan(&self, l: &Localizer) -> Vec<PlanStep> {
        let step = |access, kind, key: &str, value: Option<&str>, done| PlanStep {
            access,
            kind,
            text: match value {
                Some(v) => l.tr_arg(key, "value", v),
                None => l.tr(key),
            },
            done,
        };
        let taken = self.snapshots.len();
        let mut plan = Vec::new();
        for (i, label) in self.labels().iter().enumerate() {
            if i > 0 {
                plan.push(step(
                    Access::User,
                    StepKind::Change,
                    "analysis-step-change",
                    Some(label),
                    taken > i,
                ));
            }
            plan.push(step(
                Access::Read,
                StepKind::Snapshot,
                "analysis-step-snapshot",
                Some(label),
                taken > i,
            ));
        }
        plan.push(step(
            Access::Local,
            StepKind::Evaluate,
            "analysis-step-evaluate",
            None,
            self.complete(),
        ));
        if self.write_requested() {
            let w = self.write.as_ref();
            let (new, old) = (self.write_label(), self.values[0].as_str());
            plan.push(step(
                Access::Write,
                StepKind::Write,
                "analysis-step-write",
                Some(new),
                w.is_some(),
            ));
            plan.push(step(
                Access::User,
                StepKind::CheckMenu,
                "analysis-step-check",
                Some(new),
                w.is_some_and(|w| w.menu_after_write.is_some()),
            ));
            plan.push(step(
                Access::Write,
                StepKind::Restore,
                "analysis-step-restore",
                Some(old),
                w.is_some_and(|w| w.restored),
            ));
            plan.push(step(
                Access::User,
                StepKind::CheckMenu,
                "analysis-step-check",
                Some(old),
                w.is_some_and(|w| w.menu_after_restore.is_some()),
            ));
        }
        plan.push(step(
            Access::Local,
            StepKind::Report,
            "analysis-step-report",
            None,
            self.saved.is_some() && self.complete(),
        ));
        plan
    }

    pub(crate) fn record(&mut self, access: Access, kind: StepKind, value: Option<&str>) {
        self.steps.push(StepRecord {
            access,
            kind,
            value: value.map(str::to_string),
            at: timestamp(),
        });
    }

    /// Read the complete backup image in a service session of its own.
    ///
    /// The session ends before this returns, so the user can unplug the
    /// camera and change the menu afterwards.
    pub fn take_snapshot(
        &mut self,
        l: &Localizer,
        sink: &mut dyn ProgressSink,
    ) -> Result<SnapshotInfo> {
        if self.complete() {
            return Err(Error::Operation(l.tr("analysis-already-complete")));
        }
        // Over USB, before service mode: the backup does not carry it.
        let firmware = if self.snapshots.is_empty() {
            crate::camera_info().ok().and_then(|c| c.firmware)
        } else {
            None
        };

        let image = {
            let mut cam = Service::open(l, sink)?;
            sink.emit(ProgressEvent::step(ProgressKind::Info, l.tr("tweaks-reading")));
            cam.0.get_backup_preset_data(true).map_err(|e| {
                Error::Operation(l.tr_arg("tweaks-full-read-failed", "detail", &e.to_string()))
            })?
        };
        let info = self.add_image(l, image, firmware)?;
        sink.emit(ProgressEvent::step(ProgressKind::Done, l.tr("analysis-snapshot-done")));
        Ok(info)
    }

    /// Everything [`Self::take_snapshot`] does after the image is read.
    pub(crate) fn add_image(
        &mut self,
        l: &Localizer,
        image: Vec<u8>,
        firmware: Option<String>,
    ) -> Result<SnapshotInfo> {
        let label = self.next_label().unwrap_or_default();
        let snapshot =
            Snapshot::parse(label.clone(), &image).map_err(|e| Error::Operation(e.to_string()))?;
        let revision = BackupFile::new(&image[..]).map(|f| f.revision()).unwrap_or_default();
        let size = image.len();
        let patch = BackupPatch::new(image).map_err(|e| Error::Operation(e.to_string()))?;
        let key = direct::camera_key(&patch);
        match &self.key {
            Some(first) if *first != key => {
                return Err(Error::Operation(l.tr("analysis-other-camera")));
            }
            Some(_) => {}
            None => {
                self.camera = Some(facts(&patch, size, revision, firmware));
                self.key = Some(key);
            }
        }

        if !self.snapshots.is_empty() {
            self.record(Access::User, StepKind::Change, Some(&label));
        }
        self.record(Access::Read, StepKind::Snapshot, Some(&label));
        self.snapshots.push(snapshot);
        self.evaluation = std::cell::OnceCell::new();
        Ok(SnapshotInfo {
            index: self.snapshots.len(),
            total: self.values.len() + 1,
            label,
            properties: self.snapshots.last().map_or(0, Snapshot::len),
        })
    }

    /// Compare the snapshots. Only meaningful once every one is taken.
    pub fn evaluate(&self, l: &Localizer) -> EvaluationView {
        let evaluation = self.evaluation();
        let labels = self.labels();
        let (write, write_refusal) = match self.preview(evaluation) {
            Ok(preview) => (Some(preview), None),
            Err(refusal) => (None, Some(refusal_text(l, refusal))),
        };
        EvaluationView {
            compared: evaluation.compared,
            candidates: evaluation.candidates.iter().map(|c| view(c, &labels)).collect(),
            running: evaluation.running.len(),
            changed: small_running(evaluation, &labels),
            write,
            write_refusal,
        }
    }

    /// The comparison of the snapshots taken so far, computed once per change.
    pub(crate) fn evaluation(&self) -> &find::Evaluation {
        self.evaluation.get_or_init(|| find::evaluate(&self.snapshots))
    }

    /// What the write test would write, from an evaluation.
    pub(crate) fn preview(
        &self,
        evaluation: &find::Evaluation,
    ) -> std::result::Result<WritePreview, Refusal> {
        let target = find::write_target(evaluation)?;
        Ok(WritePreview {
            id: format!("{:#010x}", target.id),
            size: target.size(),
            attr: format!("{:#04x}", target.attr),
            old_label: self.values[0].clone(),
            old_hex: hex(&target.values[0]),
            new_label: self.write_label().to_string(),
            new_hex: hex(&target.values[1]),
        })
    }

    /// Write the value the camera stored for the second menu value.
    ///
    /// Refused unless the user agreed to the write test, every snapshot is
    /// taken, exactly one safe candidate exists, and the camera still holds
    /// the original value. The original is recorded before the write.
    pub fn write(&mut self, l: &Localizer, sink: &mut dyn ProgressSink) -> Result<WritePreview> {
        if !self.write_requested() || !self.complete() || self.write.is_some() {
            return Err(Error::Operation(l.tr("analysis-write-not-allowed")));
        }
        let evaluation = self.evaluation();
        let target =
            find::write_target(evaluation).map_err(|r| Error::Operation(refusal_text(l, r)))?;
        let preview = self.preview(evaluation).map_err(|r| Error::Operation(refusal_text(l, r)))?;
        let (id, original, new) = (target.id, target.values[0].clone(), target.values[1].clone());
        let key = self.rollback_key();
        let dir = direct::rollback_dir()
            .ok_or_else(|| Error::Operation(l.tr("tweaks-rollback-unsaved")))?;

        let mut cam = Service::open(l, sink)?;
        self.ensure_same_camera(l, &mut cam.0)?;
        let current = cam.0.read_backup(id)?;
        if current != original {
            return Err(Error::Operation(l.tr("analysis-camera-changed")));
        }
        let writes = [DirectWrite { id, old: original.clone(), new: new.clone() }];
        direct::remember(&dir, &key, &writes)
            .map_err(|e| Error::Operation(format!("{} ({e})", l.tr("tweaks-rollback-unsaved"))))?;
        sink.emit(ProgressEvent::step(ProgressKind::Info, l.tr("tweaks-writing")));
        if let Err(e) = write_checked(&mut cam.0, l, &writes) {
            if matches!(e, Failure::Unchanged(_)) {
                direct::forget(&dir, &key);
            }
            return Err(e.into());
        }
        drop(cam);

        self.write = Some(WriteTest {
            id,
            original,
            written: new,
            label: self.write_label().to_string(),
            menu_after_write: None,
            survived_restart: None,
            restored: false,
            menu_after_restore: None,
        });
        let label = self.write_label().to_string();
        self.record(Access::Write, StepKind::Write, Some(&label));
        sink.emit(ProgressEvent::step(ProgressKind::Done, l.tr("analysis-written")));
        Ok(preview)
    }

    /// Note what the menu showed after the last write or restore.
    pub fn record_menu(&mut self, observed: &str) {
        let observed = observed.trim().to_string();
        let Some(w) = self.write.as_mut() else { return };
        if w.restored {
            w.menu_after_restore = Some(observed.clone());
        } else {
            w.menu_after_write = Some(observed.clone());
        }
        self.record(Access::User, StepKind::CheckMenu, Some(&observed));
    }

    /// Write the original value back. Returns whether the written value was
    /// still there, i.e. survived the restart in between.
    pub fn restore(&mut self, l: &Localizer, sink: &mut dyn ProgressSink) -> Result<bool> {
        let key = self.rollback_key();
        let Some(w) = self.write.as_ref().filter(|w| !w.restored).cloned() else {
            return Err(Error::Operation(l.tr("analysis-write-not-allowed")));
        };
        let mut cam = Service::open(l, sink)?;
        self.ensure_same_camera(l, &mut cam.0)?;
        let current = cam.0.read_backup(w.id)?;
        let survived = current == w.written;
        if current != w.original {
            sink.emit(ProgressEvent::step(ProgressKind::Info, l.tr("tweaks-writing")));
            let writes = [DirectWrite { id: w.id, old: current, new: w.original.clone() }];
            write_checked(&mut cam.0, l, &writes).map_err(Error::from)?;
        }
        drop(cam);
        if let Some(dir) = direct::rollback_dir() {
            direct::forget(&dir, &key);
        }
        if let Some(w) = self.write.as_mut() {
            w.survived_restart = Some(survived);
            w.restored = true;
        }
        let label = self.values[0].clone();
        self.record(Access::Write, StepKind::Restore, Some(&label));
        sink.emit(ProgressEvent::step(ProgressKind::Done, l.tr("analysis-restored")));
        Ok(survived)
    }

    /// Everything a front-end needs to show where the run stands.
    pub fn state(&self, l: &Localizer) -> RunState {
        let w = self.write.as_ref();
        RunState {
            setting: self.setting.clone(),
            values: self.values.clone(),
            plan: self.plan(l),
            next_label: self.next_label(),
            taken: self.taken(),
            total: self.labels().len(),
            evaluation: self.complete().then(|| self.evaluate(l)),
            write_requested: self.write_requested(),
            written: w.is_some(),
            menu_after_write: w.and_then(|w| w.menu_after_write.clone()),
            restored: w.is_some_and(|w| w.restored),
            survived_restart: w.and_then(|w| w.survived_restart),
            menu_after_restore: w.and_then(|w| w.menu_after_restore.clone()),
            saved: self.saved.as_ref().map(|p| p.display().to_string()),
            can_cross_check: self.can_cross_check(),
            cross_checks: self.cross_checks.clone(),
        }
    }

    /// Whether the write test wrote a value that is not yet restored.
    pub fn needs_restore(&self) -> bool {
        self.write.as_ref().is_some_and(|w| !w.restored)
    }

    pub(crate) fn rollback_key(&self) -> String {
        format!("{}{ROLLBACK_SUFFIX}", self.key.as_deref().unwrap_or("camera"))
    }
}

/// Whether a write test left a value behind that was never restored.
pub fn has_pending_restore() -> bool {
    let Some(dir) = direct::rollback_dir() else { return false };
    std::fs::read_dir(dir).is_ok_and(|entries| {
        entries.filter_map(|e| e.ok()).any(|e| {
            e.file_name().to_str().is_some_and(|n| n.ends_with(&format!("{ROLLBACK_SUFFIX}.json")))
        })
    })
}

/// Write back what an interrupted write test left on the connected camera.
///
/// Returns how many values were written. A camera without such a record gets
/// an error, so the user knows the record belongs to another one.
pub fn restore_pending(l: &Localizer, sink: &mut dyn ProgressSink) -> Result<usize> {
    let dir =
        direct::rollback_dir().ok_or_else(|| Error::Operation(l.tr("analysis-pending-none")))?;
    let mut cam = Service::open(l, sink)?;
    let key = format!("{}{ROLLBACK_SUFFIX}", direct::camera_key(&identify(&mut cam.0)?));
    let record =
        direct::load(&dir, &key).ok_or_else(|| Error::Operation(l.tr("analysis-pending-none")))?;
    let writes = direct::undo_writes(&mut cam.0, &record);
    if !writes.is_empty() {
        sink.emit(ProgressEvent::step(ProgressKind::Info, l.tr("tweaks-writing")));
        write_checked(&mut cam.0, l, &writes).map_err(Error::from)?;
    }
    drop(cam);
    direct::forget(&dir, &key);
    sink.emit(ProgressEvent::step(ProgressKind::Done, l.tr("analysis-restored")));
    Ok(writes.len())
}

/// A service session that ends when it goes out of scope, on every path.
pub(crate) struct Service(pub(crate) SenserCamera<NusbBackend>);

impl Service {
    pub(crate) fn open(l: &Localizer, sink: &mut dyn ProgressSink) -> Result<Self> {
        Ok(Self(open_service_camera(l, sink)?))
    }
}

impl Drop for Service {
    fn drop(&mut self) {
        leave_service_mode(&mut self.0);
    }
}

/// A failed write, by whether the camera was left as it was.
pub(crate) enum Failure {
    Unchanged(Error),
    Changed(Error),
}

impl From<Failure> for Error {
    fn from(f: Failure) -> Self {
        match f {
            Failure::Unchanged(e) | Failure::Changed(e) => e,
        }
    }
}

/// Unlock, write, read back, lock and persist; see [`direct::write`].
pub(crate) fn write_checked(
    cam: &mut SenserCamera<NusbBackend>,
    l: &Localizer,
    writes: &[DirectWrite],
) -> std::result::Result<(), Failure> {
    direct::write(cam, writes, &mut |_| {}).map_err(|e| match e {
        direct::WriteError::Refused { cause, unchanged: true } => Failure::Unchanged(
            Error::Operation(l.tr_arg("analysis-write-refused", "detail", &cause.to_string())),
        ),
        direct::WriteError::Refused { cause, unchanged: false } => Failure::Changed(
            Error::Operation(l.tr_arg("tweaks-direct-revert-failed", "detail", &cause.to_string())),
        ),
        direct::WriteError::Usb(e) => Failure::Changed(Error::from(e)),
    })
}

/// The camera facts the report may carry.
pub(crate) fn facts(
    patch: &BackupPatch,
    size: usize,
    revision: u8,
    firmware: Option<String>,
) -> CameraFacts {
    let ids = pmca_platform::identity::list(patch);
    let get = |id: &str| ids.iter().find(|v| v.id == id).map(|v| v.value.clone());
    CameraFacts {
        model_name: get("modelName"),
        model_code: get("modelCode"),
        region: patch.region().to_string(),
        platform_version: get("androidPlatformVersion"),
        firmware,
        backup_size: size,
        backup_revision: revision,
    }
}

/// The running values small enough to show; larger ones may hold names.
pub(crate) fn small_running(
    evaluation: &find::Evaluation,
    labels: &[String],
) -> Vec<CandidateView> {
    evaluation
        .running
        .iter()
        .filter(|c| c.size() <= MAX_WRITE_SIZE)
        .map(|c| view(c, labels))
        .collect()
}

pub(crate) fn view(c: &Candidate, labels: &[String]) -> CandidateView {
    let small = c.size() <= MAX_WRITE_SIZE;
    let first = &c.values[0];
    CandidateView {
        id: format!("{:#010x}", c.id),
        size: c.size(),
        attr: format!("{:#04x}", c.attr),
        attr_names: attr::names(c.attr).into_iter().map(str::to_string).collect(),
        reset: c.reset.as_deref().filter(|_| small).map(hex),
        consistent: c.consistent,
        values: c
            .values
            .iter()
            .zip(labels)
            .map(|(v, label)| StepValue {
                label: label.clone(),
                hex: small.then(|| hex(v)),
                changed_offsets: (0..v.len().max(first.len()))
                    .filter(|&i| v.get(i) != first.get(i))
                    .collect(),
            })
            .collect(),
    }
}

pub(crate) fn refusal_text(l: &Localizer, refusal: Refusal) -> String {
    match refusal {
        Refusal::NoCandidate => l.tr("analysis-refuse-none"),
        Refusal::Ambiguous(n) => l.tr_arg("analysis-refuse-ambiguous", "count", &n.to_string()),
        Refusal::Inconsistent => l.tr("analysis-refuse-inconsistent"),
        Refusal::TooLarge(n) => l.tr_arg("analysis-refuse-size", "size", &n.to_string()),
        Refusal::ReadOnly => l.tr("analysis-refuse-read-only"),
        Refusal::Protected => l.tr("analysis-refuse-protected"),
    }
}

pub(crate) fn refusal_code(refusal: Refusal) -> &'static str {
    match refusal {
        Refusal::NoCandidate => "no-candidate",
        Refusal::Ambiguous(_) => "ambiguous",
        Refusal::Inconsistent => "inconsistent",
        Refusal::TooLarge(_) => "too-large",
        Refusal::ReadOnly => "read-only",
        Refusal::Protected => "protected",
    }
}
