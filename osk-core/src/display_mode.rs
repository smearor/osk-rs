//! Display mode for the on-screen keyboard.

use serde::{Deserialize, Serialize};

/// Display mode for the on-screen keyboard.
///
/// Controls how the keyboard is positioned on screen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DisplayMode {
    /// Full-width keyboard spanning the entire bottom of the screen
    Full,
    /// Split keyboard with two halves on left and right sides
    Split,
    /// Floating keyboard that can be dragged to any position
    Floating,
}
