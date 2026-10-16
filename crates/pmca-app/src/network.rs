//! Camera network settings: GPS assist data, the access points stored for
//! live streaming and the live-streaming configuration.
//!
//! Port of `gpsUpdateCommand`, `wifiCommand` and `streamingCommand` in
//! `pmca/commands/usb.py`. Everything here needs a camera in a command mode
//! (mass storage or MTP); the byte-level wire formats live in
//! [`pmca_usb::network`] and are unit-tested there without hardware.
//!
//! Despite the name, `wifiCommand` never touched the camera's own Wi-Fi. All
//! three functions live in the camera's `NetworkServiceInfo` plug-in, which
//! bodies without a live-streaming feature do not carry at all — they refuse
//! even its capability query. See `cargo run -p pmca-usb --example
//! probe_extcmd`.
//!
//! **These settings contain secrets.** Wi-Fi passphrases and the streaming
//! OAuth tokens are stored and returned by the camera in clear text, so a file
//! written by [`read_wifi`] or [`read_streaming`] must be treated accordingly.

use pmca_i18n::Localizer;
use pmca_usb::discovery::{self, CameraMode};
use pmca_usb::sony_extcmd::{GPS_DATA_SIZE, SonyExtCmdCamera};

use crate::dto::{
    ProgressEvent, ProgressKind, SocialSettings, StreamingService, StreamingSettings,
    WifiAccessPoint,
};
use crate::{Error, ProgressSink, Result};

/// Where Sony publishes the GPS assist data.
pub const GPS_ASSIST_URL: &str = "https://control.d-imaging.sony.co.jp/GPS/assistme.dat";

/// Opens the connected camera for external commands.
///
/// Only mass-storage and MTP mode expose the command set; in any other mode we
/// fail with a localized message instead of sending commands the camera cannot
/// answer.
fn open_camera(
    l: &Localizer,
) -> Result<SonyExtCmdCamera<Box<dyn pmca_usb::command::SonyExtCmdTransport>>> {
    let camera = discovery::find_single_camera()?;
    if !matches!(camera.mode, CameraMode::MassStorage | CameraMode::Mtp) {
        return Err(Error::WrongMode { message: l.tr("msg-switch-mode"), mode: camera.mode });
    }
    Ok(SonyExtCmdCamera::new(discovery::open_extcmd_transport(&camera)?))
}

/// Reads the access points the camera stores for live streaming.
///
/// These are the streaming plug-in's access points, not the camera's general
/// Wi-Fi configuration; a body without a streaming feature refuses the command
/// with [`pmca_usb::Error::Unsupported`].
///
/// `multi` selects the multi-access-point list instead of the single one.
pub fn read_wifi(l: &Localizer, multi: bool) -> Result<Vec<WifiAccessPoint>> {
    let mut cam = open_camera(l)?;
    let aps = if multi { cam.get_multi_wifi_ap_info()? } else { cam.get_wifi_ap_info()? };
    Ok(aps.into_iter().map(WifiAccessPoint::from).collect())
}

/// Writes the live-streaming access points, replacing the stored list.
pub fn write_wifi(l: &Localizer, multi: bool, aps: &[WifiAccessPoint]) -> Result<()> {
    let converted: Vec<_> = aps.iter().cloned().map(pmca_usb::network::ApInfo::from).collect();
    let mut cam = open_camera(l)?;
    if multi {
        cam.set_multi_wifi_ap_info(&converted)?;
    } else {
        cam.set_wifi_ap_info(&converted)?;
    }
    Ok(())
}

/// Reads the camera's live-streaming configuration.
pub fn read_streaming(l: &Localizer) -> Result<StreamingSettings> {
    let mut cam = open_camera(l)?;
    let social = SocialSettings::from(cam.get_live_streaming_social_info()?);
    let services =
        cam.get_live_streaming_service_info()?.into_iter().map(StreamingService::from).collect();
    Ok(StreamingSettings { social, services })
}

/// Writes the camera's live-streaming configuration.
///
/// The social block and the service list are two separate camera commands. The
/// social block is written first because it is the smaller, lower-risk write; if
/// the service list then fails, the camera is left in a state the caller can
/// diagnose by reading the settings back.
pub fn write_streaming(l: &Localizer, settings: &StreamingSettings) -> Result<()> {
    let social = pmca_usb::network::SocialInfo::from(settings.social.clone());
    let services: Vec<_> =
        settings.services.iter().cloned().map(pmca_usb::network::StreamingService::from).collect();

    let mut cam = open_camera(l)?;
    cam.set_live_streaming_social_info(&social)?;
    cam.set_live_streaming_service_info(&services)?;
    Ok(())
}

/// Uploads GPS assist data to the camera.
///
/// When `data` is `None` the current file is downloaded from Sony over HTTPS.
/// The download happens before the camera is opened so a network failure cannot
/// interrupt a transfer that is already under way.
pub fn update_gps(l: &Localizer, data: Option<&[u8]>, sink: &mut dyn ProgressSink) -> Result<()> {
    let downloaded = match data {
        Some(_) => None,
        None => {
            sink.emit(ProgressEvent::step(ProgressKind::Info, l.tr("gps-downloading")));
            Some(pmca_market::appstore::download_asset(GPS_ASSIST_URL)?)
        }
    };
    let data = data.or(downloaded.as_deref()).unwrap_or_default();
    if data.len() != GPS_DATA_SIZE as usize {
        return Err(Error::Operation(l.tr("gps-bad-size")));
    }

    let mut cam = open_camera(l)?;
    sink.emit(ProgressEvent::step(ProgressKind::Info, l.tr("gps-writing")));
    cam.write_gps_data(data)?;
    sink.emit(ProgressEvent::step(ProgressKind::Done, l.tr("msg-done")));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wifi_access_points_convert_both_ways() {
        let dto = WifiAccessPoint { key_type: 4, ssid: "net".into(), key: "pass".into() };
        let wire = pmca_usb::network::ApInfo::from(dto.clone());
        assert_eq!(wire.ssid, "net");
        assert_eq!(WifiAccessPoint::from(wire), dto);
    }

    #[test]
    fn streaming_settings_survive_a_json_round_trip() {
        let settings = StreamingSettings {
            social: SocialSettings { twitter_enabled: true, ..SocialSettings::default() },
            services: vec![StreamingService::from(pmca_usb::network::StreamingService::default())],
        };
        let json = serde_json::to_string(&settings).unwrap();
        assert!(json.contains("twitterEnabled"), "fields must be camelCase: {json}");
        assert!(json.contains("macIssueTime"), "fields must be camelCase: {json}");
        assert_eq!(serde_json::from_str::<StreamingSettings>(&json).unwrap(), settings);
    }

    #[test]
    fn the_gps_url_is_https() {
        assert!(GPS_ASSIST_URL.starts_with("https://"));
    }
}
