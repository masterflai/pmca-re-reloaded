//! Values in a settings store that may identify a person or a camera, or open
//! a network or an account.
//!
//! The one place that decides it. Dumps, comparisons, the memory view and
//! every export ask [`classify`], and what they replace by default is listed
//! in [`CATALOG`] for the user to read.
//!
//! Three ways to recognise such a value, in this order:
//!
//! 1. **By id**: the serial number and the owner fields the toolkit knows, and
//!    ids measured on our own cameras by setting known placeholder values.
//! 2. **By name**: where an installed name table gives the firmware's name,
//!    words such as `WIFI`, `PASSWORD` or `IPTC` in it.
//! 3. **By content**: a value that reads as text and has no name that says it
//!    is harmless. Without the firmware's names for every field there is no
//!    knowing which text is a network name and which a menu label, so all of
//!    it counts — some harmless values are replaced too, on purpose.
//!
//! A whole backup image (`Backup.bin`) is never filtered: writing it back needs
//! every byte. It always holds all of this.

use pmca_i18n::Localizer;
use pmca_platform::props;
use serde::{Deserialize, Serialize};

/// What kind of sensitive value a property may hold.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export, rename = "SensitiveKind"))]
pub enum Kind {
    /// The camera's serial number.
    Serial,
    /// Photographer and copyright holder.
    Owner,
    /// Passwords and keys: WiFi, the camera's own access point, FTP, access
    /// authentication, proxy.
    Credentials,
    /// Network names (SSID), the device name, names of paired devices.
    NetworkName,
    /// MAC and IP addresses, gateway, DNS.
    Address,
    /// Accounts, tokens, stream keys, licence and activation data.
    Account,
    /// IPTC metadata: creator, contact, address.
    Iptc,
    /// Bluetooth pairing.
    Pairing,
    /// A location taken over from a phone.
    Location,
    /// Text without a name that says what it is.
    Text,
}

impl Kind {
    /// The catalogue key of the short label.
    #[must_use]
    pub const fn label_key(self) -> &'static str {
        match self {
            Self::Serial => "sensitive-kind-serial",
            Self::Owner => "sensitive-kind-owner",
            Self::Credentials => "sensitive-kind-credentials",
            Self::NetworkName => "sensitive-kind-network-name",
            Self::Address => "sensitive-kind-address",
            Self::Account => "sensitive-kind-account",
            Self::Iptc => "sensitive-kind-iptc",
            Self::Pairing => "sensitive-kind-pairing",
            Self::Location => "sensitive-kind-location",
            Self::Text => "sensitive-kind-text",
        }
    }

    /// The catalogue key of what the kind covers and how it is recognised.
    #[must_use]
    pub const fn detail_key(self) -> &'static str {
        match self {
            Self::Serial => "sensitive-kind-serial-detail",
            Self::Owner => "sensitive-kind-owner-detail",
            Self::Credentials => "sensitive-kind-credentials-detail",
            Self::NetworkName => "sensitive-kind-network-name-detail",
            Self::Address => "sensitive-kind-address-detail",
            Self::Account => "sensitive-kind-account-detail",
            Self::Iptc => "sensitive-kind-iptc-detail",
            Self::Pairing => "sensitive-kind-pairing-detail",
            Self::Location => "sensitive-kind-location-detail",
            Self::Text => "sensitive-kind-text-detail",
        }
    }

    /// The word written into an export in place of the value.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Serial => "serial",
            Self::Owner => "owner",
            Self::Credentials => "credentials",
            Self::NetworkName => "networkName",
            Self::Address => "address",
            Self::Account => "account",
            Self::Iptc => "iptc",
            Self::Pairing => "pairing",
            Self::Location => "location",
            Self::Text => "text",
        }
    }
}

/// Every kind, in the order the catalogue lists them.
pub const CATALOG: &[Kind] = &[
    Kind::Serial,
    Kind::Owner,
    Kind::Credentials,
    Kind::NetworkName,
    Kind::Address,
    Kind::Account,
    Kind::Iptc,
    Kind::Pairing,
    Kind::Location,
    Kind::Text,
];

/// Files that always hold everything, whatever an export would replace.
pub const ALWAYS_COMPLETE: &[&str] = &[
    "sensitive-full-backup",
    "sensitive-full-rollback",
    "sensitive-full-network",
    "sensitive-full-usage-log",
    "sensitive-full-raw",
];

/// The catalogue as a front-end shows it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct Catalog {
    /// What the catalogue is about and what it cannot promise.
    pub intro: Vec<String>,
    /// One line per kind of value that is replaced by default.
    pub kinds: Vec<CatalogKind>,
    /// Heading of the files that always hold everything.
    pub complete_heading: String,
    /// The files that always hold everything.
    pub complete: Vec<String>,
}

/// One kind of value in the catalogue.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct CatalogKind {
    /// The kind.
    pub kind: Kind,
    /// Its label.
    pub label: String,
    /// What it covers and how it is recognised.
    pub detail: String,
}

/// The catalogue in the user's language.
#[must_use]
pub fn catalog(l: &Localizer) -> Catalog {
    Catalog {
        intro: ["sensitive-intro", "sensitive-uncertain", "sensitive-default"]
            .iter()
            .map(|k| l.tr(k))
            .collect(),
        kinds: CATALOG
            .iter()
            .map(|&kind| CatalogKind {
                kind,
                label: l.tr(kind.label_key()),
                detail: l.tr(kind.detail_key()),
            })
            .collect(),
        complete_heading: l.tr("sensitive-full-heading"),
        complete: ALWAYS_COMPLETE.iter().map(|k| l.tr(k)).collect(),
    }
}

/// Ids measured on our own cameras by setting a known placeholder in the menu
/// and finding it in the next image: id, value size, kind.
///
/// Only taken where the size matches as well, so the same id on another body
/// that uses it for something else of a different length is not caught.
const MEASURED: &[(u32, usize, Kind)] = &[];

/// Why `id` may hold a sensitive value, or `None`.
///
/// `data` is the value as stored, `fw_name` the firmware's name where a table
/// gives one.
#[must_use]
pub fn classify(id: u32, data: &[u8], fw_name: Option<&str>) -> Option<Kind> {
    by_id(id, data.len()).or_else(|| fw_name.and_then(by_name)).or_else(|| {
        (!known_harmless(id, data.len()) && looks_like_text(data)).then_some(Kind::Text)
    })
}

fn by_id(id: u32, size: usize) -> Option<Kind> {
    let known = props::PROPERTIES.iter().find(|p| p.part_size == size && p.ids().any(|i| i == id));
    match known.map(|p| p.name) {
        Some("serialNumber") => return Some(Kind::Serial),
        Some(
            "photographer1" | "photographer2" | "photographer1Alt" | "copyright1" | "copyright2"
            | "copyright1Alt",
        ) => return Some(Kind::Owner),
        _ => {}
    }
    MEASURED.iter().find(|(i, s, _)| *i == id && *s == size).map(|(_, _, k)| *k)
}

/// Whether the toolkit names this property as a setting, which says it is not
/// one of the sensitive fields.
fn known_harmless(id: u32, size: usize) -> bool {
    props::PROPERTIES.iter().any(|p| p.part_size == size && p.ids().any(|i| i == id))
}

/// The kind a firmware name points at, if any.
///
/// Most terms are matched as whole `_`-separated tokens, so `OWNERSHIP` does
/// not trigger on `OWNER` the way `OWNER_NAME` does; the handful the firmware
/// spells as one compound word are matched as substrings instead.
#[must_use]
pub fn by_name(name: &str) -> Option<Kind> {
    let upper = name.to_ascii_uppercase();
    let tokens: Vec<&str> = upper.split(['_', ' ', '|']).collect();
    let has = |words: &[&str]| tokens.iter().any(|t| words.contains(t));
    let contains = |parts: &[&str]| parts.iter().any(|p| upper.contains(p));

    if has(&[
        "PASSWORD",
        "PASSWD",
        "PASSPHRASE",
        "CREDENTIAL",
        "FINGER",
        "PSK",
        "WPA",
        "FTP",
        "SSH",
        "PROXY",
    ]) {
        return Some(Kind::Credentials);
    }
    if contains(&["MAC_ADDR", "IP_ADDR", "BSSID", "GATEWAY", "DNS_"]) {
        return Some(Kind::Address);
    }
    if has(&["ACCOUNT", "TOKEN", "LICENSE", "LICENCE", "STREAMKEY"]) {
        return Some(Kind::Account);
    }
    if has(&["IPTC"]) {
        return Some(Kind::Iptc);
    }
    if has(&["BLUETOOTH", "PAIRING"]) {
        return Some(Kind::Pairing);
    }
    if has(&["LATITUDE", "LONGITUDE"]) || contains(&["LOCATION_INFO"]) {
        return Some(Kind::Location);
    }
    if has(&["OWNER", "COPYRIGHT", "PHOTOGRAPHER", "CREATOR"]) {
        return Some(Kind::Owner);
    }
    if has(&["WIFI", "WLAN", "SSID"]) || contains(&["DEVICE_NAME", "USER_NAME"]) {
        return Some(Kind::NetworkName);
    }
    None
}

/// Whether `data` reads as a string: written words rather than numbers that
/// happen to be printable, as UTF-8 or as UTF-16LE, then nothing but zeros.
///
/// Measured against real images: arrays of one repeated byte (`UUUU`, `,,,,`)
/// and random bytes that decode to stray symbols are common and are no text;
/// what is asked for is at least two different characters, at least two of
/// them ASCII letters or digits, or else nothing but letters of another
/// script.
#[must_use]
pub fn looks_like_text(data: &[u8]) -> bool {
    let padded = |rest: &[u8]| rest.iter().all(|&b| b == 0);

    let end = data.iter().position(|&b| b == 0).unwrap_or(data.len());
    if let Ok(text) = std::str::from_utf8(&data[..end])
        && reads_as_words(text)
        && padded(&data[end..])
    {
        return true;
    }

    let units: Vec<u16> =
        data.as_chunks::<2>().0.iter().map(|&pair| u16::from_le_bytes(pair)).collect();
    let end = units.iter().position(|&u| u == 0).unwrap_or(units.len());
    let Ok(text) = String::from_utf16(&units[..end]) else { return false };
    // Latin text in UTF-16 has most second bytes zero; that is the shape asked
    // for, not a run of numbers that happens to decode.
    units[..end].iter().filter(|&&u| u < 0x100).count() * 2 >= end
        && reads_as_words(&text)
        && padded(&data[end * 2..])
}

fn reads_as_words(text: &str) -> bool {
    let chars: Vec<char> = text.chars().collect();
    if chars.iter().any(|c| c.is_control() || *c == '\u{fffd}') {
        return false;
    }
    let mut distinct = chars.clone();
    distinct.sort_unstable();
    distinct.dedup();
    let alnum = chars.iter().filter(|c| c.is_ascii_alphanumeric()).count();
    let foreign_letters = chars.iter().all(|c| !c.is_ascii() && c.is_alphabetic());
    distinct.len() >= 2
        && ((chars.len() >= 3 && alnum >= 2) || (chars.len() >= 2 && foreign_letters))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_serial_number_and_the_owner_fields_are_known_by_id() {
        assert_eq!(classify(0x00e7_0003, &[1, 2, 3, 4], None), Some(Kind::Serial));
        assert_eq!(classify(0x02cf_0bd1, &[0; 47], None), Some(Kind::Owner));
        // Same id, other size: not the field the toolkit knows.
        assert_eq!(classify(0x00e7_0003, &[1, 2], None), None);
    }

    #[test]
    fn firmware_names_point_at_the_kind() {
        for (name, kind) in [
            ("BKID_APP_WIFI_SSID", Kind::NetworkName),
            ("BKID_NET_WLAN_PASSWORD", Kind::Credentials),
            ("BKID_FTP_SERVER_USER", Kind::Credentials),
            ("BKID_NET_SSH_FINGER_PRINT", Kind::Credentials),
            ("BKID_NET_MAC_ADDRESS", Kind::Address),
            ("BKID_APP_LICENSE_INFO", Kind::Account),
            ("BKID_APP_IPTC_CREATOR", Kind::Iptc),
            ("BKID_APP_DEVICE_NAME", Kind::NetworkName),
            ("BKID_APP_OWNER_NAME", Kind::Owner),
            ("BKID_BLUETOOTH_PAIRING_LIST", Kind::Pairing),
            ("BKID_APP_FTP_SERVER_PASSWORD", Kind::Credentials),
            ("PARENT_CHILD_SERVER_CAMERA_SSH_FINGER_PRINT_FIRST_HALF", Kind::Credentials),
            ("BKID_MAC_ADDR", Kind::Address),
            ("BKID_NET_SERVICE_DEVICE_NAME_VALID", Kind::NetworkName),
            ("BKID_APP_LICENSE_KEY", Kind::Account),
        ] {
            assert_eq!(by_name(name), Some(kind), "{name}");
        }
        for name in [
            "BKID_APP_MR_SHUTTER_SPEED",
            "BKID_APP_OWNERSHIP_FLAG",
            "BKID_APP_MENU_NTSC_PAL",
            "BKID_DISPLAY_EVF_DEVICE",
            "BKID_APP_SETTING_GRID",
        ] {
            assert_eq!(by_name(name), None, "{name}");
        }
    }

    #[test]
    fn text_counts_as_sensitive_unless_the_toolkit_names_the_field() {
        assert_eq!(classify(0x0100_0001, b"HomeNet\0\0\0\0\0", None), Some(Kind::Text));
        assert_eq!(classify(0x0100_0001, "Jürgen\0\0".as_bytes(), None), Some(Kind::Text));
        let utf16: Vec<u8> =
            "Anna".encode_utf16().flat_map(u16::to_le_bytes).chain([0, 0]).collect();
        assert_eq!(classify(0x0100_0001, &utf16, None), Some(Kind::Text));
        // The model name is text, but the toolkit knows what it is.
        assert_eq!(classify(0x003e_0005, b"ILCE-7CR\0\0\0\0\0\0\0\0", None), None);
        assert_eq!(classify(0x0100_0001, "山田太郎\0".as_bytes(), None), Some(Kind::Text));
    }

    #[test]
    fn numbers_and_short_values_are_not_text() {
        for data in [
            &[0x01, 0x00][..],
            &[0x41, 0x42],
            &[0x0a, 0x00, 0x0b, 0x00],
            &[0xab; 8],
            &[0; 16],
            b"AB\0\x01",
            b"UUUU",
            b",,,,,,,,",
            "ÿ䔀 ".as_bytes(),
            &[0xbb, 0x00, 0xbb, 0x00, 0xd3, 0xfc],
        ] {
            assert!(!looks_like_text(data), "{data:?}");
        }
    }

    #[test]
    fn the_catalogue_is_written_out_in_words() {
        for locale in ["en-US", "de-DE", "ja-JP"] {
            let c = catalog(&Localizer::new(locale));
            let texts = c
                .intro
                .iter()
                .chain(&c.complete)
                .chain([&c.complete_heading])
                .chain(c.kinds.iter().flat_map(|k| [&k.label, &k.detail]));
            for text in texts {
                assert!(!text.starts_with("sensitive-"), "{locale}: {text}");
            }
        }
    }

    #[test]
    fn every_kind_is_in_the_catalogue_once() {
        let mut seen = CATALOG.to_vec();
        seen.sort();
        seen.dedup();
        assert_eq!(seen.len(), CATALOG.len());
        assert_eq!(CATALOG.len(), 10);
    }
}
