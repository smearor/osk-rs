//! Modifier state tracking for the virtual keyboard.

use osk_core::Modifier;

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
        (self.depressed | self.latched) & Modifier::Shift.bitmask() != 0
    }

    /// Check if CapsLock is locked on.
    pub fn is_caps_lock_on(&self) -> bool {
        self.locked & Modifier::CapsLock.bitmask() != 0
    }

    /// Check if Ctrl is currently active.
    pub fn is_ctrl_active(&self) -> bool {
        (self.depressed | self.latched) & Modifier::Ctrl.bitmask() != 0
    }

    /// Check if Alt is currently active.
    pub fn is_alt_active(&self) -> bool {
        (self.depressed | self.latched) & Modifier::Alt.bitmask() != 0
    }

    /// Check if Super (Meta) is currently active.
    pub fn is_super_active(&self) -> bool {
        (self.depressed | self.latched) & Modifier::Super.bitmask() != 0
    }

    /// Press a modifier key, updating the state accordingly.
    ///
    /// - Shift, Ctrl, Alt, Super: set the depressed bit.
    /// - CapsLock: toggle the locked bit.
    pub fn press(&mut self, modifier: Modifier) {
        if modifier == Modifier::CapsLock {
            self.locked ^= modifier.bitmask();
        } else {
            self.depressed |= modifier.bitmask();
        }
    }

    /// Release a modifier key, clearing the depressed bit.
    ///
    /// CapsLock is not affected by release — it stays locked until toggled.
    pub fn release(&mut self, modifier: Modifier) {
        if modifier != Modifier::CapsLock {
            self.depressed &= !modifier.bitmask();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_state_is_empty() {
        let state = ModifierState::new();
        assert_eq!(state.depressed, 0);
        assert_eq!(state.latched, 0);
        assert_eq!(state.locked, 0);
        assert_eq!(state.group, 0);
        assert!(!state.is_shift_active());
        assert!(!state.is_caps_lock_on());
        assert!(!state.is_ctrl_active());
        assert!(!state.is_alt_active());
        assert!(!state.is_super_active());
    }

    #[test]
    fn test_press_shift_sets_depressed() {
        let mut state = ModifierState::new();
        state.press(Modifier::Shift);
        assert!(state.is_shift_active());
        assert_eq!(state.depressed, Modifier::Shift.bitmask());
    }

    #[test]
    fn test_release_shift_clears_depressed() {
        let mut state = ModifierState::new();
        state.press(Modifier::Shift);
        state.release(Modifier::Shift);
        assert!(!state.is_shift_active());
        assert_eq!(state.depressed, 0);
    }

    #[test]
    fn test_caps_lock_toggles_on_press() {
        let mut state = ModifierState::new();
        state.press(Modifier::CapsLock);
        assert!(state.is_caps_lock_on());
        assert_eq!(state.locked, Modifier::CapsLock.bitmask());

        state.press(Modifier::CapsLock);
        assert!(!state.is_caps_lock_on());
        assert_eq!(state.locked, 0);
    }

    #[test]
    fn test_caps_lock_release_does_nothing() {
        let mut state = ModifierState::new();
        state.press(Modifier::CapsLock);
        state.release(Modifier::CapsLock);
        assert!(state.is_caps_lock_on());
    }

    #[test]
    fn test_multiple_modifiers() {
        let mut state = ModifierState::new();
        state.press(Modifier::Ctrl);
        state.press(Modifier::Alt);
        assert!(state.is_ctrl_active());
        assert!(state.is_alt_active());
        assert!(!state.is_shift_active());

        state.release(Modifier::Ctrl);
        assert!(!state.is_ctrl_active());
        assert!(state.is_alt_active());
    }

    #[test]
    fn test_super_modifier() {
        let mut state = ModifierState::new();
        state.press(Modifier::Super);
        assert!(state.is_super_active());
        state.release(Modifier::Super);
        assert!(!state.is_super_active());
    }
}
