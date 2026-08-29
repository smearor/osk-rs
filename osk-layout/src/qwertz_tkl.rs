//! Hardcoded QWERTZ TKL (80%) layout for Phase 1.
//!
//! This layout is used as the default and only layout in Phase 1.
//! It defines 6 rows of keys with correct evdev keycodes for a German
//! QWERTZ keyboard with ISO L-shaped Enter key.

use osk_core::Key;
use osk_core::LayoutDef;
use osk_core::SizeVariant;
use osk_core::XkbVariant;
use osk_core::keycode;

/// Build the hardcoded QWERTZ TKL layout definition.
///
/// This produces a 6-row layout with:
/// - Row 0: Function row (Esc, F1–F12, with nav cluster gaps)
/// - Row 1: Number row (grave through backspace)
/// - Row 2: Top row (Tab through ISO L-Enter)
/// - Row 3: Home row (CapsLock through hash/backslash)
/// - Row 4: Bottom row (Shift through Shift)
/// - Row 5: Modifier row (Ctrl through Ctrl)
///
/// Plus a nav cluster and arrow keys for TKL.
pub fn qwertz_tkl() -> LayoutDef {
    LayoutDef {
        name: "de".to_string(),
        variant: XkbVariant::Default,
        size: SizeVariant::Tenkeyless80,
        rows: vec![
            // Row 0: Function row + nav cluster
            vec![
                Key::func("Esc", keycode::KEY_ESC),
                Key::spacer(1.0),
                Key::func("F1", keycode::KEY_F1),
                Key::func("F2", keycode::KEY_F2),
                Key::func("F3", keycode::KEY_F3),
                Key::func("F4", keycode::KEY_F4),
                Key::spacer(0.5),
                Key::func("F5", keycode::KEY_F5),
                Key::func("F6", keycode::KEY_F6),
                Key::func("F7", keycode::KEY_F7),
                Key::func("F8", keycode::KEY_F8),
                Key::spacer(0.5),
                Key::func("F9", keycode::KEY_F9),
                Key::func("F10", keycode::KEY_F10),
                Key::func("F11", keycode::KEY_F11),
                Key::func("F12", keycode::KEY_F12),
                Key::spacer(0.5),
                Key::special("Prn", keycode::KEY_PRINT, 1.0),
                Key::special("Scl", keycode::KEY_SCROLLLOCK, 1.0),
                Key::special("Pse", keycode::KEY_PAUSE, 1.0),
            ],
            // Row 1: Number row
            vec![
                Key::char("^", keycode::KEY_GRAVE),
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
                Key::char("ß", keycode::KEY_MINUS),
                Key::char("´", keycode::KEY_EQUAL),
                Key::special("⌫", keycode::KEY_BACKSPACE, 2.0),
                Key::spacer(0.5),
                Key::special("Ins", keycode::KEY_INSERT, 1.0),
                Key::special("Hom", keycode::KEY_HOME, 1.0),
                Key::special("PgU", keycode::KEY_PAGEUP, 1.0),
            ],
            // Row 2: Top row (Q W E R T Z U I O P Ü +) with ISO L-Enter
            vec![
                Key::special("⇥", keycode::KEY_TAB, 1.5),
                Key::char("Q", keycode::KEY_Q),
                Key::char("W", keycode::KEY_W),
                Key::char("E", keycode::KEY_E),
                Key::char("R", keycode::KEY_R),
                Key::char("T", keycode::KEY_T),
                Key::char("Z", keycode::KEY_Z),
                Key::char("U", keycode::KEY_U),
                Key::char("I", keycode::KEY_I),
                Key::char("O", keycode::KEY_O),
                Key::char("P", keycode::KEY_P),
                Key::char("Ü", keycode::KEY_LEFTBRACE),
                Key::char("+", keycode::KEY_RIGHTBRACE),
                Key::l_enter("⏎", keycode::KEY_ENTER, 1.5, 1.25),
                Key::spacer(0.5),
                Key::special("Del", keycode::KEY_DELETE, 1.0),
                Key::special("End", keycode::KEY_END, 1.0),
                Key::special("PgD", keycode::KEY_PAGEDOWN, 1.0),
            ],
            // Row 3: Home row (CapsLock A S D F G H J K L Ö Ä #)
            // ISO layout: the # key sits where the Enter key's L-shape extends down
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
                Key::char("Ö", keycode::KEY_SEMICOLON),
                Key::char("Ä", keycode::KEY_APOSTROPHE),
                Key::char("#", keycode::KEY_BACKSLASH),
                Key::spacer(0.5 + 1.5 + 1.0),
            ],
            // Row 4: Bottom row (Shift < Y X C V B N M , . - Shift)
            vec![
                Key::modifier("⇧", keycode::KEY_LEFTSHIFT, 1.25),
                Key::char("<", keycode::KEY_LESS),
                Key::char("Y", keycode::KEY_Y),
                Key::char("X", keycode::KEY_X),
                Key::char("C", keycode::KEY_C),
                Key::char("V", keycode::KEY_V),
                Key::char("B", keycode::KEY_B),
                Key::char("N", keycode::KEY_N),
                Key::char("M", keycode::KEY_M),
                Key::char(",", keycode::KEY_COMMA),
                Key::char(".", keycode::KEY_DOT),
                Key::char("-", keycode::KEY_SLASH),
                Key::modifier("⇧", keycode::KEY_RIGHTSHIFT, 2.75),
                Key::spacer(0.5),
                Key::spacer(1.0),
                Key::special("↑", keycode::KEY_UP, 1.0),
                Key::spacer(1.0),
            ],
            // Row 5: Modifier row (Ctrl Super Alt Space Alt Super Menu Ctrl)
            vec![
                Key::modifier("Ctrl", keycode::KEY_LEFTCTRL, 1.25),
                Key::modifier("Super", keycode::KEY_LEFTMETA, 1.25),
                Key::modifier("Alt", keycode::KEY_LEFTALT, 1.25),
                Key::char_w("Space", keycode::KEY_SPACE, 6.25),
                Key::modifier("AltGr", keycode::KEY_RIGHTALT, 1.25),
                Key::modifier("Super", keycode::KEY_RIGHTMETA, 1.25),
                Key::modifier("Menu", keycode::KEY_COMPOSE, 1.25),
                Key::modifier("Ctrl", keycode::KEY_RIGHTCTRL, 1.25),
                Key::spacer(0.5),
                Key::spacer(1.0),
                Key::special("←", keycode::KEY_LEFT, 1.0),
                Key::special("↓", keycode::KEY_DOWN, 1.0),
                Key::special("→", keycode::KEY_RIGHT, 1.0),
            ],
        ],
    }
}
