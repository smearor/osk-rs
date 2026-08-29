//! Display configuration section.

use serde::{Deserialize, Serialize};

/// Display configuration section.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DisplayConfig {
    /// Size variant: "compact", "tkl", "full"
    pub size: Option<String>,
    /// Key scale factor
    pub scale: Option<f32>,
    /// Display mode: "full", "split", "floating"
    pub mode: Option<String>,
    /// GTK CSS theme name
    pub theme: Option<String>,
}
