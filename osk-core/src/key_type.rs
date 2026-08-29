//! Semantic type of a key, determining its behavior when pressed.

use serde::Deserialize;
use serde::Serialize;

/// Semantic type of a key, determining its behavior when pressed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeyType {
    /// Regular character key (letters, numbers, symbols)
    Character,
    /// Modifier key (Shift, Ctrl, Alt, Super)
    Modifier,
    /// Special key (Esc, Enter, Tab, Backspace, Delete, arrows, F-keys)
    Special,
    /// Custom user-defined key
    Custom,
}
