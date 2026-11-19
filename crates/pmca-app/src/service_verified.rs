//! Verified, read-only camera-to-host exports.
//!
//! A successful USB read is not proof that a partition image is complete.
//! Require a caller-supplied exact size, read the same source twice, compare
//! SHA-256 digests and publish only a completely verified host file.
//! The reader is injected so every failure path can be tested without USB.

use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use pmca_i18n::Localizer;
use sha2::{Digest, Sha256};

use crate::{Error, Result};

/// Conservative per-export limit; larger partitions need an explicit review.
const MAX_EXPORT_BYTES: u64 = 128 * 1024 * 1024;
static NEXT_TEMP: AtomicU64 = AtomicU64::new(0);

/// A completed export; the second readback matched exactly.
pub(crate) struct VerifiedFile {
    pub bytes: u64,
    pub sha256: String,
}

/// Parse an unprefixed, exact 64-digit SHA-256 string.
pub(crate) fn parse_sha256(value: &str) -> Option<[u8; 32]> {
    if value.len() != 64 || !value.is_ascii() {
        return None;
    }
    let mut bytes = [0u8; 32];
    for (i, byte) in bytes.iter_mut().enumerate() {
        *byte = u8::from_str_radix(&value[2 * i..2 * i + 2], 16).ok()?;
    }
    Some(bytes)
}

fn hex(bytes: &[u8; 32]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

/// Count and hash only bytes the downstream writer actually accepted.
struct HashedWriter<'a> {
    out: &'a mut dyn Write,
    digest: Sha256,
    bytes: u64,
    limit: u64,
}

impl<'a> HashedWriter<'a> {
    fn new(out: &'a mut dyn Write, limit: u64) -> Self {
        Self { out, digest: Sha256::new(), bytes: 0, limit }
    }

    fn finish(self) -> (u64, [u8; 32]) {
        (self.bytes, self.digest.finalize().into())
    }
}

impl Write for HashedWriter<'_> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let proposed = self
            .bytes
            .checked_add(buf.len() as u64)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "export length overflow"))?;
        if proposed > self.limit {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "export exceeds expected size"));
        }
        let written = self.out.write(buf)?;
        self.digest.update(&buf[..written]);
        self.bytes += written as u64;
        Ok(written)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.out.flush()
    }
}

/// A same-directory staging file, removed on every error path.
///
/// Publication uses hard_link rather than rename: rename replaces an
/// existing destination on Unix, while hard_link atomically fails if it
/// already exists (including when it is a symlink). FAT and exFAT drives have
/// no hard links; there the verified bytes are copied into a file created with
/// `create_new`, which still never replaces anything.
struct StagingFile {
    path: PathBuf,
    file: Option<File>,
}

impl StagingFile {
    fn create(l: &Localizer, destination: &Path) -> Result<Self> {
        let staging_error =
            || {
                Error::Operation(l.tr_args(
                    "service-verify-staging",
                    &[("path", &destination.display().to_string())],
                ))
            };
        let parent = destination
            .parent()
            .filter(|path| !path.as_os_str().is_empty())
            .unwrap_or_else(|| Path::new("."));
        let name = destination.file_name().ok_or_else(staging_error)?.to_string_lossy();
        for _ in 0..32 {
            let sequence = NEXT_TEMP.fetch_add(1, Ordering::Relaxed);
            let path = parent.join(format!(".{name}.pmca-{}-{sequence}.tmp", std::process::id()));
            let mut options = File::options();
            options.write(true).create_new(true);
            // Raw camera partitions may carry credentials and identifiers;
            // protect the staging file before the first byte is written.
            #[cfg(unix)]
            {
                use std::os::unix::fs::OpenOptionsExt;
                options.mode(0o600);
            }
            match options.open(&path) {
                Ok(file) => return Ok(Self { path, file: Some(file) }),
                Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
                Err(error) => return Err(error.into()),
            }
        }
        Err(staging_error())
    }

    fn publish(&mut self, l: &Localizer, destination: &Path) -> Result<()> {
        let file = self.file.take().expect("staging file available until publication");
        file.sync_all()?;
        // Windows cannot reliably unlink an open file. Close it before
        // publishing or cleaning up, on both success and failure.
        drop(file);
        let exists = || {
            Error::Operation(
                l.tr_args("service-verify-exists", &[("path", &destination.display().to_string())]),
            )
        };
        match fs::hard_link(&self.path, destination) {
            Ok(()) => Ok(()),
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => Err(exists()),
            Err(_) => {
                let mut out = match File::options().write(true).create_new(true).open(destination) {
                    Ok(out) => out,
                    Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
                        return Err(exists());
                    }
                    Err(error) => return Err(error.into()),
                };
                let copied =
                    io::copy(&mut File::open(&self.path)?, &mut out).and_then(|_| out.sync_all());
                if let Err(error) = copied {
                    drop(out);
                    let _ = fs::remove_file(destination);
                    return Err(error.into());
                }
                Ok(())
            }
        }
    }
}

impl Drop for StagingFile {
    fn drop(&mut self) {
        drop(self.file.take());
        let _ = fs::remove_file(&self.path);
    }
}

/// Export two independent reads of a camera source to a new host file.
///
/// The source is never opened for writing. `read_once` must perform one
/// complete USB read per invocation, propagating errors and writing all data
/// into the supplied sink. An optional externally trusted hash prevents two
/// consistently wrong camera reads from being accepted as the correct image.
pub(crate) fn export_twice(
    l: &Localizer,
    destination: &Path,
    expected_bytes: u64,
    expected_sha256: Option<[u8; 32]>,
    mut read_once: impl FnMut(&mut dyn Write) -> Result<()>,
) -> Result<VerifiedFile> {
    if !(1..=MAX_EXPORT_BYTES).contains(&expected_bytes) {
        return Err(Error::Operation(
            l.tr_args("service-verify-size", &[("max", &MAX_EXPORT_BYTES.to_string())]),
        ));
    }
    let length_error = |got: u64| {
        Error::Operation(l.tr_args(
            "service-verify-length",
            &[("expected", &expected_bytes.to_string()), ("got", &got.to_string())],
        ))
    };

    let mut staging = StagingFile::create(l, destination)?;
    let mut first =
        HashedWriter::new(staging.file.as_mut().expect("staging file available"), expected_bytes);
    read_once(&mut first)?;
    let (first_len, first_hash) = first.finish();
    if first_len != expected_bytes {
        return Err(length_error(first_len));
    }

    // A second USB read goes only into a bounded hash sink: no duplicate RAM
    // allocation and no second file that could accidentally be published.
    let mut sink = io::sink();
    let mut second = HashedWriter::new(&mut sink, expected_bytes);
    read_once(&mut second)?;
    let (second_len, second_hash) = second.finish();
    if second_len != expected_bytes {
        return Err(length_error(second_len));
    }
    if second_hash != first_hash {
        return Err(Error::Operation(l.tr("service-verify-mismatch")));
    }
    if expected_sha256.is_some_and(|expected| expected != first_hash) {
        return Err(Error::Operation(l.tr("service-verify-reference")));
    }

    staging.publish(l, destination)?;
    Ok(VerifiedFile { bytes: first_len, sha256: hex(&first_hash) })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn export_twice(
        destination: &Path,
        expected_bytes: u64,
        expected_sha256: Option<[u8; 32]>,
        read_once: impl FnMut(&mut dyn Write) -> Result<()>,
    ) -> Result<VerifiedFile> {
        super::export_twice(
            &Localizer::new("en-US"),
            destination,
            expected_bytes,
            expected_sha256,
            read_once,
        )
    }

    struct TempDir(PathBuf);

    impl TempDir {
        fn new() -> Self {
            let sequence = NEXT_TEMP.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir()
                .join(format!("pmca-verified-test-{}-{sequence}", std::process::id()));
            fs::create_dir(&path).unwrap();
            Self(path)
        }

        fn dest(&self) -> PathBuf {
            self.0.join("export.bin")
        }

        fn entries(&self) -> usize {
            fs::read_dir(&self.0).unwrap().count()
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn two_identical_reads_publish_once() {
        let dir = TempDir::new();
        let mut calls = 0;
        let result = export_twice(&dir.dest(), 5, Some(Sha256::digest(b"hello").into()), |out| {
            calls += 1;
            out.write_all(b"hello")?;
            Ok(())
        })
        .unwrap();
        assert_eq!(calls, 2);
        assert_eq!(result.bytes, 5);
        assert_eq!(result.sha256, hex(&Sha256::digest(b"hello").into()));
        assert_eq!(fs::read(dir.dest()).unwrap(), b"hello");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            assert_eq!(fs::metadata(dir.dest()).unwrap().permissions().mode() & 0o777, 0o600);
        }
        assert_eq!(dir.entries(), 1);
    }

    #[test]
    fn corrupted_readback_leaves_no_output() {
        let dir = TempDir::new();
        let mut calls = 0;
        assert!(
            export_twice(&dir.dest(), 5, None, |out| {
                calls += 1;
                out.write_all(if calls == 1 { b"hello" } else { b"HELLO" })?;
                Ok(())
            })
            .is_err()
        );
        assert_eq!(calls, 2);
        assert_eq!(dir.entries(), 0);
    }

    #[test]
    fn wrong_size_and_reference_hash_fail_closed() {
        let dir = TempDir::new();
        assert!(
            export_twice(&dir.dest(), 6, None, |out| {
                out.write_all(b"hello")?;
                Ok(())
            })
            .is_err()
        );
        assert_eq!(dir.entries(), 0);
        assert!(
            export_twice(&dir.dest(), 5, Some([0; 32]), |out| {
                out.write_all(b"hello")?;
                Ok(())
            })
            .is_err()
        );
        assert_eq!(dir.entries(), 0);
    }

    #[test]
    fn existing_host_file_is_never_replaced() {
        let dir = TempDir::new();
        fs::write(dir.dest(), b"original").unwrap();
        assert!(
            export_twice(&dir.dest(), 5, None, |out| {
                out.write_all(b"hello")?;
                Ok(())
            })
            .is_err()
        );
        assert_eq!(fs::read(dir.dest()).unwrap(), b"original");
        assert_eq!(dir.entries(), 1);
    }

    #[test]
    fn usb_error_and_oversize_never_publish() {
        let dir = TempDir::new();
        assert!(
            export_twice(&dir.dest(), 5, None, |out| {
                out.write_all(b"he")?;
                Err(Error::Operation("simulated USB disconnect".into()))
            })
            .is_err()
        );
        assert_eq!(dir.entries(), 0);
        assert!(
            export_twice(&dir.dest(), 5, None, |out| {
                out.write_all(b"123456")?;
                Ok(())
            })
            .is_err()
        );
        assert_eq!(dir.entries(), 0);
    }

    #[test]
    fn hash_parser_rejects_malformed_input() {
        let good = hex(&Sha256::digest(b"hello").into());
        assert_eq!(parse_sha256(&good), Some(Sha256::digest(b"hello").into()));
        assert!(parse_sha256("ab").is_none());
        assert!(parse_sha256(&"g".repeat(64)).is_none());
        assert!(parse_sha256(&"é".repeat(32)).is_none());
    }
}
