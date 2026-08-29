//! Errors that can occur during Wayland client operations.

use thiserror::Error;

/// Errors that can occur during Wayland client operations.
#[derive(Debug, Error)]
pub enum WaylandError {
    /// The compositor does not support a required protocol
    #[error("required protocol not found in registry: {0}")]
    ProtocolNotFound(String),
    /// The layer-shell surface could not be created
    #[error("layer-shell surface creation failed: {0}")]
    SurfaceCreationFailed(String),
    /// The wl_display connection could not be obtained from GTK
    #[error("failed to get wl_display from GTK")]
    DisplayNotAvailable,
}
