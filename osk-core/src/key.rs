//! A single key definition in the keyboard grid.

use crate::CustomWidget;
use crate::KeyCode;
use crate::KeyShape;
use crate::KeyType;
use serde::Deserialize;
use serde::Serialize;

/// A single key definition in the keyboard grid.
///
/// Each key has a Linux evdev keycode, a display label, a geometric shape,
/// a semantic type that determines how it behaves when pressed, an
/// optional CSS class for styling, and an optional custom widget type
/// for rich rendering (color picker, image, piano, etc.).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Key {
    /// Linux evdev keycode for this key
    pub keycode: KeyCode,
    /// Display label shown on the key cap
    pub label: String,
    /// Geometric shape of the key (rectangular, L-shaped, or tall)
    pub shape: KeyShape,
    /// Semantic type of the key (alpha, modifier, special, custom)
    pub key_type: KeyType,
    /// Optional CSS class for custom styling
    #[serde(default)]
    pub css_class: Option<String>,
    /// Optional custom widget for rich rendering (color picker, image, piano)
    #[serde(default)]
    pub custom: CustomWidget,
}

impl Key {
    /// Create a character key with a standard 1u rectangular shape.
    pub fn char(label: &str, keycode: KeyCode) -> Self {
        Self {
            keycode,
            label: label.to_string(),
            shape: KeyShape::Rect { width_u: 1.0 },
            key_type: KeyType::Character,
            css_class: None,
            custom: CustomWidget::None,
        }
    }

    /// Create a character key with a custom width.
    pub fn char_w(label: &str, keycode: KeyCode, width_u: f32) -> Self {
        Self {
            keycode,
            label: label.to_string(),
            shape: KeyShape::Rect { width_u },
            key_type: KeyType::Character,
            css_class: None,
            custom: CustomWidget::None,
        }
    }

    /// Create a modifier key with a custom width.
    pub fn modifier(label: &str, keycode: KeyCode, width_u: f32) -> Self {
        Self {
            keycode,
            label: label.to_string(),
            shape: KeyShape::Rect { width_u },
            key_type: KeyType::Modifier,
            css_class: Some("key-modifier".to_string()),
            custom: CustomWidget::None,
        }
    }

    /// Create a special key with a custom width.
    pub fn special(label: &str, keycode: KeyCode, width_u: f32) -> Self {
        Self {
            keycode,
            label: label.to_string(),
            shape: KeyShape::Rect { width_u },
            key_type: KeyType::Special,
            css_class: Some("key-special".to_string()),
            custom: CustomWidget::None,
        }
    }

    /// Create a function key (F1–F12) with a standard 1u shape.
    pub fn func(label: &str, keycode: KeyCode) -> Self {
        Self {
            keycode,
            label: label.to_string(),
            shape: KeyShape::Rect { width_u: 1.0 },
            key_type: KeyType::Special,
            css_class: Some("key-fn".to_string()),
            custom: CustomWidget::None,
        }
    }

    /// Create a spacer key (invisible, used for alignment).
    pub fn spacer(width_u: f32) -> Self {
        Self {
            keycode: KeyCode::new(0),
            label: String::new(),
            shape: KeyShape::Rect { width_u },
            key_type: KeyType::Special,
            css_class: Some("key-spacer".to_string()),
            custom: CustomWidget::None,
        }
    }

    /// Create an ISO L-shaped Enter key.
    pub fn l_enter(label: &str, keycode: KeyCode, top_width_u: f32, bottom_width_u: f32) -> Self {
        Self {
            keycode,
            label: label.to_string(),
            shape: KeyShape::LShape { top_width_u, bottom_width_u },
            key_type: KeyType::Special,
            css_class: Some("key-enter".to_string()),
            custom: CustomWidget::None,
        }
    }

    /// Create a tall key spanning multiple grid rows (e.g. numpad +/Enter).
    pub fn tall(label: &str, keycode: KeyCode, width_u: f32, height_rows: u32) -> Self {
        Self {
            keycode,
            label: label.to_string(),
            shape: KeyShape::Tall { width_u, height_rows },
            key_type: KeyType::Special,
            css_class: Some("key-tall".to_string()),
            custom: CustomWidget::None,
        }
    }

    /// Set a custom widget for this key.
    pub fn with_custom(mut self, widget: CustomWidget) -> Self {
        self.custom = widget;
        self
    }
}
