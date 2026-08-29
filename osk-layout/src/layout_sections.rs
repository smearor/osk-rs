//! Layout sections for building size variants.
//!
//! A keyboard layout is divided into logical sections that can be combined
//! differently depending on the size variant (60%, TKL, Full-Size).

use osk_core::KeyGrid;
use osk_core::KeyRow;
use osk_core::XkbLayout;
use osk_core::XkbVariant;

/// Layout sections for building size variants.
///
/// Each section corresponds to a physical region of a full-size keyboard.
/// The [`SizeVariantBuilder`](crate::SizeVariantBuilder) combines these
/// sections into a [`LayoutDef`](osk_core::LayoutDef) based on the
/// requested size variant.
#[derive(Debug, Clone, PartialEq)]
pub struct LayoutSections {
    /// XKB layout name
    pub name: XkbLayout,
    /// XKB layout variant
    pub variant: XkbVariant,
    /// Function row (Esc, F1–F12) — 1 row
    pub function_row: KeyRow,
    /// Main block (alpha keys, modifiers, space) — 5 rows
    pub main_block: KeyGrid,
    /// Nav cluster aligned to function_row + main_block rows — 6 rows.
    ///
    /// Each row contains the nav keys for that row, preceded by a spacer
    /// for separation from the main block. Rows with no nav keys contain
    /// only a spacer to maintain alignment.
    pub nav_cluster: KeyGrid,
    /// Numpad aligned to main_block rows — 5 rows.
    ///
    /// Each row contains the numpad keys for that row, preceded by a spacer
    /// for separation from the nav cluster.
    pub numpad: KeyGrid,
}

impl LayoutSections {
    /// Create a new `LayoutSections` with the given layout and variant.
    pub fn new(name: XkbLayout, variant: XkbVariant) -> Self {
        Self {
            name,
            variant,
            function_row: KeyRow::new(),
            main_block: KeyGrid::new(),
            nav_cluster: KeyGrid::new(),
            numpad: KeyGrid::new(),
        }
    }
}
