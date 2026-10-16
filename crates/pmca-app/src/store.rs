//! Community app store orchestration.
//!
//! Wraps [`pmca_market::appstore`] so both front-ends can list the apps from
//! `OpenMemories-AppList` and download one by package name. Listing is a single
//! HTTPS request; resolving the newest build costs one more per app.
//!
//! Resolved releases are kept for the life of the process. Most apps publish
//! on GitHub, whose API allows 60 unauthenticated requests an hour; showing
//! every app's version and then downloading a few would otherwise ask twice
//! for the same answer, and reloading the list would soon run out.

use std::collections::BTreeMap;
use std::sync::{Mutex, PoisonError};

use pmca_i18n::Localizer;
use pmca_market::appstore::{self, Release, ReleaseSource};

use crate::dto::CommunityApp;
use crate::{Error, ProgressEvent, ProgressKind, ProgressSink, Result};

/// Where each listed app publishes its builds, from the last list fetched.
static SOURCES: Mutex<BTreeMap<String, ReleaseSource>> = Mutex::new(BTreeMap::new());

/// Releases resolved so far, by package.
static RELEASES: Mutex<BTreeMap<String, Release>> = Mutex::new(BTreeMap::new());

/// Fetch the app list and remember where each app publishes its builds.
fn fetch_list() -> Result<Vec<appstore::StoreApp>> {
    let apps = appstore::get_store_apps(
        appstore::DEFAULT_LIST_USER,
        appstore::DEFAULT_LIST_REPO,
        appstore::DEFAULT_LIST_BRANCH,
        appstore::DEFAULT_LIST_FILE,
    )?;
    let mut sources = SOURCES.lock().unwrap_or_else(PoisonError::into_inner);
    sources.clear();
    sources.extend(apps.iter().map(|app| (app.package.clone(), app.release.clone())));
    Ok(apps)
}

/// Fetch the community app list.
pub fn community_apps() -> Result<Vec<CommunityApp>> {
    Ok(fetch_list()?.into_iter().map(CommunityApp::from).collect())
}

/// The newest release of `package`, resolved once per process.
fn latest_release(l: &Localizer, package: &str) -> Result<Release> {
    if let Some(release) = RELEASES.lock().unwrap_or_else(PoisonError::into_inner).get(package) {
        return Ok(release.clone());
    }

    let known = SOURCES.lock().unwrap_or_else(PoisonError::into_inner).get(package).cloned();
    let source = match known {
        Some(source) => source,
        None => {
            fetch_list()?
                .into_iter()
                .find(|a| a.package == package)
                .ok_or_else(|| {
                    Error::Operation(format!("{}: {package}", l.tr("store-unknown-app")))
                })?
                .release
        }
    };
    if matches!(source, ReleaseSource::None) {
        return Err(Error::Operation(format!("{}: {package}", l.tr("store-no-release"))));
    }

    // Resolved without holding the lock: this is a network request, and other
    // apps' lookups should not queue behind it.
    let release = appstore::resolve_release(&source)?
        .ok_or_else(|| Error::Operation(format!("{}: {package}", l.tr("store-no-release"))))?;
    RELEASES
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .insert(package.to_string(), release.clone());
    Ok(release)
}

/// The version label of the newest release of `package`, as published.
pub fn latest_version(l: &Localizer, package: &str) -> Result<String> {
    Ok(latest_release(l, package)?.version)
}

/// An APK fetched from the community store.
pub struct DownloadedApp {
    /// The version the camera will report once the app is installed: the
    /// APK's `versionName`, or the release tag where the manifest has none.
    pub version: String,
    /// The raw APK bytes, ready to be handed to [`crate::install_app`].
    pub data: Vec<u8>,
}

/// Resolve `package` to its newest release and download the APK.
pub fn download_community_app(
    l: &Localizer,
    package: &str,
    sink: &mut dyn ProgressSink,
) -> Result<DownloadedApp> {
    sink.emit(ProgressEvent::step(ProgressKind::Info, l.tr("store-resolving")));
    let release = latest_release(l, package)?;

    sink.emit(ProgressEvent::step(
        ProgressKind::Info,
        format!("{} {}", l.tr("store-downloading"), release.version),
    ));
    let data = appstore::download_asset(&release.url)?;
    // The camera lists installed apps by `versionName`, so that is what a
    // comparison with an installed copy has to show.
    let version =
        pmca_apk::read(&data).ok().and_then(|info| info.version_name).unwrap_or(release.version);
    Ok(DownloadedApp { version, data })
}
