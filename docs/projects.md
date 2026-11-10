# Listing independent projects

The *Projects* page of PMCA-RE RELOADED (and `pmca-console projects`) helps
users follow independent projects around Sony cameras: tools, scripts, apps,
plugins, guides. A project is listed straight from its own GitHub repository.
Nothing of it is downloaded or installed: the program shows what GitHub and
the project's own file say, its newest release included, and the addresses to
open in a browser.

The projects are independent. This program does not select, check, moderate
or curate them, listing one is no recommendation, and their authors alone are
responsible for their content, function and safety. Anyone may list a project;
nobody has to approve it.

This page describes what a project repository has to look like (format
version 1).

## Quick start

1. In the public repository of your project, add `pmca-project.yaml` at the
   root of the default branch:

   ```yaml
   format: 1
   name: Lens Profiles
   summary: Lens corrections for adapted manual lenses
   category: tool
   cameras: ILCE-6000, ILCE-7M3
   platforms: windows, linux
   author: Jane Doe
   homepage: https://janedoe.example/lens-profiles
   ```

2. Give the repository the topic `pmca-project`.
3. Publish releases as usual. The newest stable release is shown with its
   version and date; drafts and pre-releases are not.
4. Check it yourself: `pmca-console projects pin janedoe/lens-profiles`, then
   `pmca-console projects refresh`.

## `pmca-project.yaml`

Plain `key: value` lines; only the first `---`-separated document is read and
anything else is ignored. Every value is shown as plain text.

| Key | Required | Meaning |
|---|---|---|
| `format` | no | `1`. Reserved for later versions of this format. |
| `name` | yes | Name shown in the program, at most 100 characters. |
| `summary` | no | One line, at most 200 characters. Without it, the repository's GitHub description is shown. |
| `category` | no | `tool`, `app`, `plugin`, `script`, `guide` or `other` (the default, also for any other value). |
| `cameras` | no | Camera models you have tried it with, comma-separated, at most 20. |
| `platforms` | no | Comma-separated, out of `windows`, `macos`, `linux`, `camera`, `android`, `web`. |
| `author` | no | Shown in the details. |
| `homepage` | no | Project website; must start with `https://`. |

Control characters are removed and longer values are cut. License, stars,
forks, programming language and the date of the last change come from GitHub,
not from the file.

## What users see

* **Search** (`pmca-console projects search`) lists the repositories with the
  topic `pmca-project`, most-starred first, as GitHub ranks them. It runs only
  when the user asks.
* **Pinned projects** are kept on the user's computer only. *Refresh*
  (`pmca-console projects refresh`) asks GitHub about each of them and marks a
  release as *new* when it differs from the one shown at the last refresh.
* Addresses (repository, release page, website) are shown as text with a copy
  button.

GitHub allows 60 unauthenticated API requests per hour and address. A search
costs one; refreshing costs two per pinned project. When the allowance is used
up, the program says so instead of failing silently.

## Reporting a project

Projects are not reviewed. If a listed project obviously infringes the law —
for example, it distributes someone else's copyrighted work or malware —
please report it to GitHub first
([Reporting abuse or spam](https://docs.github.com/en/communities/maintaining-your-safety-on-github/reporting-abuse-or-spam)),
which hosts it. To have this program stop showing it as well, send the
repository name and the reason through a
[private report](https://github.com/masterflai/pmca-re-reloaded/security/advisories/new).
A project found to be obviously unlawful is left out of the search and can no
longer be pinned from the next release on. That list is the answer to such
notices; it is not a selection, and nothing else is filtered.
