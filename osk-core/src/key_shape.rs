//! Geometric shape of a key on the keyboard grid.

use serde::Deserialize;
use serde::Serialize;

/// Geometric shape of a key on the keyboard grid.
///
/// Most keys are simple rectangles with a width in grid units. The ISO Enter
/// key has an L-shape that spans two rows. Tall keys (e.g. numpad +/Enter)
/// are simple rectangles spanning multiple rows.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum KeyShape {
    /// Rectangular key with a width in grid units (1u = one standard key width)
    Rect {
        /// Width in grid units (e.g. 1.0 for a standard key, 2.0 for a wide key)
        width_u: f32,
    },
    /// L-shaped key (ISO Enter) spanning two rows
    LShape {
        /// Width of the upper part in grid units
        top_width_u: f32,
        /// Width of the lower part in grid units
        bottom_width_u: f32,
    },
    /// Tall rectangular key spanning multiple rows (e.g. numpad +/Enter)
    Tall {
        /// Width in grid units
        width_u: f32,
        /// Number of grid rows this key spans
        height_rows: u32,
    },
}

impl Default for KeyShape {
    fn default() -> Self {
        KeyShape::Rect { width_u: 1.0 }
    }
}

impl KeyShape {
    /// Returns the width in grid units of the widest part of this shape.
    pub fn width_u(&self) -> f32 {
        match self {
            KeyShape::Rect { width_u } => *width_u,
            KeyShape::LShape { top_width_u, bottom_width_u } => top_width_u.max(*bottom_width_u),
            KeyShape::Tall { width_u, .. } => *width_u,
        }
    }

    /// Returns the number of grid rows this shape spans.
    pub fn height_rows(&self) -> u32 {
        match self {
            KeyShape::Rect { .. } => 1,
            KeyShape::LShape { .. } => 2,
            KeyShape::Tall { height_rows, .. } => *height_rows,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_shape_rect_serialization() {
        let shape = KeyShape::Rect { width_u: 1.5 };
        let json = serde_json::to_string(&shape).unwrap();
        let deserialized: KeyShape = serde_json::from_str(&json).unwrap();
        assert_eq!(shape, deserialized);
    }
}
