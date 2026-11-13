//! `projects`: independent GitHub repositories the user follows; nothing of
//! them is downloaded.

use crate::*;

/// Show, pin, unpin, search and refresh projects.
pub(crate) fn projects_command(
    l: &Localizer,
    action: &ProjectsAction,
) -> Result<(), Box<dyn std::error::Error>> {
    use pmca_app::projects;

    match action {
        ProjectsAction::List => {
            let pins = projects::list();
            if pins.is_empty() {
                println!("{}", l.tr("projects-pinned-none"));
            }
            for pin in &pins {
                println!("{}", pin.id);
                println!("  {}", pin.url);
            }
        }
        ProjectsAction::Pin { repo } => {
            let info = projects::pin(l, repo)?;
            println!("{}", l.tr_arg("projects-pinned-done", "project", &info.repo.id));
            print_project(l, &info);
        }
        ProjectsAction::Unpin { id } => {
            projects::unpin(l, id)?;
            println!("{}", l.tr_arg("projects-unpinned", "project", id));
        }
        ProjectsAction::Search => {
            println!("{}", l.tr("projects-notice"));
            println!();
            let hits = projects::search()?;
            if hits.is_empty() {
                println!("{}", l.tr("projects-search-none"));
            }
            for hit in &hits {
                let mut notes = Vec::new();
                if hit.archived {
                    notes.push(l.tr("sources-archived"));
                }
                if hit.pinned {
                    notes.push(l.tr("projects-pinned-badge"));
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
        ProjectsAction::Refresh => {
            println!("{}", l.tr("projects-notice"));
            let list = projects::refresh(l)?;
            for info in &list.entries {
                print_project(l, info);
            }
            for failure in &list.failed {
                eprintln!("{}: {}", failure.id, failure_text(l, failure));
            }
            if list.entries.is_empty() && list.failed.is_empty() {
                println!();
                println!("{}", l.tr("projects-pinned-none"));
            }
        }
    }
    Ok(())
}

fn print_project(l: &Localizer, info: &pmca_app::projects::ProjectInfo) {
    let repo = &info.repo;
    let name = info.details.as_ref().map_or(repo.id.as_str(), |d| d.name.as_str());
    println!();
    println!("{name}  ({})  ★ {}", repo.id, repo.stars);
    let release = match &info.release {
        Some(release) => {
            let date = release.published_at.as_deref().map(day).unwrap_or_default();
            let new = if info.new_release {
                format!("  [{}]", l.tr("projects-new"))
            } else {
                String::new()
            };
            format!(
                "{}{}{new}",
                release.version,
                if date.is_empty() { String::new() } else { format!(" ({date})") }
            )
        }
        None => l.tr("projects-no-release"),
    };
    println!("  {}: {release}", l.tr("projects-field-release"));
    if let Some(summary) =
        info.details.as_ref().and_then(|d| d.summary.as_ref()).or(repo.description.as_ref())
    {
        println!("  {summary}");
    }
    if info.details.is_none() {
        println!("  {}", l.tr("projects-no-file-note"));
    }
    if repo.archived {
        println!("  {}", l.tr("sources-archived"));
    }
    println!("  {}", repo.url);
}

/// The date of an ISO-8601 timestamp.
fn day(iso: &str) -> &str {
    iso.get(..10).unwrap_or(iso)
}

/// A failure in the user's language where a translation exists, as `error_text` does.
fn failure_text(l: &Localizer, failure: &pmca_app::projects::ProjectFailure) -> String {
    let key = format!("err-{}", failure.code);
    let translated = l.tr(&key);
    if failure.code == "operation" || translated == key {
        return failure.message.clone();
    }
    match &failure.detail {
        Some(detail) => format!("{translated} ({detail})"),
        None => translated,
    }
}
