//! High-level service-mode ("senser") camera commands.
//!
//! Byte-faithful port of `SonySenserCamera` from `pmca/usb/sony.py`, layered on
//! top of [`crate::senser::SenserDevice`].
//!
//! # Safety / risk note
//!
//! Several commands here write to the camera's filesystem, raw memory or
//! calibration backup. Misuse can permanently damage a device. Callers are
//! expected to gate the write paths behind explicit user confirmation.

use std::io::Write;

use crate::senser::{SenserDevice, WireTrace};
use crate::{Error, Result, UsbBackend};

/// Senser subsystem selectors (`pFunc`).
pub mod pfunc {
    /// Product information / terminal control.
    pub const PRODUCT_INFO: u16 = 0x10;
    /// Firmware update.
    pub const FIRMWARE_UPDATE: u16 = 0x20;
    /// Micon (microcontroller) access.
    pub const MICON_ACCESS: u16 = 0x30;
    /// Adjustment / calibration control.
    pub const ADJUST_CONTROL: u16 = 0x40;
    /// Test mode.
    pub const TEST_MODE: u16 = 0xff00;
    /// Filesystem access.
    pub const FILE_CONTROL: u16 = 0xff01;
    /// Sonar subsystem.
    pub const SONAR: u16 = 0xff02;
    /// Raw memory read/write.
    pub const MEMORY_DUMP: u16 = 0xff03;
}

/// `(category, command)` pair: read the HASP (hardware protection) block.
pub const PRODUCT_INFO_READ_HASP: (u16, u16) = (0, 0x001f);
/// `(category, command)` pair: enable or disable the debug terminal.
pub const PRODUCT_INFO_TERM_CHANGE: (u16, u16) = (0, 0x00f1);

/// `(block, code)` pair: switch the senser terminal mode.
pub const ADJUST_SENSER_TERM_MODE: (u16, u16) = (0x601, 0x1f);
/// `(block, code)` pair: read a backup entry.
pub const ADJUST_BACKUP_READ: (u16, u16) = (0x603, 1);
/// `(block, code)` pair: write a backup entry.
pub const ADJUST_BACKUP_WRITE: (u16, u16) = (0x603, 2);
/// `(block, code)` pair: persist the backup of a subsystem.
pub const ADJUST_BACKUP_SAVE: (u16, u16) = (0x603, 3);
/// `(block, code)` pair: write the backup preset data.
pub const ADJUST_BACKUP_PDT_WRITE: (u16, u16) = (0x603, 4);
/// `(block, code)` pair: read the backup preset data.
pub const ADJUST_BACKUP_PDT_READ: (u16, u16) = (0x603, 5);
/// `(block, code)` pair: query the backup preset data status.
pub const ADJUST_BACKUP_PDT_STAT: (u16, u16) = (0x603, 6);
/// `(block, code)` pair: get/set backup ID1.
pub const ADJUST_BACKUP_ID1: (u16, u16) = (0x603, 15);

/// File control sub-command: write a file.
pub const FILE_CONTROL_WRITE: u16 = 1;
/// File control sub-command: read a file.
pub const FILE_CONTROL_READ: u16 = 2;
/// File control sub-command: delete a file.
pub const FILE_CONTROL_DELETE: u16 = 3;

/// Packs a `(category, command)` command header (little-endian).
fn command_header(category: u16, command: u16) -> [u8; 4] {
    let mut out = [0u8; 4];
    out[0..2].copy_from_slice(&category.to_le_bytes());
    out[2..4].copy_from_slice(&command.to_le_bytes());
    out
}

/// Pads a filename to a multiple of four bytes, always appending at least one
/// NUL terminator (matching the reference implementation).
fn pad_filename(filename: &str) -> Result<Vec<u8>> {
    let mut bytes = Vec::with_capacity(filename.len() + 4);
    for c in filename.chars() {
        // The reference encodes filenames as Latin-1.
        let code = c as u32;
        if code > 0xff {
            return Err(Error::Protocol("filename is not representable in latin-1"));
        }
        bytes.push(code as u8);
    }
    bytes.resize(bytes.len() + (4 - bytes.len() % 4), 0);
    Ok(bytes)
}

/// Service-mode command interface for an authenticated camera.
pub struct SenserCamera<B: UsbBackend> {
    device: SenserDevice<B>,
}

impl<B: UsbBackend> SenserCamera<B> {
    /// Wraps an authenticated senser transport.
    pub fn new(device: SenserDevice<B>) -> Self {
        Self { device }
    }

    /// Returns the underlying transport (e.g. to access the debug terminal).
    pub fn device_mut(&mut self) -> &mut SenserDevice<B> {
        &mut self.device
    }

    /// Asks the camera to leave service mode; see [`SenserDevice::stop`].
    pub fn stop(&mut self) {
        self.device.stop();
    }

    /// Returns the wrapped transport.
    pub fn into_inner(self) -> SenserDevice<B> {
        self.device
    }

    fn send_product_info(&mut self, category: u16, command: u16, data: &[u8]) -> Result<Vec<u8>> {
        let mut payload = command_header(category, command).to_vec();
        payload.extend_from_slice(data);
        let (res, out) = self.device.send_packet_buffered(pfunc::PRODUCT_INFO, &payload)?;
        if res != 1 {
            return Err(Error::Senser(format!("senser product info error {res}")));
        }
        Ok(out)
    }

    fn send_adjust_control(&mut self, block: u16, code: u16, data: &[u8]) -> Result<Vec<u8>> {
        let mut payload = command_header(block, code).to_vec();
        payload.extend_from_slice(data);
        let (res, out) = self.device.send_packet_buffered(pfunc::ADJUST_CONTROL, &payload)?;
        if res != 0 {
            return Err(Error::Senser(format!("senser adjust control error {res}")));
        }
        Ok(out)
    }

    fn send_file_control(
        &mut self,
        cmd: u16,
        filename: &str,
        data: &[u8],
        out: &mut dyn Write,
    ) -> Result<()> {
        let name = pad_filename(filename)?;
        let mut payload = Vec::with_capacity(4 + name.len() + data.len());
        payload.extend_from_slice(&cmd.to_le_bytes());
        payload.extend_from_slice(&(name.len() as u16).to_le_bytes());
        payload.extend_from_slice(&name);
        payload.extend_from_slice(data);
        let res = self.device.send_packet(pfunc::FILE_CONTROL, &payload, out)?;
        if res != 1 {
            return Err(Error::Senser(format!("senser file control error {res}")));
        }
        Ok(())
    }

    fn send_memory_dump(
        &mut self,
        base: u32,
        size: u32,
        data: &[u8],
        out: &mut dyn Write,
    ) -> Result<()> {
        let mut payload = Vec::with_capacity(8 + data.len());
        payload.extend_from_slice(&base.to_le_bytes());
        payload.extend_from_slice(&size.to_le_bytes());
        payload.extend_from_slice(data);
        let res = self.device.send_packet(pfunc::MEMORY_DUMP, &payload, out)?;
        if res != 0 && res != 1 {
            return Err(Error::Senser(format!("senser memory dump error {res}")));
        }
        Ok(())
    }

    /// Reads the HASP (hardware protection) block.
    pub fn read_hasp(&mut self) -> Result<Vec<u8>> {
        let (category, command) = PRODUCT_INFO_READ_HASP;
        self.send_product_info(category, command, &[0])
    }

    /// Reads `filename` from the camera into `out`.
    pub fn read_file(&mut self, filename: &str, out: &mut dyn Write) -> Result<()> {
        self.send_file_control(FILE_CONTROL_READ, filename, &[], out)
    }

    /// Reads `filename` into memory.
    pub fn read_file_buffered(&mut self, filename: &str) -> Result<Vec<u8>> {
        let mut out = Vec::new();
        self.read_file(filename, &mut out)?;
        Ok(out)
    }

    /// Writes `data` to `filename` on the camera.
    pub fn write_file(&mut self, filename: &str, data: &[u8]) -> Result<()> {
        self.send_file_control(FILE_CONTROL_WRITE, filename, data, &mut Vec::new())
    }

    /// Deletes `filename` from the camera.
    pub fn delete_file(&mut self, filename: &str) -> Result<()> {
        self.send_file_control(FILE_CONTROL_DELETE, filename, &[], &mut Vec::new())
    }

    /// Reads `size` bytes of raw memory starting at `base` into `out`.
    pub fn read_memory(&mut self, base: u32, size: u32, out: &mut dyn Write) -> Result<()> {
        self.send_memory_dump(base, size, &[], out)
    }

    /// Reads `size` bytes of raw memory starting at `base` into a buffer.
    pub fn read_memory_buffered(&mut self, base: u32, size: u32) -> Result<Vec<u8>> {
        let mut out = Vec::new();
        self.read_memory(base, size, &mut out)?;
        Ok(out)
    }

    /// Writes `data` to raw memory starting at `base`.
    pub fn write_memory(&mut self, base: u32, data: &[u8]) -> Result<()> {
        self.send_memory_dump(base, data.len() as u32, data, &mut Vec::new())
    }

    /// Enables or disables the debug terminal.
    ///
    /// The follow-up adjustment command is best-effort: not every model
    /// implements it, and the reference ignores its failure.
    pub fn set_terminal_enable(&mut self, enable: bool) -> Result<()> {
        let (category, command) = PRODUCT_INFO_TERM_CHANGE;
        self.send_product_info(category, command, &[u8::from(enable)])?;
        let (block, code) = ADJUST_SENSER_TERM_MODE;
        let _ = self.send_adjust_control(block, code, &[if enable { 2 } else { 0 }]);
        Ok(())
    }

    /// Reads backup entry `id`.
    pub fn read_backup(&mut self, id: u32) -> Result<Vec<u8>> {
        let (block, code) = ADJUST_BACKUP_READ;
        self.send_adjust_control(block, code, &id.to_le_bytes())
    }

    /// Writes `data` to backup entry `id`.
    pub fn write_backup(&mut self, id: u32, data: &[u8]) -> Result<()> {
        let (block, code) = ADJUST_BACKUP_WRITE;
        let mut payload = id.to_le_bytes().to_vec();
        payload.extend_from_slice(data);
        self.send_adjust_control(block, code, &payload)?;
        Ok(())
    }

    /// Persists the backup of `subsystem` to flash.
    pub fn save_backup(&mut self, subsystem: u16) -> Result<()> {
        let (block, code) = ADJUST_BACKUP_SAVE;
        self.send_adjust_control(block, code, &subsystem.to_le_bytes())?;
        Ok(())
    }

    /// Writes the backup preset data.
    pub fn set_backup_preset_data(&mut self, protect_mode: u32, data: &[u8]) -> Result<Vec<u8>> {
        let (block, code) = ADJUST_BACKUP_PDT_WRITE;
        let mut payload = protect_mode.to_le_bytes().to_vec();
        payload.extend_from_slice(data);
        self.send_adjust_control(block, code, &payload)
    }

    /// Reads the backup preset data, either from memory or from flash.
    pub fn get_backup_preset_data(&mut self, from_memory: bool) -> Result<Vec<u8>> {
        let (block, code) = ADJUST_BACKUP_PDT_READ;
        self.send_adjust_control(block, code, &[u8::from(from_memory)])
    }

    /// Queries the backup preset data status.
    pub fn get_backup_preset_data_status(&mut self) -> Result<Vec<u8>> {
        let (block, code) = ADJUST_BACKUP_PDT_STAT;
        self.send_adjust_control(block, code, &[])
    }

    /// Diagnostic: records how the camera delivers the backup preset data on
    /// the wire; see [`SenserDevice::trace_packet`].
    pub fn trace_backup_preset_data(&mut self, limit: usize) -> Result<WireTrace> {
        let (block, code) = ADJUST_BACKUP_PDT_READ;
        let mut payload = command_header(block, code).to_vec();
        payload.push(1);
        self.device.trace_packet(pfunc::ADJUST_CONTROL, &payload, limit)
    }

    /// Sets backup ID1 and returns the camera's confirmation byte.
    pub fn set_backup_id1(&mut self, value: bool) -> Result<u8> {
        let (block, code) = ADJUST_BACKUP_ID1;
        let out = self.send_adjust_control(block, code, &[u8::from(value)])?;
        out.first().copied().ok_or(Error::Protocol("empty backup id1 response"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::MockBackend;
    use crate::senser::{PID_SENSER_EXTENDED, SenserEndpoints, SenserPacketHeader};

    const EP_IN: u8 = 0x81;
    const EP_OUT: u8 = 0x02;

    fn endpoints() -> SenserEndpoints {
        SenserEndpoints { ep_in: EP_IN, ep_out: EP_OUT, terminal_in: None, terminal_out: None }
    }

    /// Builds an unpadded reply header carrying `payload` in a second transfer.
    fn queue_reply(backend: &mut MockBackend, sequence: u16, response: u8, payload: &[u8]) {
        let header = SenserPacketHeader {
            size: payload.len() as u32,
            sequence,
            response,
            ..Default::default()
        };
        backend.queue_read(EP_IN, header.pack().to_vec());
        if !payload.is_empty() {
            backend.queue_read(EP_IN, payload.to_vec());
        }
    }

    fn camera(backend: MockBackend) -> SenserCamera<MockBackend> {
        SenserCamera::new(SenserDevice::new(backend, endpoints(), PID_SENSER_EXTENDED))
    }

    /// Payload of the n-th OUT write, with the senser header stripped.
    fn sent_payload(backend: &MockBackend, n: usize) -> &[u8] {
        &backend.writes[n].data[12..]
    }

    #[test]
    fn pads_filenames_to_four_bytes_with_terminator() {
        assert_eq!(pad_filename("a").unwrap(), b"a\0\0\0");
        assert_eq!(pad_filename("abcd").unwrap(), b"abcd\0\0\0\0");
        assert_eq!(pad_filename("abcde").unwrap(), b"abcde\0\0\0");
        assert!(pad_filename("\u{20ac}").is_err());
    }

    #[test]
    fn read_hasp_sends_product_info_command() {
        let mut backend = MockBackend::new();
        queue_reply(&mut backend, 1, 1, b"\xde\xad");
        let mut cam = camera(backend);
        assert_eq!(cam.read_hasp().unwrap(), b"\xde\xad");

        let backend = cam.into_inner().into_inner();
        assert_eq!(backend.writes[0].data[4..6], pfunc::PRODUCT_INFO.to_le_bytes());
        assert_eq!(sent_payload(&backend, 0), b"\x00\x00\x1f\x00\x00");
    }

    #[test]
    fn product_info_error_is_reported() {
        let mut backend = MockBackend::new();
        queue_reply(&mut backend, 1, 5, b"");
        let mut cam = camera(backend);
        assert!(matches!(cam.read_hasp(), Err(Error::Senser(_))));
    }

    #[test]
    fn read_file_frames_padded_name() {
        let mut backend = MockBackend::new();
        queue_reply(&mut backend, 1, 1, b"data");
        let mut cam = camera(backend);
        assert_eq!(cam.read_file_buffered("a.txt").unwrap(), b"data");

        let backend = cam.into_inner().into_inner();
        assert_eq!(backend.writes[0].data[4..6], pfunc::FILE_CONTROL.to_le_bytes());
        // cmd=2, nameSize=8, "a.txt\0\0\0"
        assert_eq!(sent_payload(&backend, 0), b"\x02\x00\x08\x00a.txt\0\0\0");
    }

    #[test]
    fn write_file_appends_payload() {
        let mut backend = MockBackend::new();
        queue_reply(&mut backend, 1, 1, b"");
        let mut cam = camera(backend);
        cam.write_file("f", b"xy").unwrap();

        let backend = cam.into_inner().into_inner();
        assert_eq!(sent_payload(&backend, 0), b"\x01\x00\x04\x00f\0\0\0xy");
    }

    #[test]
    fn read_memory_frames_base_and_size() {
        let mut backend = MockBackend::new();
        queue_reply(&mut backend, 1, 0, b"\x01\x02\x03\x04");
        let mut cam = camera(backend);
        assert_eq!(cam.read_memory_buffered(0x1000, 4).unwrap(), b"\x01\x02\x03\x04");

        let backend = cam.into_inner().into_inner();
        assert_eq!(backend.writes[0].data[4..6], pfunc::MEMORY_DUMP.to_le_bytes());
        assert_eq!(sent_payload(&backend, 0), b"\x00\x10\x00\x00\x04\x00\x00\x00");
    }

    #[test]
    fn memory_dump_accepts_zero_and_one_but_rejects_others() {
        let mut backend = MockBackend::new();
        queue_reply(&mut backend, 1, 2, b"");
        let mut cam = camera(backend);
        assert!(matches!(cam.read_memory_buffered(0, 0), Err(Error::Senser(_))));
    }

    #[test]
    fn write_memory_uses_data_length_as_size() {
        let mut backend = MockBackend::new();
        queue_reply(&mut backend, 1, 1, b"");
        let mut cam = camera(backend);
        cam.write_memory(0x20, b"\xaa\xbb\xcc").unwrap();

        let backend = cam.into_inner().into_inner();
        assert_eq!(sent_payload(&backend, 0), b"\x20\x00\x00\x00\x03\x00\x00\x00\xaa\xbb\xcc");
    }

    #[test]
    fn backup_commands_use_adjust_control_block() {
        let mut backend = MockBackend::new();
        queue_reply(&mut backend, 1, 0, b"\x11");
        queue_reply(&mut backend, 2, 0, b"");
        queue_reply(&mut backend, 3, 0, b"");
        let mut cam = camera(backend);
        assert_eq!(cam.read_backup(0x1234).unwrap(), b"\x11");
        cam.write_backup(1, b"\x99").unwrap();
        cam.save_backup(3).unwrap();

        let backend = cam.into_inner().into_inner();
        assert_eq!(backend.writes[0].data[4..6], pfunc::ADJUST_CONTROL.to_le_bytes());
        assert_eq!(sent_payload(&backend, 0), b"\x03\x06\x01\x00\x34\x12\x00\x00");
        assert_eq!(sent_payload(&backend, 1), b"\x03\x06\x02\x00\x01\x00\x00\x00\x99");
        assert_eq!(sent_payload(&backend, 2), b"\x03\x06\x03\x00\x03\x00");
    }

    #[test]
    fn adjust_control_requires_zero_result() {
        let mut backend = MockBackend::new();
        queue_reply(&mut backend, 1, 1, b"");
        let mut cam = camera(backend);
        assert!(matches!(cam.read_backup(0), Err(Error::Senser(_))));
    }

    #[test]
    fn set_terminal_enable_ignores_failing_adjust_command() {
        let mut backend = MockBackend::new();
        queue_reply(&mut backend, 1, 1, b"");
        // The adjust follow-up fails (result 9) and must be swallowed.
        queue_reply(&mut backend, 2, 9, b"");
        let mut cam = camera(backend);
        cam.set_terminal_enable(true).unwrap();

        let backend = cam.into_inner().into_inner();
        assert_eq!(sent_payload(&backend, 0), b"\x00\x00\xf1\x00\x01");
        assert_eq!(sent_payload(&backend, 1), b"\x01\x06\x1f\x00\x02");
    }

    #[test]
    fn set_backup_id1_returns_first_byte() {
        let mut backend = MockBackend::new();
        queue_reply(&mut backend, 1, 0, b"\x01");
        let mut cam = camera(backend);
        assert_eq!(cam.set_backup_id1(true).unwrap(), 1);
    }
}
