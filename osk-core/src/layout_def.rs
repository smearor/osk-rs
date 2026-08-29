//! Layout definition for a complete keyboard.

use serde::{Deserialize, Serialize};

use crate::{Key, SizeVariant, XkbVariant};

/// Layout definition for a complete keyboard.
///
/// Contains the layout name, optional variant, size variant, and the full
/// grid of keys organized by rows.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct LayoutDef {
    /// XKB layout name (e.g. "de", "us", "fr")
    pub name: String,
    /// XKB layout variant (e.g. `Nodeadkeys`, `Intl`, `Default`)
    pub variant: XkbVariant,
    /// Size variant of this layout
    pub size: SizeVariant,
    /// Rows of keys, from top (function row) to bottom (modifier row)
    pub rows: Vec<Vec<Key>>,
}
