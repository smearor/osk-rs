//! Trait for building keyboard layouts from XKB definitions.

use crate::LayoutError;
use osk_core::LayoutDef;
use osk_core::SizeVariant;
use osk_core::XkbLayout;
use osk_core::XkbVariant;

/// Trait for building keyboard layouts from XKB definitions.
///
/// Implementations parse XKB keymap data and produce a `LayoutDef`
/// containing the visual key grid for a specific size variant.
pub trait LayoutBuilder {
    /// Build a layout definition for the given layout, variant, and size.
    ///
    /// # Errors
    ///
    /// Returns `LayoutError` if the layout or variant is not found, or if
    /// the keymap cannot be compiled.
    fn build(&self, layout: XkbLayout, variant: XkbVariant, size: SizeVariant) -> Result<LayoutDef, LayoutError>;
}
