//! `osk-rs` on-screen keyboard application.
//!
//! This binary connects all `osk-rs` modules: configuration, layout,
//! input, Wayland, Hyprland integration, UI rendering, and IPC.

mod app;

fn main() {
    app::run();
}
