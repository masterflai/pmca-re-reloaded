//! On-camera platform shell protocol (`USB_FEATURE_SHELL`).
//!
//! Once a custom updater payload (`updatershell`) is running on the camera it
//! exposes an interactive maintenance shell tunnelled over the Sony
//! external-command channel. This module is a Rust port of
//! `pmca/platform/backend/usb/` (the `UsbSequenceTransfer` framing plus the
//! `UsbPlatformBackend` request set).
//!
//! The layer is deliberately transport-agnostic: it is generic over
//! [`SonyExtCmdTransport`], so the exact same code runs against real hardware
//! (mass-storage or MTP) and against the in-memory mock transport used by the
//! unit tests below. No physical camera is required to exercise the framing.

use std::io::{Read, Write};

use crate::command::SonyExtCmdTransport;
use crate::{Error, Result};

/// Sony external-command id carrying platform-shell traffic.
pub const USB_FEATURE_SHELL: u32 = 0x23;

/// Generic failure result code returned by the on-camera shell.
pub const USB_RESULT_ERROR: i32 = -1;
/// Result code meaning "backup protection is enabled".
pub const USB_RESULT_ERROR_PROTECTION: i32 = -2;

/// Size in bytes of the sequence header prefixing every transfer.
const SEQUENCE_HEADER_SIZE: usize = 4;
/// Maximum payload carried in a single shell request/data frame.
const DATA_MAX: usize = 0xfff8;
/// Size of a [`UsbShellResponse`] (a single little-endian `i32`).
const SHELL_RESPONSE_SIZE: usize = 4;
/// Size of a status message (a single little-endian `i32`).
const STATUS_MSG_SIZE: usize = 4;
/// Size of a data message: a 4-byte length prefix plus [`DATA_MAX`] payload.
const DATA_MSG_SIZE: usize = 4 + DATA_MAX;

/// Status flag: transfer cancelled by the host.
const USB_STATUS_CANCEL: i32 = 1;

/// Status flag: this side of the tunnel has closed.
const USB_STATUS_EOF: i32 = 1;

/// Size of a socket-tunnel header: `status`, `rxSize`, `txSize`.
const SOCKET_HEADER_SIZE: usize = 12;

/// Payload carried in a single direction per tunnel round-trip.
const USB_SOCKET_BUFFER_SIZE: usize = 0xfff4;

/// The local end of a tunnelled shell session.
///
/// The tunnel protocol has to announce, in every round-trip, how many bytes it
/// can accept and how many it has ready. That is only possible if the local end
/// can be polled without blocking, which is what this trait requires: an
/// implementation must answer immediately, reporting "nothing right now" rather
/// than waiting for data. A blocking implementation would stall the camera side
/// of the tunnel just as much as the local one.
pub trait ShellEndpoint {
    /// Reads whatever is available right now.
    ///
    /// Returns `Ok(None)` when no data is currently available, and
    /// `Ok(Some(0))` when the local end has reached end of input.
    fn try_read(&mut self, buf: &mut [u8]) -> std::io::Result<Option<usize>>;

    /// Writes as much of `data` as it can accept right now, returning the count.
    fn try_write(&mut self, data: &[u8]) -> std::io::Result<usize>;

    /// Releases the local end; called at most once.
    fn close(&mut self);
}

/// Builds a padded shell request frame: `cmd` (4 bytes) + `data` padded to
/// [`DATA_MAX`] with zeros.
fn pack_shell_request(cmd: &[u8; 4], data: &[u8]) -> Result<Vec<u8>> {
    if data.len() > DATA_MAX {
        return Err(Error::Protocol("shell request payload too large"));
    }
    let mut packet = Vec::with_capacity(4 + DATA_MAX);
    packet.extend_from_slice(cmd);
    packet.extend_from_slice(data);
    packet.resize(4 + DATA_MAX, 0);
    Ok(packet)
}

/// Interactive maintenance shell exposed by the on-camera updater payload.
///
/// Wraps a [`SonyExtCmdTransport`] and speaks the `UsbPlatformBackend` request
/// set (`TEST`, `PUSH`, `PULL`, `RMEM`, `BKRD`, ...). Streaming operations
/// (file/memory transfers) are exposed through [`Read`]/[`Write`] sinks.
pub struct PlatformShell<T: SonyExtCmdTransport> {
    transport: T,
    command: u32,
    sequence: u32,
}

impl<T: SonyExtCmdTransport> PlatformShell<T> {
    /// Wraps `transport`, using the default [`USB_FEATURE_SHELL`] command id.
    pub fn new(transport: T) -> Self {
        Self { transport, command: USB_FEATURE_SHELL, sequence: 0 }
    }

    /// Consumes the shell, returning the underlying transport.
    pub fn into_transport(self) -> T {
        self.transport
    }

    /// Low-level sequenced send, keeping the shell's sequence counter in sync.
    fn transfer(&mut self, data: &[u8], buffer_size: usize) -> Result<Vec<u8>> {
        let mut packet = Vec::with_capacity(SEQUENCE_HEADER_SIZE + data.len());
        packet.extend_from_slice(&self.sequence.to_le_bytes());
        packet.extend_from_slice(data);

        let response = self.transport.send_sony_ext_command(
            self.command,
            &packet,
            SEQUENCE_HEADER_SIZE + buffer_size,
        )?;

        if response.len() < SEQUENCE_HEADER_SIZE {
            return Err(Error::Protocol("short sequence-transfer response"));
        }
        let echoed = u32::from_le_bytes(response[0..SEQUENCE_HEADER_SIZE].try_into().unwrap());
        if echoed != self.sequence {
            return Err(Error::Protocol("wrong sequence-transfer sequence"));
        }
        self.sequence += 1;
        Ok(response[SEQUENCE_HEADER_SIZE..].to_vec())
    }

    /// Sends a shell command and returns its non-negative result value.
    ///
    /// The result doubles as a payload length for follow-up reads. A result
    /// with the high bit set is an error; `protection_error` remaps the
    /// dedicated protection code to [`Error::Protocol`].
    fn req(&mut self, cmd: &[u8; 4], data: &[u8]) -> Result<u32> {
        let packet = pack_shell_request(cmd, data)?;
        let response = self.transfer(&packet, SHELL_RESPONSE_SIZE)?;
        if response.len() < SHELL_RESPONSE_SIZE {
            return Err(Error::Protocol("short shell response"));
        }
        let result = i32::from_le_bytes(response[0..4].try_into().unwrap());
        if result & i32::MIN != 0 {
            return Err(shell_error(result));
        }
        Ok(result as u32)
    }

    /// Waits for the on-camera shell to come up, retrying `TEST` a few times.
    pub fn start(&mut self) -> Result<()> {
        let mut last = Error::Protocol("shell not connected");
        for _ in 0..10 {
            match self.req(b"TEST", &[]) {
                Ok(_) => return Ok(()),
                Err(e) => last = e,
            }
        }
        Err(last)
    }

    /// Tells the on-camera shell to exit and tear down.
    pub fn stop(&mut self) -> Result<()> {
        self.req(b"EXIT", &[])?;
        Ok(())
    }

    /// Reads backup property `id`, returning its raw bytes.
    pub fn read_backup(&mut self, id: u32) -> Result<Vec<u8>> {
        let size = self.req(b"BKRD", &id.to_le_bytes())? as usize;
        self.transfer(&[], size)
    }

    /// Writes `data` to backup property `id`.
    ///
    /// Fails with [`Error::Protocol`] if backup protection is enabled.
    pub fn write_backup(&mut self, id: u32, data: &[u8]) -> Result<()> {
        if data.len() > 0xfff4 {
            return Err(Error::Protocol("backup value too large"));
        }
        let mut payload = Vec::with_capacity(8 + data.len());
        payload.extend_from_slice(&id.to_le_bytes());
        payload.extend_from_slice(&(data.len() as u32).to_le_bytes());
        payload.extend_from_slice(data);
        self.req(b"BKWR", &payload)?;
        Ok(())
    }

    /// Flushes pending backup changes to persistent storage.
    pub fn sync_backup(&mut self) -> Result<()> {
        self.req(b"BKSY", &[])?;
        Ok(())
    }

    /// Reads the backup status blob.
    pub fn backup_status(&mut self) -> Result<Vec<u8>> {
        let size = self.req(b"BKST", &[])? as usize;
        self.transfer(&[], size)
    }

    /// Enables or disables protection of restricted backup settings.
    pub fn set_backup_protection(&mut self, enable: bool) -> Result<()> {
        self.req(b"BKPR", &(enable as i32).to_le_bytes())?;
        Ok(())
    }

    /// Reads the full backup region into `out`.
    pub fn read_backup_data(&mut self, out: &mut dyn Write) -> Result<()> {
        self.req(b"BKDA", &[])?;
        self.stream_read(out)
    }

    /// Overwrites the full backup region with `data`.
    pub fn write_backup_data(&mut self, data: &[u8]) -> Result<()> {
        self.req(b"BKDW", &(data.len() as u32).to_le_bytes())?;
        self.stream_write(&mut &data[..])
    }

    /// Copies the on-camera file at `path` into `out`.
    pub fn read_file(&mut self, path: &str, out: &mut dyn Write) -> Result<u32> {
        let size = self.req(b"PULL", path.as_bytes())?;
        self.stream_read(out)?;
        Ok(size)
    }

    /// Writes `input` to the on-camera file at `path`.
    pub fn write_file(&mut self, path: &str, input: &mut dyn Read) -> Result<()> {
        self.req(b"PUSH", path.as_bytes())?;
        self.stream_write(input)
    }

    /// Dumps `size` bytes of device memory at `offset` into `out`.
    pub fn read_memory(&mut self, offset: u32, size: u32, out: &mut dyn Write) -> Result<()> {
        let mut payload = Vec::with_capacity(8);
        payload.extend_from_slice(&offset.to_le_bytes());
        payload.extend_from_slice(&size.to_le_bytes());
        self.req(b"RMEM", &payload)?;
        self.stream_read(out)
    }

    /// Mounts the Android data partition, returning its device path.
    pub fn mount_android_data(&mut self) -> Result<String> {
        let size = self.req(b"AMNT", &[])? as usize;
        let data = self.transfer(&[], size)?;
        Ok(String::from_utf8_lossy(&data).into_owned())
    }

    /// Unmounts the Android data partition, optionally committing backup.
    pub fn unmount_android_data(&mut self, commit_backup: bool) -> Result<()> {
        self.req(b"AUMT", &(commit_backup as i32).to_le_bytes())?;
        Ok(())
    }

    /// Opens the on-camera interactive shell and bridges it to `endpoint`.
    ///
    /// This hands the caller a shell running on the camera as root. It is the
    /// most powerful thing the platform payload offers, which is precisely why
    /// the endpoint is supplied by the caller: this layer does not decide who
    /// may reach it.
    pub fn interactive_shell(&mut self, endpoint: &mut dyn ShellEndpoint) -> Result<()> {
        self.req(b"SHEL", &[])?;
        self.tunnel(endpoint)
    }

    /// Pumps bytes between `endpoint` and the on-camera shell until both ends
    /// have closed.
    ///
    /// USB gives us no way to be notified about incoming data, so each
    /// round-trip starts by exchanging headers stating what each side can
    /// accept and has ready; the actual transfer is then sized to whatever both
    /// sides agreed on. The loop never blocks on the local end, because doing so
    /// would also stall the camera's side of the conversation.
    fn tunnel(&mut self, endpoint: &mut dyn ShellEndpoint) -> Result<()> {
        // rx: camera -> local. tx: local -> camera.
        let mut rx_buf: Vec<u8> = Vec::new();
        let mut tx_buf: Vec<u8> = Vec::new();
        let mut closed = false;

        loop {
            // Push whatever the camera sent towards the local end.
            if !closed && !rx_buf.is_empty() {
                match endpoint.try_write(&rx_buf) {
                    Ok(n) => drop(rx_buf.drain(..n)),
                    Err(_) => {
                        endpoint.close();
                        closed = true;
                    }
                }
            }
            if closed {
                rx_buf.clear();
            }

            // Only pick up new local input once the previous chunk is gone, so
            // a slow camera cannot make the host buffer grow without bound.
            if !closed && tx_buf.is_empty() {
                let mut buf = vec![0u8; USB_SOCKET_BUFFER_SIZE];
                match endpoint.try_read(&mut buf) {
                    Ok(Some(0)) | Err(_) => {
                        endpoint.close();
                        closed = true;
                    }
                    Ok(Some(n)) => {
                        buf.truncate(n);
                        tx_buf = buf;
                    }
                    Ok(None) => {}
                }
            }

            let master_status = if closed { USB_STATUS_EOF } else { 0 };
            // Advertise receive space only while nothing is still queued.
            let master_rx = if rx_buf.is_empty() { USB_SOCKET_BUFFER_SIZE } else { 0 };
            let master_tx = tx_buf.len();

            let mut header = Vec::with_capacity(SOCKET_HEADER_SIZE);
            header.extend_from_slice(&master_status.to_le_bytes());
            header.extend_from_slice(&(master_rx as u32).to_le_bytes());
            header.extend_from_slice(&(master_tx as u32).to_le_bytes());
            let response = self.transfer(&header, SOCKET_HEADER_SIZE)?;
            if response.len() < SOCKET_HEADER_SIZE {
                return Err(Error::Protocol("short shell-tunnel header"));
            }
            let slave_status = i32::from_le_bytes(response[0..4].try_into().unwrap());
            let slave_rx = u32::from_le_bytes(response[4..8].try_into().unwrap()) as usize;
            let slave_tx = u32::from_le_bytes(response[8..12].try_into().unwrap()) as usize;

            // Each direction moves what the sender has and the receiver can take.
            let rx_size = master_rx.min(slave_tx);
            let tx_size = master_tx.min(slave_rx);

            if master_status == USB_STATUS_EOF && slave_status == USB_STATUS_EOF {
                break;
            }

            // The camera is done, and nothing of its output is left to deliver.
            if !closed && rx_buf.is_empty() && slave_status == USB_STATUS_EOF {
                endpoint.close();
                closed = true;
            }

            let mut data = self.transfer(&tx_buf[..tx_size], rx_size)?;
            drop(tx_buf.drain(..tx_size));
            if rx_size > 0 {
                // Safe to replace rather than append: a non-zero rx_size means
                // we advertised space, which only happens while rx_buf is empty.
                data.truncate(rx_size);
                rx_buf = data;
            }
        }
        Ok(())
    }

    /// Reads the device-driven data stream into `out` until an empty frame.
    fn stream_read(&mut self, out: &mut dyn Write) -> Result<()> {
        loop {
            let status = 0i32.to_le_bytes();
            let response = self.transfer(&status, DATA_MSG_SIZE)?;
            if response.len() < 4 {
                return Err(Error::Protocol("short data message"));
            }
            let size = u32::from_le_bytes(response[0..4].try_into().unwrap()) as usize;
            let end = 4usize
                .checked_add(size)
                .filter(|e| *e <= response.len())
                .ok_or(Error::Protocol("data message size out of bounds"))?;
            out.write_all(&response[4..end])
                .map_err(|_| Error::Protocol("failed writing stream output"))?;
            if size == 0 {
                break;
            }
        }
        Ok(())
    }

    /// Writes `input` to the device as a sequence of data frames.
    fn stream_write(&mut self, input: &mut dyn Read) -> Result<()> {
        let mut buf = vec![0u8; DATA_MAX];
        loop {
            let n = read_up_to(input, &mut buf)?;
            let mut payload = Vec::with_capacity(DATA_MSG_SIZE);
            payload.extend_from_slice(&(n as u32).to_le_bytes());
            payload.extend_from_slice(&buf[..n]);
            payload.resize(DATA_MSG_SIZE, 0);
            let response = self.transfer(&payload, STATUS_MSG_SIZE)?;
            if response.len() < 4 {
                return Err(Error::Protocol("short status message"));
            }
            let status = i32::from_le_bytes(response[0..4].try_into().unwrap());
            if n == 0 || status == USB_STATUS_CANCEL {
                break;
            }
        }
        Ok(())
    }
}

/// Reads up to `buf.len()` bytes, returning the count (0 at EOF).
fn read_up_to(input: &mut dyn Read, buf: &mut [u8]) -> Result<usize> {
    let mut filled = 0;
    while filled < buf.len() {
        match input.read(&mut buf[filled..]) {
            Ok(0) => break,
            Ok(n) => filled += n,
            Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(_) => return Err(Error::Protocol("failed reading stream input")),
        }
    }
    Ok(filled)
}

/// Maps a negative shell result code to an [`Error`].
fn shell_error(result: i32) -> Error {
    match result {
        USB_RESULT_ERROR_PROTECTION => Error::Protocol("backup protection enabled"),
        _ => Error::Protocol("shell command failed"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    /// Scripted transport: verifies each request and replays a canned response.
    #[derive(Default)]
    struct MockTransport {
        expected: VecDeque<(u32, Vec<u8>, Vec<u8>)>,
    }

    impl MockTransport {
        fn expect(&mut self, cmd: u32, request: Vec<u8>, response: Vec<u8>) {
            self.expected.push_back((cmd, request, response));
        }
    }

    impl SonyExtCmdTransport for MockTransport {
        fn send_sony_ext_command(
            &mut self,
            cmd: u32,
            data: &[u8],
            _buffer_size: usize,
        ) -> Result<Vec<u8>> {
            let (exp_cmd, exp_req, resp) = self.expected.pop_front().expect("unexpected request");
            assert_eq!(cmd, exp_cmd, "command id mismatch");
            assert_eq!(data, &exp_req[..], "request payload mismatch");
            Ok(resp)
        }
    }

    /// Builds the wire request for sequence `seq`: header + cmd + padded data.
    fn shell_request(seq: u32, cmd: &[u8; 4], data: &[u8]) -> Vec<u8> {
        let mut v = seq.to_le_bytes().to_vec();
        v.extend_from_slice(cmd);
        v.extend_from_slice(data);
        v.resize(SEQUENCE_HEADER_SIZE + 4 + DATA_MAX, 0);
        v
    }

    /// Builds a shell response: sequence header + `i32` result.
    fn shell_response(seq: u32, result: i32) -> Vec<u8> {
        let mut v = seq.to_le_bytes().to_vec();
        v.extend_from_slice(&result.to_le_bytes());
        v
    }

    #[test]
    fn start_succeeds_on_first_test() {
        let mut t = MockTransport::default();
        t.expect(USB_FEATURE_SHELL, shell_request(0, b"TEST", &[]), shell_response(0, 0));
        let mut shell = PlatformShell::new(t);
        shell.start().unwrap();
    }

    /// Builds a tunnel header frame carrying the sequence number.
    fn tunnel_header(seq: u32, status: i32, rx: u32, tx: u32) -> Vec<u8> {
        let mut v = seq.to_le_bytes().to_vec();
        v.extend_from_slice(&status.to_le_bytes());
        v.extend_from_slice(&rx.to_le_bytes());
        v.extend_from_slice(&tx.to_le_bytes());
        v
    }

    /// Builds a tunnel data frame carrying the sequence number.
    fn tunnel_data(seq: u32, data: &[u8]) -> Vec<u8> {
        let mut v = seq.to_le_bytes().to_vec();
        v.extend_from_slice(data);
        v
    }

    /// A local end scripted with the reads it should hand out.
    #[derive(Default)]
    struct MockEndpoint {
        reads: VecDeque<Option<Vec<u8>>>,
        written: Vec<u8>,
        closed: bool,
    }

    impl ShellEndpoint for MockEndpoint {
        fn try_read(&mut self, buf: &mut [u8]) -> std::io::Result<Option<usize>> {
            match self.reads.pop_front() {
                // A queued chunk, "nothing yet", or end of input.
                Some(Some(data)) => {
                    buf[..data.len()].copy_from_slice(&data);
                    Ok(Some(data.len()))
                }
                Some(None) => Ok(None),
                None => Ok(Some(0)),
            }
        }

        fn try_write(&mut self, data: &[u8]) -> std::io::Result<usize> {
            self.written.extend_from_slice(data);
            Ok(data.len())
        }

        fn close(&mut self) {
            self.closed = true;
        }
    }

    #[test]
    fn the_tunnel_carries_bytes_in_both_directions() {
        let mut t = MockTransport::default();
        // Round 1: we have "ls\n" ready, the camera offers 5 bytes.
        t.expect(
            USB_FEATURE_SHELL,
            tunnel_header(0, 0, USB_SOCKET_BUFFER_SIZE as u32, 3),
            tunnel_header(0, 0, USB_SOCKET_BUFFER_SIZE as u32, 5),
        );
        t.expect(USB_FEATURE_SHELL, tunnel_data(1, b"ls\n"), tunnel_data(1, b"a.txt"));
        // Round 2: the camera's output still has to be delivered locally, so no
        // receive space is advertised; the local end reports end of input.
        t.expect(
            USB_FEATURE_SHELL,
            tunnel_header(2, USB_STATUS_EOF, USB_SOCKET_BUFFER_SIZE as u32, 0),
            tunnel_header(2, USB_STATUS_EOF, USB_SOCKET_BUFFER_SIZE as u32, 0),
        );

        let mut endpoint = MockEndpoint::default();
        endpoint.reads.push_back(Some(b"ls\n".to_vec()));
        let mut shell = PlatformShell::new(t);
        shell.tunnel(&mut endpoint).unwrap();

        assert_eq!(endpoint.written, b"a.txt", "camera output reached the local end");
        assert!(endpoint.closed, "the local end is released when the tunnel ends");
    }

    #[test]
    fn the_tunnel_closes_the_local_end_when_the_camera_hangs_up() {
        let mut t = MockTransport::default();
        // Nothing pending in either direction and the camera reports EOF.
        t.expect(
            USB_FEATURE_SHELL,
            tunnel_header(0, 0, USB_SOCKET_BUFFER_SIZE as u32, 0),
            tunnel_header(0, USB_STATUS_EOF, 0, 0),
        );
        t.expect(USB_FEATURE_SHELL, tunnel_data(1, &[]), tunnel_data(1, &[]));
        t.expect(
            USB_FEATURE_SHELL,
            tunnel_header(2, USB_STATUS_EOF, USB_SOCKET_BUFFER_SIZE as u32, 0),
            tunnel_header(2, USB_STATUS_EOF, 0, 0),
        );

        let mut endpoint = MockEndpoint::default();
        // "Nothing to read yet" - the local end has not closed by itself.
        endpoint.reads.push_back(None);
        let mut shell = PlatformShell::new(t);
        shell.tunnel(&mut endpoint).unwrap();

        assert!(endpoint.closed, "a camera-side hang-up must close the local end");
    }

    #[test]
    fn a_truncated_tunnel_header_is_rejected() {
        let mut t = MockTransport::default();
        t.expect(
            USB_FEATURE_SHELL,
            tunnel_header(0, 0, USB_SOCKET_BUFFER_SIZE as u32, 0),
            tunnel_data(0, &[0, 0, 0, 0]),
        );
        let mut endpoint = MockEndpoint::default();
        endpoint.reads.push_back(None);
        let mut shell = PlatformShell::new(t);
        assert!(shell.tunnel(&mut endpoint).is_err());
    }

    #[test]
    fn start_retries_until_success() {
        let mut t = MockTransport::default();
        // First TEST returns an error (protocol), retried with the next
        // sequence number until an OK is seen.
        t.expect(
            USB_FEATURE_SHELL,
            shell_request(0, b"TEST", &[]),
            shell_response(0, USB_RESULT_ERROR),
        );
        t.expect(USB_FEATURE_SHELL, shell_request(1, b"TEST", &[]), shell_response(1, 0));
        let mut shell = PlatformShell::new(t);
        shell.start().unwrap();
    }

    #[test]
    fn read_backup_returns_payload() {
        let mut t = MockTransport::default();
        // BKRD for id 0x12 -> result = 3 (payload length)
        let mut req = 0x12u32.to_le_bytes().to_vec();
        req.resize(DATA_MAX, 0);
        t.expect(USB_FEATURE_SHELL, shell_request(0, b"BKRD", &req), shell_response(0, 3));
        // Follow-up empty request reads the 3-byte payload.
        let mut payload = 1u32.to_le_bytes().to_vec(); // sequence header
        payload.truncate(SEQUENCE_HEADER_SIZE);
        payload.extend_from_slice(&[0xaa, 0xbb, 0xcc]);
        t.expect(USB_FEATURE_SHELL, 1u32.to_le_bytes().to_vec(), payload);

        let mut shell = PlatformShell::new(t);
        let value = shell.read_backup(0x12).unwrap();
        assert_eq!(value, vec![0xaa, 0xbb, 0xcc]);
    }

    #[test]
    fn write_backup_protection_maps_to_error() {
        let mut t = MockTransport::default();
        let mut req = Vec::new();
        req.extend_from_slice(&0x34u32.to_le_bytes());
        req.extend_from_slice(&2u32.to_le_bytes());
        req.extend_from_slice(&[0x01, 0x02]);
        req.resize(DATA_MAX, 0);
        t.expect(
            USB_FEATURE_SHELL,
            shell_request(0, b"BKWR", &req),
            shell_response(0, USB_RESULT_ERROR_PROTECTION),
        );
        let mut shell = PlatformShell::new(t);
        let err = shell.write_backup(0x34, &[0x01, 0x02]).unwrap_err();
        assert!(matches!(err, Error::Protocol(_)));
    }

    #[test]
    fn read_file_streams_until_empty_frame() {
        let mut t = MockTransport::default();
        // PULL -> result = total size 5
        let mut req = b"/tmp/x".to_vec();
        req.resize(DATA_MAX, 0);
        t.expect(USB_FEATURE_SHELL, shell_request(0, b"PULL", &req), shell_response(0, 5));
        // First data frame: 5 bytes.
        let mut frame1 = 1u32.to_le_bytes().to_vec(); // sequence header
        frame1.extend_from_slice(&5u32.to_le_bytes());
        frame1.extend_from_slice(b"hello");
        t.expect(
            USB_FEATURE_SHELL,
            {
                let mut r = 1u32.to_le_bytes().to_vec();
                r.extend_from_slice(&0i32.to_le_bytes());
                r
            },
            frame1,
        );
        // Terminating empty frame.
        let mut frame2 = 2u32.to_le_bytes().to_vec();
        frame2.extend_from_slice(&0u32.to_le_bytes());
        t.expect(
            USB_FEATURE_SHELL,
            {
                let mut r = 2u32.to_le_bytes().to_vec();
                r.extend_from_slice(&0i32.to_le_bytes());
                r
            },
            frame2,
        );

        let mut shell = PlatformShell::new(t);
        let mut out = Vec::new();
        let size = shell.read_file("/tmp/x", &mut out).unwrap();
        assert_eq!(size, 5);
        assert_eq!(out, b"hello");
    }

    #[test]
    fn write_file_streams_input() {
        let mut t = MockTransport::default();
        // PUSH -> result 0
        let mut req = b"/tmp/y".to_vec();
        req.resize(DATA_MAX, 0);
        t.expect(USB_FEATURE_SHELL, shell_request(0, b"PUSH", &req), shell_response(0, 0));
        // Data frame with 3 bytes.
        let mut data_req = 1u32.to_le_bytes().to_vec();
        data_req.extend_from_slice(&3u32.to_le_bytes());
        data_req.extend_from_slice(b"abc");
        data_req.resize(SEQUENCE_HEADER_SIZE + DATA_MSG_SIZE, 0);
        t.expect(USB_FEATURE_SHELL, data_req, shell_response(1, 0));
        // Terminating empty frame.
        let mut end_req = 2u32.to_le_bytes().to_vec();
        end_req.extend_from_slice(&0u32.to_le_bytes());
        end_req.resize(SEQUENCE_HEADER_SIZE + DATA_MSG_SIZE, 0);
        t.expect(USB_FEATURE_SHELL, end_req, shell_response(2, 0));

        let mut shell = PlatformShell::new(t);
        let mut input = &b"abc"[..];
        shell.write_file("/tmp/y", &mut input).unwrap();
    }

    #[test]
    fn wrong_sequence_is_rejected() {
        let mut t = MockTransport::default();
        // Respond with the wrong echoed sequence (1 instead of 0). `stop`
        // issues a single request and does not retry, so the mismatch surfaces.
        t.expect(USB_FEATURE_SHELL, shell_request(0, b"EXIT", &[]), shell_response(1, 0));
        let mut shell = PlatformShell::new(t);
        let err = shell.stop().unwrap_err();
        assert!(matches!(err, Error::Protocol(_)));
    }
}
