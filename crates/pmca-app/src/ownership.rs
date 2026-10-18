//! Files a run under `sudo` creates in the user's data folder go back to the user.
//!
//! macOS needs root to take a mass-storage camera from the disk driver, so the
//! service-mode operations run through `sudo` or the window's elevated helper.
//! Without this, a key, a rollback record or the data folder itself would end
//! up owned by root, and the next run as the user could not write there.

/// Gives the user's data folder back on creation and again when dropped.
///
/// Hold one for the length of a run. It does nothing unless `SUDO_UID` and
/// `SUDO_GID` say whose files they are, as `sudo` and the elevated helper set.
#[must_use = "the files are only handed back when the guard is dropped"]
pub struct OwnershipGuard(());

impl OwnershipGuard {
    /// Hand back what an earlier run left and arm the guard for this one.
    pub fn new() -> Self {
        return_user_data();
        Self(())
    }
}

impl Default for OwnershipGuard {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for OwnershipGuard {
    fn drop(&mut self) {
        return_user_data();
    }
}

/// Make everything under the user's data folder belong to the invoking user.
pub fn return_user_data() {
    #[cfg(unix)]
    if let (Some(uid), Some(gid)) = invoking_user()
        && let Some(dir) = pmca_i18n::catalog::user_data_dir()
    {
        give(&dir, uid, gid);
    }
}

/// Give single files a root run wrote outside the data folder to the invoking user.
pub fn return_paths(paths: &[std::path::PathBuf]) {
    #[cfg(unix)]
    if let (Some(uid), Some(gid)) = invoking_user() {
        for path in paths {
            let _ = std::os::unix::fs::lchown(path, Some(uid), Some(gid));
        }
    }
    #[cfg(not(unix))]
    let _ = paths;
}

/// `SUDO_UID` and `SUDO_GID`, as `sudo` and the macOS broker set them.
#[cfg(unix)]
fn invoking_user() -> (Option<u32>, Option<u32>) {
    let id = |name| std::env::var(name).ok()?.parse::<u32>().ok();
    (id("SUDO_UID"), id("SUDO_GID"))
}

/// `lchown` the tree at `path`, never following a link out of it.
#[cfg(unix)]
fn give(path: &std::path::Path, uid: u32, gid: u32) {
    use std::os::unix::fs::MetadataExt;

    let Ok(meta) = std::fs::symlink_metadata(path) else { return };
    if meta.uid() != uid || meta.gid() != gid {
        // Fails without root, which is the case this is not needed for.
        let _ = std::os::unix::fs::lchown(path, Some(uid), Some(gid));
    }
    if meta.is_dir()
        && let Ok(entries) = std::fs::read_dir(path)
    {
        for entry in entries.flatten() {
            give(&entry.path(), uid, gid);
        }
    }
}
