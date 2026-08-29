//! Wayland client managing layer-shell surface and protocol bindings.

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
