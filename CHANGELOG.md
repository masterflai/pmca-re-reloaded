# Changelog

All notable changes to this project are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the project uses
[Semantic Versioning](https://semver.org/).

## [Unreleased]

## [1.1.3] - 2026-10-17

### Added

- Service mode now offers the camera's interactive serial console (a root
  BusyBox shell over USB on modern Linux models). In `pmca-console` type `term`
  in the service shell; in the app a console panel sits on the service page.
  From there, remounting `/usr` read-write and editing files in the camera's
  Linux userland are ordinary shell commands. The console still needs the
  service-mode keys, exactly as the rest of service mode does.

### Fixed

- macOS: opening the camera failed with “could not open interface for
  exclusive access” and the app wrongly suggested another cable or avoiding
  USB hubs. In MTP mode the system's camera service `ptpcamerad` holds the
  camera; the program now ends it and retries. The system restarts the
  service when it is needed again.
- macOS: in mass-storage mode the system's disk driver holds the camera and
  cannot be displaced. The error now says so and asks to set the camera's USB
  connection to MTP.
- macOS: the setup notes claimed that mass-storage mode works through Sony's
  Camera Driver. It does not; they now ask for MTP mode instead.

### Changed

- Windows: the installers now look like the app. The setup file has its own
  icon instead of the generic NSIS one, and the NSIS and MSI pages show the
  app's colours, camera and wordmark.
- The demo animation in the README now follows the reader's light or dark
  colour scheme, like the screenshots.

## [1.1.2] - 2026-10-17

### Fixed

- Linux: in mass-storage mode the kernel's `usb-storage` driver held the
  camera, so opening it failed with “interface is busy” and the app wrongly
  suggested another cable. The program now detaches that driver itself when it
  claims the interface, so no manual step is needed.
- Linux: if another program still holds the camera (for example the desktop's
  media monitor), the error now says so and points to `scripts/linux-prepare.sh`
  instead of suggesting a cable change.

## [1.1.1] - 2026-10-17

### Fixed

- Linux: when the program was not allowed to open the camera over USB, it
  wrongly suggested trying another cable or port. It now explains that this is
  a permission question, solved once with a udev rule, and points to
  `scripts/linux-prepare.sh`.
- Linux: `scripts/linux-prepare.sh` installed its udev rule under a name that
  sorted too late for the access-granting `uaccess` tag to take effect, so
  opening the camera still failed. The rule is now installed where it works,
  and an earlier, wrongly named copy is removed.

## [1.1.0] - 2026-10-16

### Added

- Windows: drivers that Zadig or a similar tool left for Sony cameras — as
  the original Sony-PMCA-RE had users install — are found at start, and the
  app offers to remove them. They took the camera before this program's own
  driver and made service mode fail. On the command line:
  `pmca-console driver zadig [--remove]`.

### Fixed

- License page: the fwtool.py link pointed to a repository that does not exist.
- License page: the Nex-Hack wiki is offline; the link now leads to its archived copy.
- Windows installer: *Delete the application data* during uninstallation now
  also removes imported keys, plugins, edited translations and rollback copies.

## [1.0.0] - 2026-10-16

The first public release: a new implementation of the ideas behind
Sony-PMCA-RE, as a desktop application and a command line for Windows, macOS
and Linux.

### Added

- **Desktop app** for Windows, macOS and Linux, with light and dark theme and
  21 languages, each editable from within the app.
- **Command line** `pmca-console` with the same functions, for scripts and
  experts.
- **Camera information**: model, product code, serial number, firmware, lens
  and USB mode; the shutter count read from ARW, JPEG or HEIF photos; the
  camera's usage log, decoded.
- **Apps**: install your own `.apk` files, or several apps at once from the
  community app list, with a comparison against the apps already on the camera.
- **Firmware**: check an official firmware file against the connected camera
  and install it (experimental); the updater shell, with the update image
  generated on your computer.
- **Settings memory**: read and change settings the camera has but does not
  offer in its menu — video recording limit, menu languages, PAL/NTSC and
  more. Every change is backed up, read back, compared and reversible.
- **Function analysis** (experimental): find where a menu setting is stored in
  the settings memory and compare findings between cameras.
- **Network settings**: GPS assist data, access points and live-streaming
  settings.
- **File tools**: inspect backup images, usage logs and firmware files offline.
- **Service shell and updater shell** behind an *Advanced mode* switch.
- **Driver handling on Windows** without Zadig: WinUSB is assigned only for
  service mode, by a helper that asks for administrator rights once per
  session, and removed again afterwards.
- **Key import**: no Sony key material ships with the program; you import it
  from a source of your choice, and only byte sequences with a known digest
  are kept.
- **Plugins**: translations, compatibility notes, settings switches and
  sandboxed WebAssembly tools, installed from your own disk.
- **Device list** in the app with the functions each known model supports,
  plus community reports for further models.

[Unreleased]: https://github.com/masterflai/pmca-re-reloaded/compare/v1.1.0...HEAD
[1.1.0]: https://github.com/masterflai/pmca-re-reloaded/compare/v1.0.0...v1.1.0
[1.0.0]: https://github.com/masterflai/pmca-re-reloaded/releases/tag/v1.0.0
