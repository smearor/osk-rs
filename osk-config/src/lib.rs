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
mod key_scale;
mod keyboard_height;
mod layout_config;

pub use behavior_config::BehaviorConfig;
pub use cli::Cli;
pub use config::Config;
pub use display_config::DisplayConfig;
pub use error::ConfigError;
pub use key_scale::KeyScale;
pub use keyboard_height::KeyboardHeight;
pub use layout_config::LayoutConfig;
