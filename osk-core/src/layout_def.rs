//! Layout definition for a complete keyboard.

use crate::KeyGrid;
use crate::SizeVariant;
use crate::XkbLayout;
use crate::XkbVariant;
use serde::Deserialize;
use serde::Serialize;

/// Layout definition for a complete keyboard.
///
/// Contains the layout name, optional variant, size variant, and the full
/// grid of keys organized by rows.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct LayoutDef {
    /// XKB layout name
    pub name: XkbLayout,
    /// XKB layout variant (e.g. `Nodeadkeys`, `Intl`, `Default`)
    pub variant: XkbVariant,
    /// Size variant of this layout
    pub size: SizeVariant,
    /// Rows of keys, from top (function row) to bottom (modifier row)
    pub rows: KeyGrid,
}
