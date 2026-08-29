//! Newtype for keyboard height as a percentage of screen height.

use serde::Deserialize;
use serde::Serialize;
use std::fmt;

/// Keyboard height as a percentage of screen height.
///
/// A value of `40.0` means the keyboard occupies 40% of the screen height.
/// Must be between 0.0 and 100.0.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct KeyboardHeight(pub f32);

impl KeyboardHeight {
    /// Create a new keyboard height from a raw `f32` percentage value.
    pub const fn new(value: f32) -> Self {
        Self(value)
    }

    /// Return the raw `f32` percentage value.
    pub const fn value(self) -> f32 {
        self.0
    }
}

impl Default for KeyboardHeight {
    fn default() -> Self {
        Self::new(40.0)
    }
}

impl fmt::Display for KeyboardHeight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}%", self.0)
    }
}

impl From<f32> for KeyboardHeight {
    fn from(value: f32) -> Self {
        Self(value)
    }
}

impl From<KeyboardHeight> for f32 {
    fn from(height: KeyboardHeight) -> Self {
        height.0
    }
}
