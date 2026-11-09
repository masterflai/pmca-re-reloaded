//! Reading a settings store out as JSON and comparing several of them.
//!
//! Only [`compare_dump_camera`] touches a camera; the other three work on
//! files alone and need neither the camera lock nor elevated rights.
//!
//! The comparison is kept in [`AppState`] rather than handed back and forth:
//! a real image holds thousands of properties, and sending the whole result
//! into the webview only to send it back for an export would move several
//! megabytes through the IPC channel for no gain.

use crate::*;

/// The three switches the view offers, as the dump layer wants them.
fn privacy(serial: bool, photographer: bool, copyright: bool) -> pmca_app::compare::Privacy {
    pmca_app::compare::Privacy { serial, photographer, copyright }
}

/// Read the connected camera and write a dump of it into `folder`.
///
/// The one command here that needs the camera, so it takes the camera lock
/// and goes through the elevated helper on macOS, like every other
/// service-mode operation.
#[tauri::command]
pub(crate) async fn compare_dump_camera(
    app: AppHandle,
    folder: String,
    with_serial: bool,
    with_photographer: bool,
    with_copyright: bool,
    keep_image: bool,
) -> Result<pmca_app::compare::DumpSummary, CommandError> {
    with_camera(app, move |app, state| {
        let folder = PathBuf::from(&folder);
        #[cfg(target_os = "macos")]
        {
            let relayed = remote::call(
                app,
                state,
                remote::Request::CompareDumpCamera {
                    folder: folder.clone(),
                    with_serial,
                    with_photographer,
                    with_copyright,
                    keep_image,
                },
            )?;
            pmca_app::ownership::return_paths(&[folder]);
            serde_json::from_value(relayed).map_err(CommandError::internal)
        }
        #[cfg(not(target_os = "macos"))]
        {
            let l = state.localizer()?;
            Ok(pmca_app::compare::dump_camera(
                &l,
                &mut progress(app),
                &folder,
                privacy(with_serial, with_photographer, with_copyright),
                keep_image,
            )?)
        }
    })
    .await
}

/// Read a dump or a backup image and report what it holds, writing nothing.
///
/// What the list of dumps shows beside each entry: which camera it came from
/// and how much it holds. Without this a dump somebody else made would appear
/// as a file name and nothing else.
#[tauri::command]
pub(crate) async fn compare_inspect(
    app: AppHandle,
    path: String,
) -> Result<pmca_app::compare::DumpSummary, CommandError> {
    in_background(move || {
        let state = app.state::<AppState>();
        let localizer = state.localizer()?;
        let path = PathBuf::from(path);
        let dump =
            pmca_app::compare::read_dump(&localizer, &path, pmca_app::compare::Privacy::MASKED)?;
        Ok(dump.summary(&path, None))
    })
    .await
}

/// Ask for the folder the dumps of a session are written to.
///
/// `None` when the dialog was dismissed.
#[tauri::command(async)]
pub(crate) fn compare_pick_folder(
    app: AppHandle,
    window: Window,
) -> Result<Option<String>, CommandError> {
    Ok(app.dialog().file().set_parent(&window).blocking_pick_folder().map(|p| p.to_string()))
}

/// The folder the view offers before anyone has chosen one.
#[tauri::command(async)]
pub(crate) fn compare_default_folder() -> Option<String> {
    pmca_app::compare::default_folder().map(|p| p.display().to_string())
}

/// Turn a backup image into a dump in `folder`.
#[tauri::command]
pub(crate) async fn compare_dump(
    app: AppHandle,
    input: String,
    folder: String,
    with_serial: bool,
    with_photographer: bool,
    with_copyright: bool,
) -> Result<pmca_app::compare::DumpSummary, CommandError> {
    in_background(move || {
        let state = app.state::<AppState>();
        let localizer = state.localizer()?;
        Ok(pmca_app::compare::dump_image_file(
            &localizer,
            &PathBuf::from(input),
            &PathBuf::from(folder),
            privacy(with_serial, with_photographer, with_copyright),
        )?)
    })
    .await
}

/// Compare two or more dumps or images and keep the result for an export.
#[tauri::command]
pub(crate) async fn compare_run(
    app: AppHandle,
    paths: Vec<String>,
    similar_distance: usize,
    auto_noise: bool,
) -> Result<pmca_app::compare::CompareReport, CommandError> {
    in_background(move || {
        let state = app.state::<AppState>();
        let localizer = state.localizer()?;
        let paths: Vec<PathBuf> = paths.into_iter().map(PathBuf::from).collect();
        let options = pmca_app::compare::CompareOptions {
            similar_distance,
            auto_noise,
            ..pmca_app::compare::CompareOptions::default()
        };
        let report = pmca_app::compare::compare_paths(&localizer, &paths, &options)?;
        *state.comparison.lock().map_err(CommandError::internal)? = Some(report.clone());
        Ok(report)
    })
    .await
}

/// Write the comparison that was last run out as JSON, keeping only the rows
/// the filter asks for. Returns how many rows were written.
#[tauri::command]
pub(crate) async fn compare_export(
    app: AppHandle,
    path: String,
    classes: Vec<String>,
    only_writable: bool,
    only_named: bool,
    only_switches: bool,
) -> Result<usize, CommandError> {
    in_background(move || {
        let state = app.state::<AppState>();
        let localizer = state.localizer()?;
        let report = state
            .comparison
            .lock()
            .map_err(CommandError::internal)?
            .clone()
            .ok_or_else(|| CommandError::localized(localizer.tr("compare-needs-two")))?;
        let filter = pmca_app::compare::Filter {
            classes: classes.into_iter().collect(),
            only_writable,
            only_named,
            only_switches,
        };
        let narrowed = report.filtered(&filter);
        pmca_app::compare::save(&narrowed, &PathBuf::from(path))?;
        Ok(narrowed.rows.len())
    })
    .await
}

/// Forget the comparison, so leaving the page does not keep a large report in
/// memory for the rest of the session.
#[tauri::command(async)]
pub(crate) fn compare_close(state: State<'_, AppState>) -> Result<(), CommandError> {
    *state.comparison.lock().map_err(CommandError::internal)? = None;
    Ok(())
}
