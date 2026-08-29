//! Newtype for the key scale factor.

use serde::Deserialize;
use serde::Serialize;
use std::fmt;

/// Key scale factor controlling the size of keyboard keys.
///
/// A value of `1.0` represents 100% (default size).
/// Values greater than `1.0` enlarge keys, values less than `1.0` shrink them.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct KeyScale(pub f32);

impl KeyScale {
    /// Create a new key scale from a raw `f32` value.
    pub const fn new(value: f32) -> Self {
        Self(value)
    }

    /// Return the raw `f32` scale value.
    pub const fn value(self) -> f32 {
        self.0
    }
}

impl Default for KeyScale {
    fn default() -> Self {
        Self::new(1.0)
    }
}

impl fmt::Display for KeyScale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<f32> for KeyScale {
    fn from(value: f32) -> Self {
        Self(value)
    }
}

impl From<KeyScale> for f32 {
    fn from(scale: KeyScale) -> Self {
        scale.0
    }
}
