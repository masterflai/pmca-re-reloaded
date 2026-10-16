//! Terminal and TCP endpoints for the interactive updater shell.

use crate::*;

/// The terminal as the local end of a tunnelled shell.
///
/// Input comes from the process-wide reader in [`stdin_lines`] rather than a
/// reader of its own, so no keystroke is lost when the tunnel closes.
pub(crate) struct StdioEndpoint {
    pending: Vec<u8>,
    eof: bool,
}

impl StdioEndpoint {
    pub(crate) fn new() -> Self {
        Self { pending: Vec::new(), eof: false }
    }
}

/// Lines typed at the console, shared by every consumer in the process.
///
/// A blocking read cannot be cancelled portably, so one long-lived thread owns
/// stdin and hands whole lines over a channel. It has to be a *single* thread
/// for the whole process: a reader started per shell session stays parked
/// inside `read` after its session ends and then swallows the first line typed
/// at the next prompt.
pub(crate) fn stdin_lines() -> &'static Mutex<Receiver<String>> {
    static LINES: OnceLock<Mutex<Receiver<String>>> = OnceLock::new();
    LINES.get_or_init(|| {
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let stdin = std::io::stdin();
            loop {
                let mut line = String::new();
                match stdin.read_line(&mut line) {
                    Ok(0) | Err(_) => break,
                    Ok(_) => {}
                }
                if tx.send(line).is_err() {
                    break;
                }
            }
        });
        Mutex::new(rx)
    })
}

/// Blocks until the next line is typed; `None` at end of input.
pub(crate) fn read_stdin_line() -> Option<String> {
    stdin_lines().lock().ok()?.recv().ok()
}

/// Returns the next line only if one is already waiting.
pub(crate) fn try_read_stdin_line() -> std::result::Result<String, TryRecvError> {
    match stdin_lines().lock() {
        Ok(rx) => rx.try_recv(),
        Err(_) => Err(TryRecvError::Disconnected),
    }
}

/// Renders a typed line as the bytes the camera's shell expects.
///
/// Windows delivers `\r\n`. The camera runs BusyBox `ash`, which treats the
/// `\r` as part of the final word, so `ls /` asks for a file named `"/\r"` and
/// every command fails — including `exit`, which leaves no way out of the
/// tunnel. The reference implementation reads stdin in Python's text mode,
/// where this translation happens invisibly.
pub(crate) fn shell_line_bytes(line: &str) -> Vec<u8> {
    let mut out = line.trim_end_matches(['\r', '\n']).as_bytes().to_vec();
    out.push(b'\n');
    out
}

impl pmca_usb::platform_shell::ShellEndpoint for StdioEndpoint {
    fn try_read(&mut self, buf: &mut [u8]) -> std::io::Result<Option<usize>> {
        if self.pending.is_empty() {
            if self.eof {
                return Ok(Some(0));
            }
            match try_read_stdin_line() {
                Ok(line) => self.pending = shell_line_bytes(&line),
                Err(TryRecvError::Empty) => return Ok(None),
                Err(TryRecvError::Disconnected) => return Ok(Some(0)),
            }
        }
        let n = self.pending.len().min(buf.len());
        buf[..n].copy_from_slice(&self.pending[..n]);
        self.pending.drain(..n);
        Ok(Some(n))
    }

    fn try_write(&mut self, data: &[u8]) -> std::io::Result<usize> {
        let mut out = std::io::stdout().lock();
        out.write_all(data)?;
        out.flush()?;
        Ok(data.len())
    }

    fn close(&mut self) {
        self.eof = true;
    }
}

/// A loopback TCP connection as the local end of a tunnelled shell.
pub(crate) struct TcpEndpoint(std::net::TcpStream);

impl TcpEndpoint {
    /// Wraps an accepted connection, switching it to non-blocking mode so the
    /// tunnel cannot be stalled by an idle client.
    pub(crate) fn new(stream: std::net::TcpStream) -> std::io::Result<Self> {
        stream.set_nonblocking(true)?;
        Ok(Self(stream))
    }
}

impl pmca_usb::platform_shell::ShellEndpoint for TcpEndpoint {
    fn try_read(&mut self, buf: &mut [u8]) -> std::io::Result<Option<usize>> {
        match self.0.read(buf) {
            Ok(n) => Ok(Some(n)),
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => Ok(None),
            Err(e) => Err(e),
        }
    }

    fn try_write(&mut self, data: &[u8]) -> std::io::Result<usize> {
        match self.0.write(data) {
            Ok(n) => Ok(n),
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => Ok(0),
            Err(e) => Err(e),
        }
    }

    fn close(&mut self) {
        // Best effort: the client may already have hung up.
        let _ = self.0.shutdown(std::net::Shutdown::Both);
    }
}
