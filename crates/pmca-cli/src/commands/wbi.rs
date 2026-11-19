//! `wbi`: Sony WBI1 warm-boot images on disk, through [`pmca_app::wbi`].
//!
//! Text output goes through the shared file tools; `--json` and `--out` print
//! or save the same report as JSON for scripts.

use std::fs::File;
use std::io::Write;
use std::path::Path;

use pmca_app::tools::ToolRequest;
use pmca_i18n::Localizer;
use serde::Serialize;

use crate::{WbiAction, tool_command};

/// Run one `wbi` subcommand.
pub(crate) fn wbi_command(
    l: &Localizer,
    action: &WbiAction,
) -> Result<(), Box<dyn std::error::Error>> {
    match action {
        WbiAction::Inspect { file, json, out } => {
            if *json || out.is_some() {
                emit_json(&pmca_app::wbi::inspect(l, file)?, out.as_deref())?;
            } else {
                tool_command(l, ToolRequest::PrintWbi { input: file.clone() })?;
            }
        }
        WbiAction::Unpack { file, out } => {
            tool_command(l, ToolRequest::UnpackWbi { input: file.clone(), output: out.clone() })?;
        }
        WbiAction::Diff { before, after, json, out } => {
            if *json || out.is_some() {
                emit_json(&pmca_app::wbi::diff(l, before, after)?, out.as_deref())?;
            } else {
                tool_command(
                    l,
                    ToolRequest::CompareWbi { input: before.clone(), against: after.clone() },
                )?;
            }
        }
    }
    Ok(())
}

/// Print `value` as JSON, or write it to a new file that must not exist yet.
fn emit_json(value: &impl Serialize, out: Option<&Path>) -> Result<(), Box<dyn std::error::Error>> {
    let rendered = serde_json::to_string_pretty(value)?;
    match out {
        Some(path) => {
            let mut file = File::options().write(true).create_new(true).open(path)?;
            file.write_all(rendered.as_bytes())?;
            file.write_all(b"\n")?;
            file.sync_all()?;
        }
        None => println!("{rendered}"),
    }
    Ok(())
}
