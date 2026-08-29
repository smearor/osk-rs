//! CLI argument structure for `osk-rs`.

use clap::Parser;

/// CLI argument structure for `osk-rs`.
///
/// CLI arguments override configuration file values at runtime.
#[derive(Debug, Clone, Parser)]
#[command(name = "osk-rs", about = "Wayland on-screen keyboard")]
pub struct Cli {
    /// Override the keyboard layout (e.g. "de", "us")
    #[arg(long)]
    pub layout: Option<String>,

    /// Override the keyboard size variant ("compact", "tkl", "full")
    #[arg(long)]
    pub size: Option<String>,

    /// Override the key scale factor
    #[arg(long)]
    pub scale: Option<f32>,

    /// Override the display mode ("full", "split", "floating")
    #[arg(long)]
    pub mode: Option<String>,

    /// Enable verbose tracing output
    #[arg(long)]
    pub verbose: bool,
}
