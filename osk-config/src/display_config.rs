//! Display configuration section.

use crate::KeyScale;
use crate::KeyboardHeight;
use serde::Deserialize;
use serde::Serialize;

/// Display configuration section.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DisplayConfig {
    /// Size variant: "compact", "tkl", "full"
    pub size: Option<String>,
    /// Key scale factor (1.0 = 100%)
    #[serde(default)]
    pub scale: KeyScale,
    /// Display mode: "full", "split", "floating"
    pub mode: Option<String>,
    /// GTK CSS theme name
    pub theme: Option<String>,
    /// Keyboard height as percentage of screen height (default 40%)
    #[serde(default)]
    pub height_percent: KeyboardHeight,
}
