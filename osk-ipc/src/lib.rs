//! D-Bus session bus service, Unix socket server, and signal handling for `osk-rs`.
//!
//! This crate provides IPC interfaces for controlling the OSK at runtime:
//! - D-Bus session bus service (`org.example.OSK`)
//! - Unix domain socket server for CLI commands
//! - POSIX signal handling (SIGUSR1 for toggle, SIGTERM for shutdown)

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

/// Stub for the OSK IPC service.
///
/// In Phase 1, this will provide a D-Bus interface and Unix socket server
/// for runtime control of the keyboard (show, hide, toggle, set layout, etc.).
pub struct OskIpc;

impl OskIpc {
    /// Create a new IPC service stub.
    pub fn new() -> Self {
        Self
    }
}

impl Default for OskIpc {
    fn default() -> Self {
        Self::new()
    }
}
