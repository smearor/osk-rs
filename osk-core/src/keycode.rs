//! Linux evdev keycode constants used in keyboard layouts.
//!
//! These constants map to the keycodes defined in
//! `<linux/input-event-codes.h>` and are used by the
//! `zwp_virtual_keyboard_v1` protocol.

use crate::modifier::Modifier;
use serde::Deserialize;
use serde::Serialize;

/// A Linux evdev keycode.
///
/// This newtype wraps a `u32` to provide type safety and prevent accidental
/// confusion with other integer values. The inner value corresponds to the
/// keycode constants defined in `<linux/input-event-codes.h>`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct KeyCode(pub u32);

impl KeyCode {
    /// Create a keycode from a raw evdev keycode value.
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    /// Return the raw evdev keycode value.
    pub const fn raw(self) -> u32 {
        self.0
    }

    /// Return the modifier for this keycode, if it is a modifier key.
    ///
    /// Returns `Some(Modifier)` for Shift, CapsLock, Ctrl, Alt, and Super keys.
    /// Returns `None` for non-modifier keys.
    pub fn modifier(self) -> Option<Modifier> {
        match self {
            KEY_LEFTSHIFT | KEY_RIGHTSHIFT => Some(Modifier::Shift),
            KEY_CAPSLOCK => Some(Modifier::CapsLock),
            KEY_LEFTCTRL | KEY_RIGHTCTRL => Some(Modifier::Ctrl),
            KEY_LEFTALT | KEY_RIGHTALT => Some(Modifier::Alt),
            KEY_LEFTMETA | KEY_RIGHTMETA => Some(Modifier::Super),
            _ => None,
        }
    }
}

impl From<u32> for KeyCode {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<KeyCode> for u32 {
    fn from(keycode: KeyCode) -> Self {
        keycode.0
    }
}

/// Escape
pub const KEY_ESC: KeyCode = KeyCode::new(1);
/// F1
pub const KEY_F1: KeyCode = KeyCode::new(59);
/// F2
pub const KEY_F2: KeyCode = KeyCode::new(60);
/// F3
pub const KEY_F3: KeyCode = KeyCode::new(61);
/// F4
pub const KEY_F4: KeyCode = KeyCode::new(62);
/// F5
pub const KEY_F5: KeyCode = KeyCode::new(63);
/// F6
pub const KEY_F6: KeyCode = KeyCode::new(64);
/// F7
pub const KEY_F7: KeyCode = KeyCode::new(65);
/// F8
pub const KEY_F8: KeyCode = KeyCode::new(66);
/// F9
pub const KEY_F9: KeyCode = KeyCode::new(67);
/// F10
pub const KEY_F10: KeyCode = KeyCode::new(68);
/// F11
pub const KEY_F11: KeyCode = KeyCode::new(87);
/// F12
pub const KEY_F12: KeyCode = KeyCode::new(88);

/// Backtick / grave accent
pub const KEY_GRAVE: KeyCode = KeyCode::new(41);
/// 1
pub const KEY_1: KeyCode = KeyCode::new(2);
/// 2
pub const KEY_2: KeyCode = KeyCode::new(3);
/// 3
pub const KEY_3: KeyCode = KeyCode::new(4);
/// 4
pub const KEY_4: KeyCode = KeyCode::new(5);
/// 5
pub const KEY_5: KeyCode = KeyCode::new(6);
/// 6
pub const KEY_6: KeyCode = KeyCode::new(7);
/// 7
pub const KEY_7: KeyCode = KeyCode::new(8);
/// 8
pub const KEY_8: KeyCode = KeyCode::new(9);
/// 9
pub const KEY_9: KeyCode = KeyCode::new(10);
/// 0
pub const KEY_0: KeyCode = KeyCode::new(11);
/// ß (German sharp s) / minus on US
pub const KEY_MINUS: KeyCode = KeyCode::new(12);
/// ´ (acute accent) / equal on US
pub const KEY_EQUAL: KeyCode = KeyCode::new(13);
/// Backspace
pub const KEY_BACKSPACE: KeyCode = KeyCode::new(14);

/// Tab
pub const KEY_TAB: KeyCode = KeyCode::new(15);
/// Q
pub const KEY_Q: KeyCode = KeyCode::new(16);
/// W
pub const KEY_W: KeyCode = KeyCode::new(17);
/// E
pub const KEY_E: KeyCode = KeyCode::new(18);
/// R
pub const KEY_R: KeyCode = KeyCode::new(19);
/// T
pub const KEY_T: KeyCode = KeyCode::new(20);
/// Z (QWERTZ) / Y (QWERTY)
pub const KEY_Z: KeyCode = KeyCode::new(21);
/// U
pub const KEY_U: KeyCode = KeyCode::new(22);
/// I
pub const KEY_I: KeyCode = KeyCode::new(23);
/// O
pub const KEY_O: KeyCode = KeyCode::new(24);
/// P
pub const KEY_P: KeyCode = KeyCode::new(25);
/// Ü (German) / [ on US
pub const KEY_LEFTBRACE: KeyCode = KeyCode::new(26);
/// + (German) / ] on US
pub const KEY_RIGHTBRACE: KeyCode = KeyCode::new(27);
/// Enter
pub const KEY_ENTER: KeyCode = KeyCode::new(28);

/// CapsLock
pub const KEY_CAPSLOCK: KeyCode = KeyCode::new(58);
/// A
pub const KEY_A: KeyCode = KeyCode::new(30);
/// S
pub const KEY_S: KeyCode = KeyCode::new(31);
/// D
pub const KEY_D: KeyCode = KeyCode::new(32);
/// F
pub const KEY_F: KeyCode = KeyCode::new(33);
/// G
pub const KEY_G: KeyCode = KeyCode::new(34);
/// H
pub const KEY_H: KeyCode = KeyCode::new(35);
/// J
pub const KEY_J: KeyCode = KeyCode::new(36);
/// K
pub const KEY_K: KeyCode = KeyCode::new(37);
/// L
pub const KEY_L: KeyCode = KeyCode::new(38);
/// Ö (German) / semicolon on US
pub const KEY_SEMICOLON: KeyCode = KeyCode::new(39);
/// Ä (German) / apostrophe on US
pub const KEY_APOSTROPHE: KeyCode = KeyCode::new(40);
/// # (German) / backslash on US
pub const KEY_BACKSLASH: KeyCode = KeyCode::new(43);

/// Left Shift
pub const KEY_LEFTSHIFT: KeyCode = KeyCode::new(42);
/// < (German ISO key) / non-US backslash
pub const KEY_LESS: KeyCode = KeyCode::new(86);
/// Y (QWERTZ) / Z (QWERTY)
pub const KEY_Y: KeyCode = KeyCode::new(44);
/// X
pub const KEY_X: KeyCode = KeyCode::new(45);
/// C
pub const KEY_C: KeyCode = KeyCode::new(46);
/// V
pub const KEY_V: KeyCode = KeyCode::new(47);
/// B
pub const KEY_B: KeyCode = KeyCode::new(48);
/// N
pub const KEY_N: KeyCode = KeyCode::new(49);
/// M
pub const KEY_M: KeyCode = KeyCode::new(50);
/// , (comma)
pub const KEY_COMMA: KeyCode = KeyCode::new(51);
/// . (period)
pub const KEY_DOT: KeyCode = KeyCode::new(52);
/// - (minus / slash on US)
pub const KEY_SLASH: KeyCode = KeyCode::new(53);
/// Right Shift
pub const KEY_RIGHTSHIFT: KeyCode = KeyCode::new(54);

/// Left Ctrl
pub const KEY_LEFTCTRL: KeyCode = KeyCode::new(29);
/// Super (Meta / Windows key)
pub const KEY_LEFTMETA: KeyCode = KeyCode::new(125);
/// Left Alt
pub const KEY_LEFTALT: KeyCode = KeyCode::new(56);
/// Space
pub const KEY_SPACE: KeyCode = KeyCode::new(57);
/// Right Alt (AltGr)
pub const KEY_RIGHTALT: KeyCode = KeyCode::new(100);
/// Right Super
pub const KEY_RIGHTMETA: KeyCode = KeyCode::new(126);
/// Compose / Menu
pub const KEY_COMPOSE: KeyCode = KeyCode::new(127);
/// Right Ctrl
pub const KEY_RIGHTCTRL: KeyCode = KeyCode::new(97);

/// Insert
pub const KEY_INSERT: KeyCode = KeyCode::new(110);
/// Delete
pub const KEY_DELETE: KeyCode = KeyCode::new(111);
/// Home
pub const KEY_HOME: KeyCode = KeyCode::new(102);
/// End
pub const KEY_END: KeyCode = KeyCode::new(107);
/// Page Up
pub const KEY_PAGEUP: KeyCode = KeyCode::new(104);
/// Page Down
pub const KEY_PAGEDOWN: KeyCode = KeyCode::new(109);

/// Up arrow
pub const KEY_UP: KeyCode = KeyCode::new(103);
/// Down arrow
pub const KEY_DOWN: KeyCode = KeyCode::new(108);
/// Left arrow
pub const KEY_LEFT: KeyCode = KeyCode::new(105);
/// Right arrow
pub const KEY_RIGHT: KeyCode = KeyCode::new(106);

/// Print Screen
pub const KEY_PRINT: KeyCode = KeyCode::new(99);
/// Scroll Lock
pub const KEY_SCROLLLOCK: KeyCode = KeyCode::new(70);
/// Pause / Break
pub const KEY_PAUSE: KeyCode = KeyCode::new(119);
