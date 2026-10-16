//! Native file and choice dialogs, and how paths are shown.

use crate::*;

/// Render `path` as an absolute path fit for showing to a user.
///
/// `canonicalize` is what resolves a relative name, but on Windows it returns
/// the `\\?\` extended-length form, which is correct yet unfamiliar enough to
/// look like a defect. The prefix is therefore dropped again for display. If
/// the path cannot be resolved at all the original is kept: a plain name is a
/// better answer than none.
pub(crate) fn absolute_display(path: &Path) -> String {
    let resolved = std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
    let text = resolved.to_string_lossy();
    text.strip_prefix(r"\\?\").unwrap_or(&text).to_string()
}

/// Extensions and catalogue key for each kind of file the dialogs offer.
///
/// The mapping lives here rather than in the window so the front-end cannot ask
/// for a filter the application does not support.
pub(crate) fn dialog_filter(
    kind: &str,
) -> Result<(&'static str, &'static [&'static str]), CommandError> {
    match kind {
        "apk" => Ok(("file-filter-apk", &["apk"])),
        "firmware" => Ok(("file-filter-firmware", &["dat"])),
        "gps" => Ok(("file-filter-gps", &["dat"])),
        "usage-log" => Ok(("file-filter-usage-log", &["bin"])),
        "catalog" => Ok(("file-filter-catalog", &["ftl"])),
        "analysis" => Ok(("file-filter-analysis", &["json"])),
        "spk" => Ok(("file-filter-spk", &["spk"])),
        "photo" => Ok(("file-filter-photo", &["arw", "jpg", "jpeg", "hif", "heif", "heic"])),
        "plugin" => Ok(("file-filter-plugin", &["pmcaplugin"])),
        "any" => Ok(("file-filter-any", &["*"])),
        _ => Err(CommandError {
            code: "unknown-file-kind",
            message: format!("unknown file kind: {kind}"),
            detail: Some(kind.to_string()),
            recovery: None,
        }),
    }
}

/// Ask the user to select an existing file.
///
/// Dropping a file onto the window covers only pointer users who already have
/// an Explorer window open; typing a path exactly is error-prone. A dialog is
/// the way the platform expects a file to be chosen, and it is reachable from
/// the keyboard.
///
/// A dismissed dialog yields `None` rather than an error — cancelling is a
/// normal outcome, not a failure worth reporting in red.
///
/// `async` is required, not cosmetic: Tauri 2 runs a plain synchronous command
/// on the main thread, and the blocking dialog call would then stall the event
/// loop until the dialog closes — the window turns "not responding" while the
/// unowned dialog may sit hidden behind it. As an async command it waits on a
/// runtime worker instead. The parent window makes the dialog modal and keeps
/// it in front of the window that asked for it.
#[tauri::command(async)]
pub(crate) fn pick_file(
    app: AppHandle,
    window: Window,
    state: State<'_, AppState>,
    kind: String,
) -> Result<Option<String>, CommandError> {
    let (label_key, extensions) = dialog_filter(&kind)?;
    let label = state.localizer()?.tr(label_key);
    let picked =
        app.dialog().file().set_parent(&window).add_filter(label, extensions).blocking_pick_file();
    Ok(picked.map(|p| p.to_string()))
}

/// Show the native save dialog for `kind`; `None` when it was dismissed.
///
/// Only called by the commands that write the file themselves, so the window
/// never supplies a path to write to.
pub(crate) fn ask_save_path(
    app: &AppHandle,
    window: &Window,
    state: &AppState,
    kind: &str,
    default_name: &str,
) -> Result<Option<PathBuf>, CommandError> {
    let (label_key, extensions) = dialog_filter(kind)?;
    let label = state.localizer()?.tr(label_key);
    let picked = app
        .dialog()
        .file()
        .set_parent(window)
        .add_filter(label, extensions)
        .set_file_name(default_name)
        .blocking_save_file();
    picked.map(|p| p.into_path().map_err(CommandError::internal)).transpose()
}

/// Offer a choice between two named actions in a native, modal dialog.
///
/// Replaces `window.confirm`, which the dialog plugin overrides with a call to
/// a `confirm` command it no longer registers. Both buttons name what they do,
/// since a bare "Yes"/"No" is ambiguous after a warning; `warning` selects the
/// warning icon for risky steps. Only the `ok` button returns `true`. Runs
/// `async` for the reason given at [`pick_file`].
#[tauri::command(async)]
pub(crate) fn ask_choice(
    app: AppHandle,
    window: Window,
    message: String,
    ok: String,
    cancel: String,
    warning: Option<bool>,
) -> bool {
    let title = window.title().unwrap_or_default();
    let kind =
        if warning == Some(true) { MessageDialogKind::Warning } else { MessageDialogKind::Info };
    app.dialog()
        .message(message)
        .title(title)
        .kind(kind)
        .buttons(MessageDialogButtons::OkCancelCustom(ok, cancel))
        .parent(&window)
        .blocking_show()
}
