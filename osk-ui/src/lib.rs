//! GTK 4 rendering and touch hit-testing for `osk-rs`.
//!
//! This crate renders the keyboard as a GTK 4 widget grid and handles
//! touch events with custom hit-testing for non-rectangular key shapes
//! (e.g. ISO L-shaped Enter key).

mod custom_key_widget;
mod error;
mod key_visual_state;
mod window;

pub use custom_key_widget::CustomKeyWidget;
pub use error::UiError;
pub use key_visual_state::KeyVisualState;
pub use window::DEFAULT_CSS;
pub use window::OskWindow;
