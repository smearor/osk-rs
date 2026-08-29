//! Wayland client for `osk-rs` — layer-shell surface, registry bind, seat handling.
//!
//! This crate manages the Wayland connection by reusing GTK 4's native
//! `wl_display` via `gdk4-wayland` FFI. No separate event queue is created.

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

/// Stub for the Wayland client managing layer-shell surface and protocol bindings.
///
/// In Phase 1, this will bind to `zwlr_layer_shell_v1`,
/// `zwp_virtual_keyboard_manager_v1`, and `zwp_input_method_v2` using
/// GTK's native `wl_display` pointer.
pub struct WaylandClient;

impl WaylandClient {
    /// Create a new Wayland client stub.
    pub fn new() -> Self {
        Self
    }
}

impl Default for WaylandClient {
    fn default() -> Self {
        Self::new()
    }
}
