# Changelog

All notable changes to this project are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the project uses
[Semantic Versioning](https://semver.org/).

## [Unreleased]

### Added

- Settings: an update notification, off by default. When you switch it on, the
  app asks GitHub at every start whether a newer release exists and, if so,
  shows its version and download address at the top of the window.
  Pre-releases are not announced.

### Changed

- The "show all menu languages" switch now counts language slots rather than
  languages, and notes that a camera shows only the languages its firmware
  knows: an RX1R II shows 25 of the 35. Which language sits in which slot has
  been measured on that camera, which also confirms that the two languages a
  Japanese camera cannot show are simplified and traditional Chinese.

### Fixed

- Plugins: a plugin made for a newer or a different edition of the program was
  refused with a raw parser message listing every allowed field. It now says
  in your language which feature this version does not know.
- Command line: commands that never use a camera (plugins, keys, the file
  tools, `license`, `apps`) no longer look for drivers an earlier run left
  behind. A camera command that finds one now actually asks for the
  administrator rights its message announces.

## [1.1.4] - 2026-10-21

**Caution on macOS:** the new macOS functions of this release have not yet
been tried on a Mac with a camera. This matters most for firmware updates: an
update that breaks off can leave the camera unable to start. Until a
successful update on a Mac has been reported, update the firmware from Windows
or with Sony's own updater, and use the updater shell on a Mac only if you
know how to recover the camera.

### Added

- macOS: the tweaks, the service shell, the function analysis, firmware
  updates and the updater shell work in the app. macOS hands the camera's
  mass-storage connection only to a program with administrator rights, so the
  app asks for your password once per run through the system dialog and keeps
  an elevated copy of itself for this work. The tweaks keep their service-mode
  session between reading and applying. The camera's card leaves Finder while
  this runs and comes back afterwards. Files the service and updater shells
  write to this computer belong to you, and relative paths start in your home
  folder.
- macOS: the service shell and the function analysis are available from the
  command line run with `sudo`, for example `sudo ./pmca-console serviceshell`.
  Without those rights the error message says what to do.

### Changed

- The theme is chosen with a single button that cycles through system, light
  and dark. A sun or moon shows the active theme; in system mode a small "A"
  sits next to it.
- Files a run under `sudo` creates in your user folder (keys, records kept
  before a change) now belong to you, not to the administrator account.
- Linux: the command-line program is now built on Ubuntu 22.04, so it also
  starts on distributions with an older C library (glibc).

### Fixed

- Windows: after writing to the camera in service mode, the camera could come
  back with the service-mode driver still assigned to its mass-storage side
  and ask you to check the connected device. The clean-up now also releases
  the driver of a camera that was switching modes at that moment; one left
  behind by an earlier version is released at the next camera task, after
  Windows asks for administrator rights.
- macOS and Linux, MTP mode: reading the camera information failed with
  “The camera aborted the USB transfer”. The program sent Sony's commands
  without first opening a session with the camera, which the camera requires.
  It now opens one, as it always did on Windows. If the camera rejects the very
  first request because an earlier program left a transfer unfinished, the
  connection is reset and the request is sent again.
- macOS: without administrator rights, a firmware update or the updater shell
  stops before the camera restarts into update mode. Before, the camera
  restarted first and then stayed in update mode until its battery was taken
  out.
- Command line: after `exit`, the service and updater shells printed their
  closing message twice.

### Security

- Downloads over HTTPS no longer accept certificates issued through the
  distrusted emSign intermediates (RUSTSEC-2026-0381, RUSTSEC-2026-0382).

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

[Unreleased]: https://github.com/masterflai/pmca-re-reloaded/compare/v1.1.4...HEAD
[1.1.0]: https://github.com/masterflai/pmca-re-reloaded/compare/v1.0.0...v1.1.0
[1.0.0]: https://github.com/masterflai/pmca-re-reloaded/releases/tag/v1.0.0
