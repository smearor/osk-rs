//! Shared core types for the `osk-rs` on-screen keyboard.
//!
//! This crate provides the fundamental type definitions used across all
//! `osk-rs` crates: key definitions, key states, layout definitions, size
//! variants, display modes, and input type hints.

mod display_mode;
mod input_type;
mod key;
mod key_grid;
mod key_row;
mod key_shape;
mod key_state;
mod key_type;
pub mod keycode;
mod layout_def;
mod modifier;
mod size_variant;
mod xkb_layout;
mod xkb_variant;

pub use display_mode::DisplayMode;
pub use input_type::InputType;
pub use key::Key;
pub use key_grid::KeyGrid;
pub use key_row::KeyRow;
pub use key_shape::KeyShape;
pub use key_state::KeyState;
pub use key_type::KeyType;
pub use keycode::KeyCode;
pub use layout_def::LayoutDef;
pub use modifier::Modifier;
pub use size_variant::SizeVariant;
pub use xkb_layout::XkbLayout;
pub use xkb_variant::XkbVariant;

#[cfg(test)]
mod tests {
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
