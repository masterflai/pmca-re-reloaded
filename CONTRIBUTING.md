# Contributing

Thank you for helping. There are three ways to contribute, and the first two
need no programming at all.

## Report how a camera behaves

Used the program with a camera model? Fill in the
[camera report](https://github.com/masterflai/pmca-re-reloaded/issues/new?template=camera_report.yml).
Reports — including "this did not work" — decide what the device list in the
app says about a model. For questions and experiences that are not a report,
use [Discussions](https://github.com/masterflai/pmca-re-reloaded/discussions).

**Never post** serial numbers, key material, backup images (`Backup.bin`) or
firmware files. The usage log and photos can contain the serial number as
well; remove it before attaching anything.

## Improve a translation

Every text of the program lives in one file per language:
[`locales/<language>/pmca.ftl`](locales), for example `locales/de-DE/pmca.ftl`.
The files use [Fluent](https://projectfluent.org/fluent/guide/); in practice
each line is `key = text`, and only the text right of the equals sign is
translated. `{ $name }` placeholders and the key stay exactly as they are.

### Try it in the app first

You do not need to build anything to see your translation:

1. In the app, open *Settings → Edit translation* and choose *Download…* for
   your language.
2. Edit the file in any text editor and save it as UTF-8.
3. Choose *Upload…*. The app uses your file at once; entries you left out keep
   their built-in text. *Restore built-in* undoes everything.

### Send it back

When you are happy with it, open a pull request that changes
`locales/<language>/pmca.ftl`, or attach the file to an issue if you prefer not
to use Git. Please keep:

* the order of the keys and the comments as in `locales/en-US/pmca.ftl`, which
  is the reference;
* UTF-8 without byte-order mark and Windows line endings (CRLF);
* the product name *PMCA-RE RELOADED*, Sony's trademarks and model names
  untranslated.

`tools/check-locales.ps1` compares every language with en-US (keys, encoding,
line endings), and `tools/check-untranslated.ps1` lists entries that are still
identical to English. A missing or extra key also makes the CI fail.

### A new language

Copy `locales/en-US` to a folder named after the language tag (for example
`it-IT`) and translate the file. The program finds the new folder by itself and
shows the language under its own name; no code change is needed. A language
can also be shipped as a plugin without changing this repository — see
[README → Plugins](README.md#plugins).

## Change the code

The project is a Rust workspace; [ARCHITECTURE.md](ARCHITECTURE.md) explains
how the crates fit together and [README → Building from source](README.md#building-from-source)
lists what you need. Before opening a pull request:

```bash
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cd crates/pmca-gui && npm ci && npm run typecheck
```

Some guidelines:

* **One concern per pull request**, with a short description of what changes
  for the user.
* **New texts go into every language.** Add the key to all
  `locales/*/pmca.ftl` files; English text in a non-English file is fine as a
  start and is listed by `check-untranslated.ps1`.
* **No key material, no Sony firmware, no camera dumps** in the repository —
  not even in tests. Test data uses invented values.
* **Writes to a camera need a safety net**: a saved copy before, a read-back
  after, and a clear confirmation in the interface.
* Record user-visible changes under *Unreleased* in [CHANGELOG.md](CHANGELOG.md).

By contributing you agree that your contribution is published under the
project's [MIT license](LICENSE.txt).
