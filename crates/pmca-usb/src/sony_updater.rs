//! Sony firmware-updater protocol.
//!
//! Port of `SonyUpdaterCamera` in `pmca/usb/sony.py`. The updater speaks a
//! 32-byte packet header over the same [`SonyExtCmdTransport`] as the external
//! commands, using ext-cmd number `0`.

use crate::command::SonyExtCmdTransport;
use crate::{Error, Result};

/// Length of the updater packet header.
pub const PACKET_HEADER_SIZE: usize = 32;

/// Protocol version reported in every request.
const PROTOCOL_VERSION: u16 = 0x100;

/// ext-cmd number carrying updater packets.
const SONY_CMD_UPDATER: u32 = 0;

/// Default response buffer size.
const BUFFER_SIZE: usize = 512;

/// Updater command ids.
mod cmd {
    pub const INIT: u16 = 0x1;
    pub const CHK_GUARD: u16 = 0x10;
    pub const QUERY_VERSION: u16 = 0x20;
    pub const SWITCH_MODE: u16 = 0x30;
    pub const WRITE_FIRM: u16 = 0x40;
    pub const COMPLETE: u16 = 0x100;
    pub const GET_STATE: u16 = 0x200;
}

/// Response-level error codes.
const ERR_OK: u16 = 0x1;
const ERR_SEQUENCE: u16 = 0x101;

/// Per-chunk status codes.
const STAT_OK: u16 = 0x1;
const STAT_BUSY: u16 = 0x2;

/// Maps a per-chunk updater status code to its human-readable meaning.
///
/// Port of `SonyUpdaterCamera._statusToStr` in `pmca/usb/sony.py`.
fn status_to_str(status: u16) -> String {
    match status {
        0x2 => "busy".to_string(),
        0x40 => "invalid data".to_string(),
        0x100 => "low battery".to_string(),
        0x101 => "bad battery".to_string(),
        0x102 => "battery inserted".to_string(),
        0x103 => "AC adapter required".to_string(),
        0x120 => "disk inserted".to_string(),
        0x121 => "memory card inserted".to_string(),
        0x140 => "invalid model".to_string(),
        0x141 => "invalid region".to_string(),
        0x142 => "invalid version".to_string(),
        other => format!("unknown (0x{other:x})"),
    }
}

/// Renders a list of non-OK status codes for an error message.
fn describe_statuses(status: &[u16]) -> String {
    status
        .iter()
        .filter(|&&s| s != STAT_OK)
        .map(|&s| status_to_str(s))
        .collect::<Vec<_>>()
        .join(", ")
}

/// Firmware version pair `(old, new)`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FirmwareVersions {
    /// Version currently on the camera.
    pub old: String,
    /// Version contained in the update.
    pub new: String,
}

/// Builds the 32-byte updater packet header.
fn pack_header(body_size: u32, command_id: u16) -> [u8; PACKET_HEADER_SIZE] {
    let mut h = [0u8; PACKET_HEADER_SIZE];
    h[0..4].copy_from_slice(&body_size.to_le_bytes());
    h[4..6].copy_from_slice(&PROTOCOL_VERSION.to_le_bytes());
    h[6..8].copy_from_slice(&command_id.to_le_bytes());
    // responseId (8..10) and sequenceNumber (10..12) stay 0.
    h
}

/// A camera in firmware-updater mode.
pub struct SonyUpdaterCamera<T: SonyExtCmdTransport> {
    transport: T,
}

/// Callback invoked once the final firmware chunk has been sent.
///
/// It receives the camera so it can issue the COMPLETE command on the same
/// session, which is why it is a trait object rather than a plain closure
/// type parameter.
pub type OnComplete<'a, T> = &'a mut dyn FnMut(&mut SonyUpdaterCamera<T>) -> Result<()>;

impl<T: SonyExtCmdTransport> SonyUpdaterCamera<T> {
    /// Wraps a transport.
    pub fn new(transport: T) -> Self {
        Self { transport }
    }

    fn send_command(&mut self, command: u16, data: &[u8], buffer_size: usize) -> Result<Vec<u8>> {
        let mut packet = pack_header(data.len() as u32, command).to_vec();
        packet.extend_from_slice(data);
        let response =
            self.transport.send_sony_ext_command(SONY_CMD_UPDATER, &packet, buffer_size)?;

        if buffer_size == 0 {
            return Ok(Vec::new());
        }
        if response.len() < PACKET_HEADER_SIZE {
            return Err(Error::Protocol("short updater response"));
        }
        let body_size = u32::from_le_bytes(response[0..4].try_into().unwrap()) as usize;
        let response_id = u16::from_le_bytes(response[8..10].try_into().unwrap());
        if response_id != ERR_OK {
            if response_id == ERR_SEQUENCE {
                return Err(Error::Protocol("wrong updater command sequence"));
            }
            return Err(Error::Protocol("updater response error"));
        }
        let end = PACKET_HEADER_SIZE
            .checked_add(body_size)
            .filter(|e| *e <= response.len())
            .ok_or(Error::Protocol("updater body size out of bounds"))?;
        Ok(response[PACKET_HEADER_SIZE..end].to_vec())
    }

    fn parse_write_response(data: &[u8]) -> Result<(u32, Vec<u16>)> {
        if data.len() < 8 {
            return Err(Error::Protocol("short write response"));
        }
        let window_size = u32::from_le_bytes(data[0..4].try_into().unwrap());
        let num_status = u32::from_le_bytes(data[4..8].try_into().unwrap()) as usize;
        let end = 8usize
            .checked_add(num_status.checked_mul(2).ok_or(Error::Protocol("status overflow"))?)
            .filter(|e| *e <= data.len())
            .ok_or(Error::Protocol("write status out of bounds"))?;
        let status =
            data[8..end].as_chunks::<2>().0.iter().map(|c| u16::from_le_bytes(*c)).collect();
        Ok((window_size, status))
    }

    /// Device-driven flow-controlled write loop shared by CHK_GUARD / WRITE_FIRM.
    fn send_write_commands(
        &mut self,
        command: u16,
        firmware: &[u8],
        mut on_complete: Option<OnComplete<'_, T>>,
    ) -> Result<()> {
        let size = firmware.len();
        let mut data_number: u32 = 0;
        let mut written = 0usize;
        let mut window_size = 0usize;
        let mut complete_called = false;

        loop {
            data_number += 1;
            let chunk_len = window_size.min(size - written);
            let chunk = &firmware[written..written + chunk_len];
            written += chunk_len;

            let mut param = data_number.to_le_bytes().to_vec();
            param.extend_from_slice(&((size - written) as u32).to_le_bytes());
            param.extend_from_slice(chunk);

            let response = self.send_command(command, &param, BUFFER_SIZE)?;
            let (new_window, status) = Self::parse_write_response(&response)?;
            window_size = new_window as usize;

            if written == size
                && !complete_called
                && let Some(cb) = on_complete.as_deref_mut()
            {
                cb(self)?;
                complete_called = true;
            }

            if status == [STAT_OK] {
                break;
            } else if status != [STAT_BUSY] {
                return Err(Error::UpdaterStatus(describe_statuses(&status)));
            }
        }
        Ok(())
    }

    /// Returns the current updater state id.
    pub fn get_state(&mut self) -> Result<u16> {
        let data = self.send_command(cmd::GET_STATE, &[], BUFFER_SIZE)?;
        if data.len() < 2 {
            return Err(Error::Protocol("short state response"));
        }
        Ok(u16::from_le_bytes(data[0..2].try_into().unwrap()))
    }

    /// Initializes the updater session.
    pub fn init(&mut self) -> Result<()> {
        self.send_command(cmd::INIT, &[], BUFFER_SIZE)?;
        Ok(())
    }

    /// Sends the guard-check phase for `firmware`.
    pub fn check_guard(&mut self, firmware: &[u8]) -> Result<()> {
        self.send_write_commands(cmd::CHK_GUARD, firmware, None)
    }

    /// Queries the (old, new) firmware versions.
    pub fn get_firmware_version(&mut self) -> Result<FirmwareVersions> {
        let data = self.send_command(cmd::QUERY_VERSION, &[], BUFFER_SIZE)?;
        if data.len() < 8 {
            return Err(Error::Protocol("short version response"));
        }
        let old_minor = u16::from_le_bytes(data[0..2].try_into().unwrap());
        let old_major = u16::from_le_bytes(data[2..4].try_into().unwrap());
        let new_minor = u16::from_le_bytes(data[4..6].try_into().unwrap());
        let new_major = u16::from_le_bytes(data[6..8].try_into().unwrap());
        Ok(FirmwareVersions {
            old: format!("{old_major:x}.{old_minor:02x}"),
            new: format!("{new_major:x}.{new_minor:02x}"),
        })
    }

    /// Switches the updater into firmware-write mode.
    pub fn switch_mode(&mut self) -> Result<()> {
        let response = self.send_command(cmd::SWITCH_MODE, &[], BUFFER_SIZE)?;
        let (_window, status) = Self::parse_write_response(&response)?;
        if status != [STAT_OK] && status != [STAT_BUSY] {
            return Err(Error::Protocol("updater mode switch failed"));
        }
        Ok(())
    }

    /// Writes `firmware` to the camera, invoking `on_complete` once the final
    /// chunk has been sent (used to emit the COMPLETE command).
    pub fn write_firmware(
        &mut self,
        firmware: &[u8],
        on_complete: Option<OnComplete<'_, T>>,
    ) -> Result<()> {
        self.send_write_commands(cmd::WRITE_FIRM, firmware, on_complete)
    }

    /// Sends the COMPLETE command (no response expected).
    pub fn complete(&mut self) -> Result<()> {
        self.send_command(cmd::COMPLETE, &[], 0)?;
        Ok(())
    }

    /// Consumes the camera, returning the underlying transport.
    pub fn into_transport(self) -> T {
        self.transport
    }

    /// Borrows the underlying transport mutably.
    ///
    /// Used to run the interactive platform shell over the same connection once
    /// the custom updater payload is live, without moving the transport out of
    /// the updater session.
    pub fn transport_mut(&mut self) -> &mut T {
        &mut self.transport
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    /// Transport that returns scripted responses and records requests.
    struct ScriptTransport {
        responses: VecDeque<Vec<u8>>,
        requests: Vec<Vec<u8>>,
    }

    impl ScriptTransport {
        fn new(responses: Vec<Vec<u8>>) -> Self {
            Self { responses: responses.into(), requests: Vec::new() }
        }
    }

    impl SonyExtCmdTransport for ScriptTransport {
        fn send_sony_ext_command(
            &mut self,
            _cmd: u32,
            data: &[u8],
            buffer_size: usize,
        ) -> Result<Vec<u8>> {
            self.requests.push(data.to_vec());
            if buffer_size == 0 {
                return Ok(Vec::new());
            }
            Ok(self.responses.pop_front().unwrap_or_default())
        }
    }

    fn updater_response(body: &[u8]) -> Vec<u8> {
        let mut v = pack_header(body.len() as u32, 0).to_vec();
        v[8..10].copy_from_slice(&ERR_OK.to_le_bytes()); // responseId = OK
        v.extend_from_slice(body);
        v
    }

    fn write_response(window: u32, status: &[u16]) -> Vec<u8> {
        let mut body = window.to_le_bytes().to_vec();
        body.extend_from_slice(&(status.len() as u32).to_le_bytes());
        for s in status {
            body.extend_from_slice(&s.to_le_bytes());
        }
        updater_response(&body)
    }

    #[test]
    fn get_state_parses_body() {
        let t = ScriptTransport::new(vec![updater_response(&0x0007u16.to_le_bytes())]);
        let mut cam = SonyUpdaterCamera::new(t);
        assert_eq!(cam.get_state().unwrap(), 7);
    }

    #[test]
    fn firmware_version_formatting() {
        // old = 1.02, new = 2.10
        let body = [0x02, 0x00, 0x01, 0x00, 0x10, 0x00, 0x02, 0x00];
        let t = ScriptTransport::new(vec![updater_response(&body)]);
        let mut cam = SonyUpdaterCamera::new(t);
        let v = cam.get_firmware_version().unwrap();
        assert_eq!(v.old, "1.02");
        assert_eq!(v.new, "2.10");
    }

    #[test]
    fn sequence_error_is_reported() {
        let mut resp = pack_header(0, 0).to_vec();
        resp[8..10].copy_from_slice(&ERR_SEQUENCE.to_le_bytes());
        let t = ScriptTransport::new(vec![resp]);
        let mut cam = SonyUpdaterCamera::new(t);
        let err = cam.init().unwrap_err();
        assert!(matches!(err, Error::Protocol(m) if m.contains("sequence")));
    }

    #[test]
    fn write_firmware_flow_control() {
        // First reply: window=4, busy (so more chunks are sent).
        // Second reply: window=4, ok (loop ends).
        let firmware = vec![0xaau8; 4];
        let t = ScriptTransport::new(vec![
            write_response(4, &[STAT_BUSY]),
            write_response(4, &[STAT_OK]),
        ]);
        let mut cam = SonyUpdaterCamera::new(t);
        let mut completed = false;
        {
            let mut cb = |_c: &mut SonyUpdaterCamera<ScriptTransport>| {
                completed = true;
                Ok(())
            };
            cam.write_firmware(&firmware, Some(&mut cb)).unwrap();
        }
        assert!(completed);

        // Inspect what was written: first request sends 0 data bytes (window 0),
        // second request sends the 4-byte chunk.
        let t = cam.into_transport();
        // header(32) + WriteParam(8) + data
        assert_eq!(t.requests[0].len(), PACKET_HEADER_SIZE + 8);
        assert_eq!(t.requests[1].len(), PACKET_HEADER_SIZE + 8 + 4);
    }

    #[test]
    fn write_firmware_reports_rejection_status() {
        // Camera rejects the chunk with a non-OK, non-BUSY status (low battery).
        let firmware = vec![0xaau8; 4];
        let t = ScriptTransport::new(vec![write_response(4, &[0x100])]);
        let mut cam = SonyUpdaterCamera::new(t);
        let err = cam.write_firmware(&firmware, None).unwrap_err();
        assert!(matches!(err, Error::UpdaterStatus(m) if m.contains("low battery")));
    }

    #[test]
    fn status_strings_map_known_codes() {
        assert_eq!(status_to_str(0x100), "low battery");
        assert_eq!(status_to_str(0x140), "invalid model");
        assert_eq!(status_to_str(0x142), "invalid version");
        assert!(status_to_str(0x9999).contains("9999"));
        // OK/BUSY are filtered out of the human-readable summary.
        assert_eq!(describe_statuses(&[STAT_OK]), "");
        assert_eq!(describe_statuses(&[STAT_OK, 0x100]), "low battery");
    }
}
