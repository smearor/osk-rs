//! Errors that can occur during configuration operations.

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
