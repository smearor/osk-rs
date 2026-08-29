//! TOML configuration, hot-reload, and CLI args for `osk-rs`.
//!
//! This crate handles loading and hot-reloading the configuration file
//! at the system config directory (e.g. `~/.config/osk-rs/config.toml`
//! on Linux), as well as parsing CLI overrides via `clap`.

mod behavior_config;
mod cli;
mod config;
mod config_watcher;
mod display_config;
mod error;
mod key_scale;
mod keyboard_height;
mod layout_config;

pub use behavior_config::BehaviorConfig;
pub use cli::Cli;
pub use config::Config;
pub use config_watcher::ConfigWatcher;
pub use display_config::DisplayConfig;
pub use error::ConfigError;
pub use key_scale::KeyScale;
pub use keyboard_height::KeyboardHeight;
pub use layout_config::LayoutConfig;
