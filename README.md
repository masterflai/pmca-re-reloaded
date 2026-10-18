<p align="center">
  <img src="crates/pmca-gui/src-tauri/icons/icon.png" width="112" alt="PMCA-RE RELOADED logo">
</p>

<h1 align="center">PMCA-RE RELOADED</h1>

<p align="center">
  <b>A desktop toolkit for Sony cameras — over plain USB, on Windows, macOS and Linux.</b><br>
  Camera information, apps, firmware and the camera's settings memory, in one application.
</p>

<p align="center">
  <a href="https://github.com/masterflai/pmca-re-reloaded/releases/latest"><img src="https://img.shields.io/github/v/release/masterflai/pmca-re-reloaded?label=release&color=0078d4" alt="Latest release"></a>
  <a href="https://github.com/masterflai/pmca-re-reloaded/actions/workflows/ci.yml"><img src="https://github.com/masterflai/pmca-re-reloaded/actions/workflows/ci.yml/badge.svg" alt="CI status"></a>
  <img src="https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-555" alt="Windows, macOS, Linux">
  <img src="https://img.shields.io/badge/languages-21-555" alt="21 languages">
  <a href="LICENSE.txt"><img src="https://img.shields.io/badge/license-MIT-green" alt="MIT license"></a>
</p>

<p align="center">
  <a href="https://github.com/masterflai/pmca-re-reloaded/releases/latest"><b>Download</b></a> ·
  <a href="#getting-started">Getting started</a> ·
  <a href="#what-it-does">What it does</a> ·
  <a href="#supported-devices">Supported devices</a> ·
  <a href="#safety">Safety</a> ·
  <a href="CHANGELOG.md">Changelog</a>
</p>

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="docs/images/camera-info-dark.png">
    <img src="docs/images/camera-info-light.png" width="49%" alt="Camera info page showing model, product code, serial number, firmware version and lens of a connected camera (example data)">
  </picture>
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="docs/images/app-install-dark.png">
    <img src="docs/images/app-install-light.png" width="49%" alt="Apps page: the community app store installing three selected apps one after another, the second at 64 percent">
  </picture>
</p>

PMCA-RE RELOADED talks to Sony cameras and other Sony imaging devices through
their USB interfaces — from early NEX and Alpha bodies to current models, and
from compact cameras to action cameras. What a device offers depends on its
generation; the *Camera info* page shows what applies to the one connected.
It reads what the device knows about itself, installs apps although Sony's app
store has closed, runs firmware updates, and gives careful, reversible access
to the settings memory.
It is a desktop application with a command line beside it, written from scratch
in Rust, and it needs neither Python nor extra driver tools.

> **Independent project.** Not made by, endorsed by, or connected to Sony.
> "Sony", "PlayMemories", "α"/"Alpha" and the camera model names are trademarks
> of Sony Group Corporation, used only to say which cameras this tool speaks to.
> **Use it with cameras you own.** Some functions change settings inside the
> camera and may affect your warranty — read [Safety](#safety) first. The
> software comes without warranty; see [LICENSE.txt](LICENSE.txt).

## Why PMCA-RE RELOADED

| | |
|---|---|
| **One app, three systems** | A desktop app for Windows, macOS and Linux, plus `pmca-console` for scripts and experts. Both share the same core. |
| **No driver hunting** | Most tasks use the drivers your system already has. Where raw USB is unavoidable, the program assigns the driver itself — no Zadig — and removes it again afterwards. |
| **Safety net built in** | Every write to the settings memory is preceded by a saved copy, read back and compared, and can be rolled back. |
| **Your keys, not ours** | No Sony key material and no Sony firmware files ship with the program. Functions that need keys use material you import yourself. |
| **Private by design** | No telemetry, no account. The network is used only by the features that obviously need it, and only when you use them. |
| **Extensible** | Plugins add languages, compatibility notes, settings and sandboxed WebAssembly tools. |
| **21 languages** | Every text is translated, and each translation can be edited from within the app. |

## What it does

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="docs/images/demo-dark.gif">
    <img src="docs/images/demo-light.gif" width="720" alt="Short demo: reading the camera info, loading the community app list and installing three apps">
  </picture>
</p>

**Camera information** — model, firmware version, serial number and USB mode at
a glance; the shutter count, read from any ARW, JPEG or HEIF photo the camera
took; and the camera's usage log, decoded. No keys and no driver change needed.

**Apps** — Sony's PlayMemories store has been shut down; installing apps still
works. Install an `.apk` file you have, or pick one from the community app
list. Recommended start: [OpenMemories: Tweak](https://github.com/ma1co/OpenMemories-Tweak).

**Firmware** — check an official Sony firmware file against the connected camera
and install it, or start the updater shell: the camera boots its update mode
and runs this project's own code instead of an update, so the installed firmware
stays untouched. The update image for that is generated on your computer.
*Flashing official firmware is ported but has not yet been tried on a camera —
treat it as experimental.*

**Settings memory** — read and change settings the camera has but does not show
in its menu, such as the video recording limit, all menu languages or the
PAL/NTSC video system. Each change is backed up, verified and reversible.

**Function analysis** *(experimental)* — find out in which value of the
settings memory one of the camera's own menu settings is stored, and compare
the finding with other cameras. Reading only, unless you ask for a write test.

**Network settings** — GPS assist data, and the access points and settings of
the live-streaming feature.

**File tools** — inspect backup images, usage logs and firmware files offline.

**Expert shells** — the service shell and the updater shell give direct access
to the camera's Linux system. Hidden behind an *Advanced mode* switch in the
app, so reaching them stays a deliberate step.

## Getting started

1. **Download** the [latest release](https://github.com/masterflai/pmca-re-reloaded/releases/latest)
   for your system (see [Downloads](#downloads)) and start it.
2. **Connect the camera** by USB. In the camera's menu, set *USB Connection* to
   *Mass Storage* or *MTP*. The *Welcome* page explains what to expect.
3. **Optional: import key material.** App installation, the settings memory and
   function analysis need keys that do not ship with the program. While any are
   missing, the *Welcome* page shows an import panel — see [Key material](#key-material).

Everything is also available on the command line; `pmca-console --help` lists
the commands, for example `pmca-console info`, `pmca-console install --apk <file>`
or `pmca-console tweaks`.

## Supported devices

What works depends on the camera's generation, not on this program:

| | Camera info, shutter count, usage log | Apps | Firmware, updater shell | Settings memory, service shell |
|---|:-:|:-:|:-:|:-:|
| **Cameras with PlayMemories Camera Apps** (NEX, α, RX and other Cyber-shot models of roughly 2012–2017) | ✔ | ✔ | ✔ | ✔ |
| **Newer Sony cameras** (roughly 2018 onwards) | ✔ | ✖ | ✖ | many models |
| **Other Sony imaging devices** (camcorders, action cameras, compacts without app support) | ✔ | ✖ | model-dependent | model-dependent |

The exact list is in the app: *Camera info → Compatible devices* shows every
known model with its processor generation and which functions apply, and
*More devices* adds further models together with what the community has
reported. Before a function changes anything, the program checks the
connected model against this list and warns when it is untested.

The shutter count does not even need the camera: it is read from any photo
the camera took. If you have used the program with a model, a
[camera report](https://github.com/masterflai/pmca-re-reloaded/issues/new?template=camera_report.yml)
helps everyone else.

## Downloads

| System | Desktop app | Command line |
|---|---|---|
| **Windows** (x64) | `…_x64-setup.exe` (installer) · `…_x64_en-US.msi` · `pmca-gui-portable-x86_64-windows.zip` (no installation) | `pmca-console-x86_64-windows.zip` |
| **macOS** (Apple silicon and Intel) | `…_universal.dmg` | `pmca-console-aarch64-macos.tar.gz` · `pmca-console-x86_64-macos.tar.gz` |
| **Linux** (x64) | `….AppImage` (portable) · `…_amd64.deb` | `pmca-console-x86_64-linux.tar.gz` |

**Windows** — The portable `.zip` runs from any folder, a USB stick included;
keep the `bodyfs` folder next to `pmca-gui.exe`. The window needs the WebView2
runtime, which Windows 11 has and Windows 10 may not. Settings and edited
translations are kept in your user profile.

**macOS** — The `.app` in the `.dmg` runs from any folder. Close applications
that might access the camera (Photos, cloud sync tools) before connecting it,
and set the camera's USB connection to MTP. Tweaks, the service shell, the
function analysis, firmware updates and the updater shell go through the
camera's mass-storage connection, which macOS hands to a program only when it
runs with administrator rights. The app asks
for your password once per run and does the rest itself; on the command line
use `sudo`, for example `sudo ./pmca-console serviceshell`. The camera's card
leaves Finder while this runs and comes back afterwards, so eject it there
first if files on it are open.

**Linux** — Make the `.AppImage` executable and run it, or install the `.deb`.
[`scripts/linux-prepare.sh`](scripts/linux-prepare.sh) sets up the udev rule
for Sony's vendor id and stops the desktop from claiming the camera.

The Linux build has been tried with a camera, the macOS build only in part so
far. Reports are welcome — see [Feedback](#feedback).

### Signatures

The Windows downloads are to be signed through SignPath (see the
[code signing policy](#code-signing-policy)); the release notes say whether a
given release is. Until then, and on the other systems:

* **Windows** shows a SmartScreen warning for an unsigned release: *More info* → *Run anyway*.
* **macOS** reports the app as *damaged*. It is not — the file carries a
  quarantine flag. Right-click it and choose *Open*, or run
  `xattr -dr com.apple.quarantine "/Applications/PMCA-RE RELOADED.app"`.
* **Linux** starts the packages without complaint.

Every download has a `.sha256` file beside it, which confirms that the file
arrived intact.

## Key material

Some camera protocols need keys that are not part of this program: the
service-mode authentication keys, the checksum key for app installation, the
key material that packs an app for the camera, and the keys for firmware
update images. Without them, app installation, the settings memory, function
analysis and the updater shell are unavailable. Everything else works without
any setup.

The program contains only the SHA-256 digests of this material. You supply the
material from a source of your own choosing — in the app via the panel on the
*Welcome* page, or with `pmca-console keys import <url or file>`. The program
reads the source, keeps only byte sequences whose digest matches, and stores
them in your user profile. Nothing from the source is executed, and nothing is
sent anywhere.

The program names no source. **Observe the laws that apply where you are, and
work only with cameras and data that are your own.**

## Drivers on Windows

Most functions use the drivers Windows has already installed: mass storage
mode works through SCSI pass-through, MTP mode through the Windows Portable
Devices API. If a camera is refused in one of these modes, do not replace its
driver.

Only service mode — used by the settings memory, function analysis and the
service shell — needs raw USB access through **WinUSB**. The program handles
that itself:

* It builds a minimal driver package for the camera it found and signs it with
  a short-lived certificate created on the spot (valid one day, code signing
  only, private key never written to disk). The certificate sits in the
  machine's trust stores only for the few seconds Windows needs to accept the
  package, and is removed right after.
* Neither the app nor the console has to run as administrator. The first time a
  driver must change, a small helper asks for administrator rights once per
  session; it only assigns and removes drivers for Sony devices and ends with
  the program.
* When the session ends, the driver packages are removed and the camera returns
  to the drivers Windows chose. A clean-up at program start removes anything a
  crash may have left behind; `pmca-console driver unbind` does the same by hand.
* Drivers that Zadig installed for a Sony camera — the original Sony-PMCA-RE
  asked for that — stay in Windows and take the camera before this program's
  driver can, which makes service mode fail. The app finds them at start and
  offers to remove them; on the command line, `pmca-console driver zadig`
  lists them and `--remove` removes them.

Power-cycling the camera always returns it to normal mode.

## Plugins

A plugin is a `.pmcaplugin` file you install from your own disk — in the app
under *Settings → Plugins*, or with `pmca-console plugins install <file>`.
There is no plugin store and nothing is downloaded automatically. A plugin can
bring a complete translation, compatibility notes, switches for camera settings
(limited to two exact values and, optionally, to named models), and tools or
diagnostics as WebAssembly modules that run in a sandbox without access to
files, the network or key material. Before installing, the program shows what
a plugin may do. A plugin cannot reword the program's own messages and cannot
change a camera's identity.

To write one, see [`crates/pmca-plugin-sdk`](crates/pmca-plugin-sdk/src/lib.rs)
and its [examples](crates/pmca-plugin-sdk/examples);
`pmca-console plugins pack <folder> <file>` builds and checks a plugin file.

## Safety

What this program does was worked out by reverse engineering, and it goes
beyond what Sony intends for these cameras. It is built to be careful:

* **Reading comes first.** Camera information, shutter count, usage log and the
  file tools never write to the camera.
* **Every write is guarded.** Before the settings memory is changed, the
  original is saved to your user profile; after writing, the program reads the
  memory back and compares it. An interrupted change can be rolled back.
* **Writing is a decision.** Changes need confirmation, the expert shells are
  hidden behind *Advanced mode*, and function analysis only writes when you
  ask for its write test.
* **Tested.** The protocol logic is covered by an extensive automated test
  suite, and the core functions have been verified on real cameras. Not every
  model and not every function has been tried on hardware — firmware flashing
  in particular is still experimental.

Some functions can leave a camera in a state that needs recovering, and
flashing firmware can in principle leave one that does not start. Using this
program may affect your warranty. Whether studying a device's interface to make
an independent program work with it is permitted depends on where you are; use
the program at your own risk.

## Feedback

Found a bug, or used the program with a camera model? Please open an
[issue](https://github.com/masterflai/pmca-re-reloaded/issues/new/choose) —
there is a form for camera reports. Questions, ideas and experiences belong in
[Discussions](https://github.com/masterflai/pmca-re-reloaded/discussions).
Do not post serial numbers or key material.
Security problems: please report them
[privately](https://github.com/masterflai/pmca-re-reloaded/security/advisories/new).

Translations and code are welcome — [CONTRIBUTING.md](CONTRIBUTING.md) explains
how, including how to improve a language from within the app.

## Building from source

With the [Rust toolchain](https://rustup.rs/) installed:

```bash
cargo build --release -p pmca-cli          # the command line
./target/release/pmca-console --help
```

The desktop app additionally needs Node 22:

```bash
cd crates/pmca-gui
npm ci
npm run tauri build
```

On Linux, install `libudev-dev` for the command line and, for the app,
`libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf` (Debian and
Ubuntu names; WebKitGTK **4.1** is required, so Ubuntu 22.04 / Debian 12 or
newer). The authoritative lists are in [.github/workflows/ci.yml](.github/workflows/ci.yml).
[ARCHITECTURE.md](ARCHITECTURE.md) explains how the crates fit together.

## Code signing policy

Free code signing provided by [SignPath.io](https://about.signpath.io/),
certificate by [SignPath Foundation](https://signpath.org/).

Only files built by this repository's
[release workflow](.github/workflows/release.yml) from its own source code are
signed, and every signing request is approved by hand.

* Committers and reviewers: [masterflai](https://github.com/masterflai)
* Approvers: [masterflai](https://github.com/masterflai)

Privacy: this program will not transfer any information to other networked
systems unless specifically requested by the user or the person installing or
operating it. The features that reach the network do so only when used: the
GPS assist data (from Sony), the community app list (from GitHub) and the key
import (from the address you enter). There is no telemetry.

## Acknowledgements and license

PMCA-RE RELOADED builds on the research of
[Sony-PMCA-RE](https://github.com/ma1co/Sony-PMCA-RE) by ma1co, which first
worked out camera detection, the external command protocol, the updater
procedure and service mode, and of [fwtool.py](https://github.com/ma1co/fwtool.py),
whose work on Sony's firmware format underlies the generation of update images.
Sony-PMCA-RE targets Python 2 and depends in parts
on services Sony has since shut down. This project continues the idea as a new
implementation — with a desktop app, its own driver handling, key import
instead of shipped keys, locally generated update images, a guarded write path,
function analysis, plugins and 21 languages — so that these cameras stay usable.

Thanks to ma1co and the contributors to
[OpenMemories: Tweak](https://github.com/ma1co/OpenMemories-Tweak),
[OpenMemories: Platform](https://github.com/ma1co/OpenMemories-Platform), the
[app list](https://github.com/ma1co/OpenMemories-AppList) and the
[device documentation](https://openmemories.readthedocs.io/devices.html); to the
people at [nex-hack](https://web.archive.org/web/20201027234246/http://www.personal-view.com/faqs/sony-hack/hack-development)
(archived);
and to [zv-e10-re](https://github.com/qn06142/zv-e10-re), whose research
confirmed several findings on newer camera generations.

The code was written with the help of AI (GitHub Copilot) and reviewed and
tested by the author. If you recognise code here that you hold rights to,
please open an issue and it will be attributed or replaced.

Published under the MIT license; the original copyright notice is kept, as that
license requires — see [LICENSE.txt](LICENSE.txt), also shown by
`pmca-console license` and on the app's *License* page. The notices of the Rust
libraries compiled into the binaries are collected in
[THIRD-PARTY.md](THIRD-PARTY.md).
