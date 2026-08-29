//! Virtual keyboard protocol client and modifier state machine for `osk-rs`.
//!
//! This crate handles key event injection via `zwp_virtual_keyboard_v1`,
//! modifier state tracking, and key repeat behavior.

mod error;
mod modifier_state;
mod virtual_keyboard;

pub use error::InputError;
pub use modifier_state::ModifierState;
pub use virtual_keyboard::VirtualKeyboard;
pub use virtual_keyboard::WaylandVirtualKeyboard;
