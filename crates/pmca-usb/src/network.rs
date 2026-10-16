//! Wire formats for the camera's network-service settings.
//!
//! Port of the `APInfo`, `LiveStreamingSNSInfo` and `LiveStreamingServiceInfo*`
//! structures in `pmca/usb/sony.py`, plus the list framing that
//! `pmca/commands/usb.py` assembled inline.
//!
//! This module is deliberately **transport-free**: it only turns byte buffers
//! into values and back, so the riskiest part of the feature — writing
//! credentials into fixed-size slots in the camera's flash — is fully
//! unit-testable without hardware.
//!
//! Two rules apply throughout:
//!
//! * Text fields are fixed-size, NUL-padded and Latin-1. Packing a value that
//!   does not fit is an **error**, never a silent truncation: a half-written
//!   Wi-Fi key or access token would be stored on the camera as-is.
//! * Every read is bounds-checked, so a short or hostile response yields
//!   `Err` rather than a panic.

use crate::{Error, Result};

/// Size of one access-point record on the wire.
pub const AP_INFO_SIZE: usize = 100;
/// Size of the social-media configuration block on the wire.
pub const SNS_INFO_SIZE: usize = 5929;

/// Capacity of the SSID field, including the trailing NUL.
pub const SSID_CAPACITY: usize = 33;
/// Capacity of the passphrase field, including the trailing NUL.
pub const KEY_CAPACITY: usize = 65;
/// Capacity of an OAuth token field, including the trailing NUL.
pub const TOKEN_CAPACITY: usize = 1025;
/// Capacity of a status-message field, including the trailing NUL.
pub const MESSAGE_CAPACITY: usize = 401;
/// Capacity of the streaming member-id fields, including the trailing NUL.
pub const MAC_CAPACITY: usize = 41;
/// Length of the streaming member-id issue timestamp (raw bytes, not text).
pub const MAC_ISSUE_TIME_SIZE: usize = 8;
/// Capacity of the short-URL field, including the trailing NUL.
pub const SHORT_URL_CAPACITY: usize = 101;

/// Upper bound on the number of records accepted in one list.
///
/// The camera returns a 32-bit count ahead of the records. Without a cap, a
/// corrupt count would make us allocate before the length check catches it.
const MAX_RECORDS: usize = 1024;

/// A Wi-Fi access point stored in the camera for live streaming.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ApInfo {
    /// Encryption type as understood by the camera (`0` = open).
    pub key_type: u8,
    /// Network name.
    pub ssid: String,
    /// Passphrase.
    pub key: String,
}

/// The social-media configuration used when a stream starts.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SocialInfo {
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
    /// Message posted to Twitter.
    pub twitter_message: String,
    /// Whether the camera posts to Facebook.
    pub facebook_enabled: bool,
    /// Facebook access token.
    pub facebook_access_token: String,
    /// Message posted to Facebook.
    pub facebook_message: String,
}

/// One configured live-streaming service.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StreamingService {
    /// Service id as understood by the camera.
    pub service: u32,
    /// Whether this service is active.
    pub enabled: bool,
    /// Streaming member id.
    pub mac_id: String,
    /// Streaming member secret.
    pub mac_secret: String,
    /// Hex-encoded issue timestamp of the member id.
    pub mac_issue_time: String,
    /// Channel ids offered by the service.
    pub channels: Vec<u32>,
    /// Short URL of the stream.
    pub short_url: String,
    /// Selected video format id.
    pub video_format: u32,
    /// Video format ids the service supports.
    pub supported_formats: Vec<u32>,
    /// Whether the camera also records while streaming.
    pub enable_record_mode: bool,
    /// Title of the published video.
    pub video_title: String,
    /// Description of the published video.
    pub video_description: String,
    /// Tags of the published video.
    pub video_tag: String,
}

impl Default for StreamingService {
    /// An empty service whose timestamp is all zeroes.
    ///
    /// [`mac_issue_time`](Self::mac_issue_time) is hex text of a fixed 8-byte
    /// field, so an empty string would not be serializable; spelling the zero
    /// value out here keeps `Default` usable as a starting point.
    fn default() -> Self {
        Self {
            service: 0,
            enabled: false,
            mac_id: String::new(),
            mac_secret: String::new(),
            mac_issue_time: "0".repeat(MAC_ISSUE_TIME_SIZE * 2),
            channels: Vec::new(),
            short_url: String::new(),
            video_format: 0,
            supported_formats: Vec::new(),
            enable_record_mode: false,
            video_title: String::new(),
            video_description: String::new(),
            video_tag: String::new(),
        }
    }
}

// ---------------------------------------------------------------------------
// Access points
// ---------------------------------------------------------------------------

/// Parses the camera's access-point list (`count` followed by the records).
pub fn parse_ap_infos(data: &[u8]) -> Result<Vec<ApInfo>> {
    let mut c = Reader::new(data);
    let count = c.read_count(AP_INFO_SIZE)?;
    (0..count)
        .map(|_| {
            let record = c.take(AP_INFO_SIZE)?;
            Ok(ApInfo {
                key_type: record[0],
                ssid: unpack_str(&record[1..1 + SSID_CAPACITY]),
                // One padding byte sits between the SSID and the key.
                key: unpack_str(&record[1 + SSID_CAPACITY + 1..]),
            })
        })
        .collect()
}

/// Serializes an access-point list for the camera.
pub fn pack_ap_infos(aps: &[ApInfo]) -> Result<Vec<u8>> {
    let mut out = Vec::with_capacity(4 + aps.len() * AP_INFO_SIZE);
    out.extend_from_slice(&(aps.len() as u32).to_le_bytes());
    for ap in aps {
        out.push(ap.key_type);
        pack_str(&mut out, &ap.ssid, SSID_CAPACITY)?;
        out.push(0); // padding
        pack_str(&mut out, &ap.key, KEY_CAPACITY)?;
    }
    Ok(out)
}

// ---------------------------------------------------------------------------
// Social media
// ---------------------------------------------------------------------------

/// Parses the social-media configuration block.
pub fn parse_social_info(data: &[u8]) -> Result<SocialInfo> {
    let mut c = Reader::new(data);
    Ok(SocialInfo {
        twitter_enabled: c.read_u8()? != 0,
        twitter_consumer_key: c.read_str(TOKEN_CAPACITY)?,
        twitter_consumer_secret: c.read_str(TOKEN_CAPACITY)?,
        twitter_access_token: c.read_str(TOKEN_CAPACITY)?,
        twitter_access_token_secret: c.read_str(TOKEN_CAPACITY)?,
        twitter_message: c.read_str(MESSAGE_CAPACITY)?,
        facebook_enabled: c.read_u8()? != 0,
        facebook_access_token: c.read_str(TOKEN_CAPACITY)?,
        facebook_message: c.read_str(MESSAGE_CAPACITY)?,
    })
}

/// Serializes the social-media configuration block.
pub fn pack_social_info(info: &SocialInfo) -> Result<Vec<u8>> {
    let mut out = Vec::with_capacity(SNS_INFO_SIZE);
    out.push(u8::from(info.twitter_enabled));
    pack_str(&mut out, &info.twitter_consumer_key, TOKEN_CAPACITY)?;
    pack_str(&mut out, &info.twitter_consumer_secret, TOKEN_CAPACITY)?;
    pack_str(&mut out, &info.twitter_access_token, TOKEN_CAPACITY)?;
    pack_str(&mut out, &info.twitter_access_token_secret, TOKEN_CAPACITY)?;
    pack_str(&mut out, &info.twitter_message, MESSAGE_CAPACITY)?;
    out.push(u8::from(info.facebook_enabled));
    pack_str(&mut out, &info.facebook_access_token, TOKEN_CAPACITY)?;
    pack_str(&mut out, &info.facebook_message, MESSAGE_CAPACITY)?;
    debug_assert_eq!(out.len(), SNS_INFO_SIZE);
    Ok(out)
}

// ---------------------------------------------------------------------------
// Streaming services
// ---------------------------------------------------------------------------

/// Parses the live-streaming service list.
///
/// The payload starts with a version word, then a record count, then a
/// variable-length record per service.
pub fn parse_streaming_services(data: &[u8]) -> Result<Vec<StreamingService>> {
    let mut c = Reader::new(data);
    c.take(4)?; // leading version word, always 1
    let count = c.read_count(1)?;
    (0..count)
        .map(|_| {
            let service = c.read_u32le()?;
            let enabled = c.read_u8()? != 0;
            let mac_id = c.read_str(MAC_CAPACITY)?;
            let mac_secret = c.read_str(MAC_CAPACITY)?;
            let mac_issue_time = hex(c.take(MAC_ISSUE_TIME_SIZE)?);
            let _unknown = c.read_u32le()?;
            let channels = c.read_u32_list()?;
            let short_url = c.read_str(SHORT_URL_CAPACITY)?;
            let video_format = c.read_u32le()?;
            let supported_formats = c.read_u32_list()?;
            let enable_record_mode = c.read_u8()? != 0;
            let video_title = c.read_str(MESSAGE_CAPACITY)?;
            let video_description = c.read_str(MESSAGE_CAPACITY)?;
            let video_tag = c.read_str(MESSAGE_CAPACITY)?;
            Ok(StreamingService {
                service,
                enabled,
                mac_id,
                mac_secret,
                mac_issue_time,
                channels,
                short_url,
                video_format,
                supported_formats,
                enable_record_mode,
                video_title,
                video_description,
                video_tag,
            })
        })
        .collect()
}

/// Serializes the live-streaming service list.
///
/// The `unknown` word is written as zero, matching the reference implementation
/// which deliberately did not echo back the value it read.
pub fn pack_streaming_services(services: &[StreamingService]) -> Result<Vec<u8>> {
    let mut out = Vec::new();
    out.extend_from_slice(&1u32.to_le_bytes());
    out.extend_from_slice(&(services.len() as u32).to_le_bytes());
    for s in services {
        out.extend_from_slice(&s.service.to_le_bytes());
        out.push(u8::from(s.enabled));
        pack_str(&mut out, &s.mac_id, MAC_CAPACITY)?;
        pack_str(&mut out, &s.mac_secret, MAC_CAPACITY)?;
        let issue_time = unhex(&s.mac_issue_time)?;
        if issue_time.len() != MAC_ISSUE_TIME_SIZE {
            return Err(Error::Protocol("macIssueTime must be 8 bytes (16 hex digits)"));
        }
        out.extend_from_slice(&issue_time);
        out.extend_from_slice(&0u32.to_le_bytes()); // unknown
        pack_u32_list(&mut out, &s.channels);
        pack_str(&mut out, &s.short_url, SHORT_URL_CAPACITY)?;
        out.extend_from_slice(&s.video_format.to_le_bytes());
        pack_u32_list(&mut out, &s.supported_formats);
        out.push(u8::from(s.enable_record_mode));
        pack_str(&mut out, &s.video_title, MESSAGE_CAPACITY)?;
        pack_str(&mut out, &s.video_description, MESSAGE_CAPACITY)?;
        pack_str(&mut out, &s.video_tag, MESSAGE_CAPACITY)?;
    }
    Ok(out)
}

// ---------------------------------------------------------------------------
// Field helpers
// ---------------------------------------------------------------------------

/// Reads a fixed-size NUL-padded Latin-1 field, stopping at the first NUL.
fn unpack_str(field: &[u8]) -> String {
    let end = field.iter().position(|&b| b == 0).unwrap_or(field.len());
    field[..end].iter().map(|&b| b as char).collect()
}

/// Appends a fixed-size NUL-padded Latin-1 field.
///
/// `capacity` includes the terminating NUL, so the value itself may occupy at
/// most `capacity - 1` bytes. Both an oversized value and a non-Latin-1
/// character are rejected rather than mangled: a silently truncated Wi-Fi key
/// or access token would be written to the camera's flash as-is.
fn pack_str(out: &mut Vec<u8>, value: &str, capacity: usize) -> Result<()> {
    let start = out.len();
    for ch in value.chars() {
        let code = ch as u32;
        if code == 0 || code > 0xff {
            out.truncate(start);
            return Err(Error::Protocol("field contains a NUL or non-Latin-1 character"));
        }
        out.push(code as u8);
    }
    if out.len() - start >= capacity {
        out.truncate(start);
        return Err(Error::Protocol("field is too long for the camera's fixed-size slot"));
    }
    out.resize(start + capacity, 0);
    Ok(())
}

/// Appends a length-prefixed list of 32-bit values.
fn pack_u32_list(out: &mut Vec<u8>, values: &[u32]) {
    out.extend_from_slice(&(values.len() as u32).to_le_bytes());
    for v in values {
        out.extend_from_slice(&v.to_le_bytes());
    }
}

/// Hex-encodes bytes (lowercase).
fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// Decodes a lowercase or uppercase hex string.
fn unhex(text: &str) -> Result<Vec<u8>> {
    if !text.len().is_multiple_of(2) {
        return Err(Error::Protocol("hex string has an odd number of digits"));
    }
    text.as_bytes()
        .chunks(2)
        .map(|pair| {
            let s = std::str::from_utf8(pair).map_err(|_| Error::Protocol("invalid hex string"))?;
            u8::from_str_radix(s, 16).map_err(|_| Error::Protocol("invalid hex string"))
        })
        .collect()
}

/// Minimal bounds-checked forward reader.
struct Reader<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }

    fn take(&mut self, len: usize) -> Result<&'a [u8]> {
        let end = self.pos.checked_add(len).ok_or(Error::Protocol("read overflow"))?;
        let slice =
            self.data.get(self.pos..end).ok_or(Error::Protocol("short network response"))?;
        self.pos = end;
        Ok(slice)
    }

    fn read_u8(&mut self) -> Result<u8> {
        Ok(self.take(1)?[0])
    }

    fn read_u32le(&mut self) -> Result<u32> {
        Ok(u32::from_le_bytes(self.take(4)?.try_into().unwrap()))
    }

    fn read_str(&mut self, capacity: usize) -> Result<String> {
        Ok(unpack_str(self.take(capacity)?))
    }

    /// Reads a record count and rejects one the remaining data cannot back.
    ///
    /// `record_size` is the minimum size of a single record, so a corrupt count
    /// is caught before any allocation.
    fn read_count(&mut self, record_size: usize) -> Result<usize> {
        let count = self.read_u32le()? as usize;
        let remaining = self.data.len() - self.pos;
        if count > MAX_RECORDS || count.saturating_mul(record_size) > remaining {
            return Err(Error::Protocol("record count exceeds the response body"));
        }
        Ok(count)
    }

    fn read_u32_list(&mut self) -> Result<Vec<u32>> {
        let count = self.read_count(4)?;
        (0..count).map(|_| self.read_u32le()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_ap() -> ApInfo {
        ApInfo { key_type: 4, ssid: "MyNetwork".into(), key: "s3cr3t-passphrase".into() }
    }

    fn sample_social() -> SocialInfo {
        SocialInfo {
            twitter_enabled: true,
            twitter_consumer_key: "ck".into(),
            twitter_consumer_secret: "cs".into(),
            twitter_access_token: "at".into(),
            twitter_access_token_secret: "ats".into(),
            twitter_message: "streaming now".into(),
            facebook_enabled: false,
            facebook_access_token: "fb".into(),
            facebook_message: "hello".into(),
        }
    }

    fn sample_service() -> StreamingService {
        StreamingService {
            service: 1,
            enabled: true,
            mac_id: "member-1".into(),
            mac_secret: "secret-1".into(),
            mac_issue_time: "0011223344556677".into(),
            channels: vec![7, 8, 9],
            short_url: "http://ust.re/abc".into(),
            video_format: 3,
            supported_formats: vec![1, 2, 3],
            enable_record_mode: true,
            video_title: "Title".into(),
            video_description: "Description".into(),
            video_tag: "tag".into(),
        }
    }

    #[test]
    fn ap_list_round_trips() {
        let aps = vec![sample_ap(), ApInfo::default()];
        let packed = pack_ap_infos(&aps).unwrap();
        assert_eq!(packed.len(), 4 + 2 * AP_INFO_SIZE);
        assert_eq!(parse_ap_infos(&packed).unwrap(), aps);
    }

    #[test]
    fn ap_record_layout_matches_the_reference() {
        let packed = pack_ap_infos(&[sample_ap()]).unwrap();
        let record = &packed[4..];
        assert_eq!(record[0], 4);
        assert_eq!(&record[1..10], b"MyNetwork");
        assert_eq!(record[1 + SSID_CAPACITY], 0, "padding byte between ssid and key");
        assert_eq!(&record[35..52], b"s3cr3t-passphrase");
    }

    #[test]
    fn social_info_round_trips_at_the_documented_size() {
        let info = sample_social();
        let packed = pack_social_info(&info).unwrap();
        assert_eq!(packed.len(), SNS_INFO_SIZE);
        assert_eq!(parse_social_info(&packed).unwrap(), info);
    }

    #[test]
    fn streaming_services_round_trip_with_variable_lists() {
        let services = vec![sample_service(), StreamingService::default()];
        let packed = pack_streaming_services(&services).unwrap();
        let parsed = parse_streaming_services(&packed).unwrap();
        // `unknown` is intentionally not round-tripped; everything else is.
        assert_eq!(parsed, services);
    }

    #[test]
    fn an_empty_service_list_is_valid() {
        let packed = pack_streaming_services(&[]).unwrap();
        assert_eq!(packed, [1, 0, 0, 0, 0, 0, 0, 0]);
        assert!(parse_streaming_services(&packed).unwrap().is_empty());
    }

    #[test]
    fn an_oversized_field_is_rejected_instead_of_truncated() {
        let ap = ApInfo { key_type: 0, ssid: "x".repeat(SSID_CAPACITY), key: String::new() };
        assert!(pack_ap_infos(&[ap]).is_err());

        // One byte less fits, because the capacity includes the NUL.
        let ap = ApInfo { key_type: 0, ssid: "x".repeat(SSID_CAPACITY - 1), key: String::new() };
        assert!(pack_ap_infos(&[ap]).is_ok());
    }

    #[test]
    fn non_latin1_text_is_rejected() {
        let ap = ApInfo { key_type: 0, ssid: "Wi-Fi \u{2764}".into(), key: String::new() };
        assert!(pack_ap_infos(&[ap]).is_err());
    }

    #[test]
    fn an_embedded_nul_is_rejected() {
        let ap = ApInfo { key_type: 0, ssid: "a\0b".into(), key: String::new() };
        assert!(pack_ap_infos(&[ap]).is_err());
    }

    #[test]
    fn a_bad_issue_timestamp_is_rejected() {
        for bad in ["", "00", "zz11223344556677", "001122334455667"] {
            let service = StreamingService { mac_issue_time: bad.into(), ..sample_service() };
            assert!(pack_streaming_services(&[service]).is_err(), "accepted {bad:?}");
        }
    }

    #[test]
    fn a_lying_record_count_is_rejected_before_allocating() {
        let mut data = 0xffff_ffffu32.to_le_bytes().to_vec();
        data.extend_from_slice(&[0; AP_INFO_SIZE]);
        assert!(parse_ap_infos(&data).is_err());
    }

    #[test]
    fn truncated_input_is_an_error_not_a_panic() {
        let packed = pack_social_info(&sample_social()).unwrap();
        for len in [0, 1, 100, SNS_INFO_SIZE - 1] {
            assert!(parse_social_info(&packed[..len]).is_err(), "accepted {len} bytes");
        }
        let packed = pack_streaming_services(&[sample_service()]).unwrap();
        for len in [0, 4, 8, 20, packed.len() - 1] {
            assert!(parse_streaming_services(&packed[..len]).is_err(), "accepted {len} bytes");
        }
    }

    #[test]
    fn trailing_bytes_after_a_field_are_ignored() {
        // The camera NUL-pads; anything after the first NUL must not leak in.
        let mut packed = pack_ap_infos(&[sample_ap()]).unwrap();
        packed[4 + 1 + 12] = b'!'; // past the SSID's NUL terminator
        assert_eq!(parse_ap_infos(&packed).unwrap()[0].ssid, "MyNetwork");
    }
}
