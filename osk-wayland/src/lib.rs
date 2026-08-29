//! Wayland client for `osk-rs` — layer-shell surface, registry bind, seat handling.
//!
//! This crate manages the Wayland connection by reusing GTK 4's native
//! `wl_display` via `gdk4-wayland` FFI. No separate event queue is created.

mod client;
mod error;

pub use client::WaylandClient;
pub use error::WaylandError;
