//! Trait for building keyboard layouts from XKB definitions.

use osk_core::{LayoutDef, SizeVariant};

use crate::LayoutError;

/// Trait for building keyboard layouts from XKB definitions.
///
/// Implementations parse XKB keymap data and produce a `LayoutDef`
/// containing the visual key grid for a specific size variant.
pub trait LayoutBuilder {
    /// Build a layout definition for the given layout name, variant, and size.
    ///
    /// # Errors
    ///
    /// Returns `LayoutError` if the layout or variant is not found, or if
    /// the keymap cannot be compiled.
    fn build(&self, layout: &str, variant: Option<&str>, size: SizeVariant) -> Result<LayoutDef, LayoutError>;
}
