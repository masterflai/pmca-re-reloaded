//! Community app sources – grown out of the port of `pmca/appstore`.
//!
//! A source is a YAML file in a GitHub repository: `pmca-source.yaml` by
//! default, or the `apps.yaml` of `ma1co/OpenMemories-AppList`, whose format it
//! extends. Each entry names an app or a plugin and a release source, which is
//! resolved to a downloadable file either through the GitHub releases API,
//! through a second YAML feed, through a JSON feed with a download template, or
//! directly. An entry may also name the SHA-256 of its signing certificate, so
//! even the first download is checked against the author's own statement.
//!
//! Everything that interprets remote data ([`parse_app_list`],
//! [`parse_github_latest`], [`parse_release_feed`], [`parse_json_feed`]) is
//! pure and unit-tested; only the fetching lives behind the `client` feature.
//!
//! ## Why a hand-written YAML reader
//!
//! `apps.yaml` uses a deliberately tiny subset: documents separated by `---`,
//! scalar `key: value` pairs and a single nested `release:` map. Parsing that
//! subset explicitly avoids pulling a full YAML engine (and its advisory
//! surface) into a security-sensitive tool. Anything outside the subset is
//! ignored rather than guessed at, and entries missing `package`/`name` are
//! dropped — matching the original Python.
//!
//! ## Untrusted URLs
//!
//! Release URLs come from a third-party repository, so they are restricted to
//! `https://` and downloads are size-capped before an APK is ever handed to a
//! camera.

use crate::{Error, Result};

/// File a source repository publishes at its root.
pub const SOURCE_FILE: &str = "pmca-source.yaml";
/// GitHub topic that makes a source repository findable.
pub const SOURCE_TOPIC: &str = "pmca-source";
/// The branch raw.githubusercontent.com resolves to the default branch.
pub const DEFAULT_BRANCH: &str = "HEAD";

/// Asset content type GitHub reports for an APK.
const APK_CONTENT_TYPE: &str = "application/vnd.android.package-archive";

/// What an entry of a source offers.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum EntryKind {
    /// A camera app (`.apk`); also every entry without `kind`.
    #[default]
    App,
    /// A plugin for this program (`.pmcaplugin`).
    Plugin,
}

impl EntryKind {
    /// The file name ending of a release asset of this kind.
    #[must_use]
    pub fn suffix(self) -> &'static str {
        match self {
            EntryKind::App => ".apk",
            EntryKind::Plugin => ".pmcaplugin",
        }
    }
}

/// Where the newest build of an app can be found.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReleaseSource {
    /// Releases published on a GitHub repository.
    Github {
        /// Repository owner.
        user: String,
        /// Repository name.
        repo: String,
    },
    /// A YAML feed listing releases.
    Yaml {
        /// URL of the feed.
        url: String,
    },
    /// A JSON document naming the newest version, and a template that turns
    /// it into the download URL (see [`parse_json_feed`]).
    Json {
        /// URL of the JSON document.
        url: String,
        /// Download URL with `{version}` and optionally `{build}` placeholders.
        download: String,
    },
    /// A fixed release spelled out in the app list itself.
    Direct(Release),
    /// The app declares no usable release.
    None,
}

/// A downloadable build of an app.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Release {
    /// Version label as published.
    pub version: String,
    /// Direct download URL of the APK.
    pub url: String,
    /// Publication date, as published (ISO-8601 for GitHub).
    pub date: Option<String>,
    /// Release notes.
    pub desc: Option<String>,
}

/// An app or plugin offered by a source.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreApp {
    /// App or plugin.
    pub kind: EntryKind,
    /// Android package name, or the plugin id; the source's primary key.
    pub package: String,
    /// Display name.
    pub name: String,
    /// Author, as published.
    pub author: Option<String>,
    /// Short description.
    pub desc: Option<String>,
    /// Project homepage.
    pub homepage: Option<String>,
    /// How to obtain the newest build.
    pub release: ReleaseSource,
    /// SHA-256 of the certificate the author signs with, as lowercase hex
    /// without separators, when the entry names one. A value that is not 64
    /// hex digits is kept as written, so it matches no signer and the entry
    /// cannot be installed rather than silently losing the check.
    pub cert_sha256: Option<String>,
}

/// Parse a source file (`pmca-source.yaml`, `apps.yaml`) into its entries, in
/// file order.
///
/// Entries without both `package` and `name`, and entries of an unknown `kind`,
/// are skipped.
pub fn parse_app_list(data: &[u8]) -> Result<Vec<StoreApp>> {
    let text = String::from_utf8_lossy(data);
    Ok(yaml::documents(&text)
        .into_iter()
        .filter_map(|doc| {
            let kind = match doc.scalar("kind") {
                None | Some("app") => EntryKind::App,
                Some("plugin") => EntryKind::Plugin,
                Some(_) => return None,
            };
            let package = doc.scalar("package")?.to_string();
            let name = doc.scalar("name")?.to_string();
            Some(StoreApp {
                kind,
                package,
                name,
                author: doc.scalar("author").map(str::to_string),
                desc: doc.scalar("desc").map(str::to_string),
                homepage: doc.scalar("homepage").map(str::to_string),
                release: release_source(&doc),
                cert_sha256: doc.scalar("cert_sha256").map(normalize_fingerprint),
            })
        })
        .collect())
}

/// Bring a published certificate fingerprint into the form [`StoreApp::cert_sha256`]
/// documents: separators (`:`, spaces) removed, lowercase.
fn normalize_fingerprint(value: &str) -> String {
    value
        .chars()
        .filter(|c| *c != ':' && !c.is_whitespace())
        .collect::<String>()
        .to_ascii_lowercase()
}

/// Interpret the `release:` block of an app-list entry.
fn release_source(doc: &yaml::Document) -> ReleaseSource {
    let Some(release) = doc.map("release") else {
        return ReleaseSource::None;
    };
    let get = |key: &str| release.iter().find(|(k, _)| k == key).map(|(_, v)| v.as_str());

    match get("type") {
        Some("github") => match (get("user"), get("repo")) {
            (Some(user), Some(repo)) => {
                ReleaseSource::Github { user: user.to_string(), repo: repo.to_string() }
            }
            _ => ReleaseSource::None,
        },
        Some("yaml") => match get("url") {
            Some(url) if is_https(url) => ReleaseSource::Yaml { url: url.to_string() },
            _ => ReleaseSource::None,
        },
        Some("json") => match (get("url"), get("download")) {
            (Some(url), Some(download))
                if is_https(url) && is_https(download) && download.contains("{version}") =>
            {
                ReleaseSource::Json { url: url.to_string(), download: download.to_string() }
            }
            _ => ReleaseSource::None,
        },
        _ => match (get("version"), get("url")) {
            (Some(version), Some(url)) if is_https(url) => ReleaseSource::Direct(Release {
                version: plain_version(version).to_string(),
                url: url.to_string(),
                date: get("date").map(str::to_string),
                desc: get("desc").map(str::to_string),
            }),
            _ => ReleaseSource::None,
        },
    }
}

/// A version label without the `v` of a tag such as `v1.2.0`, so it reads as
/// the `versionName` a camera reports.
#[must_use]
pub fn plain_version(label: &str) -> &str {
    match label.strip_prefix(['v', 'V']) {
        Some(rest) if rest.starts_with(|c: char| c.is_ascii_digit()) => rest,
        _ => label,
    }
}

/// Read the release GitHub flags as "Latest" and its asset of `kind`.
///
/// `data` is the JSON body of `/repos/{user}/{repo}/releases/latest`, which
/// never names a draft or a pre-release. Older releases are not searched: a
/// latest release without a matching asset yields no release.
pub fn parse_github_latest(data: &[u8], kind: EntryKind) -> Result<Option<Release>> {
    let release: serde_json::Value = serde_json::from_slice(data)?;
    let Some(tag) = release.get("tag_name").and_then(|v| v.as_str()) else {
        return Err(Error::MissingField("tag_name"));
    };
    let flagged = |key: &str| release.get(key).and_then(serde_json::Value::as_bool) == Some(true);
    if flagged("draft") || flagged("prerelease") {
        return Ok(None);
    }

    let matches = |asset: &serde_json::Value| {
        let named = asset
            .get("name")
            .and_then(|n| n.as_str())
            .is_some_and(|n| n.to_ascii_lowercase().ends_with(kind.suffix()));
        named
            || (kind == EntryKind::App
                && asset.get("content_type").and_then(|c| c.as_str()) == Some(APK_CONTENT_TYPE))
    };
    let Some(url) = release
        .get("assets")
        .and_then(|a| a.as_array())
        .into_iter()
        .flatten()
        .find(|asset| matches(asset))
        .and_then(|asset| asset.get("browser_download_url"))
        .and_then(|u| u.as_str())
        .filter(|u| is_https(u))
    else {
        return Ok(None);
    };
    let version =
        release.get("name").and_then(|v| v.as_str()).filter(|s| !s.is_empty()).unwrap_or(tag);
    Ok(Some(Release {
        version: plain_version(version).to_string(),
        url: url.to_string(),
        date: release.get("created_at").and_then(|v| v.as_str()).map(str::to_string),
        desc: release.get("body").and_then(|v| v.as_str()).map(str::to_string),
    }))
}

/// Pick the first entry of a YAML release feed that has a version and an URL.
pub fn parse_release_feed(data: &[u8]) -> Result<Option<Release>> {
    let text = String::from_utf8_lossy(data);
    Ok(yaml::documents(&text).into_iter().find_map(|doc| {
        let version = doc.scalar("version")?;
        let url = doc.scalar("url").filter(|u| is_https(u))?;
        Some(Release {
            version: plain_version(version).to_string(),
            url: url.to_string(),
            date: doc.scalar("date").map(str::to_string),
            desc: doc.scalar("desc").map(str::to_string),
        })
    }))
}

/// Read a JSON release document and build the download URL from `download`.
///
/// The document is an object with `version` and optionally `build`, each a
/// string or a number, for example `{"schema":1,"version":"2.05","build":58}`.
/// `{version}` and `{build}` in `download` are replaced by those values. The
/// values end up in a URL, so only letters, digits, `.`, `-` and `_` are
/// accepted; anything else, a missing `build` the template asks for, or a
/// result that is not `https://` yields no release.
pub fn parse_json_feed(data: &[u8], download: &str) -> Result<Option<Release>> {
    let value: serde_json::Value = serde_json::from_slice(data)?;
    let field = |key: &str| -> Option<String> {
        let text = match value.get(key)? {
            serde_json::Value::String(s) => s.clone(),
            serde_json::Value::Number(n) => n.to_string(),
            _ => return None,
        };
        let safe = !text.is_empty()
            && text.len() <= 64
            && text.bytes().all(|b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'-' | b'_'));
        safe.then_some(text)
    };
    let Some(version) = field("version") else {
        return Ok(None);
    };
    let mut url = download.replace("{version}", &version);
    if url.contains("{build}") {
        let Some(build) = field("build") else {
            return Ok(None);
        };
        url = url.replace("{build}", &build);
    }
    if !is_https(&url) || url.contains('{') || url.contains('}') {
        return Ok(None);
    }
    Ok(Some(Release {
        version: plain_version(&version).to_string(),
        url,
        date: value.get("date").and_then(|v| v.as_str()).map(str::to_string),
        desc: value.get("desc").and_then(|v| v.as_str()).map(str::to_string),
    }))
}

/// Only `https://` URLs are accepted from the (third-party) app list.
fn is_https(url: &str) -> bool {
    url.len() > 8 && url[..8].eq_ignore_ascii_case("https://")
}

/// A GitHub repository, as the API describes it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepoInfo {
    /// Account that owns the repository.
    pub owner: String,
    /// Repository name.
    pub name: String,
    /// The repository's own one-line description.
    pub description: Option<String>,
    /// How many GitHub users have starred it.
    pub stars: u32,
    /// Last push, ISO-8601.
    pub pushed_at: Option<String>,
    /// Whether the owner has archived it.
    pub archived: bool,
}

/// Whether `name` can be a GitHub account or repository name.
///
/// Both end up in URLs, so anything else is refused rather than escaped.
#[must_use]
pub fn valid_repo_name(name: &str) -> bool {
    !name.is_empty()
        && name.len() <= 100
        && name != "."
        && name != ".."
        && name.bytes().all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.'))
}

fn repo_from_json(value: &serde_json::Value) -> Option<RepoInfo> {
    let owner = value.get("owner")?.get("login")?.as_str()?;
    let name = value.get("name")?.as_str()?;
    if !valid_repo_name(owner) || !valid_repo_name(name) {
        return None;
    }
    Some(RepoInfo {
        owner: owner.to_string(),
        name: name.to_string(),
        description: value
            .get("description")
            .and_then(|d| d.as_str())
            .filter(|d| !d.is_empty())
            .map(str::to_string),
        stars: value
            .get("stargazers_count")
            .and_then(serde_json::Value::as_u64)
            .map_or(0, |n| u32::try_from(n).unwrap_or(u32::MAX)),
        pushed_at: value.get("pushed_at").and_then(|d| d.as_str()).map(str::to_string),
        archived: value.get("archived").and_then(serde_json::Value::as_bool).unwrap_or(false),
    })
}

/// Parse the JSON body of `/repos/{owner}/{repo}`.
pub fn parse_repo(data: &[u8]) -> Result<RepoInfo> {
    let value: serde_json::Value = serde_json::from_slice(data)?;
    repo_from_json(&value).ok_or(Error::MissingField("repository"))
}

/// Parse the JSON body of `/search/repositories`, in the order GitHub ranked it.
pub fn parse_repo_search(data: &[u8]) -> Result<Vec<RepoInfo>> {
    let value: serde_json::Value = serde_json::from_slice(data)?;
    let items =
        value.get("items").and_then(|i| i.as_array()).ok_or(Error::MissingField("items"))?;
    Ok(items.iter().filter_map(repo_from_json).collect())
}

/// Minimal reader for the flat YAML subset the app lists use.
mod yaml {
    /// One `---`-separated document.
    #[derive(Debug, Default)]
    pub struct Document {
        scalars: Vec<(String, String)>,
        maps: Vec<(String, Vec<(String, String)>)>,
    }

    impl Document {
        /// A top-level scalar value.
        pub fn scalar(&self, key: &str) -> Option<&str> {
            self.scalars.iter().find(|(k, _)| k == key).map(|(_, v)| v.as_str())
        }

        /// A top-level nested map.
        pub fn map(&self, key: &str) -> Option<&[(String, String)]> {
            self.maps.iter().find(|(k, _)| k == key).map(|(_, v)| v.as_slice())
        }

        fn is_empty(&self) -> bool {
            self.scalars.is_empty() && self.maps.is_empty()
        }
    }

    /// Split `text` into documents and parse each one.
    pub fn documents(text: &str) -> Vec<Document> {
        let mut docs = Vec::new();
        let mut current = Document::default();
        // Index into `current.maps` of the nested map an indented line belongs
        // to, reset whenever a top-level key is seen.
        let mut open_map: Option<usize> = None;

        for line in text.lines() {
            let trimmed = line.trim_end();
            if trimmed.trim_start() == "---" {
                if !current.is_empty() {
                    docs.push(std::mem::take(&mut current));
                } else {
                    current = Document::default();
                }
                open_map = None;
                continue;
            }
            let stripped = trimmed.trim_start();
            if stripped.is_empty() || stripped.starts_with('#') {
                continue;
            }
            let Some((key, value)) = split_pair(stripped) else {
                continue;
            };
            let indented = trimmed.len() != stripped.len();

            if indented {
                // Only one nesting level is supported; deeper structures and
                // stray indentation are ignored rather than misinterpreted.
                if let Some(index) = open_map
                    && !value.is_empty()
                {
                    current.maps[index].1.push((key, value));
                }
            } else if value.is_empty() {
                current.maps.push((key, Vec::new()));
                open_map = Some(current.maps.len() - 1);
            } else {
                current.scalars.push((key, value));
                open_map = None;
            }
        }
        if !current.is_empty() {
            docs.push(current);
        }
        docs
    }

    /// Split `key: value`, returning the unquoted value (possibly empty).
    fn split_pair(line: &str) -> Option<(String, String)> {
        let (key, value) = line.split_once(':')?;
        let key = key.trim();
        if key.is_empty() || key.contains(' ') {
            return None;
        }
        Some((key.to_string(), unquote(value.trim()).to_string()))
    }

    /// Strip a single layer of matching quotes.
    fn unquote(value: &str) -> &str {
        for quote in ['"', '\''] {
            if value.len() >= 2 && value.starts_with(quote) && value.ends_with(quote) {
                return &value[1..value.len() - 1];
            }
        }
        value
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn splits_documents_and_nested_maps() {
            let docs = documents(
                "---\npackage: a.b\nname: \"A B\"\nrelease:\n  type: github\n  user: u\n---\npackage: c.d\nname: CD\n",
            );
            assert_eq!(docs.len(), 2);
            assert_eq!(docs[0].scalar("package"), Some("a.b"));
            assert_eq!(docs[0].scalar("name"), Some("A B"));
            assert_eq!(docs[0].map("release").unwrap().len(), 2);
            assert_eq!(docs[1].scalar("name"), Some("CD"));
            assert!(docs[1].map("release").is_none());
        }

        #[test]
        fn ignores_comments_blank_lines_and_deep_nesting() {
            let docs = documents("# c\n\npackage: x\nrelease:\n  type: github\n    deep: no\n");
            assert_eq!(docs.len(), 1);
            assert_eq!(docs[0].scalar("package"), Some("x"));
            // The over-indented line is folded into the open map, but nothing
            // outside the subset is invented.
            assert!(docs[0].map("release").unwrap().iter().any(|(k, _)| k == "type"));
        }

        #[test]
        fn a_top_level_key_closes_the_open_map() {
            let docs = documents("release:\n  type: github\nname: after\n  stray: value\n");
            assert_eq!(docs[0].scalar("name"), Some("after"));
            assert_eq!(docs[0].map("release").unwrap().len(), 1);
        }

        #[test]
        fn empty_input_yields_no_documents() {
            assert!(documents("").is_empty());
            assert!(documents("---\n---\n").is_empty());
        }
    }
}

#[cfg(feature = "client")]
mod net {
    use std::io::Read;

    use super::*;

    /// Base URL of the GitHub REST API.
    const API_BASE: &str = "https://api.github.com";
    /// Base URL for raw repository file access.
    const RAW_BASE: &str = "https://raw.githubusercontent.com";
    /// GitHub rejects requests without a User-Agent.
    const USER_AGENT: &str = "pmca-console";
    /// Upper bound for a downloaded APK.
    const MAX_ASSET_SIZE: u64 = 64 * 1024 * 1024;
    /// Upper bound for a metadata document.
    const MAX_METADATA_SIZE: u64 = 4 * 1024 * 1024;
    /// How many search results are asked for; GitHub allows 100 per page.
    const SEARCH_PAGE: u32 = 50;

    /// Fetch and parse the source file `file` of `user/repo` at `branch`.
    pub fn get_store_apps(
        user: &str,
        repo: &str,
        branch: &str,
        file: &str,
    ) -> Result<Vec<StoreApp>> {
        let url = format!("{RAW_BASE}/{user}/{repo}/{branch}/{file}");
        parse_app_list(&fetch(&url, MAX_METADATA_SIZE)?)
    }

    /// Describe one GitHub repository, its stars included.
    pub fn repo_info(owner: &str, repo: &str) -> Result<RepoInfo> {
        parse_repo(&fetch(&format!("{API_BASE}/repos/{owner}/{repo}"), MAX_METADATA_SIZE)?)
    }

    /// The repositories tagged with [`SOURCE_TOPIC`], most-starred first.
    pub fn search_sources() -> Result<Vec<RepoInfo>> {
        let url = format!(
            "{API_BASE}/search/repositories?q=topic:{SOURCE_TOPIC}&sort=stars&order=desc&per_page={SEARCH_PAGE}"
        );
        parse_repo_search(&fetch(&url, MAX_METADATA_SIZE)?)
    }

    /// Resolve an entry's [`ReleaseSource`] to a concrete [`Release`].
    pub fn resolve_release(source: &ReleaseSource, kind: EntryKind) -> Result<Option<Release>> {
        match source {
            ReleaseSource::Github { user, repo } => {
                if !valid_repo_name(user) || !valid_repo_name(repo) {
                    return Ok(None);
                }
                let url = format!("{API_BASE}/repos/{user}/{repo}/releases/latest");
                // GitHub answers 404 while a repository has no release flagged latest.
                match fetch_unless_missing(&url, MAX_METADATA_SIZE)? {
                    Some(data) => parse_github_latest(&data, kind),
                    None => Ok(None),
                }
            }
            ReleaseSource::Yaml { url } => parse_release_feed(&fetch(url, MAX_METADATA_SIZE)?),
            ReleaseSource::Json { url, download } => {
                parse_json_feed(&fetch(url, MAX_METADATA_SIZE)?, download)
            }
            ReleaseSource::Direct(release) => Ok(Some(release.clone())),
            ReleaseSource::None => Ok(None),
        }
    }

    /// Download a release asset (an APK).
    pub fn download_asset(url: &str) -> Result<Vec<u8>> {
        fetch(url, MAX_ASSET_SIZE)
    }

    /// Download arbitrary content from an https URL, capped at `limit` bytes.
    ///
    /// Used by the key import to fetch whatever source the user points at. The
    /// content is never executed, only scanned for key material.
    pub fn download(url: &str, limit: u64) -> Result<Vec<u8>> {
        fetch(url, limit)
    }

    /// Perform a size-capped HTTPS GET.
    ///
    /// Redirects are followed (GitHub serves release files from another host),
    /// but only over HTTPS; what a download contains is checked afterwards.
    fn fetch(url: &str, limit: u64) -> Result<Vec<u8>> {
        fetch_unless_missing(url, limit)?
            .ok_or_else(|| Error::Http(ureq::Error::StatusCode(404).to_string()))
    }

    /// [`fetch`], with `Ok(None)` for a 404.
    fn fetch_unless_missing(url: &str, limit: u64) -> Result<Option<Vec<u8>>> {
        if !is_https(url) {
            return Err(Error::Http(format!("refusing non-https url: {url}")));
        }
        let response = match ureq::get(url)
            .config()
            .https_only(true)
            .build()
            .header("User-Agent", USER_AGENT)
            .call()
        {
            Ok(response) => response,
            Err(ureq::Error::StatusCode(404)) => return Ok(None),
            Err(e) => return Err(Error::Http(e.to_string())),
        };
        let mut buf = Vec::new();
        response
            .into_body()
            .into_reader()
            .take(limit)
            .read_to_end(&mut buf)
            .map_err(|e| Error::Http(e.to_string()))?;
        Ok(Some(buf))
    }
}

#[cfg(feature = "client")]
pub use net::{
    download, download_asset, get_store_apps, repo_info, resolve_release, search_sources,
};

#[cfg(test)]
mod tests {
    use super::*;

    const APPS: &str = "---\npackage: com.example.demo\nname: Demo\nauthor: ma1co\ndesc: A demo app\nhomepage: https://example.com\nrelease:\n  type: github\n  user: ma1co\n  repo: Demo\n\n---\npackage: com.example.feed\nname: Feed\nrelease:\n  type: yaml\n  url: https://example.com/releases.yaml\n\n---\nname: no package\n";

    #[test]
    fn parses_the_app_list() {
        let apps = parse_app_list(APPS.as_bytes()).unwrap();
        assert_eq!(apps.len(), 2);
        assert_eq!(apps[0].package, "com.example.demo");
        assert_eq!(apps[0].author.as_deref(), Some("ma1co"));
        assert_eq!(
            apps[0].release,
            ReleaseSource::Github { user: "ma1co".into(), repo: "Demo".into() }
        );
        assert_eq!(
            apps[1].release,
            ReleaseSource::Yaml { url: "https://example.com/releases.yaml".into() }
        );
    }

    #[test]
    fn rejects_insecure_release_urls() {
        let list =
            "---\npackage: a\nname: A\nrelease:\n  type: yaml\n  url: http://example.com/r.yaml\n";
        assert_eq!(parse_app_list(list.as_bytes()).unwrap()[0].release, ReleaseSource::None);
    }

    #[test]
    fn accepts_a_direct_release() {
        let list = "---\npackage: a\nname: A\nrelease:\n  version: 1.0\n  url: https://example.com/a.apk\n";
        assert_eq!(
            parse_app_list(list.as_bytes()).unwrap()[0].release,
            ReleaseSource::Direct(Release {
                version: "1.0".into(),
                url: "https://example.com/a.apk".into(),
                date: None,
                desc: None,
            })
        );
    }

    #[test]
    fn an_incomplete_release_block_is_dropped() {
        let list = "---\npackage: a\nname: A\nrelease:\n  type: github\n  user: only\n";
        assert_eq!(parse_app_list(list.as_bytes()).unwrap()[0].release, ReleaseSource::None);
    }

    #[test]
    fn reads_the_latest_github_release() {
        let json = br#"{"name":"v1.11.0","tag_name":"v1.11.0","created_at":"2023-01-02T03:04:05Z","body":"notes",
             "assets":[{"content_type":"text/plain","browser_download_url":"https://x/a.txt"},
                       {"content_type":"application/vnd.android.package-archive",
                        "browser_download_url":"https://x/a.apk"}]}"#;
        let release = parse_github_latest(json, EntryKind::App).unwrap().unwrap();
        assert_eq!(release.version, "1.11.0");
        assert_eq!(release.url, "https://x/a.apk");
        assert_eq!(release.date.as_deref(), Some("2023-01-02T03:04:05Z"));
    }

    #[test]
    fn drafts_and_pre_releases_are_never_offered() {
        let asset = r#""assets":[{"name":"a.apk","browser_download_url":"https://x/a.apk"}]"#;
        for flag in ["draft", "prerelease"] {
            let json = format!(r#"{{"tag_name":"v1.11.1-dev.2","{flag}":true,{asset}}}"#);
            assert!(parse_github_latest(json.as_bytes(), EntryKind::App).unwrap().is_none());
        }
    }

    #[test]
    fn a_plugin_entry_picks_the_pmcaplugin_asset() {
        let json = br#"{"tag_name":"v3","assets":[
            {"name":"app.apk","content_type":"application/vnd.android.package-archive","browser_download_url":"https://x/app.apk"},
            {"name":"Demo.PMCAPLUGIN","content_type":"application/zip","browser_download_url":"https://x/demo.pmcaplugin"}]}"#;
        let plugin = parse_github_latest(json, EntryKind::Plugin).unwrap().unwrap();
        assert_eq!(plugin.url, "https://x/demo.pmcaplugin");
        let app = parse_github_latest(json, EntryKind::App).unwrap().unwrap();
        assert_eq!(app.url, "https://x/app.apk");
    }

    #[test]
    fn an_apk_is_recognised_by_its_name_too() {
        let json = br#"{"tag_name":"v1","assets":[{"name":"a.apk","content_type":"application/octet-stream","browser_download_url":"https://x/a.apk"}]}"#;
        assert!(parse_github_latest(json, EntryKind::App).unwrap().is_some());
        assert!(parse_github_latest(json, EntryKind::Plugin).unwrap().is_none());
    }

    #[test]
    fn entries_name_their_kind() {
        let list = "---\npackage: a\nname: A\n---\nkind: plugin\npackage: demo\nname: Demo\n---\nkind: theme\npackage: t\nname: T\n";
        let entries = parse_app_list(list.as_bytes()).unwrap();
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].kind, EntryKind::App);
        assert_eq!(entries[1].kind, EntryKind::Plugin);
        assert_eq!(entries[1].package, "demo");
    }

    #[test]
    fn reads_a_repository_and_a_search() {
        let repo = br#"{"name":"src","owner":{"login":"dev"},"description":"","stargazers_count":42,"pushed_at":"2026-01-02T00:00:00Z","archived":true}"#;
        let info = parse_repo(repo).unwrap();
        assert_eq!((info.owner.as_str(), info.name.as_str(), info.stars), ("dev", "src", 42));
        assert!(info.description.is_none());
        assert!(info.archived);

        let search = br#"{"total_count":2,"items":[
            {"name":"a","owner":{"login":"x"},"stargazers_count":5},
            {"name":"bad/name","owner":{"login":"y"},"stargazers_count":9}]}"#;
        let hits = parse_repo_search(search).unwrap();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].stars, 5);
        assert!(parse_repo_search(br#"{"message":"rate limited"}"#).is_err());
    }

    #[test]
    fn repository_names_are_checked() {
        assert!(valid_repo_name("OpenMemories-AppList"));
        assert!(valid_repo_name("a.b_c"));
        assert!(!valid_repo_name(""));
        assert!(!valid_repo_name(".."));
        assert!(!valid_repo_name("a/b"));
        assert!(!valid_repo_name("a?b"));
    }

    #[test]
    fn falls_back_to_the_tag_name() {
        let json = br#"{"name":"","tag_name":"v9","assets":[{"content_type":"application/vnd.android.package-archive","browser_download_url":"https://x/a.apk"}]}"#;
        assert_eq!(parse_github_latest(json, EntryKind::App).unwrap().unwrap().version, "9");
    }

    #[test]
    fn version_labels_lose_the_v_of_a_tag() {
        assert_eq!(plain_version("v1.11.0"), "1.11.0");
        assert_eq!(plain_version("V2"), "2");
        assert_eq!(plain_version("1.0"), "1.0");
        assert_eq!(plain_version("version 1"), "version 1");
        assert_eq!(plain_version("v"), "v");
    }

    #[test]
    fn a_latest_release_without_an_apk_yields_nothing() {
        let json = br#"{"tag_name":"v1","assets":[]}"#;
        assert!(parse_github_latest(json, EntryKind::App).unwrap().is_none());
        assert!(parse_github_latest(br#"{"tag_name":"v1"}"#, EntryKind::App).unwrap().is_none());
    }

    #[test]
    fn a_response_that_is_no_release_is_rejected() {
        assert!(parse_github_latest(br#"{"message":"rate limited"}"#, EntryKind::App).is_err());
        assert!(parse_github_latest(b"[]", EntryKind::App).is_err());
        assert!(parse_github_latest(b"not json", EntryKind::App).is_err());
    }

    #[test]
    fn parses_a_yaml_release_feed() {
        let feed = "---\nversion: 2.0\nurl: https://example.com/app.apk\ndesc: notes\n";
        let release = parse_release_feed(feed.as_bytes()).unwrap().unwrap();
        assert_eq!(release.version, "2.0");
        assert_eq!(release.desc.as_deref(), Some("notes"));
    }

    #[test]
    fn a_feed_without_a_secure_url_yields_nothing() {
        let feed = "---\nversion: 2.0\nurl: http://example.com/app.apk\n";
        assert!(parse_release_feed(feed.as_bytes()).unwrap().is_none());
    }

    const JSON_ENTRY: &str = "---\npackage: com.example.cam\nname: Cam\ncert_sha256: AB:cd 01\nrelease:\n  type: json\n  url: https://example.com/cam-release.json\n  download: https://dl.example.com/CAM-v{version}.apk?build={build}\n";

    #[test]
    fn reads_a_json_release_entry_and_its_fingerprint() {
        let apps = parse_app_list(JSON_ENTRY.as_bytes()).unwrap();
        assert_eq!(
            apps[0].release,
            ReleaseSource::Json {
                url: "https://example.com/cam-release.json".into(),
                download: "https://dl.example.com/CAM-v{version}.apk?build={build}".into(),
            }
        );
        assert_eq!(apps[0].cert_sha256.as_deref(), Some("abcd01"));
        assert_eq!(parse_app_list(APPS.as_bytes()).unwrap()[0].cert_sha256, None);
    }

    #[test]
    fn a_json_entry_needs_secure_urls_and_a_version_placeholder() {
        let entry = |url: &str, download: &str| {
            format!(
                "---\npackage: p\nname: n\nrelease:\n  type: json\n  url: {url}\n  download: {download}\n"
            )
        };
        for (url, download) in [
            ("http://example.com/r.json", "https://example.com/{version}.apk"),
            ("https://example.com/r.json", "http://example.com/{version}.apk"),
            ("https://example.com/r.json", "https://example.com/latest.apk"),
        ] {
            let apps = parse_app_list(entry(url, download).as_bytes()).unwrap();
            assert_eq!(apps[0].release, ReleaseSource::None, "{url} {download}");
        }
    }

    #[test]
    fn a_json_feed_fills_the_download_template() {
        let template = "https://dl.example.com/CAM-v{version}.apk?build={build}";
        let release = parse_json_feed(br#"{"schema":1,"version":"2.05","build":58}"#, template)
            .unwrap()
            .unwrap();
        assert_eq!(release.version, "2.05");
        assert_eq!(release.url, "https://dl.example.com/CAM-v2.05.apk?build=58");

        // A number works as a version, and a template without {build} needs none.
        let release = parse_json_feed(br#"{"version":3}"#, "https://dl.example.com/{version}.apk")
            .unwrap()
            .unwrap();
        assert_eq!(release.url, "https://dl.example.com/3.apk");
    }

    #[test]
    fn a_json_feed_refuses_unsafe_or_missing_values() {
        let template = "https://dl.example.com/CAM-v{version}.apk?build={build}";
        for body in [
            r#"{"build":58}"#,
            r#"{"version":"2.05"}"#,
            r#"{"version":"2.05/../x","build":1}"#,
            r#"{"version":"2.05","build":"1&x=evil"}"#,
            r#"{"version":"","build":1}"#,
            r#"{"version":["2"],"build":1}"#,
        ] {
            assert!(parse_json_feed(body.as_bytes(), template).unwrap().is_none(), "{body}");
        }
        assert!(parse_json_feed(b"not json", template).is_err());
    }

    #[test]
    fn https_detection_is_case_insensitive_and_strict() {
        assert!(is_https("HTTPS://example.com/a"));
        assert!(!is_https("https://"));
        assert!(!is_https("ftp://example.com"));
        assert!(!is_https("file:///etc/passwd"));
    }
}
