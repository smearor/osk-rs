//! Input type hint from `zwp_input_method_v2` content_type.

use serde::Deserialize;
use serde::Serialize;

/// Input type hint from `zwp_input_method_v2` content_type.
///
/// The OSK adapts its layout based on this hint to show the most relevant
/// keys for the focused input field.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum InputType {
    /// Standard text input — no special adaptation
    #[default]
    Text,
    /// Email input — standard layout with @ and domain shortcuts
    Email,
    /// Numeric input — numpad-only layout
    Number,
    /// Telephone number input — numpad-only layout
    Tel,
    /// Password input — no predictive text, no clipboard, AT-SPI muted
    Password,
    /// URL input — standard layout with URL-relevant keys
    Url,
    /// Digit input — numpad-only layout
    Digits,
    /// Emoji input — emoji selector grid
    Emoji,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_type_default_is_text() {
        assert_eq!(InputType::default(), InputType::Text);
    }
}
