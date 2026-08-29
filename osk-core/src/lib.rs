//! Shared core types for the `osk-rs` on-screen keyboard.
//!
//! This crate provides the fundamental type definitions used across all
//! `osk-rs` crates: key definitions, key states, layout definitions, size
//! variants, display modes, and input type hints.

mod display_mode;
mod input_type;
mod key;
mod key_shape;
mod key_state;
mod key_type;
mod layout_def;
mod size_variant;

pub use display_mode::DisplayMode;
pub use input_type::InputType;
pub use key::Key;
pub use key_shape::KeyShape;
pub use key_state::KeyState;
pub use key_type::KeyType;
pub use layout_def::LayoutDef;
pub use size_variant::SizeVariant;

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
