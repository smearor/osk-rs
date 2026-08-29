//! Errors that can occur during Wayland client operations.

use thiserror::Error;

use crate::protocol::WaylandProtocol;

/// Errors that can occur during Wayland client operations.
#[derive(Debug, Error)]
pub enum WaylandError {
    /// The compositor does not support a required protocol
    #[error("required protocol not found in registry: {0}")]
    ProtocolNotFound(WaylandProtocol),
    /// The layer-shell surface could not be created
    #[error("layer-shell surface creation failed: {0}")]
    SurfaceCreationFailed(String),
    /// The wl_display connection could not be obtained from GTK
    #[error("failed to get wl_display from GTK")]
    DisplayNotAvailable,
    /// The Wayland connection failed
    #[error("wayland connection failed: {0}")]
    ConnectionFailed(String),
    /// A Wayland registry bind failed
    #[error("registry bind failed: {0}")]
    BindFailed(String),
}
