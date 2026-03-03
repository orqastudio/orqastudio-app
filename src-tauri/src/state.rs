use std::sync::Mutex;

use rusqlite::Connection;

/// Application state managed by Tauri.
///
/// The `Mutex<Connection>` is safe for single-writer SQLite with WAL mode.
/// Tauri manages this as shared state across all commands.
pub struct AppState {
    pub db: Mutex<Connection>,
}
