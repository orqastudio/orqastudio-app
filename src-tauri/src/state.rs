use std::sync::Mutex;

use rusqlite::Connection;

use crate::search::SearchEngine;
use crate::sidecar::manager::SidecarManager;

/// Application state managed by Tauri.
///
/// The `Mutex<Connection>` is safe for single-writer SQLite with WAL mode.
/// The `SidecarManager` uses interior mutability via its own `Mutex` fields.
/// The `SearchEngine` is lazily initialized when a project is first indexed.
/// Tauri manages this as shared state across all commands.
pub struct AppState {
    pub db: Mutex<Connection>,
    pub sidecar: SidecarManager,
    pub search: Mutex<Option<SearchEngine>>,
}
