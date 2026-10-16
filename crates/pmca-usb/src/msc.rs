//! USB Mass-Storage Bulk-Only Transport (BBB): CBW / CSW framing.
//!
//! Byte-faithful port of `MscBbbDriver` in `pmca/usb/driver/generic`. A
//! Command Block Wrapper (31 bytes) is written to the OUT endpoint, optional
//! data is exchanged, then a Command Status Wrapper (13 bytes) is read from the
//! IN endpoint.

use crate::{Error, Result, UsbBackend};

/// CBW signature `USBC` (little-endian on the wire, but it is a literal ASCII tag).
const CBW_SIGNATURE: &[u8; 4] = b"USBC";
/// CSW signature `USBS`.
const CSW_SIGNATURE: &[u8; 4] = b"USBS";

/// Length of a Command Block Wrapper.
pub const CBW_SIZE: usize = 31;
/// Length of a Command Status Wrapper.
pub const CSW_SIZE: usize = 13;

/// Maximum SCSI command block length inside a CBW.
const COMMAND_MAX: usize = 16;

/// Data-transfer direction flags for the CBW.
const DIRECTION_WRITE: u8 = 0x00;
const DIRECTION_READ: u8 = 0x80;

/// SCSI REQUEST SENSE opcode.
const MSC_OC_REQUEST_SENSE: u8 = 0x03;
/// SCSI INQUIRY opcode.
const MSC_OC_INQUIRY: u8 = 0x12;

/// Parsed SCSI sense triple `(key, asc, ascq)`.
pub type Sense = (u8, u8, u8);

/// "No error" sense.
pub const SENSE_OK: Sense = (0, 0, 0);

/// SCSI sense key 0x5, ILLEGAL REQUEST.
///
/// A Sony camera answers an external command it does not implement with this
/// key, so it means "this model cannot do that", not "the transfer broke".
const SENSE_KEY_ILLEGAL_REQUEST: u8 = 0x05;

/// Converts a non-OK sense triple into the matching [`Error`].
///
/// Single point of translation for every mass-storage transport, so that an
/// unsupported command is reported the same way regardless of whether it went
/// through Bulk-Only or the Windows SCSI pass-through path.
pub fn sense_error(sense: Sense) -> Error {
    let (key, asc, ascq) = sense;
    if key == SENSE_KEY_ILLEGAL_REQUEST {
        Error::Unsupported
    } else {
        Error::MscSense { key, asc, ascq }
    }
}

/// Builds a 31-byte Command Block Wrapper.
///
/// `command` must be at most 16 bytes; longer input is a programming error and
/// is rejected instead of silently truncated (safe output encoding).
fn build_cbw(
    direction: u8,
    command: &[u8],
    data_len: u32,
    tag: u32,
    lun: u8,
) -> Result<[u8; CBW_SIZE]> {
    if command.is_empty() || command.len() > COMMAND_MAX {
        return Err(Error::Protocol("invalid SCSI command length"));
    }
    let mut cbw = [0u8; CBW_SIZE];
    cbw[0..4].copy_from_slice(CBW_SIGNATURE);
    cbw[4..8].copy_from_slice(&tag.to_le_bytes());
    cbw[8..12].copy_from_slice(&data_len.to_le_bytes());
    cbw[12] = direction;
    cbw[13] = lun;
    cbw[14] = command.len() as u8;
    cbw[15..15 + command.len()].copy_from_slice(command);
    Ok(cbw)
}

/// Parses a Command Status Wrapper, returning its `status` byte.
fn parse_csw(buf: &[u8]) -> Result<u8> {
    if buf.len() < CSW_SIZE {
        return Err(Error::Protocol("short command status wrapper"));
    }
    if &buf[0..4] != CSW_SIGNATURE {
        return Err(Error::Protocol("wrong status signature"));
    }
    Ok(buf[12])
}

/// Parses the SCSI sense triple from an 18-byte REQUEST SENSE response.
fn parse_sense(data: &[u8]) -> Sense {
    let key = data.get(2).copied().unwrap_or(0) & 0x0f;
    let asc = data.get(12).copied().unwrap_or(0);
    let ascq = data.get(13).copied().unwrap_or(0);
    (key, asc, ascq)
}

/// A mass-storage device reachable over Bulk-Only Transport.
pub struct MscDevice<B: UsbBackend> {
    backend: B,
    ep_in: u8,
    ep_out: u8,
    tag: u32,
}

impl<B: UsbBackend> MscDevice<B> {
    /// Creates a driver bound to the given bulk endpoints.
    pub fn new(backend: B, ep_in: u8, ep_out: u8) -> Self {
        Self { backend, ep_in, ep_out, tag: 0 }
    }

    /// Re-initialises the transport right after opening, mirroring the intent
    /// of the reference `MscDevice.reset()`.
    ///
    /// Performs the documented USB Bulk-Only Transport error recovery: a
    /// Bulk-Only Mass Storage Reset (backend `reset`) to resynchronise the
    /// device's command/data/status state machine, followed by clearing the
    /// halt on both bulk pipes. This is required on Windows/WinUSB after the
    /// camera reboots into updater mode or after a previous session left a
    /// transfer half-finished; without it the first real transfer stalls. All
    /// steps are best-effort — a healthy device is unaffected, and any genuine
    /// problem still surfaces on the following real command.
    pub fn reset(&mut self) {
        let _ = self.backend.reset();
        let _ = self.backend.clear_halt(self.ep_out);
        let _ = self.backend.clear_halt(self.ep_in);
    }

    fn next_tag(&mut self) -> u32 {
        self.tag = self.tag.wrapping_add(1);
        self.tag
    }
    fn write_command(&mut self, direction: u8, command: &[u8], data_len: u32) -> Result<()> {
        let tag = self.next_tag();
        let cbw = build_cbw(direction, command, data_len, tag, 0)?;
        self.backend.write(self.ep_out, &cbw)
    }

    /// Reads and validates the CSW; on non-zero status issues REQUEST SENSE.
    ///
    /// Per the USB Bulk-Only Transport spec (6.7.2/6.7.3), a STALL on the
    /// bulk-IN endpoint during the status phase is recoverable: clear the halt
    /// and read the CSW once more. This matters right after the camera reboots
    /// into updater mode, where the first status transfer can arrive stalled.
    fn read_response(&mut self) -> Result<Sense> {
        let csw = match self.backend.read(self.ep_in, CSW_SIZE) {
            Ok(csw) => csw,
            Err(Error::Stall) => {
                self.backend.clear_halt(self.ep_in)?;
                self.backend.read(self.ep_in, CSW_SIZE)?
            }
            Err(e) => return Err(e),
        };
        let status = parse_csw(&csw)?;
        if status != 0 {
            return self.request_sense();
        }
        Ok(SENSE_OK)
    }

    /// Issues SCSI REQUEST SENSE and returns the parsed sense triple.
    pub fn request_sense(&mut self) -> Result<Sense> {
        let size = 18u8;
        let command = [MSC_OC_REQUEST_SENSE, 0, 0, 0, size, 0];
        self.write_command(DIRECTION_READ, &command, size as u32)?;
        let data = self.backend.read(self.ep_in, size as usize)?;
        // Read the terminating CSW; a failure here is fatal.
        let csw = self.backend.read(self.ep_in, CSW_SIZE)?;
        if parse_csw(&csw)? != 0 {
            return Err(Error::MscStatus { status: 1 });
        }
        Ok(parse_sense(&data))
    }

    /// Sends a command with no data phase.
    pub fn send_command(&mut self, command: &[u8]) -> Result<Sense> {
        self.write_command(DIRECTION_WRITE, command, 0)?;
        self.read_response()
    }

    /// Sends a command followed by an OUT data phase.
    ///
    /// A STALL on the data endpoint is an expected Bulk-Only signal: the halt
    /// is cleared and the status wrapper is still read. If the device then
    /// reports success despite the stall, that is treated as an error, matching
    /// the reference driver.
    pub fn send_write_command(&mut self, command: &[u8], data: &[u8]) -> Result<Sense> {
        self.write_command(DIRECTION_WRITE, command, data.len() as u32)?;
        let stalled = match self.backend.write(self.ep_out, data) {
            Ok(()) => false,
            Err(Error::Stall) => {
                self.backend.clear_halt(self.ep_out)?;
                true
            }
            Err(e) => return Err(e),
        };
        let sense = self.read_response()?;
        if stalled && sense == SENSE_OK {
            return Err(Error::Protocol("mass-storage write error after stall"));
        }
        Ok(sense)
    }

    /// Sends a command and reads `size` bytes in the IN data phase.
    ///
    /// A STALL on the data endpoint is cleared and the status wrapper is still
    /// read (the returned data is empty in that case).
    pub fn send_read_command(&mut self, command: &[u8], size: usize) -> Result<(Sense, Vec<u8>)> {
        self.write_command(DIRECTION_READ, command, size as u32)?;
        let (stalled, data) = match self.backend.read(self.ep_in, size) {
            Ok(data) => (false, data),
            Err(Error::Stall) => {
                self.backend.clear_halt(self.ep_in)?;
                (true, Vec::new())
            }
            Err(e) => return Err(e),
        };
        let sense = self.read_response()?;
        if stalled && sense == SENSE_OK {
            return Err(Error::Protocol("mass-storage read error after stall"));
        }
        Ok((sense, data))
    }

    /// SCSI INQUIRY: returns the device's vendor/model strings.
    ///
    /// Strings are decoded as Latin-1 and trimmed of spaces/NULs, matching the
    /// original tool's explicit input decoding.
    pub fn get_device_info(&mut self) -> Result<crate::sony::MscDeviceInfo> {
        // First read the standard 5-byte header to learn the additional length.
        let inquiry = |size: u8| [MSC_OC_INQUIRY, 0, 0, 0, size, 0];
        let (sense, header) = self.send_read_command(&inquiry(5), 5)?;
        check_sense(sense)?;
        let additional = *header.get(4).ok_or(Error::Protocol("short inquiry header"))?;
        let total = 5usize + additional as usize;
        let (sense, data) = self.send_read_command(&inquiry(total as u8), total)?;
        check_sense(sense)?;
        if data.len() < 32 {
            return Err(Error::Protocol("short inquiry response"));
        }
        Ok(crate::sony::MscDeviceInfo {
            manufacturer: latin1_trim(&data[8..16]),
            model: latin1_trim(&data[16..32]),
        })
    }

    /// Consumes the driver, returning the underlying backend.
    pub fn into_backend(self) -> B {
        self.backend
    }
}

fn check_sense(sense: Sense) -> Result<()> {
    if sense == SENSE_OK { Ok(()) } else { Err(sense_error(sense)) }
}

/// Decodes bytes as Latin-1 and trims trailing spaces and NULs.
fn latin1_trim(bytes: &[u8]) -> String {
    let s: String = bytes.iter().map(|&b| b as char).collect();
    s.trim_matches(|c| c == ' ' || c == '\0').to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::MockBackend;

    const EP_IN: u8 = 0x81;
    const EP_OUT: u8 = 0x02;

    fn csw(status: u8) -> Vec<u8> {
        let mut b = vec![0u8; CSW_SIZE];
        b[0..4].copy_from_slice(CSW_SIGNATURE);
        b[12] = status;
        b
    }

    #[test]
    fn cbw_layout_is_31_bytes_and_well_formed() {
        let cbw = build_cbw(DIRECTION_READ, &[0x12, 0, 0, 0, 5, 0], 5, 7, 0).unwrap();
        assert_eq!(cbw.len(), CBW_SIZE);
        assert_eq!(&cbw[0..4], b"USBC");
        assert_eq!(u32::from_le_bytes(cbw[4..8].try_into().unwrap()), 7);
        assert_eq!(u32::from_le_bytes(cbw[8..12].try_into().unwrap()), 5);
        assert_eq!(cbw[12], DIRECTION_READ);
        assert_eq!(cbw[14], 6);
    }

    #[test]
    fn rejects_oversized_command() {
        assert!(build_cbw(DIRECTION_WRITE, &[0u8; 17], 0, 0, 0).is_err());
        assert!(build_cbw(DIRECTION_WRITE, &[], 0, 0, 0).is_err());
    }

    #[test]
    fn inquiry_roundtrip_via_mock() {
        // Build the two INQUIRY responses the device would return.
        let mut header = vec![0u8; 5];
        header[4] = 27; // additional length -> total 32
        let mut full = vec![0u8; 32];
        full[4] = 27;
        full[8..16].copy_from_slice(b"Sony    ");
        full[16..32].copy_from_slice(b"DSC             ");

        let mut backend = MockBackend::new();
        // send_read_command(5): data then CSW
        backend.queue_read(EP_IN, header);
        backend.queue_read(EP_IN, csw(0));
        // send_read_command(32): data then CSW
        backend.queue_read(EP_IN, full);
        backend.queue_read(EP_IN, csw(0));

        let mut dev = MscDevice::new(backend, EP_IN, EP_OUT);
        let info = dev.get_device_info().unwrap();
        assert_eq!(info.manufacturer, "Sony");
        assert_eq!(info.model, "DSC");
    }

    #[test]
    fn nonzero_status_triggers_request_sense() {
        let mut backend = MockBackend::new();
        // send_command -> CSW status 1
        backend.queue_read(EP_IN, csw(1));
        // request_sense -> 18 bytes sense (key=2) then CSW ok
        let mut sense = vec![0u8; 18];
        sense[2] = 0x02;
        sense[12] = 0xff;
        sense[13] = 0xff;
        backend.queue_read(EP_IN, sense);
        backend.queue_read(EP_IN, csw(0));

        let mut dev = MscDevice::new(backend, EP_IN, EP_OUT);
        let result = dev.send_command(&[0x00, 0, 0, 0, 0, 0]).unwrap();
        assert_eq!(result, (0x02, 0xff, 0xff));
    }

    /// A camera that lacks a feature answers with ILLEGAL REQUEST. Reporting
    /// that as a generic storage failure sends the user hunting for a broken
    /// cable, so the distinction is worth pinning down.
    #[test]
    fn an_illegal_request_is_reported_as_unsupported() {
        assert!(matches!(sense_error((0x05, 0x20, 0x00)), Error::Unsupported));
    }

    #[test]
    fn any_other_sense_keeps_its_full_triple() {
        match sense_error((0x03, 0x11, 0x04)) {
            Error::MscSense { key, asc, ascq } => assert_eq!((key, asc, ascq), (0x03, 0x11, 0x04)),
            other => panic!("expected a sense error, got {other:?}"),
        }
    }
}
