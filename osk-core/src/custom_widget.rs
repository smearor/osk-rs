//! Custom widget type for special key rendering.

use serde::Deserialize;
use serde::Serialize;

/// Custom widget type that can be rendered instead of a standard key label.
///
/// The `Custom` variant carries a loose-coupling identifier string that
/// the UI layer interprets to decide how to render the key.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub enum CustomWidget {
    /// No custom widget — render as a standard key
    #[default]
    None,
    /// Custom widget identified by a loose-coupling string (e.g. "color_picker", "piano")
    Custom(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn custom_widget_default_is_none() {
        assert_eq!(CustomWidget::default(), CustomWidget::None);
    }

    #[test]
    fn custom_widget_serialization() {
        let widget = CustomWidget::Custom("color_picker".to_string());
        let json = serde_json::to_string(&widget).unwrap();
        let deserialized: CustomWidget = serde_json::from_str(&json).unwrap();
        assert_eq!(widget, deserialized);
    }
}
