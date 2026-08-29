//! LayoutDetector trait and auto-detection backend selection for `osk-rs`.
//!
//! This crate provides a trait abstraction for layout detection so that
//! different backends (Hyprland IPC, environment variables, config fallback)
//! can be selected at runtime.

use thiserror::Error;

/// Errors that can occur during layout detection.
#[derive(Debug, Error)]
pub enum DetectError {
    /// No layout detector backend is available
    #[error("no layout detector available")]
    NoDetectorAvailable,
    /// The layout detection query failed
    #[error("detection failed: {0}")]
    DetectionFailed(String),
}

/// Trait for layout detection backends.
///
/// Implementations query the active desktop environment for the current
/// keyboard layout and optionally watch for changes.
pub trait LayoutDetector {
    /// Detect the current keyboard layout name.
    ///
    /// # Errors
    ///
    /// Returns `DetectError` if the layout cannot be detected.
    fn detect(&self) -> Result<String, DetectError>;

    /// Check if this detector backend is available on the current system.
    fn is_available(&self) -> bool;
}

/// Auto-detector that selects the best available backend at runtime.
///
/// Tries backends in order: Hyprland IPC, environment variables, config fallback.
pub struct AutoDetector;

impl LayoutDetector for AutoDetector {
    fn detect(&self) -> Result<String, DetectError> {
        // TODO: Implement in Phase 3
        Err(DetectError::NoDetectorAvailable)
    }

    fn is_available(&self) -> bool {
        false
    }
}
