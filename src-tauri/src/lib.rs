pub mod commands;
pub mod db;
pub mod domain;
pub mod error;
pub mod repo;
pub mod sidecar;
pub mod state;

use tauri::Manager;

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_dir = app
                .path()
                .app_data_dir()
                .map_err(|e| format!("failed to resolve app data dir: {e}"))?;
            std::fs::create_dir_all(&app_dir)
                .map_err(|e| format!("failed to create app data dir: {e}"))?;

            let db_path = app_dir.join("forge.db");
            let db_path_str = db_path
                .to_str()
                .ok_or_else(|| "app data path is not valid UTF-8".to_string())?;

            let conn = db::init_db(db_path_str)
                .map_err(|e| format!("failed to initialize database: {e}"))?;

            app.manage(state::AppState {
                db: std::sync::Mutex::new(conn),
            });

            Ok(())
        })
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .invoke_handler(tauri::generate_handler![
            commands::sidecar_commands::sidecar_status,
            commands::sidecar_commands::sidecar_restart,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
