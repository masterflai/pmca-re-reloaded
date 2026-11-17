# Verified read-only partition/file export (PR2)

The shared PMCA service shell (console **and** GUI) now supports a bounded
`pull-verified` command. It uses the existing Senser read-only file-control
command; it does not send a camera file-write, partition-flash, erase,
remount or updater command.

## Syntax

```text
pull-verified <camera-path> <host-file> <expected-decimal-bytes> [expected-sha256]
```

Example (illustrative only; **do not assume this path or size matches a
particular camera**):

```text
pull-verified /dev/nflasha05 flash-a05.bin 41943040
```

The exact byte length is **mandatory** and must be established independently
for the connected camera and partition. The optional 64-digit unprefixed SHA-256
must come from an independently trusted reference; without it, two matching
reads demonstrate repeatability, **not** that the camera supplied a correct
reference image. Both reads are performed over the same authenticated service
session. The default maximum is 128 MiB per export.

## Safety and verification

1. Open one host staging file in the destination directory with `create_new`.
2. Read once from the camera into that file while counting and hashing streamed
   bytes; abort if it exceeds or does not exactly match the declared length.
3. Read the same camera source **again** into a hash-only sink; require identical
   length and SHA-256, plus the optional supplied reference SHA-256.
4. `sync_all` the staging file, close it and create the final path through an
   **atomic, no-overwrite hard link**. Clean the staging path on both success
   and failure. Neither existing destination files nor symlinks are replaced.

The final file appears only after both reads pass. Filesystems lacking hard-link
support fail closed instead of falling back to a potentially overwriting rename.
Any USB error, short read, mismatched hash, oversize response, or existing host
path aborts publication. Private image contents are never logged by this
command; the result includes only byte count and SHA-256.

## Limitations and qualification

- **No HX50V live 40-MiB qualification is implied.** USB/Senser service-mode
  entry and exit remain real camera state changes; a read operation is not
  guaranteed to be harmless on an unqualified block-device path.
- This command does not discover valid partitions or lengths. Do not guess the
  path, the partition's size, or its stability between reads.
- Matching hashes do not prove snapshot consistency if the underlying device
  changes while it is being read; the optional trusted reference adds
  evidence. The host directory's crash-durability is filesystem-dependent.
- Unit tests exercise synthetic readers, mismatch/error/overwrite cases, with
  no USB hardware. A separately authorized model-specific hardware test is
  needed before claiming full large-partition reliability.

Checks:

```sh
cargo test -p pmca-app service_verified
cargo test -p pmca-app service::tests
```
