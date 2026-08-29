//! Errors that can occur during UI operations.

use thiserror::Error;

/// Errors that can occur during UI operations.
#[derive(Debug, Error)]
pub enum UiError {
    /// The GTK theme could not be loaded
    #[error("theme load failed: {0}")]
    ThemeLoadFailed(String),
    /// A widget rendering error occurred
    #[error("rendering error: {0}")]
    RenderingError(String),
}
