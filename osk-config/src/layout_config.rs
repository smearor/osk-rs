//! Layout configuration section.

use serde::Deserialize;
use serde::Serialize;

/// Layout configuration section.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LayoutConfig {
    /// XKB layout name (e.g. "de", "us", "fr")
    pub name: Option<String>,
    /// XKB variant (e.g. "nodeadkeys", "intl")
    pub variant: Option<String>,
    /// Whether to auto-detect the layout from the desktop environment
    #[serde(default)]
    pub auto_detect: bool,
}
