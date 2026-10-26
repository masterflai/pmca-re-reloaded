//! `sources`: the GitHub repositories apps and plugins come from.

use crate::*;

/// Show, add, remove and search for sources.
pub(crate) fn sources_command(
    l: &Localizer,
    action: &SourcesAction,
) -> Result<(), Box<dyn std::error::Error>> {
    use pmca_app::sources;

    match action {
        SourcesAction::List => {
            let list = sources::list();
            if list.is_empty() {
                println!("{}", l.tr("sources-none"));
            }
            for source in &list {
                let stars = sources::repo(l, &source.id).map(|r| format!("  ★ {}", r.stars));
                println!("{}{}", source.id, stars.unwrap_or_default());
                println!("  {}", source.url);
            }
            for suggestion in sources::suggestions() {
                println!();
                println!("{}: {}", l.tr("sources-suggestion-heading"), suggestion.source.id);
                println!("  {}", l.tr(&suggestion.note_key));
                println!("  pmca-console sources add {}", suggestion.source.id);
            }
            println!();
            println!("{}", l.tr("sources-intro"));
        }
        SourcesAction::Add { repo } => {
            let source = sources::add(l, repo)?;
            println!("{}", l.tr_arg("sources-added", "source", &source.id));
            println!("{}", l.tr_arg("sources-check-repo", "url", &source.url));
        }
        SourcesAction::Remove { id } => {
            sources::remove(l, id)?;
            println!("{}", l.tr_arg("sources-removed", "source", id));
        }
        SourcesAction::Search => {
            println!("{}", l.tr("sources-search-hint"));
            println!();
            let hits = sources::search()?;
            if hits.is_empty() {
                println!("{}", l.tr("sources-search-none"));
            }
            for hit in &hits {
                let mut notes = Vec::new();
                if hit.archived {
                    notes.push(l.tr("sources-archived"));
                }
                if hit.added {
                    notes.push(l.tr("sources-already"));
                }
                let notes = if notes.is_empty() {
                    String::new()
                } else {
                    format!("  ({})", notes.join(", "))
                };
                println!("{}  ★ {}{}", hit.id, hit.stars, notes);
                if let Some(description) = &hit.description {
                    println!("  {description}");
                }
                println!("  {}", hit.url);
            }
        }
    }
    Ok(())
}

/// The one source offering `package`, or the one the user named.
pub(crate) fn pick_source(
    l: &Localizer,
    package: &str,
    named: Option<&str>,
    kind: pmca_market::appstore::EntryKind,
) -> Result<String, Box<dyn std::error::Error>> {
    if let Some(source) = named {
        return Ok(source.to_string());
    }
    let offering = pmca_app::store::sources_offering(package, kind);
    match offering.as_slice() {
        [one] => Ok(one.clone()),
        [] => Err(format!("{}: {package}", l.tr("store-unknown-app")).into()),
        many => Err(l
            .tr_args("source-ambiguous", &[("package", package), ("sources", &many.join(", "))])
            .into()),
    }
}

/// Print what a listing could not read.
pub(crate) fn print_failed_sources(l: &Localizer, failed: &[pmca_app::dto::SourceFailure]) {
    for failure in failed {
        eprintln!(
            "{}",
            l.tr_args("source-failed", &[("source", &failure.source), ("reason", &failure.reason)])
        );
    }
}
