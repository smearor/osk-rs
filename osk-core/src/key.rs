//! A single key definition in the keyboard grid.

use serde::{Deserialize, Serialize};

use crate::{KeyShape, KeyType};

/// A single key definition in the keyboard grid.
///
/// Each key has a Linux evdev keycode, a display label, a geometric shape,
/// and a semantic type that determines how it behaves when pressed.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Key {
    /// Linux evdev keycode for this key
    pub keycode: u32,
    /// Display label shown on the key cap
    pub label: String,
    /// Geometric shape of the key (rectangular or L-shaped)
    pub shape: KeyShape,
    /// Semantic type of the key (alpha, modifier, special, custom)
    pub key_type: KeyType,
}
