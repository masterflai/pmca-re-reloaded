//! `backup`: save the camera's whole backup image to a file or write one back.

use crate::*;

/// Save or restore the camera's whole backup image.
///
/// Exactly one of `save`/`restore` is given (clap enforces it); `restore`
/// additionally needs `--confirm` and asks for the confirmation word.
pub(crate) fn backup_command(
    l: &Localizer,
    save: Option<&std::path::Path>,
    restore: Option<&std::path::Path>,
    confirm: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut sink = |event: pmca_app::ProgressEvent| println!("{}", event.message);
    eprintln!("{}", l.tr("usb-mode-advice"));
    if let Some(path) = save {
        let out = pmca_app::backup::save(l, &mut sink, path).map_err(|e| app_driver_hint(l, e))?;
        println!(
            "{}",
            l.tr_args(
                "backup-saved-at",
                &[("bytes", &out.bytes.to_string()), ("path", &out.path)],
            )
        );
    } else if let Some(path) = restore {
        // `--confirm` is required by clap; the word is the second gate.
        let _ = confirm;
        eprintln!("{}", l.tr("backup-restore-warning"));
        if !confirm_word(l) {
            return Err(l.tr("analysis-write-declined").into());
        }
        let out =
            pmca_app::backup::restore(l, &mut sink, path).map_err(|e| app_driver_hint(l, e))?;
        println!("{}", l.tr_arg("backup-restored-done", "bytes", &out.bytes.to_string()));
        if let Some(copy) = &out.safety_copy {
            println!("{}", l.tr_arg("backup-safety-copy", "path", copy));
        }
    }
    Ok(())
}
