//! Predefined keyboard layout definitions.
//!
//! Each module provides a layout implementation via the [`LayoutDefinition`]
//! trait. The sections are combined by the
//! [`SizeVariantBuilder`](crate::SizeVariantBuilder) to produce
//! [`LayoutDef`](osk_core::LayoutDef)s for different size variants.

mod azerty;
mod dvorak;
mod qwerty;
mod qwertz;
mod qwertz_tkl;

use osk_core::Key;
use osk_core::KeyGrid;
use osk_core::KeyRow;
use osk_core::XkbLayout;
use osk_core::XkbVariant;
use osk_core::keycode;

pub use azerty::Azerty;
pub use dvorak::Dvorak;
pub use qwerty::Qwerty;
pub use qwertz::Qwertz;
pub use qwertz_tkl::qwertz_tkl;

/// Trait for predefined keyboard layout definitions.
///
/// Each layout provides its XKB identity and the key sections that make up
/// the keyboard. The `function_row`, `nav_cluster`, and `numpad` sections
/// are shared across all standard layouts and have default implementations.
/// Layouts only need to implement [`layout`](Self::layout),
/// [`variant`](Self::variant), and [`main_block`](Self::main_block).
pub trait LayoutDefinition {
    /// XKB layout name
    fn layout() -> XkbLayout;

    /// XKB layout variant
    fn variant() -> XkbVariant;

    /// Function row (Esc, F1–F12) — 1 row
    fn function_row() -> KeyRow {
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
        ]
        .into()
    }

    /// Main block (alpha keys, modifiers, space) — 5 rows
    fn main_block() -> KeyGrid;

    /// Nav cluster aligned to function_row + main_block rows — 6 rows.
    ///
    /// Each row contains the nav keys for that row, preceded by a spacer
    /// for separation from the main block. Rows with no nav keys contain
    /// only a spacer to maintain alignment.
    fn nav_cluster() -> KeyGrid {
        vec![
            // Row 0: aligned with function row — no extra nav keys
            vec![Key::spacer(0.5), Key::spacer(3.0)],
            // Row 1: Ins, Hom, PgU
            vec![
                Key::spacer(0.5),
                Key::special("Ins", keycode::KEY_INSERT, 1.0),
                Key::special("Hom", keycode::KEY_HOME, 1.0),
                Key::special("PgU", keycode::KEY_PAGEUP, 1.0),
            ],
            // Row 2: Del, End, PgD
            vec![
                Key::spacer(0.5),
                Key::special("Del", keycode::KEY_DELETE, 1.0),
                Key::special("End", keycode::KEY_END, 1.0),
                Key::special("PgD", keycode::KEY_PAGEDOWN, 1.0),
            ],
            // Row 3: empty (aligned with home row)
            vec![Key::spacer(0.5 + 3.0)],
            // Row 4: empty (aligned with bottom row)
            vec![Key::spacer(0.5), Key::spacer(1.0), Key::special("↑", keycode::KEY_UP, 1.0), Key::spacer(1.0)],
            // Row 5: arrow keys (aligned with modifier row)
            vec![
                Key::spacer(0.5),
                Key::spacer(1.0),
                Key::special("←", keycode::KEY_LEFT, 1.0),
                Key::special("↓", keycode::KEY_DOWN, 1.0),
                Key::special("→", keycode::KEY_RIGHT, 1.0),
            ],
        ]
        .into()
    }

    /// Numpad aligned to main_block rows — 5 rows.
    ///
    /// Each row contains the numpad keys for that row, preceded by a spacer
    /// for separation from the nav cluster.
    fn numpad() -> KeyGrid {
        vec![
            // Row 0: NumLock, /, *, -
            vec![
                Key::spacer(0.5),
                Key::special("Num", keycode::KEY_NUMLOCK, 1.0),
                Key::special("/", keycode::KEY_KPSLASH, 1.0),
                Key::special("*", keycode::KEY_KPASTERISK, 1.0),
                Key::special("-", keycode::KEY_KPMINUS, 1.0),
            ],
            // Row 1: 7, 8, 9, +
            vec![
                Key::spacer(0.5),
                Key::char("7", keycode::KEY_KP7),
                Key::char("8", keycode::KEY_KP8),
                Key::char("9", keycode::KEY_KP9),
                Key::special("+", keycode::KEY_KPPLUS, 1.0),
            ],
            // Row 2: 4, 5, 6
            vec![
                Key::spacer(0.5),
                Key::char("4", keycode::KEY_KP4),
                Key::char("5", keycode::KEY_KP5),
                Key::char("6", keycode::KEY_KP6),
                Key::spacer(1.0),
            ],
            // Row 3: 1, 2, 3, Enter
            vec![
                Key::spacer(0.5),
                Key::char("1", keycode::KEY_KP1),
                Key::char("2", keycode::KEY_KP2),
                Key::char("3", keycode::KEY_KP3),
                Key::special("⏎", keycode::KEY_KPENTER, 1.0),
            ],
            // Row 4: 0, .
            vec![
                Key::spacer(0.5),
                Key::char_w("0", keycode::KEY_KP0, 2.0),
                Key::char(".", keycode::KEY_KPDOT),
                Key::spacer(1.0),
            ],
        ]
        .into()
    }

    /// Build the complete [`LayoutSections`] from this layout definition.
    fn sections() -> crate::LayoutSections {
        crate::LayoutSections {
            name: Self::layout(),
            variant: Self::variant(),
            function_row: Self::function_row(),
            main_block: Self::main_block(),
            nav_cluster: Self::nav_cluster(),
            numpad: Self::numpad(),
        }
    }
}
