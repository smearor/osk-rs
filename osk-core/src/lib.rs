//! Shared core types for the `osk-rs` on-screen keyboard.
//!
//! This crate provides the fundamental type definitions used across all
//! `osk-rs` crates: key definitions, key states, layout definitions, size
//! variants, display modes, and input type hints.

use serde::{Deserialize, Serialize};

/// A single key definition in the keyboard grid.
///
/// Each key has a Linux evdev keycode, a display label, a geometric shape,
/// and a semantic type that determines how it behaves when pressed.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Key {
    /// Linux evdev keycode for this key
    pub keycode: u32,
    /// Display label shown on the key cap
    pub label: String,
    /// Geometric shape of the key (rectangular or L-shaped)
    pub shape: KeyShape,
    /// Semantic type of the key (alpha, modifier, special, custom)
    pub key_type: KeyType,
}

/// Geometric shape of a key on the keyboard grid.
///
/// Most keys are simple rectangles with a width in grid units. The ISO Enter
/// key has an L-shape that spans two rows.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum KeyShape {
    /// Rectangular key with a width in grid units (1u = one standard key width)
    Rect {
        /// Width in grid units (e.g. 1.0 for a standard key, 2.0 for a wide key)
        width_u: f32,
    },
    /// L-shaped key (ISO Enter) spanning two rows
    LShape {
        /// Width of the upper part in grid units
        top_width_u: f32,
        /// Width of the lower part in grid units
        bottom_width_u: f32,
    },
}

/// Semantic type of a key, determining its behavior when pressed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeyType {
    /// Regular character key (letters, numbers, symbols)
    Character,
    /// Modifier key (Shift, Ctrl, Alt, Super)
    Modifier,
    /// Special key (Esc, Enter, Tab, Backspace, Delete, arrows, F-keys)
    Special,
    /// Custom user-defined key
    Custom,
}

/// Pressed or released state of a key.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeyState {
    /// Key is pressed down
    Pressed,
    /// Key is released
    Released,
}

/// Size variant of the keyboard layout.
///
/// Determines which key blocks are included in the layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SizeVariant {
    /// 60% compact — main block only, no function row, no nav cluster, no numpad
    Compact60,
    /// 80% tenkeyless — main block + function row + nav cluster, no numpad
    Tenkeyless80,
    /// 100% full-size — all blocks including numpad
    Full100,
}

/// Display mode for the on-screen keyboard.
///
/// Controls how the keyboard is positioned on screen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DisplayMode {
    /// Full-width keyboard spanning the entire bottom of the screen
    Full,
    /// Split keyboard with two halves on left and right sides
    Split,
    /// Floating keyboard that can be dragged to any position
    Floating,
}

/// Input type hint from `zwp_input_method_v2` content_type.
///
/// The OSK adapts its layout based on this hint to show the most relevant
/// keys for the focused input field.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum InputType {
    /// Standard text input — no special adaptation
    #[default]
    Text,
    /// Email input — standard layout with @ and domain shortcuts
    Email,
    /// Numeric input — numpad-only layout
    Number,
    /// Telephone number input — numpad-only layout
    Tel,
    /// Password input — no predictive text, no clipboard, AT-SPI muted
    Password,
    /// URL input — standard layout with URL-relevant keys
    Url,
    /// Digit input — numpad-only layout
    Digits,
    /// Emoji input — emoji selector grid
    Emoji,
}

/// Layout definition for a complete keyboard.
///
/// Contains the layout name, optional variant, size variant, and the full
/// grid of keys organized by rows.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LayoutDef {
    /// XKB layout name (e.g. "de", "us", "fr")
    pub name: String,
    /// Optional XKB variant (e.g. "nodeadkeys", "intl")
    pub variant: Option<String>,
    /// Size variant of this layout
    pub size: SizeVariant,
    /// Rows of keys, from top (function row) to bottom (modifier row)
    pub rows: Vec<Vec<Key>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_type_default_is_text() {
        assert_eq!(InputType::default(), InputType::Text);
    }

    #[test]
    fn key_shape_rect_serialization() {
        let shape = KeyShape::Rect { width_u: 1.5 };
        let json = serde_json::to_string(&shape).unwrap();
        let deserialized: KeyShape = serde_json::from_str(&json).unwrap();
        assert_eq!(shape, deserialized);
    }

    #[test]
    fn size_variant_equality() {
        assert_eq!(SizeVariant::Compact60, SizeVariant::Compact60);
        assert_ne!(SizeVariant::Compact60, SizeVariant::Tenkeyless80);
    }

    #[test]
    fn book_summary_has_all_chapters() {
        let summary = include_str!("../../book/src/SUMMARY.md");
        assert!(summary.contains("Introduction"));
        assert!(summary.contains("Getting Started"));
        assert!(summary.contains("Architecture"));
        assert!(summary.contains("Wayland Protocols"));
        assert!(summary.contains("Layout System"));
        assert!(summary.contains("Input Injection"));
        assert!(summary.contains("Hyprland Integration"));
        assert!(summary.contains("Layout Detection"));
        assert!(summary.contains("Touchscreen Support"));
        assert!(summary.contains("Configuration"));
        assert!(summary.contains("Platform Notes"));
    }
}
