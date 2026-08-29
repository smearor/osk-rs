//! Errors that can occur during Hyprland IPC operations.

use thiserror::Error;

/// Errors that can occur during Hyprland IPC operations.
#[derive(Debug, Error)]
pub enum HyprlandError {
    /// The Hyprland IPC socket was not found
    #[error("Hyprland IPC socket not found")]
    SocketNotFound,
    /// A command sent to Hyprland failed
    #[error("Hyprland command failed: {0}")]
    CommandFailed(String),
    /// The JSON response from Hyprland could not be parsed
    #[error("JSON parse error: {0}")]
    JsonParseError(#[from] serde_json::Error),
    /// The I/O operation failed
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
}
