//! Configuration file watcher with hot-reload support.
//!
//! Uses the `notify` crate to watch the config file for changes and
//! reloads the configuration when modifications are detected.

use crate::Config;
use crate::ConfigError;
use notify::EventKind;
use notify::RecommendedWatcher;
use notify::RecursiveMode;
use notify::Watcher;
use std::path::PathBuf;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use tracing::debug;
use tracing::info;
use tracing::warn;

/// Configuration file watcher with hot-reload support.
///
/// Watches the config file at the system config directory
/// (e.g. `~/.config/osk-rs/config.toml` on Linux) for changes
/// and reloads the configuration when modifications are detected.
pub struct ConfigWatcher {
    /// The underlying notify watcher
    watcher: RecommendedWatcher,
    /// Channel for receiving file change events
    receiver: Receiver<notify::Result<notify::Event>>,
    /// Path to the config file being watched
    config_path: PathBuf,
}

impl ConfigWatcher {
    /// Create a new `ConfigWatcher` watching the default config path.
    ///
    /// # Errors
    ///
    /// Returns `ConfigError` if the watcher cannot be created or the
    /// config file path cannot be determined.
    pub fn new() -> Result<Self, ConfigError> {
        let config_dir = dirs::config_dir().ok_or(ConfigError::ConfigDirNotFound)?;
        let path = config_dir.join("osk-rs").join("config.toml");
        Self::watching(path.to_str().unwrap_or(""))
    }

    /// Create a new `ConfigWatcher` watching a specific config file path.
    ///
    /// # Errors
    ///
    /// Returns `ConfigError` if the watcher cannot be created.
    pub fn watching(config_path: &str) -> Result<Self, ConfigError> {
        let config_path = PathBuf::from(config_path);
        let (sender, receiver) = mpsc::channel();

        let mut watcher = notify::recommended_watcher(move |res: notify::Result<notify::Event>| {
            if let Err(e) = sender.send(res) {
                warn!("Config watcher channel send error: {e}");
            }
        })?;

        // Watch the parent directory (the file itself may not exist yet)
        if let Some(parent) = config_path.parent() {
            if parent.exists() {
                watcher.watch(parent, RecursiveMode::NonRecursive)?;
                debug!("Watching config directory: {}", parent.display());
            } else {
                warn!("Config directory does not exist: {}", parent.display());
            }
        }

        Ok(Self {
            watcher,
            receiver,
            config_path,
        })
    }

    /// Poll for config changes and return the new config if changed.
    ///
    /// This method is non-blocking. It checks for pending file change
    /// events and, if the config file was modified, reloads the config.
    ///
    /// # Errors
    ///
    /// Returns `ConfigError` if the changed config file cannot be parsed.
    pub fn poll_reload(&self) -> Result<Option<Config>, ConfigError> {
        let mut changed = false;

        while let Ok(result) = self.receiver.try_recv() {
            match result {
                Ok(event) => {
                    if Self::is_config_change(&event, &self.config_path) {
                        changed = true;
                    }
                }
                Err(e) => {
                    warn!("Config watcher error: {e}");
                }
            }
        }

        if changed {
            info!("Config file changed, reloading...");
            // Small delay to let the file write complete
            std::thread::sleep(std::time::Duration::from_millis(50));
            let config = Config::load_from(self.config_path.to_str().unwrap_or(""))?;
            Ok(Some(config))
        } else {
            Ok(None)
        }
    }

    /// Check if a notify event corresponds to a change in the config file.
    fn is_config_change(event: &notify::Event, config_path: &std::path::Path) -> bool {
        let is_relevant = matches!(event.kind, EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_));
        if !is_relevant {
            return false;
        }
        event.paths.iter().any(|p| p == config_path)
    }

    /// Return the path to the config file being watched.
    pub fn config_path(&self) -> &std::path::Path {
        &self.config_path
    }
}

impl Drop for ConfigWatcher {
    fn drop(&mut self) {
        // Explicitly stop the watcher to clean up resources
        if let Err(e) = self.watcher.unwatch(&self.config_path) {
            debug!("Config watcher unwatch error (expected during drop): {e}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_config_watcher_creation() {
        let temp_dir = std::env::temp_dir().join("osk_config_watcher_test");
        std::fs::create_dir_all(&temp_dir).unwrap();
        let config_path = temp_dir.join("config.toml");

        // Create a minimal config file
        std::fs::write(&config_path, "").unwrap();

        let watcher = ConfigWatcher::watching(config_path.to_str().unwrap());
        assert!(watcher.is_ok());

        // Cleanup
        std::fs::remove_file(&config_path).ok();
        std::fs::remove_dir(&temp_dir).ok();
    }

    #[test]
    fn test_poll_reload_no_change() {
        let temp_dir = std::env::temp_dir().join("osk_config_poll_test");
        std::fs::create_dir_all(&temp_dir).unwrap();
        let config_path = temp_dir.join("config.toml");
        std::fs::write(&config_path, "").unwrap();

        let watcher = ConfigWatcher::watching(config_path.to_str().unwrap()).unwrap();
        let result = watcher.poll_reload().unwrap();
        assert!(result.is_none());

        // Cleanup
        std::fs::remove_file(&config_path).ok();
        std::fs::remove_dir(&temp_dir).ok();
    }

    #[test]
    fn test_poll_reload_with_change() {
        let temp_dir = std::env::temp_dir().join("osk_config_reload_test");
        std::fs::create_dir_all(&temp_dir).unwrap();
        let config_path = temp_dir.join("config.toml");
        std::fs::write(&config_path, "").unwrap();

        let watcher = ConfigWatcher::watching(config_path.to_str().unwrap()).unwrap();

        // Wait a moment for the watcher to be ready
        std::thread::sleep(std::time::Duration::from_millis(100));

        // Modify the config file
        let mut file = std::fs::File::create(&config_path).unwrap();
        writeln!(file, "[layout]").unwrap();
        writeln!(file, "name = \"us\"").unwrap();
        drop(file);

        // Wait for the event to propagate
        std::thread::sleep(std::time::Duration::from_millis(200));

        let result = watcher.poll_reload().unwrap();
        assert!(result.is_some());
        let config = result.unwrap();
        assert_eq!(config.layout.name, Some(osk_core::XkbLayout::Us));

        // Cleanup
        std::fs::remove_file(&config_path).ok();
        std::fs::remove_dir(&temp_dir).ok();
    }
}
