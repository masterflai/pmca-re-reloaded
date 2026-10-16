//! Community app store – port of `pmca/appstore`.
//!
//! The list of installable community apps lives in a YAML file in a GitHub
//! repository (`ma1co/OpenMemories-AppList` by default). Each entry names a
//! release source, which is resolved to a downloadable APK either through the
//! GitHub releases API, through a second YAML feed, or directly.
//!
//! Everything that interprets remote data ([`parse_app_list`],
//! [`parse_github_releases`], [`parse_release_feed`]) is pure and unit-tested;
//! only the fetching lives behind the `client` feature.
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

/// GitHub user owning the default app list.
pub const DEFAULT_LIST_USER: &str = "ma1co";
/// Repository holding the default app list.
pub const DEFAULT_LIST_REPO: &str = "OpenMemories-AppList";
/// Branch of the default app list.
pub const DEFAULT_LIST_BRANCH: &str = "master";
/// File name of the default app list.
pub const DEFAULT_LIST_FILE: &str = "apps.yaml";

/// Asset content type GitHub reports for an APK.
const APK_CONTENT_TYPE: &str = "application/vnd.android.package-archive";

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

/// An app offered by the community store.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreApp {
    /// Android package name; the store's primary key.
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
}

/// Parse `apps.yaml` into the list of apps, in file order.
///
/// Entries without both `package` and `name` are skipped.
pub fn parse_app_list(data: &[u8]) -> Result<Vec<StoreApp>> {
    let text = String::from_utf8_lossy(data);
    Ok(yaml::documents(&text)
        .into_iter()
        .filter_map(|doc| {
            let package = doc.scalar("package")?.to_string();
            let name = doc.scalar("name")?.to_string();
            Some(StoreApp {
                package,
                name,
                author: doc.scalar("author").map(str::to_string),
                desc: doc.scalar("desc").map(str::to_string),
                homepage: doc.scalar("homepage").map(str::to_string),
                release: release_source(&doc),
            })
        })
        .collect())
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
        _ => match (get("version"), get("url")) {
            (Some(version), Some(url)) if is_https(url) => ReleaseSource::Direct(Release {
                version: version.to_string(),
                url: url.to_string(),
                date: get("date").map(str::to_string),
                desc: get("desc").map(str::to_string),
            }),
            _ => ReleaseSource::None,
        },
    }
}

/// Pick the newest GitHub release that ships an APK asset.
///
/// `data` is the JSON body of `/repos/{user}/{repo}/releases`, which GitHub
/// returns newest-first.
pub fn parse_github_releases(data: &[u8]) -> Result<Option<Release>> {
    let value: serde_json::Value = serde_json::from_slice(data)?;
    let releases = value.as_array().ok_or(Error::MissingField("releases"))?;

    for release in releases {
        let Some(url) = release
            .get("assets")
            .and_then(|a| a.as_array())
            .into_iter()
            .flatten()
            .find(|asset| {
                asset.get("content_type").and_then(|c| c.as_str()) == Some(APK_CONTENT_TYPE)
            })
            .and_then(|asset| asset.get("browser_download_url"))
            .and_then(|u| u.as_str())
            .filter(|u| is_https(u))
        else {
            continue;
        };
        let version = release
            .get("name")
            .and_then(|v| v.as_str())
            .filter(|s| !s.is_empty())
            .or_else(|| release.get("tag_name").and_then(|v| v.as_str()))
            .unwrap_or_default();
        return Ok(Some(Release {
            version: version.to_string(),
            url: url.to_string(),
            date: release.get("created_at").and_then(|v| v.as_str()).map(str::to_string),
            desc: release.get("body").and_then(|v| v.as_str()).map(str::to_string),
        }));
    }
    Ok(None)
}

/// Pick the first entry of a YAML release feed that has a version and an URL.
pub fn parse_release_feed(data: &[u8]) -> Result<Option<Release>> {
    let text = String::from_utf8_lossy(data);
    Ok(yaml::documents(&text).into_iter().find_map(|doc| {
        let version = doc.scalar("version")?;
        let url = doc.scalar("url").filter(|u| is_https(u))?;
        Some(Release {
            version: version.to_string(),
            url: url.to_string(),
            date: doc.scalar("date").map(str::to_string),
            desc: doc.scalar("desc").map(str::to_string),
        })
    }))
}

/// Only `https://` URLs are accepted from the (third-party) app list.
fn is_https(url: &str) -> bool {
    url.len() > 8 && url[..8].eq_ignore_ascii_case("https://")
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

    /// Fetch and parse the community app list.
    pub fn get_store_apps(
        user: &str,
        repo: &str,
        branch: &str,
        file: &str,
    ) -> Result<Vec<StoreApp>> {
        let url = format!("{RAW_BASE}/{user}/{repo}/{branch}/{file}");
        parse_app_list(&fetch(&url, MAX_METADATA_SIZE)?)
    }

    /// Resolve an app's [`ReleaseSource`] to a concrete [`Release`].
    pub fn resolve_release(source: &ReleaseSource) -> Result<Option<Release>> {
        match source {
            ReleaseSource::Github { user, repo } => {
                let url = format!("{API_BASE}/repos/{user}/{repo}/releases");
                parse_github_releases(&fetch(&url, MAX_METADATA_SIZE)?)
            }
            ReleaseSource::Yaml { url } => parse_release_feed(&fetch(url, MAX_METADATA_SIZE)?),
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
    fn fetch(url: &str, limit: u64) -> Result<Vec<u8>> {
        if !is_https(url) {
            return Err(Error::Http(format!("refusing non-https url: {url}")));
        }
        let response = ureq::get(url)
            .header("User-Agent", USER_AGENT)
            .call()
            .map_err(|e| Error::Http(e.to_string()))?;
        let mut buf = Vec::new();
        response
            .into_body()
            .into_reader()
            .take(limit)
            .read_to_end(&mut buf)
            .map_err(|e| Error::Http(e.to_string()))?;
        Ok(buf)
    }
}

#[cfg(feature = "client")]
pub use net::{download, download_asset, get_store_apps, resolve_release};

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
    fn picks_the_first_github_release_with_an_apk() {
        let json = br#"[
            {"name":"","tag_name":"v2","created_at":"2024-01-02T03:04:05Z","body":"notes",
             "assets":[{"content_type":"text/plain","browser_download_url":"https://x/a.txt"}]},
            {"name":"v1","created_at":"2023-01-02T03:04:05Z","body":"older",
             "assets":[{"content_type":"application/vnd.android.package-archive",
                        "browser_download_url":"https://x/a.apk"}]}
        ]"#;
        let release = parse_github_releases(json).unwrap().unwrap();
        assert_eq!(release.version, "v1");
        assert_eq!(release.url, "https://x/a.apk");
        assert_eq!(release.date.as_deref(), Some("2023-01-02T03:04:05Z"));
    }

    #[test]
    fn falls_back_to_the_tag_name() {
        let json = br#"[{"tag_name":"v9","assets":[{"content_type":"application/vnd.android.package-archive","browser_download_url":"https://x/a.apk"}]}]"#;
        assert_eq!(parse_github_releases(json).unwrap().unwrap().version, "v9");
    }

    #[test]
    fn releases_without_an_apk_yield_nothing() {
        assert!(parse_github_releases(b"[]").unwrap().is_none());
        assert!(parse_github_releases(br#"[{"assets":[]}]"#).unwrap().is_none());
    }

    #[test]
    fn a_non_array_release_response_is_rejected() {
        assert!(parse_github_releases(br#"{"message":"rate limited"}"#).is_err());
        assert!(parse_github_releases(b"not json").is_err());
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

    #[test]
    fn https_detection_is_case_insensitive_and_strict() {
        assert!(is_https("HTTPS://example.com/a"));
        assert!(!is_https("https://"));
        assert!(!is_https("ftp://example.com"));
        assert!(!is_https("file:///etc/passwd"));
    }
}
