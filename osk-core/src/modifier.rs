//! Modifier types for the Wayland virtual keyboard protocol.
//!
//! The [`Modifier`] enum represents the five modifier keys tracked by the
//! on-screen keyboard. Each variant maps to a bitmask value used by the
//! `zwp_virtual_keyboard_v1` `modifiers` event.

/// A keyboard modifier key.
///
/// Each variant corresponds to a modifier bitmask used by the
/// `zwp_virtual_keyboard_v1` `modifiers` event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Modifier {
    /// Shift (bit 0)
    Shift,
    /// CapsLock (bit 1)
    CapsLock,
    /// Ctrl (bit 2)
    Ctrl,
    /// Alt (bit 3)
    Alt,
    /// Super / Meta (bit 6)
    Super,
}

impl Modifier {
    /// Return the Wayland protocol bitmask for this modifier.
    pub const fn bitmask(self) -> u32 {
        match self {
            Self::Shift => 0x01,
            Self::CapsLock => 0x02,
            Self::Ctrl => 0x04,
            Self::Alt => 0x08,
            Self::Super => 0x40,
        }
    }
}

impl From<Modifier> for u32 {
    fn from(modifier: Modifier) -> Self {
        modifier.bitmask()
    }
}
