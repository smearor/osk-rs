//! Pressed or released state of a key.

use serde::Deserialize;
use serde::Serialize;

/// Pressed or released state of a key.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeyState {
    /// Key is pressed down
    Pressed,
    /// Key is released
    Released,
}
