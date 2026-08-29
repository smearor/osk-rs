//! Registry of available predefined layouts.
//!
//! Provides lookup of layout sections by [`XkbLayout`], enabling runtime
//! layout switching without restarting the application.

use crate::Azerty;
use crate::Dvorak;
use crate::LayoutDefinition;
use crate::LayoutError;
use crate::LayoutSections;
use crate::Qwerty;
use crate::Qwertz;
use crate::SizeVariantBuilder;
use osk_core::LayoutDef;
use osk_core::SizeVariant;
use osk_core::XkbLayout;
use osk_core::XkbVariant;

/// Registry of available predefined layouts.
///
/// Provides lookup of layout sections by [`XkbLayout`], enabling runtime
/// layout switching without restarting the application.
pub struct LayoutRegistry;

impl LayoutRegistry {
    /// Get the list of available predefined layouts.
    pub fn available_layouts() -> &'static [XkbLayout] {
        &[XkbLayout::De, XkbLayout::Us, XkbLayout::Fr]
    }

    /// Get the layout sections for a given layout and variant.
    ///
    /// Returns `None` if the layout is not recognized as a predefined layout.
    pub fn get_sections(layout: XkbLayout, variant: XkbVariant) -> Option<LayoutSections> {
        match layout {
            XkbLayout::Us if variant == XkbVariant::Dvorak => Some(Dvorak::sections()),
            XkbLayout::De => Some(Qwertz::sections()),
            XkbLayout::Us => Some(Qwerty::sections()),
            XkbLayout::Fr => Some(Azerty::sections()),
            _ => None,
        }
    }

    /// Build a layout definition for the given layout and size variant.
    ///
    /// # Errors
    ///
    /// Returns `LayoutError` if the layout is not recognized.
    pub fn build(layout: XkbLayout, size: SizeVariant) -> Result<LayoutDef, LayoutError> {
        Self::build_with_variant(layout, XkbVariant::Default, size)
    }

    /// Build a layout definition for the given layout, variant, and size.
    ///
    /// # Errors
    ///
    /// Returns `LayoutError` if the layout is not recognized.
    pub fn build_with_variant(layout: XkbLayout, variant: XkbVariant, size: SizeVariant) -> Result<LayoutDef, LayoutError> {
        let sections = Self::get_sections(layout.clone(), variant).ok_or_else(|| LayoutError::LayoutNotFound(layout.to_string()))?;
        Ok(SizeVariantBuilder::build(&sections, size))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_available_layouts() {
        let layouts = LayoutRegistry::available_layouts();
        assert!(layouts.contains(&XkbLayout::De));
        assert!(layouts.contains(&XkbLayout::Us));
        assert!(layouts.contains(&XkbLayout::Fr));
    }

    #[test]
    fn test_get_sections_de() {
        let sections = LayoutRegistry::get_sections(XkbLayout::De, XkbVariant::Default).unwrap();
        assert_eq!(sections.name, XkbLayout::De);
        assert!(!sections.main_block.is_empty());
    }

    #[test]
    fn test_get_sections_us_dvorak() {
        let sections = LayoutRegistry::get_sections(XkbLayout::Us, XkbVariant::Dvorak).unwrap();
        assert_eq!(sections.name, XkbLayout::Us);
    }

    #[test]
    fn test_get_sections_unknown() {
        assert!(LayoutRegistry::get_sections(XkbLayout::Custom("xx".to_string()), XkbVariant::Default).is_none());
    }

    #[test]
    fn test_build_us_compact() {
        let layout = LayoutRegistry::build(XkbLayout::Us, SizeVariant::Compact60).unwrap();
        assert_eq!(layout.name, XkbLayout::Us);
        assert_eq!(layout.size, SizeVariant::Compact60);
    }

    #[test]
    fn test_build_unknown_fails() {
        assert!(LayoutRegistry::build(XkbLayout::Custom("xx".to_string()), SizeVariant::Compact60).is_err());
    }

    #[test]
    fn test_build_with_variant_dvorak() {
        let layout = LayoutRegistry::build_with_variant(XkbLayout::Us, XkbVariant::Dvorak, SizeVariant::Tenkeyless80).unwrap();
        assert_eq!(layout.name, XkbLayout::Us);
        assert_eq!(layout.size, SizeVariant::Tenkeyless80);
    }
}
