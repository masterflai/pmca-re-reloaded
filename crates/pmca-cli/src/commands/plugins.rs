//! `plugins`: list, check, install, remove, run and pack plugins.

use crate::*;

/// Manage and run plugins.
pub(crate) fn plugins_command(
    l: &Localizer,
    action: &PluginsAction,
) -> Result<(), Box<dyn std::error::Error>> {
    use pmca_app::plugins;

    match action {
        PluginsAction::List => {
            let list = plugins::list();
            if list.plugins.is_empty() && list.failed.is_empty() {
                println!("{}", l.tr("plugin-none"));
            }
            for plugin in &list.plugins {
                print_plugin(l, plugin);
            }
            for failed in &list.failed {
                println!("{}: {}", failed.file, plugins::refusal(l, &failed.reason));
            }
            if list.restart_required {
                println!("{}", l.tr("plugin-restart"));
            }
        }
        PluginsAction::Inspect { file } => print_plugin(l, &plugins::inspect(l, file)?),
        PluginsAction::Install { file } => {
            print_plugin(l, &plugins::install(l, file)?);
            println!("{}", l.tr("plugin-installed"));
            println!("{}", l.tr("plugin-restart"));
        }
        PluginsAction::Remove { id } => {
            plugins::remove(l, id)?;
            println!("{}", l.tr_arg("plugin-removed", "id", id));
            println!("{}", l.tr("plugin-restart"));
        }
        PluginsAction::Run { plugin, entry, input, output } => {
            let outcome = match input {
                Some(input) => plugins::run_tool(l, plugin, entry, input)?,
                None => plugins::run_diagnostic(l, plugin, entry)?,
            };
            if outcome.status == "error" {
                return Err(l.tr_arg("plugin-failed", "reason", &outcome.report).into());
            }
            if !outcome.report.is_empty() {
                println!("{}", outcome.report);
            }
            if let Some(extension) = &outcome.output_extension {
                // A diagnostic has no input to name the output after.
                let path = match (output, input) {
                    (Some(path), _) => path.clone(),
                    (None, Some(input)) => input.with_extension(extension),
                    (None, None) => std::path::PathBuf::from(format!("{entry}.{extension}")),
                };
                std::fs::write(&path, &outcome.output)?;
                println!("{}", l.tr_arg("tools-written", "path", &path.display().to_string()));
            }
        }
        PluginsAction::Pack { dir, file, key } => {
            print_plugin(l, &plugins::pack(l, dir, file, key.as_deref())?);
            println!("{}", l.tr_arg("tools-written", "path", &file.display().to_string()));
        }
        PluginsAction::Keygen { file } => {
            let public_key = plugins::generate_key(l, file)?;
            let path = file.display().to_string();
            println!(
                "{}",
                l.tr_args("plugin-key-written", &[("path", &path), ("key", &public_key)])
            );
        }
        PluginsAction::Available => {
            if pmca_app::sources::list().is_empty() {
                println!("{}", l.tr("store-no-sources"));
                return Ok(());
            }
            let list = pmca_app::store::community_plugins();
            print_failed_sources(l, &list.failed);
            if list.entries.is_empty() {
                println!("{}", l.tr("plugin-sources-none"));
            }
            for entry in &list.entries {
                println!("{:<32} {}  [{}]", entry.package, entry.name, entry.source);
                if let Some(desc) = entry.description.as_deref().filter(|d| !d.is_empty()) {
                    println!("{:<32} {desc}", "");
                }
            }
        }
        PluginsAction::Get { id, source } => {
            let kind = pmca_market::appstore::EntryKind::Plugin;
            let source = pick_source(l, id, source.as_deref(), kind)?;
            let mut sink = |event: pmca_app::ProgressEvent| println!("{}", event.message);
            let (info, bytes) =
                pmca_app::store::download_community_plugin(l, &source, id, &mut sink)?;
            print_plugin(l, &info);
            println!("{}", l.tr_arg("plugin-source-unchecked", "source", &source));
            plugins::install_bytes(l, &bytes, &format!("{id}.pmcaplugin"))?;
            println!("{}", l.tr("plugin-installed"));
            println!("{}", l.tr("plugin-restart"));
        }
    }
    Ok(())
}

/// Name, version, what the plugin may do, and how to run its entries.
fn print_plugin(l: &Localizer, plugin: &pmca_app::plugins::PluginInfo) {
    // Until the next start a new plugin's own messages are not loaded yet.
    let own = |key: &Option<String>, written: &str| match key {
        Some(key) if l.tr(key) != *key => l.tr(key),
        _ => written.to_string(),
    };
    println!("{} {} ({})", own(&plugin.name_key, &plugin.name), plugin.version, plugin.id);
    if !plugin.author.is_empty() {
        println!("  {}: {}", l.tr("plugin-author"), plugin.author);
    }
    if !plugin.description.is_empty() {
        println!("  {}", own(&plugin.description_key, &plugin.description));
    }
    match &plugin.signer {
        Some(key) => println!("  {}", l.tr_arg("plugin-signed", "key", key)),
        None => println!("  {}", l.tr("plugin-unsigned")),
    }
    for permission in &plugin.permissions {
        println!("  - {}", l.tr(&format!("plugin-perm-{permission}")));
    }
    for entry in plugin.tools.iter().chain(&plugin.diagnostics) {
        // Until the next start a new plugin's names are not loaded yet.
        let label = l.tr(&entry.label_key);
        if label == entry.label_key {
            println!("  {} {}", entry.plugin, entry.id);
        } else {
            println!("  {} {} — {label}", entry.plugin, entry.id);
        }
    }
}
