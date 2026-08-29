//! Visual state of a key for CSS styling.

/// Visual state of a key, determining which CSS class is applied.
///
/// Unlike [`osk_core::KeyState`], which represents the Wayland protocol
/// press/release state, this enum covers all visual CSS states including
/// modifier-active indicators.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyVisualState {
    /// Key is idle — no CSS class.
    Idle,
    /// Key is currently pressed — `"key-pressed"`.
    Pressed,
    /// Modifier key is active (depressed, latched, or locked) — `"key-active"`.
    Active,
}

impl KeyVisualState {
    /// Return the CSS class name associated with this visual state.
    ///
    /// Returns an empty string for `Idle`, meaning no class is added or removed.
    pub const fn css_class(self) -> &'static str {
        match self {
            Self::Idle => "",
            Self::Pressed => "key-pressed",
            Self::Active => "key-active",
        }
    }
}
