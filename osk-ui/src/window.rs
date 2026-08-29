//! OSK window that renders the keyboard as a GTK 4 layer-shell surface.

/// Stub for the OSK window that renders the keyboard as a GTK 4 layer-shell surface.
///
/// In Phase 1, this will create a `gtk4::ApplicationWindow` with
/// `gtk4-layer-shell` initialization and render the key grid.
pub struct OskWindow;

impl OskWindow {
    /// Create a new OSK window stub.
    pub fn new() -> Self {
        Self
    }
}

impl Default for OskWindow {
    fn default() -> Self {
        Self::new()
    }
}
