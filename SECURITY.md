# Security policy

## Supported versions

Security fixes go into the latest release. Pre-releases are for testing and
are replaced by the next release. Please check that a problem still occurs with
the latest release before reporting it.

## Reporting a vulnerability

Please do not open a public issue. Report it privately through GitHub instead:
[report a vulnerability](https://github.com/masterflai/pmca-re-reloaded/security/advisories/new).

Helpful details:

- the version and the operating system,
- the steps or the file that trigger the problem,
- what an attacker gains by it.

The reply comes in the private advisory. Once a fix is released, the advisory is
published and the changelog lists the fix under *Security*. Say whether you would
like to be credited.

## Scope

In scope is everything in this repository: the desktop app, the command line,
the plugin host, the installers and the release workflow. Examples are code run
through a crafted APK, plugin or source file, a way past the signature or
certificate checks of apps and plugins, files written outside the chosen folder,
or rights gained through the helpers that install the Windows driver or ask for
the macOS password.

Out of scope are the cameras and Sony's firmware, the apps that sources list
(please report those to their authors; this project hosts none), and flaws in
dependencies that do not affect this program. For those, the dependency's own
project is the right place.
