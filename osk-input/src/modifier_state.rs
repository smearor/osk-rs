//! Modifier state tracking for the virtual keyboard.

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
