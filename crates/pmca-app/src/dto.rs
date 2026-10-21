//! Serializable data-transfer objects shared by the CLI and the GUI.
//!
//! These types deliberately contain no `pmca-usb` / `pmca-market` types so the
//! GUI layer can hand them to the front-end (as JSON) without leaking transport
//! details, and so the shape of the UI contract is reviewable in one place.
//!
//! All structs serialize as `camelCase` because the primary consumer is the
//! TypeScript front-end.

use serde::{Deserialize, Serialize};

/// Identifying details of a connected camera.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct CameraSummary {
    /// Detected operating mode as a stable id (e.g. `mass-storage`, `mtp`).
    pub mode: String,
    /// Human-readable model name, if the camera reported one.
    pub model_name: Option<String>,
    /// Hex-encoded model code.
    pub model_code: Option<String>,
    /// Hex-encoded serial number.
    pub serial: Option<String>,
    /// Firmware version currently installed.
    pub firmware: Option<String>,
    /// Mounted-lens description, if a lens is present.
    pub lens: Option<String>,
}

impl From<pmca_usb::discovery::CameraInfoReport> for CameraSummary {
    fn from(r: pmca_usb::discovery::CameraInfoReport) -> Self {
        Self {
            mode: r.mode,
            model_name: r.model_name,
            model_code: r.model_code,
            serial: r.serial,
            firmware: r.firmware,
            lens: r.lens,
        }
    }
}

/// What a photo says about the camera that took it.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct PhotoSummary {
    /// File name without its directory.
    pub file_name: String,
    /// Camera model (e.g. `ILCE-5100`).
    pub model: Option<String>,
    /// Firmware that wrote the file (e.g. `ILCE-5100 v3.10`).
    pub software: Option<String>,
    /// Lens model, if the lens reported one.
    pub lens: Option<String>,
    /// Capture time, `YYYY-MM-DD hh:mm:ss` plus the UTC offset if recorded.
    pub taken: Option<String>,
    /// The shutter count stored in the file.
    pub shutter_count: Option<u32>,
    /// Why there is no count: `not-sony`, `unknown-model` or `not-recorded`.
    pub count_unavailable: Option<String>,
    /// `mechanical` or `electronic`, where the file records it.
    pub shutter: Option<String>,
}

impl PhotoSummary {
    /// Convert the parser's result, naming the file it came from.
    pub fn new(file_name: String, info: pmca_core::photo::PhotoInfo) -> Self {
        use pmca_core::photo::{Shutter, ShutterCount};
        let (shutter_count, unavailable) = match info.shutter_count {
            ShutterCount::Counted(n) => (Some(n), None),
            ShutterCount::NotSony => (None, Some("not-sony")),
            ShutterCount::UnknownModel => (None, Some("unknown-model")),
            ShutterCount::NotRecorded => (None, Some("not-recorded")),
        };
        Self {
            file_name,
            model: info.model,
            software: info.software,
            lens: info.lens,
            taken: info.taken,
            shutter_count,
            count_unavailable: unavailable.map(str::to_string),
            shutter: info.shutter.map(|s| {
                match s {
                    Shutter::Mechanical => "mechanical",
                    Shutter::Electronic => "electronic",
                }
                .to_string()
            }),
        }
    }
}

/// The device identity a camera posts to the local market server during an
/// application installation.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct InstalledDevice {
    /// Model name as reported by the camera.
    pub name: String,
    /// Product code.
    pub product_code: String,
    /// Device / serial id.
    pub device_id: String,
    /// Firmware version.
    pub firmware_version: String,
}

/// Result of a completed application installation.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct InstallOutcome {
    /// Device details captured by the market server, if the camera sent them.
    pub device: Option<InstalledDevice>,
    /// Metadata of the installed APK, when one was supplied and readable.
    pub apk: Option<ApkSummary>,
    /// The camera's raw completion payload, for diagnostics.
    #[cfg_attr(test, ts(type = "unknown"))]
    pub raw: Option<serde_json::Value>,
}

/// What a firmware image did — or would do — to the attached camera.
///
/// The version pair is reported even when nothing was written, so a front-end
/// can show the user exactly which transition it is about to authorise before
/// asking for confirmation.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct FirmwareOutcome {
    /// Firmware version currently installed on the camera.
    pub current_version: String,
    /// Firmware version contained in the supplied image.
    pub new_version: String,
    /// Whether the payload was actually written.
    pub flashed: bool,
}

/// An application offered by the community app store (`OpenMemories-AppList`).
///
/// The newest build is *not* resolved while listing: every entry would cost one
/// GitHub API request, and the unauthenticated rate limit is low. Use
/// [`crate::store::download_community_app`] to resolve and fetch one app.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct CommunityApp {
    /// Android package name; the identifier used to install the app.
    pub package: String,
    /// Display name.
    pub name: String,
    /// Author, as published.
    pub author: Option<String>,
    /// Short description.
    pub description: Option<String>,
    /// Project homepage.
    pub homepage: Option<String>,
    /// Whether the entry declares a usable release source.
    pub installable: bool,
}

impl From<pmca_market::appstore::StoreApp> for CommunityApp {
    fn from(a: pmca_market::appstore::StoreApp) -> Self {
        let installable = !matches!(a.release, pmca_market::appstore::ReleaseSource::None);
        Self {
            package: a.package,
            name: a.name,
            author: a.author,
            description: a.desc,
            homepage: a.homepage,
            installable,
        }
    }
}

/// Metadata read out of the APK that is about to be installed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct ApkSummary {
    /// Android package name.
    pub package: String,
    /// `android:versionName`, if declared.
    pub version: Option<String>,
    /// Declared minimum SDK level, if any.
    pub min_sdk_version: Option<u32>,
    /// Whether the archive carries a v1 (JAR) signature.
    pub signed: bool,
    /// Whether the declared minimum SDK level fits the cameras' Android build.
    pub compatible: bool,
}

/// Highest Android SDK level the PMCA cameras provide.
const MAX_CAMERA_SDK: u32 = 10;

impl From<pmca_apk::ApkInfo> for ApkSummary {
    fn from(a: pmca_apk::ApkInfo) -> Self {
        // An APK that declares no minimum is treated as compatible, matching
        // the reference implementation, which only warned on a declared value.
        let compatible = a.min_sdk_version.is_none_or(|sdk| sdk <= MAX_CAMERA_SDK);
        Self {
            package: a.package,
            version: a.version_name,
            min_sdk_version: a.min_sdk_version,
            signed: a.signed,
            compatible,
        }
    }
}

/// A progress or status notification emitted during a long-running operation.
///
/// The `kind` field is a stable machine-readable discriminator; `message` is an
/// already-localized string and `percent` is present only for measurable steps.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct ProgressEvent {
    /// Stable identifier of the step (see [`ProgressKind`]).
    pub kind: ProgressKind,
    /// Localized, user-facing description.
    pub message: String,
    /// Completion percentage, when the step reports one.
    pub percent: Option<u8>,
}

impl ProgressEvent {
    /// A step notification without a percentage.
    pub fn step(kind: ProgressKind, message: impl Into<String>) -> Self {
        Self { kind, message: message.into(), percent: None }
    }

    /// A measurable progress notification.
    pub fn progress(message: impl Into<String>, percent: u8) -> Self {
        Self {
            kind: ProgressKind::Progress,
            message: message.into(),
            percent: Some(percent.min(100)),
        }
    }
}

/// Machine-readable classification of a [`ProgressEvent`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub enum ProgressKind {
    /// The camera is being switched into another USB mode.
    Switching,
    /// Waiting for the camera to re-enumerate.
    Waiting,
    /// Authenticating against the camera.
    Authenticating,
    /// A long-running transfer is running.
    Progress,
    /// A milestone was reached.
    Info,
    /// Something the user has to act on, which outlives the events after it.
    ///
    /// Distinct from [`ProgressKind::Info`] because a front-end that shows one
    /// event at a time would otherwise drop it: these are emitted as an
    /// operation winds down, milliseconds before its closing event.
    Notice,
    /// The operation finished successfully.
    Done,
}

/// The two named alternatives of a tweak that is a choice rather than a switch.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct TweakChoice {
    /// Fluent message id naming the state stored as "off".
    pub off_key: String,
    /// Fluent message id naming the state stored as "on".
    pub on_key: String,
}

/// One tweak offered by the connected camera.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct TweakState {
    /// Stable tweak identifier, used when requesting a change.
    pub id: String,
    /// Fluent message id describing the tweak to the user.
    pub label_key: String,
    /// Whether the camera currently has the tweaked value.
    pub enabled: bool,
    /// Human-readable rendering of the camera's current value.
    pub value: String,
    /// Names of the two alternatives when this tweak is a choice.
    ///
    /// `None` for an ordinary switch, which is the overwhelming majority, so a
    /// front-end that renders a checkbox by default stays correct.
    pub choice: Option<TweakChoice>,
    /// Fluent message id of something the user should know before changing
    /// this tweak, or `None` where there is nothing to say.
    ///
    /// Distinct from [`TweakState::caution_key`]: a note has nothing to accept.
    /// It describes a consequence outside the toolkit's reach — the camera
    /// wanting the memory card reformatted, say — which happens the same way
    /// whatever the user decides, so it is stated rather than asked about, and
    /// stated before the change rather than after it.
    pub note_key: Option<String>,
    /// Fluent message id of a warning the user must read before enabling this
    /// tweak, or `None` where enabling it needs no decision.
    ///
    /// Set per camera, not per tweak: the same tweak is unremarkable on most
    /// models and needs an explicit choice on a few.
    pub caution_key: Option<String>,
}

impl From<pmca_platform::tweaks::TweakStatus> for TweakState {
    fn from(t: pmca_platform::tweaks::TweakStatus) -> Self {
        // The Fluent ids predate the tweak ids and are kebab-cased, so the
        // mapping is explicit rather than derived.
        let label_key = match t.id.as_str() {
            "recLimit" => "tweak-rec-limit",
            "recLimit4k" => "tweak-rec-limit-4k",
            "language" => "tweak-language",
            "palNtscSelector" => "tweak-pal-ntsc",
            "palNtsc" => "tweak-pal-ntsc-mode",
            "usbAppInstaller" => "tweak-usb-installer",
            other => other,
        }
        .to_string();
        let choice = match t.presentation {
            pmca_platform::tweaks::Presentation::Toggle => None,
            // The names are proper nouns and read the same in most languages,
            // but they still go through the catalogue: a script that does not
            // use the Latin alphabet may well set them differently.
            pmca_platform::tweaks::Presentation::Choice { .. } => Some(TweakChoice {
                off_key: "tweak-pal-ntsc-pal".to_string(),
                on_key: "tweak-pal-ntsc-ntsc".to_string(),
            }),
        };
        // Attached to the tweak rather than to the camera, unlike a caution:
        // this is a property of what the setting does, not of what this
        // particular body supports. `palNtscSelector` deliberately has none —
        // it only reveals a menu entry and writes no video system, so the
        // camera never asks for the card there.
        let note_key = match t.id.as_str() {
            "palNtsc" => Some("tweak-pal-ntsc-note".to_string()),
            "language" => Some("tweak-language-note".to_string()),
            _ => None,
        };
        let caution_key = t.caution.map(|c| {
            match c {
                pmca_platform::tweaks::Caution::UnsupportedLanguages => "tweaks-caution-languages",
            }
            .to_string()
        });
        Self {
            id: t.id,
            label_key,
            enabled: t.enabled,
            value: t.value,
            choice,
            note_key,
            caution_key,
        }
    }
}

/// A requested change to one tweak.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct TweakChange {
    /// Identifier from [`TweakState::id`].
    pub id: String,
    /// Desired state.
    pub enabled: bool,
    /// Whether the user has explicitly accepted the warning named by
    /// [`TweakState::caution_key`].
    ///
    /// Defaults to "not accepted" so that a caller unaware of the field — an
    /// older front-end, a hand-written request — gets the cautious value
    /// rather than silently consenting on the user's behalf.
    #[serde(default)]
    pub accept_caution: bool,
}

/// One read-only identifying value from the camera's backup memory.
///
/// These are reported alongside the tweaks because they come out of the same
/// backup image, at no extra cost to the camera. They are **not** offered as
/// something to change: see [`pmca_platform::identity`] for why.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct DeviceProperty {
    /// Stable identifier of the value.
    pub id: String,
    /// Fluent message id naming the value to the user.
    pub label_key: String,
    /// Human-readable rendering of the camera's current value.
    pub value: String,
}

impl From<pmca_platform::identity::IdentityValue> for DeviceProperty {
    fn from(v: pmca_platform::identity::IdentityValue) -> Self {
        // The first five reuse the info view's Fluent ids, so the same camera
        // reads the same way wherever it is shown.
        let label_key = match v.id {
            "modelName" => "info-model",
            "modelCode" => "info-product-code",
            "serialNumber" => "info-serial",
            "backupRegion" => "info-region",
            "androidPlatformVersion" => "info-android-version",
            "photographer" => "info-photographer",
            "copyright" => "info-copyright",
            "photographer1" => "info-photographer-1",
            "photographer2" => "info-photographer-2",
            "copyright1" => "info-copyright-1",
            "copyright2" => "info-copyright-2",
            other => other,
        }
        .to_string();
        Self { id: v.id.to_string(), label_key, value: v.value }
    }
}

/// A running Windows service that can keep a camera from being reached.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct InterferingService {
    /// Key name, as `net stop` expects it.
    pub name: String,
    /// The name Windows shows, in the system's language.
    pub display_name: String,
}

impl From<pmca_usb::host_services::RunningService> for InterferingService {
    fn from(s: pmca_usb::host_services::RunningService) -> Self {
        Self { name: s.name, display_name: s.display_name }
    }
}

/// What a tweak read or apply reports back.
///
/// Both operations download the camera's backup image anyway, so the
/// identifying values are free to include and the caller never has to open a
/// second service-mode session just to display them.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct TweakReport {
    /// Read-only identifying values recovered from the backup image.
    pub properties: Vec<DeviceProperty>,
    /// The tweaks this camera supports and their current state.
    pub tweaks: Vec<TweakState>,
    /// Settings are read and written one property at a time, because this
    /// camera's backup image exceeds 1 MB or [`Self::direct_forced`].
    pub direct: bool,
    /// The per-property path was forced by `PMCA_TWEAKS_DIRECT` on a camera
    /// that would not need it.
    pub direct_forced: bool,
    /// Values from before an earlier per-property change are saved and can
    /// be written back.
    pub can_undo: bool,
}

/// A Wi-Fi access point stored in the camera for live streaming.
///
/// Note that [`key`](Self::key) is the network passphrase in clear text — the
/// camera stores and returns it unencrypted, so a settings file written from
/// here contains a secret.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct WifiAccessPoint {
    /// Encryption type as understood by the camera (`0` = open).
    pub key_type: u8,
    /// Network name.
    pub ssid: String,
    /// Passphrase.
    pub key: String,
}

impl From<pmca_usb::network::ApInfo> for WifiAccessPoint {
    fn from(ap: pmca_usb::network::ApInfo) -> Self {
        Self { key_type: ap.key_type, ssid: ap.ssid, key: ap.key }
    }
}

impl From<WifiAccessPoint> for pmca_usb::network::ApInfo {
    fn from(ap: WifiAccessPoint) -> Self {
        Self { key_type: ap.key_type, ssid: ap.ssid, key: ap.key }
    }
}

/// The camera's social-media configuration for live streaming.
///
/// Every token field is a credential; treat a serialized copy as secret.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct SocialSettings {
    /// Whether the camera posts to Twitter.
    pub twitter_enabled: bool,
    /// Twitter OAuth consumer key.
    pub twitter_consumer_key: String,
    /// Twitter OAuth consumer secret.
    pub twitter_consumer_secret: String,
    /// Twitter OAuth access token.
    pub twitter_access_token: String,
    /// Twitter OAuth access-token secret.
    pub twitter_access_token_secret: String,
    /// Message posted to Twitter when a stream starts.
    pub twitter_message: String,
    /// Whether the camera posts to Facebook.
    pub facebook_enabled: bool,
    /// Facebook access token.
    pub facebook_access_token: String,
    /// Message posted to Facebook when a stream starts.
    pub facebook_message: String,
}

impl From<pmca_usb::network::SocialInfo> for SocialSettings {
    fn from(i: pmca_usb::network::SocialInfo) -> Self {
        Self {
            twitter_enabled: i.twitter_enabled,
            twitter_consumer_key: i.twitter_consumer_key,
            twitter_consumer_secret: i.twitter_consumer_secret,
            twitter_access_token: i.twitter_access_token,
            twitter_access_token_secret: i.twitter_access_token_secret,
            twitter_message: i.twitter_message,
            facebook_enabled: i.facebook_enabled,
            facebook_access_token: i.facebook_access_token,
            facebook_message: i.facebook_message,
        }
    }
}

impl From<SocialSettings> for pmca_usb::network::SocialInfo {
    fn from(i: SocialSettings) -> Self {
        Self {
            twitter_enabled: i.twitter_enabled,
            twitter_consumer_key: i.twitter_consumer_key,
            twitter_consumer_secret: i.twitter_consumer_secret,
            twitter_access_token: i.twitter_access_token,
            twitter_access_token_secret: i.twitter_access_token_secret,
            twitter_message: i.twitter_message,
            facebook_enabled: i.facebook_enabled,
            facebook_access_token: i.facebook_access_token,
            facebook_message: i.facebook_message,
        }
    }
}

/// One live-streaming service configured on the camera.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct StreamingService {
    /// Service id as understood by the camera.
    pub service: u32,
    /// Whether this service is active.
    pub enabled: bool,
    /// Streaming member id.
    pub mac_id: String,
    /// Streaming member secret.
    pub mac_secret: String,
    /// Hex-encoded issue timestamp of the member id (8 bytes / 16 digits).
    pub mac_issue_time: String,
    /// Channel ids offered by the service.
    pub channels: Vec<u32>,
    /// Short URL of the stream.
    pub short_url: String,
    /// Selected video format id.
    pub video_format: u32,
    /// Video format ids the service supports.
    pub supported_formats: Vec<u32>,
    /// Whether the camera also records locally while streaming.
    pub enable_record_mode: bool,
    /// Title of the published video.
    pub video_title: String,
    /// Description of the published video.
    pub video_description: String,
    /// Tags of the published video.
    pub video_tag: String,
}

impl From<pmca_usb::network::StreamingService> for StreamingService {
    fn from(s: pmca_usb::network::StreamingService) -> Self {
        Self {
            service: s.service,
            enabled: s.enabled,
            mac_id: s.mac_id,
            mac_secret: s.mac_secret,
            mac_issue_time: s.mac_issue_time,
            channels: s.channels,
            short_url: s.short_url,
            video_format: s.video_format,
            supported_formats: s.supported_formats,
            enable_record_mode: s.enable_record_mode,
            video_title: s.video_title,
            video_description: s.video_description,
            video_tag: s.video_tag,
        }
    }
}

impl From<StreamingService> for pmca_usb::network::StreamingService {
    fn from(s: StreamingService) -> Self {
        Self {
            service: s.service,
            enabled: s.enabled,
            mac_id: s.mac_id,
            mac_secret: s.mac_secret,
            mac_issue_time: s.mac_issue_time,
            channels: s.channels,
            short_url: s.short_url,
            video_format: s.video_format,
            supported_formats: s.supported_formats,
            enable_record_mode: s.enable_record_mode,
            video_title: s.video_title,
            video_description: s.video_description,
            video_tag: s.video_tag,
        }
    }
}

/// The camera's complete live-streaming configuration.
///
/// This is the document read and written by `pmca-console stream`. The original
/// Python tool used a flat list of `[key, value]` pairs; a nested object is used
/// here so the two independent camera commands behind it stay distinguishable.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct StreamingSettings {
    /// Social-media configuration.
    pub social: SocialSettings,
    /// Configured streaming services.
    pub services: Vec<StreamingService>,
}

/// A published release newer than the running build.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct AvailableUpdate {
    /// Version of the running build.
    pub current: String,
    /// Version of the newest release, without the tag's `v`.
    pub latest: String,
    /// Page the release is downloaded from.
    pub url: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn progress_event_clamps_percentage() {
        assert_eq!(ProgressEvent::progress("x", 250).percent, Some(100));
        assert_eq!(ProgressEvent::progress("x", 42).percent, Some(42));
    }

    #[test]
    fn progress_kind_serializes_as_kebab_case() {
        let json = serde_json::to_string(&ProgressEvent::step(ProgressKind::Authenticating, "hi"))
            .unwrap();
        assert!(json.contains(r#""kind":"authenticating""#), "{json}");
        assert!(json.contains(r#""percent":null"#), "{json}");
    }

    #[test]
    fn camera_summary_round_trips_through_json() {
        let s = CameraSummary {
            mode: "mtp".into(),
            model_name: Some("ILCE-5100".into()),
            ..Default::default()
        };
        let json = serde_json::to_string(&s).unwrap();
        assert!(json.contains(r#""modelName":"ILCE-5100""#), "{json}");
        assert_eq!(serde_json::from_str::<CameraSummary>(&json).unwrap(), s);
    }
}
