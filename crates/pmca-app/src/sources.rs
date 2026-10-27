//! Where apps and plugins come from: GitHub repositories the user adds.
//!
//! The program ships no source and recommends none. A source is a public
//! repository whose author publishes `pmca-source.yaml` (see
//! `docs/app-sources.md`); the user adds it by name, and only then are its
//! entries read. The exception are *suggestions* the user can switch on: the
//! list of the original project, kept by its own author, and a list of archived
//! apps kept by this program's author that only points to the apps' own
//! releases.
//!
//! Nothing is checked on anyone's behalf. What the program does keep is the
//! signer of every app at its first download, so a later version signed by
//! someone else is refused rather than installed.

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::{Mutex, PoisonError};

use pmca_i18n::Localizer;
use pmca_market::appstore::{self, RepoInfo};
use serde::{Deserialize, Serialize};

use crate::{Error, Result};

/// The file the sources are kept in, under the user's data folder.
const FILE: &str = "sources.json";

/// A source as written to [`FILE`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Stored {
    pub(crate) owner: String,
    pub(crate) repo: String,
    pub(crate) branch: String,
    pub(crate) path: String,
    /// SHA-256 of each app's signer certificate (hex), by package, from its
    /// first download.
    #[serde(default)]
    signers: BTreeMap<String, String>,
}

impl Stored {
    pub(crate) fn id(&self) -> String {
        format!("{}/{}", self.owner, self.repo)
    }

    fn public(&self) -> Source {
        Source {
            id: self.id(),
            url: format!("https://github.com/{}/{}", self.owner, self.repo),
            file: self.path.clone(),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct SourceFile {
    #[serde(default)]
    sources: Vec<Stored>,
}

/// A source the user has added.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct Source {
    /// `owner/repo`, the name every other call takes.
    pub id: String,
    /// The repository on GitHub, to look at before trusting it.
    pub url: String,
    /// The file in the repository the entries are read from.
    pub file: String,
}

/// A repository as GitHub describes it, for the list and the search.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct SourceRepo {
    /// `owner/repo`.
    pub id: String,
    /// The repository on GitHub.
    pub url: String,
    /// The repository's own description.
    pub description: Option<String>,
    /// How many GitHub users have starred it.
    pub stars: u32,
    /// Last push, ISO-8601.
    pub pushed_at: Option<String>,
    /// Whether its owner has archived it.
    pub archived: bool,
    /// Whether the user has already added it.
    pub added: bool,
}

/// A source offered for the user to switch on, with why it is offered.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct SourceSuggestion {
    /// The source it would add.
    pub source: Source,
    /// Message id of the sentence that describes it.
    pub note_key: String,
}

/// A repository offered for the user to switch on, and where its list lives.
struct Suggestion {
    owner: &'static str,
    repo: &'static str,
    branch: &'static str,
    path: &'static str,
    note_key: &'static str,
}

/// Offered, never switched on. The first one is what an empty app list offers.
const SUGGESTIONS: &[Suggestion] = &[
    Suggestion {
        owner: "ma1co",
        repo: "OpenMemories-AppList",
        branch: "master",
        path: "apps.yaml",
        note_key: "sources-suggestion-applist",
    },
    Suggestion {
        owner: "masterflai",
        repo: "pmca-community-sources",
        branch: appstore::DEFAULT_BRANCH,
        path: appstore::SOURCE_FILE,
        note_key: "sources-suggestion-community",
    },
];

/// Serialises read-modify-write of [`FILE`] within the process.
static LOCK: Mutex<()> = Mutex::new(());

/// Repositories described so far, by id; GitHub's rate limit is low.
static REPOS: Mutex<BTreeMap<String, RepoInfo>> = Mutex::new(BTreeMap::new());

fn path() -> Option<PathBuf> {
    Some(pmca_i18n::catalog::user_data_dir()?.join(FILE))
}

/// The stored sources; a missing or unreadable file means none.
fn load() -> SourceFile {
    path()
        .and_then(|p| std::fs::read(p).ok())
        .and_then(|bytes| serde_json::from_slice(&bytes).ok())
        .unwrap_or_default()
}

fn save(l: &Localizer, file: &SourceFile) -> Result<()> {
    let path = path().ok_or_else(|| Error::Operation(l.tr("sources-no-folder")))?;
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    let json = serde_json::to_vec_pretty(file).map_err(pmca_market::Error::from)?;
    std::fs::write(path, json)?;
    Ok(())
}

/// `owner/repo` out of what a user types: `owner/repo`, `github:owner/repo`
/// or the repository's address on GitHub.
fn parse_spec(spec: &str) -> Option<(String, String)> {
    let mut rest = spec.trim();
    let mut address = false;
    for prefix in ["https://", "http://"] {
        if rest.len() >= prefix.len() && rest[..prefix.len()].eq_ignore_ascii_case(prefix) {
            rest = &rest[prefix.len()..];
            address = true;
        }
    }
    let mut on_github = false;
    for prefix in ["www.github.com/", "github.com/", "github:"] {
        if rest.len() >= prefix.len() && rest[..prefix.len()].eq_ignore_ascii_case(prefix) {
            rest = &rest[prefix.len()..];
            on_github = true;
            break;
        }
    }
    if address && !on_github {
        return None;
    }
    let mut parts = rest.split('/');
    let owner = parts.next()?;
    let repo = parts.next()?;
    let repo = repo.strip_suffix(".git").unwrap_or(repo);
    // GitHub account names are letters, digits and hyphens; a dot means a host.
    let account = owner.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-');
    (account && appstore::valid_repo_name(owner) && appstore::valid_repo_name(repo))
        .then(|| (owner.to_string(), repo.to_string()))
}

/// The sources the user has added, in the order they were added.
#[must_use]
pub fn list() -> Vec<Source> {
    load().sources.iter().map(Stored::public).collect()
}

/// The stored sources, for reading their entries.
pub(crate) fn configured() -> Vec<Stored> {
    load().sources
}

/// The suggestions the user has not yet switched on.
#[must_use]
pub fn suggestions() -> Vec<SourceSuggestion> {
    let added = load().sources;
    SUGGESTIONS
        .iter()
        .filter(|s| {
            !added.iter().any(|a| {
                a.owner.eq_ignore_ascii_case(s.owner) && a.repo.eq_ignore_ascii_case(s.repo)
            })
        })
        .map(|s| SourceSuggestion {
            source: Stored {
                owner: s.owner.to_string(),
                repo: s.repo.to_string(),
                branch: s.branch.to_string(),
                path: s.path.to_string(),
                signers: BTreeMap::new(),
            }
            .public(),
            note_key: s.note_key.to_string(),
        })
        .collect()
}

/// Add a source by name, after reading its file once to be sure it has one.
pub fn add(l: &Localizer, spec: &str) -> Result<Source> {
    let (owner, repo) = parse_spec(spec).ok_or_else(|| Error::Operation(l.tr("source-invalid")))?;
    let (branch, path) = SUGGESTIONS
        .iter()
        .find(|s| s.owner.eq_ignore_ascii_case(&owner) && s.repo.eq_ignore_ascii_case(&repo))
        .map_or((appstore::DEFAULT_BRANCH, appstore::SOURCE_FILE), |s| (s.branch, s.path));
    let stored = Stored {
        owner,
        repo,
        branch: branch.to_string(),
        path: path.to_string(),
        signers: BTreeMap::new(),
    };
    let id = stored.id();
    if load().sources.iter().any(|s| s.id().eq_ignore_ascii_case(&id)) {
        return Err(Error::Operation(l.tr_arg("source-exists", "source", &id)));
    }
    appstore::get_store_apps(&stored.owner, &stored.repo, &stored.branch, &stored.path).map_err(
        |e| {
            Error::Operation(format!(
                "{} ({e})",
                l.tr_args("source-unreadable", &[("source", &id), ("file", &stored.path)])
            ))
        },
    )?;

    let _guard = LOCK.lock().unwrap_or_else(PoisonError::into_inner);
    let mut file = load();
    if !file.sources.iter().any(|s| s.id().eq_ignore_ascii_case(&id)) {
        file.sources.push(stored.clone());
        save(l, &file)?;
    }
    Ok(stored.public())
}

/// Remove a source, and with it the signers remembered for its apps.
pub fn remove(l: &Localizer, id: &str) -> Result<()> {
    let _guard = LOCK.lock().unwrap_or_else(PoisonError::into_inner);
    let mut file = load();
    let before = file.sources.len();
    file.sources.retain(|s| !s.id().eq_ignore_ascii_case(id));
    if file.sources.len() == before {
        return Err(Error::Operation(l.tr_arg("source-unknown", "source", id)));
    }
    save(l, &file)
}

fn describe(info: &RepoInfo, added: &[Stored]) -> SourceRepo {
    let id = format!("{}/{}", info.owner, info.name);
    SourceRepo {
        url: format!("https://github.com/{id}"),
        added: added.iter().any(|s| s.id().eq_ignore_ascii_case(&id)),
        id,
        description: info.description.clone(),
        stars: info.stars,
        pushed_at: info.pushed_at.clone(),
        archived: info.archived,
    }
}

/// GitHub's description of a source or a suggestion, its stars included.
pub fn repo(l: &Localizer, id: &str) -> Result<SourceRepo> {
    let (owner, repo) = parse_spec(id).ok_or_else(|| Error::Operation(l.tr("source-invalid")))?;
    let key = format!("{owner}/{repo}").to_ascii_lowercase();
    let cached = REPOS.lock().unwrap_or_else(PoisonError::into_inner).get(&key).cloned();
    let info = match cached {
        Some(info) => info,
        None => {
            let info = appstore::repo_info(&owner, &repo)?;
            REPOS.lock().unwrap_or_else(PoisonError::into_inner).insert(key, info.clone());
            info
        }
    };
    Ok(describe(&info, &load().sources))
}

/// Search GitHub for repositories tagged as sources, most-starred first.
///
/// Only ever run when the user asks; the program does not look for sources by
/// itself.
pub fn search() -> Result<Vec<SourceRepo>> {
    let hits = appstore::search_sources()?;
    let added = load().sources;
    let mut cache = REPOS.lock().unwrap_or_else(PoisonError::into_inner);
    Ok(hits
        .into_iter()
        .map(|info| {
            let entry = describe(&info, &added);
            cache.insert(entry.id.to_ascii_lowercase(), info);
            entry
        })
        .collect())
}

/// What to do with a download, given the signer remembered and the one found.
#[derive(Debug, PartialEq, Eq)]
enum Trust {
    /// Same signer as before, or nothing to compare with and nothing to remember.
    Keep,
    /// First signed download from this source: remember its signer.
    Remember,
    /// Signed by someone else, or no longer signed: refuse.
    Refuse,
}

fn trust(known: Option<&str>, now: Option<&str>) -> Trust {
    match (known, now) {
        (Some(known), Some(now)) if known == now => Trust::Keep,
        (Some(_), _) => Trust::Refuse,
        (None, Some(_)) => Trust::Remember,
        (None, None) => Trust::Keep,
    }
}

/// Check an entry's signer against the one remembered under `key`, and
/// remember it if this is the first download from that source.
///
/// `key` is the app's package or `plugin:<id>`; `name` is what the message
/// shows. `signer` is `None` for an unsigned file or an unreadable signature;
/// once a signer is remembered, that counts as a different one.
pub(crate) fn check_signer(
    l: &Localizer,
    source: &str,
    key: &str,
    name: &str,
    signer: Option<&str>,
) -> Result<()> {
    let _guard = LOCK.lock().unwrap_or_else(PoisonError::into_inner);
    let mut file = load();
    let Some(stored) = file.sources.iter_mut().find(|s| s.id().eq_ignore_ascii_case(source)) else {
        return Err(Error::Operation(l.tr_arg("source-unknown", "source", source)));
    };
    match (trust(stored.signers.get(key).map(String::as_str), signer), signer) {
        (Trust::Keep, _) => Ok(()),
        (Trust::Refuse, _) => Err(Error::Operation(
            l.tr_args("source-signer-changed", &[("name", name), ("source", source)]),
        )),
        (Trust::Remember, now) => {
            stored.signers.insert(key.to_string(), now.unwrap_or_default().to_string());
            save(l, &file)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_what_users_type() {
        let pair = |o: &str, r: &str| Some((o.to_string(), r.to_string()));
        assert_eq!(parse_spec("dev/apps"), pair("dev", "apps"));
        assert_eq!(parse_spec(" github:dev/apps "), pair("dev", "apps"));
        assert_eq!(parse_spec("https://github.com/dev/apps.git"), pair("dev", "apps"));
        assert_eq!(parse_spec("HTTPS://GitHub.com/dev/apps/tree/main"), pair("dev", "apps"));
        assert_eq!(parse_spec("github.com/dev/apps/"), pair("dev", "apps"));
        assert_eq!(parse_spec("dev"), None);
        assert_eq!(parse_spec("https://example.com/dev"), None);
        assert_eq!(parse_spec("example.com/dev"), None);
        assert_eq!(parse_spec("dev/../x"), None);
        assert_eq!(parse_spec("dev/a b"), None);
    }

    #[test]
    fn the_first_signer_is_kept_and_a_new_one_refused() {
        assert_eq!(trust(None, Some("aa")), Trust::Remember);
        assert_eq!(trust(Some("aa"), Some("aa")), Trust::Keep);
        assert_eq!(trust(Some("aa"), Some("bb")), Trust::Refuse);
        assert_eq!(trust(Some("aa"), None), Trust::Refuse);
        assert_eq!(trust(None, None), Trust::Keep);
    }

    #[test]
    fn the_first_suggestion_reads_the_original_list() {
        let s = &SUGGESTIONS[0];
        assert_eq!((s.owner, s.repo, s.path), ("ma1co", "OpenMemories-AppList", "apps.yaml"));
    }

    #[test]
    fn every_suggestion_is_described() {
        let l = pmca_i18n::Localizer::new("en-US");
        for s in SUGGESTIONS {
            assert!(l.tr(s.note_key) != s.note_key, "{}", s.note_key);
            assert!(appstore::valid_repo_name(s.owner) && appstore::valid_repo_name(s.repo));
        }
    }

    #[test]
    fn a_stored_source_reads_back() {
        let file = SourceFile {
            sources: vec![Stored {
                owner: "dev".into(),
                repo: "apps".into(),
                branch: "HEAD".into(),
                path: "pmca-source.yaml".into(),
                signers: BTreeMap::from([("a.b".to_string(), "00ff".to_string())]),
            }],
        };
        let json = serde_json::to_vec(&file).unwrap();
        let back: SourceFile = serde_json::from_slice(&json).unwrap();
        assert_eq!(back.sources, file.sources);
        assert_eq!(back.sources[0].public().url, "https://github.com/dev/apps");
    }
}
