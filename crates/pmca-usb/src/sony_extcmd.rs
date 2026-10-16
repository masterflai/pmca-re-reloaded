//! High-level Sony external-command camera operations.
//!
//! Port of `SonyExtCmdCamera` in `pmca/usb/sony.py`. Every command is framed
//! with the reference's 16-byte `ExtCmdHeader` and carried over a
//! [`SonyExtCmdTransport`].
//! All response parsing is bounds-checked and returns `Result` instead of
//! panicking, and text fields are decoded as Latin-1 explicitly.

use crate::command::SonyExtCmdTransport;
use crate::network::{
    ApInfo, SocialInfo, StreamingService, pack_ap_infos, pack_social_info, pack_streaming_services,
    parse_ap_infos, parse_social_info, parse_streaming_services,
};
use crate::{Error, Result};

/// Length of the external-command header.
pub const EXT_CMD_HEADER_SIZE: usize = 16;

/// Default read/write buffer size for external commands.
pub const DEFAULT_BUFFER_SIZE: usize = 0x2000;

/// Buffer size used by the GPS-assist commands.
const GPS_BUFFER_SIZE: usize = 0x10000;

/// Read-buffer size used by the GPS-assist commands.
const GPS_READ_BUFFER_SIZE: usize = 0x200;

/// Total size of an assist-data upload, as fixed by the camera protocol.
///
/// 30 days x 4 dates/day x 32 frames/date x 72 bytes/frame.
pub const GPS_DATA_SIZE: u32 = 0x43800;

/// Length of the data-transfer header preceding each GPS chunk.
const DATA_TRANSFER_HEADER_SIZE: usize = 12;

/// Upper bound on the chunks accepted while reading the usage log.
///
/// The camera ends the transfer by reporting `remaining == 0`. A device that
/// never does would otherwise keep the host reading — and allocating —
/// forever, so the loop is bounded. At the default buffer size this still
/// allows a log far larger than any observed camera produces.
const USAGE_LOG_MAX_CHUNKS: usize = 4096;

/// A Sony external command, identified by `(ext_cmd, sub_cmd)`.
pub type SonyCmd = (u32, u16);

// DevInfoSender
const CMD_GET_MODEL_INFO: SonyCmd = (1, 1);
// KikiLogSender
const CMD_INIT_KIKI_LOG: SonyCmd = (2, 1);
const CMD_READ_KIKI_LOG: SonyCmd = (2, 2);
// GpsAssist
const CMD_INIT_GPS: SonyCmd = (3, 1);
const CMD_WRITE_GPS: SonyCmd = (3, 2);
// ExtBackupCommunicator
const CMD_FORCE_POWER_OFF: SonyCmd = (4, 5);
const CMD_GET_DEVICE_NET_INFO: SonyCmd = (4, 9);
// ScalarExtCmdPlugIn
const CMD_NOTIFY_SCALAR_DLMODE: SonyCmd = (5, 2);
// LensCommunicator
const CMD_GET_MOUNTED_LENS_INFO: SonyCmd = (6, 2);
// NetworkServiceInfo
const CMD_SET_LIVE_STREAMING_SERVICE_INFO: SonyCmd = (7, 2);
const CMD_GET_LIVE_STREAMING_SERVICE_INFO: SonyCmd = (7, 3);
const CMD_SET_LIVE_STREAMING_SNS_INFO: SonyCmd = (7, 4);
const CMD_GET_LIVE_STREAMING_SNS_INFO: SonyCmd = (7, 5);
const CMD_SET_WIFI_AP_INFO: SonyCmd = (7, 6);
const CMD_GET_WIFI_AP_INFO: SonyCmd = (7, 7);
const CMD_SET_MULTI_WIFI_AP_INFO: SonyCmd = (7, 9);
const CMD_GET_MULTI_WIFI_AP_INFO: SonyCmd = (7, 10);

/// The validity window of the assist data currently stored on the camera.
///
/// The values are raw camera timestamps: hours since 1980-01-06.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GpsDataRange {
    /// Status word reported by the camera.
    pub status: u16,
    /// Hours since 1980-01-06 at which the data becomes valid.
    pub first_date: u32,
    /// Hours since 1980-01-06 at which the data expires.
    pub last_date: u32,
}

/// Information identifying a camera.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CameraInfo {
    /// Raw device-properties plist (bytes).
    pub plist: Vec<u8>,
    /// Human-readable model name (e.g. `ILCE-5100`).
    pub model_name: String,
    /// Hex-encoded 5-byte model code.
    pub model_code: String,
    /// Hex-encoded 4-byte serial number.
    pub serial: String,
}

/// Information about a mounted lens.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LensInfo {
    /// Lens type id.
    pub lens_type: u32,
    /// Model id.
    pub model: u32,
    /// Region id.
    pub region: u32,
    /// Firmware version string (`major.minor`).
    pub version: String,
}

/// Builds the 16-byte external-command header.
fn pack_ext_cmd_header(data_size: u32, cmd: u16, direction: u16) -> [u8; EXT_CMD_HEADER_SIZE] {
    let mut h = [0u8; EXT_CMD_HEADER_SIZE];
    h[0..4].copy_from_slice(&data_size.to_le_bytes());
    h[4..6].copy_from_slice(&cmd.to_le_bytes());
    h[6..8].copy_from_slice(&direction.to_le_bytes());
    h
}

/// A camera addressed via Sony external commands over a given transport.
pub struct SonyExtCmdCamera<T: SonyExtCmdTransport> {
    transport: T,
}

impl<T: SonyExtCmdTransport> SonyExtCmdCamera<T> {
    /// Wraps a transport.
    pub fn new(transport: T) -> Self {
        Self { transport }
    }

    /// Sends an external command and returns the unwrapped response payload.
    pub fn send_command(
        &mut self,
        cmd: SonyCmd,
        data: &[u8],
        write_buffer_size: usize,
        read_buffer_size: usize,
    ) -> Result<Vec<u8>> {
        let mut request = pack_ext_cmd_header(data.len() as u32, cmd.1, 0).to_vec();
        request.extend_from_slice(data);
        if request.len() < write_buffer_size {
            request.resize(write_buffer_size, 0);
        }

        let response = self.transport.send_sony_ext_command(cmd.0, &request, read_buffer_size)?;
        if read_buffer_size == 0 {
            return Ok(Vec::new());
        }
        if response.len() < EXT_CMD_HEADER_SIZE {
            return Err(Error::Protocol("short external-command response"));
        }
        let data_size = u32::from_le_bytes(response[0..4].try_into().unwrap()) as usize;
        let end = EXT_CMD_HEADER_SIZE
            .checked_add(data_size)
            .filter(|e| *e <= response.len())
            .ok_or(Error::Protocol("external-command response length out of bounds"))?;
        Ok(response[EXT_CMD_HEADER_SIZE..end].to_vec())
    }

    /// Convenience wrapper using the default buffer sizes.
    fn send_default(&mut self, cmd: SonyCmd, data: &[u8]) -> Result<Vec<u8>> {
        self.send_command(cmd, data, DEFAULT_BUFFER_SIZE, DEFAULT_BUFFER_SIZE)
    }

    /// Reads model identification information from the camera.
    pub fn get_camera_info(&mut self) -> Result<CameraInfo> {
        let data = self.send_default(CMD_GET_MODEL_INFO, &[])?;
        let mut c = Cursor::new(&data);
        let plist_size = c.read_u32le()? as usize;
        let plist = c.read_bytes(plist_size)?.to_vec();
        c.skip(4)?;
        let model_size = c.read_u8()? as usize;
        let model_name = latin1(c.read_bytes(model_size)?);
        let model_code = hex(c.read_bytes(5)?);
        let serial = hex(c.read_bytes(4)?);
        Ok(CameraInfo { plist, model_name, model_code, serial })
    }

    /// Tells the camera to switch into application-installation mode.
    pub fn switch_to_app_installer(&mut self) -> Result<()> {
        self.send_command(CMD_NOTIFY_SCALAR_DLMODE, &[], DEFAULT_BUFFER_SIZE, 0)?;
        Ok(())
    }

    /// Reads the camera's usage log ("KikiLog").
    ///
    /// The log is a diagnostic blob Sony's service tooling collects; its
    /// internal structure is undocumented, so the bytes are returned as-is
    /// rather than being parsed into a shape this project cannot verify.
    ///
    /// The transfer is read-only: the init command opens a read cursor on the
    /// camera, it does not clear or rewrite the log.
    pub fn get_usage_log(&mut self) -> Result<Vec<u8>> {
        self.send_default(CMD_INIT_KIKI_LOG, &[])?;

        let mut log = Vec::new();
        for _ in 0..USAGE_LOG_MAX_CHUNKS {
            let data = self.send_default(CMD_READ_KIKI_LOG, &[])?;
            if data.len() < DATA_TRANSFER_HEADER_SIZE {
                return Err(Error::Protocol("short usage-log response"));
            }
            let remaining = u32::from_le_bytes(data[4..8].try_into().unwrap());
            let size = u32::from_le_bytes(data[8..12].try_into().unwrap()) as usize;
            let end = DATA_TRANSFER_HEADER_SIZE
                .checked_add(size)
                .filter(|e| *e <= data.len())
                .ok_or(Error::Protocol("usage-log chunk size out of bounds"))?;
            log.extend_from_slice(&data[DATA_TRANSFER_HEADER_SIZE..end]);
            if remaining == 0 {
                return Ok(log);
            }
        }
        Err(Error::Protocol("usage log did not end"))
    }

    /// Forces the camera to power off.
    pub fn power_off(&mut self) -> Result<()> {
        self.send_command(CMD_FORCE_POWER_OFF, &[], DEFAULT_BUFFER_SIZE, 0)?;
        Ok(())
    }

    /// Returns the camera's MAC address as a colon-separated string.
    pub fn get_mac_address(&mut self) -> Result<String> {
        let data = self.send_default(CMD_GET_DEVICE_NET_INFO, &[])?;
        let addr = data.get(2..8).ok_or(Error::Protocol("short net-info response"))?;
        Ok(addr.iter().map(|b| format!("{b:02X}")).collect::<Vec<_>>().join(":"))
    }

    /// Returns information about the mounted lens.
    pub fn get_lens_info(&mut self) -> Result<LensInfo> {
        let data = self.send_default(CMD_GET_MOUNTED_LENS_INFO, &[])?;
        if data.len() < 14 {
            return Err(Error::Protocol("short lens-info response"));
        }
        let lens_type = u32::from_le_bytes(data[0..4].try_into().unwrap());
        let version_minor = data[4];
        let version_major = data[5];
        let model_raw = &data[6..10];
        let region_raw = &data[10..14];
        // model: reorder bytes [0,1,3,2] then read as big-endian.
        let model = u32::from_be_bytes([model_raw[0], model_raw[1], model_raw[3], model_raw[2]]);
        let region = u32::from_be_bytes(region_raw.try_into().unwrap());
        Ok(LensInfo {
            lens_type,
            model,
            region,
            version: format!("{version_major:x}.{version_minor:02x}"),
        })
    }

    /// Consumes the camera, returning the underlying transport.
    pub fn into_transport(self) -> T {
        self.transport
    }

    // -----------------------------------------------------------------
    // GPS assist data
    // -----------------------------------------------------------------

    /// Queries the validity window of the assist data on the camera.
    pub fn get_gps_data_range(&mut self) -> Result<GpsDataRange> {
        let mut request = Vec::with_capacity(20);
        request.extend_from_slice(&0u32.to_le_bytes()); // firstDate
        request.extend_from_slice(&0u32.to_le_bytes()); // lastDate
        request.extend_from_slice(&1u32.to_le_bytes()); // one
        request.extend_from_slice(&0u32.to_le_bytes()); // crc32
        request.extend_from_slice(&GPS_DATA_SIZE.to_le_bytes());

        let data =
            self.send_command(CMD_INIT_GPS, &request, GPS_BUFFER_SIZE, GPS_READ_BUFFER_SIZE)?;
        if data.len() < 10 {
            return Err(Error::Protocol("short GPS init response"));
        }
        Ok(GpsDataRange {
            status: u16::from_le_bytes(data[0..2].try_into().unwrap()),
            first_date: u32::from_le_bytes(data[2..6].try_into().unwrap()),
            last_date: u32::from_le_bytes(data[6..10].try_into().unwrap()),
        })
    }

    /// Uploads an `assistme.dat` image to the camera.
    ///
    /// The camera expects exactly [`GPS_DATA_SIZE`] bytes. A shorter file is
    /// rejected up front rather than leaving the transfer half-finished, and a
    /// longer one is refused instead of being silently cut short.
    pub fn write_gps_data(&mut self, data: &[u8]) -> Result<()> {
        if data.len() != GPS_DATA_SIZE as usize {
            return Err(Error::Protocol("GPS assist data has an unexpected size"));
        }
        let chunk_size = GPS_BUFFER_SIZE - EXT_CMD_HEADER_SIZE - DATA_TRANSFER_HEADER_SIZE;
        let mut remaining = data.len();
        for (index, chunk) in data.chunks(chunk_size).enumerate() {
            remaining -= chunk.len();
            let mut request = Vec::with_capacity(DATA_TRANSFER_HEADER_SIZE + chunk.len());
            request.extend_from_slice(&((index + 1) as u32).to_le_bytes());
            request.extend_from_slice(&(remaining as u32).to_le_bytes());
            request.extend_from_slice(&(chunk.len() as u32).to_le_bytes());
            request.extend_from_slice(chunk);

            let response =
                self.send_command(CMD_WRITE_GPS, &request, GPS_BUFFER_SIZE, GPS_READ_BUFFER_SIZE)?;
            let expected: &[u8] = if remaining == 0 { &[1, 0] } else { &[0, 0] };
            if response != expected {
                return Err(Error::Protocol("camera rejected a GPS assist-data chunk"));
            }
        }
        Ok(())
    }

    // -----------------------------------------------------------------
    // Network services
    // -----------------------------------------------------------------

    /// Reads the configured live-streaming access points.
    pub fn get_wifi_ap_info(&mut self) -> Result<Vec<ApInfo>> {
        let data = self.send_default(CMD_GET_WIFI_AP_INFO, &[])?;
        parse_ap_infos(&data)
    }

    /// Writes the live-streaming access points.
    pub fn set_wifi_ap_info(&mut self, aps: &[ApInfo]) -> Result<()> {
        let data = pack_ap_infos(aps)?;
        self.send_default(CMD_SET_WIFI_AP_INFO, &data)?;
        Ok(())
    }

    /// Reads the multi-access-point configuration.
    pub fn get_multi_wifi_ap_info(&mut self) -> Result<Vec<ApInfo>> {
        let data = self.send_default(CMD_GET_MULTI_WIFI_AP_INFO, &[])?;
        parse_ap_infos(&data)
    }

    /// Writes the multi-access-point configuration.
    pub fn set_multi_wifi_ap_info(&mut self, aps: &[ApInfo]) -> Result<()> {
        let data = pack_ap_infos(aps)?;
        self.send_default(CMD_SET_MULTI_WIFI_AP_INFO, &data)?;
        Ok(())
    }

    /// Reads the social-media configuration used when streaming starts.
    pub fn get_live_streaming_social_info(&mut self) -> Result<SocialInfo> {
        let data = self.send_default(CMD_GET_LIVE_STREAMING_SNS_INFO, &[])?;
        parse_social_info(&data)
    }

    /// Writes the social-media configuration.
    pub fn set_live_streaming_social_info(&mut self, info: &SocialInfo) -> Result<()> {
        let data = pack_social_info(info)?;
        self.send_default(CMD_SET_LIVE_STREAMING_SNS_INFO, &data)?;
        Ok(())
    }

    /// Reads the configured live-streaming services.
    pub fn get_live_streaming_service_info(&mut self) -> Result<Vec<StreamingService>> {
        let data = self.send_default(CMD_GET_LIVE_STREAMING_SERVICE_INFO, &[])?;
        parse_streaming_services(&data)
    }

    /// Writes the live-streaming service configuration.
    pub fn set_live_streaming_service_info(&mut self, services: &[StreamingService]) -> Result<()> {
        let data = pack_streaming_services(services)?;
        self.send_default(CMD_SET_LIVE_STREAMING_SERVICE_INFO, &data)?;
        Ok(())
    }
}

/// Decodes bytes as Latin-1.
fn latin1(bytes: &[u8]) -> String {
    bytes.iter().map(|&b| b as char).collect()
}

/// Hex-encodes bytes (lowercase).
fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// Minimal bounds-checked forward reader over a byte slice.
struct Cursor<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Cursor<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }

    fn read_bytes(&mut self, len: usize) -> Result<&'a [u8]> {
        let end = self.pos.checked_add(len).ok_or(Error::Protocol("read overflow"))?;
        let slice = self.data.get(self.pos..end).ok_or(Error::Protocol("read past end of data"))?;
        self.pos = end;
        Ok(slice)
    }

    fn read_u8(&mut self) -> Result<u8> {
        Ok(self.read_bytes(1)?[0])
    }

    fn read_u32le(&mut self) -> Result<u32> {
        Ok(u32::from_le_bytes(self.read_bytes(4)?.try_into().unwrap()))
    }

    fn skip(&mut self, len: usize) -> Result<()> {
        self.read_bytes(len).map(|_| ())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::MockBackend;
    use crate::msc::{CSW_SIZE, MscDevice};
    use crate::mtp::MtpDevice;
    use crate::mtp::{container_type, pack_container, ptp};

    const EP_IN: u8 = 0x81;
    const EP_OUT: u8 = 0x02;

    /// Wrap a payload into the ext-cmd response envelope (header + data).
    fn ext_response(payload: &[u8]) -> Vec<u8> {
        let mut v = pack_ext_cmd_header(payload.len() as u32, 0, 0).to_vec();
        v.extend_from_slice(payload);
        v
    }

    fn model_info_payload() -> Vec<u8> {
        let mut p = Vec::new();
        let plist = b"<plist/>";
        p.extend_from_slice(&(plist.len() as u32).to_le_bytes());
        p.extend_from_slice(plist);
        p.extend_from_slice(&[0, 0, 0, 0]); // skipped 4 bytes
        let model = b"ILCE-5100";
        p.push(model.len() as u8);
        p.extend_from_slice(model);
        p.extend_from_slice(&[0x11, 0x22, 0x33, 0x44, 0x55]); // model code
        p.extend_from_slice(&[0xaa, 0xbb, 0xcc, 0xdd]); // serial
        p
    }

    #[test]
    fn get_camera_info_over_mtp() {
        let payload = model_info_payload();
        let mut backend = MockBackend::new();
        // write phase response
        backend.queue_read(EP_IN, pack_container(container_type::RESPONSE, ptp::RC_OK, 1, &[]));
        // read phase: data (ext envelope) + response
        backend.queue_read(
            EP_IN,
            pack_container(container_type::DATA, 0x9281, 2, &ext_response(&payload)),
        );
        backend.queue_read(EP_IN, pack_container(container_type::RESPONSE, ptp::RC_OK, 2, &[]));

        let dev = MtpDevice::new(backend, EP_IN, EP_OUT);
        let mut cam = SonyExtCmdCamera::new(dev);
        let info = cam.get_camera_info().unwrap();
        assert_eq!(info.model_name, "ILCE-5100");
        assert_eq!(info.model_code, "1122334455");
        assert_eq!(info.serial, "aabbccdd");
        assert_eq!(info.plist, b"<plist/>");
    }

    #[test]
    fn get_camera_info_over_msc() {
        let payload = model_info_payload();
        let envelope = ext_response(&payload);

        let mut csw = vec![0u8; CSW_SIZE];
        csw[0..4].copy_from_slice(b"USBS");

        let mut backend = MockBackend::new();
        // write phase: CSW ok
        backend.queue_read(EP_IN, csw.clone());
        // read phase: data then CSW ok
        backend.queue_read(EP_IN, envelope);
        backend.queue_read(EP_IN, csw);

        let dev = MscDevice::new(backend, EP_IN, EP_OUT);
        let mut cam = SonyExtCmdCamera::new(dev);
        let info = cam.get_camera_info().unwrap();
        assert_eq!(info.model_name, "ILCE-5100");
        assert_eq!(info.model_code, "1122334455");
        assert_eq!(info.serial, "aabbccdd");
    }

    /// A transport returning a fixed, attacker-controlled response.
    struct FixedTransport(Vec<u8>);
    impl SonyExtCmdTransport for FixedTransport {
        fn send_sony_ext_command(&mut self, _cmd: u32, _data: &[u8], _n: usize) -> Result<Vec<u8>> {
            Ok(self.0.clone())
        }
    }

    #[test]
    fn short_response_is_rejected() {
        // Envelope claims a large data size but the buffer is too short.
        let mut bad = pack_ext_cmd_header(0xffff, 0, 0).to_vec();
        bad.truncate(8);
        let mut cam = SonyExtCmdCamera::new(FixedTransport(bad));
        assert!(
            cam.send_command(CMD_GET_MODEL_INFO, &[], DEFAULT_BUFFER_SIZE, DEFAULT_BUFFER_SIZE)
                .is_err()
        );
    }

    #[test]
    fn oversized_data_size_is_rejected() {
        // Header claims more data than present -> out-of-bounds guarded.
        let envelope = pack_ext_cmd_header(1000, 0, 0).to_vec();
        let mut cam = SonyExtCmdCamera::new(FixedTransport(envelope));
        assert!(
            cam.send_command(CMD_GET_MODEL_INFO, &[], DEFAULT_BUFFER_SIZE, DEFAULT_BUFFER_SIZE)
                .is_err()
        );
    }

    /// A transport that records requests and replays queued responses.
    #[derive(Default)]
    struct ScriptedTransport {
        requests: Vec<(u32, Vec<u8>)>,
        responses: Vec<Vec<u8>>,
    }

    impl ScriptedTransport {
        fn queue(&mut self, payload: &[u8]) {
            self.responses.push(ext_response(payload));
        }
    }

    impl SonyExtCmdTransport for ScriptedTransport {
        fn send_sony_ext_command(&mut self, cmd: u32, data: &[u8], n: usize) -> Result<Vec<u8>> {
            self.requests.push((cmd, data.to_vec()));
            if n == 0 {
                return Ok(Vec::new());
            }
            Ok(if self.responses.is_empty() { ext_response(&[]) } else { self.responses.remove(0) })
        }
    }

    /// Reads the sub-command and payload back out of a recorded request.
    fn request_parts(request: &[u8]) -> (u16, &[u8]) {
        let sub_cmd = u16::from_le_bytes(request[4..6].try_into().unwrap());
        let size = u32::from_le_bytes(request[0..4].try_into().unwrap()) as usize;
        (sub_cmd, &request[EXT_CMD_HEADER_SIZE..EXT_CMD_HEADER_SIZE + size])
    }

    #[test]
    fn wifi_access_points_round_trip_through_the_camera() {
        let aps = vec![ApInfo { key_type: 4, ssid: "net".into(), key: "pass".into() }];
        let mut transport = ScriptedTransport::default();
        transport.queue(&pack_ap_infos(&aps).unwrap());
        let mut cam = SonyExtCmdCamera::new(transport);

        assert_eq!(cam.get_wifi_ap_info().unwrap(), aps);
        cam.set_multi_wifi_ap_info(&aps).unwrap();

        let transport = cam.into_transport();
        assert_eq!(transport.requests[0].0, 7, "network commands use ext-cmd group 7");
        assert_eq!(request_parts(&transport.requests[0].1).0, 7, "GetWifiAPInfo");
        let (sub_cmd, payload) = request_parts(&transport.requests[1].1);
        assert_eq!(sub_cmd, 9, "SetMultiWifiAPInfo");
        assert_eq!(parse_ap_infos(payload).unwrap(), aps);
    }

    #[test]
    fn writing_gps_data_chunks_the_upload_and_counts_down() {
        // The camera answers `00 00` for every chunk but the last, which gets `01 00`.
        let mut transport = ScriptedTransport::default();
        let chunks = GPS_DATA_SIZE as usize
            / (GPS_BUFFER_SIZE - EXT_CMD_HEADER_SIZE - DATA_TRANSFER_HEADER_SIZE)
            + 1;
        for i in 0..chunks {
            transport.queue(if i == chunks - 1 { &[1, 0] } else { &[0, 0] });
        }
        let mut cam = SonyExtCmdCamera::new(transport);
        cam.write_gps_data(&vec![0xab; GPS_DATA_SIZE as usize]).unwrap();

        let transport = cam.into_transport();
        assert_eq!(transport.requests.len(), chunks);
        let mut total = 0usize;
        for (index, (_, request)) in transport.requests.iter().enumerate() {
            let (sub_cmd, payload) = request_parts(request);
            assert_eq!(sub_cmd, 2, "WriteGps");
            let sequence = u32::from_le_bytes(payload[0..4].try_into().unwrap());
            let remaining = u32::from_le_bytes(payload[4..8].try_into().unwrap()) as usize;
            let size = u32::from_le_bytes(payload[8..12].try_into().unwrap()) as usize;
            assert_eq!(sequence as usize, index + 1, "sequence numbers start at one");
            total += size;
            assert_eq!(remaining, GPS_DATA_SIZE as usize - total);
        }
        assert_eq!(total, GPS_DATA_SIZE as usize);
    }

    #[test]
    fn gps_data_of_the_wrong_size_is_refused_before_any_transfer() {
        let mut cam = SonyExtCmdCamera::new(ScriptedTransport::default());
        assert!(cam.write_gps_data(&[]).is_err());
        assert!(cam.write_gps_data(&vec![0; GPS_DATA_SIZE as usize - 1]).is_err());
        assert!(cam.into_transport().requests.is_empty(), "nothing must reach the camera");
    }

    /// Wraps a usage-log chunk in its data-transfer header.
    fn usage_log_chunk(sequence: u32, remaining: u32, data: &[u8]) -> Vec<u8> {
        let mut v = Vec::new();
        v.extend_from_slice(&sequence.to_le_bytes());
        v.extend_from_slice(&remaining.to_le_bytes());
        v.extend_from_slice(&(data.len() as u32).to_le_bytes());
        v.extend_from_slice(data);
        v
    }

    #[test]
    fn the_usage_log_is_reassembled_until_nothing_remains() {
        let mut transport = ScriptedTransport::default();
        transport.queue(&[]); // InitKikiLog
        transport.queue(&usage_log_chunk(1, 4, b"abc"));
        transport.queue(&usage_log_chunk(2, 0, b"defg"));
        let mut cam = SonyExtCmdCamera::new(transport);

        assert_eq!(cam.get_usage_log().unwrap(), b"abcdefg");

        let transport = cam.into_transport();
        assert_eq!(transport.requests.len(), 3);
        assert!(transport.requests.iter().all(|(cmd, _)| *cmd == 2), "KikiLogSender group");
        assert_eq!(request_parts(&transport.requests[0].1).0, 1, "InitKikiLog");
        assert_eq!(request_parts(&transport.requests[1].1).0, 2, "ReadKikiLog");
    }

    #[test]
    fn a_usage_log_chunk_longer_than_its_frame_is_rejected() {
        // The header claims more payload than the frame carries.
        let mut transport = ScriptedTransport::default();
        transport.queue(&[]);
        transport.queue(&usage_log_chunk(1, 0, b"short")[..DATA_TRANSFER_HEADER_SIZE + 2]);
        let mut cam = SonyExtCmdCamera::new(transport);
        assert!(cam.get_usage_log().is_err());
    }

    #[test]
    fn a_usage_log_that_never_ends_is_abandoned() {
        // Every chunk claims more data is coming; without a bound this would
        // read - and allocate - forever.
        struct EndlessLog;
        impl SonyExtCmdTransport for EndlessLog {
            fn send_sony_ext_command(&mut self, _c: u32, _d: &[u8], _n: usize) -> Result<Vec<u8>> {
                Ok(ext_response(&usage_log_chunk(1, 1, b"x")))
            }
        }
        let mut cam = SonyExtCmdCamera::new(EndlessLog);
        assert!(cam.get_usage_log().is_err());
    }

    #[test]
    fn a_rejected_gps_chunk_aborts_the_upload() {
        let mut transport = ScriptedTransport::default();
        transport.queue(&[0xff, 0xff]);
        let mut cam = SonyExtCmdCamera::new(transport);
        assert!(cam.write_gps_data(&vec![0; GPS_DATA_SIZE as usize]).is_err());
        assert_eq!(cam.into_transport().requests.len(), 1, "stops at the first rejection");
    }

    #[test]
    fn social_info_survives_a_camera_round_trip() {
        let info = SocialInfo {
            twitter_enabled: true,
            twitter_message: "live".into(),
            facebook_access_token: "token".into(),
            ..SocialInfo::default()
        };
        let mut transport = ScriptedTransport::default();
        transport.queue(&pack_social_info(&info).unwrap());
        let mut cam = SonyExtCmdCamera::new(transport);
        assert_eq!(cam.get_live_streaming_social_info().unwrap(), info);
    }
}
