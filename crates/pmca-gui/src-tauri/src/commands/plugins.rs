//! Plugins: listing, installing, removing, and running their tools and
//! diagnostics.

use serde::Serialize;

use crate::*;

/// The installed plugins and whether a restart is needed to apply changes.
#[tauri::command]
pub(crate) fn plugins_list() -> pmca_app::plugins::PluginList {
    pmca_app::plugins::list()
}

/// Check a plugin file the user picked and say what it would do.
#[tauri::command]
pub(crate) async fn plugins_inspect(
    app: AppHandle,
    path: String,
) -> Result<pmca_app::plugins::PluginInfo, CommandError> {
    in_background(move || {
        let localizer = app.state::<AppState>().localizer()?;
        Ok(pmca_app::plugins::inspect(&localizer, Path::new(&path))?)
    })
    .await
}

/// Install a plugin file; it takes effect at the next start.
#[tauri::command]
pub(crate) async fn plugins_install(
    app: AppHandle,
    path: String,
) -> Result<pmca_app::plugins::PluginInfo, CommandError> {
    in_background(move || {
        let localizer = app.state::<AppState>().localizer()?;
        Ok(pmca_app::plugins::install(&localizer, Path::new(&path))?)
    })
    .await
}

/// Remove an installed plugin; it stays in effect until the next start.
#[tauri::command]
pub(crate) fn plugins_remove(state: State<'_, AppState>, id: String) -> Result<(), CommandError> {
    Ok(pmca_app::plugins::remove(&state.localizer()?, &id)?)
}

/// The compatibility rows the loaded plugins add.
#[tauri::command]
pub(crate) fn plugins_compatibility() -> Vec<pmca_app::plugins::PluginCompat> {
    pmca_app::plugins::compatibility()
}

/// The tools (`diagnostics == false`) or diagnostics of the loaded plugins.
#[tauri::command]
pub(crate) fn plugins_entries(diagnostics: bool) -> Vec<pmca_app::plugins::PluginEntry> {
    pmca_app::plugins::entries(diagnostics)
}

/// The result of a plugin tool, with where its output went, if anywhere.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PluginToolResult {
    outcome: pmca_app::plugins::PluginRunOutcome,
    /// The path the output was saved to; `None` if there was none or the
    /// save dialog was dismissed.
    written: Option<String>,
}

/// Run a plugin tool on a file, then offer to save what it produced.
///
/// The output never crosses into the window: it is written here, to the path
/// the user picks in a native dialog, so the window cannot choose where.
#[tauri::command]
pub(crate) async fn plugins_run_tool(
    app: AppHandle,
    window: Window,
    plugin: String,
    tool: String,
    input: String,
) -> Result<PluginToolResult, CommandError> {
    in_background(move || {
        let localizer = app.state::<AppState>().localizer()?;
        let input = PathBuf::from(input);
        let outcome = pmca_app::plugins::run_tool(&localizer, &plugin, &tool, &input)?;
        let written = match &outcome.output_extension {
            Some(extension) if outcome.status != "error" => {
                let name = input.file_stem().map_or_else(
                    || format!("output.{extension}"),
                    |stem| format!("{}.{extension}", stem.to_string_lossy()),
                );
                let picked = app
                    .dialog()
                    .file()
                    .set_parent(&window)
                    .add_filter(extension.to_uppercase(), &[extension.as_str()])
                    .set_file_name(&name)
                    .blocking_save_file();
                match picked {
                    Some(path) => {
                        let path = path.into_path().map_err(CommandError::internal)?;
                        std::fs::write(&path, &outcome.output).map_err(pmca_app::Error::from)?;
                        Some(absolute_display(&path))
                    }
                    None => None,
                }
            }
            _ => None,
        };
        Ok(PluginToolResult { outcome, written })
    })
    .await
}

/// Run a plugin diagnostic.
#[tauri::command]
pub(crate) async fn plugins_run_diagnostic(
    app: AppHandle,
    plugin: String,
    diagnostic: String,
) -> Result<pmca_app::plugins::PluginRunOutcome, CommandError> {
    in_background(move || {
        let localizer = app.state::<AppState>().localizer()?;
        Ok(pmca_app::plugins::run_diagnostic(&localizer, &plugin, &diagnostic)?)
    })
    .await
}
