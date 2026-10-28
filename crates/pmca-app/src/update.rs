//! Check for a newer release of this program.
//!
//! One request to GitHub's "latest release" endpoint, which leaves out drafts
//! and pre-releases. The front-end calls this at start only when the user has
//! switched that on, and whenever the user asks for it.

use crate::Result;
use crate::dto::AvailableUpdate;

const LATEST_RELEASE_API: &str =
    "https://api.github.com/repos/masterflai/pmca-re-reloaded/releases/latest";

/// Fixed rather than taken from the response, so the address shown is always ours.
pub const RELEASES_PAGE: &str = "https://github.com/masterflai/pmca-re-reloaded/releases/latest";

/// The response is a few kilobytes; anything near this is not GitHub's answer.
const MAX_RESPONSE_BYTES: u64 = 1024 * 1024;

/// Ask GitHub for the newest release and report it if it is newer than this build.
pub fn check() -> Result<Option<AvailableUpdate>> {
    let body = pmca_market::appstore::download(LATEST_RELEASE_API, MAX_RESPONSE_BYTES)?;
    newer_release(&body, env!("CARGO_PKG_VERSION"))
}

fn newer_release(body: &[u8], current: &str) -> Result<Option<AvailableUpdate>> {
    let value: serde_json::Value =
        serde_json::from_slice(body).map_err(pmca_market::Error::from)?;
    let tag = value
        .get("tag_name")
        .and_then(|v| v.as_str())
        .ok_or(pmca_market::Error::MissingField("tag_name"))?;
    let latest = tag.strip_prefix('v').unwrap_or(tag);
    let (Some(new), Some(old)) = (parse_version(latest), parse_version(current)) else {
        return Ok(None);
    };
    Ok((new > old).then(|| AvailableUpdate {
        current: current.to_string(),
        latest: latest.to_string(),
        url: RELEASES_PAGE.to_string(),
    }))
}

/// `major.minor.patch`; a pre-release or build suffix is ignored.
fn parse_version(text: &str) -> Option<(u64, u64, u64)> {
    let core = text.split(['-', '+']).next()?;
    let mut parts = core.split('.').map(|p| p.parse::<u64>().ok());
    let version = (parts.next()??, parts.next()??, parts.next()??);
    parts.next().is_none().then_some(version)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn release(tag: &str) -> Vec<u8> {
        format!(r#"{{"tag_name":"{tag}","html_url":"https://example.invalid/"}}"#).into_bytes()
    }

    #[test]
    fn reports_a_newer_release() {
        let update = newer_release(&release("v1.2.0"), "1.1.4").unwrap().unwrap();
        assert_eq!(update.current, "1.1.4");
        assert_eq!(update.latest, "1.2.0");
        assert_eq!(update.url, RELEASES_PAGE);
    }

    #[test]
    fn compares_numerically_not_textually() {
        assert!(newer_release(&release("v1.1.10"), "1.1.9").unwrap().is_some());
        assert!(newer_release(&release("v1.1.9"), "1.1.10").unwrap().is_none());
    }

    #[test]
    fn stays_quiet_for_the_same_or_an_older_release() {
        assert!(newer_release(&release("v1.1.4"), "1.1.4").unwrap().is_none());
        assert!(newer_release(&release("v1.0.0"), "1.1.4").unwrap().is_none());
    }

    #[test]
    fn ignores_tags_that_are_not_versions() {
        assert!(newer_release(&release("nightly"), "1.1.4").unwrap().is_none());
        assert!(newer_release(&release("v2.0"), "1.1.4").unwrap().is_none());
    }

    #[test]
    fn rejects_a_response_without_a_tag() {
        assert!(newer_release(b"{}", "1.1.4").is_err());
        assert!(newer_release(b"not json", "1.1.4").is_err());
    }

    #[test]
    fn parses_versions_with_a_suffix() {
        assert_eq!(parse_version("1.2.3-rc.1"), Some((1, 2, 3)));
        assert_eq!(parse_version("1.2.3+build"), Some((1, 2, 3)));
        assert_eq!(parse_version("1.2.3.4"), None);
    }
}
