//! Errors that can occur during layout parsing or building.

use thiserror::Error;

/// Errors that can occur during layout parsing or building.
#[derive(Debug, Error)]
pub enum LayoutError {
    /// The requested layout name was not found in the XKB database
    #[error("layout not found: {0}")]
    LayoutNotFound(String),
    /// The requested variant was not found for the given layout
    #[error("variant not found: {0} for layout {1}")]
    VariantNotFound(String, String),
    /// The XKB keymap could not be compiled from the given RMLVO
    #[error("keymap compilation failed: {0}")]
    KeymapCompilationFailed(String),
}
