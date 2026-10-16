# Fuzz targets

Every parser that reads bytes the program did not produce itself has a
target here. The directory is a workspace of its own and is excluded from the
main one, so a normal build never compiles libFuzzer.

| Target | Input |
|---|---|
| `backup` | settings backup image (`pmca_core::backup`) |
| `snapshot` | the same image as the function analysis reads it |
| `apk` | APK file and binary XML manifest (`pmca_apk`) |
| `usagelog` | usage log (`pmca_core::usagelog`) |
| `firmware` | firmware `.dat` (`pmca_core::firmware`) |
| `photo` | JPEG/ARW/HEIF for the shutter count (`pmca_core::photo`) |
| `catalog` | uploaded translation catalogue (`pmca_i18n::catalog`) |

Needs a nightly toolchain and `cargo install cargo-fuzz`. Linux or macOS is
the easy route; on Windows libFuzzer needs the MSVC toolchain with
AddressSanitizer.

```bash
cargo +nightly fuzz run backup -- -max_total_time=300
```

A crash is written to `fuzz/artifacts/<target>/`. Turn it into a regular unit
test in the crate concerned before fixing it, so it stays fixed.
