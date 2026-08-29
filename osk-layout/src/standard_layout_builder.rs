//! Builder for predefined standard layouts (QWERTZ, QWERTY, AZERTY, Dvorak).

use crate::LayoutBuilder;
use crate::LayoutDefinition;
use crate::LayoutError;
use crate::LayoutSections;
use crate::SizeVariantBuilder;
use crate::Azerty;
use crate::Dvorak;
use crate::Qwerty;
use crate::Qwertz;
use osk_core::LayoutDef;
use osk_core::SizeVariant;
use osk_core::XkbLayout;
use osk_core::XkbVariant;

/// Builder for predefined standard layouts (QWERTZ, QWERTY, AZERTY, Dvorak).
///
/// This builder uses hardcoded layout definitions for the four standard
/// layouts and does not require XKB to be installed on the system.
/// It combines the layout sections with the requested size variant using
/// [`SizeVariantBuilder`].
pub struct StandardLayoutBuilder;

impl LayoutBuilder for StandardLayoutBuilder {
    fn build(&self, layout: XkbLayout, variant: XkbVariant, size: SizeVariant) -> Result<LayoutDef, LayoutError> {
        let sections = Self::get_sections(layout, variant);
        Ok(SizeVariantBuilder::build(&sections, size))
    }
}

impl StandardLayoutBuilder {
    /// Get the layout sections for a given layout and variant.
    pub fn get_sections(layout: XkbLayout, variant: XkbVariant) -> LayoutSections {
        match layout {
            XkbLayout::Us if variant == XkbVariant::Dvorak => Dvorak::sections(),
            XkbLayout::De => Qwertz::sections(),
            XkbLayout::Us => Qwerty::sections(),
            XkbLayout::Fr => Azerty::sections(),
            _ => Qwertz::sections(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_qwerty_tkl() {
        let builder = StandardLayoutBuilder;
        let layout = builder.build(XkbLayout::Us, XkbVariant::Default, SizeVariant::Tenkeyless80).unwrap();
        assert_eq!(layout.name, XkbLayout::Us);
        assert_eq!(layout.size, SizeVariant::Tenkeyless80);
    }

    #[test]
    fn test_build_qwertz_tkl() {
        let builder = StandardLayoutBuilder;
        let layout = builder.build(XkbLayout::De, XkbVariant::Default, SizeVariant::Tenkeyless80).unwrap();
        assert_eq!(layout.name, XkbLayout::De);
        assert_eq!(layout.size, SizeVariant::Tenkeyless80);
        assert_eq!(layout.rows.len(), 6);
    }

    #[test]
    fn test_build_azerty_tkl() {
        let builder = StandardLayoutBuilder;
        let layout = builder.build(XkbLayout::Fr, XkbVariant::Default, SizeVariant::Tenkeyless80).unwrap();
        assert_eq!(layout.name, XkbLayout::Fr);
    }

    #[test]
    fn test_build_dvorak_tkl() {
        let builder = StandardLayoutBuilder;
        let layout = builder.build(XkbLayout::Us, XkbVariant::Dvorak, SizeVariant::Tenkeyless80).unwrap();
        assert_eq!(layout.name, XkbLayout::Us);
    }

    #[test]
    fn test_build_compact60() {
        let builder = StandardLayoutBuilder;
        let layout = builder.build(XkbLayout::De, XkbVariant::Default, SizeVariant::Compact60).unwrap();
        assert_eq!(layout.rows.len(), 5);
    }

    #[test]
    fn test_build_full100() {
        let builder = StandardLayoutBuilder;
        let layout = builder.build(XkbLayout::De, XkbVariant::Default, SizeVariant::Full100).unwrap();
        assert_eq!(layout.rows.len(), 6);
    }
}
