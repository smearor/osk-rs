//! Builder for predefined standard layouts (QWERTZ, QWERTY, AZERTY, Dvorak).

use crate::LayoutBuilder;
use crate::LayoutError;
use osk_core::LayoutDef;
use osk_core::SizeVariant;

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
