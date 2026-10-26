//! Sources: the GitHub repositories apps and plugins come from, and installing
//! a plugin from one.

use serde::Serialize;

use crate::*;
use pmca_app::dto::CommunityList;
use pmca_app::plugins::PluginInfo;
use pmca_app::sources::{Source, SourceRepo, SourceSuggestion};

/// The sources the user has added and the suggestions not yet switched on.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SourcesState {
    sources: Vec<Source>,
    suggestions: Vec<SourceSuggestion>,
}

/// Key of a download kept for the window: source and package.
pub(crate) fn download_key(source: &str, package: &str) -> String {
    format!("{}\n{package}", source.to_ascii_lowercase())
}

/// The sources and suggestions; reads only the local file.
#[tauri::command]
pub(crate) fn sources_list() -> SourcesState {
    SourcesState {
        sources: pmca_app::sources::list(),
        suggestions: pmca_app::sources::suggestions(),
    }
}

/// Add a source after reading its file once.
#[tauri::command]
pub(crate) async fn sources_add(app: AppHandle, spec: String) -> Result<Source, CommandError> {
    in_background(move || {
        let localizer = app.state::<AppState>().localizer()?;
        Ok(pmca_app::sources::add(&localizer, &spec)?)
    })
    .await
}

/// Remove a source and the signers remembered for its apps.
#[tauri::command]
pub(crate) fn sources_remove(state: State<'_, AppState>, id: String) -> Result<(), CommandError> {
    Ok(pmca_app::sources::remove(&state.localizer()?, &id)?)
}

/// GitHub's description of one repository, its stars included.
#[tauri::command]
pub(crate) async fn source_repo(app: AppHandle, id: String) -> Result<SourceRepo, CommandError> {
    in_background(move || {
        let localizer = app.state::<AppState>().localizer()?;
        Ok(pmca_app::sources::repo(&localizer, &id)?)
    })
    .await
}

/// Search GitHub for source repositories; only ever on the user's request.
#[tauri::command]
pub(crate) async fn sources_search() -> Result<Vec<SourceRepo>, CommandError> {
    in_background(|| Ok(pmca_app::sources::search()?)).await
}

/// The plugins the user's sources offer.
#[tauri::command]
pub(crate) async fn list_community_plugins() -> Result<CommunityList, CommandError> {
    in_background(|| Ok(pmca_app::store::community_plugins())).await
}

/// Download a plugin from a source and say what it would do; the bytes wait
/// here for [`install_downloaded_plugin`].
#[tauri::command]
pub(crate) async fn download_community_plugin(
    app: AppHandle,
    source: String,
    id: String,
) -> Result<PluginInfo, CommandError> {
    in_background(move || {
        let state = app.state::<AppState>();
        let localizer = state.localizer()?;
        let (info, bytes) = pmca_app::store::download_community_plugin(
            &localizer,
            &source,
            &id,
            &mut progress(&app),
        )?;
        state
            .plugin_downloads
            .lock()
            .map_err(CommandError::internal)?
            .insert(download_key(&source, &id), bytes);
        Ok(info)
    })
    .await
}

/// Install a plugin fetched by [`download_community_plugin`], once the user
/// has agreed; it takes effect at the next start.
#[tauri::command]
pub(crate) fn install_downloaded_plugin(
    state: State<'_, AppState>,
    source: String,
    id: String,
) -> Result<PluginInfo, CommandError> {
    let localizer = state.localizer()?;
    let bytes = state
        .plugin_downloads
        .lock()
        .map_err(CommandError::internal)?
        .remove(&download_key(&source, &id))
        .ok_or_else(|| CommandError::internal(format!("{id} has not been downloaded")))?;
    Ok(pmca_app::plugins::install_bytes(&localizer, &bytes, &format!("{id}.pmcaplugin"))?)
}

/// Drop a downloaded plugin the user decided against.
#[tauri::command]
pub(crate) fn discard_plugin_download(
    state: State<'_, AppState>,
    source: String,
    id: String,
) -> Result<(), CommandError> {
    state
        .plugin_downloads
        .lock()
        .map_err(CommandError::internal)?
        .remove(&download_key(&source, &id));
    Ok(())
}
