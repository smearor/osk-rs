//! Auto-detector that selects the best available backend at runtime.

use crate::{DetectError, LayoutDetector};

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
