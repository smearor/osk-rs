//! Errors that can occur during layout detection.

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
