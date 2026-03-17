//! Plugin discovery — scan the `plugins/` directory for installed plugins.

use serde::Serialize;
use std::path::Path;

use super::manifest::read_manifest;

/// A discovered plugin from scanning the plugins/ directory.
#[derive(Debug, Clone, Serialize)]
pub struct DiscoveredPlugin {
    pub name: String,
    pub version: String,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub path: String,
    pub source: String, // "installed" or "local"
}

/// Scan the `plugins/` directory at the project root for plugin manifests.
pub fn scan_plugins(project_root: &Path) -> Vec<DiscoveredPlugin> {
    let plugins_dir = project_root.join("plugins");

    if !plugins_dir.is_dir() {
        return vec![];
    }

    let mut discovered = Vec::new();

    let entries = match std::fs::read_dir(&plugins_dir) {
        Ok(entries) => entries,
        Err(_) => return vec![],
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }

        // Skip hidden directories
        if entry
            .file_name()
            .to_str()
            .is_some_and(|n| n.starts_with('.'))
        {
            continue;
        }

        match read_manifest(&path) {
            Ok(manifest) => {
                discovered.push(DiscoveredPlugin {
                    name: manifest.name.clone(),
                    version: manifest.version.clone(),
                    display_name: manifest.display_name.clone(),
                    description: manifest.description.clone(),
                    path: path.to_string_lossy().to_string(),
                    source: "local".to_string(),
                });
            }
            Err(_) => {
                // Not a valid plugin directory — skip
            }
        }
    }

    discovered
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn scan_empty_project() {
        let plugins = scan_plugins(&PathBuf::from("/nonexistent"));
        assert!(plugins.is_empty());
    }
}
