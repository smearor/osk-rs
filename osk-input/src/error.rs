//! Errors that can occur during virtual keyboard operations.

use thiserror::Error;

/// Errors that can occur during virtual keyboard operations.
#[derive(Debug, Error)]
pub enum InputError {
    /// The virtual keyboard protocol is not available on this compositor
    #[error("virtual keyboard manager not found in registry")]
    ManagerNotFound,
    /// The keymap could not be published to the compositor
    #[error("keymap publication failed: {0}")]
    KeymapPublicationFailed(String),
    /// A key event was sent with an invalid keycode
    #[error("invalid keycode: {0}")]
    InvalidKeycode(u32),
}
