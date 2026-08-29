//! AZERTY layout sections (French keyboard layout).
//!
//! Provides the layout sections for a French AZERTY keyboard with ISO
//! L-shaped Enter key. These sections are combined by the
//! [`SizeVariantBuilder`](crate::SizeVariantBuilder) to produce
//! [`LayoutDef`](osk_core::LayoutDef)s for different size variants.

use crate::layouts::LayoutDefinition;
use osk_core::Key;
use osk_core::KeyGrid;
use osk_core::XkbLayout;
use osk_core::XkbVariant;
use osk_core::keycode;

/// AZERTY (French) keyboard layout.
pub struct Azerty;

impl LayoutDefinition for Azerty {
    fn layout() -> XkbLayout {
        XkbLayout::Fr
    }

    fn variant() -> XkbVariant {
        XkbVariant::Default
    }

    fn main_block() -> KeyGrid {
        vec![
            // Row 0: Number row (AZERTY: ² & é " ' ( - è _ ç à ) = Backspace)
            vec![
                Key::char("²", keycode::KEY_GRAVE),
                Key::char("&", keycode::KEY_1),
                Key::char("é", keycode::KEY_2),
                Key::char("\"", keycode::KEY_3),
                Key::char("'", keycode::KEY_4),
                Key::char("(", keycode::KEY_5),
                Key::char("-", keycode::KEY_6),
                Key::char("è", keycode::KEY_7),
                Key::char("_", keycode::KEY_8),
                Key::char("ç", keycode::KEY_9),
                Key::char("à", keycode::KEY_0),
                Key::char(")", keycode::KEY_MINUS),
                Key::char("=", keycode::KEY_EQUAL),
                Key::special("⌫", keycode::KEY_BACKSPACE, 2.0),
            ],
            // Row 1: Top row (A Z E R T Y U I O P ^ $) with ISO L-Enter
            vec![
                Key::special("⇥", keycode::KEY_TAB, 1.5),
                Key::char("A", keycode::KEY_Q),
                Key::char("Z", keycode::KEY_W),
                Key::char("E", keycode::KEY_E),
                Key::char("R", keycode::KEY_R),
                Key::char("T", keycode::KEY_T),
                Key::char("Y", keycode::KEY_Y),
                Key::char("U", keycode::KEY_U),
                Key::char("I", keycode::KEY_I),
                Key::char("O", keycode::KEY_O),
                Key::char("P", keycode::KEY_P),
                Key::char("^", keycode::KEY_LEFTBRACE),
                Key::char("$", keycode::KEY_RIGHTBRACE),
                Key::l_enter("⏎", keycode::KEY_ENTER, 1.5, 1.25),
            ],
            // Row 2: Home row (CapsLock Q S D F G H J K L M ù *)
            vec![
                Key::modifier("⇪", keycode::KEY_CAPSLOCK, 1.75),
                Key::char("Q", keycode::KEY_A),
                Key::char("S", keycode::KEY_S),
                Key::char("D", keycode::KEY_D),
                Key::char("F", keycode::KEY_F),
                Key::char("G", keycode::KEY_G),
                Key::char("H", keycode::KEY_H),
                Key::char("J", keycode::KEY_J),
                Key::char("K", keycode::KEY_K),
                Key::char("L", keycode::KEY_L),
                Key::char("M", keycode::KEY_SEMICOLON),
                Key::char("ù", keycode::KEY_APOSTROPHE),
                Key::char("*", keycode::KEY_BACKSLASH),
                Key::spacer(1.5 + 1.25),
            ],
            // Row 3: Bottom row (Shift < W X C V B N , ; : ! Shift)
            vec![
                Key::modifier("⇧", keycode::KEY_LEFTSHIFT, 1.25),
                Key::char("<", keycode::KEY_LESS),
                Key::char("W", keycode::KEY_Z),
                Key::char("X", keycode::KEY_X),
                Key::char("C", keycode::KEY_C),
                Key::char("V", keycode::KEY_V),
                Key::char("B", keycode::KEY_B),
                Key::char("N", keycode::KEY_N),
                Key::char(",", keycode::KEY_COMMA),
                Key::char(";", keycode::KEY_DOT),
                Key::char(":", keycode::KEY_SLASH),
                Key::char("!", keycode::KEY_APOSTROPHE),
                Key::modifier("⇧", keycode::KEY_RIGHTSHIFT, 2.75),
            ],
            // Row 4: Modifier row (Ctrl Super Alt Space AltGr Super Menu Ctrl)
            vec![
                Key::modifier("Ctrl", keycode::KEY_LEFTCTRL, 1.25),
                Key::modifier("Super", keycode::KEY_LEFTMETA, 1.25),
                Key::modifier("Alt", keycode::KEY_LEFTALT, 1.25),
                Key::char_w("Space", keycode::KEY_SPACE, 6.25),
                Key::modifier("AltGr", keycode::KEY_RIGHTALT, 1.25),
                Key::modifier("Super", keycode::KEY_RIGHTMETA, 1.25),
                Key::modifier("Menu", keycode::KEY_COMPOSE, 1.25),
                Key::modifier("Ctrl", keycode::KEY_RIGHTCTRL, 1.25),
            ],
        ]
        .into()
    }
}
