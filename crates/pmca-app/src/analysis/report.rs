//! The report: what it contains, where it is saved, and its formatting
//! helpers.

use super::*;

impl AnalysisRun {
    /// The report as it stands.
    pub fn report(&self) -> AnalysisReport {
        let evaluation = self.evaluation();
        let labels = self.labels();
        let refusal = match (self.complete(), self.preview(evaluation)) {
            (true, Err(r)) => Some(refusal_code(r).to_string()),
            _ => None,
        };
        AnalysisReport {
            schema: REPORT_SCHEMA.to_string(),
            tool: format!("PMCA-RE RELOADED {}", env!("CARGO_PKG_VERSION")),
            started: self.started.clone(),
            camera: self.camera.clone().unwrap_or_default(),
            setting: self.setting.clone(),
            values: self.values.clone(),
            complete: self.complete(),
            steps: self.steps.clone(),
            compared: evaluation.compared,
            candidates: evaluation.candidates.iter().map(|c| view(c, &labels)).collect(),
            running: evaluation.running.iter().map(|c| format!("{:#010x}", c.id)).collect(),
            running_values: small_running(evaluation, &labels),
            write_test_consent: self.write_consent.clone(),
            write_test_refusal: refusal,
            write_test: self.write.as_ref().map(|w| WriteTestReport {
                id: format!("{:#010x}", w.id),
                written_label: w.label.clone(),
                written_hex: hex(&w.written),
                original_hex: hex(&w.original),
                verified: true,
                menu_after_write: w.menu_after_write.clone(),
                survived_restart: w.survived_restart,
                restored: w.restored,
                menu_after_restore: w.menu_after_restore.clone(),
            }),
            cross_checks: self.cross_checks.clone(),
            privacy: "No serial number, no backup image, no values of other properties."
                .to_string(),
        }
    }

    /// The report as pretty-printed JSON.
    pub fn report_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(&self.report())?)
    }

    /// Save the report in the user's data folder, overwriting this run's
    /// earlier copy. Returns the path.
    pub fn save_report(&mut self) -> Result<PathBuf> {
        let path = match &self.saved {
            Some(path) => path.clone(),
            None => {
                let dir = report_dir()
                    .ok_or_else(|| Error::Io(std::io::Error::other("no user data folder")))?;
                std::fs::create_dir_all(&dir)?;
                let model = self
                    .camera
                    .as_ref()
                    .and_then(|c| c.model_name.clone())
                    .unwrap_or_else(|| "camera".into());
                let model: String = model
                    .chars()
                    .map(|c| if c.is_ascii_alphanumeric() || c == '-' { c } else { '_' })
                    .collect();
                let stamp: String = self.started.chars().filter(char::is_ascii_digit).collect();
                dir.join(format!("{model}-{stamp}.json"))
            }
        };
        std::fs::write(&path, self.report_json()?)?;
        self.saved = Some(path.clone());
        Ok(path)
    }
}

/// Where reports are saved.
pub fn report_dir() -> Option<PathBuf> {
    Some(pmca_i18n::catalog::user_data_dir()?.join("analysis"))
}

/// Bytes as lowercase hex, separated by spaces.
pub fn hex(data: &[u8]) -> String {
    data.iter().map(|b| format!("{b:02x}")).collect::<Vec<_>>().join(" ")
}

/// The current time as `YYYY-MM-DDTHH:MM:SSZ`.
pub(crate) fn timestamp() -> String {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or_default();
    format_utc(secs)
}

pub(crate) fn format_utc(secs: u64) -> String {
    let days = (secs / 86_400) as i64;
    let rest = secs % 86_400;
    // Howard Hinnant's civil-from-days.
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = yoe + era * 400 + i64::from(month <= 2);
    format!(
        "{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}Z",
        rest / 3600,
        rest % 3600 / 60,
        rest % 60
    )
}
