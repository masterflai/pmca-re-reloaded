//! Writing a camera's whole settings store out as JSON, and comparing several.
//!
//! Two operations, both of which work on files alone once the image has been
//! saved. The dump turns a `Backup.bin` into a readable, diffable JSON
//! document; the comparison lays two or more of those side by side and sorts
//! every property into the classes of [`pmca_platform::compare`]. Neither
//! touches a camera, so both can be used on images that were saved months ago
//! or that somebody else shared.
//!
//! # Privacy
//!
//! A complete dump contains everything the camera stores, and that includes
//! its serial number. These files exist to be exchanged — that is the whole
//! point of comparing two bodies — so by default the serial number property is
//! written out as zeros of the right length and marked `redacted`. The length
//! is kept so the structural picture stays honest; only the value goes.
//!
//! The camera is still identified well enough to tell two dumps of the *same*
//! body apart from two different ones: [`DumpCamera::camera_id`] is a short
//! digest of model and serial, which matches itself and reveals nothing.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use pmca_core::backup::{BackupFile, attr};
use pmca_i18n::Localizer;
use pmca_platform::compare::{self as cmp, Class, Dump, DumpEntry, Shape};
use pmca_platform::{BackupPatch, props};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::analysis::report::{hex, timestamp};
use crate::dto::{ProgressEvent, ProgressKind};
use crate::{Error, ProgressSink, Result};

/// Re-exported so a front-end that does not depend on the platform crate — the
/// console does not — can still name the classes and the default rule.
pub use pmca_platform::compare::{Class as RowClass, DEFAULT_SIMILAR_DISTANCE};

/// Identifies the dump format; bumped when a field changes meaning.
pub const DUMP_SCHEMA: &str = "pmca-backup-dump/1";

/// Identifies the comparison format.
pub const COMPARE_SCHEMA: &str = "pmca-backup-compare/1";

/// The most images one comparison accepts.
///
/// A wider table stops being readable, and every source is held in memory in
/// full while the comparison runs.
pub const MAX_SOURCES: usize = 8;

/// The most gaps a dump lists individually. The total is always reported; the
/// list is a sample of the largest, because a fragmented image has thousands
/// and no reader wants them all.
const MAX_GAPS: usize = 32;

/// Which identifying values a dump carries.
///
/// Every field means *include*, and every one is off by default. These files
/// exist to be exchanged, so what identifies a person or a body has to be
/// asked for rather than merely not prevented. The photographer and copyright
/// strings are the camera's own owner fields: a dump of a camera that has
/// been filled in carries somebody's name unless this says otherwise.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Privacy {
    /// Include the camera's serial number.
    pub serial: bool,
    /// Include the photographer fields.
    pub photographer: bool,
    /// Include the copyright fields.
    pub copyright: bool,
}

impl Privacy {
    /// Nothing identifying; what a dump carries unless asked otherwise.
    pub const MASKED: Self = Self { serial: false, photographer: false, copyright: false };
    /// Everything the camera holds.
    pub const FULL: Self = Self { serial: true, photographer: true, copyright: true };

    /// A sentence for the file saying what it does and does not carry.
    fn describe(self) -> String {
        let withheld: Vec<&str> = [
            (!self.serial).then_some("serial number"),
            (!self.photographer).then_some("photographer"),
            (!self.copyright).then_some("copyright holder"),
        ]
        .into_iter()
        .flatten()
        .collect();
        if withheld.is_empty() {
            "Complete, identifying fields included. Treat this file as personal data.".to_string()
        } else {
            format!("Withheld: {}. Every other value as the camera holds it.", withheld.join(", "))
        }
    }
}

/// What identifies the camera a dump came from.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct DumpCamera {
    /// Model name, where the image carries one.
    pub model: Option<String>,
    /// Region string from the image header.
    pub region: String,
    /// Firmware version, where a camera was there to ask. Model, region and
    /// firmware together identify what a dump describes well enough to line
    /// two of them up later.
    pub firmware: Option<String>,
    /// Android platform version, where the image carries one.
    pub android_platform_version: Option<String>,
    /// Short digest of model and serial number. Two dumps of the same body
    /// share it; the serial cannot be read back out of it.
    pub camera_id: String,
}

/// One run of unclaimed bytes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct GapView {
    /// First unclaimed byte, as hex.
    pub offset: String,
    /// How many bytes the run holds.
    pub size: usize,
}

/// How the bytes of an image divide up.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct FreeSpaceView {
    /// Bytes of header.
    pub header: usize,
    /// Bytes of subsystem table.
    pub subsystem_table: usize,
    /// Bytes of property table.
    pub property_table: usize,
    /// Bytes set aside for table entries that do not exist.
    pub table_reserve: usize,
    /// Bytes in the data region.
    pub data_size: usize,
    /// Data-region bytes a property claims.
    pub data_used: usize,
    /// Data-region bytes no property claims.
    pub data_free: usize,
    /// Bytes reserved by variable-size properties beyond what they hold.
    pub data_slack: usize,
    /// Size of the longest unclaimed run.
    pub largest_gap: usize,
    /// How many unclaimed runs there are in total.
    pub gap_count: usize,
    /// The largest of them, at most `MAX_GAPS`.
    pub gaps: Vec<GapView>,
    /// Bytes past the end the header declares; outside the checksum.
    pub trailing: usize,
}

/// The shape of the image a dump came from.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct DumpLayout {
    /// Backup format revision (2 or 4).
    pub revision: u8,
    /// Meaningful bytes in the image.
    pub image_size: usize,
    /// First byte of the data region, as hex.
    pub data_offset: String,
    /// First byte after the property table, as hex.
    pub table_end: String,
    /// Subsystems the header declares.
    pub num_subsystems: u32,
    /// Property table entries the header declares.
    pub num_properties: u32,
    /// Properties that actually have an entry.
    pub present: usize,
    /// How many properties each subsystem holds, by subsystem.
    pub subsystems: BTreeMap<String, usize>,
    /// What is claimed and what is not.
    pub free: FreeSpaceView,
}

/// One property, as a dump records it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct PropertyView {
    /// Composite id, as hex.
    pub id: String,
    /// Subsystem half of the id, as hex.
    pub subsystem: String,
    /// The toolkit's name for the setting, where it knows one.
    pub name: Option<String>,
    /// First byte of the value in the image, as hex.
    pub offset: String,
    /// Length of the value.
    pub size: usize,
    /// Bytes reserved for the value.
    pub max_size: usize,
    /// Attribute bitfield, as hex.
    pub attr: String,
    /// The names of the bits that are set.
    pub attr_names: Vec<String>,
    /// The value as spaced lowercase hex.
    pub hex: String,
    /// The factory value, where the property has one.
    pub reset: Option<String>,
    /// Whether the value differs from the factory value.
    pub touched: bool,
    /// Whether a value could be written here at all.
    pub writable: bool,
    /// Whether the value looks like it carries no information.
    pub empty: bool,
    /// Whether the value was withheld; see the module documentation.
    pub redacted: bool,
}

/// A camera's whole settings store.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct DumpFile {
    /// Format identifier, [`DUMP_SCHEMA`].
    pub schema: String,
    /// Which version of the toolkit wrote it.
    pub tool: String,
    /// When, as `YYYY-MM-DDTHH:MM:SSZ`.
    pub taken: String,
    /// Which camera it came from.
    pub camera: DumpCamera,
    /// The shape of the image.
    pub layout: DumpLayout,
    /// Every property that has a table entry.
    pub properties: Vec<PropertyView>,
    /// What was left out, in plain words.
    pub privacy: String,
}

/// One property as a comparison sees it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct CompareRow {
    /// Composite id, as hex.
    pub id: String,
    /// Subsystem half of the id, as hex.
    pub subsystem: String,
    /// The toolkit's name for the setting, where it knows one.
    pub name: Option<String>,
    /// The class, as named by [`Class::as_str`].
    pub class: String,
    /// The common length, where every camera agrees on one.
    pub size: Option<usize>,
    /// Attribute bitfield of the first camera that has the property, as hex.
    pub attr: Option<String>,
    /// The names of the bits that are set.
    pub attr_names: Vec<String>,
    /// Whether the cameras disagree about the attribute bits.
    pub attr_differs: bool,
    /// Most differing byte positions between any value and the first.
    pub distance: usize,
    /// Whether any camera holds something other than its factory value.
    pub touched: bool,
    /// Whether a value could be written here at all.
    pub writable: bool,
    /// What the form of the value suggests: `switch`, `choice` or `plain`.
    pub shape: String,
    /// The value on each camera, `null` where it has no such property.
    pub values: Vec<Option<String>>,
}

/// Where one side of a comparison came from.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct CompareSource {
    /// The file that was read.
    pub file: String,
    /// The camera it describes.
    pub camera: DumpCamera,
    /// How many properties it holds.
    pub properties: usize,
}

/// The result of comparing two or more dumps.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct CompareReport {
    /// Format identifier, [`COMPARE_SCHEMA`].
    pub schema: String,
    /// Which version of the toolkit wrote it.
    pub tool: String,
    /// When, as `YYYY-MM-DDTHH:MM:SSZ`.
    pub created: String,
    /// The rule that decided what counts as near.
    pub similar_distance: usize,
    /// Properties that were excluded as self-changing, as hex ids.
    pub noise_ids: Vec<String>,
    /// Whether that list was derived from two readings of the same camera.
    pub noise_measured: bool,
    /// The dumps that were compared, in column order.
    pub sources: Vec<CompareSource>,
    /// How many properties fell into each class, before any filtering.
    pub counts: BTreeMap<String, usize>,
    /// How many properties were compared in total.
    pub total: usize,
    /// Which classes `rows` holds.
    pub included: Vec<String>,
    /// One row per property, by ascending id.
    pub rows: Vec<CompareRow>,
    /// What was left out, in plain words.
    pub privacy: String,
}

/// Which rows an export keeps.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Filter {
    /// The classes to keep, by their [`Class::as_str`] names.
    pub classes: BTreeSet<String>,
    /// Keep only properties that could be written.
    pub only_writable: bool,
    /// Keep only properties the toolkit has a name for.
    pub only_named: bool,
    /// Keep only properties whose form suggests a setting (`switch` or
    /// `choice`).
    pub only_switches: bool,
}

impl Default for Filter {
    fn default() -> Self {
        Self {
            classes: Class::all().iter().map(|c| c.as_str().to_string()).collect(),
            only_writable: false,
            only_named: false,
            only_switches: false,
        }
    }
}

/// What the comparison treats as near, and what it ignores.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompareOptions {
    /// Largest number of differing bytes still called similar.
    pub similar_distance: usize,
    /// Properties to exclude as self-changing.
    pub noise: BTreeSet<u32>,
    /// Also exclude whatever differs between two dumps of the same camera.
    pub auto_noise: bool,
}

impl Default for CompareOptions {
    fn default() -> Self {
        Self {
            similar_distance: cmp::DEFAULT_SIMILAR_DISTANCE,
            noise: BTreeSet::new(),
            auto_noise: true,
        }
    }
}

/// What a front-end shows after a dump was written.
///
/// Everything but the values: a window needs the shape of the image and the
/// tally, and shipping several megabytes of hex through the IPC channel to
/// display a dozen numbers would be waste.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct DumpSummary {
    /// Where the dump was written.
    pub path: String,
    /// Which camera it came from.
    pub camera: DumpCamera,
    /// The shape of the image.
    pub layout: DumpLayout,
    /// How many properties were written.
    pub properties: usize,
    /// How many of them had their value withheld.
    pub redacted: usize,
    /// Where the backup image was kept, when the user asked to keep it.
    pub image_path: Option<String>,
    /// What was left out, in plain words.
    pub privacy: String,
}

impl DumpFile {
    /// The parts of this dump a front-end displays.
    #[must_use]
    pub fn summary(&self, path: &Path, image_path: Option<&Path>) -> DumpSummary {
        DumpSummary {
            path: path.display().to_string(),
            camera: self.camera.clone(),
            layout: self.layout.clone(),
            properties: self.properties.len(),
            redacted: self.properties.iter().filter(|p| p.redacted).count(),
            image_path: image_path.map(|p| p.display().to_string()),
            privacy: self.privacy.clone(),
        }
    }

    /// A file name that says which camera and which day, without an extension.
    ///
    /// Model and region are what distinguishes the cameras a comparison is
    /// about, so a list of these files reads correctly without anybody having
    /// named them.
    #[must_use]
    pub fn stem(&self) -> String {
        let keep = |s: &str| -> String {
            s.chars().map(|c| if c.is_ascii_alphanumeric() || c == '-' { c } else { '_' }).collect()
        };
        let model = self.camera.model.as_deref().unwrap_or("camera");
        let day = self.taken.split('T').next().unwrap_or(&self.taken);
        format!("{}-{}-{}", keep(model), keep(&self.camera.region), keep(day))
    }
}

/// Turn a backup image on disk into a dump in `folder`.
///
/// Named the way a camera read names its own, so a folder of dumps reads the
/// same however each of them was made — and so one cannot quietly overwrite
/// another that happens to share a file name.
pub fn dump_image_file(
    l: &Localizer,
    input: &Path,
    folder: &Path,
    privacy: Privacy,
) -> Result<DumpSummary> {
    let image = std::fs::read(input)?;
    // No camera to ask: a file does not carry the firmware version.
    let dump = dump_image(l, &image, privacy, None)?;
    std::fs::create_dir_all(folder)?;
    let path = folder.join(format!("{}.json", unique_stem(folder, &dump.stem())));
    save(&dump, &path)?;
    Ok(dump.summary(&path, None))
}

/// Read the connected camera and write a dump of it into `folder`.
///
/// The image is written out before it is parsed, because reading it costs a
/// service-mode session of a minute or two and a parse failure should not
/// throw that away. When `keep_image` is false the file is removed once the
/// dump exists — but not if the dump could not be made, since then it is the
/// only thing left of the read.
pub fn dump_camera(
    l: &Localizer,
    sink: &mut dyn ProgressSink,
    folder: &Path,
    privacy: Privacy,
    keep_image: bool,
) -> Result<DumpSummary> {
    // Asked before service mode starts, as the reference check does: the
    // version is reported by the ordinary connection, not by the backup.
    let firmware = crate::camera_info().ok().and_then(|c| c.firmware);
    let image = crate::backup::read_whole_image(l, sink)?;

    let scratch = if keep_image { folder.to_path_buf() } else { scratch_dir(folder) };
    std::fs::create_dir_all(&scratch)?;
    let provisional = scratch.join(format!("camera-backup-{}.bin", stamp()));
    std::fs::write(&provisional, &image)?;

    let dump = dump_image(l, &image, privacy, firmware).inspect_err(|_| {
        sink.emit(ProgressEvent::step(
            ProgressKind::Info,
            l.tr_arg("compare-image-kept", "path", &provisional.display().to_string()),
        ));
    })?;

    std::fs::create_dir_all(folder)?;
    let stem = unique_stem(folder, &dump.stem());
    let json_path = folder.join(format!("{stem}.json"));
    save(&dump, &json_path)?;

    let image_path = if keep_image {
        let kept = folder.join(format!("{stem}.bin"));
        move_file(&provisional, &kept)?;
        Some(kept)
    } else {
        let _ = std::fs::remove_file(&provisional);
        None
    };

    sink.emit(ProgressEvent::step(ProgressKind::Done, l.tr("backup-saved")));
    Ok(dump.summary(&json_path, image_path.as_deref()))
}

/// Where dumps go unless the user names somewhere else. `None` when this
/// machine has no per-user data folder, which leaves the choice to them.
pub fn default_folder() -> Option<PathBuf> {
    pmca_i18n::catalog::user_data_dir().map(|dir| dir.join("dumps"))
}

/// Where a backup image waits while it is turned into a dump, when the user
/// does not want to keep it. Beside the other per-user records, not in the
/// folder they chose: a file they did not ask for should not appear there.
fn scratch_dir(fallback: &Path) -> PathBuf {
    default_folder().unwrap_or_else(|| fallback.to_path_buf())
}

fn stamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or_default()
}

/// `stem`, or `stem-2`, `stem-3`… if a dump of that name is already there.
fn unique_stem(folder: &Path, stem: &str) -> String {
    if !folder.join(format!("{stem}.json")).exists() {
        return stem.to_string();
    }
    (2..1000)
        .map(|n| format!("{stem}-{n}"))
        .find(|candidate| !folder.join(format!("{candidate}.json")).exists())
        .unwrap_or_else(|| format!("{stem}-{}", stamp()))
}

/// Rename, falling back to copy-and-delete when the two sit on different
/// volumes, which `rename` refuses.
fn move_file(from: &Path, to: &Path) -> Result<()> {
    if std::fs::rename(from, to).is_ok() {
        return Ok(());
    }
    std::fs::copy(from, to)?;
    let _ = std::fs::remove_file(from);
    Ok(())
}

/// Turn a `Backup.bin` image into a dump.
///
/// `firmware` comes from the connected camera when there is one; a file alone
/// does not carry the version.
pub fn dump_image(
    l: &Localizer,
    image: &[u8],
    privacy: Privacy,
    firmware: Option<String>,
) -> Result<DumpFile> {
    let file = BackupFile::new(image).map_err(|e| invalid(l, &e))?;
    let dump = Dump::from_image(&file).map_err(|e| Error::Operation(e.to_string()))?;
    let patch = BackupPatch::new(image.to_vec()).map_err(|e| invalid(l, &e))?;
    let identity = pmca_platform::identity::list(&patch);
    let value = |id: &str| identity.iter().find(|v| v.id == id).map(|v| v.value.clone());

    let properties = dump.entries.iter().map(|e| property_view(e, privacy)).collect();
    Ok(DumpFile {
        schema: DUMP_SCHEMA.to_string(),
        tool: format!("PMCA-RE RELOADED {}", env!("CARGO_PKG_VERSION")),
        taken: timestamp(),
        camera: DumpCamera {
            model: value("modelName"),
            region: dump.region.clone(),
            firmware,
            android_platform_version: value("androidPlatformVersion"),
            camera_id: camera_id(&patch),
        },
        layout: layout_view(&dump),
        properties,
        privacy: privacy.describe(),
    })
}

/// Read a dump from a file, which may be either a saved JSON dump or a raw
/// `Backup.bin` image.
///
/// Accepting both means the comparison can be run straight away on images that
/// were saved before this feature existed.
pub fn read_dump(l: &Localizer, path: &Path, privacy: Privacy) -> Result<DumpFile> {
    let data = std::fs::read(path)?;
    if looks_like_json(&data) {
        let file: DumpFile = serde_json::from_slice(&data).map_err(|e| {
            Error::Operation(l.tr_arg("compare-invalid-dump", "detail", &e.to_string()))
        })?;
        if file.schema != DUMP_SCHEMA {
            return Err(Error::Operation(l.tr_arg("compare-wrong-schema", "schema", &file.schema)));
        }
        Ok(file)
    } else {
        // A raw image on its own: no camera was asked, so no firmware version.
        dump_image(l, &data, privacy, None)
    }
}

/// Write a dump or a report out as pretty-printed JSON.
pub fn save<T: Serialize>(value: &T, path: &Path) -> Result<()> {
    std::fs::write(path, serde_json::to_string_pretty(value)?)?;
    Ok(())
}

/// Compare the dumps in `paths`.
pub fn compare_paths(
    l: &Localizer,
    paths: &[PathBuf],
    options: &CompareOptions,
) -> Result<CompareReport> {
    if paths.len() < 2 {
        return Err(Error::Operation(l.tr("compare-needs-two")));
    }
    if paths.len() > MAX_SOURCES {
        return Err(Error::Operation(l.tr_arg(
            "compare-too-many",
            "max",
            &MAX_SOURCES.to_string(),
        )));
    }
    let files = paths
        .iter()
        .map(|p| read_dump(l, p, Privacy::MASKED).map(|d| (p.clone(), d)))
        .collect::<Result<Vec<_>>>()?;
    Ok(compare_dumps(&files, options))
}

/// Compare dumps that have already been read.
#[must_use]
pub fn compare_dumps(files: &[(PathBuf, DumpFile)], options: &CompareOptions) -> CompareReport {
    let dumps: Vec<Dump> = files.iter().map(|(_, f)| f.to_dump()).collect();

    let mut noise = options.noise.clone();
    let measured = options.auto_noise && add_measured_noise(files, &dumps, &mut noise);

    let refs: Vec<&Dump> = dumps.iter().collect();
    let comparison = cmp::compare(
        &refs,
        &cmp::Options { similar_distance: options.similar_distance, noise: noise.clone() },
    );

    let counts =
        comparison.counts().into_iter().map(|(class, n)| (class.as_str().to_string(), n)).collect();
    CompareReport {
        schema: COMPARE_SCHEMA.to_string(),
        tool: format!("PMCA-RE RELOADED {}", env!("CARGO_PKG_VERSION")),
        created: timestamp(),
        similar_distance: options.similar_distance,
        noise_ids: noise.iter().map(|id| format!("{id:#010x}")).collect(),
        noise_measured: measured,
        sources: files
            .iter()
            .map(|(path, file)| CompareSource {
                file: path
                    .file_name()
                    .map_or_else(|| path.display().to_string(), |n| n.to_string_lossy().into()),
                camera: file.camera.clone(),
                properties: file.properties.len(),
            })
            .collect(),
        total: comparison.rows.len(),
        included: Class::all().iter().map(|c| c.as_str().to_string()).collect(),
        rows: comparison.rows.iter().map(compare_row).collect(),
        counts,
        privacy: "Serial numbers are withheld unless a source dump was made with them.".to_string(),
    }
}

impl CompareReport {
    /// The same report with only the rows a filter keeps.
    ///
    /// `counts` is left as it was on purpose: a reader has to be able to see
    /// what the file does *not* contain, or an export of the differences alone
    /// reads as though nothing else was ever compared.
    #[must_use]
    pub fn filtered(&self, filter: &Filter) -> Self {
        let mut out = self.clone();
        out.included = Class::all()
            .iter()
            .map(|c| c.as_str().to_string())
            .filter(|c| filter.classes.contains(c))
            .collect();
        out.rows.retain(|row| {
            filter.classes.contains(&row.class)
                && (!filter.only_writable || row.writable)
                && (!filter.only_named || row.name.is_some())
                && (!filter.only_switches || row.shape != Shape::Plain.as_str())
        });
        out
    }
}

/// How many bytes of a value the text report shows before cutting it short.
const TEXT_VALUE_BYTES: usize = 8;

/// Render a comparison as fixed-width text.
///
/// Shared by the console and the window's report pane, so a comparison reads
/// the same wherever it is shown.
#[must_use]
pub fn report_text(l: &Localizer, report: &CompareReport) -> String {
    use std::fmt::Write as _;
    let mut out = String::new();

    let _ = writeln!(out, "{}:", l.tr("compare-heading-cameras"));
    for (i, source) in report.sources.iter().enumerate() {
        let _ = writeln!(
            out,
            "  {}  {:<16} {:<12} {}",
            i + 1,
            source.camera.model.as_deref().unwrap_or("?"),
            source.camera.region,
            source.file
        );
    }

    let _ = writeln!(out, "\n{}: {}", l.tr("compare-compared"), report.total);
    for class in Class::all() {
        let count = report.counts.get(class.as_str()).copied().unwrap_or(0);
        let _ = writeln!(out, "  {:<28} {count:>6}", class_label(l, class.as_str()));
    }
    if !report.noise_ids.is_empty() {
        let key =
            if report.noise_measured { "compare-noise-measured" } else { "compare-noise-supplied" };
        let _ = writeln!(out, "\n{}", l.tr_arg(key, "count", &report.noise_ids.len().to_string()));
    }

    if report.rows.is_empty() {
        let _ = writeln!(out, "\n{}", l.tr("compare-no-rows"));
        return out;
    }

    let _ = write!(out, "\n{:<12} {:>5}  {:<12} {:<22}", "id", "size", "class", "name");
    for i in 1..=report.sources.len() {
        let _ = write!(out, " {:<24}", format!("#{i}"));
    }
    out.push('\n');
    for row in &report.rows {
        let _ = write!(
            out,
            "{:<12} {:>5}  {:<12} {:<22}",
            row.id,
            row.size.map_or_else(|| "-".to_string(), |s| s.to_string()),
            row.class,
            row.name.as_deref().unwrap_or("")
        );
        for value in &row.values {
            let text = value.as_deref().map_or_else(|| "-".to_string(), shorten);
            let _ = write!(out, " {text:<24}");
        }
        out.push('\n');
    }
    out
}

/// A translated name for a class, falling back to its stable name.
fn class_label(l: &Localizer, class: &str) -> String {
    let key = format!("compare-class-{class}");
    let text = l.tr(&key);
    if text == key { class.to_string() } else { text }
}

/// Spaced hex cut to `TEXT_VALUE_BYTES`, so one long array cannot push every
/// other column off the line.
fn shorten(hex: &str) -> String {
    let bytes: Vec<&str> = hex.split_whitespace().collect();
    if bytes.len() <= TEXT_VALUE_BYTES {
        return bytes.join(" ");
    }
    format!("{} …", bytes[..TEXT_VALUE_BYTES].join(" "))
}

impl DumpFile {
    /// The properties of this dump, as the comparison layer wants them.
    #[must_use]
    pub fn to_dump(&self) -> Dump {
        let mut entries: Vec<DumpEntry> = self
            .properties
            .iter()
            .filter_map(|p| {
                let id = parse_hex_u32(&p.id)?;
                Some(DumpEntry {
                    id,
                    attr: parse_hex_u32(&p.attr)? as u16,
                    offset: parse_hex_u32(&p.offset)? as usize,
                    max_size: p.max_size,
                    slot_size: p.max_size,
                    data: parse_hex_bytes(&p.hex),
                    reset: p.reset.as_deref().map(parse_hex_bytes),
                })
            })
            .collect();
        entries.sort_unstable_by_key(|e| e.id);
        Dump {
            region: self.camera.region.clone(),
            revision: self.layout.revision,
            image_size: self.layout.image_size,
            buffer_size: self.layout.image_size,
            data_offset: parse_hex_u32(&self.layout.data_offset).unwrap_or_default() as usize,
            table_end: parse_hex_u32(&self.layout.table_end).unwrap_or_default() as usize,
            num_subsystems: self.layout.num_subsystems,
            num_properties: self.layout.num_properties,
            entries,
        }
    }
}

/// Add whatever differs between two dumps of the same camera to `noise`.
///
/// Nothing changed between those two readings, so anything that moved did so
/// by itself. Returns whether such a pair was found: a report has to say
/// whether its noise list was measured or merely supplied.
fn add_measured_noise(
    files: &[(PathBuf, DumpFile)],
    dumps: &[Dump],
    noise: &mut BTreeSet<u32>,
) -> bool {
    let mut seen: BTreeMap<&str, usize> = BTreeMap::new();
    let mut measured = false;
    for (i, (_, file)) in files.iter().enumerate() {
        if file.camera.camera_id.is_empty() {
            continue;
        }
        match seen.get(file.camera.camera_id.as_str()) {
            Some(&first) => {
                noise.extend(cmp::drift(&dumps[first], &dumps[i]));
                measured = true;
            }
            None => {
                seen.insert(file.camera.camera_id.as_str(), i);
            }
        }
    }
    measured
}

fn property_view(entry: &DumpEntry, privacy: Privacy) -> PropertyView {
    let redacted = is_identifying(entry.id, entry.data.len(), privacy);
    let masked = vec![0u8; entry.data.len()];
    let data = if redacted { &masked } else { &entry.data };
    PropertyView {
        id: format!("{:#010x}", entry.id),
        subsystem: format!("{:#06x}", entry.id >> 16),
        name: prop_name(entry.id, entry.data.len()),
        offset: format!("{:#010x}", entry.offset),
        size: entry.data.len(),
        max_size: entry.max_size,
        attr: format!("{:#04x}", entry.attr),
        attr_names: attr::names(entry.attr).iter().map(|s| (*s).to_string()).collect(),
        hex: hex(data),
        reset: if redacted { None } else { entry.reset.as_deref().map(hex) },
        touched: !redacted && entry.touched(),
        writable: entry.writable(),
        empty: entry.looks_empty(),
        redacted,
    }
}

fn layout_view(dump: &Dump) -> DumpLayout {
    let free = dump.free_space();
    let mut gaps = free.gaps.clone();
    gaps.sort_unstable_by(|a, b| b.size.cmp(&a.size).then(a.offset.cmp(&b.offset)));
    DumpLayout {
        revision: dump.revision,
        image_size: dump.image_size,
        data_offset: format!("{:#010x}", dump.data_offset),
        table_end: format!("{:#010x}", dump.table_end),
        num_subsystems: dump.num_subsystems,
        num_properties: dump.num_properties,
        present: dump.entries.len(),
        subsystems: dump
            .subsystems()
            .into_iter()
            .map(|(sub, n)| (format!("{sub:#06x}"), n))
            .collect(),
        free: FreeSpaceView {
            header: free.header,
            subsystem_table: free.subsystem_table,
            property_table: free.property_table,
            table_reserve: free.table_reserve,
            data_size: free.data_size,
            data_used: free.data_used,
            data_free: free.data_free(),
            data_slack: free.data_slack,
            largest_gap: free.largest_gap,
            gap_count: free.gaps.len(),
            gaps: gaps
                .iter()
                .take(MAX_GAPS)
                .map(|g| GapView { offset: format!("{:#010x}", g.offset), size: g.size })
                .collect(),
            trailing: free.trailing,
        },
    }
}

fn compare_row(row: &cmp::Row) -> CompareRow {
    CompareRow {
        id: format!("{:#010x}", row.id),
        subsystem: format!("{:#06x}", row.id >> 16),
        name: row.size.and_then(|size| prop_name(row.id, size)),
        class: row.class.as_str().to_string(),
        size: row.size,
        attr: row.attr.map(|a| format!("{a:#04x}")),
        attr_names: row
            .attr
            .map(|a| attr::names(a).iter().map(|s| (*s).to_string()).collect())
            .unwrap_or_default(),
        attr_differs: row.attr_differs,
        distance: row.distance,
        touched: row.touched,
        writable: row.writable,
        shape: row.shape.as_str().to_string(),
        values: row.values.iter().map(|v| v.as_deref().map(hex)).collect(),
    }
}

/// The toolkit's name for a setting, matched the way the text dump matches it.
fn prop_name(id: u32, size: usize) -> Option<String> {
    props::PROPERTIES
        .iter()
        .find(|p| p.part_size == size && p.ids().any(|i| i == id))
        .map(|p| p.name.to_string())
}

/// Whether this property names a person or a body and was not asked for.
///
/// The photographer and copyright strings live under several ids, because the
/// newer bodies moved them; all of them are covered by the one switch.
fn is_identifying(id: u32, size: usize, privacy: Privacy) -> bool {
    let withheld = |name: &str| match name {
        "serialNumber" => !privacy.serial,
        "photographer1" | "photographer2" | "photographer1Alt" => !privacy.photographer,
        "copyright1" | "copyright2" | "copyright1Alt" => !privacy.copyright,
        _ => false,
    };
    props::PROPERTIES
        .iter()
        .any(|p| withheld(p.name) && p.part_size == size && p.ids().any(|i| i == id))
}

/// A value that matches itself across dumps of one camera and says nothing
/// about which camera it was.
fn camera_id(patch: &BackupPatch) -> String {
    let digest = Sha256::digest(crate::direct::camera_key(patch).as_bytes());
    digest.iter().take(6).map(|b| format!("{b:02x}")).collect()
}

/// Whether a file is JSON rather than a backup image.
///
/// A `Backup.bin` starts with binary header fields, so the first non-blank
/// byte decides it; no need to parse either format to find out.
fn looks_like_json(data: &[u8]) -> bool {
    data.iter().find(|b| !b.is_ascii_whitespace()) == Some(&b'{')
}

fn parse_hex_u32(text: &str) -> Option<u32> {
    u32::from_str_radix(text.trim().trim_start_matches("0x"), 16).ok()
}

/// Bytes from spaced or unspaced lowercase hex; anything unparseable is
/// dropped rather than failing the whole dump.
fn parse_hex_bytes(text: &str) -> Vec<u8> {
    let digits: Vec<u8> = text.bytes().filter(|b| !b.is_ascii_whitespace()).collect();
    digits
        .chunks(2)
        .filter(|c| c.len() == 2)
        .filter_map(|c| u8::from_str_radix(std::str::from_utf8(c).ok()?, 16).ok())
        .collect()
}

fn invalid(l: &Localizer, error: &impl std::fmt::Display) -> Error {
    Error::Operation(l.tr_arg("tools-backup-invalid", "detail", &error.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn localizer() -> Localizer {
        Localizer::new("en-US")
    }

    /// A revision-4 image whose subsystem `0x00e7` holds three single bytes
    /// and, at index 3, the four bytes the toolkit knows as the serial number.
    ///
    /// The subsystem table has to run all the way to `0x00e7` for that id to
    /// resolve at all, which is why the tables are this large.
    fn image(serial: [u8; 4], other: u8) -> Vec<u8> {
        const SUBSYSTEMS: u32 = 0xe8;
        const PROPERTIES: u32 = 4;
        let data_offset: u32 = 0x700;
        let data_size: u32 = 0x100;
        let mut buf = vec![0u8; (data_offset + data_size) as usize];
        buf[12..16].copy_from_slice(b"BK4\0");
        buf[16..20].copy_from_slice(&SUBSYSTEMS.to_le_bytes());
        buf[20..24].copy_from_slice(&PROPERTIES.to_le_bytes());
        buf[24..28].copy_from_slice(&data_offset.to_le_bytes());
        buf[28..32].copy_from_slice(&data_size.to_le_bytes());
        buf[0xc0..0xc4].copy_from_slice(b"XX_U");

        // Every subsystem is empty except 0x00e7, which owns all four entries.
        let subsystem = 0x100 + 0xe7 * 6;
        buf[subsystem..subsystem + 2].copy_from_slice(&(PROPERTIES as u16).to_le_bytes());
        buf[subsystem + 2..subsystem + 6].copy_from_slice(&0u32.to_le_bytes());

        let table = 0x100 + SUBSYSTEMS as usize * 6;
        let entries: [(u16, u32, u32); 4] =
            [(0, 1, 0x700), (0, 1, 0x701), (0, 1, 0x702), (0, 4, 0x710)];
        for (i, (attr, size, off)) in entries.iter().enumerate() {
            let e = table + i * 6;
            buf[e..e + 2].copy_from_slice(&attr.to_le_bytes());
            buf[e + 2..e + 6].copy_from_slice(&((size << 24) | off).to_le_bytes());
        }
        buf[0x700] = other;
        buf[0x710..0x714].copy_from_slice(&serial);

        let end = (data_offset + data_size) as usize;
        let sum: u32 = buf[..end]
            .iter()
            .enumerate()
            .filter(|(i, _)| !(0x20..0x24).contains(i))
            .map(|(_, &b)| u32::from(b))
            .fold(0u32, u32::wrapping_add);
        buf[0x20..0x24].copy_from_slice(&sum.to_le_bytes());
        buf
    }

    /// The id of the first of those single bytes.
    const FIRST: &str = "0x00e70000";

    fn dump_of(serial: [u8; 4], other: u8) -> DumpFile {
        dump_image(&localizer(), &image(serial, other), Privacy::MASKED, None).unwrap()
    }

    #[test]
    fn a_dump_round_trips_through_json() {
        let file = dump_of([1, 2, 3, 4], 9);
        let text = serde_json::to_string(&file).unwrap();
        let back: DumpFile = serde_json::from_str(&text).unwrap();
        assert_eq!(file, back);
        assert_eq!(back.to_dump().entries.len(), file.properties.len());
    }

    #[test]
    fn values_survive_the_hex_round_trip() {
        let file = dump_of([1, 2, 3, 4], 9);
        let dump = file.to_dump();
        assert_eq!(dump.entry(0x00e7_0000).unwrap().data, vec![9]);
    }

    #[test]
    fn the_serial_number_is_withheld_but_its_length_is_kept() {
        let file = dump_of([1, 2, 3, 4], 9);
        let serial = file.properties.iter().find(|p| p.id == "0x00e70003").unwrap();
        assert_eq!(serial.name.as_deref(), Some("serialNumber"));
        assert!(serial.redacted);
        assert_eq!(serial.size, 4);
        assert_eq!(serial.hex, "00 00 00 00");
        assert!(!serde_json::to_string(&file).unwrap().contains("01 02 03 04"));
    }

    #[test]
    fn the_camera_id_matches_itself_without_carrying_the_serial() {
        let one = dump_of([1, 2, 3, 4], 9);
        let same = dump_of([1, 2, 3, 4], 7);
        let other = dump_of([9, 9, 9, 9], 9);
        assert_eq!(one.camera.camera_id, same.camera.camera_id);
        assert_ne!(one.camera.camera_id, other.camera.camera_id);
        assert!(!one.camera.camera_id.contains("01020304"));
    }

    #[test]
    fn two_identical_images_compare_as_identical() {
        let a = (PathBuf::from("a.json"), dump_of([1, 2, 3, 4], 9));
        let b = (PathBuf::from("b.json"), dump_of([1, 2, 3, 4], 9));
        let report = compare_dumps(&[a, b], &CompareOptions::default());
        assert!(report.rows.iter().all(|r| r.class != "different"));
        assert_eq!(report.total, report.rows.len());
    }

    #[test]
    fn a_changed_byte_shows_up_as_similar() {
        let a = (PathBuf::from("a.json"), dump_of([1, 2, 3, 4], 9));
        let b = (PathBuf::from("b.json"), dump_of([9, 9, 9, 9], 8));
        let options = CompareOptions { auto_noise: false, ..CompareOptions::default() };
        let report = compare_dumps(&[a, b], &options);
        let row = report.rows.iter().find(|r| r.id == FIRST).unwrap();
        assert_eq!(row.class, "similar");
        assert_eq!(row.values, vec![Some("09".into()), Some("08".into())]);
    }

    #[test]
    fn two_different_cameras_produce_no_measured_noise() {
        let a = (PathBuf::from("a.json"), dump_of([1, 2, 3, 4], 9));
        let b = (PathBuf::from("b.json"), dump_of([9, 9, 9, 9], 8));
        let report = compare_dumps(&[a, b], &CompareOptions::default());
        assert!(!report.noise_measured);
        assert!(report.noise_ids.is_empty());
    }

    #[test]
    fn the_same_camera_twice_turns_its_differences_into_noise() {
        let a = (PathBuf::from("a.json"), dump_of([1, 2, 3, 4], 9));
        let b = (PathBuf::from("b.json"), dump_of([1, 2, 3, 4], 8));
        let report = compare_dumps(&[a, b], &CompareOptions::default());
        assert!(report.noise_measured);
        let row = report.rows.iter().find(|r| r.id == FIRST).unwrap();
        assert_eq!(row.class, "noise");
    }

    #[test]
    fn a_filter_keeps_the_counts_but_drops_the_rows() {
        let a = (PathBuf::from("a.json"), dump_of([1, 2, 3, 4], 9));
        let b = (PathBuf::from("b.json"), dump_of([9, 9, 9, 9], 8));
        let options = CompareOptions { auto_noise: false, ..CompareOptions::default() };
        let report = compare_dumps(&[a, b], &options);
        let filter =
            Filter { classes: ["similar".to_string()].into_iter().collect(), ..Filter::default() };
        let narrowed = report.filtered(&filter);
        assert_eq!(narrowed.counts, report.counts);
        assert_eq!(narrowed.included, vec!["similar".to_string()]);
        assert!(narrowed.rows.iter().all(|r| r.class == "similar"));
        assert!(narrowed.rows.len() < report.rows.len());
    }

    #[test]
    fn the_layout_reports_the_reserve_before_the_data_region() {
        let file = dump_of([1, 2, 3, 4], 9);
        assert_eq!(file.layout.free.subsystem_table, 0xe8 * 6);
        assert_eq!(file.layout.free.property_table, 4 * 6);
        assert_eq!(file.layout.free.table_reserve, 0x700 - (0x100 + 0xe8 * 6 + 4 * 6));
        assert_eq!(file.layout.free.data_size, 0x100);
        assert_eq!(file.layout.free.data_used, 3 + 4);
        assert_eq!(file.layout.free.data_free, 0x100 - 7);
        assert_eq!(file.layout.present, 4);
    }

    #[test]
    fn json_is_recognised_without_parsing_it() {
        assert!(looks_like_json(b"  {\"schema\": 1}"));
        assert!(!looks_like_json(&image([0; 4], 0)));
    }

    #[test]
    fn hex_parsing_accepts_both_spacings() {
        assert_eq!(parse_hex_bytes("00 ff 1a"), vec![0x00, 0xff, 0x1a]);
        assert_eq!(parse_hex_bytes("00ff1a"), vec![0x00, 0xff, 0x1a]);
        assert_eq!(parse_hex_bytes(""), Vec::<u8>::new());
    }

    #[test]
    fn comparing_fewer_than_two_files_is_refused() {
        let err =
            compare_paths(&localizer(), &[PathBuf::from("a.json")], &CompareOptions::default());
        assert!(err.is_err());
    }
}
