//! Errors that can occur during IPC operations.

use thiserror::Error;

/// Errors that can occur during IPC operations.
#[derive(Debug, Error)]
pub enum IpcError {
    /// The D-Bus connection could not be established
    #[error("D-Bus connection failed: {0}")]
    DbusConnectionFailed(String),
    /// The Unix socket could not be created
    #[error("socket creation failed: {0}")]
    SocketCreationFailed(String),
    /// A command was received that could not be parsed
    #[error("invalid command: {0}")]
    InvalidCommand(String),
}
