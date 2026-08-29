//! Virtual keyboard protocol client and modifier state machine for `osk-rs`.
//!
//! This crate handles key event injection via `zwp_virtual_keyboard_v1`,
//! modifier state tracking, and key repeat behavior.

use osk_core::KeyState;
use thiserror::Error;

/// Errors that can occur during virtual keyboard operations.
#[derive(Debug, Error)]
pub enum InputError {
    /// The virtual keyboard protocol is not available on this compositor
    #[error("virtual keyboard manager not found in registry")]
    ManagerNotFound,
    /// The keymap could not be published to the compositor
    #[error("keymap publication failed: {0}")]
    KeymapPublicationFailed(String),
    /// A key event was sent with an invalid keycode
    #[error("invalid keycode: {0}")]
    InvalidKeycode(u32),
}

/// Modifier state tracking for the virtual keyboard.
///
/// Tracks which modifiers are currently depressed, latched, or locked.
/// The compositor is notified of changes via `virtual_keyboard.modifiers()`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ModifierState {
    /// Bitmask of currently depressed modifiers (held down by the user)
    pub depressed: u32,
    /// Bitmask of latched modifiers (active for one keypress, then cleared)
    pub latched: u32,
    /// Bitmask of locked modifiers (toggled on/off, e.g. CapsLock)
    pub locked: u32,
    /// Current keyboard layout group index
    pub group: u32,
}

impl ModifierState {
    /// Create a new modifier state with no modifiers active.
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if Shift is currently active (depressed or latched).
    pub fn is_shift_active(&self) -> bool {
        (self.depressed | self.latched) & 0x01 != 0
    }

    /// Check if CapsLock is locked on.
    pub fn is_caps_lock_on(&self) -> bool {
        self.locked & 0x02 != 0
    }

    /// Check if Ctrl is currently active.
    pub fn is_ctrl_active(&self) -> bool {
        (self.depressed | self.latched) & 0x04 != 0
    }

    /// Check if Alt is currently active.
    pub fn is_alt_active(&self) -> bool {
        (self.depressed | self.latched) & 0x08 != 0
    }

    /// Check if Super (Meta) is currently active.
    pub fn is_super_active(&self) -> bool {
        (self.depressed | self.latched) & 0x40 != 0
    }
}

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
