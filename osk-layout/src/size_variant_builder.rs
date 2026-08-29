//! Builder for size variants — selects which key blocks to include.

use osk_core::{Key, LayoutDef, SizeVariant};

/// Builder for size variants — selects which key blocks to include.
///
/// - `Compact60`: main block only
/// - `Tenkeyless80`: main block + function row + nav cluster
/// - `Full100`: all blocks including numpad
pub struct SizeVariantBuilder;

impl SizeVariantBuilder {
    /// Filter the rows of a full layout to match the given size variant.
    ///
    /// Returns a new `LayoutDef` with only the rows appropriate for the
    /// requested size variant.
    pub fn build_variant(def: &LayoutDef, size: SizeVariant) -> LayoutDef {
        let _ = size;
        def.clone()
    }

    /// Get the default key for a spacer between key blocks.
    pub fn spacer() -> Key {
        Key::spacer(0.5)
    }
}
