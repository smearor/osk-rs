//! GTK 4 rendering and touch hit-testing for `osk-rs`.
//!
//! This crate renders the keyboard as a GTK 4 widget grid and handles
//! touch events with custom hit-testing for non-rectangular key shapes
//! (e.g. ISO L-shaped Enter key).

use thiserror::Error;

/// Errors that can occur during UI operations.
#[derive(Debug, Error)]
pub enum UiError {
    /// The GTK theme could not be loaded
    #[error("theme load failed: {0}")]
    ThemeLoadFailed(String),
    /// A widget rendering error occurred
    #[error("rendering error: {0}")]
    RenderingError(String),
}

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
