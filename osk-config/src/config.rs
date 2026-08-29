//! Root configuration structure for `osk-rs`.

use serde::{Deserialize, Serialize};

use crate::{BehaviorConfig, ConfigError, DisplayConfig, LayoutConfig};

/// Root configuration structure for `osk-rs`.
///
/// Loaded from `~/.config/osk-rs/config.toml` with CLI overrides applied.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
    /// Layout configuration section
    #[serde(default)]
    pub layout: LayoutConfig,
    /// Display configuration section
    #[serde(default)]
    pub display: DisplayConfig,
    /// Behavior configuration section
    #[serde(default)]
    pub behavior: BehaviorConfig,
}

impl Config {
    /// Load configuration from the default path (`~/.config/osk-rs/config.toml`).
    ///
    /// # Errors
    ///
    /// Returns `ConfigError` if the file cannot be read or parsed.
    pub fn load() -> Result<Self, ConfigError> {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
        let path = format!("{home}/.config/osk-rs/config.toml");
        Self::load_from(&path)
    }

    /// Load configuration from a specific file path.
    ///
    /// # Errors
    ///
    /// Returns `ConfigError` if the file cannot be read or parsed.
    pub fn load_from(path: &str) -> Result<Self, ConfigError> {
        let content = std::fs::read_to_string(path)?;
        let config: Self = toml::from_str(&content)?;
        Ok(config)
    }
}
