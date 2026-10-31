# Changelog

All notable changes to this project are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the project uses
[Semantic Versioning](https://semver.org/).

## [Unreleased]

## [1.8.0] - 2026-10-31

**Find app sources from the Apps page.** *Search GitHub* now sits next to the
app list and shows only the sources not added yet, the list is called
*Community apps*, and the Linux AppImage can update itself through
AppImageUpdate.

### Added

- *Search GitHub* on the Apps page as well, next to *Load available apps*. It
  shows only sources not added yet, or says that every repository GitHub knows
  is already there; an added source's apps join the list at once.
- The Linux AppImage carries update information, and a `.zsync` file is
  published beside it, so AppImageUpdate and similar tools can update it in
  place. They only offer releases marked *Latest*.

### Changed

- The Apps page calls its list *Community apps* instead of *Community app
  store*, since it gathers apps from any number of sources. The note that Sony's
  own store is gone is plain text now rather than a highlighted notice.

## [1.7.1] - 2026-10-30

**Only stable app releases.** Apps from a GitHub source now come from the
release its developer marks as *Latest*, never from a pre-release, and
versions read the same everywhere.

### Fixed

- Apps from a GitHub source come from the release marked *Latest*. Before,
  the newest release with an APK was taken, a pre-release included.
- Versions in the app list no longer start with a `v` (`v1.11.0` is shown as
  `1.11.0`), so they read the same as the version on the camera.

### Security

- `zip` updated to 9.1.3 (GHSA-m665-v7pm-7grj): archives whose local and
  central headers disagree could be read differently than they were checked.
  The program reads apps and plugins with it.

## [1.7.0] - 2026-10-28

**Finding apps and cameras more easily.** Camera compatibility has its own
page in the navigation, an empty app list offers the community list first and
the original project's list second, and apps whose developers have archived
their projects are marked as such.

### Added

- An app whose GitHub repository its developer has archived is marked
  *archived* in the app list. Its details explain that the developer no longer
  works on it actively and that it may not work fully on every camera.

### Changed

- Camera compatibility is a page of its own in the navigation, right below
  Camera info, instead of a section at the end of that page. The device list
  is shown at once, without a button to unfold it.
- With no source added yet, the app list first offers this program's
  community list (`masterflai/pmca-community-sources`) and then the original
  project's list, each with its own question. The community list now comes
  first among the suggestions and is always listed first among the sources;
  an app offered identically by two sources is shown under it.

## [1.6.0] - 2026-10-28

**Apps are checked in full before they reach the camera.** An app from a
source is installed only when its signature holds for every file in it, which
makes a pinned certificate a real promise. With that in place, the suggested
community source `masterflai/pmca-community-sources` now also lists
[JPEGCAM](https://jpeg.cam) by James (jbuch1984), with his permission: it is
always downloaded from his own server and installed only if it is signed with
his certificate. On macOS 26 the mass-storage connection is still being
tested.

**Key material.** Installing an app, JPEGCAM included, needs key material the
program does not ship. Import it once, on the *Welcome* page or with
`pmca-console keys import <url or file>`, as described under
[Key material](https://github.com/masterflai/pmca-re-reloaded#key-material) in
the README. If it is missing, the program says so before the camera is
switched.

### Security

- An app from a source is installed only when its v1 signature holds for every
  file in it, as Android checks before installing. Until now only the
  certificate in the signature was compared, so a changed APK that kept the
  original signature files would have passed a `cert_sha256` pin and the
  check against the first download.
- Downloads follow redirects only over HTTPS.

### Added

- The key-material card on the Welcome page has a folded section that lists
  which features need which key material: camera info none, app installation
  the XPD checksum key (and SPK for an app), service mode the service-mode
  keys, updater mode the FDAT key material.
- Clicking the version number below the navigation checks for a new version
  right away, also when the check at start is switched off, and says how it
  ended. The focus then moves to the notice.

### Changed

- The notice about a new version can be folded away.
- The suggested community source is described as also listing apps whose
  authors asked to be listed.
- `docs/app-sources.md` describes what happens to a published app: where it
  is downloaded from, what is checked, that it is not changed, and that an
  update goes over the installed copy.
- macOS: after the mass-storage drivers have been taken off the camera, its
  interface is claimed through `nusb` first and through IOUSBHost only if that
  fails.

### Fixed

- Installing an app, or reading the camera's apps, now checks for the needed
  key material before the camera is switched, and offers the key import when
  it is missing. Before, the camera could be left in app-install mode with a
  general error.
- Switching a camera from MTP to mass storage no longer fails when the camera
  leaves the bus before it has answered the request, which happened on macOS.

## [1.5.0] - 2026-10-28

**More ways to publish and to trust apps.** Sources can point to a JSON
release document and name the certificate their apps are signed with, a second
list is offered for apps whose authors have archived their projects, and the
app list can be narrowed to one source.

### Added

- Sources can name an app's release through a JSON document of the author's
  own: `release: type: json` with the document's `url` and a `download` address
  containing `{version}` and `{build}`. Authors who already publish such a
  document need not change anything (format in `docs/app-sources.md`).
- An app entry can name the SHA-256 of its signing certificate (`cert_sha256`).
  Every download is then checked against it, the first one included; an APK
  signed otherwise is refused before it reaches the camera.
- A second source is offered as a suggestion under *Settings → App and plugin
  sources*: `masterflai/pmca-community-sources`, for apps whose authors have
  archived their projects. It points only to the authors' own releases.
- Community app store: once the list comes from more than one source, it can
  be narrowed to a single source.

### Changed

- Console: `install --app` no longer asks for `--source` when several sources
  offer the same app in the same way (same release and certificate); the
  source added first is used. Only offers that lead to different files still
  need the choice.

## [1.4.1] - 2026-10-27

Two small fixes: apps with an obfuscated manifest are read correctly, and the
README describes how to open the app on current macOS versions.

### Fixed

- Apps whose manifest has blanked attribute names (left by some obfuscators)
  now show their version and minimum Android version; the names are taken from
  the attribute ids.
- README: on macOS 15 and later a blocked first start is allowed under
  *System Settings → Privacy & Security → Open Anyway*; right-click → *Open*
  no longer does it.

## [1.4.0] - 2026-10-26

**Back up your camera's apps.** The apps installed on a camera can now be
copied to your computer and installed again later, even once their source is
gone. The console catches up with the window: shutter count from a photo, the
switch to mass storage and the warnings about models the lists do not cover.

**macOS:** the app backup uses the camera's updater mode, which has still not
been tried on a Mac with a camera. Until it has been reported to work there,
make backups from Windows or Linux.

### Added

- App backup: the apps installed on a camera are copied to this computer
  (Apps page, `pmca-console apps backup --confirm`), one `.apk` per app with a
  checksum, in a folder per camera. The camera restarts into updater mode for
  it; its app storage is mounted read-only. A saved app is restored with the
  normal installation.
- The backup folder can be chosen on the Apps page; the choice is remembered
  and also used by `pmca-console apps backup` without `--out`.
- Tweaks page: a folded explanation of what tweaks are, how reading and
  writing them works, the way back, and what is needed.
- Console: `pmca-console photo <file>` reads the shutter count from a photo,
  and `pmca-console mass-storage` switches a camera in MTP mode to mass
  storage; a command that fails on an MTP camera points to it.
- Console: before installing apps, sending GPS data, the firmware update, the
  updater shell and the app backup, a warning appears when the device lists
  do not vouch for the camera, as the window has always shown. A running
  Windows service known to block the camera is named at start.

### Changed

- The device lists live in the shared core instead of the window, so the
  console and the window check models against the same lists.

### Fixed

- Windows: entering updater mode (firmware update, updater shell) sometimes
  failed with "SCSI pass-through ioctl failed" right after the camera
  restarted. The first command is now retried for a few seconds, and the
  error message includes the Windows error code.

## [1.3.0] - 2026-10-26

**Apps and plugins from anyone.** Developers publish camera apps and plugins
in a GitHub repository of their own; users add the sources they choose. The
program hosts nothing and checks nothing. How to publish:
[Publishing apps and plugins](https://github.com/masterflai/pmca-re-reloaded/blob/main/docs/app-sources.md).

**macOS:** firmware updates and the updater shell have still not been tried
on a Mac with a camera. Until a successful update on a Mac has been reported,
update the firmware from Windows or with Sony's own updater.

**macOS 26:** reading the camera information with the camera's USB connection
set to MTP is confirmed. The reworked mass-storage connection below, which the
tweaks, the service shell, the function analysis, firmware updates and the
updater shell need, has not yet been confirmed on macOS 26.

### Added

- App and plugin sources. Apps and plugins from other developers now come from
  sources you add yourself: public GitHub repositories with a
  `pmca-source.yaml` (format in `docs/app-sources.md`, compatible with the
  OpenMemories app list). The program ships no source and checks none. Under
  *Settings → App and plugin sources* you add or remove sources, switch on the
  original project's app list as a suggestion, and search GitHub for
  repositories with the topic `pmca-source`; stars are shown for every source.
  Plugins can be installed from your sources in the plugin card. Console:
  `sources list|add|remove|search`, `plugins available|get`,
  `install --app <package> [--source <owner/repo>]`.
- The first download of an app from a source remembers its signing
  certificate; a later version signed by someone else is refused.
- Plugin signatures (Ed25519). `pmca-console plugins keygen` creates a key,
  `plugins pack --key` signs a plugin. The program shows the key before
  installing, remembers it for plugins from a source like an app's
  certificate, and replaces an installed signed plugin only with a file signed
  by the same key.
- A plugin can translate its name and description: the messages
  `plg-<id>-name` and `plg-<id>-description` in its own language files are
  shown in place of the manifest's text once the plugin is installed.
- Six more languages: Traditional Chinese (Taiwan and Hong Kong), Korean,
  Vietnamese, Thai and Ukrainian. 27 languages in total.

### Changed

- The OpenMemories app list is no longer read without being asked: after the
  update the app list stays empty until you add a source or switch on the
  suggestion. Loading the apps without any source offers to switch it on.
- Apps, plugins and sources have a *Details* panel with the description and
  the addresses of their repository, project page and source, each with a
  copy button: the program still opens no browser itself.
- Every file field now has its "Browse…" button at the start of the line.
- The areas a file can be dropped onto are outlined and tinted in the accent
  colour, so they stand out before a file is dragged over them.
- "No camera found" now says what to check when the camera's display keeps
  saying that it is connecting: a charge-only cable carries no data, so the
  computer never sees the camera. The message suggests a data cable, a direct
  connection without hub or adapter, switching the camera off and on, and
  briefly removing the battery. It also gives a quick cable test: over a data
  cable even a switched-off camera is detected and reported as not ready, so
  "No camera found" with the camera off points to a charge-only cable.
- macOS: with the camera's USB connection set to Auto or Mass Storage, the
  app now reads the camera information as well. It asks for your
  administrator password for this, as it does for the tweaks; in MTP mode no
  password is needed.
- macOS: when taking the camera's mass-storage connection fails, the error
  message names the macOS version, the system drivers that hold the camera
  before and after the attempt, and every step with its timing.

### Fixed

- macOS: the camera's mass-storage connection, which the tweaks, the service
  shell and the other functions in service or updater mode go through, was
  reworked. The program unmounts the camera's card first, takes the camera
  from the system through Apple's IOUSBHost framework (macOS 26 no longer lets
  programs reach an interface held by one of its drivers through the older
  interface, even with administrator rights), keeps hold of it while working,
  and sets up its USB configuration afresh. If that is refused, it tries the
  earlier ways in turn; if nothing works, the camera is reset so its card comes
  back. Before, these functions failed with "The USB transfer failed" (error
  0xe00002be) once the camera had switched to mass-storage mode.

### Security

- `ctutils` updated to 0.4.5 (RUSTSEC-2026-0385, non-constant-time mask
  generation and `u128` comparisons).
- GUI: `tokio` updated to 1.54.1 (RUSTSEC-2026-0386, `AsyncFd::new` violates
  I/O safety).

## [1.2.0] - 2026-10-21

**macOS:** firmware updates and the updater shell have still not been tried
on a Mac with a camera. Until a successful update on a Mac has been reported,
update the firmware from Windows or with Sony's own updater.

**macOS 26:** for now only reading the camera information works, with the
camera's USB connection set to MTP. The service menu cannot be reached yet:
the tweaks, the service shell, the function analysis, firmware updates and the
updater shell all need the camera's mass-storage connection, and macOS 26 no
longer hands it to a program, even with administrator rights. This is being
investigated.

**Linux:** this release has been confirmed on a Sony α6000 (ILCE-6000,
firmware 3.21) with its USB connection set to *Auto*: reading the camera
information, reading and changing the settings memory and installing apps all
worked.

### Added

- Settings: an update notification, off by default. When you switch it on, the
  app asks GitHub at every start whether a newer release exists and, if so,
  shows its version and download address at the top of the window.
  Pre-releases are not announced.

### Changed

- The "show all menu languages" switch now counts language slots rather than
  languages, and notes that a camera shows only the languages its firmware
  knows: an RX1R shows 25 of the 35. Which language sits in which slot has
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

[Unreleased]: https://github.com/masterflai/pmca-re-reloaded/compare/v1.2.0...HEAD
[1.1.0]: https://github.com/masterflai/pmca-re-reloaded/compare/v1.0.0...v1.1.0
[1.0.0]: https://github.com/masterflai/pmca-re-reloaded/releases/tag/v1.0.0
