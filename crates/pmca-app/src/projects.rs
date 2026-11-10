//! Independent projects: GitHub repositories the user follows here.
//!
//! A project publishes `pmca-project.yaml` and carries the topic
//! `pmca-project` (see `docs/projects.md`). Unlike a source, nothing of it is
//! downloaded or installed: the program only shows what GitHub and the file say,
//! the newest release included. It ships no project, recommends none and checks
//! none; the user finds them through a search and pins the ones to follow.
//!
//! The one exception is [`BLOCKED`]: a project reported for an obvious
//! infringement of the law is left out of the search and cannot be pinned. That
//! is the answer to a notice, not a selection, and the list only changes with a
//! release of this program.

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::{Mutex, PoisonError};

use pmca_i18n::Localizer;
use pmca_market::appstore::{self, ProjectFile, RepoInfo};
use serde::{Deserialize, Serialize};

use crate::sources::parse_spec;
use crate::{Error, Result};

/// The file the pins are kept in, under the user's data folder.
const FILE: &str = "projects.json";

/// Projects left out after a notice of an obvious infringement, as lowercase
/// `owner/repo`.
const BLOCKED: &[&str] = &[];

/// A pinned project as written to [`FILE`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Pin {
    owner: String,
    repo: String,
    /// The release shown at the last refresh, to tell a new one apart.
    #[serde(default)]
    seen_release: Option<String>,
}

impl Pin {
    fn id(&self) -> String {
        format!("{}/{}", self.owner, self.repo)
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct PinFile {
    #[serde(default)]
    pins: Vec<Pin>,
}

/// A pinned project, as far as it is known without asking GitHub.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct PinnedProject {
    /// `owner/repo`.
    pub id: String,
    /// The repository on GitHub.
    pub url: String,
}

/// A repository as GitHub describes it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct ProjectRepo {
    /// `owner/repo`.
    pub id: String,
    /// The repository on GitHub.
    pub url: String,
    /// The repository's own description.
    pub description: Option<String>,
    /// How many GitHub users have starred it.
    pub stars: u32,
    /// How many forks it has.
    pub forks: u32,
    /// Last push, ISO-8601.
    pub pushed_at: Option<String>,
    /// Whether its owner has archived it.
    pub archived: bool,
    /// SPDX identifier of its licence, if GitHub recognised one.
    pub license: Option<String>,
    /// Its main programming language.
    pub language: Option<String>,
    /// The website its owner gave on GitHub.
    pub homepage: Option<String>,
    /// Whether the user has pinned it.
    pub pinned: bool,
}

/// What a project says about itself in `pmca-project.yaml`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct ProjectDetails {
    /// Display name.
    pub name: String,
    /// One-line summary.
    pub summary: Option<String>,
    /// `tool`, `app`, `plugin`, `script`, `guide` or `other`.
    pub category: String,
    /// Camera models the author names.
    pub cameras: Vec<String>,
    /// Platforms the author names.
    pub platforms: Vec<String>,
    /// Author, as published.
    pub author: Option<String>,
    /// Project website.
    pub homepage: Option<String>,
}

/// The newest stable release of a project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct ProjectRelease {
    /// Version, from the tag.
    pub version: String,
    /// Publication date, ISO-8601.
    pub published_at: Option<String>,
    /// The release page on GitHub.
    pub url: Option<String>,
}

/// Everything shown about one project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct ProjectInfo {
    /// What GitHub says.
    pub repo: ProjectRepo,
    /// What the project file says; `None` if the repository has none (any more).
    pub details: Option<ProjectDetails>,
    /// The newest stable release; `None` if there is none.
    pub release: Option<ProjectRelease>,
    /// Whether the release differs from the one shown at the last refresh.
    pub new_release: bool,
}

/// A pinned project that could not be read.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct ProjectFailure {
    /// `owner/repo`.
    pub id: String,
    /// Stable error code, as [`Error::code`].
    pub code: String,
    /// The error's own message.
    pub message: String,
    /// The variable part of the message, as [`Error::detail`].
    pub detail: Option<String>,
}

/// The pinned projects after a refresh.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct ProjectList {
    /// The projects that could be read, in the order they were pinned.
    pub entries: Vec<ProjectInfo>,
    /// The projects that could not.
    pub failed: Vec<ProjectFailure>,
}

/// Serialises read-modify-write of [`FILE`] within the process.
static LOCK: Mutex<()> = Mutex::new(());

/// Repositories described so far, by lowercase id; GitHub's rate limit is low.
static REPOS: Mutex<BTreeMap<String, RepoInfo>> = Mutex::new(BTreeMap::new());

fn path() -> Option<PathBuf> {
    Some(pmca_i18n::catalog::user_data_dir()?.join(FILE))
}

/// The stored pins; a missing or unreadable file means none.
fn load() -> PinFile {
    path()
        .and_then(|p| std::fs::read(p).ok())
        .and_then(|bytes| serde_json::from_slice(&bytes).ok())
        .unwrap_or_default()
}

fn save(l: &Localizer, file: &PinFile) -> Result<()> {
    let path = path().ok_or_else(|| Error::Operation(l.tr("projects-no-folder")))?;
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    let json = serde_json::to_vec_pretty(file).map_err(pmca_market::Error::from)?;
    std::fs::write(path, json)?;
    Ok(())
}

fn blocked(id: &str) -> bool {
    BLOCKED.contains(&id.to_ascii_lowercase().as_str())
}

fn is_pinned(pins: &[Pin], id: &str) -> bool {
    pins.iter().any(|p| p.id().eq_ignore_ascii_case(id))
}

/// The pinned projects, in the order they were pinned; reads only the local file.
#[must_use]
pub fn list() -> Vec<PinnedProject> {
    load()
        .pins
        .iter()
        .filter(|p| !blocked(&p.id()))
        .map(|p| PinnedProject { id: p.id(), url: format!("https://github.com/{}", p.id()) })
        .collect()
}

/// Pin a project by name, after making sure it publishes a project file.
pub fn pin(l: &Localizer, spec: &str) -> Result<PinnedProject> {
    let (owner, repo) =
        parse_spec(spec).ok_or_else(|| Error::Operation(l.tr("project-invalid")))?;
    let id = format!("{owner}/{repo}");
    if blocked(&id) {
        return Err(Error::Operation(l.tr_arg("project-blocked", "project", &id)));
    }
    if is_pinned(&load().pins, &id) {
        return Err(Error::Operation(l.tr_arg("project-exists", "project", &id)));
    }
    if appstore::project_file(&owner, &repo)?.is_none() {
        return Err(Error::Operation(
            l.tr_args("project-no-file", &[("project", &id), ("file", appstore::PROJECT_FILE)]),
        ));
    }

    let _guard = LOCK.lock().unwrap_or_else(PoisonError::into_inner);
    let mut file = load();
    if !is_pinned(&file.pins, &id) {
        file.pins.push(Pin { owner, repo, seen_release: None });
        save(l, &file)?;
    }
    Ok(PinnedProject { url: format!("https://github.com/{id}"), id })
}

/// Unpin a project.
pub fn unpin(l: &Localizer, id: &str) -> Result<()> {
    let _guard = LOCK.lock().unwrap_or_else(PoisonError::into_inner);
    let mut file = load();
    let before = file.pins.len();
    file.pins.retain(|p| !p.id().eq_ignore_ascii_case(id));
    if file.pins.len() == before {
        return Err(Error::Operation(l.tr_arg("project-unknown", "project", id)));
    }
    save(l, &file)
}

fn describe(info: &RepoInfo, pins: &[Pin]) -> ProjectRepo {
    let id = format!("{}/{}", info.owner, info.name);
    ProjectRepo {
        url: format!("https://github.com/{id}"),
        pinned: is_pinned(pins, &id),
        id,
        description: info.description.clone(),
        stars: info.stars,
        forks: info.forks,
        pushed_at: info.pushed_at.clone(),
        archived: info.archived,
        license: info.license.clone(),
        language: info.language.clone(),
        homepage: info.homepage.clone(),
    }
}

/// Search GitHub for projects, most-starred first, without the blocked ones.
///
/// Only ever run when the user asks.
pub fn search() -> Result<Vec<ProjectRepo>> {
    let hits = appstore::search_projects()?;
    let pins = load().pins;
    let mut cache = REPOS.lock().unwrap_or_else(PoisonError::into_inner);
    Ok(hits
        .into_iter()
        .filter_map(|info| {
            let entry = describe(&info, &pins);
            if blocked(&entry.id) {
                return None;
            }
            cache.insert(entry.id.to_ascii_lowercase(), info);
            Some(entry)
        })
        .collect())
}

fn details(file: ProjectFile) -> ProjectDetails {
    ProjectDetails {
        name: file.name,
        summary: file.summary,
        category: file.category.id().to_string(),
        cameras: file.cameras,
        platforms: file.platforms,
        author: file.author,
        homepage: file.homepage,
    }
}

fn release(release: appstore::ProjectRelease) -> ProjectRelease {
    ProjectRelease {
        version: release.version,
        published_at: release.published_at,
        url: release.url,
    }
}

/// Read one project; `fresh` skips the repository description kept from earlier.
fn read(owner: &str, repo: &str, pins: &[Pin], fresh: bool) -> Result<ProjectInfo> {
    let key = format!("{owner}/{repo}").to_ascii_lowercase();
    let cached = if fresh {
        None
    } else {
        REPOS.lock().unwrap_or_else(PoisonError::into_inner).get(&key).cloned()
    };
    let info = match cached {
        Some(info) => info,
        None => {
            let info = appstore::repo_info(owner, repo)?;
            REPOS.lock().unwrap_or_else(PoisonError::into_inner).insert(key, info.clone());
            info
        }
    };
    Ok(ProjectInfo {
        repo: describe(&info, pins),
        details: appstore::project_file(owner, repo)?.map(details),
        release: appstore::project_release(owner, repo)?.map(release),
        new_release: false,
    })
}

/// Everything about one project, for a search hit the user opens; the release
/// is not remembered as seen.
pub fn info(l: &Localizer, id: &str) -> Result<ProjectInfo> {
    let (owner, repo) = parse_spec(id).ok_or_else(|| Error::Operation(l.tr("project-invalid")))?;
    if blocked(&format!("{owner}/{repo}")) {
        return Err(Error::Operation(l.tr_arg("project-blocked", "project", id)));
    }
    read(&owner, &repo, &load().pins, false)
}

/// Whether `now` is a release other than the one shown last time.
fn is_new(seen: Option<&str>, now: Option<&str>) -> bool {
    matches!((seen, now), (Some(seen), Some(now)) if seen != now)
}

/// Read every pinned project afresh and remember the releases now shown.
///
/// Once GitHub refuses for too many requests, the remaining projects are not
/// asked for at all; they fail with the same reason.
pub fn refresh(l: &Localizer) -> Result<ProjectList> {
    let pins: Vec<Pin> = load().pins.into_iter().filter(|p| !blocked(&p.id())).collect();
    let mut list = ProjectList::default();
    let mut seen = BTreeMap::new();
    let mut limited = false;
    for pin in &pins {
        let result = if limited {
            Err(Error::Market(pmca_market::Error::RateLimited))
        } else {
            read(&pin.owner, &pin.repo, &pins, true)
        };
        match result {
            Ok(mut info) => {
                let now = info.release.as_ref().map(|r| r.version.clone());
                info.new_release = is_new(pin.seen_release.as_deref(), now.as_deref());
                if let Some(now) = now {
                    seen.insert(pin.id().to_ascii_lowercase(), now);
                }
                list.entries.push(info);
            }
            Err(e) => {
                limited |= matches!(e, Error::Market(pmca_market::Error::RateLimited));
                list.failed.push(ProjectFailure {
                    id: pin.id(),
                    code: e.code().to_string(),
                    message: e.to_string(),
                    detail: e.detail(),
                });
            }
        }
    }

    if !seen.is_empty() {
        let _guard = LOCK.lock().unwrap_or_else(PoisonError::into_inner);
        let mut file = load();
        for pin in &mut file.pins {
            if let Some(now) = seen.remove(&pin.id().to_ascii_lowercase()) {
                pin.seen_release = Some(now);
            }
        }
        save(l, &file)?;
    }
    Ok(list)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_release_is_new_only_after_another_was_shown() {
        assert!(!is_new(None, Some("1.0")));
        assert!(!is_new(Some("1.0"), Some("1.0")));
        assert!(is_new(Some("1.0"), Some("1.1")));
        assert!(!is_new(Some("1.0"), None));
    }

    #[test]
    fn blocked_entries_are_lowercase_repository_names() {
        for id in BLOCKED {
            assert_eq!(*id, id.to_ascii_lowercase());
            assert_eq!(parse_spec(id).map(|(o, r)| format!("{o}/{r}")).as_deref(), Some(*id));
        }
    }

    #[test]
    fn a_pin_file_without_seen_releases_still_reads() {
        let file: PinFile =
            serde_json::from_str(r#"{"pins":[{"owner":"dev","repo":"p"}]}"#).unwrap();
        assert_eq!(file.pins[0].id(), "dev/p");
        assert!(file.pins[0].seen_release.is_none());
        assert!(is_pinned(&file.pins, "DEV/P"));
    }

    #[test]
    fn every_message_is_translated() {
        let l = Localizer::new("en-US");
        for key in [
            "projects-no-folder",
            "project-invalid",
            "project-blocked",
            "project-exists",
            "project-unknown",
            "project-no-file",
        ] {
            assert_ne!(l.tr_args(key, &[("project", "x/y"), ("file", "f")]), key, "{key}");
        }
    }
}
