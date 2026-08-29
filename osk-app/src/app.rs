//! Application entry point for `osk-rs`.

use clap::Parser;
use osk_config::Cli;

/// Run the `osk-rs` application.
pub fn run() {
    let cli = Cli::parse();

    if cli.verbose {
        tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).init();
    } else {
        tracing_subscriber::fmt().with_max_level(tracing::Level::INFO).init();
    }

    tracing::info!("osk-rs starting up");

    // TODO: Implement in Phase 1
    // - Load configuration
    // - Initialize Wayland client
    // - Create layer-shell surface
    // - Set up virtual keyboard
    // - Render keyboard UI
    // - Start IPC service

    tracing::info!("osk-rs shutting down");
}
