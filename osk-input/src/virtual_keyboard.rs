//! Trait for virtual keyboard protocol clients.

use osk_core::KeyState;

use crate::{InputError, ModifierState};

/// Trait for virtual keyboard protocol clients.
///
/// Implementations send key events and modifier state to the compositor
/// via `zwp_virtual_keyboard_v1`.
pub trait VirtualKeyboard {
    /// Publish the XKB keymap string to the compositor.
    ///
    /// # Errors
    ///
    /// Returns `InputError` if the keymap cannot be published.
    fn publish_keymap(&mut self, keymap_str: &str) -> Result<(), InputError>;

    /// Send a key event to the compositor.
    ///
    /// # Errors
    ///
    /// Returns `InputError` if the key event cannot be sent.
    fn send_key(&mut self, keycode: u32, state: KeyState) -> Result<(), InputError>;

    /// Send the current modifier state to the compositor.
    ///
    /// # Errors
    ///
    /// Returns `InputError` if the modifier state cannot be sent.
    fn send_modifiers(&mut self, modifiers: &ModifierState) -> Result<(), InputError>;
}
