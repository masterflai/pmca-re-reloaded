//! Apps and plugins from the sources the user has added.
//!
//! Wraps [`pmca_market::appstore`] so both front-ends can list what the
//! sources in [`crate::sources`] offer and download one entry. Listing costs
//! one HTTPS request per source; resolving the newest build one more per entry.
//!
//! Resolved releases are kept for the life of the process. Most entries
//! publish on GitHub, whose API allows 60 unauthenticated requests an hour;
//! showing every app's version and then downloading a few would otherwise ask
//! twice for the same answer, and reloading the list would soon run out.

use std::collections::BTreeMap;
use std::sync::{Mutex, PoisonError};

use pmca_i18n::Localizer;
use pmca_market::appstore::{self, EntryKind, Release, ReleaseSource};
use sha2::{Digest, Sha256};

use crate::dto::{CommunityApp, CommunityList, SourceFailure};
use crate::{Error, ProgressEvent, ProgressKind, ProgressSink, Result, sources};

/// An entry is named by its source and its package (or plugin id).
type Key = (String, String);

/// Where each listed entry publishes its builds, from the last listing.
static RELEASE_SOURCES: Mutex<BTreeMap<Key, (EntryKind, ReleaseSource)>> =
    Mutex::new(BTreeMap::new());

/// Releases resolved so far.
static RELEASES: Mutex<BTreeMap<Key, Release>> = Mutex::new(BTreeMap::new());

fn key(source: &str, package: &str) -> Key {
    (source.to_ascii_lowercase(), package.to_string())
}

/// Read every source and remember where each entry publishes its builds.
fn fetch_all() -> (Vec<(String, appstore::StoreApp)>, Vec<SourceFailure>) {
    let mut entries = Vec::new();
    let mut failed = Vec::new();
    for source in sources::configured() {
        let id = source.id();
        match appstore::get_store_apps(&source.owner, &source.repo, &source.branch, &source.path) {
            Ok(apps) => {
                let mut known = RELEASE_SOURCES.lock().unwrap_or_else(PoisonError::into_inner);
                for app in apps {
                    known.insert(key(&id, &app.package), (app.kind, app.release.clone()));
                    entries.push((id.clone(), app));
                }
            }
            Err(e) => failed.push(SourceFailure { source: id, reason: e.to_string() }),
        }
    }
    (entries, failed)
}

fn listing(kind: EntryKind) -> CommunityList {
    let (entries, failed) = fetch_all();
    let entries = entries
        .into_iter()
        .filter(|(_, app)| app.kind == kind)
        .map(|(source, app)| CommunityApp::new(source, app))
        .collect();
    CommunityList { entries, failed }
}

/// The apps the user's sources offer. A source that cannot be read is
/// reported in [`CommunityList::failed`] and does not hide the others.
#[must_use]
pub fn community_apps() -> CommunityList {
    listing(EntryKind::App)
}

/// The plugins the user's sources offer.
#[must_use]
pub fn community_plugins() -> CommunityList {
    listing(EntryKind::Plugin)
}

/// The sources offering `package`, for a front-end that names only the package.
#[must_use]
pub fn sources_offering(package: &str, kind: EntryKind) -> Vec<String> {
    let (entries, _) = fetch_all();
    entries
        .into_iter()
        .filter(|(_, app)| app.kind == kind && app.package == package)
        .map(|(source, _)| source)
        .collect()
}

/// The newest release of an entry, resolved once per process.
fn latest_release(l: &Localizer, source: &str, package: &str, kind: EntryKind) -> Result<Release> {
    let k = key(source, package);
    if let Some(release) = RELEASES.lock().unwrap_or_else(PoisonError::into_inner).get(&k) {
        return Ok(release.clone());
    }
    let unknown =
        || Error::Operation(format!("{}: {package} ({source})", l.tr("store-unknown-app")));

    let lookup = || RELEASE_SOURCES.lock().unwrap_or_else(PoisonError::into_inner).get(&k).cloned();
    let known = match lookup() {
        Some(known) => Some(known),
        None => {
            fetch_all();
            lookup()
        }
    };
    let (found, release_source) = known.ok_or_else(unknown)?;
    if found != kind {
        return Err(unknown());
    }
    let no_release = || Error::Operation(format!("{}: {package}", l.tr("store-no-release")));
    if matches!(release_source, ReleaseSource::None) {
        return Err(no_release());
    }

    // Resolved without holding the lock: this is a network request, and other
    // entries' lookups should not queue behind it.
    let release = appstore::resolve_release(&release_source, kind)?.ok_or_else(no_release)?;
    RELEASES.lock().unwrap_or_else(PoisonError::into_inner).insert(k, release.clone());
    Ok(release)
}

/// The version label of the newest release of an app, as published.
pub fn latest_version(l: &Localizer, source: &str, package: &str) -> Result<String> {
    Ok(latest_release(l, source, package, EntryKind::App)?.version)
}

/// An APK fetched from a source.
pub struct DownloadedApp {
    /// The version the camera will report once the app is installed: the
    /// APK's `versionName`, or the release tag where the manifest has none.
    pub version: String,
    /// The raw APK bytes, ready to be handed to [`crate::install_app`].
    pub data: Vec<u8>,
}

/// SHA-256 of the APK's signer certificate, hex; `None` when unsigned.
fn signer(data: &[u8]) -> Option<String> {
    let certificate = pmca_apk::signer_certificate(data).ok().flatten()?;
    Some(Sha256::digest(&certificate).iter().map(|b| format!("{b:02x}")).collect())
}

/// Resolve an app to its newest release and download the APK.
///
/// The first download from a source remembers who signed the app; a later one
/// signed by someone else is refused before it reaches the camera.
pub fn download_community_app(
    l: &Localizer,
    source: &str,
    package: &str,
    sink: &mut dyn ProgressSink,
) -> Result<DownloadedApp> {
    sink.emit(ProgressEvent::step(ProgressKind::Info, l.tr("store-resolving")));
    let release = latest_release(l, source, package, EntryKind::App)?;

    sink.emit(ProgressEvent::step(
        ProgressKind::Info,
        format!("{} {}", l.tr("store-downloading"), release.version),
    ));
    let data = appstore::download_asset(&release.url)?;
    sources::check_signer(l, source, package, package, signer(&data).as_deref())?;
    // The camera lists installed apps by `versionName`, so that is what a
    // comparison with an installed copy has to show.
    let version =
        pmca_apk::read(&data).ok().and_then(|info| info.version_name).unwrap_or(release.version);
    Ok(DownloadedApp { version, data })
}

/// Resolve a plugin to its newest release and download the file.
///
/// The bytes are checked as a plugin file is, and the plugin has to carry the
/// id the source announces, so an entry cannot replace another plugin.
pub fn download_community_plugin(
    l: &Localizer,
    source: &str,
    id: &str,
    sink: &mut dyn ProgressSink,
) -> Result<(crate::plugins::PluginInfo, Vec<u8>)> {
    sink.emit(ProgressEvent::step(ProgressKind::Info, l.tr("store-resolving")));
    let release = latest_release(l, source, id, EntryKind::Plugin)?;
    sink.emit(ProgressEvent::step(
        ProgressKind::Info,
        format!("{} {}", l.tr("store-downloading"), release.version),
    ));
    let data = appstore::download_asset(&release.url)?;
    let info = crate::plugins::inspect_bytes(l, &data, &format!("{id}.pmcaplugin"))?;
    if info.id != id {
        return Err(Error::Operation(
            l.tr_args("source-plugin-mismatch", &[("found", &info.id), ("id", id)]),
        ));
    }
    let name = info.name.clone();
    sources::check_signer(l, source, &format!("plugin:{id}"), &name, info.signer.as_deref())?;
    Ok((info, data))
}
