//! Behavior configuration section.

use serde::Deserialize;
use serde::Serialize;

/// Behavior configuration section.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BehaviorConfig {
    /// Whether to auto-show the keyboard when a text field is focused
    #[serde(default)]
    pub auto_show: bool,
    /// Whether to auto-hide the keyboard when focus leaves a text field
    #[serde(default)]
    pub auto_hide: bool,
}
