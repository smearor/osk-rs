//! Trait for layout detection backends.

use crate::DetectError;

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
