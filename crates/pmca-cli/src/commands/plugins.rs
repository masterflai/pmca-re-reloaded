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
        PluginsAction::Names { find, backup, model, firmware } => {
            print_names(
                l,
                find.as_deref(),
                backup.as_deref(),
                model.as_deref(),
                firmware.as_deref(),
            )?;
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

/// The installed name tables, or the names that match `find`, optionally for one camera.
fn print_names(
    l: &Localizer,
    find: Option<&str>,
    backup: Option<&std::path::Path>,
    model: Option<&str>,
    firmware: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    use pmca_app::names;

    let tables = names::tables();
    if tables.is_empty() {
        println!("{}", l.tr("plugin-names-none"));
        return Ok(());
    }
    let camera = match (backup, model) {
        (Some(path), model) => {
            let mut key = names::camera_key_of_file(path, firmware)?;
            if let Some(model) = model {
                let given = names::camera_key_of_model(model, firmware);
                (key.model, key.platform) = (given.model, given.platform);
            }
            Some(key)
        }
        (None, Some(model)) => Some(names::camera_key_of_model(model, firmware)),
        (None, None) => None,
    };
    let Some(camera) = camera else {
        // No camera: every table on its own, so unrelated tables do not read as conflicts.
        let needle = find.map(str::to_ascii_uppercase);
        for table in tables {
            let Some(needle) = &needle else {
                println!(
                    "{}/{}  {} ({})  {}",
                    table.plugin,
                    table.id,
                    table.entries.len(),
                    names::describe_scope(&table.scope),
                    table.source
                );
                continue;
            };
            for (id, entry) in &table.entries {
                if entry.name.to_ascii_uppercase().contains(needle.as_str()) {
                    let evidence = entry.evidence.as_str();
                    println!(
                        "0x{id:08x}  fw=\"{}\" [{evidence}]  ({}/{})",
                        entry.name, table.plugin, table.id
                    );
                }
            }
        }
        return Ok(());
    };
    let view = names::NameView::new(tables, &camera);
    if view.is_empty() {
        println!("{}", l.tr("plugin-names-no-match"));
        return Ok(());
    }
    let Some(text) = find else {
        for table in view.tables() {
            println!(
                "{}/{}  {} ({})  {}",
                table.plugin,
                table.id,
                table.entries.len(),
                names::describe_scope(&table.scope),
                table.source
            );
        }
        return Ok(());
    };
    let hits = view.find(text);
    if hits.is_empty() {
        println!("{}", l.tr("plugin-names-no-match"));
    }
    for (id, named) in hits {
        println!(
            "0x{id:08x}  {}  ({}/{})",
            names::render(&named),
            named.table.plugin,
            named.table.id
        );
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
