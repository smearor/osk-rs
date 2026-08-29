//! Wayland client for `osk-rs` — layer-shell surface, registry bind, seat handling.
//!
//! This crate manages the Wayland connection by reusing GTK 4's native
//! `wl_display` via `gdk4-wayland` FFI. No separate event queue is created.

mod client;
mod context;
mod error;
mod protocol;
mod state;

pub use client::WaylandClient;
pub use context::WaylandContext;
pub use error::WaylandError;
pub use protocol::WaylandProtocol;
pub use state::WaylandState;
