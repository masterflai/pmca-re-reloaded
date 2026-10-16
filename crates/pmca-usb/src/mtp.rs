//! PTP/MTP container framing and a Bulk PTP/MTP device.
//!
//! Byte-faithful port of `MtpDriver` / `MtpDevice` in `pmca/usb`. PTP transfers
//! consist of 12-byte little-endian container headers followed by a payload;
//! command parameters are 32-bit little-endian words.

use crate::{Error, Result, UsbBackend};

/// Length of a PTP container header.
pub const PTP_HEADER_SIZE: usize = 12;

/// Maximum single bulk packet length used when reading containers.
const MAX_PKG_LEN: usize = 512;

/// PTP container types.
pub mod container_type {
    /// Command block.
    pub const COMMAND: u16 = 1;
    /// Data block.
    pub const DATA: u16 = 2;
    /// Response block.
    pub const RESPONSE: u16 = 3;
}

/// PTP response / operation codes used by the base MTP device.
pub mod ptp {
    /// GetDeviceInfo.
    pub const OC_GET_DEVICE_INFO: u16 = 0x1001;
    /// OpenSession.
    pub const OC_OPEN_SESSION: u16 = 0x1002;
    /// CloseSession.
    pub const OC_CLOSE_SESSION: u16 = 0x1003;

    /// OK.
    pub const RC_OK: u16 = 0x2001;
    /// Session not open.
    pub const RC_SESSION_NOT_OPEN: u16 = 0x2003;
    /// Operation not supported.
    pub const RC_OPERATION_NOT_SUPPORTED: u16 = 0x2005;
    /// Parameter not supported.
    pub const RC_PARAMETER_NOT_SUPPORTED: u16 = 0x2006;
    /// Device property not supported.
    pub const RC_DEVICE_PROP_NOT_SUPPORTED: u16 = 0x2009;
    /// Device busy.
    pub const RC_DEVICE_BUSY: u16 = 0x2019;
    /// Session already open.
    pub const RC_SESSION_ALREADY_OPENED: u16 = 0x201e;
}

/// Sony reconnect (switch-to-MSC) operation code.
const SONY_REQ_RECONNECT: u16 = 0x9282;

/// The four PTP command shapes every MTP transport must provide.
///
/// [`MtpDevice`] implements this over raw bulk endpoints. On Windows the
/// app-install and MTP interfaces are claimed by the `WUDFWpdMtp` class driver,
/// so bulk endpoints are unavailable; [`crate::wpd::WpdDevice`] implements the
/// same four shapes through the WPD MTP extension commands instead. Everything
/// above this trait (Sony external commands, the application-install proxy) is
/// written against it and therefore works over either transport.
pub trait PtpCommands {
    /// Sends a command with no data phase, returning the response code.
    fn send_command(&mut self, code: u16, args: &[u32]) -> Result<u16>;

    /// Sends a command with an OUT data phase, returning the response code.
    fn send_write_command(&mut self, code: u16, args: &[u32], data: &[u8]) -> Result<u16>;

    /// Sends a command with an IN data phase, returning `(response, data)`.
    fn send_read_command(&mut self, code: u16, args: &[u32]) -> Result<(u16, Vec<u8>)>;

    /// Like [`send_read_command`](Self::send_read_command) but tolerates a
    /// missing data phase, returning an empty payload in that case.
    fn send_read_command_opt(&mut self, code: u16, args: &[u32]) -> Result<(u16, Vec<u8>)>;
}

impl<B: UsbBackend> PtpCommands for MtpDevice<B> {
    fn send_command(&mut self, code: u16, args: &[u32]) -> Result<u16> {
        MtpDevice::send_command(self, code, args)
    }

    fn send_write_command(&mut self, code: u16, args: &[u32], data: &[u8]) -> Result<u16> {
        MtpDevice::send_write_command(self, code, args, data)
    }

    fn send_read_command(&mut self, code: u16, args: &[u32]) -> Result<(u16, Vec<u8>)> {
        MtpDevice::send_read_command(self, code, args)
    }

    fn send_read_command_opt(&mut self, code: u16, args: &[u32]) -> Result<(u16, Vec<u8>)> {
        MtpDevice::send_read_command_opt(self, code, args)
    }
}

/// A parsed PTP container.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PtpContainer {
    /// Container type (see [`container_type`]).
    pub container_type: u16,
    /// Operation / response code.
    pub code: u16,
    /// Transaction id.
    pub transaction: u32,
    /// Payload following the 12-byte header.
    pub payload: Vec<u8>,
}

/// Serializes a PTP container to bytes.
pub fn pack_container(container_type: u16, code: u16, transaction: u32, payload: &[u8]) -> Vec<u8> {
    let size = (PTP_HEADER_SIZE + payload.len()) as u32;
    let mut out = Vec::with_capacity(PTP_HEADER_SIZE + payload.len());
    out.extend_from_slice(&size.to_le_bytes());
    out.extend_from_slice(&container_type.to_le_bytes());
    out.extend_from_slice(&code.to_le_bytes());
    out.extend_from_slice(&transaction.to_le_bytes());
    out.extend_from_slice(payload);
    out
}

/// Parses a PTP container header, returning the container and its declared size.
fn parse_header(buf: &[u8]) -> Result<(u32, u16, u16, u32)> {
    if buf.len() < PTP_HEADER_SIZE {
        return Err(Error::Protocol("short PTP header"));
    }
    let size = u32::from_le_bytes(buf[0..4].try_into().unwrap());
    let container_type = u16::from_le_bytes(buf[4..6].try_into().unwrap());
    let code = u16::from_le_bytes(buf[6..8].try_into().unwrap());
    let transaction = u32::from_le_bytes(buf[8..12].try_into().unwrap());
    Ok((size, container_type, code, transaction))
}

/// Encodes PTP command parameters as little-endian 32-bit words.
fn pack_args(args: &[u32]) -> Vec<u8> {
    let mut out = Vec::with_capacity(args.len() * 4);
    for a in args {
        out.extend_from_slice(&a.to_le_bytes());
    }
    out
}

/// A PTP/MTP device reachable over two bulk endpoints.
pub struct MtpDevice<B: UsbBackend> {
    backend: B,
    ep_in: u8,
    ep_out: u8,
    transaction: u32,
}

impl<B: UsbBackend> MtpDevice<B> {
    /// Creates a driver bound to the given bulk endpoints.
    pub fn new(backend: B, ep_in: u8, ep_out: u8) -> Self {
        Self { backend, ep_in, ep_out, transaction: 0 }
    }

    fn write_ptp(
        &mut self,
        container_type: u16,
        code: u16,
        transaction: u32,
        payload: &[u8],
    ) -> Result<()> {
        let packet = pack_container(container_type, code, transaction, payload);
        self.backend.write(self.ep_out, &packet)
    }

    /// Reads a full PTP container (following continuation reads if needed).
    fn read_ptp(&mut self) -> Result<PtpContainer> {
        let mut data = Vec::new();
        // Skip zero-length packets, as the reference driver does.
        while data.is_empty() {
            data = self.backend.read(self.ep_in, MAX_PKG_LEN)?;
        }
        let (size, container_type, code, transaction) = parse_header(&data)?;
        let size = size as usize;
        if size < PTP_HEADER_SIZE {
            return Err(Error::Protocol("PTP size smaller than header"));
        }
        if size > data.len() {
            let mut rest = self.backend.read(self.ep_in, size - data.len())?;
            data.append(&mut rest);
        }
        let end = size.min(data.len());
        let payload = data[PTP_HEADER_SIZE..end].to_vec();
        Ok(PtpContainer { container_type, code, transaction, payload })
    }

    fn read_data(&mut self) -> Result<Vec<u8>> {
        let c = self.read_ptp()?;
        if c.container_type != container_type::DATA {
            return Err(Error::Protocol("expected PTP data phase"));
        }
        Ok(c.payload)
    }

    fn read_response(&mut self) -> Result<u16> {
        let c = self.read_ptp()?;
        if c.container_type != container_type::RESPONSE {
            return Err(Error::Protocol("expected PTP response phase"));
        }
        Ok(c.code)
    }

    fn write_initial_command(&mut self, code: u16, args: &[u32]) -> Result<()> {
        self.transaction = self.transaction.wrapping_add(1);
        let t = self.transaction;
        self.write_ptp(container_type::COMMAND, code, t, &pack_args(args))
    }

    /// Sends a command with no data phase, returning the response code.
    pub fn send_command(&mut self, code: u16, args: &[u32]) -> Result<u16> {
        self.write_initial_command(code, args)?;
        self.read_response()
    }

    /// Sends a command with an OUT data phase, returning the response code.
    pub fn send_write_command(&mut self, code: u16, args: &[u32], data: &[u8]) -> Result<u16> {
        self.write_initial_command(code, args)?;
        let t = self.transaction;
        self.write_ptp(container_type::DATA, code, t, data)?;
        self.read_response()
    }

    /// Sends a command with an IN data phase, returning `(response, data)`.
    pub fn send_read_command(&mut self, code: u16, args: &[u32]) -> Result<(u16, Vec<u8>)> {
        self.write_initial_command(code, args)?;
        let data = self.read_data()?;
        let response = self.read_response()?;
        Ok((response, data))
    }

    /// Like [`send_read_command`](Self::send_read_command) but tolerates a
    /// missing data phase: if the device answers with a RESPONSE container
    /// directly (no DATA), an empty payload is returned.
    ///
    /// The Sony application-install proxy uses this: `GetProxyMessage` may
    /// answer `NoData` when no message is pending.
    pub fn send_read_command_opt(&mut self, code: u16, args: &[u32]) -> Result<(u16, Vec<u8>)> {
        self.write_initial_command(code, args)?;
        let c = self.read_ptp()?;
        match c.container_type {
            container_type::DATA => {
                let response = self.read_response()?;
                Ok((response, c.payload))
            }
            container_type::RESPONSE => Ok((c.code, Vec::new())),
            _ => Err(Error::Protocol("expected PTP data or response phase")),
        }
    }

    /// Opens an MTP session (accepting an already-open session).
    pub fn open_session(&mut self, id: u32) -> Result<()> {
        let code = self.send_command(ptp::OC_OPEN_SESSION, &[id])?;
        if code == ptp::RC_OK || code == ptp::RC_SESSION_ALREADY_OPENED {
            Ok(())
        } else {
            Err(Error::Protocol("MTP OpenSession failed"))
        }
    }

    /// Fetches and parses the device info descriptor.
    pub fn get_device_info(&mut self) -> Result<crate::sony::MtpDeviceInfo> {
        let (code, data) = self.send_read_command(ptp::OC_GET_DEVICE_INFO, &[])?;
        if code != ptp::RC_OK {
            return Err(Error::Protocol("MTP GetDeviceInfo failed"));
        }
        parse_device_info(&data)
    }

    /// Consumes the driver, returning the underlying backend.
    pub fn into_backend(self) -> B {
        self.backend
    }
}

/// Asks an MTP-mode camera to reconnect as a mass-storage device.
///
/// Sony's vendor reconnect request (`0x9282`, parameter 0) makes the camera
/// drop off the bus and re-enumerate in mass-storage mode. The change is
/// transient: the USB mode configured in the camera's own menu is untouched, so
/// the device returns to it on the next power cycle or replug. There is no
/// counterpart command for the opposite direction.
///
/// Free function rather than an inherent method so it works over both the
/// bulk-endpoint [`MtpDevice`] and the Windows WPD transport.
pub fn switch_to_msc<D: PtpCommands + ?Sized>(dev: &mut D) -> Result<()> {
    let code = dev.send_command(SONY_REQ_RECONNECT, &[0])?;
    if code == ptp::RC_OK { Ok(()) } else { Err(Error::Protocol("Sony reconnect request failed")) }
}

/// Reads a PTP string (`u8` length prefix, then UTF-16LE, NUL-terminated).
fn parse_string(data: &[u8], offset: usize) -> Result<(usize, String)> {
    let length = *data.get(offset).ok_or(Error::Protocol("short PTP string"))? as usize;
    let start = offset + 1;
    let end = start + 2 * length;
    if end > data.len() {
        return Err(Error::Protocol("PTP string out of bounds"));
    }
    // Decode UTF-16LE, dropping the trailing NUL terminator if present.
    let units: Vec<u16> =
        data[start..end].as_chunks::<2>().0.iter().map(|c| u16::from_le_bytes(*c)).collect();
    let trimmed = if units.last() == Some(&0) { &units[..units.len() - 1] } else { &units[..] };
    let s =
        String::from_utf16(trimmed).map_err(|_| Error::Protocol("invalid UTF-16 in PTP string"))?;
    Ok((end, s))
}

/// Reads a PTP array of 16-bit values (`u32` length prefix).
fn parse_u16_array(data: &[u8], offset: usize) -> Result<(usize, Vec<u16>)> {
    if offset + 4 > data.len() {
        return Err(Error::Protocol("short PTP array length"));
    }
    let length = u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap()) as usize;
    let start = offset + 4;
    let end = start + 2 * length;
    if end > data.len() {
        return Err(Error::Protocol("PTP array out of bounds"));
    }
    let values =
        data[start..end].as_chunks::<2>().0.iter().map(|c| u16::from_le_bytes(*c)).collect();
    Ok((end, values))
}

/// Parses a PTP DeviceInfo dataset into an [`MtpDeviceInfo`](crate::sony::MtpDeviceInfo).
pub fn parse_device_info(data: &[u8]) -> Result<crate::sony::MtpDeviceInfo> {
    // Skip StandardVersion(2) + VendorExtensionID(4) + VendorExtensionVersion(2).
    let mut offset = 8usize;
    let (o, vendor_extension) = parse_string(data, offset)?;
    offset = o + 2; // skip FunctionalMode(2)

    let (o, operations_supported) = parse_u16_array(data, offset)?;
    offset = o;
    let (o, _events) = parse_u16_array(data, offset)?;
    offset = o;
    let (o, _device_props) = parse_u16_array(data, offset)?;
    offset = o;
    let (o, _capture_formats) = parse_u16_array(data, offset)?;
    offset = o;
    let (o, _image_formats) = parse_u16_array(data, offset)?;
    offset = o;

    let (o, manufacturer) = parse_string(data, offset)?;
    offset = o;
    let (o, model) = parse_string(data, offset)?;
    offset = o;
    let (o, _version) = parse_string(data, offset)?;
    offset = o;
    let (_o, serial) = parse_string(data, offset)?;

    Ok(crate::sony::MtpDeviceInfo {
        manufacturer,
        model,
        serial_number: serial,
        operations_supported,
        vendor_extension,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::MockBackend;

    const EP_IN: u8 = 0x81;
    const EP_OUT: u8 = 0x02;

    #[test]
    fn container_roundtrip() {
        let packet = pack_container(container_type::COMMAND, 0x1002, 7, &[1, 0, 0, 0]);
        assert_eq!(packet.len(), PTP_HEADER_SIZE + 4);
        let (size, ty, code, tx) = parse_header(&packet).unwrap();
        assert_eq!(size as usize, packet.len());
        assert_eq!(ty, container_type::COMMAND);
        assert_eq!(code, 0x1002);
        assert_eq!(tx, 7);
    }

    #[test]
    fn send_command_reads_response_code() {
        let mut backend = MockBackend::new();
        // Response container, code = RC_OK.
        backend.queue_read(EP_IN, pack_container(container_type::RESPONSE, ptp::RC_OK, 1, &[]));
        let mut dev = MtpDevice::new(backend, EP_IN, EP_OUT);
        assert_eq!(dev.send_command(ptp::OC_CLOSE_SESSION, &[]).unwrap(), ptp::RC_OK);

        // First outgoing write must be a COMMAND container with a bumped transaction.
        let backend = dev.into_backend();
        let (_, ty, code, tx) = parse_header(&backend.writes[0].data).unwrap();
        assert_eq!(ty, container_type::COMMAND);
        assert_eq!(code, ptp::OC_CLOSE_SESSION);
        assert_eq!(tx, 1);
    }

    #[test]
    fn send_read_command_returns_data_then_response() {
        let mut backend = MockBackend::new();
        backend.queue_read(EP_IN, pack_container(container_type::DATA, 0x1001, 1, &[0xaa, 0xbb]));
        backend.queue_read(EP_IN, pack_container(container_type::RESPONSE, ptp::RC_OK, 1, &[]));
        let mut dev = MtpDevice::new(backend, EP_IN, EP_OUT);
        let (rc, data) = dev.send_read_command(ptp::OC_GET_DEVICE_INFO, &[]).unwrap();
        assert_eq!(rc, ptp::RC_OK);
        assert_eq!(data, vec![0xaa, 0xbb]);
    }

    #[test]
    fn open_session_accepts_already_open() {
        let mut backend = MockBackend::new();
        backend.queue_read(
            EP_IN,
            pack_container(container_type::RESPONSE, ptp::RC_SESSION_ALREADY_OPENED, 1, &[]),
        );
        let mut dev = MtpDevice::new(backend, EP_IN, EP_OUT);
        assert!(dev.open_session(1).is_ok());
    }

    fn ptp_string(s: &str) -> Vec<u8> {
        let mut units: Vec<u16> = s.encode_utf16().collect();
        units.push(0); // NUL terminator
        let mut out = vec![units.len() as u8];
        for u in units {
            out.extend_from_slice(&u.to_le_bytes());
        }
        out
    }

    fn ptp_u16_array(values: &[u16]) -> Vec<u8> {
        let mut out = (values.len() as u32).to_le_bytes().to_vec();
        for v in values {
            out.extend_from_slice(&v.to_le_bytes());
        }
        out
    }

    #[test]
    fn device_info_parse() {
        let mut data = vec![0u8; 8]; // version + vendor ext id/version
        data.extend(ptp_string("")); // vendorExtension
        data.extend_from_slice(&[0, 0]); // functional mode
        data.extend(ptp_u16_array(&[0x9280, 0x9281, 0x9282])); // operations
        data.extend(ptp_u16_array(&[])); // events
        data.extend(ptp_u16_array(&[])); // device props
        data.extend(ptp_u16_array(&[])); // capture formats
        data.extend(ptp_u16_array(&[])); // image formats
        data.extend(ptp_string("Sony Corporation")); // manufacturer
        data.extend(ptp_string("ILCE-5100")); // model
        data.extend(ptp_string("1.00")); // version
        data.extend(ptp_string("0123456789")); // serial

        let info = parse_device_info(&data).unwrap();
        assert_eq!(info.manufacturer, "Sony Corporation");
        assert_eq!(info.model, "ILCE-5100");
        assert_eq!(info.serial_number, "0123456789");
        assert_eq!(info.operations_supported, vec![0x9280, 0x9281, 0x9282]);
        assert!(info.vendor_extension.is_empty());
        // And it is recognized as a Sony ext-cmd camera.
        assert!(crate::sony::is_sony_mtp_camera(&info));
    }
}
