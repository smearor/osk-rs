//! XKB layout parsing via `xkbcommon`.
//!
//! Provides [`XkbLayoutParser`] for compiling XKB keymaps from RMLVO
//! (Rules, Model, Layout, Variant, Options) and extracting the
//! keycode→keysym mapping needed by the virtual keyboard.

use crate::LayoutError;
use osk_core::KeyShape;
use osk_core::XkbLayout;
use osk_core::XkbVariant;
use std::collections::HashMap;
use xkbcommon::xkb;

/// A parsed XKB keymap containing the keycode→keysym mapping and metadata.
///
/// This struct holds the essential data extracted from an XKB keymap:
/// the keycode-to-keysym mapping for each shift level, the keymap string
/// that can be published to the Wayland virtual keyboard, and layout
/// metadata such as key shapes.
#[derive(Debug, Clone)]
pub struct XkbKeymap {
    /// XKB layout name, e.g. "de", "us", "fr"
    pub layout_name: String,
    /// Optional XKB variant, e.g. "nodeadkeys", "intl"
    pub variant: Option<String>,
    /// The keymap as a string in XKB text format, suitable for
    /// publishing to `zwp_virtual_keyboard_v1`.
    pub keymap_string: String,
    /// Mapping from evdev keycode to the keysym at shift level 0
    /// (unshifted) and shift level 1 (shifted).
    pub keycodes: HashMap<u32, KeySymEntry>,
    /// Whether this layout uses ISO L-shaped Enter (true) or ANSI
    /// wide Enter (false).
    pub iso_enter: bool,
}

/// A single keycode's keysym data across shift levels.
#[derive(Debug, Clone, PartialEq)]
pub struct KeySymEntry {
    /// Keysym at shift level 0 (unshifted)
    pub base_sym: u32,
    /// Keysym at shift level 1 (shifted)
    pub shift_sym: u32,
    /// Human-readable label for the base symbol
    pub base_label: String,
    /// Human-readable label for the shifted symbol
    pub shift_label: String,
}

/// Parser for XKB layout definitions via `xkbcommon`.
///
/// Wraps an `xkbcommon` context and compiles keymaps from RMLVO
/// specifications. The context is reused across multiple `parse()`
/// calls for efficiency.
pub struct XkbLayoutParser {
    context: xkb::Context,
}

impl XkbLayoutParser {
    /// Create a new XKB layout parser with default context settings.
    ///
    /// # Errors
    ///
    /// Returns `LayoutError` if the xkbcommon context cannot be created.
    pub fn new() -> Result<Self, LayoutError> {
        let context = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);
        Ok(Self { context })
    }

    /// Parse a layout and return the compiled keymap data.
    ///
    /// Uses the `evdev` rules with the `pc104` model by default.
    ///
    /// # Arguments
    ///
    /// * `layout` - XKB layout name (e.g. "de", "us", "fr")
    /// * `variant` - Optional XKB variant (e.g. "nodeadkeys", "dvorak")
    ///
    /// # Errors
    ///
    /// Returns `LayoutError::KeymapCompilationFailed` if the keymap
    /// cannot be compiled from the given RMLVO.
    pub fn parse(&self, layout: &XkbLayout, variant: &XkbVariant) -> Result<XkbKeymap, LayoutError> {
        let layout_str = layout.as_str();
        let variant_str = variant.as_str().unwrap_or("");

        let keymap = xkb::Keymap::new_from_names(&self.context, "evdev", "pc104", layout_str, variant_str, None, xkb::KEYMAP_COMPILE_NO_FLAGS);

        let keymap = keymap.ok_or_else(|| {
            LayoutError::KeymapCompilationFailed(format!("xkbcommon failed to compile keymap for layout='{layout_str}', variant='{variant_str}'"))
        })?;

        let keymap_string = keymap.get_as_string(xkb::KEYMAP_FORMAT_TEXT_V1);

        let mut keycodes = HashMap::new();
        let min_kc = keymap.min_keycode().raw();
        let max_kc = keymap.max_keycode().raw();

        let layout_idx = keymap.layout_get_index(layout_str);
        let active_layout = if layout_idx == xkb::LAYOUT_INVALID { 0 } else { layout_idx };

        for kc_raw in min_kc..=max_kc {
            let kc = xkb::Keycode::new(kc_raw);
            let num_levels = keymap.num_levels_for_key(kc, active_layout);
            if num_levels == 0 {
                continue;
            }

            let base_syms = keymap.key_get_syms_by_level(kc, active_layout, 0);
            let shift_syms = if num_levels > 1 {
                keymap.key_get_syms_by_level(kc, active_layout, 1)
            } else {
                &[]
            };

            let base_sym = base_syms.first().map(|s| s.raw()).unwrap_or(0);
            let shift_sym = shift_syms.first().map(|s| s.raw()).unwrap_or(0);

            if base_sym == 0 && shift_sym == 0 {
                continue;
            }

            let base_label = sym_to_label(base_sym);
            let shift_label = sym_to_label(shift_sym);

            keycodes.insert(
                kc_raw,
                KeySymEntry {
                    base_sym,
                    shift_sym,
                    base_label,
                    shift_label,
                },
            );
        }

        let iso_enter = is_iso_layout(layout);

        Ok(XkbKeymap {
            layout_name: layout_str.to_string(),
            variant: variant.as_str().map(|s| s.to_string()),
            keymap_string,
            keycodes,
            iso_enter,
        })
    }

    /// Return the [`KeyShape`] for the Enter key based on the layout.
    ///
    /// ISO layouts (de, fr, uk) use an L-shaped Enter; ANSI layouts (us)
    /// use a wide rectangular Enter.
    pub fn enter_shape(layout: &XkbLayout) -> KeyShape {
        if is_iso_layout(layout) {
            KeyShape::LShape {
                top_width_u: 1.0,
                bottom_width_u: 1.5,
            }
        } else {
            KeyShape::Rect { width_u: 2.0 }
        }
    }
}

impl Default for XkbLayoutParser {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| Self {
            context: xkb::Context::new(xkb::CONTEXT_NO_FLAGS),
        })
    }
}

/// Determine whether a layout uses ISO L-shaped Enter.
///
/// ISO layouts typically include German (de), French (fr), British (gb),
/// and other European layouts. ANSI layouts include US (us).
fn is_iso_layout(layout: &XkbLayout) -> bool {
    match layout {
        XkbLayout::Us => false,
        XkbLayout::De
        | XkbLayout::Fr
        | XkbLayout::It
        | XkbLayout::Es
        | XkbLayout::Gb
        | XkbLayout::Ru
        | XkbLayout::No
        | XkbLayout::Se
        | XkbLayout::Dk
        | XkbLayout::Fi
        | XkbLayout::Nl
        | XkbLayout::Be
        | XkbLayout::Ch
        | XkbLayout::Pl
        | XkbLayout::Tr
        | XkbLayout::Cz => true,
        XkbLayout::Custom(_) => true,
    }
}

/// Convert an XKB keysym to a human-readable label.
///
/// This handles the most common keysyms. For unknown keysyms, the
/// raw hex value is returned.
fn sym_to_label(sym: u32) -> String {
    if sym == 0 {
        return String::new();
    }

    // Try to get the keysym name from xkbcommon
    let name = xkb::keysym_get_name(sym.into());
    if !name.is_empty() {
        // For single-character keysyms, return the character itself
        if let Some(utf8) = keysym_to_utf8(sym)
            && !utf8.is_empty()
        {
            return utf8;
        }
        // Otherwise return the keysym name
        return name;
    }

    format!("0x{sym:04x}")
}

/// Convert an XKB keysym to its UTF-8 representation.
fn keysym_to_utf8(sym: u32) -> Option<String> {
    let s = xkb::keysym_to_utf8(sym.into());
    if s.is_empty() { None } else { Some(s) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_creation() {
        let parser = XkbLayoutParser::new();
        assert!(parser.is_ok());
    }

    #[test]
    fn test_parse_us_layout() {
        let parser = XkbLayoutParser::new().unwrap();
        let keymap = parser.parse(&XkbLayout::Us, &XkbVariant::Default);
        assert!(keymap.is_ok(), "Failed to parse US layout");
        let keymap = keymap.unwrap();
        assert_eq!(keymap.layout_name, "us");
        assert!(!keymap.keymap_string.is_empty());
        assert!(!keymap.iso_enter, "US layout should have ANSI Enter");
    }

    #[test]
    fn test_parse_de_layout() {
        let parser = XkbLayoutParser::new().unwrap();
        let keymap = parser.parse(&XkbLayout::De, &XkbVariant::Default);
        assert!(keymap.is_ok(), "Failed to parse DE layout");
        let keymap = keymap.unwrap();
        assert_eq!(keymap.layout_name, "de");
        assert!(!keymap.keymap_string.is_empty());
        assert!(keymap.iso_enter, "DE layout should have ISO Enter");
    }

    #[test]
    fn test_parse_fr_layout() {
        let parser = XkbLayoutParser::new().unwrap();
        let keymap = parser.parse(&XkbLayout::Fr, &XkbVariant::Default);
        assert!(keymap.is_ok(), "Failed to parse FR layout");
        let keymap = keymap.unwrap();
        assert_eq!(keymap.layout_name, "fr");
        assert!(keymap.iso_enter, "FR layout should have ISO Enter");
    }

    #[test]
    fn test_parse_dvorak_variant() {
        let parser = XkbLayoutParser::new().unwrap();
        let keymap = parser.parse(&XkbLayout::Us, &XkbVariant::Dvorak);
        assert!(keymap.is_ok(), "Failed to parse Dvorak variant");
        let keymap = keymap.unwrap();
        assert_eq!(keymap.layout_name, "us");
        assert_eq!(keymap.variant.as_deref(), Some("dvorak"));
    }

    #[test]
    fn test_keymap_string_is_valid_xkb() {
        let parser = XkbLayoutParser::new().unwrap();
        let keymap = parser.parse(&XkbLayout::De, &XkbVariant::Default).unwrap();
        assert!(keymap.keymap_string.starts_with("xkb_keymap"), "Keymap string should start with 'xkb_keymap'");
    }

    #[test]
    fn test_keycodes_contain_alpha_keys() {
        let parser = XkbLayoutParser::new().unwrap();
        let keymap = parser.parse(&XkbLayout::Us, &XkbVariant::Default).unwrap();
        // Evdev keycode 38 = KEY_A, should map to 'a' in US layout
        assert!(keymap.keycodes.contains_key(&38), "Keycode 38 (A) should be in the keymap");
        let entry = keymap.keycodes.get(&38).unwrap();
        assert_eq!(entry.base_label.to_lowercase(), "a");
    }

    #[test]
    fn test_enter_shape_iso_for_de() {
        let shape = XkbLayoutParser::enter_shape(&XkbLayout::De);
        assert!(matches!(shape, KeyShape::LShape { .. }));
    }

    #[test]
    fn test_enter_shape_ansi_for_us() {
        let shape = XkbLayoutParser::enter_shape(&XkbLayout::Us);
        assert!(matches!(shape, KeyShape::Rect { width_u: 2.0 }));
    }
}
