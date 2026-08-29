//! Builder for size variants — selects which key blocks to include.

use crate::LayoutSections;
use osk_core::Key;
use osk_core::KeyGrid;
use osk_core::LayoutDef;
use osk_core::SizeVariant;

/// Builder for size variants — selects which key blocks to include.
///
/// - `Compact60`: main block only (no function row, no nav cluster, no numpad)
/// - `Tenkeyless80`: function row + main block + nav cluster (no numpad)
/// - `Full100`: function row + main block + nav cluster + numpad
pub struct SizeVariantBuilder;

impl SizeVariantBuilder {
    /// Build a [`LayoutDef`] from [`LayoutSections`] for the given size variant.
    ///
    /// Combines the layout sections into a single grid of rows, selecting
    /// which sections to include based on the size variant.
    pub fn build(sections: &LayoutSections, size: SizeVariant) -> LayoutDef {
        let rows = match size {
            SizeVariant::Compact60 => Self::build_compact60(sections),
            SizeVariant::Tenkeyless80 => Self::build_tkl(sections),
            SizeVariant::Full100 => Self::build_full(sections),
        };

        LayoutDef {
            name: sections.name.clone(),
            variant: sections.variant.clone(),
            size,
            rows,
        }
    }

    /// Build a 60% compact layout — main block only.
    fn build_compact60(sections: &LayoutSections) -> KeyGrid {
        sections.main_block.clone()
    }

    /// Build a TKL (80%) layout — function row + main block + nav cluster.
    fn build_tkl(sections: &LayoutSections) -> KeyGrid {
        let mut rows = KeyGrid::new();

        // Row 0: function row + nav cluster row 0
        let mut row0 = sections.function_row.clone();
        if let Some(nav_row) = sections.nav_cluster.first() {
            row0.extend(nav_row);
        }
        rows.push(row0);

        // Rows 1–5: main block rows + nav cluster rows 1–5
        for i in 0..sections.main_block.len() {
            let mut row = sections.main_block[i].clone();
            if let Some(nav_row) = sections.nav_cluster.get(i + 1) {
                row.extend(nav_row);
            }
            rows.push(row);
        }

        rows
    }

    /// Build a full-size (100%) layout — function row + main block + nav cluster + numpad.
    fn build_full(sections: &LayoutSections) -> KeyGrid {
        let mut rows = KeyGrid::new();

        // Row 0: function row + nav cluster row 0
        let mut row0 = sections.function_row.clone();
        if let Some(nav_row) = sections.nav_cluster.first() {
            row0.extend(nav_row);
        }
        rows.push(row0);

        // Rows 1–5: main block rows + nav cluster rows 1–5 + numpad rows 0–4
        for i in 0..sections.main_block.len() {
            let mut row = sections.main_block[i].clone();
            if let Some(nav_row) = sections.nav_cluster.get(i + 1) {
                row.extend(nav_row);
            }
            if let Some(numpad_row) = sections.numpad.get(i) {
                row.extend(numpad_row);
            }
            rows.push(row);
        }

        rows
    }

    /// Get the default key for a spacer between key blocks.
    pub fn spacer() -> Key {
        Key::spacer(0.5)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::LayoutDefinition;
    use crate::Qwertz;
    use osk_core::XkbLayout;

    #[test]
    fn test_compact60_has_5_rows() {
        let sections = Qwertz::sections();
        let layout = SizeVariantBuilder::build(&sections, SizeVariant::Compact60);
        assert_eq!(layout.rows.len(), 5);
        assert_eq!(layout.size, SizeVariant::Compact60);
    }

    #[test]
    fn test_tkl_has_6_rows() {
        let sections = Qwertz::sections();
        let layout = SizeVariantBuilder::build(&sections, SizeVariant::Tenkeyless80);
        assert_eq!(layout.rows.len(), 6);
        assert_eq!(layout.size, SizeVariant::Tenkeyless80);
    }

    #[test]
    fn test_full_has_6_rows() {
        let sections = Qwertz::sections();
        let layout = SizeVariantBuilder::build(&sections, SizeVariant::Full100);
        assert_eq!(layout.rows.len(), 6);
        assert_eq!(layout.size, SizeVariant::Full100);
    }

    #[test]
    fn test_compact60_has_no_function_row() {
        let sections = Qwertz::sections();
        let layout = SizeVariantBuilder::build(&sections, SizeVariant::Compact60);
        // First row should be the number row, not the function row
        let first_row_labels: Vec<&str> = layout.rows[0].iter().map(|k| k.label.as_str()).collect();
        assert!(!first_row_labels.contains(&"Esc"));
        assert!(first_row_labels.contains(&"1"));
    }

    #[test]
    fn test_tkl_has_function_row() {
        let sections = Qwertz::sections();
        let layout = SizeVariantBuilder::build(&sections, SizeVariant::Tenkeyless80);
        let first_row_labels: Vec<&str> = layout.rows[0].iter().map(|k| k.label.as_str()).collect();
        assert!(first_row_labels.contains(&"Esc"));
        assert!(first_row_labels.contains(&"F12"));
    }

    #[test]
    fn test_tkl_has_nav_keys() {
        let sections = Qwertz::sections();
        let layout = SizeVariantBuilder::build(&sections, SizeVariant::Tenkeyless80);
        let all_keycodes: Vec<_> = layout.rows.iter().flat_map(|row| row.iter().map(|k| k.keycode)).collect();
        assert!(all_keycodes.contains(&osk_core::keycode::KEY_INSERT));
        assert!(all_keycodes.contains(&osk_core::keycode::KEY_DELETE));
        assert!(all_keycodes.contains(&osk_core::keycode::KEY_UP));
    }

    #[test]
    fn test_tkl_has_no_numpad() {
        let sections = Qwertz::sections();
        let layout = SizeVariantBuilder::build(&sections, SizeVariant::Tenkeyless80);
        let all_keycodes: Vec<_> = layout.rows.iter().flat_map(|row| row.iter().map(|k| k.keycode)).collect();
        assert!(!all_keycodes.contains(&osk_core::keycode::KEY_NUMLOCK));
        assert!(!all_keycodes.contains(&osk_core::keycode::KEY_KP1));
    }

    #[test]
    fn test_full_has_numpad() {
        let sections = Qwertz::sections();
        let layout = SizeVariantBuilder::build(&sections, SizeVariant::Full100);
        let all_keycodes: Vec<_> = layout.rows.iter().flat_map(|row| row.iter().map(|k| k.keycode)).collect();
        assert!(all_keycodes.contains(&osk_core::keycode::KEY_NUMLOCK));
        assert!(all_keycodes.contains(&osk_core::keycode::KEY_KP1));
        assert!(all_keycodes.contains(&osk_core::keycode::KEY_KPENTER));
    }

    #[test]
    fn test_layout_name_preserved() {
        let sections = Qwertz::sections();
        let layout = SizeVariantBuilder::build(&sections, SizeVariant::Tenkeyless80);
        assert_eq!(layout.name, XkbLayout::De);
    }
}
