//! The device lists: which Sony models reach which of this program's features.
//!
//! Shared by both front-ends, so the console warns about the same models the
//! window does. The lists say what is known, not what is possible; a model a
//! list does not vouch for may still work.

use serde::{Deserialize, Serialize};

use pmca_i18n::Localizer;

/// A feature whose model support the lists record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub enum Capability {
    /// Firmware update and updater shell.
    Updater,
    /// PlayMemories Camera Apps.
    Apps,
    /// GPS assist data.
    Gps,
}

impl Capability {
    fn id(self) -> &'static str {
        match self {
            Self::Updater => "updater",
            Self::Apps => "apps",
            Self::Gps => "gps",
        }
    }
}

/// What the lists say about one model and one capability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub enum Support {
    /// Listed as supported.
    Yes,
    /// Listed as not supported.
    No,
    /// Listed, but nobody has confirmed it either way.
    Untested,
    /// In no list.
    Unknown,
}

impl Support {
    fn id(self) -> &'static str {
        match self {
            Self::Yes => "yes",
            Self::No => "no",
            Self::Untested => "untested",
            Self::Unknown => "unknown",
        }
    }
}

/// What the community reports about a model the original project does not list.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub enum CommunityReport {
    /// Reported to work with the original tool.
    Works,
    /// Failures were reported that may no longer hold.
    Disputed,
}

/// A model of the original project's list.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct CompatModel {
    /// Model name as the camera reports it.
    pub model: String,
    /// Processor architecture.
    pub arch: String,
    /// Version of the app platform, `None` where the model runs no apps.
    pub apps: Option<String>,
    /// Whether the updater shell reaches the model.
    pub updater: bool,
}

/// A further Sony model, outside the original project's list.
///
/// Nothing is known about the updater shell on these, so it counts as
/// unavailable; the service shell is assumed unless reports dispute it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct MoreModel {
    /// Model name as the camera reports it.
    pub model: String,
    /// Processor architecture, where a source names it.
    pub arch: Option<String>,
    /// Whether the model runs PlayMemories Camera Apps, where known.
    pub apps: Option<bool>,
    /// The community's report, if there is one.
    pub report: Option<CommunityReport>,
}

/// A model with a GPS receiver.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct GpsModel {
    /// Model family as the original list names it.
    pub model: String,
    /// The variant that carries the receiver (Sony's "V" suffix).
    pub variant: String,
    /// Processor architecture, where known.
    pub arch: Option<String>,
    /// Whether the model runs PlayMemories Camera Apps, where known.
    pub apps: Option<bool>,
}

/// Models under a heading (a catalogue key).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct DeviceGroup<T> {
    /// Catalogue key of the heading.
    pub heading: String,
    /// The models, in display order.
    pub models: Vec<T>,
}

/// Every device list, grouped as the window shows them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct DeviceLists {
    /// The original project's list.
    pub compat: Vec<DeviceGroup<CompatModel>>,
    /// Further Sony models.
    pub more: Vec<DeviceGroup<MoreModel>>,
    /// Models with a GPS receiver.
    pub gps: Vec<DeviceGroup<GpsModel>>,
    /// Models with a GPS receiver nobody has tried.
    pub gps_untested: Vec<GpsModel>,
}

type Compat = (&'static str, &'static str, Option<&'static str>, bool);
type More = (&'static str, Option<&'static str>, Option<bool>, Option<CommunityReport>);
type Gps = (&'static str, &'static str, Option<&'static str>, Option<bool>);

fn compat_model(&(model, arch, apps, updater): &Compat) -> CompatModel {
    CompatModel { model: model.into(), arch: arch.into(), apps: apps.map(Into::into), updater }
}

fn more_model(&(model, arch, apps, report): &More) -> MoreModel {
    MoreModel { model: model.into(), arch: arch.map(Into::into), apps, report }
}

fn gps_model(&(model, variant, arch, apps): &Gps) -> GpsModel {
    GpsModel { model: model.into(), variant: variant.into(), arch: arch.map(Into::into), apps }
}

fn groups<R, T>(
    table: &[(&str, &[R])],
    keep: impl Fn(&R) -> bool,
    convert: impl Fn(&R) -> T,
) -> Vec<DeviceGroup<T>> {
    table
        .iter()
        .map(|(heading, rows)| DeviceGroup {
            heading: (*heading).to_string(),
            models: rows.iter().filter(|r| keep(r)).map(&convert).collect(),
        })
        .filter(|g| !g.models.is_empty())
        .collect()
}

/// All lists.
pub fn lists() -> DeviceLists {
    DeviceLists {
        compat: groups(COMPAT, |_| true, compat_model),
        more: groups(MORE, |_| true, more_model),
        gps: groups(GPS, |_| true, gps_model),
        gps_untested: GPS_UNTESTED.iter().map(gps_model).collect(),
    }
}

/// The original and the further models that have `capability`.
///
/// Only the two general lists take part; the GPS lists name receivers, not
/// the features this narrows down to.
pub fn capable(capability: Capability) -> DeviceLists {
    DeviceLists {
        compat: groups(COMPAT, |r| compat_support(r, capability) == Support::Yes, compat_model),
        more: groups(MORE, |r| more_support(r, capability) == Support::Yes, more_model),
        gps: Vec::new(),
        gps_untested: Vec::new(),
    }
}

fn compat_support(&(_, _, apps, updater): &Compat, capability: Capability) -> Support {
    let yes = match capability {
        Capability::Apps => apps.is_some(),
        Capability::Updater => updater,
        Capability::Gps => false,
    };
    if yes { Support::Yes } else { Support::No }
}

fn more_support(&(_, _, apps, _): &More, capability: Capability) -> Support {
    match (capability, apps) {
        (Capability::Apps, Some(true)) => Support::Yes,
        (Capability::Apps, None) => Support::Untested,
        _ => Support::No,
    }
}

/// Find `model` in a grouped table.
///
/// Several lists name a model family (DSC-HX60) where the camera may report the
/// variant with Sony's GPS suffix (DSC-HX60V), so a name ending in V is also
/// tried without it.
fn find<'a, R>(table: &'a [(&str, &[R])], name: impl Fn(&R) -> &str, model: &str) -> Option<&'a R> {
    let rows = || table.iter().flat_map(|(_, rows)| rows.iter());
    let key = model.trim().to_uppercase();
    let by = |wanted: &str| rows().find(|r| name(r).eq_ignore_ascii_case(wanted));
    by(&key).or_else(|| key.strip_suffix('V').and_then(by))
}

/// What the lists say about `model` and `capability`.
pub fn support(model: &str, capability: Capability) -> Support {
    let listed = find(COMPAT, |r| r.0, model);
    let more = if listed.is_some() { None } else { find(MORE, |r| r.0, model) };
    if capability == Capability::Gps {
        let key = model.trim().to_uppercase();
        if GPS.iter().any(|(_, rows)| rows.iter().any(|r| r.1 == key)) {
            return Support::Yes;
        }
        if GPS_UNTESTED.iter().any(|r| r.1 == key) {
            return Support::Untested;
        }
        return if listed.is_some() || more.is_some() { Support::No } else { Support::Unknown };
    }
    match (listed, more) {
        (Some(row), _) => compat_support(row, capability),
        (None, Some(row)) => more_support(row, capability),
        (None, None) => Support::Unknown,
    }
}

/// The sentence that tells the user what the lists say about their camera.
///
/// `None` is a camera whose model could not be read.
pub fn support_text(l: &Localizer, model: Option<&str>, capability: Capability) -> String {
    match model {
        None => l.tr("check-unidentified"),
        Some(model) => l.tr_arg(
            &format!("check-{}-{}", capability.id(), support(model, capability).id()),
            "model",
            model,
        ),
    }
}

const COMPAT: &[(&str, &[Compat])] = &[
    (
        "devices-group-ilc",
        &[
            ("ILCA-77M2", "CXD90014", None, true),
            ("ILCA-99M2", "CXD90014", None, true),
            ("ILCE-5000", "CXD90014", Some("2.3"), true),
            ("ILCE-5100", "CXD90014", Some("2.7"), true),
            ("ILCE-6000", "CXD90014", Some("2.4"), true),
            ("ILCE-6300", "CXD90014", Some("2.15"), true),
            ("ILCE-6400", "CXD90045", None, false),
            ("ILCE-6500", "CXD90014", Some("2.17"), true),
            ("ILCE-6600", "CXD90045", None, false),
            ("ILCE-7", "CXD90014", Some("2.3"), true),
            ("ILCE-7C", "CXD90045", None, false),
            ("ILCE-7M2", "CXD90014", Some("2.10"), true),
            ("ILCE-7M3", "CXD90045", None, false),
            ("ILCE-7R", "CXD90014", Some("2.3"), true),
            ("ILCE-7RM2", "CXD90014", Some("2.14"), true),
            ("ILCE-7RM3", "CXD90045", None, false),
            ("ILCE-7RM4", "CXD90045", None, false),
            ("ILCE-7RM4A", "CXD90045", None, false),
            ("ILCE-7S", "CXD90014", Some("2.6"), true),
            ("ILCE-7SM2", "CXD90014", Some("2.14"), true),
            ("ILCE-9", "CXD90045", None, false),
            ("ILCE-9M2", "CXD90045", None, false),
            ("ILCE-QX1", "CXD90014", Some("2.8"), true),
            ("NEX-3", "CXD4115", None, true),
            ("NEX-3N", "CXD4132", None, true),
            ("NEX-5", "CXD4115", None, true),
            ("NEX-5N", "CXD4132", None, true),
            ("NEX-5R", "CXD4132", Some("1.1"), true),
            ("NEX-5T", "CXD4132", Some("1.1"), true),
            ("NEX-6", "CXD4132", Some("1.1"), true),
            ("NEX-7", "CXD4132", None, true),
            ("NEX-C3", "CXD4115", None, true),
            ("NEX-F3", "CXD4132", None, true),
            ("NEX-VG10", "CXD4115", None, true),
            ("SLT-A33", "CXD4115", None, true),
            ("SLT-A37", "CXD4132", None, true),
            ("SLT-A55", "CXD4115", None, true),
            ("SLT-A57", "CXD4132", None, true),
            ("SLT-A58", "CXD4132", None, true),
            ("SLT-A65", "CXD4132", None, true),
            ("SLT-A77", "CXD4132", None, true),
            ("SLT-A99", "CXD4132", None, true),
            ("ZV-E10", "CXD90045", None, false),
        ],
    ),
    (
        "gps-devices-compact",
        &[
            ("DSC-G1", "CXD4105", None, false),
            ("DSC-G3", "CXD4108", None, false),
            ("DSC-HX5V", "CXD4115", None, true),
            ("DSC-HX9V", "CXD4132", None, true),
            ("DSC-HX20", "CXD4132", None, true),
            ("DSC-HX30", "CXD4132", None, true),
            ("DSC-HX50", "CXD4132", None, true),
            ("DSC-HX60", "CXD90014", Some("2.4"), true),
            ("DSC-HX90", "CXD90014", Some("2.13"), true),
            ("DSC-HX99", "CXD90045", None, false),
            ("DSC-HX100V", "CXD4132", None, true),
            ("DSC-HX200V", "CXD4132", None, true),
            ("DSC-HX300", "CXD4132", None, true),
            ("DSC-HX350", "CXD90014", None, true),
            ("DSC-HX400", "CXD90014", Some("2.4"), true),
            ("DSC-QX10", "CXD4132", None, true),
            ("DSC-QX100", "CXD4132", None, true),
            ("DSC-RX0", "CXD90014", None, true),
            ("DSC-RX0M2", "CXD90045", None, false),
            ("DSC-RX1", "CXD4132", None, true),
            ("DSC-RX1R", "CXD4132", None, true),
            ("DSC-RX1RM2", "CXD90014", Some("2.14"), true),
            ("DSC-RX10", "CXD90014", None, true),
            ("DSC-RX10M2", "CXD90014", Some("2.14"), true),
            ("DSC-RX10M3", "CXD90014", Some("2.15"), true),
            ("DSC-RX10M4", "CXD90045", None, false),
            ("DSC-RX100", "CXD4132", None, true),
            ("DSC-RX100M2", "CXD4132", None, true),
            ("DSC-RX100M3", "CXD90014", Some("2.6"), true),
            ("DSC-RX100M4", "CXD90014", Some("2.14"), true),
            ("DSC-RX100M5", "CXD90014", Some("2.16"), true),
            ("DSC-RX100M5A", "CXD90045", None, false),
            ("DSC-T100", "CXD4108", None, false),
            ("DSC-TX5", "CXD4115", None, true),
            ("DSC-TX7", "CXD4115", None, true),
            ("DSC-W90", "CXD4108", None, false),
            ("DSC-WX5", "CXD4115", None, true),
            ("DSC-WX60", "CXD4132", None, true),
            ("DSC-WX80", "CXD4132", None, true),
            ("DSC-WX100", "CXD4132", None, true),
            ("DSC-WX200", "CXD4132", None, true),
            ("DSC-WX220", "CXD90014", None, true),
            ("DSC-WX300", "CXD4132", None, true),
            ("DSC-WX350", "CXD90014", None, true),
            ("DSC-WX500", "CXD90014", Some("2.13"), true),
            ("ZV-1", "CXD90045", None, false),
        ],
    ),
    (
        "gps-devices-action",
        &[
            ("FDR-X1000V", "CXD90014", Some("2.12"), true),
            ("FDR-X3000", "CXD90014", Some("2.15"), true),
            ("HDR-AS10", "CXD4132", None, true),
            ("HDR-AS15", "CXD4132", None, true),
            ("HDR-AS30V", "CXD4132", None, true),
            ("HDR-AS50", "CXD90014", Some("2.15"), true),
            ("HDR-AS100V", "CXD90014", Some("2.5"), true),
            ("HDR-AS200V", "CXD90014", Some("2.12"), true),
            ("HDR-AS300", "CXD90014", Some("2.15"), true),
            ("HDR-AZ1", "CXD90014", Some("2.9"), true),
        ],
    ),
    (
        "gps-devices-camcorder",
        &[
            ("DCR-SX31", "MB8AC102", None, true),
            ("FDR-AX30", "CXD90014", Some("2.11"), true),
            ("FDR-AX33", "CXD90014", Some("2.11"), true),
            ("FDR-AX40", "CXD90014", Some("2.15"), true),
            ("FDR-AX43", "CXD90014", None, true),
            ("FDR-AX45", "CXD90014", None, true),
            ("FDR-AX53", "CXD90014", Some("2.15"), true),
            ("FDR-AX55", "CXD90014", Some("2.15"), true),
            ("FDR-AX60", "CXD90014", None, true),
            ("FDR-AX100", "CXD90014", None, true),
            ("FDR-AX700", "CXD90045", None, false),
            ("FDR-AXP33", "CXD90014", Some("2.11"), true),
            ("FDR-AXP35", "CXD90014", Some("2.11"), true),
            ("FDR-AXP55", "CXD90014", Some("2.15"), true),
            ("HDR-CX106", "CXD4105", None, true),
            ("HDR-CX240", "CXD90014", None, true),
            ("HDR-CX405", "CXD90014", Some("2.11"), true),
            ("HDR-CX450", "CXD90014", Some("2.15"), true),
            ("HDR-CX560", "CXD4132", None, true),
            ("HDR-CX625", "CXD90014", Some("2.15"), true),
            ("HDR-CX670", "CXD90014", Some("2.11"), true),
            ("HDR-CX675", "CXD90014", Some("2.15"), true),
            ("HDR-CX680", "CXD90014", Some("2.15"), true),
            ("HDR-CX700", "CXD4132", None, true),
            ("HDR-CX900", "CXD90014", None, true),
            ("HDR-MV1", "CXD4132", None, true),
            ("HDR-PJ240", "CXD90014", None, true),
            ("HDR-PJ410", "CXD90014", Some("2.11"), true),
            ("HDR-PJ540", "CXD90014", None, true),
            ("HDR-PJ590V", "CXD4132", None, true),
            ("HDR-PJ670", "CXD90014", Some("2.11"), true),
            ("HDR-PJ675", "CXD90014", Some("2.15"), true),
            ("HDR-PJ760V", "CXD4132", None, true),
            ("HDR-PJ790V", "CXD4132", None, true),
            ("HDR-PJ800", "CXD90014", None, true),
            ("HDR-PJ810", "CXD90014", None, true),
            ("HDR-SR1", "CXD4105", None, false),
            ("HDR-TD10", "CXD4132", None, true),
            ("HDR-TD20", "CXD4132", None, true),
            ("HDR-UX1", "CXD4105", None, false),
            ("HDR-XR550", "CXD4120", None, true),
        ],
    ),
    (
        "devices-group-remote",
        &[
            ("RM-LVR1", "CXD90014", None, true),
            ("RM-LVR2", "CXD90014", None, true),
            ("RM-LVR3", "CXD90014", None, true),
        ],
    ),
];

const MORE: &[(&str, &[More])] = &[
    (
        "devices-group-ilc",
        &[
            ("DSLR-A100", None, Some(false), None),
            ("DSLR-A200", None, Some(false), None),
            ("DSLR-A230", None, Some(false), None),
            ("DSLR-A290", None, Some(false), None),
            ("DSLR-A300", None, Some(false), None),
            ("DSLR-A330", None, Some(false), None),
            ("DSLR-A350", None, Some(false), None),
            ("DSLR-A380", None, Some(false), None),
            ("DSLR-A390", None, Some(false), None),
            ("DSLR-A450", None, Some(false), None),
            ("DSLR-A500", None, Some(false), None),
            ("DSLR-A550", None, Some(false), None),
            ("DSLR-A560", None, Some(false), None),
            ("DSLR-A580", None, Some(false), None),
            ("DSLR-A700", None, Some(false), None),
            ("DSLR-A850", None, Some(false), None),
            ("DSLR-A900", None, Some(false), None),
            ("ILCA-68", None, Some(false), None),
            ("ILCE-1", Some("CXD90057"), Some(false), Some(CommunityReport::Works)),
            ("ILCE-1M2", Some("CXD90057"), Some(false), Some(CommunityReport::Works)),
            ("ILCE-3000", None, Some(false), None),
            ("ILCE-3500", None, Some(false), None),
            ("ILCE-6100", Some("CXD90045"), Some(false), Some(CommunityReport::Works)),
            ("ILCE-6700", Some("CXD90057"), Some(false), Some(CommunityReport::Works)),
            ("ILCE-7CM2", Some("CXD90057"), Some(false), Some(CommunityReport::Works)),
            ("ILCE-7CR", Some("CXD90057"), Some(false), Some(CommunityReport::Works)),
            ("ILCE-7M4", Some("CXD90057"), Some(false), Some(CommunityReport::Works)),
            ("ILCE-7M5", Some("CXD90057"), Some(false), Some(CommunityReport::Disputed)),
            ("ILCE-7RM3A", Some("CXD90045"), Some(false), None),
            ("ILCE-7RM5", Some("CXD90057"), Some(false), Some(CommunityReport::Works)),
            ("ILCE-7RM6", Some("CXD90057"), Some(false), Some(CommunityReport::Disputed)),
            ("ILCE-7SM3", Some("CXD90057"), Some(false), Some(CommunityReport::Works)),
            ("ILCE-9M3", Some("CXD90057"), Some(false), Some(CommunityReport::Works)),
            ("ILME-FX2", None, Some(false), Some(CommunityReport::Disputed)),
            ("ILME-FX3", None, Some(false), Some(CommunityReport::Works)),
            ("ILME-FX30", None, Some(false), Some(CommunityReport::Works)),
            ("ILME-FX3A", None, Some(false), Some(CommunityReport::Works)),
            ("ILME-FX6", None, Some(false), Some(CommunityReport::Disputed)),
            ("NEX-VG20", None, Some(false), None),
            ("NEX-VG30", None, Some(false), None),
            ("NEX-VG900", None, Some(false), None),
            ("SLT-A35", None, Some(false), None),
            ("ZV-E1", Some("CXD90057"), Some(false), Some(CommunityReport::Works)),
            ("ZV-E10M2", Some("CXD90057"), Some(false), Some(CommunityReport::Works)),
        ],
    ),
    (
        "gps-devices-compact",
        &[
            ("DSC-HX1", None, Some(false), None),
            ("DSC-HX7V", None, Some(false), Some(CommunityReport::Works)),
            ("DSC-HX10V", None, Some(false), Some(CommunityReport::Works)),
            ("DSC-HX80", None, Some(true), Some(CommunityReport::Works)),
            ("DSC-HX95", None, Some(false), None),
            ("DSC-QX30", None, None, None),
            ("DSC-RX1RM3", Some("CXD90057"), Some(false), Some(CommunityReport::Works)),
            ("DSC-RX100M6", Some("CXD90045"), Some(false), Some(CommunityReport::Works)),
            ("DSC-RX100M7", Some("CXD90045"), Some(false), Some(CommunityReport::Works)),
            ("DSC-TX300V", None, Some(false), Some(CommunityReport::Works)),
            ("DSC-W830", None, Some(false), Some(CommunityReport::Disputed)),
            ("DSC-WX30", None, Some(false), None),
            ("DSC-WX170", None, Some(false), Some(CommunityReport::Disputed)),
            ("DSC-WX800", Some("CXD90045"), Some(false), Some(CommunityReport::Works)),
            ("ZV-1F", Some("CXD90057"), Some(false), Some(CommunityReport::Disputed)),
            ("ZV-1M2", Some("CXD90045"), Some(false), Some(CommunityReport::Works)),
        ],
    ),
    ("gps-devices-camcorder", &[("HDR-PJ30E", None, Some(false), Some(CommunityReport::Works))]),
];

const GPS: &[(&str, &[Gps])] = &[
    (
        "gps-devices-compact",
        &[
            ("DSC-HX5V", "DSC-HX5V", Some("CXD4115"), Some(false)),
            ("DSC-HX9V", "DSC-HX9V", Some("CXD4132"), Some(false)),
            ("DSC-HX20", "DSC-HX20V", Some("CXD4132"), Some(false)),
            ("DSC-HX30", "DSC-HX30V", Some("CXD4132"), Some(false)),
            ("DSC-HX50", "DSC-HX50V", Some("CXD4132"), Some(false)),
            ("DSC-HX60", "DSC-HX60V", Some("CXD90014"), Some(true)),
            ("DSC-HX90", "DSC-HX90V", Some("CXD90014"), Some(true)),
            ("DSC-HX100V", "DSC-HX100V", Some("CXD4132"), Some(false)),
            ("DSC-HX200V", "DSC-HX200V", Some("CXD4132"), Some(false)),
            ("DSC-HX400", "DSC-HX400V", Some("CXD90014"), Some(true)),
        ],
    ),
    (
        "gps-devices-slt",
        &[
            ("SLT-A55", "SLT-A55V", Some("CXD4115"), Some(false)),
            ("SLT-A65", "SLT-A65V", Some("CXD4132"), Some(false)),
            ("SLT-A77", "SLT-A77V", Some("CXD4132"), Some(false)),
            ("SLT-A99", "SLT-A99V", Some("CXD4132"), Some(false)),
        ],
    ),
    (
        "gps-devices-action",
        &[
            ("HDR-AS30V", "HDR-AS30V", Some("CXD4132"), Some(false)),
            ("HDR-AS100V", "HDR-AS100V", Some("CXD90014"), Some(true)),
            ("HDR-AS200V", "HDR-AS200V", Some("CXD90014"), Some(true)),
            ("FDR-X1000V", "FDR-X1000V", Some("CXD90014"), Some(true)),
        ],
    ),
    (
        "gps-devices-camcorder",
        &[
            ("HDR-CX560", "HDR-CX560V", Some("CXD4132"), Some(false)),
            ("HDR-CX700", "HDR-CX700V", Some("CXD4132"), Some(false)),
            ("HDR-XR550", "HDR-XR550V", Some("CXD4120"), Some(false)),
            ("HDR-PJ590V", "HDR-PJ590V", Some("CXD4132"), Some(false)),
            ("HDR-PJ760V", "HDR-PJ760V", Some("CXD4132"), Some(false)),
            ("HDR-PJ790V", "HDR-PJ790V", Some("CXD4132"), Some(false)),
            ("HDR-TD10", "HDR-TD10", Some("CXD4132"), Some(false)),
            ("HDR-TD20", "HDR-TD20V", Some("CXD4132"), Some(false)),
        ],
    ),
];

const GPS_UNTESTED: &[Gps] = &[
    ("DSC-HX7V", "DSC-HX7V", None, None),
    ("DSC-HX10V", "DSC-HX10V", None, None),
    ("DSC-TX100V", "DSC-TX100V", None, None),
    ("DSC-TX200V", "DSC-TX200V", None, None),
    ("DSC-TX300V", "DSC-TX300V", None, None),
    ("HDR-XR520V", "HDR-XR520V", None, None),
    ("HDR-CX550V", "HDR-CX550V", None, None),
    ("HDR-CX260V", "HDR-CX260V", None, None),
    ("HDR-PJ260V", "HDR-PJ260V", None, None),
    ("HDR-CX580V", "HDR-CX580V", None, None),
    ("HDR-PJ580V", "HDR-PJ580V", None, None),
    ("HDR-PJ710V", "HDR-PJ710V", None, None),
    ("HDR-CX760V", "HDR-CX760V", None, None),
    ("HDR-TD30V", "HDR-TD30V", None, None),
    ("HXR-NX5", "HXR-NX5", None, None),
    ("HXR-NX30", "HXR-NX30", None, None),
    ("HXR-NX70", "HXR-NX70", None, None),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn support_follows_the_lists() {
        assert_eq!(support("DSC-RX1RM2", Capability::Apps), Support::Yes);
        assert_eq!(support("dsc-rx1rm2", Capability::Updater), Support::Yes);
        assert_eq!(support("DSC-RX1R", Capability::Apps), Support::No);
        assert_eq!(support("ILCE-7M3", Capability::Updater), Support::No);
        assert_eq!(support("ILCE-7M4", Capability::Updater), Support::No);
        assert_eq!(support("ILCE-7RM3A", Capability::Apps), Support::No);
        assert_eq!(support("DSC-QX30", Capability::Apps), Support::Untested);
        assert_eq!(support("XYZ-1", Capability::Apps), Support::Unknown);
    }

    #[test]
    fn a_gps_suffix_finds_the_family() {
        assert_eq!(support("DSC-HX60V", Capability::Apps), Support::Yes);
        assert_eq!(support("DSC-HX60V", Capability::Gps), Support::Yes);
        assert_eq!(support("DSC-HX60", Capability::Gps), Support::No);
        assert_eq!(support("DSC-TX300V", Capability::Gps), Support::Untested);
        assert_eq!(support("XYZ-1", Capability::Gps), Support::Unknown);
    }

    #[test]
    fn capable_lists_keep_only_capable_models() {
        let apps = capable(Capability::Apps);
        assert!(apps.compat.iter().flat_map(|g| &g.models).all(|m| m.apps.is_some()));
        assert!(apps.more.iter().flat_map(|g| &g.models).all(|m| m.apps == Some(true)));
        let updater = capable(Capability::Updater);
        assert!(updater.compat.iter().flat_map(|g| &g.models).all(|m| m.updater));
        assert!(updater.more.is_empty());
    }

    #[test]
    fn every_list_is_filled() {
        let all = lists();
        let count = |groups: &[DeviceGroup<CompatModel>]| {
            groups.iter().map(|g| g.models.len()).sum::<usize>()
        };
        assert_eq!(count(&all.compat), 143);
        assert_eq!(all.more.iter().map(|g| g.models.len()).sum::<usize>(), 61);
        assert_eq!(all.gps.iter().map(|g| g.models.len()).sum::<usize>(), 26);
        assert_eq!(all.gps_untested.len(), 17);
    }
}
