//! GTK 4 rendering and touch hit-testing for `osk-rs`.
//!
//! This crate renders the keyboard as a GTK 4 widget grid and handles
//! touch events with custom hit-testing for non-rectangular key shapes
//! (e.g. ISO L-shaped Enter key).

mod error;
mod window;

pub use error::UiError;
pub use window::OskWindow;
