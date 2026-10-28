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

/// What the last listing said about an entry.
#[derive(Clone)]
struct Known {
    kind: EntryKind,
    release: ReleaseSource,
    /// The signing certificate the entry names, if any (apps only).
    cert_sha256: Option<String>,
}

/// Where each listed entry publishes its builds, from the last listing.
static RELEASE_SOURCES: Mutex<BTreeMap<Key, Known>> = Mutex::new(BTreeMap::new());

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
                    known.insert(
                        key(&id, &app.package),
                        Known {
                            kind: app.kind,
                            release: app.release.clone(),
                            cert_sha256: app.cert_sha256.clone(),
                        },
                    );
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
///
/// Entries that publish the same way (same release source and certificate pin)
/// lead to the same file, so they count once, under the source listed first.
#[must_use]
pub fn sources_offering(package: &str, kind: EntryKind) -> Vec<String> {
    let (entries, _) = fetch_all();
    distinct_offers(entries, package, kind)
}

fn distinct_offers(
    entries: Vec<(String, appstore::StoreApp)>,
    package: &str,
    kind: EntryKind,
) -> Vec<String> {
    let mut seen: Vec<(ReleaseSource, Option<String>)> = Vec::new();
    let mut sources = Vec::new();
    for (source, app) in entries {
        if app.kind != kind || app.package != package {
            continue;
        }
        let offer = (app.release, app.cert_sha256);
        if !seen.contains(&offer) {
            seen.push(offer);
            sources.push(source);
        }
    }
    sources
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
    let Known { kind: found, release: release_source, .. } = known.ok_or_else(unknown)?;
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

/// SHA-256 of the APK's signer certificate, hex; `Ok(None)` when unsigned.
///
/// The signature has to hold for every file of the archive. One this program
/// cannot check (DSA, EC) still names its signer for an entry without a pin,
/// as before the check existed; an entry with a pin needs a checked one.
fn signer(data: &[u8], pinned: bool) -> std::result::Result<Option<String>, String> {
    use pmca_apk::Signature;
    let fingerprint = |certificate: &[u8]| {
        Sha256::digest(certificate).iter().map(|b| format!("{b:02x}")).collect()
    };
    match pmca_apk::verify_signature(data) {
        Ok(Signature::Unsigned) => Ok(None),
        Ok(Signature::Valid(certificate)) => Ok(Some(fingerprint(&certificate))),
        Ok(Signature::Unsupported(certificate)) if !pinned => Ok(Some(fingerprint(&certificate))),
        Ok(Signature::Unsupported(_)) => Err("signature cannot be checked".into()),
        Ok(Signature::Invalid(reason)) => Err(reason.into()),
        Err(e) => Err(e.to_string()),
    }
}

/// The certificate fingerprint the source names for `package`, if any.
fn pinned_certificate(source: &str, package: &str) -> Option<String> {
    RELEASE_SOURCES
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .get(&key(source, package))
        .and_then(|known| known.cert_sha256.clone())
}

/// Whether an APK signed by `signer` may be installed for an entry that names
/// `pinned` as its certificate. Without a pin every signer passes here.
fn matches_pin(pinned: Option<&str>, signer: Option<&str>) -> bool {
    match pinned {
        None => true,
        Some(pin) => signer == Some(pin),
    }
}

/// Resolve an app to its newest release and download the APK.
///
/// The APK's signature has to hold for every file in it. When the entry names
/// its signing certificate, the APK has to be signed with exactly that one, the
/// first download included. Beyond that, the first download from a source
/// remembers who signed the app; a later one signed by someone else is refused
/// before it reaches the camera.
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
    let pin = pinned_certificate(source, package);
    let signed_by = signer(&data, pin.is_some()).map_err(|reason| {
        Error::Operation(l.tr_args(
            "source-signature-invalid",
            &[("name", package), ("source", source), ("reason", &reason)],
        ))
    })?;
    if !matches_pin(pin.as_deref(), signed_by.as_deref()) {
        return Err(Error::Operation(
            l.tr_args("source-signer-pinned", &[("name", package), ("source", source)]),
        ));
    }
    sources::check_signer(l, source, package, package, signed_by.as_deref())?;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_pin_admits_only_its_own_certificate() {
        assert!(matches_pin(None, Some("aa")));
        assert!(matches_pin(None, None));
        assert!(matches_pin(Some("aa"), Some("aa")));
        assert!(!matches_pin(Some("aa"), Some("bb")));
        assert!(!matches_pin(Some("aa"), None));
    }

    #[test]
    fn only_a_readable_signature_names_a_signer() {
        let mut writer = zip::ZipWriter::new(std::io::Cursor::new(Vec::new()));
        writer.start_file("classes.dex", zip::write::SimpleFileOptions::default()).unwrap();
        let unsigned = writer.finish().unwrap().into_inner();
        assert_eq!(signer(&unsigned, true), Ok(None));
        assert!(signer(b"not an apk", false).is_err());
    }

    fn offers(sources: &[(&str, &str)]) -> Vec<(String, appstore::StoreApp)> {
        sources
            .iter()
            .flat_map(|(source, yaml)| {
                appstore::parse_app_list(yaml.as_bytes())
                    .unwrap()
                    .into_iter()
                    .map(|app| (source.to_string(), app))
            })
            .collect()
    }

    const GITHUB: &str =
        "---\npackage: a.b\nname: A\nrelease:\n  type: github\n  user: dev\n  repo: a\n";
    const FORK: &str =
        "---\npackage: a.b\nname: A\nrelease:\n  type: github\n  user: other\n  repo: a\n";

    #[test]
    fn identical_offers_count_once_under_the_first_source() {
        let entries = offers(&[("x/list", GITHUB), ("y/list", GITHUB)]);
        assert_eq!(distinct_offers(entries, "a.b", EntryKind::App), ["x/list"]);
    }

    #[test]
    fn offers_of_different_builds_stay_apart() {
        let entries = offers(&[("x/list", GITHUB), ("y/list", FORK), ("z/list", GITHUB)]);
        assert_eq!(distinct_offers(entries, "a.b", EntryKind::App), ["x/list", "y/list"]);
        let pinned = format!("{GITHUB}cert_sha256: aa\n");
        let entries = offers(&[("x/list", GITHUB), ("y/list", &pinned)]);
        assert_eq!(distinct_offers(entries, "a.b", EntryKind::App).len(), 2);
        assert!(distinct_offers(offers(&[("x/list", GITHUB)]), "c.d", EntryKind::App).is_empty());
    }
}
