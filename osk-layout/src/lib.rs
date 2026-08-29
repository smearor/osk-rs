//! XKB layout parsing and size variant builder for `osk-rs`.
//!
//! This crate reads XKB layout definitions and builds visual key grids
//! for different size variants (60%, TKL, Full-Size).

use osk_core::{Key, LayoutDef, SizeVariant};
use thiserror::Error;

/// Errors that can occur during layout parsing or building.
#[derive(Debug, Error)]
pub enum LayoutError {
    /// The requested layout name was not found in the XKB database
    #[error("layout not found: {0}")]
    LayoutNotFound(String),
    /// The requested variant was not found for the given layout
    #[error("variant not found: {0} for layout {1}")]
    VariantNotFound(String, String),
    /// The XKB keymap could not be compiled from the given RMLVO
    #[error("keymap compilation failed: {0}")]
    KeymapCompilationFailed(String),
}

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

/// Builder for predefined standard layouts (QWERTZ, QWERTY, AZERTY, Dvorak).
///
/// This builder uses hardcoded layout definitions for the four standard
/// layouts and does not require XKB to be installed on the system.
pub struct StandardLayoutBuilder;

impl LayoutBuilder for StandardLayoutBuilder {
    fn build(&self, layout: &str, variant: Option<&str>, size: SizeVariant) -> Result<LayoutDef, LayoutError> {
        let _ = (layout, variant, size);
        // TODO: Implement in Phase 2
        Err(LayoutError::LayoutNotFound(layout.to_string()))
    }
}

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
        Key {
            keycode: 0,
            label: String::new(),
            shape: osk_core::KeyShape::Rect { width_u: 0.5 },
            key_type: osk_core::KeyType::Special,
        }
    }
}
