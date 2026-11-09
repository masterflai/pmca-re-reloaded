//! `dump-backup` and `compare-backups`: the settings store as JSON, and two or
//! more of them side by side. Neither touches a camera.

use std::path::{Path, PathBuf};

use pmca_app::compare::{
    self, CompareOptions, DEFAULT_SIMILAR_DISTANCE, Filter, Privacy, RowClass,
};
use pmca_i18n::Localizer;

/// Write one `Backup.bin` out as JSON.
pub(crate) fn dump_backup_command(
    l: &Localizer,
    file: &Path,
    out: &Path,
    privacy: Privacy,
) -> Result<(), Box<dyn std::error::Error>> {
    let image = std::fs::read(file)?;
    // A file on its own carries no firmware version; only a camera reports it.
    let dump = compare::dump_image(l, &image, privacy, None)?;
    compare::save(&dump, out)?;
    println!(
        "{}",
        l.tr_args(
            "compare-dump-saved",
            &[("count", &dump.properties.len().to_string()), ("path", &out.display().to_string()),],
        )
    );
    println!("{}", dump.privacy);
    Ok(())
}

/// Compare two or more dumps and print the result.
#[allow(clippy::too_many_arguments)]
pub(crate) fn compare_backups_command(
    l: &Localizer,
    files: &[PathBuf],
    out: Option<&Path>,
    include: Option<&[String]>,
    only_writable: bool,
    only_named: bool,
    only_switches: bool,
    similar_distance: Option<usize>,
    auto_noise: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let options = CompareOptions {
        similar_distance: similar_distance.unwrap_or(DEFAULT_SIMILAR_DISTANCE),
        auto_noise,
        ..CompareOptions::default()
    };
    let report = compare::compare_paths(l, files, &options)?;
    let filter = Filter { classes: classes(l, include)?, only_writable, only_named, only_switches };
    let narrowed = report.filtered(&filter);

    print!("{}", compare::report_text(l, &narrowed));
    if let Some(path) = out {
        compare::save(&narrowed, path)?;
        println!(
            "\n{}",
            l.tr_args(
                "compare-saved",
                &[
                    ("count", &narrowed.rows.len().to_string()),
                    ("path", &path.display().to_string()),
                ],
            )
        );
    }
    Ok(())
}

/// The classes named on the command line, or all of them.
///
/// An unknown name is refused rather than ignored: silently dropping it would
/// produce an empty export that looks like a finding.
fn classes(
    l: &Localizer,
    include: Option<&[String]>,
) -> Result<std::collections::BTreeSet<String>, Box<dyn std::error::Error>> {
    let Some(names) = include else {
        return Ok(Filter::default().classes);
    };
    let mut out = std::collections::BTreeSet::new();
    for name in names {
        let name = name.trim().to_ascii_lowercase();
        let class = RowClass::named(&name)
            .ok_or_else(|| l.tr_arg("compare-unknown-class", "value", &name))?;
        out.insert(class.as_str().to_string());
    }
    Ok(out)
}
