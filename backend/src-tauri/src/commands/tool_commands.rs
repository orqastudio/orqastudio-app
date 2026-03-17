//! Legacy tool commands — thin wrappers forwarding to cli_tool_commands.
//!
//! These are no longer registered as Tauri commands (removed from lib.rs invoke_handler).
//! Kept only so the module compiles while references are cleaned up.

use crate::cli_tools::runner::{RegisteredCliTool, CliToolResult, CliToolStatus};
use crate::error::OrqaError;
use crate::state::AppState;

/// @deprecated — use `get_registered_cli_tools` instead.
#[tauri::command]
pub fn get_registered_tools(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<RegisteredCliTool>, OrqaError> {
    super::cli_tool_commands::get_registered_cli_tools(state)
}

/// @deprecated — use `run_cli_tool` instead.
#[tauri::command]
pub fn run_tool(
    plugin_name: String,
    tool_key: String,
    state: tauri::State<'_, AppState>,
) -> Result<CliToolResult, OrqaError> {
    super::cli_tool_commands::run_cli_tool(plugin_name, tool_key, state)
}

/// @deprecated — use `cli_tool_status` instead.
#[tauri::command]
pub fn tool_status(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<CliToolStatus>, OrqaError> {
    super::cli_tool_commands::cli_tool_status(state)
}
