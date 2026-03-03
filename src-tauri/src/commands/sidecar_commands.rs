use crate::domain::settings::{SidecarState, SidecarStatus};
use crate::error::ForgeError;

/// Query the current status of the sidecar process.
///
/// Returns a static `NotStarted` status for now. Will be wired to `SidecarManager`
/// via `AppState` in Sub-Phase 8 when the streaming pipeline is connected end-to-end.
#[tauri::command]
pub fn sidecar_status() -> Result<SidecarStatus, ForgeError> {
    Ok(SidecarStatus {
        state: SidecarState::NotStarted,
        pid: None,
        uptime_seconds: None,
        cli_detected: false,
        cli_version: None,
        error_message: None,
    })
}

/// Restart the sidecar process.
///
/// Returns a static `NotStarted` status for now. Will be wired to `SidecarManager`
/// via `AppState` in Sub-Phase 8 when the streaming pipeline is connected end-to-end.
#[tauri::command]
pub fn sidecar_restart() -> Result<SidecarStatus, ForgeError> {
    Ok(SidecarStatus {
        state: SidecarState::NotStarted,
        pid: None,
        uptime_seconds: None,
        cli_detected: false,
        cli_version: None,
        error_message: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sidecar_status_returns_not_started() {
        let status = sidecar_status().expect("should succeed");
        assert_eq!(status.state, SidecarState::NotStarted);
        assert!(status.pid.is_none());
        assert!(status.uptime_seconds.is_none());
        assert!(!status.cli_detected);
        assert!(status.cli_version.is_none());
        assert!(status.error_message.is_none());
    }

    #[test]
    fn sidecar_restart_returns_not_started() {
        let status = sidecar_restart().expect("should succeed");
        assert_eq!(status.state, SidecarState::NotStarted);
        assert!(status.pid.is_none());
    }
}
