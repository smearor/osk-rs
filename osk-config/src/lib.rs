//! TOML configuration, hot-reload, and CLI args for `osk-rs`.
//!
//! This crate handles loading and hot-reloading the configuration file
//! at `~/.config/osk-rs/config.toml`, as well as parsing CLI overrides
//! via `clap`.

use clap::Parser;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Errors that can occur during configuration operations.
#[derive(Debug, Error)]
pub enum ConfigError {
    /// The configuration file could not be read
    #[error("config read error: {0}")]
    ReadError(#[from] std::io::Error),
    /// The configuration file could not be parsed as valid TOML
    #[error("config parse error: {0}")]
    ParseError(#[from] toml::de::Error),
    /// A configuration value is invalid
    #[error("invalid config value: {0}")]
    InvalidValue(String),
}

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

/// CLI argument structure for `osk-rs`.
///
/// CLI arguments override configuration file values at runtime.
#[derive(Debug, Clone, Parser)]
#[command(name = "osk-rs", about = "Wayland on-screen keyboard")]
pub struct Cli {
    /// Override the keyboard layout (e.g. "de", "us")
    #[arg(long)]
    pub layout: Option<String>,

    /// Override the keyboard size variant ("compact", "tkl", "full")
    #[arg(long)]
    pub size: Option<String>,

    /// Override the key scale factor
    #[arg(long)]
    pub scale: Option<f32>,

    /// Override the display mode ("full", "split", "floating")
    #[arg(long)]
    pub mode: Option<String>,

    /// Enable verbose tracing output
    #[arg(long)]
    pub verbose: bool,
}
