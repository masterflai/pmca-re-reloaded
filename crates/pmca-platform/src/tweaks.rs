//! The camera tweaks exposed to the user.
//!
//! Port of `pmca/platform/tweaks.py`. A tweak is a named pair of "off" and
//! "on" values for one backup setting, plus a human-readable rendering of the
//! current value. A tweak is *available* only when the camera's current value
//! is one the toolkit understands — that guard is what keeps an unknown
//! firmware from being written with values meant for a different model.

use pmca_core::util::{dump16le, parse16le};

use crate::languages::{
    LANG_DISABLED, LANG_ENABLED, LANGUAGE_COUNT, default_languages, has_unsupported_languages,
    region_key, unlockable_languages,
};
use crate::{Error, Result, Settings};

/// How a tweak derives its values and renders its state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    /// A single byte: `0` off, `1` on.
    Boolean,
    /// The video system itself: `0` PAL, `1` NTSC.
    ///
    /// Stored like a boolean but meant as a choice between two equal
    /// alternatives, so it renders as the system's name rather than as
    /// on/off. Separate from `palNtscSelector`, which only reveals the menu
    /// entry the camera's own firmware would use to write this byte.
    PalNtsc,
    /// Recording limit as `hours, minutes, seconds`.
    RecLimit,
    /// 4K recording limit as little-endian seconds.
    RecLimit4k,
    /// The 35-entry menu language table.
    Language,
}

struct Def {
    id: &'static str,
    prop: &'static str,
    kind: Kind,
}

const TWEAKS: &[Def] = &[
    Def { id: "recLimit", prop: "recLimit", kind: Kind::RecLimit },
    Def { id: "recLimit4k", prop: "recLimit4k", kind: Kind::RecLimit4k },
    Def { id: "language", prop: "language", kind: Kind::Language },
    Def { id: "palNtscSelector", prop: "palNtscSelector", kind: Kind::Boolean },
    Def { id: "palNtsc", prop: "palNtsc", kind: Kind::PalNtsc },
    Def { id: "usbAppInstaller", prop: "usbAppInstaller", kind: Kind::Boolean },
];

/// Video recording limit when disabled: 29m50s, Sony's factory value.
const REC_LIMIT_OFF: [u8; 3] = [0, 29, 50];
/// Video recording limit when the tweak is applied: 13h01m00s.
const REC_LIMIT_ON: [u8; 3] = [13, 1, 0];
/// 4K recording limit when disabled, in seconds.
const REC_LIMIT_4K_OFF: u16 = 5 * 60;
/// 4K recording limit when the tweak is applied, in seconds (9h06m07s).
const REC_LIMIT_4K_ON: u16 = 0x7fff;

/// The two video systems, as the camera menu names them.
const PAL: &str = "PAL";
const NTSC: &str = "NTSC";

/// How the user should be asked about a tweak's state.
///
/// Every tweak stores a single byte either way; this says nothing about the
/// storage and everything about what the byte *means*. Most settings are
/// something the camera either does or does not do, and a checkbox states that
/// correctly. A few are a choice between alternatives that are equally normal,
/// where "off" would name nothing the user could recognise.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Presentation {
    /// One thing, done or not done.
    Toggle,
    /// Two named alternatives; neither is the absence of the other.
    Choice {
        /// Name of the state stored as the off value.
        off: &'static str,
        /// Name of the state stored as the on value.
        on: &'static str,
    },
}

/// A consequence of enabling a tweak that the user has to be told about
/// before they can be said to have chosen it.
///
/// Distinct from unavailability: the toolkit knows exactly what this write
/// does and it is reversible. What it cannot know is whether the trade is one
/// this particular user wants, so it asks instead of deciding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Caution {
    /// Some of the languages would be ones the camera cannot display.
    UnsupportedLanguages,
}

/// The state of one tweak on the connected camera.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TweakStatus {
    /// Stable identifier; the UI translates it via the `tweak-<id>` message.
    pub id: String,
    /// Whether the camera currently has the tweaked value.
    pub enabled: bool,
    /// Human-readable rendering of the current value.
    pub value: String,
    /// How the user should be asked to change this tweak.
    pub presentation: Presentation,
    /// Set when enabling this tweak on *this* camera needs a decision first.
    ///
    /// Only ever set where it applies, so a front-end can show the warning
    /// whenever it is present without repeating the region check itself.
    pub caution: Option<Caution>,
}

fn def(id: &str) -> Result<&'static Def> {
    TWEAKS.iter().find(|t| t.id == id).ok_or_else(|| Error::Unknown(id.to_string()))
}

/// A switch defined by a plugin: two exact byte strings for one setting.
///
/// Deliberately the simplest kind there is. The plugin names both values in
/// full, and the switch is offered only while the camera holds one of them —
/// the same guard that keeps the built-in switches from writing into a
/// property that means something else on another model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PluginTweak {
    /// Identifier, which is also the Fluent id of its label.
    pub id: &'static str,
    /// The setting it changes, registered with [`crate::props::register_extra`].
    pub prop: &'static str,
    /// The value meaning "off".
    pub off: Vec<u8>,
    /// The value meaning "on".
    pub on: Vec<u8>,
    /// Model names the switch is meant for; empty means any model.
    pub models: Vec<String>,
}

static PLUGIN_TWEAKS: std::sync::OnceLock<Vec<PluginTweak>> = std::sync::OnceLock::new();

/// Register the switches installed plugins define, once per process.
///
/// Returns `false` if switches were already registered.
pub fn register_plugin_tweaks(tweaks: Vec<PluginTweak>) -> bool {
    PLUGIN_TWEAKS.set(tweaks).is_ok()
}

fn plugin_tweaks() -> &'static [PluginTweak] {
    PLUGIN_TWEAKS.get().map_or(&[][..], Vec::as_slice)
}

fn plugin_tweak(id: &str) -> Option<&'static PluginTweak> {
    plugin_tweaks().iter().find(|t| t.id == id)
}

/// The camera's model name, without the padding it is stored with.
fn model_name(patch: &(impl Settings + ?Sized)) -> Result<String> {
    let raw = patch.read("modelName")?;
    let end = raw.iter().position(|&b| b == 0).unwrap_or(raw.len());
    Ok(String::from_utf8_lossy(&raw[..end]).trim().to_string())
}

/// Whether a plugin switch applies to this camera and its value is one of the
/// two the plugin named.
fn plugin_available(t: &PluginTweak, patch: &(impl Settings + ?Sized)) -> Result<bool> {
    if !t.models.is_empty() {
        let model = model_name(patch)?;
        if !t.models.iter().any(|m| m.eq_ignore_ascii_case(&model)) {
            return Ok(false);
        }
    }
    let data = patch.read(t.prop)?;
    Ok(data == t.off || data == t.on)
}

fn plugin_status(t: &PluginTweak, patch: &(impl Settings + ?Sized)) -> Option<TweakStatus> {
    if !plugin_available(t, patch).unwrap_or(false) {
        return None;
    }
    let enabled = patch.read(t.prop).ok()? == t.on;
    Some(TweakStatus {
        id: t.id.to_string(),
        enabled,
        value: if enabled { "Enabled" } else { "Disabled" }.to_string(),
        presentation: Presentation::Toggle,
        caution: None,
    })
}

fn presentation(d: &Def) -> Presentation {
    match d.kind {
        Kind::PalNtsc => Presentation::Choice { off: PAL, on: NTSC },
        _ => Presentation::Toggle,
    }
}

/// How a tweak should be presented, without needing a camera.
///
/// The front-ends need this before they have read anything — to reject
/// `--enable` on a tweak that has no "on", for instance — so it comes from the
/// static table rather than from a report.
pub fn presentation_of(id: &str) -> Result<Presentation> {
    if plugin_tweak(id).is_some() {
        return Ok(Presentation::Toggle);
    }
    Ok(presentation(def(id)?))
}

/// What the user has to be warned about before enabling this tweak, if
/// anything.
fn caution(d: &Def, patch: &(impl Settings + ?Sized)) -> Option<Caution> {
    match d.kind {
        Kind::Language => has_unsupported_languages(region_key(patch.region())?)
            .then_some(Caution::UnsupportedLanguages),
        _ => None,
    }
}

/// The value that represents "tweak disabled" (the camera's factory state).
fn off_value(d: &Def, patch: &(impl Settings + ?Sized)) -> Result<Vec<u8>> {
    Ok(match d.kind {
        Kind::Boolean | Kind::PalNtsc => vec![0],
        Kind::RecLimit => REC_LIMIT_OFF.to_vec(),
        Kind::RecLimit4k => dump16le(REC_LIMIT_4K_OFF).to_vec(),
        Kind::Language => {
            let key =
                region_key(patch.region()).ok_or_else(|| Error::Unavailable(d.id.to_string()))?;
            default_languages(key).ok_or_else(|| Error::Unavailable(d.id.to_string()))?
        }
    })
}

/// The value that represents "tweak enabled".
///
/// The language table needs the camera's region for this and not just for the
/// off value, because which languages can be unlocked is a property of the
/// firmware: a Japanese camera reboots on two of them. `accept_caution` is the
/// user's answer to that, and is ignored where there is nothing to answer.
fn on_value(d: &Def, patch: &(impl Settings + ?Sized), accept_caution: bool) -> Result<Vec<u8>> {
    Ok(match d.kind {
        Kind::Boolean | Kind::PalNtsc => vec![1],
        Kind::RecLimit => REC_LIMIT_ON.to_vec(),
        Kind::RecLimit4k => dump16le(REC_LIMIT_4K_ON).to_vec(),
        Kind::Language => {
            let key =
                region_key(patch.region()).ok_or_else(|| Error::Unavailable(d.id.to_string()))?;
            unlockable_languages(key, accept_caution)
        }
    })
}

fn format_hms(hours: u32, minutes: u32, seconds: u32) -> String {
    format!("{hours}h {minutes:02}m {seconds:02}s")
}

/// Render the current value the way the reference `strValue` does.
fn describe(d: &Def, data: &[u8], enabled: bool) -> String {
    match d.kind {
        Kind::Boolean => if enabled { "Enabled" } else { "Disabled" }.to_string(),
        // Named rather than on/off: neither system is the absence of the
        // other, so "disabled" would say nothing about what the camera does.
        Kind::PalNtsc => if enabled { NTSC } else { PAL }.to_string(),
        Kind::RecLimit => format_hms(data[0] as u32, data[1] as u32, data[2] as u32),
        Kind::RecLimit4k => {
            let limit = parse16le(data).unwrap_or(0) as u32;
            format_hms(limit / 3600, (limit % 3600) / 60, limit % 60)
        }
        Kind::Language => {
            let on = data.iter().filter(|&&l| l == LANG_ENABLED).count();
            format!("{on} / {} language slots enabled", data.len())
        }
    }
}

/// Whether the camera's current value is one this tweak understands.
fn is_available(d: &Def, patch: &(impl Settings + ?Sized)) -> Result<bool> {
    let data = patch.read(d.prop)?;
    Ok(match d.kind {
        // Boolean tweaks must currently hold exactly 0 or 1; anything else
        // means the property has a different meaning on this model.
        Kind::Boolean | Kind::PalNtsc => {
            data == off_value(d, patch)? || data == on_value(d, patch, false)?
        }
        // The language table must be a plausible table, and the camera's
        // region must be one we have a factory table for (otherwise the tweak
        // could not be undone).
        Kind::Language => {
            data.len() == LANGUAGE_COUNT
                && data.iter().all(|&l| l == LANG_ENABLED || l == LANG_DISABLED)
                && off_value(d, patch).is_ok()
        }
        // The recording limits are free-form durations; any readable value is
        // acceptable, matching the reference's `checkValue = False`. Reaching
        // this point already proves the property exists and has the right size.
        Kind::RecLimit | Kind::RecLimit4k => !data.is_empty(),
    })
}

/// Whether the camera's current value counts as this tweak being on.
///
/// Where a tweak has a cautioned variant, *both* of its on values count. They
/// are two answers to a question the user was asked, not a right one and a
/// wrong one, and reporting the accepted-caution value as "off" would tell
/// someone who deliberately chose it that their choice did not take. Which of
/// the two is in force stays visible in the rendered value — "33 / 35" reads
/// differently from "35 / 35".
fn is_enabled(d: &Def, patch: &(impl Settings + ?Sized), data: &[u8]) -> Result<bool> {
    Ok(data == on_value(d, patch, false)? || data == on_value(d, patch, true)?)
}

/// Report every tweak this camera supports, with its current state.
///
/// Tweaks whose backup property is missing or holds an unrecognised value are
/// silently skipped, exactly as `TweakInterface.getTweaks` does. Switches from
/// plugins follow the built-in ones.
pub fn list(patch: &(impl Settings + ?Sized)) -> Vec<TweakStatus> {
    TWEAKS
        .iter()
        .filter_map(|d| {
            if !is_available(d, patch).unwrap_or(false) {
                return None;
            }
            let data = patch.read(d.prop).ok()?;
            let enabled = is_enabled(d, patch, &data).ok()?;
            Some(TweakStatus {
                id: d.id.to_string(),
                enabled,
                value: describe(d, &data, enabled),
                presentation: presentation(d),
                caution: caution(d, patch),
            })
        })
        .chain(plugin_tweaks().iter().filter_map(|t| plugin_status(t, patch)))
        .collect()
}

/// Stage a tweak's on/off value.
///
/// Fails if the tweak is unknown, or if the camera's current value is not one
/// the toolkit recognises — writing in that case could brick the camera.
///
/// `accept_caution` is the user's answer to whatever `caution` reported for
/// this tweak, and is ignored where there was nothing to report. Defaulting it
/// to `false` at every call site would be the safe reading, but it is a
/// parameter rather than a default so that a caller who has not thought about
/// the question cannot silently answer it.
pub fn set_enabled(
    patch: &mut (impl Settings + ?Sized),
    id: &str,
    enabled: bool,
    accept_caution: bool,
) -> Result<()> {
    if let Some(t) = plugin_tweak(id) {
        if !plugin_available(t, patch)? {
            return Err(Error::Unavailable(id.to_string()));
        }
        return patch.write(t.prop, if enabled { &t.on } else { &t.off });
    }
    let d = def(id)?;
    if !is_available(d, patch)? {
        return Err(Error::Unavailable(id.to_string()));
    }
    let value = if enabled { on_value(d, patch, accept_caution)? } else { off_value(d, patch)? };
    patch.write(d.prop, &value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BackupPatch;
    use crate::tests::{fix_checksum, sample_image};

    fn patch() -> BackupPatch {
        BackupPatch::new(sample_image()).unwrap()
    }

    /// The sample camera, sold in a different region.
    ///
    /// The region is a fixed-width, NUL-padded field in the header, so it is
    /// overwritten in place rather than rebuilt — and the checksum covers it,
    /// which is why it has to be recomputed afterwards.
    fn patch_for_region(region: &str) -> BackupPatch {
        const REGION: usize = 0xc0;
        const REGION_LEN: usize = 0x20;
        let mut image = sample_image();
        image[REGION..REGION + REGION_LEN].fill(0);
        image[REGION..REGION + region.len()].copy_from_slice(region.as_bytes());
        fix_checksum(&mut image);
        BackupPatch::new(image).unwrap()
    }

    #[test]
    fn lists_the_tweaks_the_sample_camera_supports() {
        let ids: Vec<String> = list(&patch()).into_iter().map(|t| t.id).collect();
        // The sample image has no `language` property, so it must be skipped.
        assert_eq!(
            ids,
            vec!["recLimit", "recLimit4k", "palNtscSelector", "palNtsc", "usbAppInstaller"]
        );
    }

    #[test]
    fn the_video_system_is_named_rather_than_switched() {
        // The two tweaks touch different properties: one reveals the camera's
        // own menu entry, the other is the setting that entry would write.
        let mut p = patch();
        let before = list(&p).into_iter().find(|t| t.id == "palNtsc").unwrap();
        assert_eq!(before.value, "PAL");
        assert!(!before.enabled);
        assert_eq!(before.presentation, Presentation::Choice { off: "PAL", on: "NTSC" });
        set_enabled(&mut p, "palNtsc", true, false).unwrap();
        assert_eq!(p.read("palNtsc").unwrap(), vec![1]);
        assert_eq!(p.read("palNtscSelector").unwrap(), vec![0]);
        let after = list(&p).into_iter().find(|t| t.id == "palNtsc").unwrap();
        assert_eq!(after.value, "NTSC");
    }

    #[test]
    fn every_other_tweak_stays_a_plain_switch() {
        // Only the video system is a choice; anything else claiming to be one
        // would need its own names, and silence here is the reminder.
        for t in list(&patch()) {
            let expected = if t.id == "palNtsc" {
                Presentation::Choice { off: "PAL", on: "NTSC" }
            } else {
                Presentation::Toggle
            };
            assert_eq!(presentation_of(&t.id).unwrap(), expected, "{}", t.id);
        }
    }

    #[test]
    fn reports_the_factory_state_as_disabled() {
        let tweaks = list(&patch());
        let rec = tweaks.iter().find(|t| t.id == "recLimit").unwrap();
        assert!(!rec.enabled);
        assert_eq!(rec.value, "0h 29m 50s");
    }

    #[test]
    fn enabling_the_rec_limit_tweak_stages_the_unlocked_duration() {
        let mut p = patch();
        set_enabled(&mut p, "recLimit", true, false).unwrap();
        assert_eq!(p.read("recLimit").unwrap(), REC_LIMIT_ON);
        let rec = list(&p).into_iter().find(|t| t.id == "recLimit").unwrap();
        assert!(rec.enabled);
        assert_eq!(rec.value, "13h 01m 00s");
    }

    #[test]
    fn disabling_restores_the_factory_duration() {
        let mut p = patch();
        set_enabled(&mut p, "recLimit", true, false).unwrap();
        set_enabled(&mut p, "recLimit", false, false).unwrap();
        assert_eq!(p.read("recLimit").unwrap(), REC_LIMIT_OFF);
    }

    #[test]
    fn rec_limit_4k_renders_seconds_as_a_duration() {
        let mut p = patch();
        let before = list(&p).into_iter().find(|t| t.id == "recLimit4k").unwrap();
        assert_eq!(before.value, "0h 05m 00s");
        set_enabled(&mut p, "recLimit4k", true, false).unwrap();
        let after = list(&p).into_iter().find(|t| t.id == "recLimit4k").unwrap();
        assert!(after.enabled);
        assert_eq!(after.value, "9h 06m 07s");
    }

    #[test]
    fn boolean_tweak_toggles_a_single_byte() {
        let mut p = patch();
        set_enabled(&mut p, "usbAppInstaller", true, false).unwrap();
        assert_eq!(p.read("usbAppInstaller").unwrap(), vec![1]);
        let t = list(&p).into_iter().find(|t| t.id == "usbAppInstaller").unwrap();
        assert_eq!(t.value, "Enabled");
    }

    #[test]
    fn no_tweak_on_this_camera_needs_a_decision_first() {
        // Region EU_CE3 can show every language, so nothing here may ask.
        assert!(list(&patch()).iter().all(|t| t.caution.is_none()));
    }

    #[test]
    fn rejects_an_unknown_tweak() {
        let mut p = patch();
        assert!(matches!(set_enabled(&mut p, "nope", true, false), Err(Error::Unknown(_))));
    }

    #[test]
    fn rejects_a_tweak_whose_property_is_missing() {
        let mut p = patch();
        // `language` is not present in the sample image.
        assert!(set_enabled(&mut p, "language", true, false).is_err());
    }

    #[test]
    fn language_values_come_from_the_region_table() {
        // Region EU_CE3 -> table CE3; the tweak must be undoable to exactly it.
        let d = def("language").unwrap();
        let p = patch();
        assert_eq!(off_value(d, &p).unwrap(), default_languages("CE3").unwrap());
        assert_eq!(on_value(d, &p, false).unwrap().len(), LANGUAGE_COUNT);
        assert!(on_value(d, &p, false).unwrap().iter().all(|&l| l == LANG_ENABLED));
        assert_eq!(caution(d, &p), None);
    }

    #[test]
    fn a_japanese_camera_does_not_unlock_the_languages_it_cannot_show() {
        let d = def("language").unwrap();
        let p = patch_for_region("CX88600_J1");
        let on = on_value(d, &p, false).unwrap();
        assert_eq!(on.len(), LANGUAGE_COUNT);
        assert_eq!(on.iter().filter(|&&l| l == LANG_ENABLED).count(), LANGUAGE_COUNT - 2);
        // Undoing the tweak is unchanged: back to Sony's own table for J1.
        assert_eq!(off_value(d, &p).unwrap(), default_languages("J1").unwrap());
    }

    #[test]
    fn a_japanese_camera_announces_the_decision_it_needs() {
        let d = def("language").unwrap();
        let p = patch_for_region("CX88600_J1");
        assert_eq!(caution(d, &p), Some(Caution::UnsupportedLanguages));
        // Only the language tweak; nothing else on this camera is affected.
        assert_eq!(caution(def("recLimit").unwrap(), &p), None);
    }

    #[test]
    fn a_japanese_camera_can_unlock_everything_on_request() {
        let d = def("language").unwrap();
        let p = patch_for_region("CX88600_J1");
        let on = on_value(d, &p, true).unwrap();
        assert!(on.iter().all(|&l| l == LANG_ENABLED));
    }

    #[test]
    fn both_unlocked_variants_count_as_enabled() {
        // A user who deliberately accepted the caution must not be told their
        // choice did not take, so 35/35 and 33/35 both read as "on".
        let d = def("language").unwrap();
        let p = patch_for_region("CX88600_J1");
        assert!(is_enabled(d, &p, &on_value(d, &p, false).unwrap()).unwrap());
        assert!(is_enabled(d, &p, &on_value(d, &p, true).unwrap()).unwrap());
        // The factory table is still off, or the tweak could never be undone.
        assert!(!is_enabled(d, &p, &off_value(d, &p).unwrap()).unwrap());
    }

    #[test]
    fn language_state_is_rendered_as_a_count() {
        let d = def("language").unwrap();
        let mut data = vec![LANG_DISABLED; LANGUAGE_COUNT];
        data[0] = LANG_ENABLED;
        data[5] = LANG_ENABLED;
        assert_eq!(describe(d, &data, false), "2 / 35 language slots enabled");
    }
}
