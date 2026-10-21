//! Per-region factory language tables.
//!
//! Port of `BackupInterface.getDefaultLanguages`. Each entry lists, for one
//! sales region, which of the 35 menu languages the camera ships with. The
//! "unlock all languages" tweak toggles between a region's table and the set
//! of languages that region's firmware can actually display, so restoring the
//! original state stays possible.
//!
//! The tables are stored as 35-character bit strings purely for
//! reviewability — a wall of `0`/`1` diffs far more clearly than nested
//! integer arrays.

/// Number of language slots in the backup property.
pub const LANGUAGE_COUNT: usize = 35;

/// Backup value meaning "this language is selectable".
pub const LANG_ENABLED: u8 = 1;

/// Backup value meaning "this language is hidden".
pub const LANG_DISABLED: u8 = 2;

/// Region key selecting every language.
pub const ALL_LANGUAGES: &str = "ALLLANG";

/// Region key of the Japanese domestic market.
pub const REGION_JAPAN: &str = "J1";

/// Language slots a Japanese camera cannot display.
///
/// Enabling these two makes the camera reboot when the language menu is
/// scrolled past them: the firmware has no resources for them, so drawing the
/// entry faults. Nothing is damaged by it, but the menu becomes unusable —
/// which on a camera whose menu is in a language the owner cannot read is
/// close to the same thing.
///
/// They are simplified and traditional Chinese: the reference discussion
/// suspected it, and [`LANGUAGE_CODES`], measured on an RX1R II, confirms it.
/// The reboot itself has not been seen here on a Japanese body.
const JAPAN_UNSUPPORTED: [usize; 2] = [7, 8];

/// The language in each slot as a BCP 47 code; `None` where not yet established.
///
/// Measured on a DSC-RX1RM2 by writing six bit masks and reading its menu. Its
/// firmware shows 25 languages; the open slots hold ones it cannot display.
/// Cross-checked against the CEC, CE3 and J1 tables.
pub const LANGUAGE_CODES: [Option<&str>; LANGUAGE_COUNT] = [
    Some("en"),
    Some("ja"),
    Some("fr"),
    Some("de"),
    Some("es"),
    Some("it"),
    Some("pt"),
    Some("zh-Hans"),
    Some("zh-Hant"),
    Some("ar"),
    Some("ko"),
    Some("nl"),
    Some("ru"),
    Some("th"),
    Some("fa"),
    Some("hu"),
    Some("cs"),
    Some("pl"),
    Some("sv"),
    Some("no"),
    Some("tr"),
    Some("el"),
    None,
    Some("ms"),
    None,
    Some("fi"),
    None,
    Some("da"),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
];

/// The enabled languages of a table: the known ones by code, and how many
/// enabled slots hold a language not yet established.
pub fn enabled_languages(langs: &[u8]) -> (Vec<&'static str>, usize) {
    let mut known = Vec::new();
    let mut unknown = 0;
    for (slot, _) in langs.iter().enumerate().filter(|(_, l)| **l == LANG_ENABLED) {
        match LANGUAGE_CODES.get(slot).copied().flatten() {
            Some(code) => known.push(code),
            None => unknown += 1,
        }
    }
    (known, unknown)
}

const TABLES: &[(&str, &str)] = &[
    ("ALLLANG", "11111111111111111111111111111111111"),
    ("AP2", "10101011111001100000100110000000000"),
    ("AU2", "10101011111001100000100110000101000"),
    ("CA2", "10101100100000000000000000000000000"),
    ("CE", "10111110000110011111110001010000000"),
    ("CE3", "10111110000100011111111001111000000"),
    ("CE7", "10101011111001100000100110000101000"),
    ("CEC", "10111110000110011111110001010010000"),
    ("CEH", "10111110000100011111111001111000000"),
    ("CN1", "10101011111001100000100110000101000"),
    ("CN2", "10101011111001100000000100000001000"),
    ("E32", "10101011111001100000100110000101000"),
    ("E33", "10101011111001100000100110000101000"),
    ("E37", "10101011111001100000100110000101000"),
    ("E38", "10101011111001100000000100000001000"),
    ("EA8", "10101011111001100000100110000101000"),
    ("HK1", "10101011111001100000100110000101000"),
    ("IN5", "10101011111001100000100110000000000"),
    ("J1", "01000000000000000000000000000000000"),
    ("JE3", "10101011111001100000100110000000000"),
    ("KR2", "10101011111001100000100110000101000"),
    ("RU2", "10111110000110011111110001010000000"),
    ("RU3", "10111110000110011111111001110010000"),
    ("TW6", "10101011111001100000100110000101000"),
    ("U2", "10101100000000000000000000000000000"),
    ("UC2", "10101101100000000000000000000000000"),
];

/// The factory language selection for a region, as backup property bytes.
///
/// `region` is the part of the backup header's region string after the first
/// underscore (e.g. `"CE3"` for `"EU_CE3"`), or [`ALL_LANGUAGES`].
pub fn default_languages(region: &str) -> Option<Vec<u8>> {
    let bits = TABLES.iter().find(|(name, _)| *name == region)?.1;
    Some(bits.bytes().map(|b| if b == b'1' { LANG_ENABLED } else { LANG_DISABLED }).collect())
}

/// Extract the region key from a backup header region string.
///
/// Sony writes e.g. `"EU_CE3"`; only the part after the first underscore
/// selects the language table.
pub fn region_key(region: &str) -> Option<&str> {
    region.split_once('_').map(|(_, key)| key)
}

/// The languages the "unlock all languages" tweak turns on for a region.
///
/// Every language, save where the camera's firmware cannot show one: a
/// Japanese camera gets everything except `JAPAN_UNSUPPORTED`. The point of
/// the tweak is to make the menu usable to someone who cannot read the factory
/// language, and a language that reboots the camera when it is scrolled past
/// does the opposite.
///
/// `accept_caution` overrides that for the one user who needs those two
/// languages more than they need a menu that never reboots. It is theirs to
/// decide — the reboot is a nuisance, not damage, and the tweak can be undone
/// — but it has to be decided, which is why there is no default for it here.
///
/// `region` is a key as returned by [`region_key`].
pub fn unlockable_languages(region: &str, accept_caution: bool) -> Vec<u8> {
    let mut langs = vec![LANG_ENABLED; LANGUAGE_COUNT];
    if region == REGION_JAPAN && !accept_caution {
        for slot in JAPAN_UNSUPPORTED {
            langs[slot] = LANG_DISABLED;
        }
    }
    langs
}

/// Whether unlocking every language would enable one this region cannot show.
///
/// The caller needs this to decide whether to warn, which it has to do before
/// it knows what the user will choose — comparing the two possible values
/// after the fact would answer the question too late.
pub fn has_unsupported_languages(region: &str) -> bool {
    region == REGION_JAPAN
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_table_has_the_expected_width() {
        for (name, bits) in TABLES {
            assert_eq!(bits.len(), LANGUAGE_COUNT, "{name}");
            assert!(bits.bytes().all(|b| b == b'0' || b == b'1'), "{name}");
        }
    }

    #[test]
    fn all_languages_enables_everything() {
        let langs = default_languages(ALL_LANGUAGES).unwrap();
        assert_eq!(langs.len(), LANGUAGE_COUNT);
        assert!(langs.iter().all(|&l| l == LANG_ENABLED));
    }

    #[test]
    fn known_region_matches_the_reference_table() {
        // AP2 from pmca/platform/backup.py.
        let expected: Vec<u8> = [
            1, 0, 1, 0, 1, 0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
        ]
        .iter()
        .map(|&l| if l == 1 { LANG_ENABLED } else { LANG_DISABLED })
        .collect();
        assert_eq!(default_languages("AP2").unwrap(), expected);
    }

    #[test]
    fn region_key_strips_the_market_prefix() {
        assert_eq!(region_key("EU_CE3"), Some("CE3"));
        assert_eq!(region_key("CE3"), None);
    }

    #[test]
    fn unknown_region_is_none() {
        assert!(default_languages("ZZ9").is_none());
    }

    #[test]
    fn most_regions_can_unlock_every_language() {
        // Tied to the reference table rather than restating it, so the two
        // definitions of "all of them" cannot drift apart.
        assert_eq!(unlockable_languages("CE3", false), default_languages(ALL_LANGUAGES).unwrap());
        assert!(!has_unsupported_languages("CE3"));
    }

    #[test]
    fn japan_keeps_the_two_unsupported_slots_off() {
        let langs = unlockable_languages(REGION_JAPAN, false);
        assert_eq!(langs.len(), LANGUAGE_COUNT);
        for (i, &l) in langs.iter().enumerate() {
            let expected =
                if JAPAN_UNSUPPORTED.contains(&i) { LANG_DISABLED } else { LANG_ENABLED };
            assert_eq!(l, expected, "slot {i}");
        }
        assert!(has_unsupported_languages(REGION_JAPAN));
    }

    #[test]
    fn japan_can_unlock_everything_when_the_caution_is_accepted() {
        assert_eq!(
            unlockable_languages(REGION_JAPAN, true),
            default_languages(ALL_LANGUAGES).unwrap()
        );
    }

    #[test]
    fn accepting_the_caution_changes_nothing_elsewhere() {
        // Only Japan has slots to leave out, so the flag must be inert for
        // every other region rather than quietly meaning something there.
        for (name, _) in TABLES {
            if *name == REGION_JAPAN {
                continue;
            }
            assert_eq!(
                unlockable_languages(name, false),
                unlockable_languages(name, true),
                "{name}"
            );
        }
    }

    #[test]
    fn japan_still_has_its_own_factory_table() {
        // The tweak's off value is unaffected: undoing it must still put the
        // camera back exactly where Sony left it.
        assert_eq!(default_languages(REGION_JAPAN).unwrap()[1], LANG_ENABLED);
        assert_ne!(
            default_languages(REGION_JAPAN).unwrap(),
            unlockable_languages(REGION_JAPAN, false)
        );
    }

    #[test]
    fn the_measured_codes_are_distinct() {
        let known: Vec<&str> = LANGUAGE_CODES.iter().flatten().copied().collect();
        assert_eq!(known.len(), 25);
        let mut sorted = known.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), known.len());
    }

    #[test]
    fn the_tables_name_what_the_rx1r_ii_showed() {
        // Photographed on the camera: CEC lists 17 languages, CE3 the same without Russian.
        let cec = [
            "en", "fr", "de", "es", "it", "pt", "nl", "ru", "hu", "cs", "pl", "sv", "no", "tr",
            "el", "fi", "da",
        ];
        assert_eq!(enabled_languages(&default_languages("CEC").unwrap()), (cec.to_vec(), 1));
        let ce3: Vec<&str> = cec.into_iter().filter(|c| *c != "ru").collect();
        assert_eq!(enabled_languages(&default_languages("CE3").unwrap()), (ce3, 3));
        assert_eq!(enabled_languages(&default_languages(REGION_JAPAN).unwrap()), (vec!["ja"], 0));
    }

    #[test]
    fn the_slots_japan_cannot_show_are_chinese() {
        assert_eq!(
            JAPAN_UNSUPPORTED.map(|s| LANGUAGE_CODES[s]),
            [Some("zh-Hans"), Some("zh-Hant")]
        );
    }
}
