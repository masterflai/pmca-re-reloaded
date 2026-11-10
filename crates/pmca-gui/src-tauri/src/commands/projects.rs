//! Projects: independent GitHub repositories the user follows; nothing of them
//! is downloaded.

use crate::*;
use pmca_app::projects::{PinnedProject, ProjectInfo, ProjectList, ProjectRepo};

/// The pinned projects; reads only the local file.
#[tauri::command]
pub(crate) fn projects_list() -> Vec<PinnedProject> {
    pmca_app::projects::list()
}

/// Pin a project after making sure it publishes a project file.
#[tauri::command]
pub(crate) async fn projects_pin(
    app: AppHandle,
    spec: String,
) -> Result<PinnedProject, CommandError> {
    in_background(move || {
        let localizer = app.state::<AppState>().localizer()?;
        Ok(pmca_app::projects::pin(&localizer, &spec)?)
    })
    .await
}

/// Unpin a project.
#[tauri::command]
pub(crate) fn projects_unpin(state: State<'_, AppState>, id: String) -> Result<(), CommandError> {
    Ok(pmca_app::projects::unpin(&state.localizer()?, &id)?)
}

/// Search GitHub for projects; only ever on the user's request.
#[tauri::command]
pub(crate) async fn projects_search() -> Result<Vec<ProjectRepo>, CommandError> {
    in_background(|| Ok(pmca_app::projects::search()?)).await
}

/// Read every pinned project afresh.
#[tauri::command]
pub(crate) async fn projects_refresh(app: AppHandle) -> Result<ProjectList, CommandError> {
    in_background(move || {
        let localizer = app.state::<AppState>().localizer()?;
        Ok(pmca_app::projects::refresh(&localizer)?)
    })
    .await
}

/// Everything about one project, for a search hit the user opens.
#[tauri::command]
pub(crate) async fn project_info(app: AppHandle, id: String) -> Result<ProjectInfo, CommandError> {
    in_background(move || {
        let localizer = app.state::<AppState>().localizer()?;
        Ok(pmca_app::projects::info(&localizer, &id)?)
    })
    .await
}
