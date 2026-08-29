//! Root configuration structure for `osk-rs`.

use crate::BehaviorConfig;
use crate::ConfigError;
use crate::DisplayConfig;
use crate::LayoutConfig;
use serde::Deserialize;
use serde::Serialize;

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
    /// Load configuration from the default config path.
    ///
    /// Uses the system config directory (e.g. `~/.config/osk-rs/config.toml`
    /// on Linux) as determined by the `dirs` crate.
    ///
    /// # Errors
    ///
    /// Returns `ConfigError` if the config directory cannot be determined,
    /// or the file cannot be read or parsed.
    pub fn load() -> Result<Self, ConfigError> {
        let config_dir = dirs::config_dir()
            .ok_or(ConfigError::ConfigDirNotFound)?;
        let path = config_dir.join("osk-rs").join("config.toml");
        Self::load_from(path.to_str().unwrap_or(""))
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
