//! Layout configuration section.

use osk_core::SizeVariant;
use osk_core::XkbLayout;
use osk_core::XkbVariant;
use serde::Deserialize;
use serde::Serialize;

/// Layout configuration section.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LayoutConfig {
    /// XKB layout name
    pub name: Option<XkbLayout>,
    /// XKB variant
    pub variant: Option<XkbVariant>,
    /// Size variant
    pub size: Option<SizeVariant>,
    /// Whether to auto-detect the layout from the desktop environment
    #[serde(default)]
    pub auto_detect: bool,
}
