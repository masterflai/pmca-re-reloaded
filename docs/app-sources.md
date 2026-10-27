# Publishing apps and plugins

PMCA-RE RELOADED installs camera apps and plugins from **sources**: public
GitHub repositories in which developers publish their own work. The program
ships no source, hosts nothing and checks nothing. Users add the sources they
choose, and the author of a source alone is responsible for what it offers.
Anyone may publish a source; nobody has to approve it.

This page describes what a source has to look like (format version 1).

## Quick start

1. Build your app (a v1-signed `.apk`) or plugin. For a plugin, create a key
   once with `pmca-console plugins keygen my-plugins.key` and pack with
   `pmca-console plugins pack <folder> my-plugin.pmcaplugin --key my-plugins.key`.
2. Publish a GitHub release of your project with the `.apk` or `.pmcaplugin`
   attached.
3. In a public repository — the project itself or one listing several of your
   projects — add `pmca-source.yaml` at the root of the default branch:

   ```yaml
   ---
   package: com.example.timelapse
   name: Timelapse
   author: Jane Doe
   desc: Interval shooting with exposure ramping
   release:
     type: github
     user: janedoe
     repo: timelapse
   ```

4. Give that repository the topic `pmca-source` and describe your work in its
   README.
5. Check it yourself: `pmca-console sources add janedoe/your-repository`,
   then `pmca-console apps` or `pmca-console plugins available`.

Later releases reach your users on their own; the source file only changes
when you add or remove an entry.

## The repository

* A source is a **public GitHub repository**. GitHub shows how many users
  have starred it, and the program shows that number next to the source, in
  the search and in the app list. The stars say nothing about quality or
  safety; they only help a user decide whether to look closer.
* At the root of the default branch lies **`pmca-source.yaml`**.
* Give the repository the **topic `pmca-source`**. The program's search, which
  runs only when a user asks for it, finds repositories by that topic.
* Describe in the repository's README what your apps and plugins do, which
  cameras you have tried them on, and how to reach you. Users are told to look
  at the repository before they add it.

Users add a source as `owner/repository` or with its address, in the app under
*Settings → App and plugin sources* or with
`pmca-console sources add owner/repository`.

## `pmca-source.yaml`

The file uses the format of the
[OpenMemories app list](https://github.com/ma1co/OpenMemories-AppList), so an
existing entry can be copied unchanged. It is a series of documents separated
by `---`, each one describing one app or plugin. Only plain `key: value` pairs
and one nested `release:` block are read; anything else is ignored.

```yaml
---
package: com.example.timelapse
name: Timelapse
author: Jane Doe
desc: Interval shooting with exposure ramping
homepage: https://github.com/janedoe/timelapse
release:
  type: github
  user: janedoe
  repo: timelapse
---
kind: plugin
package: lens-notes
name: Lens notes
author: Jane Doe
desc: Compatibility notes for adapted lenses
release:
  type: github
  user: janedoe
  repo: lens-notes-plugin
```

| Key | Required | Meaning |
|---|---|---|
| `kind` | no | `app` (default) or `plugin`. Entries of another kind are skipped. |
| `package` | yes | For an app, the Android package name from its manifest. For a plugin, its id from `plugin.json`; the plugin is refused if the two differ. |
| `name` | yes | Name shown in the program. |
| `author` | no | Shown next to the name. |
| `desc` | no | One-line description. |
| `homepage` | no | Project page. |
| `release` | no | Where the newest build is. Without it the entry is listed but cannot be installed. |
| `cert_sha256` | no | Apps only: the SHA-256 of the certificate you sign with, as `keytool -printcert -jarfile app.apk` prints it (colons and case do not matter). Every download is checked against it, the first one included. |

### `release`

Three forms are understood. Every address must start with `https://`.

* **GitHub releases** — `type: github`, `user`, `repo`. The program takes the
  newest release that has an asset of the right kind: a file ending in `.apk`
  for an app, `.pmcaplugin` for a plugin. The release title, or its tag, is the
  version shown.
* **A feed of your own** — `type: yaml`, `url`. The feed is a series of
  `---`-separated documents with `version`, `url` and optionally `date` and
  `desc`; the first one is the newest.
* **A JSON document of your own** — `type: json`, `url`, `download`. The
  document at `url` is an object with `version` and optionally `build`, each a
  string or a number, for example `{"schema":1,"version":"2.05","build":58}`.
  `download` is the address of the APK with the placeholders `{version}` and,
  if you need it, `{build}`:

  ```yaml
  release:
    type: json
    url: https://example.com/app-release.json
    download: https://dl.example.com/MyApp-v{version}.apk?build={build}
  ```

  Version and build may contain only letters, digits, `.`, `-` and `_`. Write
  the version as a string: as a number, `2.50` arrives as `2.5`.
* **A fixed build** — `version` and `url` directly in the `release:` block.

## Signing

Anyone may publish a source, and the program cannot know who you are. What a
signature gives your users is continuity: once they have installed something
from you, nobody else can slip them an "update".

**Apps.** Sign every APK with the same key (v1 / JAR signature, as the cameras
require). When a user first downloads one of your apps from your source, the
program remembers the certificate it was signed with. A later version signed
with a different certificate, or not signed at all, is refused.

**Plugins.** Create a key once and sign every plugin file with it:

```
pmca-console plugins keygen my-plugins.key
pmca-console plugins pack <folder> my-plugin.pmcaplugin --key my-plugins.key
```

The key file is your secret: keep it out of the repository and back it up.
`pack` adds `signature.json` to the archive — an Ed25519 signature over the
SHA-256 of every other file, together with your public key. The program checks
it on every install and shows the key. As with apps, the first download from
your source remembers the key, and a later version signed with another key, or
not signed, is refused; in addition, an installed signed plugin is only ever
replaced by a file signed with the same key, wherever that file comes from.

**Naming your certificate.** With `cert_sha256` in your entry, the program
does not have to trust the first download: every APK of that entry has to be
signed with exactly that certificate, or it is refused. Add it if you publish
the fingerprint anyway, for example on your website.

If you really have to change a key, tell your users: they have to remove your
source and add it again, and remove an installed plugin before installing the
newly signed one.

## Limits

* `pmca-source.yaml` and release feeds: 4 MB.
* An APK: 64 MB. A plugin: 32 MB, with the further limits of the plugin format.
* GitHub allows 60 unauthenticated API requests per hour and address. The
  program asks for the stars of a source and for the newest release of an entry
  once per run, and only when they are shown or installed.

## Writing plugins

See [`crates/pmca-plugin-sdk`](../crates/pmca-plugin-sdk/src/lib.rs) and its
examples. `pmca-console plugins pack <folder> <file>` builds a plugin file and
checks it exactly as installing does.
