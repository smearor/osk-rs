//! TOML configuration, hot-reload, and CLI args for `osk-rs`.
//!
//! This crate handles loading and hot-reloading the configuration file
//! at `~/.config/osk-rs/config.toml`, as well as parsing CLI overrides
//! via `clap`.

mod behavior_config;
mod cli;
mod config;
mod display_config;
mod error;
mod layout_config;

pub use behavior_config::BehaviorConfig;
pub use cli::Cli;
pub use config::Config;
pub use display_config::DisplayConfig;
pub use error::ConfigError;
pub use layout_config::LayoutConfig;
