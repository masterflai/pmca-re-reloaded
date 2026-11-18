//! Whether the window may run write tests at all.
//!
//! A setting of the window, kept beside the other per-user records so the
//! backend can refuse a write the window should not have offered. The console
//! does not consult it: every write there is asked for on its command line.

use serde::{Deserialize, Serialize};

use crate::{Error, Result};

const FILE: &str = "analysis.json";

#[derive(Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Stored {
    #[serde(default)]
    write_tests: bool,
}

fn path() -> Option<std::path::PathBuf> {
    pmca_i18n::catalog::user_data_dir().map(|dir| dir.join(FILE))
}

/// Whether write tests are allowed. Off unless switched on, and off whenever
/// the setting cannot be read.
#[must_use]
pub fn write_tests_allowed() -> bool {
    path()
        .and_then(|p| std::fs::read(p).ok())
        .and_then(|data| serde_json::from_slice::<Stored>(&data).ok())
        .is_some_and(|s| s.write_tests)
}

/// Allow or forbid write tests from now on.
pub fn allow_write_tests(on: bool) -> Result<()> {
    let path = path().ok_or_else(|| Error::Operation("no per-user data folder".to_string()))?;
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    std::fs::write(&path, serde_json::to_vec_pretty(&Stored { write_tests: on })?)?;
    Ok(())
}
