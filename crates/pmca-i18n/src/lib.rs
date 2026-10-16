//! # pmca-i18n
//!
//! Lightweight localization layer shared by the CLI and the GUI.
//!
//! Translations live in **standalone Fluent files** under `locales/<lang>/`
//! (one directory per language), so translators only ever edit `.ftl` text
//! files — never code. New languages are added by dropping in a new directory;
//! they are discovered automatically at build time via [`rust_embed`].
//!
//! `en-US` is the source/fallback locale. Missing keys in any locale fall back
//! to `en-US`, and a missing message id falls back to the id itself, so the UI
//! can never show an empty string.
//!
//! Users may correct a shipped translation themselves; see [`catalog`] for the
//! download/edit/upload round-trip and where the result is kept.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod catalog;

use fluent::{FluentArgs, FluentBundle, FluentResource};
use fluent_langneg::{NegotiationStrategy, negotiate_languages};
use rust_embed::RustEmbed;
use std::collections::HashMap;
use std::sync::OnceLock;
use unic_langid::LanguageIdentifier;

/// Embeds every file under the workspace `locales/` directory into the binary.
#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/../../locales"]
struct Locales;

/// The default/fallback locale identifier string.
pub const DEFAULT_LOCALE: &str = "en-US";

/// The raw Fluent source for `locale`, as embedded in the binary.
///
/// The GUI front-end needs the same catalogue the backend uses. Serving it from
/// here keeps a single embedded copy authoritative: a second copy shipped
/// alongside the web assets could drift out of sync with this one, and the
/// resulting mismatch would only show up as untranslated labels at runtime.
pub fn locale_source(locale: &str) -> Option<String> {
    let file = Locales::get(&format!("{locale}/pmca.ftl"))?;
    Some(String::from_utf8_lossy(&file.data).into_owned())
}

/// Whether `locale` is one of the catalogues built into the program.
pub fn is_shipped(locale: &str) -> bool {
    Locales::get(&format!("{locale}/pmca.ftl")).is_some()
}

/// Catalogues contributed by installed plugins, as `(locale, Fluent source)`.
static PLUGIN_CATALOGS: OnceLock<Vec<(String, String)>> = OnceLock::new();

/// Register the catalogues of the installed plugins, once per process.
///
/// They are layered *under* the built-in and user-edited catalogues: a message
/// id that already exists keeps its text, so a plugin can add a language or its
/// own `plg-` messages but never reword one of the program's warnings. Returns
/// `false` if catalogues were already registered.
pub fn register_plugin_catalogs(catalogs: Vec<(String, String)>) -> bool {
    PLUGIN_CATALOGS.set(catalogs).is_ok()
}

/// The plugin catalogues registered for `locale`, in registration order.
pub(crate) fn plugin_sources(locale: &str) -> impl Iterator<Item = &'static str> {
    PLUGIN_CATALOGS
        .get()
        .map_or(&[][..], Vec::as_slice)
        .iter()
        .filter(move |(lang, _)| lang == locale)
        .map(|(_, source)| source.as_str())
}

/// Check a catalogue a plugin brings for `locale` and return its message ids.
///
/// The rules are the ones the window's line-based reader needs — one message
/// per line — plus one of policy: for a language the program ships, a plugin
/// may only add messages of its own, named with `own_prefix`. A language the
/// program does not ship may be a complete language pack.
pub fn check_plugin_catalog(
    locale: &str,
    source: &str,
    own_prefix: &str,
) -> Result<Vec<String>, String> {
    let canonical = locale
        .parse::<LanguageIdentifier>()
        .map_err(|_| format!("`{locale}` is not a language tag"))?
        .to_string();
    if canonical != locale {
        return Err(format!("`{locale}` must be written as `{canonical}`"));
    }
    if FluentResource::try_new(source.to_string()).is_err() {
        return Err(format!("{locale}: not a valid Fluent catalogue"));
    }
    let shipped = is_shipped(locale);
    let mut ids = Vec::new();
    for line in source.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if line.starts_with([' ', '\t']) {
            return Err(format!("{locale}: every message must fit on one line"));
        }
        let Some((id, _)) = line.split_once('=') else {
            return Err(format!("{locale}: `{trimmed}` is not a `key = value` line"));
        };
        let id = id.trim();
        if shipped && !id.starts_with(own_prefix) {
            return Err(format!(
                "{locale}: `{id}` belongs to the program; a plugin may only add its own \
                 messages (`{own_prefix}…`) to a language the program ships"
            ));
        }
        ids.push(id.to_string());
    }
    Ok(ids)
}

/// A loaded set of translation bundles with language negotiation.
pub struct Localizer {
    bundles: HashMap<String, FluentBundle<FluentResource>>,
    available: Vec<LanguageIdentifier>,
    current: String,
}

impl Localizer {
    /// Load all embedded locales and select `requested` (falling back to
    /// `en-US` when the requested language is unavailable).
    ///
    /// Catalogues the user has edited (see [`catalog`]) are layered on top of
    /// the embedded ones. That means this constructor reads from the user's
    /// configuration directory, and that a correction the user made in the
    /// window also reaches the messages produced down here.
    pub fn new(requested: &str) -> Self {
        let mut bundles = HashMap::new();
        let mut available = Vec::new();

        for file in Locales::iter() {
            // Paths look like "de-DE/pmca.ftl".
            let path = file.as_ref();
            let Some((lang, _)) = path.split_once('/') else {
                continue;
            };
            if !path.ends_with(".ftl") {
                continue;
            }
            let Some(content) = Locales::get(path) else {
                continue;
            };
            let source = String::from_utf8_lossy(&content.data).into_owned();
            let Ok(res) = FluentResource::try_new(source) else {
                continue;
            };
            let langid: LanguageIdentifier = match lang.parse() {
                Ok(id) => id,
                Err(_) => continue,
            };
            let bundle = bundles.entry(lang.to_string()).or_insert_with(|| {
                let mut b = FluentBundle::new(vec![langid.clone()]);
                // Disable Unicode isolation marks so CLI output stays clean.
                b.set_use_isolating(false);
                available.push(langid.clone());
                b
            });
            // Ignore duplicate-id errors; the first definition wins.
            let _ = bundle.add_resource(res);
        }

        // Applied after the embedded catalogues rather than in the loop above,
        // because only `add_resource_overriding` lets a later definition win —
        // which is the entire purpose of a user-supplied correction.
        let langs: Vec<String> = bundles.keys().cloned().collect();
        for lang in langs {
            let Some(source) = catalog::user_catalog_source(&lang) else {
                continue;
            };
            let Ok(res) = FluentResource::try_new(source) else {
                continue;
            };
            if let Some(bundle) = bundles.get_mut(&lang) {
                bundle.add_resource_overriding(res);
            }
        }

        // Plugin catalogues come last and without overriding, so every id the
        // program already defines keeps its text. A plugin language that is not
        // shipped gets a bundle of its own and falls back to en-US per message.
        for (lang, source) in PLUGIN_CATALOGS.get().map_or(&[][..], Vec::as_slice) {
            let (Ok(res), Ok(langid)) =
                (FluentResource::try_new(source.clone()), lang.parse::<LanguageIdentifier>())
            else {
                continue;
            };
            let bundle = bundles.entry(lang.clone()).or_insert_with(|| {
                let mut b = FluentBundle::new(vec![langid.clone()]);
                b.set_use_isolating(false);
                available.push(langid.clone());
                b
            });
            let _ = bundle.add_resource(res);
        }

        let current = Self::negotiate(&available, requested);
        Self { bundles, available, current }
    }

    fn negotiate(available: &[LanguageIdentifier], requested: &str) -> String {
        let default: LanguageIdentifier = DEFAULT_LOCALE.parse().unwrap();
        let requested_id: Vec<LanguageIdentifier> = requested.parse().into_iter().collect();
        let selected = negotiate_languages(
            &requested_id,
            available,
            Some(&default),
            NegotiationStrategy::Filtering,
        );
        selected.first().map(|l| l.to_string()).unwrap_or_else(|| DEFAULT_LOCALE.to_string())
    }

    /// The currently selected locale (e.g. `"de-DE"`).
    pub fn current(&self) -> &str {
        &self.current
    }

    /// List of available locale identifiers, e.g. for a language picker.
    pub fn available(&self) -> Vec<String> {
        self.available.iter().map(|l| l.to_string()).collect()
    }

    /// Translate `id`, falling back to `en-US` and then to the id itself.
    pub fn tr(&self, id: &str) -> String {
        self.format(&self.current, id)
            .or_else(|| self.format(DEFAULT_LOCALE, id))
            .unwrap_or_else(|| id.to_string())
    }

    /// Translate `id`, substituting a single named argument `{ $name }`.
    ///
    /// Falls back to `en-US`, then to the id itself, mirroring [`Self::tr`].
    pub fn tr_arg(&self, id: &str, name: &str, value: &str) -> String {
        self.tr_args(id, &[(name, value)])
    }

    /// Translate `id`, substituting several named arguments.
    ///
    /// A message that names two things — a source and a destination, say — can
    /// only be worded naturally if the translator may put them in either order,
    /// which needs both to be available as placeables rather than one being
    /// pre-formatted into the other.
    ///
    /// Falls back to `en-US`, then to the id itself, mirroring [`Self::tr`].
    pub fn tr_args(&self, id: &str, args: &[(&str, &str)]) -> String {
        let mut fluent = FluentArgs::new();
        for (name, value) in args {
            fluent.set((*name).to_string(), (*value).to_string());
        }
        self.format_args(&self.current, id, &fluent)
            .or_else(|| self.format_args(DEFAULT_LOCALE, id, &fluent))
            .unwrap_or_else(|| id.to_string())
    }

    fn format(&self, locale: &str, id: &str) -> Option<String> {
        let bundle = self.bundles.get(locale)?;
        let msg = bundle.get_message(id)?;
        let pattern = msg.value()?;
        let mut errors = Vec::new();
        let s = bundle.format_pattern(pattern, None, &mut errors);
        if errors.is_empty() { Some(s.into_owned()) } else { None }
    }

    fn format_args(&self, locale: &str, id: &str, args: &FluentArgs) -> Option<String> {
        let bundle = self.bundles.get(locale)?;
        let msg = bundle.get_message(id)?;
        let pattern = msg.value()?;
        let mut errors = Vec::new();
        let s = bundle.format_pattern(pattern, Some(args), &mut errors);
        if errors.is_empty() { Some(s.into_owned()) } else { None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_default_and_translates_known_key() {
        let l = Localizer::new("en-US");
        assert_eq!(l.current(), "en-US");
        // app-title exists in every locale file shipped with the project.
        assert_ne!(l.tr("app-title"), "app-title");
    }

    #[test]
    fn german_is_selected_when_requested() {
        let l = Localizer::new("de-DE");
        assert_eq!(l.current(), "de-DE");
    }

    #[test]
    fn unknown_language_falls_back_to_default() {
        let l = Localizer::new("xx-XX");
        assert_eq!(l.current(), DEFAULT_LOCALE);
    }

    #[test]
    fn unknown_key_returns_the_key() {
        let l = Localizer::new("en-US");
        assert_eq!(l.tr("this-key-does-not-exist"), "this-key-does-not-exist");
    }

    #[test]
    fn the_raw_catalogue_is_served_for_every_available_locale() {
        // The GUI reads its catalogue through this function, so a locale the
        // negotiator offers but the loader cannot serve would leave the window
        // showing bare message ids.
        for locale in Localizer::new(DEFAULT_LOCALE).available() {
            let source = locale_source(&locale)
                .unwrap_or_else(|| panic!("no catalogue for advertised locale {locale}"));
            assert!(source.contains(" = "), "{locale} catalogue has no messages");
        }
    }

    #[test]
    fn an_unknown_locale_has_no_catalogue() {
        assert!(locale_source("xx-XX").is_none());
    }

    /// Collect the message ids of a catalogue, ignoring comments, blank lines
    /// and the indented continuation lines of multi-line messages.
    fn message_ids(source: &str) -> Vec<String> {
        source
            .lines()
            .filter(|line| !line.starts_with([' ', '\t', '#']))
            .filter_map(|line| line.split_once('=').map(|(id, _)| id.trim().to_string()))
            .filter(|id| !id.is_empty())
            .collect()
    }

    #[test]
    fn every_locale_defines_the_same_message_ids_as_the_default() {
        let default = message_ids(&locale_source(DEFAULT_LOCALE).expect("default catalogue"));

        for locale in Localizer::new(DEFAULT_LOCALE).available() {
            let ids = message_ids(&locale_source(&locale).expect("catalogue"));
            let missing: Vec<_> = default.iter().filter(|id| !ids.contains(id)).collect();
            let extra: Vec<_> = ids.iter().filter(|id| !default.contains(id)).collect();
            assert!(
                missing.is_empty() && extra.is_empty(),
                "{locale} is out of sync with {DEFAULT_LOCALE}: missing {missing:?}, extra {extra:?}"
            );
        }
    }

    #[test]
    fn every_shipped_locale_would_pass_the_upload_check() {
        // The rules an uploaded catalogue has to satisfy are exactly the rules
        // the shipped ones live by, so they are enforced here as well. Without
        // this, a placeholder mistyped while adding a language would only
        // surface as a message that quietly reverts to English at runtime.
        for locale in Localizer::new(DEFAULT_LOCALE).available() {
            let source = locale_source(&locale).expect("catalogue");
            let report = catalog::validate_for(&locale, &source).expect("shipped locale");
            assert!(report.is_acceptable(), "{locale}: {:?}", report.problems);
        }
    }

    #[test]
    fn no_shipped_locale_is_left_in_english() {
        // A catalogue copied but never translated would pass every structural
        // check while leaving the window in English, so coverage is asserted
        // separately. `en-US` is the original and is skipped.
        for locale in Localizer::new(DEFAULT_LOCALE).available() {
            if locale == DEFAULT_LOCALE {
                continue;
            }
            let source = locale_source(&locale).expect("catalogue");
            let report = catalog::validate_for(&locale, &source).expect("shipped locale");
            let share = report.untranslated * 100 / report.accepted.max(1);
            assert!(
                share < 25,
                "{locale} is {share}% identical to {DEFAULT_LOCALE} \
                 ({} of {} entries)",
                report.untranslated,
                report.accepted
            );
        }
    }
}
