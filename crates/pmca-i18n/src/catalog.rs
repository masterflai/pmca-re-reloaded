//! Download, edit and upload of the translation catalogues.
//!
//! The catalogues shipped with the application are compiled into the binary,
//! so a translation the user wants to correct cannot simply be written back
//! over the original. Instead an edited catalogue is stored per user (see
//! [`user_catalog_dir`]) and *layered over* the built-in one: an entry that is
//! present wins, everything else keeps the shipped wording. That layering is
//! what makes a partial file safe — a catalogue containing a single corrected
//! line is a perfectly valid upload, and a key the user never touched can never
//! go missing.
//!
//! An upload is checked before it is stored ([`validate`]). The checks exist
//! because the two consumers of a catalogue are stricter than Fluent itself:
//!
//!  * The GUI parses catalogues **line by line**, so an indented continuation
//!    line — legal Fluent — would silently truncate the message there.
//!  * A placeholder such as `{ $path }` is substituted at runtime. Renaming it
//!    does not fail loudly; the message simply stops formatting and falls back
//!    to English, which looks like the translation was ignored.
//!
//! Both are therefore rejected at upload time, where the file can still be
//! pointed at, rather than at runtime where the cause is invisible.

use std::collections::{BTreeMap, BTreeSet};
use std::io;
use std::path::PathBuf;

use crate::{DEFAULT_LOCALE, locale_source};

/// Name of the directory holding this application's per-user data.
const APP_DIR: &str = "PMCA-RE-RELOADED";

/// What is wrong with one line of an uploaded catalogue.
///
/// Every variant is blocking: none of them can be repaired here without
/// guessing what the translator meant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProblemKind {
    /// The line is neither blank, nor a comment, nor `id = value`.
    Syntax,
    /// An indented continuation line, which the GUI's parser would truncate.
    Continuation,
    /// The same message id appears twice in the file.
    Duplicate,
    /// The message id does not exist in the application.
    UnknownKey,
    /// The translation is empty, which would show as a blank label.
    Empty,
    /// The `{ $name }` placeholders differ from the English original.
    Placeholder,
}

/// A single blocking finding, addressed by the line it was found on.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogProblem {
    /// 1-based line number in the uploaded file.
    pub line: usize,
    /// The message id, or the raw line when it could not be read as one.
    pub key: String,
    /// Which check failed.
    pub kind: ProblemKind,
    /// The offending detail, ready to be shown after the translated wording.
    pub detail: String,
}

/// The verdict on an uploaded catalogue.
///
/// `problems` being empty is the sole condition for accepting the file; the
/// three counters are informational and describe an accepted file's coverage.
#[derive(Debug, Clone, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogReport {
    /// Blocking findings. A non-empty list means nothing was stored.
    pub problems: Vec<CatalogProblem>,
    /// Number of entries that would be applied.
    pub accepted: usize,
    /// Entries absent from the file, which keep their built-in wording.
    pub missing: usize,
    /// Entries that are still character-for-character the English original.
    pub untranslated: usize,
}

impl CatalogReport {
    /// Whether the catalogue may be stored.
    pub fn is_acceptable(&self) -> bool {
        self.problems.is_empty()
    }
}

/// Bring an uploaded file into the shape the rest of the code expects.
///
/// Text editors routinely add a byte-order mark and carriage returns, neither
/// of which is a mistake on the translator's part. A BOM would otherwise become
/// part of the very first message id and make it unknown; a stray `\r` would end
/// up inside the translated text and print as a control character.
pub fn normalize(source: &str) -> String {
    let mut text = source.strip_prefix('\u{feff}').unwrap_or(source).replace("\r\n", "\n");
    text = text.replace('\r', "\n");
    if !text.ends_with('\n') {
        text.push('\n');
    }
    text
}

/// Read `id = value` pairs, ignoring blank lines and comments.
///
/// Used for catalogues that are already known to be well-formed — the shipped
/// ones and files this module wrote itself. Anything unreadable is skipped
/// rather than reported, because there is no one to report it to at this point.
fn entries(source: &str) -> BTreeMap<&str, &str> {
    source
        .lines()
        .filter(|line| !line.starts_with([' ', '\t', '#']))
        .filter_map(|line| line.split_once('='))
        .map(|(id, value)| (id.trim(), value.trim()))
        .filter(|(id, _)| !id.is_empty())
        .collect()
}

/// Collect the `{ … }` placeholders of a message, normalized for comparison.
///
/// Whitespace inside the braces is insignificant to Fluent, so `{$path}` and
/// `{ $path }` are the same placeholder and a translator must not be stopped
/// over the difference. Order and repetition are not compared either: a
/// language may legitimately need the value in a different position, or twice.
fn placeholders(value: &str) -> BTreeSet<String> {
    let mut found = BTreeSet::new();
    let mut rest = value;
    while let Some(open) = rest.find('{') {
        rest = &rest[open + 1..];
        let Some(close) = rest.find('}') else { break };
        found.insert(rest[..close].split_whitespace().collect::<String>());
        rest = &rest[close + 1..];
    }
    found
}

/// Check an uploaded catalogue against `base`, the English original.
///
/// `base` is the authority for both the set of legal message ids and the
/// placeholders each message must carry. Nothing is written here; the caller
/// decides what to do with the verdict.
pub fn validate(base: &str, upload: &str) -> CatalogReport {
    let expected = entries(base);
    let mut report = CatalogReport::default();
    let mut seen: BTreeSet<&str> = BTreeSet::new();

    for (index, raw) in upload.lines().enumerate() {
        let line = index + 1;
        let text = raw.trim_end();
        if text.is_empty() || text.starts_with('#') {
            continue;
        }
        if text.starts_with([' ', '\t']) {
            report.problems.push(CatalogProblem {
                line,
                key: text.trim().to_string(),
                kind: ProblemKind::Continuation,
                detail: text.trim().to_string(),
            });
            continue;
        }
        let Some((id, value)) = text.split_once('=') else {
            report.problems.push(CatalogProblem {
                line,
                key: text.to_string(),
                kind: ProblemKind::Syntax,
                detail: text.to_string(),
            });
            continue;
        };
        let (id, value) = (id.trim(), value.trim());
        if id.is_empty() {
            report.problems.push(CatalogProblem {
                line,
                key: text.to_string(),
                kind: ProblemKind::Syntax,
                detail: text.to_string(),
            });
            continue;
        }
        if !seen.insert(id) {
            report.problems.push(CatalogProblem {
                line,
                key: id.to_string(),
                kind: ProblemKind::Duplicate,
                detail: id.to_string(),
            });
            continue;
        }
        let Some(original) = expected.get(id) else {
            report.problems.push(CatalogProblem {
                line,
                key: id.to_string(),
                kind: ProblemKind::UnknownKey,
                detail: id.to_string(),
            });
            continue;
        };
        if value.is_empty() {
            report.problems.push(CatalogProblem {
                line,
                key: id.to_string(),
                kind: ProblemKind::Empty,
                detail: id.to_string(),
            });
            continue;
        }
        let wanted = placeholders(original);
        if placeholders(value) != wanted {
            let expected_list: Vec<_> = wanted.iter().map(|p| format!("{{ {p} }}")).collect();
            report.problems.push(CatalogProblem {
                line,
                key: id.to_string(),
                kind: ProblemKind::Placeholder,
                detail: if expected_list.is_empty() {
                    "-".to_string()
                } else {
                    expected_list.join(" ")
                },
            });
            continue;
        }
        report.accepted += 1;
        if value == *original {
            report.untranslated += 1;
        }
    }

    report.missing = expected.keys().filter(|id| !seen.contains(*id)).count();
    report
}

/// Validate an upload for `locale` against the application's own catalogue.
///
/// Returns `None` for a locale the application does not ship: only the shipped
/// languages can be edited, and accepting a file for anything else would create
/// a catalogue nothing ever reads.
pub fn validate_for(locale: &str, upload: &str) -> Option<CatalogReport> {
    locale_source(locale)?;
    let base = locale_source(DEFAULT_LOCALE)?;
    Some(validate(&base, upload))
}

/// Build the editable, bilingual catalogue for `locale`.
///
/// The English original is written as a comment above every entry. A translator
/// working on, say, Croatian otherwise sees only Croatian and has no way to tell
/// what a message was supposed to say — and the reviewer of a returned file has
/// the same problem. The comment costs nothing: it is ordinary Fluent, is
/// ignored on upload, and the result is still a valid catalogue file.
///
/// The structure of the English file (its blank lines and section comments) is
/// carried over so the entries stay grouped by topic instead of arriving as one
/// undifferentiated list. Values come from the *effective* catalogue, so an
/// export made after an upload contains the user's own wording and editing can
/// continue where it left off.
pub fn export(locale: &str) -> Option<String> {
    // Normalized because the block of instructions at the top is found by
    // looking for a blank line, and the catalogues are checked out with the
    // platform's own line endings.
    let base = normalize(&locale_source(DEFAULT_LOCALE)?);
    let translated = entries_owned(&effective_source(locale)?);

    let mut out = String::new();
    out.push_str("# Translation file for PMCA-RE RELOADED – Camera Toolkit.\n");
    out.push_str(&format!("# Language: {locale}\n"));
    out.push_str("#\n");
    out.push_str("# Edit only the text to the right of the \"=\" sign. The comment above each\n");
    out.push_str("# entry shows the English original for reference.\n");
    out.push_str("#\n");
    out.push_str("# Keep placeholders such as { $path } exactly as they are: they are replaced\n");
    out.push_str("# with a value at runtime. Every entry must stay on a single line, and lines\n");
    out.push_str("# beginning with \"#\" are comments. Entries you delete keep their built-in\n");
    out.push_str("# wording, so a file containing a single corrected line is a valid upload.\n");

    // The English file opens with its own instructions, which do not belong in
    // a translator's copy. Skipping up to the first blank line drops exactly
    // that block and keeps every section comment further down.
    let body = base.split_once("\n\n").map(|(_, rest)| rest).unwrap_or(&base);

    for line in body.lines() {
        if line.trim().is_empty() {
            out.push('\n');
            continue;
        }
        if line.starts_with('#') {
            out.push_str(line);
            out.push('\n');
            continue;
        }
        let Some((id, original)) = line.split_once('=') else {
            continue;
        };
        let (id, original) = (id.trim(), original.trim());
        // An entry with no translation yet is exported as the English text, so
        // the file stays complete and shows what is left to do.
        let value = translated.get(id).map(String::as_str).unwrap_or(original);
        out.push_str(&format!("# en-US: {original}\n{id} = {value}\n"));
    }
    Some(out)
}

/// Owned variant of [`entries`], for sources that are built at runtime.
fn entries_owned(source: &str) -> BTreeMap<String, String> {
    entries(source).into_iter().map(|(id, value)| (id.to_string(), value.to_string())).collect()
}

/// Lay `user` over `base`, keeping every entry `user` does not mention.
///
/// Unknown ids and unreadable lines in `user` are dropped rather than appended:
/// the stored file has passed [`validate`], but it lives in a directory the user
/// can edit by hand afterwards, and a catalogue must not be able to grow keys
/// the application never asks for.
fn overlay(base: &str, user: &str) -> String {
    let replacements = entries(user);
    let mut out = String::with_capacity(base.len());
    for line in base.lines() {
        match line.split_once('=') {
            Some((id, _)) if !line.starts_with([' ', '\t', '#']) => {
                match replacements.get(id.trim()) {
                    Some(value) => out.push_str(&format!("{} = {}\n", id.trim(), value)),
                    None => {
                        out.push_str(line);
                        out.push('\n');
                    }
                }
            }
            _ => {
                out.push_str(line);
                out.push('\n');
            }
        }
    }
    out
}

/// The catalogue actually in force for `locale`: built-in, plus user edits,
/// plus the messages installed plugins add.
///
/// Plugin lines are appended only for ids not defined yet, so the window's
/// line-based parser — where a later line would win — resolves exactly as the
/// backend's bundles do, where the first definition wins.
pub fn effective_source(locale: &str) -> Option<String> {
    let mut plugins = crate::plugin_sources(locale).peekable();
    let base = match locale_source(locale) {
        Some(base) => match user_catalog_source(locale) {
            Some(user) => overlay(&base, &user),
            None => base,
        },
        None if plugins.peek().is_some() => String::new(),
        None => return None,
    };
    let mut seen: std::collections::HashSet<String> =
        base.lines().filter_map(message_id).map(str::to_string).collect();
    let mut out = base;
    for source in plugins {
        for line in source.lines() {
            if let Some(id) = message_id(line)
                && seen.insert(id.to_string())
            {
                if !out.is_empty() && !out.ends_with('\n') {
                    out.push('\n');
                }
                out.push_str(line);
                out.push('\n');
            }
        }
    }
    Some(out)
}

/// The id of a single-line `key = value` message, or `None` for anything else.
fn message_id(line: &str) -> Option<&str> {
    if line.starts_with([' ', '\t', '#']) {
        return None;
    }
    let (id, _) = line.split_once('=')?;
    let id = id.trim();
    (!id.is_empty()).then_some(id)
}

/// Directory holding the user's edited catalogues.
///
/// Returns `None` when the platform's home directory cannot be determined, in
/// which case editing is simply unavailable — inventing a location inside the
/// installation directory would put user data somewhere an update wipes, and
/// somewhere an unprivileged user usually cannot write.
pub fn user_catalog_dir() -> Option<PathBuf> {
    Some(user_data_dir()?.join("locales"))
}

/// The application's per-user data directory.
pub fn user_data_dir() -> Option<PathBuf> {
    let base = if cfg!(windows) {
        PathBuf::from(std::env::var_os("APPDATA")?)
    } else if cfg!(target_os = "macos") {
        PathBuf::from(std::env::var_os("HOME")?).join("Library/Application Support")
    } else {
        match std::env::var_os("XDG_CONFIG_HOME") {
            Some(dir) => PathBuf::from(dir),
            None => PathBuf::from(std::env::var_os("HOME")?).join(".config"),
        }
    };
    Some(base.join(APP_DIR))
}

/// Where the edited catalogue for `locale` is stored.
///
/// The locale is checked against the shipped catalogues before it becomes part
/// of a path. It arrives from the user interface, and an unchecked value would
/// let a name such as `../../autostart` decide where this module writes.
pub fn user_catalog_path(locale: &str) -> Option<PathBuf> {
    locale_source(locale)?;
    Some(user_catalog_dir()?.join(format!("{locale}.ftl")))
}

/// The user's edited catalogue for `locale`, if one has been stored.
///
/// A file that cannot be read — removed, or not valid UTF-8 after a hand edit —
/// is treated as absent. The built-in catalogue is always a usable answer, and
/// failing to start the window over a broken optional file would be worse than
/// ignoring it.
pub fn user_catalog_source(locale: &str) -> Option<String> {
    std::fs::read_to_string(user_catalog_path(locale)?).ok()
}

/// Store `source` as the edited catalogue for `locale` and report where.
///
/// The caller is expected to have validated `source` first; this function only
/// normalizes line endings so the stored file is readable on every platform.
pub fn store_user_catalog(locale: &str, source: &str) -> io::Result<PathBuf> {
    let path = user_catalog_path(locale)
        .ok_or_else(|| io::Error::other(format!("no catalogue location for {locale}")))?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&path, normalize(source))?;
    Ok(path)
}

/// Discard the edited catalogue for `locale`, restoring the built-in wording.
///
/// Reports whether a file was actually removed, so the window can tell "reset
/// done" from "there was nothing to reset".
pub fn remove_user_catalog(locale: &str) -> io::Result<bool> {
    let Some(path) = user_catalog_path(locale) else {
        return Ok(false);
    };
    match std::fs::remove_file(&path) {
        Ok(()) => Ok(true),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(false),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASE: &str = "\
# header
# more header

# section
greeting = Hello
saved-to = Written to { $path }
";

    #[test]
    fn a_correct_translation_is_accepted() {
        let report = validate(BASE, "greeting = Hallo\nsaved-to = Gespeichert unter { $path }\n");
        assert!(report.is_acceptable(), "{:?}", report.problems);
        assert_eq!(report.accepted, 2);
        assert_eq!(report.missing, 0);
        assert_eq!(report.untranslated, 0);
    }

    #[test]
    fn a_partial_translation_is_accepted_and_counted() {
        let report = validate(BASE, "greeting = Hallo\n");
        assert!(report.is_acceptable());
        assert_eq!(report.accepted, 1);
        assert_eq!(report.missing, 1);
    }

    #[test]
    fn text_left_in_english_is_counted_but_not_refused() {
        let report = validate(BASE, "greeting = Hello\n");
        assert!(report.is_acceptable());
        assert_eq!(report.untranslated, 1);
    }

    #[test]
    fn a_renamed_placeholder_is_refused() {
        // The message would stop formatting at runtime and silently fall back
        // to English, so this has to be caught while the file can still be
        // pointed at.
        let report = validate(BASE, "saved-to = Gespeichert unter { $pfad }\n");
        assert_eq!(report.problems.len(), 1);
        assert_eq!(report.problems[0].kind, ProblemKind::Placeholder);
        assert_eq!(report.problems[0].detail, "{ $path }");
    }

    #[test]
    fn a_dropped_placeholder_is_refused() {
        let report = validate(BASE, "saved-to = Gespeichert\n");
        assert_eq!(report.problems[0].kind, ProblemKind::Placeholder);
    }

    #[test]
    fn spacing_inside_a_placeholder_does_not_matter() {
        let report = validate(BASE, "saved-to = Nach {$path} geschrieben\n");
        assert!(report.is_acceptable(), "{:?}", report.problems);
    }

    #[test]
    fn a_continuation_line_is_refused() {
        // Legal Fluent, but the window's line-based parser would drop the
        // second half of the message without saying so.
        let report = validate(BASE, "greeting = Hallo\n    und willkommen\n");
        assert_eq!(report.problems.len(), 1);
        assert_eq!(report.problems[0].kind, ProblemKind::Continuation);
        assert_eq!(report.problems[0].line, 2);
    }

    #[test]
    fn an_unknown_key_is_refused() {
        let report = validate(BASE, "greetnig = Hallo\n");
        assert_eq!(report.problems[0].kind, ProblemKind::UnknownKey);
        assert_eq!(report.problems[0].key, "greetnig");
    }

    #[test]
    fn a_duplicate_key_is_refused() {
        let report = validate(BASE, "greeting = Hallo\ngreeting = Servus\n");
        assert_eq!(report.problems[0].kind, ProblemKind::Duplicate);
        assert_eq!(report.problems[0].line, 2);
    }

    #[test]
    fn an_empty_translation_is_refused() {
        // `tr()` prefers a present-but-empty entry over the fallback, so this
        // would show as a blank label rather than as English text.
        let report = validate(BASE, "greeting =\n");
        assert_eq!(report.problems[0].kind, ProblemKind::Empty);
    }

    #[test]
    fn a_line_without_an_equals_sign_is_refused() {
        let report = validate(BASE, "greeting Hallo\n");
        assert_eq!(report.problems[0].kind, ProblemKind::Syntax);
    }

    #[test]
    fn comments_and_blank_lines_are_ignored() {
        let report = validate(BASE, "# a comment\n\ngreeting = Hallo\n");
        assert!(report.is_acceptable());
        assert_eq!(report.accepted, 1);
    }

    #[test]
    fn normalization_removes_the_byte_order_mark_and_carriage_returns() {
        let normalized = normalize("\u{feff}greeting = Hallo\r\n");
        assert_eq!(normalized, "greeting = Hallo\n");
        // Without this the mark would become part of the first message id.
        assert!(validate(BASE, &normalized).is_acceptable());
    }

    #[test]
    fn an_overlay_replaces_only_the_entries_it_carries() {
        let merged = overlay(BASE, "greeting = Hallo\nunknown = ignoriert\n");
        assert!(merged.contains("greeting = Hallo"));
        assert!(merged.contains("saved-to = Written to { $path }"));
        assert!(!merged.contains("unknown"));
        // Section comments must survive, or a later export loses its structure.
        assert!(merged.contains("# section"));
    }

    #[test]
    fn the_export_pairs_every_entry_with_its_english_original() {
        let exported = export("de-DE").expect("shipped locale");
        assert!(exported.contains("# en-US: Camera info\n"));
        assert!(exported.contains("\nnav-camera-info = "));
        // The English file's own instructions are not a translator's business.
        assert!(!exported.contains("Add a new language by copying"));
    }

    #[test]
    fn an_export_is_a_valid_upload() {
        // The round-trip is the whole point of the feature: whatever is handed
        // out has to be acceptable again when it comes back untouched.
        let exported = export("de-DE").expect("shipped locale");
        let report = validate_for("de-DE", &exported).expect("shipped locale");
        assert!(report.is_acceptable(), "{:?}", report.problems);
        assert_eq!(report.missing, 0);
    }

    #[test]
    fn unshipped_languages_cannot_be_edited() {
        assert!(export("xx-XX").is_none());
        assert!(validate_for("xx-XX", "greeting = x").is_none());
    }

    #[test]
    fn a_locale_name_cannot_escape_the_catalogue_directory() {
        // The name arrives from the window; without the check against the
        // shipped catalogues it would decide where this module writes.
        assert!(user_catalog_path("../../../evil").is_none());
        assert!(user_catalog_path("..").is_none());
        assert!(user_catalog_path("de-DE/../../evil").is_none());
    }
}
