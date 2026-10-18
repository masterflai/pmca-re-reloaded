//! File tools that need no camera.

use crate::*;

/// Run one of the file operations that need no camera.
///
/// The six operations are the console's file subcommands; naming them by
/// string keeps the front-end from having to model the request shape, and
/// [`pmca_app::tools`] rejects an unknown one. The two converters ask for
/// their output file in a save dialog opened here; `None` when it was
/// dismissed.
#[tauri::command]
pub(crate) async fn run_tool(
    app: AppHandle,
    window: Window,
    operation: String,
    input: String,
) -> Result<Option<pmca_app::tools::ToolOutcome>, CommandError> {
    in_background(move || {
        let state = app.state::<AppState>();
        let localizer = state.localizer()?;
        let input = PathBuf::from(input);
        let output = match converted_kind(&operation) {
            Some((kind, extension)) => {
                let name = input.file_stem().map_or_else(
                    || format!("output.{extension}"),
                    |stem| format!("{}.{extension}", stem.to_string_lossy()),
                );
                match ask_save_path(&app, &window, state.inner(), kind, &name)? {
                    Some(path) => Some(path),
                    None => return Ok(None),
                }
            }
            None => None,
        };
        tool(&localizer, &operation, input, output).map(Some)
    })
    .await
}

/// The dialog kind and file extension of a converter's output.
fn converted_kind(operation: &str) -> Option<(&'static str, &'static str)> {
    match operation {
        "apk2spk" => Some(("spk", "spk")),
        "spk2apk" => Some(("apk", "apk")),
        _ => None,
    }
}

/// Map a tool name from the window onto its request and run it.
pub(crate) fn tool(
    localizer: &Localizer,
    operation: &str,
    input: PathBuf,
    output: Option<PathBuf>,
) -> Result<pmca_app::tools::ToolOutcome, CommandError> {
    let missing_output = || CommandError {
        code: "tool-output-required".into(),
        message: localizer.tr("tools-file-required"),
        detail: None,
        recovery: None,
    };
    let request = match operation {
        "apk2spk" => pmca_app::tools::ToolRequest::ApkToSpk {
            input,
            output: output.ok_or_else(missing_output)?,
        },
        "spk2apk" => pmca_app::tools::ToolRequest::SpkToApk {
            input,
            output: output.ok_or_else(missing_output)?,
        },
        "print-backup" => pmca_app::tools::ToolRequest::PrintBackup { input },
        "print-usagelog" => pmca_app::tools::ToolRequest::PrintUsageLog { input },
        "guess-firmware" => pmca_app::tools::ToolRequest::GuessFirmware { input },
        "xpd-checksum" => pmca_app::tools::ToolRequest::XpdChecksum { input },
        other => {
            return Err(CommandError {
                code: "unknown-tool".into(),
                message: format!("unknown tool: {other}"),
                detail: Some(other.to_string()),
                recovery: None,
            });
        }
    };
    Ok(pmca_app::tools::run(localizer, &request)?)
}
