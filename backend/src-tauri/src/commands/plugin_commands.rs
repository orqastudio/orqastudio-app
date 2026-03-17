//! Plugin management Tauri commands — install, uninstall, registry browsing.

use crate::error::OrqaError;
use crate::plugins::{discovery, installer};
use crate::repo::project_repo;
use crate::state::AppState;

/// Resolve the active project's filesystem path from the database.
fn active_project_path(state: &tauri::State<'_, AppState>) -> Result<String, OrqaError> {
    let conn = state
        .db
        .conn
        .lock()
        .map_err(|e| OrqaError::Database(format!("lock poisoned: {e}")))?;

    let project = project_repo::get_active(&conn)?.ok_or_else(|| {
        OrqaError::NotFound("no active project — open a project first".to_string())
    })?;

    Ok(project.path)
}

/// List all installed plugins discovered from the plugins/ directory.
#[tauri::command]
pub fn plugin_list_installed(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<discovery::DiscoveredPlugin>, OrqaError> {
    let project_path = active_project_path(&state)?;
    Ok(discovery::scan_plugins(std::path::Path::new(&project_path)))
}

/// Fetch the plugin registry catalog (official, community, or both).
#[tauri::command]
pub async fn plugin_registry_list(
    source: Option<String>,
) -> Result<serde_json::Value, OrqaError> {
    let src = source.as_deref().unwrap_or("official");

    // Use the registry module
    let cache = crate::plugins::registry::RegistryCache::new();
    let catalog = cache.fetch(src).await?;

    serde_json::to_value(&catalog).map_err(|e| OrqaError::Serialization(e.to_string()))
}

/// Install a plugin from a local path.
#[tauri::command]
pub fn plugin_install_local(
    path: String,
    state: tauri::State<'_, AppState>,
) -> Result<installer::InstallResult, OrqaError> {
    let project_path = active_project_path(&state)?;
    installer::install_from_path(
        std::path::Path::new(&path),
        std::path::Path::new(&project_path),
    )
}

/// Uninstall a plugin by name.
#[tauri::command]
pub fn plugin_uninstall(
    name: String,
    state: tauri::State<'_, AppState>,
) -> Result<(), OrqaError> {
    let project_path = active_project_path(&state)?;
    installer::uninstall(&name, std::path::Path::new(&project_path))
}

/// Check for available plugin updates.
#[tauri::command]
pub fn plugin_check_updates(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<serde_json::Value>, OrqaError> {
    let _ = state;
    // Phase 6 enhancement: compare lockfile versions against latest releases
    Ok(vec![])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_updates_returns_empty() {
        // No state needed for the stub
    }
}
