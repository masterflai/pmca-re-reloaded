# English (source / fallback) localization for PMCA-RE RELOADED.
# One message per line: <id> = <text>. Add a new language by copying this
# file into locales/<lang>/pmca.ftl and translating the right-hand side only.

app-title = PMCA-RE RELOADED – Camera Toolkit

# Navigation / tabs
nav-camera-info = Camera info
nav-install-app = Apps
nav-tweaks = Tweaks
nav-network = Network
nav-firmware = Firmware
nav-updater = Updater shell
nav-service = Service shell
nav-tools = File tools
nav-settings = Settings
nav-license = License

# Welcome page: the page the window opens on. It says what the toolkit is for,
# what to do first, and which page answers which question.
#
# "Language" means two different things in this application: the language of
# this window, and the languages the camera's own menu offers. They get a
# heading each rather than a clause each, because confusing the two is the
# mistake this page exists to prevent.
nav-welcome = Welcome
welcome-intro-heading = What this program does
welcome-intro-body = This program helps preserve your Sony camera's functionality and adapt it to your needs. Once Sony had shut down its store for PlayMemories Camera Apps, there was no way to back up or reinstall purchased or already installed apps. Resetting the camera deletes the installed apps irretrievably. This program makes it possible to back up existing apps and install them again, and it can also provide freely available community apps. You can also install your own apps (*.apk files). When problems arise, the program reads out camera information and internal logs for analysis. Settings the camera already has but does not offer in its menu can be switched and undone again. The program talks to the camera directly over the USB cable. The supported models are listed on the Camera compatibility page.
welcome-start-heading = The first step
welcome-start-body = Switch the camera on first and, while the cable is still out, set USB connection in its menu to Mass Storage or MTP. Auto usually works as well, because the camera then settles on one of those two by itself. The order matters: once the cable is in, the camera's menu can no longer be reached. Only then connect the camera by USB, open Camera info and press "Get camera info". Once the model and the firmware version appear, the connection is sound and all other features of this program are available.
welcome-start-driver = If a driver notice appears instead, follow that one first. On Windows a system driver claims the camera, and it has to be replaced before the direct USB access this toolkit needs is possible. The program does that itself, but assigning a driver is a right Windows grants only to an administrator. In this case the program offers you the option of restarting it as administrator, and Windows asks you to confirm the operation. The assignment is released again when the session ends. This is not necessary on Linux and macOS.
welcome-start-driver-linux = If a driver notice appears instead, follow that one first. On Linux nothing has to be replaced: the camera is reached directly. What is needed once is the right to open the device — a udev rule for Sony's vendor id — and, for mass-storage mode, releasing the kernel driver usb-storage. Both are prepared outside this program with root rights; the script scripts/linux-prepare.sh in the source tree does both. This window itself never asks for such rights.
welcome-start-driver-macos = If a driver notice appears instead, follow that one first. On macOS nothing has to be installed or replaced. Set the camera's USB connection to MTP. For the tweaks, the service shell, the function analysis, firmware updates and the updater shell this window asks once for your administrator password: they go through the camera's mass-storage connection, which macOS hands only to such a program.
welcome-tweaks-heading = Tweaks and languages
welcome-tweaks-body = Tweaks are settings the camera already has but does not offer in its menu: the recording time limit, the 4K recording limit, the choice between PAL and NTSC, the USB app installer — and the menu languages.
welcome-tweaks-languages-heading = The camera's languages
welcome-tweaks-languages-body = Sony cameras only show the menu languages that are common in their sales region. If you live abroad, you often cannot find your own language and have to make do with a foreign one. Yet more languages are already built into the firmware. This program brings them into the menu so you can operate your camera in a familiar language.
welcome-tweaks-ui-heading = The language of this program
welcome-tweaks-ui-body = Separate from all of that is the picker at the top. It changes the language of this program only, and never touches the camera.
welcome-tweaks-safety = Every tweak is written into the camera's backup memory, and every one of them can be taken back with the same switch. An interrupted write can permanently damage the camera, so do not unplug the cable or remove the battery while one is under way.
welcome-privacy-heading = Your data
welcome-privacy-where = The License page states all of this again in full, together with the licences of every library this program is built from.
welcome-where-heading = Where to find what
welcome-where-info = Model, serial number and firmware version. This is also the connection test.
welcome-where-compat = Which camera models support apps, the updater shell and the service shell. Needs no camera.
welcome-where-install = Apps from the community list, or an .apk file of your own.
welcome-where-tweaks = The settings the camera's menu does not reach, the languages among them.
welcome-where-network = GPS assist data, and the access points and services used for live streaming.
welcome-where-firmware = Write a Sony firmware image to the camera.
welcome-where-tools = Unpack and inspect firmware and package files. Needs no camera.
welcome-where-shells = Direct access to the camera's memory and filesystem. Switched on in Settings first.
welcome-where-settings = Theme, advanced mode, and editing the translation of this program.
welcome-where-license = Where this port came from, whose work it builds on, and the terms it is under.
welcome-where-hint = No page acts on the camera by itself. Nothing is read or written until you press a button that says it will.

# Buttons
btn-get-info = Get camera info
btn-refresh = Refresh
btn-load-apps = Load available apps
btn-list-tweaks = Show available tweaks
btn-install = Install selected app
btn-install-file = Install .apk file…
btn-apply = Apply
btn-save = Save to camera
btn-add = Add
btn-remove = Remove
btn-reveal = Show
btn-update-gps = Transfer GPS assist data to the device
btn-cancel = Cancel
btn-browse = Browse…
btn-switch-msc = Switch to mass-storage mode
btn-start-driver-helper = Start driver helper

# Labels for the file type filters offered by the file dialogs
file-filter-apk = Android application (*.apk)
file-filter-firmware = Sony firmware image (*.dat)
file-filter-usage-log = Usage log (*.bin)
file-filter-catalog = Translation file (*.ftl)
file-filter-spk = Sony app package (*.spk)
file-filter-any = Any file
file-filter-gps = GPS assist data (*.dat)

# Status / messages
msg-connecting = Connecting…
msg-usage-log-written = Usage log written to { $path }
msg-apps-found = Found { $count } apps
msg-switch-mode = Please switch the camera to MTP or mass-storage mode.
msg-no-tweaks = No tweaks available for this camera.
msg-done = Done.

# Camera USB mode switch (MTP -> mass storage). Offered below a "wrong mode"
# error; there is no USB command for the opposite direction.
mode-switching = Asking the camera to switch to mass-storage mode…
mode-switch-wait = Waiting for the camera to reconnect…
camera-confirm-hint = Messages on the camera (e.g. about www.sony.net/pm) can be dismissed; they do not interfere.
mode-switch-done = The camera is now in mass-storage mode.
mode-switch-timeout = The camera did not reappear in mass-storage mode. Please power-cycle it and try again.
mode-switch-impossible = This mode cannot be changed over USB. Please power-cycle the camera.
mode-switch-hint = Only a camera in MTP mode can be switched over USB. The change is temporary: the USB mode selected in the camera's own menu stays unchanged, so turning the camera off and on again — or unplugging and reconnecting the cable — restores the previous mode. There is no USB command for the way back.

# Camera info fields
info-model = Model
info-product-code = Product code
info-serial = Serial number
info-firmware = Firmware version
info-lens = Lens
info-region = Region
info-mode = Mode
mode-mass-storage = Mass storage
mode-mtp = MTP
mode-updater = Updater mode
mode-app-install = App-install mode
mode-service = Service mode
mode-unknown = Unknown
info-android-version = Android platform version
info-photographer = Photographer
info-copyright = Copyright
info-photographer-1 = Photographer (entry 1)
info-photographer-2 = Photographer (entry 2)
info-copyright-1 = Copyright (entry 1)
info-copyright-2 = Copyright (entry 2)
diagnose-services-heading = Camera cannot be reached?
diagnose-services-hint = Some Windows services can hold on to a camera in MTP mode, so this application cannot reach it. The check only asks Windows which services are running; it changes nothing.
btn-diagnose-services = Check Windows services
diagnose-services-none = No known interfering service is running.
diagnose-services-found = These running services can interfere with access to the camera:
diagnose-services-advice = Stop the service in Services (services.msc), or with the following command as administrator, then reconnect the camera:

# Firmware updater / shell
updater-model = Camera model
updater-firmware-current = Current firmware version
updater-guard-ok = Firmware image matches this camera.
updater-no-fdat = No bundled updater firmware is available for model { $model }.
updater-model-required = The camera is already in updater mode and cannot report its model there. Switch the camera off and on again so that it starts in normal mode, then try again. Alternatively, specify a payload file (--fdat-file) or, in the console, the model (--model <MODEL>).
updater-already-updater = Camera is already in updater mode.
updater-flash-required = Refusing to modify the camera without confirmation. Re-run with --flash to switch to updater mode and write the payload.
updater-flashing = Writing updater payload to the camera…
updater-switch-wait = Please follow the instructions on the camera screen. Waiting for the camera to switch to updater mode…
updater-switch-timeout = Timed out waiting for the camera. Please run the command again once it has reconnected.
updater-shell-ready = Updater payload is running. Type 'help' for commands, 'exit' to quit.
updater-shell-done = Shell session ended.
updater-session-panic = The updater session ended unexpectedly. Power-cycle the camera before trying again.

# What the updater shell answers with. The parse failures are worded once, in
# the service-* keys: both shells share a tokenizer, so a quoted path and a bad
# hexadecimal value fail the same way in either.
updater-backup-synced = Backup synchronised to flash.
updater-backup-locked = Protected backup settings are now locked.
updater-backup-writable = Protected backup settings are now writable.
updater-shell-console-only = The root shell needs a terminal and is only available from the console: run pmca-console updatershell --flash and type 'shell' there.

# The updater-shell page.
updater-warning = Warning: reaching this shell reboots the camera into updater mode and writes a payload to it — a small program from this project that the camera accepts like a firmware update and that then answers this shell's commands. The payload itself is discarded when the camera is switched off, but the commands it offers write the camera's persistent settings memory, and an interrupted write can permanently damage your camera. In addition, all of the camera's settings are reset in the process — unlike with the service shell.
updater-console-heading = Running the updater shell in the console
updater-driver-hint = The updater shell needs the camera on Windows' own mass-storage driver, USBSTOR, because the payload is delivered through it. This is the opposite of what the service shell needs. The application assigns and releases the WinUSB driver by itself, so nothing has to be prepared by hand — but a driver assigned by another tool has to be undone first. A camera in updater mode enumerates under its own product id and must not be rebound either.
updater-fdat-label = Payload file (optional)
updater-confirm = The camera will restart into updater mode and a payload will be written to it. Continue?
updater-cancelled = Cancelled. The camera was not modified.

# The folded-away explanation shown on both shell pages. The command syntax
# itself is never translated — those are words that have to be typed exactly —
# so only the explanation beside each one lives here.
btn-shell-primer = About this shell
btn-shell-primer-hide = Hide the explanation
shell-primer-compare-heading = Two shells, two protocols
shell-primer-compare = Both pages offer a command line to the camera, but they are not the same shell. The service shell speaks Sony's own factory service protocol, which the camera answers by itself once it has authenticated you; nothing is written to the camera to get there. The updater shell exists only because this project writes a program of its own into the camera: the camera restarts into updater mode, accepts the payload as though it were a firmware image, and the payload then answers the commands listed below. Only one of the two can be open at a time.
shell-primer-payload = "Payload" means the actual content that a delivery route carries. For the updater shell it is a small program from this project: the camera accepts it the same way it accepts a firmware update, but keeps it only until it is switched off. The service shell needs no payload, because the camera answers its commands with its own firmware. For firmware files the term means the same thing: the actual firmware image inside the container of the .dat file.
shell-primer-when-service = Reach for the service shell for anything that reads or edits the camera's stored settings while it keeps running its own firmware — reading the HASP block, saving a rollback copy of the backup region, changing single entries, copying files off the camera. It is also what the Tweaks page uses underneath, so anything you can do there you can do here by hand.
shell-primer-when-updater = Reach for the updater shell when the camera cannot be reached any other way, or when you need a shell on the running system rather than single commands. It is the only route that offers a real root shell, and it still works on a camera that refuses service mode. It costs a restart, a write and the camera's settings, so it is the second choice rather than the first.
shell-primer-driver-heading = Drivers: the two need opposite things
shell-primer-driver = The service shell speaks raw USB and needs the camera bound to WinUSB on Windows. The application assigns that driver itself on the way into service mode and releases it again when the session ends, so no separate tool is involved. The updater shell is delivered through Windows' own mass-storage driver, USBSTOR, and stops working while the camera is bound to WinUSB — releasing the assignment is therefore part of finishing a service session, not an afterthought. Assigning a driver needs administrator rights. On Linux no driver swap is needed for either; only access to the device node is.
shell-primer-risk-heading = What can go wrong
shell-primer-risk = The commands marked below write the camera's persistent settings memory. An interrupted write can leave the camera unable to start, and nothing here can be undone — which is exactly why bk d exists: save a copy of the backup region before you change anything. The updater payload itself does not stay on the camera; switching the camera off discards it. The route through updater mode does, however, reset all of the camera's settings.
shell-primer-commands-heading = Commands on this shell
shell-primer-writes = Commands marked (WRITES) change the camera. Everything else only reads.
shell-cmd-hasp = Read the HASP block, the camera's protected identity area.
shell-cmd-pull = Copy a file off the camera. Without a local name the remote file name is kept.
shell-cmd-push = Copy a local file onto the camera, replacing whatever is there. (WRITES)
shell-cmd-rm = Delete a file on the camera. (WRITES)
shell-cmd-rmem = Write a region of device memory to a local file. Offset and length are hexadecimal.
shell-cmd-bk-r = Read one entry of the backup region and show it as a hex dump.
shell-cmd-bk-d = Save the whole backup region to a file. Do this before you change anything: it is the only way back.
shell-cmd-bk-dump-txt = Save the whole backup region as <file>.bin and a readable listing as <file>.txt.
shell-cmd-bk-stat = Show the camera's backup status block, including the region.
shell-cmd-bk-w = Write one entry of the backup region, given as space-separated hex bytes. (WRITES)
shell-cmd-bk-s-sub = Persist one subsystem's backup region to flash so the change survives a restart. (WRITES)
shell-cmd-bk-s = Persist the backup region to flash so the change survives a restart. (WRITES)
shell-cmd-bk-lock = Set or clear the write protection of the protected backup entries; some settings refuse to change while it is set. (WRITES)
shell-cmd-term = Switch the camera's debug terminal on or off. (WRITES)
shell-cmd-info = Read the backup status block, which says whether the region can be written at all.
shell-cmd-shell = Open a root shell on the camera. It needs a terminal, so it is available in the console only; shell tcp <port> offers it on 127.0.0.1 for another client instead. (WRITES)
shell-cmd-help = List the commands this shell accepts.
shell-cmd-exit = End the session and leave the shell.
shell-primer-diagnostics-heading = Diagnostics and repair
shell-cmd-bk-trace = Record in a file, packet by packet, how the camera sends the whole backup region over USB. For investigating large backups; it may leave the camera stuck mid-reply. Then unplug the cable and switch the camera off and on, removing the battery if necessary.
shell-cmd-bk-rewrite = Read the backup region, write it back unchanged and read it again. Shows whether the camera accepts a whole image, without changing any setting. (WRITES)
shell-cmd-bk-restore-attrs = Set the attributes of all backup entries, such as write protection, back to those in a copy saved with bk d or bk dump-txt. The values stay as they are. Only use a copy from the same camera. (WRITES)
shell-primer-diagnostics-env = Three environment variables affect both the console and the graphical interface when set before starting, for example in PowerShell with $env:PMCA_TWEAKS_DIRECT = "1". PMCA_TWEAKS_DIRECT makes tweaks write single values even when the backup is smaller than 1 MB. PMCA_TWEAKS_IMAGE makes them write the whole image even when it is larger than 1 MB; with both set, PMCA_TWEAKS_DIRECT wins. PMCA_TLS_DUMP=<file> saves the camera's first TLS message (ClientHello) during an app installation, for investigating a failed connection.

# App installation
install-switching = Switching to app-install mode…
install-disabled = The camera refused to enter app-install mode. This is usually the "Enable USB app installer" setting: it has to be switched on under "Tweaks".
install-stays-app-mode = The camera stays in app-install mode, where further apps can be installed. For all other functions, unplug the USB cable and switch the camera off and on again.
err-usb-app-install-mode = The camera is still in app-install mode, where it accepts nothing but app installations. Unplug the USB cable, then turn the camera off and on again — restarting it while still connected puts it straight back into this mode.
install-switch-wait = Waiting for the camera to reconnect in app-install mode…
install-switch-timeout = Timed out waiting for the camera to enter app-install mode. Please run the command again once it has reconnected.
install-bad-mode = The camera cannot be used in this mode. Please switch it to MTP or mass-storage mode.
install-starting = Starting installation…
install-error = Installation communication error
install-server-failed = The local install server did not accept the camera's connection:
install-done = Task completed successfully.

# APK inspection
apk-package = Package:
apk-unreadable = Warning: the .apk file could not be analyzed.
apk-incompatible = Warning: this app declares a minimum SDK level the camera may not provide.
apk-unsigned = Warning: the .apk file carries no signature.

# Community app store
store-loading = Loading the app list…
store-resolving = Looking up the latest release…
store-downloading = Downloading
store-unknown-app = Unknown app
store-no-release = No downloadable release for this app
store-not-installable = no release
msg-no-apps = No apps found.
store-heading = Community app store
store-discontinued = Sony shut the official PlayMemories Camera Apps store down; its servers no longer answer, so apps cannot be bought or downloaded from it any more.
store-use-own-apk = You can still install your own .apk: pick the file below or drag it onto this window. The community app store further down also still works.
btn-store-sync = Check camera
btn-install-selected = Install selected
btn-store-stop = Stop after this app
btn-store-stopping = Stopping after this app…
btn-store-retry = Retry failed apps
btn-continue = Continue
btn-reinstall = Install again
btn-skip = Skip
btn-confirm-updater = Restart and write
btn-confirm-tweaks = Write tweaks
btn-confirm-languages = Enable languages
btn-confirm-wifi = Replace access points
btn-confirm-stream = Overwrite settings
btn-confirm-show-shells = Show shells
btn-confirm-discard = Discard translation
store-select-all = Select all apps
store-col-app = App
store-col-version = Version
store-col-author = Author
store-col-camera = Version on camera
store-col-status = Status
store-camera-absent = not installed
store-version-unknown = unknown
store-batch-query = Reading the installed apps from the camera…
store-sync-done = Camera checked. Listed apps installed: { $count }
store-batch-downloading = Downloading app { $current } of { $total }…
store-batch-installing = App { $current } of { $total }: installing { $name }…
store-reinstall-confirm = { $name } is already installed on the camera. Installed version: { $installed }. Version in the app store: { $store }. Install it again?
store-state-queued = waiting
store-state-installed = installed
store-state-failed = failed
store-state-skipped = skipped
store-state-not-run = not run
store-batch-summary = Installed: { $done } · failed: { $failed } · skipped: { $skipped } · not run: { $notrun }
store-sources-hint = The apps come from the sources you have added under Settings. This program does not check them.
app-backup-heading = Back up installed apps
app-backup-intro = Copies the apps installed on the camera to this computer, so you can install them again later, even if they are no longer available anywhere. For this the camera restarts into updater mode; its app storage is only read.
app-backup-own-camera = Back up the apps of your own camera only and do not pass the files on: the rights to an app belong to its author.
app-backup-restore-hint = To restore an app, choose the saved .apk file above under "Install .apk file…".
btn-app-backup = Back up apps
app-backup-confirm = The camera will restart into updater mode, and a payload that runs until the camera is switched off will be written to it. The app storage is only read. Continue?
btn-confirm-app-backup = Restart and back up
app-backup-cancelled = Backup cancelled.
app-backup-confirm-required = The backup restarts the camera into updater mode. Confirm with --confirm.
app-backup-no-folder = There is no data folder for the backups on this computer. Choose a folder with --out.
app-backup-no-model = The camera model could not be read. Switch the camera off and on, connect it in mass storage or MTP mode and try again.
app-backup-mounting = Looking for the camera's app storage…
app-backup-no-partition = No app storage was found on this camera. It probably does not support apps.
app-backup-copying = Copying { $name } ({ $current } of { $total })…
app-backup-none = No apps are installed on the camera.
app-backup-done = Apps saved: { $count }.
app-backup-power-cycle = Switch the camera off and on again to leave updater mode.
app-backup-folder = Folder
app-backup-col-file = File
app-backup-state-new = saved
app-backup-state-kept = already saved
btn-copy-path = Copy path
msg-path-copied = Path copied to the clipboard.
store-no-sources = No sources added yet. Add one under Settings → App and plugin sources; this program's community list and the app list of the original project are offered there as suggestions.
store-col-source = Source
store-filter-source = Show apps from
store-filter-all = All sources
sources-heading = App and plugin sources
sources-intro = Apps and plugins come from sources that you add here yourself: public GitHub repositories in which developers publish their work. This program does not host, check or recommend any source or anything in it; the author of a source alone is responsible for it. Look at a repository before you add it.
sources-none = No sources added yet.
sources-add-label = GitHub repository (owner/name or its address)
btn-source-add = Add source
btn-confirm-source-add = Add source
sources-add-confirm = Add { $source } as a source? This program has not checked it. What it offers is installed on the responsibility of its author. Look at the repository before you agree: { $url }
sources-adding = Reading the source…
sources-added = Source added: { $source }
sources-check-repo = Look at the repository before you install anything from it: { $url }
sources-removed = Source removed: { $source }
sources-remove-confirm = Remove the source { $source }? The signers remembered for its apps are forgotten with it.
sources-stars-title = Stars on GitHub: how many users have marked the repository. They say nothing about quality or safety.
sources-suggestion-heading = Suggestion
sources-suggestion-community = Camera apps whose authors have archived their projects, and apps whose authors asked to be listed. The list is kept by masterflai, the author of this program; it only points to the releases the app authors published themselves and holds no files of its own. The apps have not been checked.
sources-suggestion-applist = The app list of the original project Sony-PMCA-RE. It is kept by its author ma1co, not by this program, and has not been updated for years.
btn-source-activate = Activate
sources-search-heading = Find sources on GitHub
sources-search-hint = Searches GitHub for repositories with the topic pmca-source. The results are not checked by this program and are no recommendation. Look at a repository before you add it; the stars only show how many GitHub users have marked it.
btn-source-search = Search GitHub
sources-searching = Searching GitHub…
sources-search-none = GitHub found no repository with the topic pmca-source.
sources-search-found = { $count } repositories found. None of them has been checked by this program.
sources-updated = Last change: { $date }
sources-archived = archived
app-archived-note = Its developer has archived the project on GitHub and no longer works on it actively. The app may not work fully on every camera, and problems are unlikely to be fixed.
sources-already = already added
sources-no-folder = The user data folder could not be determined, so sources cannot be saved.
source-invalid = Not a GitHub repository. Enter owner/name, for example ma1co/OpenMemories-AppList, or the address of the repository.
source-exists = { $source } has already been added.
source-unknown = No such source: { $source }
source-unreadable = { $source } has no readable { $file } in its default branch.
source-failed = The source { $source } could not be read: { $reason }
source-signer-changed = { $name } from { $source } is signed by someone other than before and was not installed. If its author really changed the signing key, remove the source and add it again.
source-signer-pinned = { $name } from { $source } is not signed with the certificate its source names and was not installed.
source-signature-invalid = { $name } from { $source } does not match its signature and was not installed; the file may have been altered on the way. ({ $reason })
source-ambiguous = { $package } is offered by several sources: { $sources }. Choose one with --source.
source-plugin-mismatch = The downloaded plugin has the id { $found }, but the source announces { $id }. It was not installed.
plugin-sources-heading = Plugins from the sources you have added
plugin-sources-hint = Anyone can publish plugins in a source of their own. This list shows the plugins of all sources you have added under App and plugin sources, whoever publishes them; this program does not check them.
plugin-signed = Signed, key { $key }
plugin-unsigned = Not signed
plugin-signer-changed = { $id } is installed with a different signing key than this file carries, or this file is not signed. The plugin was not replaced. If its author really changed the key, remove the installed plugin first.
plugin-key-written = Signing key written to { $path }. Keep this file secret and back it up: whoever has it can sign plugins in your name, and without it you cannot publish updates that existing installations accept. Public key: { $key }
btn-details = Details
details-repository = Repository
details-homepage = Project page
details-source = Source
msg-url-copied-browser = Address copied to the clipboard. Paste it into your browser to look at the page.
store-activate-suggestion = No sources have been added yet. Activate the app list of the original project ({ $source })? It is kept by its author ma1co, not by this program, and this program has not checked it. The list then appears under Settings → App and plugin sources.
store-activate-community = No sources have been added yet. Activate the community list ({ $source })? It is kept by masterflai, the author of this program, and only points to the releases the app authors published themselves; the apps have not been checked. The list then appears under Settings → App and plugin sources.
btn-activate-list = Activate list
btn-plugin-sources-load = Show plugins from sources
plugin-sources-none = Your sources offer no plugins.
plugin-source-unchecked = Source: { $source }. This program has not checked this plugin.
btn-plugin-source-install = Install

# Service (senser) mode
usb-mode-advice = For tweaks, backups and the service shell, set the camera's USB connection mode to “Auto”, not permanently to mass storage. When the camera asks for the mode after a session, choose “Mass Storage (MSC)”. With mass storage set permanently, the camera often refuses the next switch to service mode.
service-switch-refused = The camera did not complete the switch to service mode and did not reconnect over USB; it then usually shows “Check the connected device”. Unplug the USB cable, plug it in again and try again.
service-switching = Switching to service mode…
service-switch-wait = Waiting for the camera to reconnect in service mode…
service-switch-timeout = The camera did not appear in service mode. It re-enumerates as a separate USB device (product id 0336), which needs its own WinUSB driver assignment; check the device manager for a Sony device without a driver. Power-cycle the camera to return it to normal mode.
service-authenticating = Authenticating…
# Shown when the camera is found still in service mode from an earlier run, and
# when that could not be undone.
service-stranded-reset = The camera is still in service mode from an earlier run. Ending that session first…
service-stranded-failed = The camera is stuck in service mode from an earlier run and did not respond to the request to leave it. Switch the camera off and on again, then retry.
service-bad-mode = The camera cannot be used in this mode. Please switch it to mass-storage mode.
service-driver-required = Raw USB access was denied and the WinUSB driver could not be assigned. Assigning a driver needs administrator rights.
service-warning = Warning: service mode grants raw access to the camera's memory, filesystem and calibration data. Write commands can permanently damage your camera.
service-shell-ready = Service shell ready. Type 'help' for commands, 'exit' to quit.
service-shell-done = Service shell session ended.
# Reported at startup when a previous run was cut short before it could
# release the driver it had assigned.
driver-swept = Removed { $count } driver assignment(s) left over from an earlier run.
# Reported at startup when that same interrupted run also left the camera in
# service mode, where it would refuse the next session.
driver-swept-camera = A camera left in service mode by an earlier run was returned to its normal mode.
driver-release-replug = Windows is waiting for the camera while the driver is released. Unplug the USB cable and plug it in again so the clean-up can finish.
# Results of the `driver` command.
driver-bound = { $id } is now using WinUSB (driver package { $package }).
driver-released = Released { $count } driver assignment(s); the camera is back on the drivers Windows ships.
driver-needs-elevation = Assigning a driver requires administrator rights. Please run this command from a console started as an administrator.
driver-leftover-needs-elevation = An earlier run left a driver assigned to the camera. While it is in place, the camera asks you to check the connected device and its mass storage does not work. Releasing it requires administrator rights; please confirm the Windows prompt.
# The driver assignment the service path makes on its own, so that no external
# tool is needed.
driver-binding = Assigning the WinUSB driver to the camera…
driver-bind-wait = Waiting for the camera to reconnect with the new driver…
driver-bind-timeout = The camera did not reappear after the driver was assigned. Please unplug it, reconnect it and try again.
driver-bind-failed = The WinUSB driver could not be assigned: { $detail }

# What the shell answers with. These are produced by the backend, so both the
# console and the window word them the same way.
service-usage = Usage: { $usage }
service-bad-hex = Not a hexadecimal value: { $value }
service-unknown-command = Unknown command: { $command }. Type 'help' for the list.
service-unterminated-quote = The line ends inside a quoted argument.
service-help = Command reference:
service-read-bytes = Read { $count } bytes.
service-pulled = Copied { $remote } from the camera to { $local }.
service-pushed = Copied { $local } to { $remote } on the camera.
service-removed = Deleted { $remote } from the camera.
service-dumped = Dumped { $size } bytes from { $offset } to { $local }.
service-backup-saved = Saved the backup image ({ $count } bytes) to { $local }.
service-backup-text-saved = Saved the backup image ({ $count } bytes) to { $bin } and the listing to { $txt }.
service-backup-status = Backup status, region: { $region }
service-backup-rewritten = Image ({ $count } bytes) written back unchanged and read again; not made permanent.
service-backup-attrs-restored = Attributes of { $count } entries set back to those in { $local }.
service-backup-written = Wrote backup entry { $id }.
service-backup-persisted = Persisted the backup of subsystem { $id } to flash.
service-terminal-on = The debug terminal is now on.
service-terminal-off = The debug terminal is now off.
btn-service-terminal-open = Open serial console
btn-service-terminal-close = Close serial console
service-terminal-heading = Serial console
service-terminal-hint = On Linux cameras the debug terminal is a root shell over USB. Open it, then type commands as you would in a terminal.
service-terminal-label = Console
service-terminal-interactive = Serial console open. Type commands; type 'exit' to return to the service shell.

# The service page as it looks without advanced mode: how to reach the shell
# from the console, and what has to be in place first.
service-console-heading = Running the service shell in the console
service-console-hint = Open a terminal in the folder holding pmca-console and run this command:
service-driver-heading = Before you start
service-driver-hint = Service mode needs raw USB access, which Windows does not grant through the mass-storage driver. The application assigns the WinUSB driver itself — twice, once while the camera is in mass-storage mode and once more after it reappears in service mode under a second product id — and releases both assignments when the session ends. Assigning a driver needs administrator rights, so start the console as an administrator; no other tool is required.
driver-note-linux = Nothing has to be installed here: libusb reaches the camera directly, without displacing a kernel driver. What it does need is permission to open the device, and a desktop that is not already holding it. Install a udev rule for Sony's vendor id 054c, and detach the usb-storage driver before using mass-storage mode. The script scripts/linux-prepare.sh in the source tree does both and stops the gvfs volume monitors as well.
driver-note-macos = Nothing has to be installed for raw USB access. Set the camera's USB connection to MTP. In MTP mode the program ends the macOS camera service ptpcamerad if it occupies the camera; the system restarts the service when needed. Tweaks, the service shell, the function analysis, firmware updates and the updater shell go through the mass-storage connection, which macOS gives only to a program with administrator rights: the window asks for your password once per run, the command line needs sudo. This platform is the least tested of the three, so expect to report a problem rather than to find it already described.
service-advanced-hint = The shell can also be shown here, under Settings, Advanced mode.
btn-copy-command = Copy command
msg-command-copied = Command copied to the clipboard.

# The service page with advanced mode on.
btn-service-open = Connect
btn-service-close = Disconnect
btn-service-run = Run
btn-service-browse = Insert file path…
shell-path-quote = This path cannot be inserted: it contains a double quote, the character the command line uses to mark where an argument begins and ends. Rename the file, or move it to a path without that character.
service-command-label = Command
service-not-connected = Not connected.

# GUI-only strings
install-apk-path = Path to a local .apk file (optional)
install-drop-hint = Drop an .apk file here

# Tweak descriptions
tweak-rec-limit = Record video without a time limit
tweak-rec-limit-4k = Record 4K video without a time limit
tweak-language = Show all menu languages
tweak-language-note = The camera shows only the languages its firmware knows, so fewer may appear than are enabled here. A DSC-RX1R, for example, shows 25 of the 35.
tweak-pal-ntsc = Enable PAL / NTSC selector & warning
tweak-pal-ntsc-mode = Video system (PAL / NTSC)
tweak-pal-ntsc-pal = PAL
tweak-pal-ntsc-ntsc = NTSC
tweak-pal-ntsc-note = After the video system is changed, the camera requires the memory card to be formatted. Back up everything on it first — photos, videos, LUTs. Simplest is to take the card out before switching.
tweaks-set-malformed = Expected --set ID=VALUE, got: { $arg }
tweaks-set-unknown-value = Unknown value "{ $value }". Expected one of: { $options }
tweaks-set-not-a-choice = { $id } is a switch, not a choice. Use --enable or --disable.
tweaks-needs-set = { $id } is a choice, not a switch. Use --set { $id }=VALUE with one of: { $options }
tweak-usb-installer = Enable USB app installer

# Tweak workflow
tweaks-warning = Warning: applying tweaks rewrites the camera's backup memory. An interrupted write can permanently damage your camera.
tweaks-info-toggle = What do tweaks do?
tweaks-info-what-heading = What they are
tweaks-info-what = Tweaks are settings your camera already has but does not offer in its menu. This program makes them available: video recording without a time limit (also in 4K), all menu languages contained in the firmware, the choice between PAL and NTSC, and the USB app installer. Plugins can add more. Which tweaks appear depends on the camera model.
tweaks-info-how-heading = How it works
tweaks-info-how = “Show available tweaks” briefly puts the camera into service mode and reads its backup memory. Nothing is changed by this. You then see which tweaks are on or off, and the camera's identity, which is only shown. “Apply” writes only the entries you changed. Afterwards the program reads the memory again and checks that the camera has taken everything over.
tweaks-info-undo-heading = The way back
tweaks-info-undo = Before every write, the program saves the previous state on this computer. Every tweak can be switched off again with the same switch. On cameras whose values are written one at a time, “Undo changes” is offered as well.
tweaks-info-needs-heading = What you need
tweaks-info-needs = The key material (see the Welcome page) and the camera's USB connection set to “Auto”, as described below. Windows and macOS ask for administrator rights so that the program can reach the camera in service mode. While writing, do not unplug the cable or remove the battery.
app-backup-target = Target folder
btn-choose-folder = Choose folder…
btn-default-folder = Use default folder
tweaks-reading = Reading camera settings…
tweaks-writing = Writing camera settings…
tweaks-done = Camera settings updated.
tweaks-write-failed = The camera did not store the new settings as they were sent. Part of them may have been written anyway: read the settings again and undo them if needed. A backup copy taken before the change is the safe way back.
tweaks-confirm-required = Refusing to write without confirmation. Re-run with --confirm once you have read the warning above.
tweaks-state-enabled = enabled
tweaks-state-disabled = disabled
tweaks-full-read-failed = The camera did not deliver its complete backup ({ $detail }). It often stops responding afterwards: switch it off, take the battery out for a few seconds, put it back and connect again.
tweaks-direct-mode = This camera's backup is larger than 1 MB, so its settings are read and written one at a time. The old values are saved before every change and can be written back with “Undo changes”.
tweaks-direct-forced = PMCA_TWEAKS_DIRECT forces the per-property path, so this camera's settings are read and written one at a time. The old values are saved before every change and can be written back with “Undo changes”.
tweaks-direct-refused = The camera refused the change ({ $detail }). Values already written were put back; nothing was changed. Some models, Japanese ones in particular, only allow this setting through the complete backup image.
tweaks-direct-revert-failed = The camera refused the change ({ $detail }), and not every value already written could be put back. The old values are saved: please try “Undo changes”.
tweaks-direct-restart = Camera settings updated. Switch the camera off and on again for the change to take effect.
tweaks-rollback-unsaved = The current values could not be saved for undoing, so nothing was changed.
tweaks-image-saved = The backup image from before the change was saved: { $path }
tweaks-whole-image-hint = To write the change through the complete backup image instead, run the same command again with --whole-image. The image from before the change is saved first.
tweaks-whole-image-confirm = Write the change through the complete backup image? The camera refused it as a single value. The whole image is written back with the change; the image from before the change is saved on this computer first.
btn-whole-image = Write complete backup
tweaks-undo-none = There are no saved values for this camera.
tweaks-undo-confirm = Write the saved values back to the camera? This undoes every change written one setting at a time since they were saved.
tweaks-undo-done = The saved values were written back. Switch the camera off and on again.
tweaks-undo-available = The values from before the first change are saved and can be written back.
btn-undo-tweaks = Undo changes
tweaks-identity-heading = Camera identity (read-only)
tweaks-caution-languages = Two languages — probably Chinese (Simplified) and Chinese (Traditional) — cannot be shown by this camera. Scrolling to them restarts the language menu, so they are left switched off as a precaution.
tweaks-caution-accept = Enable these two languages anyway (at your own risk)
tweaks-caution-confirm = These two languages can make the camera's language menu restart. Nothing is damaged and the change can be undone, but you may have to find the setting again from the start. Enable them anyway?

# Settings
settings-language = Language
settings-theme = Theme
settings-theme-system = System
settings-theme-light = Light
settings-theme-dark = Dark

# Advanced mode. A guard against a slip of the hand rather than against an
# attacker: everything it shows is reachable from the console anyway.
settings-advanced-heading = Advanced mode
settings-advanced-hint = Advanced mode shows the service shell and the updater shell on the pages of the same name. Service mode grants raw access to the camera's memory, its filesystem and its calibration data; a mistyped command can leave the camera unusable. Leave this off unless you know what you are about to write.
settings-advanced-toggle = Show the service and updater shells
settings-advanced-on = Advanced mode is on. The Service shell and Updater shell pages now offer the shells themselves.
settings-advanced-off = Advanced mode is off. The Service shell and Updater shell pages explain how to use the console instead.
settings-advanced-confirm = The service and updater shells can permanently damage your camera. Show both shells?
settings-update-heading = Update notification
settings-update-purpose = When this is on, the program asks GitHub at every start whether a newer version has been released and, if so, says so at the top of the window. GitHub learns your IP address in the process; nothing else is sent. Pre-releases are not announced.
settings-update-toggle = Tell me at start when a new version is available
settings-update-on = The update notification is on. From the next start, a new version is announced at the top of the window.
settings-update-off = The update notification is off. The program only looks for a new version when you click the version number below the navigation.
update-check-hint = Check now whether a new version is available
update-check-heading = Update check
update-checking = Looking for a new version…
update-none = You are using the newest version ({ $version }).
update-available = A new version is available
update-current = Installed version
update-latest = New version
btn-update-dismiss = Hide

# Editing a translation. Language files can be downloaded, changed in any text
# editor and uploaded again; an uploaded file is checked before it is used.
settings-catalog-heading = Edit translation
settings-catalog-hint = Download a language file, change it in any text editor and upload it again. Only the text to the right of the equals sign is translated, and entries you leave out keep their built-in wording. Picking a language here does not change the language of the application.
settings-catalog-locale = Language file
settings-catalog-builtin = The built-in translation is in use.
settings-catalog-custom = Your own translation is in use:
btn-catalog-export = Download…
btn-catalog-import = Upload…
btn-catalog-reset = Restore built-in
catalog-exported = Written to
catalog-imported = Translation applied.
catalog-reset-done = The built-in translation has been restored.
catalog-reset-none = There was no custom translation to restore.
catalog-reset-confirm = Discard your own translation for this language?
catalog-rejected = The file was not applied. Please correct these lines:
catalog-count-accepted = Entries applied
catalog-count-missing = Entries kept from the built-in translation
catalog-count-untranslated = Entries identical to the English original
catalog-line = Line
catalog-problem-syntax = not an entry of the form "id = text"
catalog-problem-continuation = continued on the following line, which is not supported
catalog-problem-duplicate = defined more than once
catalog-problem-unknown-key = not an entry this application uses
catalog-problem-empty = has no text
catalog-problem-placeholder = placeholder changed, expected:

# Provenance, acknowledgements and licence. Shown on the License page and by
# `pmca-console license`. The licence text itself is not translated: it is a
# legal document and is reproduced verbatim.
about-thanks-heading = Thanks
about-thanks-body-1 = This tool exists because of ma1co, who worked out how Sony's cameras talk over USB and released Sony-PMCA-RE in 2015, and because of everyone who contributed to it and to the OpenMemories projects in the years that followed.
about-thanks-body-2 = The camera detection, the external command protocol, the firmware updater procedure and service mode are all their work, and this port carries that work forward. Thank you. On top of that foundation, PMCA-RE RELOADED adds many new features contributed by masterflai. Chief among them is the automatic switching of the device driver on Windows, now built into the program: the camera no longer has to be assigned a different driver by hand with a separate tool, which makes the process more convenient, faster and less risky.
about-contributors-heading = Contributors
about-contributors-body = Besides ma1co, these people contributed to Sony-PMCA-RE, or published changes in their own copies of it that were carried over into this port:
about-purpose-heading = Why this port exists
about-purpose-body-1 = The original application is no longer being developed. It was written for Python 2, which reached its end of life in 2020, and parts of it rely on services Sony has since shut down, so it no longer runs without restrictions on current systems.
about-purpose-body-2 = The purpose of this port is preservation rather than replacement, and just as much trouble-free use today: to keep these cameras usable, and to work without difficulty on current computers as well as with newer camera models. Everything the original could do, this should still be able to do.
about-origin-heading = How it was made
about-origin-body-1 = The port from Python to Rust was carried out with the help of AI.
about-origin-body-2 = What that does not mean is worth stating just as plainly: the behaviour is covered by an automated test suite and by a hardware test plan. The Windows version in particular has been tested thoroughly, because the device drivers make it the most complex platform; macOS and Linux could not be tested as comprehensively. Please treat this as experimental software.
about-license-heading = License
about-license-body = This port is published under the MIT license, the same license as the original project. The original copyright notice is kept unchanged, as that license requires.
about-license-full = License text
about-third-party-heading = Third-party licences
about-third-party-body = This program is linked statically: the code of every library it uses sits inside the executable, and those libraries' licences require their copyright notices to travel with it. They are reproduced below in full, one entry per package.
about-third-party-toggle = Show all third-party notices
about-privacy-heading = Data
about-privacy-body-1 = This program runs on your own computer. It sends no usage data, there is no telemetry, and no account is required. Nothing you do here is reported anywhere.
about-privacy-body-2 = Two features do reach the network on their own, and the other end learns your IP address in the process: the GPS assist data is downloaded from Sony, and the sources you have added are read from GitHub, together with their stars and, when you ask for it, a search there; apps and plugins are downloaded from the addresses those sources name. Without those features the program stays entirely offline. If you switch on the update notification in the settings, the program also asks GitHub for the newest version at every start. Clicking the version number below the navigation asks the same question once.
about-privacy-body-3 = What is read out of the camera — its serial number and any stored Wi-Fi passphrases among it — does not leave your computer. If you write it to a file, mind who you pass that file on to.
about-privacy-body-4 = No service is operated behind this program and no data reaches its authors, so there is nothing here to ask for or to have erased. What the two services named above do with the addresses they see is governed by their own privacy statements, not by this one.
about-legal-heading = Legal notices
about-legal-trademark = This is an independent project. It is not made by, endorsed by, or connected to Sony in any way. "Sony", "PlayMemories" and the camera model names are trademarks of Sony Group Corporation, used here only to say which cameras this tool speaks to.
about-legal-own-device = This program is meant for use with cameras you own.
about-legal-warranty = What this tool does is outside what Sony intends for these cameras, and doing it may end your warranty. The software itself comes with no warranty of its own, and you use it at your own risk. Whether reverse engineering a device you own is permitted, and to what extent, depends on where you are.
about-links-heading = Projects
about-link-original = Sony-PMCA-RE — the original project
about-link-fwtool = fwtool.py — the firmware tools the update image generation is based on
about-link-tweak = OpenMemories: Tweak — the app recommended for cameras that support it
about-link-applist = OpenMemories: App list — further apps
about-link-platform = OpenMemories: Platform — the updater body bundled with this tool
about-link-devices = Supported camera models
about-link-nexhack = Nex-Hack — the community research this project builds on
about-link-zve10 = zv-e10-re — research on the ZV-E10's interfaces and settings memory
about-version = Version
btn-copy-url = Copy address
msg-url-copied = Address copied to the clipboard.

# Safety notice
notice-experimental = This is experimental software. Use at your own risk.

# Network settings (GPS assist, live-streaming access points and services)
gps-downloading = Downloading the GPS assist data…
gps-writing = Writing the GPS assist data to the camera…
gps-bad-size = The GPS assist data has an unexpected size and was not written.
wifi-read = Reading the live-streaming access points from the camera…
wifi-written = Live-streaming access points written.
stream-read = Reading the live-streaming settings from the camera…
stream-written = Live-streaming settings written.
net-secret-warning = Warning: this file contains Wi-Fi passphrases and access tokens in clear text.
wifi-secret-warning = The camera stores these passphrases unencrypted, and they are shown here exactly as stored. They stay on this computer; if you save them to a file, mind who you pass that file on to.
gps-heading = GPS assist data
gps-hint = Drag a file here, enter a path, or leave the field empty to get the current data from Sony.
btn-gps-devices = Compatible devices
btn-gps-devices-hide = Hide compatible devices
gps-devices-note = These models have a built-in GPS module. For many of them only the variant whose name ends in V carries it; variants without the V have no GPS.
gps-devices-compact = Compact cameras (DSC)
gps-devices-slt = SLT cameras (A-mount)
gps-devices-action = Action cams
gps-devices-camcorder = Camcorders (Handycam)
gps-col-variant = GPS variant
gps-col-arch = Architecture
gps-col-module = GPS module
gps-col-apps = App support
gps-yes = yes
gps-no = no
gps-unknown = unknown
gps-devices-untested = GPS present, support untested
gps-devices-untested-note = These models have a built-in GPS module but are not in the OpenMemories device list. Whether this application can talk to them has not been tested, so their architecture and app support are unknown.
wifi-heading = Access points for live streaming
wifi-hint = These are the access points the camera's live-streaming feature uses, not the camera's own Wi-Fi settings. A body without a streaming feature refuses this.
wifi-multi = Multi access-point list
wifi-none = No access points stored.
wifi-writing = Writing the live-streaming access points to the camera…
wifi-confirm = This replaces every access point stored on the camera. Continue?
wifi-empty-refused = Refusing to write an empty list, which would erase every stored access point.
stream-heading = Live streaming
stream-writing = Writing the live-streaming settings to the camera…
stream-confirm = This overwrites the camera's live-streaming settings. Continue?
stream-invalid-json = The settings are not valid JSON:

# Firmware update
firmware-warning = Warning: this overwrites the camera's firmware. Do not interrupt the update once it has started: do not unplug the USB cable and do not switch the camera off. Use only a firmware file intended for exactly this camera model, and make sure the battery is sufficiently charged. Otherwise the camera can be rendered permanently unusable.
firmware-untested = Untested: firmware flashing has been ported but never tried on a real camera. A failed or interrupted write cannot be undone, so treat everything on this page as unproven.
firmware-confirm-required = Refusing to flash without --confirm.
firmware-truncated = The firmware file is truncated: the FDAT payload extends past the end of the file.
firmware-updating = Updating firmware:
firmware-invalid = The file is not a valid Sony firmware image
firmware-dat-path = Firmware file (.dat)
firmware-drop-hint = Drop a .dat firmware file here
firmware-file-required = Choose a firmware file first.
firmware-check-ok = The camera accepts this image. Nothing has been written.
firmware-cancelled = Flashing cancelled. The camera was not modified.
firmware-current-version = Installed version
firmware-new-version = Version in this image
firmware-models-hint = A camera is compatible if the updater shell can reach it.
btn-firmware-check = Check image
btn-firmware-flash = Flash firmware
firmware-own-device = Confirm that you are aware of these risks and are updating only your own device.
btn-firmware-accept = Risks understood – flash

# File tools: the operations that need no camera at all.
tools-hint = These operations work on files and never touch the camera, so nothing here can damage it. They are the same operations pmca-console offers as apk2spk, spk2apk, print-backup, print-usagelog, guess-firmware and xpd-checksum.
tools-operation = Operation
tools-apk2spk = Wrap an .apk in an .spk container
tools-spk2apk = Unwrap an .spk back into an .apk
tools-print-backup = Show the contents of a Backup.bin
tools-print-usagelog = Show the contents of a Usagelog.bin
tools-guess-firmware = Find the payload inside a firmware .dat
tools-xpd-checksum = Compute the CIC checksum of a file
tools-input = Input file
tools-file-required = Choose the files this operation needs first.
tools-apk-invalid = The file could not be packed as an .spk
tools-spk-invalid = The file is not a valid .spk package
tools-backup-invalid = The file is not a valid Backup.bin image
tools-usagelog-invalid = The file is not a valid usage log
tools-reset-value = reset value:
tools-fdat-found = Payload found at offset { $offset }, size { $size }.
tools-checksum = Checksum
tools-written = Written to { $path }

# Pointers to the project documentation
docs-compatibility = Camera compatibility
docs-compatibility-hint = The original project provided an overview of tested cameras. It covers mainly older camera models and can be found at this address:
devices-note = Taken from the original project's overview. The number beside app support is the version of the camera's app platform.
devices-more-note = The following cameras have not been explicitly tested with this program. A tick in the last column means the community has reported using them. Where the service shell is also marked ✖, that use failed.
btn-more-models = More devices
btn-more-models-hide = Hide more devices
check-updater-yes = Your device { $model } is compatible: according to the device list, the updater shell can reach it.
check-updater-no = Your device { $model } is not compatible: according to the device list, the updater shell cannot reach it.
check-updater-unknown = The compatibility of your device { $model } is unknown: it is not in any of this program's device lists, so whether the updater shell can reach it is not known.
check-apps-no = Your device { $model } is not compatible: according to the device list, it does not support apps.
check-apps-untested = The compatibility of your device { $model } is unknown: it is in the device list, but whether it supports apps has not been confirmed.
check-apps-unknown = The compatibility of your device { $model } is unknown: it is not in any of this program's device lists, so whether it supports apps is not known.
check-gps-no = Your device { $model } is not compatible: it is not in the list of devices with a GPS receiver.
check-gps-untested = The compatibility of your device { $model } is unknown: according to the device list it has a GPS receiver, but the transfer to this model is untested.
check-gps-unknown = The compatibility of your device { $model } is unknown: it is not in any of this program's device lists, so whether it has a GPS receiver is not known.
check-unidentified = The connected device could not be identified, so its compatibility is unknown.
check-proceed = Do you want to continue anyway?
check-cancelled = Cancelled. The device was not modified.
devices-col-confirmed = Confirmed
devices-group-ilc = Interchangeable-lens cameras (E- and A-mount)
devices-group-remote = Live-View remotes
btn-copy-link = Copy link
shutter-heading = Shutter count
shutter-hint = The camera counts every shutter release and writes the current figure into every photo it saves (ARW, JPEG or HEIF). Select the most recently taken photo to read the current count. The file is only read, never modified.
shutter-source = Where the count is stored in the file differs between models. The positions come from publicly documented research (ExifTool) and are not confirmed for every model.
shutter-drop-hint = Drop a photo (ARW, JPEG, HEIF) here
btn-shutter-read = Analyze photo
shutter-file-required = Please select a photo first.
shutter-file = File
shutter-taken = Date taken
shutter-count = Shutter releases
shutter-type = Shutter
shutter-type-mechanical = Mechanical
shutter-type-electronic = Electronic
shutter-electronic-hint = This photo was taken with the electronic shutter. On many models such shots are not counted; the value then shows the count at the last mechanical release.
shutter-not-sony = The photo was not taken with a Sony camera. No shutter count can be read from it.
shutter-unknown-model = It is not known where this camera model stores the shutter count in the file, so no value is shown.
shutter-not-recorded = The file contains no readable shutter count. This happens with photos saved or exported by image-editing software, and on some models with shots taken with the electronic shutter. Select the original file from the memory card.
shutter-model-mismatch = The photo was taken with a different camera model than the connected camera.
shutter-invalid = The file is not a readable photo
file-filter-photo = Photos (ARW, JPEG, HEIF)
usage-log-heading = Usage log
usage-log-hint = The camera keeps an internal log in an undocumented format. It is saved verbatim; the camera is only read from.
usage-log-written = Written to
btn-usage-log-info = About the usage log
btn-usage-log-info-hide = Hide the explanation
usage-log-info-what-heading = What this log is
usage-log-info-what = Sony calls the log "KikiLog" internally. The camera writes it itself; this program only reads it. No public description of the format is known.
usage-log-info-read = Reading changes nothing: several downloads in a row were byte-identical.
usage-log-info-flush-heading = Why recent entries are missing
usage-log-info-flush = New entries often do not appear straight away: the camera may only save them when it is switched off, sometimes spread over several on/off cycles. Switch the camera off and on, download again, and repeat until no new entries appear.
usage-log-info-content-heading = What it tells you
usage-log-info-shutter = Shutter releases: some events each stand for one exposure. Counting them does not give the shutter count, though: the log has a fixed size and need not cover the camera's whole service life. The camera keeps the total elsewhere and writes it into the photos it saves; it can be read in the "Shutter count" section.
usage-log-info-clock = Timestamps are only as good as the camera clock. After a battery change the clock can fall back and produce entries carrying the camera's initial date.
usage-log-info-unknown = No public description of the event numbers is known. The program shows the numbers without interpreting them.
usage-log-info-privacy-heading = Privacy
usage-log-info-privacy = The file contains the serial numbers of memory cards that were inserted. Do not share or post it carelessly.
btn-usage-log = Save log
usagelog-size = Log area
usagelog-cursor = Write position
usagelog-spare = Second area
usagelog-count = Entries
usagelog-events = Events by number
usagelog-entries = Entries in order
msg-link-copied = Link copied to the clipboard.

# Backend failure messages, keyed by the stable code the backend reports.
# The variable part of a message (a path, a status byte, a device error) is
# appended by the front-end, so these read as complete sentences without it.
# There is deliberately no `err-operation`: that code marks a message the
# backend has already translated, which must be shown verbatim.
err-usb-no-device = No camera found. Connect a camera and switch it on. If the camera's display keeps saying that it is connecting, the computer is not receiving it: use a data cable rather than a charge-only cable, connect the camera directly without a hub or adapter, and switch it off and on again; if that does not help, briefly remove the battery. Over a data cable even a switched-off camera is detected and reported as not ready; if this message also appears with the camera switched off, the cable probably carries no data.
err-usb-camera-not-ready = The camera is connected but not ready: it is switched off, or it is waiting on its display for the USB mode to be chosen. Switch it on, or choose “Mass Storage (MSC)” on the camera.
camera-choose-usb-mode = The camera is waiting for the USB mode to be chosen. Please choose “Mass Storage (MSC)” on the camera.
err-usb-too-many-devices = More than one camera is connected. Disconnect all but the one you want to work with.
err-usb-transfer = The USB transfer failed. Try a different cable or port, and avoid USB hubs.
err-usb-permission = The program is not allowed to open the camera over USB. On Linux this permission is granted once by a udev rule for Sony's vendor id; the script scripts/linux-prepare.sh in the source tree installs it. Then unplug the camera and plug it back in.
err-usb-busy = Another program is using the camera, so it could not be opened. Close other programs that might access it; on Linux the desktop's media monitor can hold it, which scripts/linux-prepare.sh stops. On macOS, set the camera's USB connection to MTP: in mass-storage mode the system's own disk driver holds it.
err-usb-needs-root = macOS gives the camera's mass-storage connection to a program only when it runs with administrator rights. Start the command-line program with sudo, for example: sudo pmca-console tweaks. Reading the camera information also works without it, with the camera's USB connection set to MTP.
macos-admin-prompt = PMCA-RE RELOADED needs administrator rights to take the camera's mass-storage connection from macOS. The camera's card disappears from Finder while it works; eject it there first if files on it are open.
macos-admin-declined = Administrator rights were not granted, so the camera was not touched. On macOS this function needs them.
macos-admin-failed = The program could not start its helper with administrator rights.
err-usb-stall = The camera aborted the USB transfer.
err-usb-protocol = The camera sent an unexpected response.
err-usb-msc-status = The camera rejected a storage command.
cli-error-prefix = error
cli-hint-mass-storage = Switch to mass-storage mode: pmca-console mass-storage
err-usb-driver = Windows has assigned this camera to a driver that does not permit direct access. This mode needs the WinUSB driver, which the application can assign on its own — but only when it is running with administrator rights.
err-needs-elevation = Assigning or releasing a driver requires administrator rights, and the Windows prompt was declined. Try again with “Start driver helper”; nothing on the camera has to be changed.
err-usb-no-mass-storage-volume = The camera is connected in mass-storage mode but is not offering a drive. Wake it up, confirm the USB mode on its screen if it asks, and check that a memory card is inserted. Do not change the driver: this mode needs the standard Windows one.
err-usb-msc-sense = The camera reported a storage error.
err-usb-ptp-response = The camera rejected the command.
err-usb-unsupported = This camera model does not support that function.
err-usb-updater-status = The firmware updater on the camera rejected the operation.
err-usb-senser = A service-mode command failed.
err-market-core = A camera data file could not be read.
err-market-json = The data received was malformed.
err-market-missing-field = An expected field was missing from the response.
err-market-no-apk = No app has been selected for installation.
err-market-http = The network request failed.
err-market-io = The local install server failed.
err-market-tls = The encrypted connection to the camera could not be established.
err-apk = The app file could not be read.
err-io = A file could not be read or written.
err-json = The camera sent malformed data.
err-unknown-locale = This language is not available.
err-catalog-encoding = The translation file is not saved as UTF-8 text. Save it again using UTF-8 encoding.
err-internal = An internal error occurred. This is a bug in the application.

## Function analysis
nav-analysis = Analysis
analysis-title = Function analysis
analysis-experimental = Experimental. This feature is new and has been tried on only a few cameras.
analysis-own-hardware = Use it on your own camera only. Service mode itself is an intervention Sony does not intend; the responsibility is yours.
analysis-explanation-toggle = Explanation: purpose, procedure, risks
analysis-purpose-heading = What is it for?
analysis-purpose = The function analysis documents in which value of its settings memory (backup) the camera stores one of its own menu settings. The result describes this camera's settings memory; it can be compared with other cameras but does not change what a camera offers.
analysis-how-heading = How does it work?
analysis-how = For every menu value you enter, the program reads the complete settings memory once, each time in a short service-mode session of its own. In between you unplug the cable and change the setting in the camera menu; the camera stays switched on. Finally the first value is read once more. Values that follow the menu and are back at their starting value at the end are candidates. Values that change on their own (counters, clock) are set aside.
analysis-requirements-heading = Requirements
analysis-requirements = The camera is connected by USB, its USB mode is set to “Auto”, and after each session you choose “Mass Storage (MSC)” on the camera. On Windows the program needs administrator rights, because it briefly assigns a driver for service mode. Charge the battery; each session takes about a minute.
analysis-read-heading = Reading steps
analysis-read = The snapshots only read. Nothing in the camera's settings changes. The risk is low but not zero: switching into service mode is already an intervention Sony does not intend.
analysis-write-heading = Writing step (write test)
analysis-write = Only if you switch it on explicitly: the program writes the value it found into the camera, you check in the menu after a restart whether it takes effect, and then the original value is written back. Only a value the camera stored itself is written, and only when exactly one matching candidate was found. The old value is saved beforehand. Even so, this step changes the camera; a fault can leave it in a state only a service centre can repair.
analysis-legal-heading = Legal
analysis-legal = The software is provided without any warranty (MIT license). Interventions in the camera may affect its warranty. Use the function only on cameras you own. The analysis report contains no serial number and nothing from the settings memory except the candidates found; it is never sent anywhere automatically.
analysis-start-heading = New analysis
analysis-setting-label = Menu setting, as the camera names it
analysis-values-label = Menu values, separated by commas
analysis-values-hint = The first value is the one set now. Two to eight values; each costs one service-mode session.
analysis-write-option = Then run the write test (changes the camera)
analysis-write-warning = The write test changes the camera: it writes a value into its settings memory and afterwards writes the original value back. Only on your own camera, at your own risk.
analysis-write-consent = The camera is mine and I bear the risk.
btn-analysis-write-enable = Enable write test
btn-analysis-start = Start analysis
btn-analysis-snapshot = Read backup
btn-analysis-new = New analysis
analysis-discard-confirm = Discard the analysis under way? The report so far stays saved.
analysis-access-read = READING
analysis-access-write = WRITING
analysis-access-user = MANUAL
analysis-access-local = PC
analysis-step-snapshot = Read the backup, the setting is at “{ $value }”
analysis-step-change = Change to “{ $value }” in the camera menu
analysis-step-evaluate = Compare the snapshots
analysis-step-write = Write the value for “{ $value }” into the camera
analysis-step-check = Switch the camera off and on, read the menu (expected: “{ $value }”)
analysis-step-restore = Write back the original value for “{ $value }”
analysis-step-report = Save the analysis report
analysis-do-prepare = PREPARATION: 1. In the camera menu, set “{ $setting }” to “{ $value }” and leave the menu with the MENU button so the camera saves the setting. 2. Leave the camera switched on. 3. Plug in the cable and choose “Mass Storage (MSC)” on the camera.
analysis-do-change = YOUR TURN: 1. Unplug the cable. 2. The camera stays switched on. 3. In the camera menu, set “{ $setting }” to “{ $value }” and leave the menu with the MENU button so the camera saves the setting. 4. Plug the cable back in and choose “Mass Storage (MSC)”.
analysis-do-check = YOUR TURN: 1. Unplug the cable. 2. Switch the camera off and on again. 3. Look up what the menu shows for “{ $setting }”.
analysis-do-reconnect = Plug the cable back in and choose “Mass Storage (MSC)” on the camera.
analysis-reading = Snapshot { $index } of { $total }: do NOT unplug the cable while it is being read.
analysis-cli-continue = Press Enter to continue, q to cancel:
analysis-confirm-word = yes
analysis-cli-confirm = Type “{ $word }” to confirm:
analysis-write-declined = Not confirmed. Nothing was written.
analysis-aborted = Cancelled. Nothing was written.
analysis-snapshot-done = Snapshot read. The camera has left service mode.
analysis-result = { $compared } values compared: { $count } candidates, { $running } values changed on their own.
analysis-candidate-consistent = matches the menu unambiguously
analysis-candidate-inconsistent = does not match unambiguously
analysis-write-preview-heading = This will be written
analysis-write-preview = Value { $id } ({ $size } bytes): from { $old } (“{ $old-label }”) to { $new } (“{ $new-label }”). The camera stored the new value itself while “{ $new-label }” was set.
btn-analysis-write = Write
btn-analysis-skip-write = Finish without write test
analysis-written = Value written, read back and saved permanently.
analysis-menu-question = What does the menu show now?
analysis-menu-other = something else
btn-analysis-menu-confirm = Confirm
btn-analysis-restore = Write back the original value
analysis-restored = Original value written back and saved permanently.
analysis-survived = The written value was still stored after the restart.
analysis-not-survived = The written value was no longer stored after the restart.
analysis-report-saved = Analysis report saved: { $path }
analysis-report-hint = The report contains no serial number and no backup. You can pass it on so the finding can be checked and documented.
btn-analysis-export = Export report…
file-filter-analysis = Analysis report
analysis-pending = A write test was not completed. The original value is saved and can be written back.
analysis-pending-none = No unfinished write test is stored for this camera.
analysis-invalid-setting = Please enter the menu setting as the camera names it.
analysis-invalid-values = Please enter two to { $max } different menu values. The first is the one set now.
analysis-already-complete = All snapshots have already been read.
analysis-other-camera = This is not the same camera as in the first snapshot. The analysis was stopped.
analysis-write-not-allowed = The write test is not possible in this state.
analysis-camera-changed = The camera now holds a different value than in the first snapshot. Nothing was written.
analysis-write-refused = The camera refused the value; everything is as before. ({ $detail })
analysis-refuse-none = Write test not possible: no value followed the menu.
analysis-refuse-ambiguous = Write test not possible: { $count } values followed the menu, none can be assigned with certainty.
analysis-refuse-inconsistent = Write test not possible: the value does not match the menu values unambiguously.
analysis-refuse-size = Write test not possible: the value is { $size } bytes; only small values are written.
analysis-refuse-read-only = Write test not possible: the camera marks the value as read-only.
analysis-refuse-protected = Write test not possible: the camera marks the value as protected.
settings-analysis-heading = Function analysis
settings-analysis-purpose = The function analysis documents in which value of its settings memory the camera stores one of its own menu settings. It is meant for people who want to understand the settings memory of their model and share the results.
settings-analysis-why-off = It is switched off because normal use does not need it, because it switches the camera into service mode several times, and because on request it also writes to the camera. It is experimental and has been tried on only a few models.
settings-analysis-toggle = Switch on the function analysis (“Analysis” entry in the menu)
settings-analysis-confirm = Switch on the function analysis? It switches the camera into service mode several times and can write to the camera on request. Use it on your own camera only.
btn-analysis-enable = Switch on
settings-analysis-on = The function analysis is switched on.
settings-analysis-off = The function analysis is switched off.
analysis-needs-admin = Each snapshot needs administrator rights for service mode. Windows asks for them once per program session; you can start the driver helper now or wait until the first snapshot asks for it.
analysis-changed-hint = Please check: no candidate was found. The following values changed but did not return to their starting value. If one of them changes only once, between two values, the camera probably was not at the first menu value for the first snapshot. Then repeat the analysis with the starting value set correctly.
analysis-changed-info = Info: values that changed on their own during the analysis (e.g. counters, clock) are ignored. Nothing to do.
elevation-admin = Administrator
elevation-standard = Standard rights
elevation-admin-hint = The program runs with administrator rights and can assign the driver service mode needs.
elevation-standard-hint = The program runs with standard rights. For service mode, tweaks and the function analysis it starts a driver helper with administrator rights when needed; Windows asks once per session. Click to start it.
driver-helper-declined = The prompt for administrator rights was declined. Without them the driver for service mode cannot be assigned; nothing was changed.
btn-remove-foreign-drivers = Remove old drivers
btn-not-now = Not now
btn-ok = OK
foreign-drivers-found = Windows still holds drivers that Zadig or a similar tool installed for Sony devices: { $packages }. Windows gives the camera to them before this program can assign its own driver, so service mode fails and the camera may not appear as a drive. Remove them now? The camera then uses the drivers Windows ships. Removing them needs administrator rights once.
foreign-drivers-removed = The old drivers have been removed. Unplug the camera and plug it in again.
foreign-drivers-remaining = These drivers could not be removed: { $packages }. Restart Windows and try again, or uninstall the device together with its driver in Device Manager.
foreign-drivers-hint = Drivers that Zadig or a similar tool installed for Sony devices are still present ({ $packages }). Windows gives the camera to them before this program can assign its own driver, which can make service mode fail. Remove them with: pmca-console driver zadig --remove
driver-zadig-none = No drivers from Zadig or similar tools are installed for Sony devices.
driver-zadig-entry = { $package } ({ $provider }): { $devices }
driver-zadig-remove-hint = Remove them with: pmca-console driver zadig --remove
driver-zadig-removed = Removed { $count } old driver package(s). Unplug the camera and plug it in again.
elevation-helper = Driver helper active
elevation-helper-hint = A driver helper with administrator rights assigns the driver for service mode. It has no window and ends with the program; there will be no further prompt.
elevation-helper-confirm = Start the driver helper now? Windows asks for administrator rights once, and not again until the program ends.
analysis-cross-heading = Cross-check on another camera
analysis-cross-intro = Optional: checks whether another camera has the value found as well, and what is set there. It only reads. The same number does not necessarily mean the same function; only a look at the other camera's menu confirms that. The result goes into the same analysis report.
analysis-cross-do = 1. Unplug the analysed camera. 2. Switch on the other camera, set its USB mode to “Auto”, plug in the cable and choose “Mass Storage (MSC)”.
btn-analysis-cross = Check other camera
analysis-cross-present = present, value { $value }
analysis-cross-match = equals “{ $label }”
analysis-cross-nomatch = equals none of the measured menu values
analysis-cross-absent = not present
analysis-cross-same-camera = This is the analysed camera itself. Please connect the other camera.
analysis-cross-not-now = The cross-check is only possible once every snapshot is read and no written value is waiting to be restored.
analysis-cross-none = There is no candidate to cross-check.
analysis-wrong-camera = The connected camera is not the analysed one. Please connect the { $model }; nothing was written.
keys-fetching = Downloading the source…
keys-scanning = Searching for key material…
keys-complete = All key material is present.
keys-incomplete = Some key material is still missing.
keys-present = present
keys-missing = missing
keys-class-senser = Service-mode keys (service shell, tweaks, analysis)
keys-class-cic = XPD checksum key (app installation)
keys-class-spk = SPK key material (app installation, apk2spk)
keys-class-fdat = FDAT key material (updater-shell firmware)
plugin-heading = Plugins
plugin-intro = Plugins extend this program with languages, switches for camera settings, compatibility notes, tools and diagnostics. They come from files you choose yourself or from the sources you have added; this program does not check them. Their code runs in a sandbox without access to your files, the network or the key material. Install only plugins whose author you trust; changes take effect at the next start.
plugin-none = No plugins installed.
plugin-inactive = (active after a restart)
plugin-author = Author
plugin-restart = Restart the program for the change to take effect.
btn-plugin-install = Install plugin…
plugin-install-confirm = This plugin may:
btn-confirm-plugin-install = Install plugin
plugin-installed = The plugin has been installed.
plugin-remove-confirm = Remove the plugin “{ $name }”?
plugin-removed = The plugin { $id } has been removed.
plugin-perm-languages = add languages to the program
plugin-perm-camera-settings = change camera settings (only between the two values it names, and only after you confirm)
plugin-perm-compatibility = add entries to the compatibility overview
plugin-perm-code = run its own code in the sandbox (tools or diagnostics)
plugin-perm-host-os = ask which operating system this is
plugin-perm-host-cameras = see which Sony cameras are connected and in which mode
plugin-invalid = This is not a usable plugin: { $reason }
plugin-unknown-field = This plugin uses a feature this version of the program does not know ({ $field }). It was made for a newer or a different edition of the program.
plugin-no-folder = The user data folder could not be determined, so plugins cannot be installed.
plugin-not-installed = No plugin with the id { $id } is installed.
plugin-failed = The plugin failed: { $reason }
plugin-unknown-entry = No loaded plugin provides { $id }.
plugin-run-done = The plugin has finished.
plugin-run-warning = The plugin has finished with remarks.
plugin-run-error = The plugin reported an error.
plugin-tools-heading = Plugin tools
plugin-tools-hint = These come from installed plugins and run in a sandbox. A plugin receives only what you hand it and what it declared, and cannot change the camera.
plugin-diagnostics-heading = Plugin diagnostics
plugin-compat-heading = From a plugin
plugin-col-note = Note
file-filter-plugin = Plugin file
keys-not-set-up = The key material this operation needs is not set up.
err-missing-keys = The key material this operation needs is not set up.
keys-heading = Key material needed
keys-intro = The service-mode keys, the XPD checksum key and the SPK key material are not part of this program. Enter the address of a source you have — a repository, an archive or a file — and the program finds and imports the parts it recognises. Nothing is run, and the material is kept only on this computer.
keys-needs-toggle = Which features need which key material?
keys-needs-info = Camera info (MTP, mass storage, updater mode)
keys-needs-info-keys = None.
keys-needs-install = App installation and “Check camera”
keys-needs-install-keys = XPD checksum key; installing an app also needs the SPK key material.
keys-needs-service = Service mode: service shell, tweaks, analysis
keys-needs-service-keys = Service-mode keys.
keys-needs-updater = Updater mode: updater shell and app backup
keys-needs-updater-keys = FDAT key material.
keys-law-notice = Observe the laws that apply where you are, and work only with cameras and data that are your own.
keys-url-label = Source address (one per line)
keys-url-required = Please enter at least one source address.
btn-keys-import = Import
btn-import-keys = Set up key material
