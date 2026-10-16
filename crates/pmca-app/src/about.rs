//! Provenance, licence and acknowledgements, shared by both front-ends.
//!
//! The wording lives in the translation catalogue like every other string, but
//! *which* paragraphs there are, and in what order, is decided here so the CLI
//! and the GUI cannot drift apart. A front-end walks [`SECTIONS`], asks the
//! localizer for each key and renders the result; adding a paragraph is a
//! change in one place.
//!
//! Two things are deliberately not translatable:
//!
//!  * The licence text itself. It is a legal document; a translation beside it
//!    would be a second, unauthorised wording. It is embedded from
//!    `LICENSE.txt` with [`include_str!`], so what is shown and what ships can
//!    never disagree.
//!  * The project addresses, which are not language-dependent.

use serde::Serialize;

/// The full licence, taken from the repository's own `LICENSE.txt`.
pub const LICENSE_TEXT: &str = include_str!("../../../LICENSE.txt");

/// Every dependency's licence and copyright notice, from `THIRD-PARTY.md`.
///
/// Embedded rather than merely shipped beside the binary, because the binary
/// is statically linked: it *contains* the code of those packages, and the MIT
/// and Apache-2.0 licences both require their notices to travel with it. A
/// file left in the repository does not travel with a copied executable, so a
/// reader who has only the program would have no way to see them.
pub const THIRD_PARTY_TEXT: &str = include_str!("../../../THIRD-PARTY.md");

/// SPDX identifier of the licence both the original and this port use.
pub const LICENSE_SPDX: &str = "MIT";

/// Copyright holders, oldest first.
///
/// The upstream notices are reproduced unchanged because the MIT licence
/// requires it: the 2015 line covers Sony-PMCA-RE, the 2017 line the
/// OpenMemories-Platform updater body bundled under `updatershell/bodyfs`, and
/// the last line the port.
pub const COPYRIGHT: &[&str] =
    &["Copyright (c) 2015 ma1co", "Copyright (c) 2017 ma1co", "Copyright (c) 2026 masterflai"];

/// Everyone besides ma1co whose work is in here: the other contributors to
/// Sony-PMCA-RE, then the authors of forks whose changes were carried over.
pub const CONTRIBUTORS: &[&str] =
    &["undingen", "paulbellamy", "kratz00", "mungewell", "LAK132", "0xPIT"];

/// A heading and the paragraphs below it, all as catalogue keys.
///
/// Borrowed rather than owned throughout, and therefore only serializable: the
/// page is composed here and travels one way, to whichever front-end asked for
/// it. Nothing sends one back.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct AboutSection {
    /// Catalogue key of the heading.
    pub heading: &'static str,
    /// Catalogue keys of the paragraphs, in reading order.
    pub body: &'static [&'static str],
    /// Names listed below the paragraphs, shown as they are.
    pub people: &'static [&'static str],
}

/// A project this one builds on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct AboutLink {
    /// Catalogue key naming the project.
    pub label: &'static str,
    /// Address of the project.
    pub url: &'static str,
}

/// The page, in the order it is meant to be read.
///
/// Gratitude comes first because it is the point: the protocols, the camera
/// detection, the updater procedure and service mode were all worked out by
/// others, years ago. The remaining sections explain why a port exists at all,
/// how it came about, and under what terms it may be used.
///
/// The legal section sits last because it is the least interesting and the
/// most necessary. It is on this page rather than buried in a file because a
/// reader who wonders whether Sony publishes this needs the answer where they
/// are looking, not in a document they would have to go and find.
///
/// The data section is here for the same reason. Nothing is collected, but
/// two features do reach the network on their own. Someone who assumed an
/// offline tool would want to learn that from the tool rather than from a
/// packet capture.
pub const SECTIONS: &[AboutSection] = &[
    AboutSection {
        heading: "about-thanks-heading",
        body: &["about-thanks-body-1", "about-thanks-body-2"],
        people: &[],
    },
    AboutSection {
        heading: "about-contributors-heading",
        body: &["about-contributors-body"],
        people: CONTRIBUTORS,
    },
    AboutSection {
        heading: "about-purpose-heading",
        body: &["about-purpose-body-1", "about-purpose-body-2"],
        people: &[],
    },
    AboutSection {
        heading: "about-origin-heading",
        body: &["about-origin-body-1", "about-origin-body-2"],
        people: &[],
    },
    AboutSection { heading: "about-license-heading", body: &["about-license-body"], people: &[] },
    AboutSection {
        heading: "about-privacy-heading",
        body: &[
            "about-privacy-body-1",
            "about-privacy-body-2",
            "about-privacy-body-3",
            "about-privacy-body-4",
        ],
        people: &[],
    },
    AboutSection {
        heading: "about-legal-heading",
        body: &["about-legal-trademark", "about-legal-own-device", "about-legal-warranty"],
        people: &[],
    },
];

/// The original project and the work that grew around it.
pub const LINKS: &[AboutLink] = &[
    AboutLink { label: "about-link-original", url: "https://github.com/ma1co/Sony-PMCA-RE" },
    AboutLink { label: "about-link-fwtool", url: "https://github.com/ma1co/fwtool.py" },
    AboutLink { label: "about-link-tweak", url: "https://github.com/ma1co/OpenMemories-Tweak" },
    AboutLink { label: "about-link-applist", url: "https://github.com/ma1co/OpenMemories-AppList" },
    AboutLink {
        label: "about-link-platform",
        url: "https://github.com/ma1co/OpenMemories-Platform",
    },
    AboutLink {
        label: "about-link-devices",
        url: "https://openmemories.readthedocs.io/devices.html",
    },
    AboutLink {
        label: "about-link-nexhack",
        // The wiki is offline; this is its last complete archived copy.
        url: "https://web.archive.org/web/20201027234246/http://www.personal-view.com/faqs/sony-hack/hack-development",
    },
    AboutLink { label: "about-link-zve10", url: "https://github.com/qn06142/zv-e10-re" },
];

/// Everything a front-end needs that is not a catalogue key.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct AboutInfo {
    /// Version of the running build.
    pub version: String,
    /// SPDX identifier, see [`LICENSE_SPDX`].
    pub license: &'static str,
    /// The licence in full, see [`LICENSE_TEXT`].
    pub license_text: &'static str,
    /// The dependencies' licences, see [`THIRD_PARTY_TEXT`].
    pub third_party_text: &'static str,
    /// Copyright notices, see [`COPYRIGHT`].
    pub copyright: &'static [&'static str],
    /// The sections to render, see [`SECTIONS`].
    pub sections: &'static [AboutSection],
    /// The projects to list, see [`LINKS`].
    pub links: &'static [AboutLink],
}

/// Collect the page for the calling front-end.
///
/// The version is that of the *workspace*, which both binaries share, so the
/// number shown is the same whichever one is asked.
#[must_use]
pub fn info() -> AboutInfo {
    AboutInfo {
        version: env!("CARGO_PKG_VERSION").to_owned(),
        license: LICENSE_SPDX,
        license_text: LICENSE_TEXT,
        third_party_text: THIRD_PARTY_TEXT,
        copyright: COPYRIGHT,
        sections: SECTIONS,
        links: LINKS,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pmca_i18n::Localizer;

    /// The embedded text has to be the licence, not some other file.
    #[test]
    fn the_embedded_licence_is_the_mit_licence() {
        assert!(LICENSE_TEXT.contains("The MIT License (MIT)"));
        assert!(LICENSE_TEXT.contains("WITHOUT WARRANTY OF ANY KIND"));
    }

    /// Dropping an upstream notice would breach the licence being claimed.
    #[test]
    fn the_original_copyright_notice_is_reproduced() {
        assert!(LICENSE_TEXT.contains("Copyright (c) 2015 ma1co"));
        assert!(COPYRIGHT.contains(&"Copyright (c) 2015 ma1co"));
        // The bundled OpenMemories-Platform updater body, MIT (c) 2017 ma1co.
        assert!(COPYRIGHT.contains(&"Copyright (c) 2017 ma1co"));
    }

    /// A missing translation would show the bare key on the page.
    #[test]
    fn every_key_the_page_uses_is_translated() {
        let l = Localizer::new("en-US");
        let keys = SECTIONS
            .iter()
            .flat_map(|s| std::iter::once(s.heading).chain(s.body.iter().copied()))
            .chain(LINKS.iter().map(|link| link.label))
            .chain([
                "about-links-heading",
                "about-version",
                "about-third-party-heading",
                "about-third-party-body",
            ]);
        for key in keys {
            assert_ne!(l.tr(key), key, "{key} is missing from the catalogue");
        }
    }

    /// The notices only satisfy the licences if they are actually in there.
    ///
    /// Checks the generated header and one package known to be present, so
    /// that an empty or truncated file cannot pass as a complete one.
    #[test]
    fn the_third_party_notices_are_embedded() {
        assert!(THIRD_PARTY_TEXT.contains("Third-party licences"));
        assert!(THIRD_PARTY_TEXT.contains("Licence: `MIT`"));
        assert!(THIRD_PARTY_TEXT.len() > 10_000, "file looks truncated");
    }

    /// A typo in an address is not something a reader can recover from.
    #[test]
    fn every_link_is_an_https_address() {
        for link in LINKS {
            assert!(link.url.starts_with("https://"), "{}", link.url);
        }
    }
}
