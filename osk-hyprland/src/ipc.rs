//! Hyprland IPC client for communicating with the compositor.

use crate::HyprlandError;

/// Hyprland IPC client for communicating with the compositor.
///
/// Connects to the Hyprland IPC sockets to query the active keyboard layout,
/// monitor configuration, and listen for workspace and config change events.
pub struct HyprlandIpc {
    /// Path to the command socket (usually `~/.hypr/.socket.sock`)
    pub command_socket: String,
    /// Path to the event socket (usually `~/.hypr/.socket2.sock`)
    pub event_socket: String,
}

impl HyprlandIpc {
    /// Create a new Hyprland IPC client with default socket paths.
    pub fn new() -> Self {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
        Self {
            command_socket: format!("{home}/.hypr/.socket.sock"),
            event_socket: format!("{home}/.hypr/.socket2.sock"),
        }
    }

    /// Check if Hyprland IPC is available (socket exists).
    pub fn is_available(&self) -> bool {
        std::path::Path::new(&self.command_socket).exists()
    }

    /// Get the active keyboard layout from Hyprland.
    ///
    /// # Errors
    ///
    /// Returns `HyprlandError` if the IPC command fails or the response
    /// cannot be parsed.
    pub async fn get_active_layout(&self) -> Result<String, HyprlandError> {
        let _ = &self.command_socket;
        // TODO: Implement in Phase 3
        Err(HyprlandError::SocketNotFound)
    }
}

impl Default for HyprlandIpc {
    fn default() -> Self {
        Self::new()
    }
}
