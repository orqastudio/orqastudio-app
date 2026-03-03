use crate::domain::project_scanner::{self, ProjectScanResult};
use crate::domain::project_settings::{self, ProjectSettings};
use crate::error::ForgeError;

/// Read project settings from the `.forge/project.json` file.
///
/// Returns `None` if the settings file does not exist yet.
#[tauri::command]
pub fn project_settings_read(
    path: String,
) -> Result<Option<ProjectSettings>, ForgeError> {
    project_settings::read_settings(&path)
}

/// Write project settings to the `.forge/project.json` file.
///
/// Creates the `.forge/` directory if it does not exist.
/// Returns the written settings for confirmation.
#[tauri::command]
pub fn project_settings_write(
    path: String,
    settings: ProjectSettings,
) -> Result<ProjectSettings, ForgeError> {
    project_settings::write_settings(&path, &settings)?;
    Ok(settings)
}

/// Scan a project directory for language, framework, and governance info.
///
/// Uses the provided excluded paths or falls back to standard defaults.
#[tauri::command]
pub fn project_scan(
    path: String,
    excluded_paths: Option<Vec<String>>,
) -> Result<ProjectScanResult, ForgeError> {
    let defaults = vec![
        "node_modules".to_string(),
        ".git".to_string(),
        "target".to_string(),
        "dist".to_string(),
        "build".to_string(),
    ];
    let paths = excluded_paths.unwrap_or(defaults);
    project_scanner::scan_project(&path, &paths)
}
