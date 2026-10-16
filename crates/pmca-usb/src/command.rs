//! Transport-agnostic Sony external-command channel.
//!
//! Both the mass-storage ([`crate::msc::MscDevice`]) and MTP
//! ([`crate::mtp::MtpDevice`]) transports can carry Sony "external commands".
//! [`SonyExtCmdTransport`] hides that difference so the higher-level command
//! objects ([`crate::sony_extcmd`], [`crate::sony_updater`]) work over either.

use crate::msc::{MscDevice, Sense};
use crate::mtp::{MtpDevice, PtpCommands, ptp};
use crate::{Error, Result, UsbBackend};

/// Sends a raw Sony external command and returns the response payload.
pub trait SonyExtCmdTransport {
    /// Sends command `cmd` with `data`, then reads up to `buffer_size` bytes.
    ///
    /// When `buffer_size` is `0` no read phase is performed and an empty vector
    /// is returned.
    fn send_sony_ext_command(
        &mut self,
        cmd: u32,
        data: &[u8],
        buffer_size: usize,
    ) -> Result<Vec<u8>>;
}

/// Forwarding impl so a mutable reference can stand in for a transport (lets
/// the interactive shell borrow a transport owned by an updater session).
impl<T: SonyExtCmdTransport + ?Sized> SonyExtCmdTransport for &mut T {
    fn send_sony_ext_command(
        &mut self,
        cmd: u32,
        data: &[u8],
        buffer_size: usize,
    ) -> Result<Vec<u8>> {
        (**self).send_sony_ext_command(cmd, data, buffer_size)
    }
}

/// Forwarding impl so a boxed, type-erased transport can be used uniformly by
/// the mode-agnostic orchestration code (MSC vs MTP resolved at open time).
impl SonyExtCmdTransport for Box<dyn SonyExtCmdTransport> {
    fn send_sony_ext_command(
        &mut self,
        cmd: u32,
        data: &[u8],
        buffer_size: usize,
    ) -> Result<Vec<u8>> {
        (**self).send_sony_ext_command(cmd, data, buffer_size)
    }
}

/// SCSI opcode carrying a Sony external command in mass-storage mode.
const MSC_OC_EXT_CMD: u8 = 0x7a;
/// Sense value meaning "device busy, retry".
const MSC_SENSE_DEVICE_BUSY: Sense = (0x9, 0x81, 0x81);

/// Upper bound on device-busy retries, to avoid an unbounded loop on a
/// misbehaving/hostile device (the reference implementation looped forever).
const MAX_BUSY_RETRIES: usize = 10_000;

fn msc_ext_command(cmd: u32) -> [u8; 12] {
    let mut c = [0u8; 12];
    c[0] = MSC_OC_EXT_CMD;
    c[1..5].copy_from_slice(&cmd.to_le_bytes());
    c
}

/// A SCSI command channel that carries Sony external commands framed with the
/// `0x7a` opcode.
///
/// This abstracts the two mass-storage transports that share identical
/// SCSI-level semantics but different plumbing: the raw Bulk-Only (CBW/CSW)
/// [`MscDevice`] over WinUSB, and the Windows `IOCTL_SCSI_PASS_THROUGH_DIRECT`
/// path (`crate::scsi_passthrough`) that lets the OS mass-storage stack do the
/// framing. Both drive the exact same device-busy retry loop in
/// [`send_msc_ext_command`].
pub(crate) trait ScsiExtCmdChannel {
    /// Sends a SCSI command with an OUT data phase, returning the sense triple.
    fn scsi_write(&mut self, command: &[u8], data: &[u8]) -> Result<Sense>;
    /// Sends a SCSI command with an IN data phase of `size` bytes.
    fn scsi_read(&mut self, command: &[u8], size: usize) -> Result<(Sense, Vec<u8>)>;
}

impl<B: UsbBackend> ScsiExtCmdChannel for MscDevice<B> {
    fn scsi_write(&mut self, command: &[u8], data: &[u8]) -> Result<Sense> {
        self.send_write_command(command, data)
    }
    fn scsi_read(&mut self, command: &[u8], size: usize) -> Result<(Sense, Vec<u8>)> {
        self.send_read_command(command, size)
    }
}

/// Runs a Sony external command over any mass-storage SCSI channel.
///
/// Writes the request, then (unless `buffer_size` is `0`) reads the response,
/// retrying while the device reports the "busy, retry" sense. Shared by the
/// WinUSB Bulk-Only transport and the Windows SCSI-passthrough transport.
pub(crate) fn send_msc_ext_command<C: ScsiExtCmdChannel>(
    channel: &mut C,
    cmd: u32,
    data: &[u8],
    buffer_size: usize,
) -> Result<Vec<u8>> {
    let command = msc_ext_command(cmd);

    let mut retries = 0;
    loop {
        let sense = channel.scsi_write(&command, data)?;
        if sense != MSC_SENSE_DEVICE_BUSY {
            if sense != crate::msc::SENSE_OK {
                return Err(crate::msc::sense_error(sense));
            }
            break;
        }
        retries += 1;
        if retries > MAX_BUSY_RETRIES {
            return Err(Error::Protocol("device stayed busy (write phase)"));
        }
    }

    if buffer_size == 0 {
        return Ok(Vec::new());
    }

    retries = 0;
    loop {
        let (sense, data) = channel.scsi_read(&command, buffer_size)?;
        if sense != MSC_SENSE_DEVICE_BUSY {
            if sense != crate::msc::SENSE_OK {
                return Err(crate::msc::sense_error(sense));
            }
            return Ok(data);
        }
        retries += 1;
        if retries > MAX_BUSY_RETRIES {
            return Err(Error::Protocol("device stayed busy (read phase)"));
        }
    }
}

impl<B: UsbBackend> SonyExtCmdTransport for MscDevice<B> {
    fn send_sony_ext_command(
        &mut self,
        cmd: u32,
        data: &[u8],
        buffer_size: usize,
    ) -> Result<Vec<u8>> {
        send_msc_ext_command(self, cmd, data, buffer_size)
    }
}

/// Sony ext-cmd operation codes in MTP mode.
const PTP_OC_SONY_EXT_CMD_WRITE: u16 = 0x9280;
const PTP_OC_SONY_EXT_CMD_READ: u16 = 0x9281;

/// Translates a non-OK PTP response code into the matching [`Error`].
///
/// A camera that simply lacks a feature answers with one of the "not
/// supported" codes. That is a model boundary, not a fault, and it has to be
/// reported the way [`crate::msc::sense_error`] reports the mass-storage
/// equivalent (SCSI ILLEGAL REQUEST) — otherwise the identical refusal reads as
/// a clean limitation over one transport and as a broken transfer over the
/// other, which is what a camera in MTP mode actually did.
///
/// Any other code is carried verbatim: it is the only thing the camera said
/// about the failure.
fn ptp_response_error(phase: &'static str, code: u16) -> Error {
    match code {
        ptp::RC_OPERATION_NOT_SUPPORTED
        | ptp::RC_PARAMETER_NOT_SUPPORTED
        | ptp::RC_DEVICE_PROP_NOT_SUPPORTED => Error::Unsupported,
        _ => Error::PtpResponse { phase, code },
    }
}

impl<B: UsbBackend> SonyExtCmdTransport for MtpDevice<B> {
    fn send_sony_ext_command(
        &mut self,
        cmd: u32,
        data: &[u8],
        buffer_size: usize,
    ) -> Result<Vec<u8>> {
        send_ptp_ext_command(self, cmd, data, buffer_size)
    }
}

/// Runs one Sony external command over any PTP transport.
///
/// Shared by the bulk-endpoint [`MtpDevice`] and the Windows WPD transport, so
/// the two device-busy retry loops exist exactly once.
pub(crate) fn send_ptp_ext_command<D: PtpCommands + ?Sized>(
    dev: &mut D,
    cmd: u32,
    data: &[u8],
    buffer_size: usize,
) -> Result<Vec<u8>> {
    let mut retries = 0;
    loop {
        let rc = dev.send_write_command(PTP_OC_SONY_EXT_CMD_WRITE, &[cmd], data)?;
        if rc != ptp::RC_DEVICE_BUSY {
            if rc != ptp::RC_OK {
                return Err(ptp_response_error("write", rc));
            }
            break;
        }
        retries += 1;
        if retries > MAX_BUSY_RETRIES {
            return Err(Error::Protocol("device stayed busy (write phase)"));
        }
    }

    if buffer_size == 0 {
        return Ok(Vec::new());
    }

    retries = 0;
    loop {
        let (rc, data) = dev.send_read_command(PTP_OC_SONY_EXT_CMD_READ, &[cmd])?;
        if rc != ptp::RC_DEVICE_BUSY {
            if rc != ptp::RC_OK {
                return Err(ptp_response_error("read", rc));
            }
            return Ok(data);
        }
        retries += 1;
        if retries > MAX_BUSY_RETRIES {
            return Err(Error::Protocol("device stayed busy (read phase)"));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::MockBackend;
    use crate::mtp::{container_type, pack_container};

    #[test]
    fn mtp_ext_command_roundtrip() {
        const EP_IN: u8 = 0x81;
        const EP_OUT: u8 = 0x02;
        let mut backend = MockBackend::new();
        // write phase response
        backend.queue_read(EP_IN, pack_container(container_type::RESPONSE, ptp::RC_OK, 1, &[]));
        // read phase data + response
        backend.queue_read(
            EP_IN,
            pack_container(container_type::DATA, PTP_OC_SONY_EXT_CMD_READ, 2, &[0x11, 0x22]),
        );
        backend.queue_read(EP_IN, pack_container(container_type::RESPONSE, ptp::RC_OK, 2, &[]));

        let mut dev = MtpDevice::new(backend, EP_IN, EP_OUT);
        let out = dev.send_sony_ext_command(5, &[0xde, 0xad], 0x100).unwrap();
        assert_eq!(out, vec![0x11, 0x22]);
    }

    /// A model that lacks a feature must look the same over both transports.
    ///
    /// The mass-storage path reports SCSI ILLEGAL REQUEST as
    /// [`Error::Unsupported`]; MTP has to reach the same conclusion from the
    /// PTP response code, or the user is told the camera "sent an unexpected
    /// response" when it merely said no.
    #[test]
    fn a_not_supported_response_code_is_reported_as_unsupported() {
        for rc in [
            ptp::RC_OPERATION_NOT_SUPPORTED,
            ptp::RC_PARAMETER_NOT_SUPPORTED,
            ptp::RC_DEVICE_PROP_NOT_SUPPORTED,
        ] {
            assert!(
                matches!(ptp_response_error("write", rc), Error::Unsupported),
                "{rc:#06x} should be reported as unsupported"
            );
        }
        assert_eq!(
            crate::msc::sense_error((0x05, 0x20, 0x00)).code(),
            ptp_response_error("write", ptp::RC_OPERATION_NOT_SUPPORTED).code(),
            "the two transports must report a refusal identically"
        );
    }

    /// Any other rejection has to keep the code — it is all the camera said.
    #[test]
    fn any_other_response_code_is_carried_verbatim() {
        let error = ptp_response_error("read", 0x2002);
        assert!(matches!(error, Error::PtpResponse { phase: "read", code: 0x2002 }));
        assert_eq!(error.detail().as_deref(), Some("read 0x2002"));
    }
}
