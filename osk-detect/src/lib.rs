//! LayoutDetector trait and auto-detection backend selection for `osk-rs`.
//!
//! This crate provides a trait abstraction for layout detection so that
//! different backends (Hyprland IPC, environment variables, config fallback)
//! can be selected at runtime.

mod auto_detector;
mod error;
mod layout_detector;

pub use auto_detector::AutoDetector;
pub use error::DetectError;
pub use layout_detector::LayoutDetector;
