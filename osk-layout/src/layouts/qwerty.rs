//! QWERTY layout sections (US keyboard layout).
//!
//! Provides the layout sections for a US QWERTY keyboard with ANSI
//! wide Enter key. These sections are combined by the
//! [`SizeVariantBuilder`](crate::SizeVariantBuilder) to produce
//! [`LayoutDef`](osk_core::LayoutDef)s for different size variants.

use crate::layouts::LayoutDefinition;
use osk_core::Key;
use osk_core::KeyGrid;
use osk_core::XkbLayout;
use osk_core::XkbVariant;
use osk_core::keycode;

/// QWERTY (US English) keyboard layout.
pub struct Qwerty;

impl LayoutDefinition for Qwerty {
    fn layout() -> XkbLayout {
        XkbLayout::Us
    }

    fn variant() -> XkbVariant {
        XkbVariant::Default
    }

    fn main_block() -> KeyGrid {
        vec![
            // Row 0: Number row
            vec![
                Key::char("`", keycode::KEY_GRAVE),
                Key::char("1", keycode::KEY_1),
                Key::char("2", keycode::KEY_2),
                Key::char("3", keycode::KEY_3),
                Key::char("4", keycode::KEY_4),
                Key::char("5", keycode::KEY_5),
                Key::char("6", keycode::KEY_6),
                Key::char("7", keycode::KEY_7),
                Key::char("8", keycode::KEY_8),
                Key::char("9", keycode::KEY_9),
                Key::char("0", keycode::KEY_0),
                Key::char("-", keycode::KEY_MINUS),
                Key::char("=", keycode::KEY_EQUAL),
                Key::special("⌫", keycode::KEY_BACKSPACE, 2.0),
            ],
            // Row 1: Top row (Q W E R T Y U I O P [ ] \) with ANSI wide Enter
            vec![
                Key::special("⇥", keycode::KEY_TAB, 1.5),
                Key::char("Q", keycode::KEY_Q),
                Key::char("W", keycode::KEY_W),
                Key::char("E", keycode::KEY_E),
                Key::char("R", keycode::KEY_R),
                Key::char("T", keycode::KEY_T),
                Key::char("Y", keycode::KEY_Y),
                Key::char("U", keycode::KEY_U),
                Key::char("I", keycode::KEY_I),
                Key::char("O", keycode::KEY_O),
                Key::char("P", keycode::KEY_P),
                Key::char("[", keycode::KEY_LEFTBRACE),
                Key::char("]", keycode::KEY_RIGHTBRACE),
                Key::special("⏎", keycode::KEY_ENTER, 2.25),
            ],
            // Row 2: Home row (CapsLock A S D F G H J K L ; ' )
            vec![
                Key::modifier("⇪", keycode::KEY_CAPSLOCK, 1.75),
                Key::char("A", keycode::KEY_A),
                Key::char("S", keycode::KEY_S),
                Key::char("D", keycode::KEY_D),
                Key::char("F", keycode::KEY_F),
                Key::char("G", keycode::KEY_G),
                Key::char("H", keycode::KEY_H),
                Key::char("J", keycode::KEY_J),
                Key::char("K", keycode::KEY_K),
                Key::char("L", keycode::KEY_L),
                Key::char(";", keycode::KEY_SEMICOLON),
                Key::char("'", keycode::KEY_APOSTROPHE),
                Key::special("⏎", keycode::KEY_ENTER, 2.25),
            ],
            // Row 3: Bottom row (Shift Z X C V B N M , . / Shift)
            vec![
                Key::modifier("⇧", keycode::KEY_LEFTSHIFT, 2.25),
                Key::char("Z", keycode::KEY_Z),
                Key::char("X", keycode::KEY_X),
                Key::char("C", keycode::KEY_C),
                Key::char("V", keycode::KEY_V),
                Key::char("B", keycode::KEY_B),
                Key::char("N", keycode::KEY_N),
                Key::char("M", keycode::KEY_M),
                Key::char(",", keycode::KEY_COMMA),
                Key::char(".", keycode::KEY_DOT),
                Key::char("/", keycode::KEY_SLASH),
                Key::modifier("⇧", keycode::KEY_RIGHTSHIFT, 2.75),
            ],
            // Row 4: Modifier row (Ctrl Super Alt Space Alt Super Menu Ctrl)
            vec![
                Key::modifier("Ctrl", keycode::KEY_LEFTCTRL, 1.25),
                Key::modifier("Super", keycode::KEY_LEFTMETA, 1.25),
                Key::modifier("Alt", keycode::KEY_LEFTALT, 1.25),
                Key::char_w("Space", keycode::KEY_SPACE, 6.25),
                Key::modifier("Alt", keycode::KEY_RIGHTALT, 1.25),
                Key::modifier("Super", keycode::KEY_RIGHTMETA, 1.25),
                Key::modifier("Menu", keycode::KEY_COMPOSE, 1.25),
                Key::modifier("Ctrl", keycode::KEY_RIGHTCTRL, 1.25),
            ],
        ]
        .into()
    }
}
