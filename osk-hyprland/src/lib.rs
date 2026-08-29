//! Hyprland IPC client for `osk-rs` — layout detection, config read, workspace events.
//!
//! This crate communicates with Hyprland via its IPC sockets:
//! - `~/.hypr/.socket.sock` for commands
//! - `~/.hypr/.socket2.sock` for events

mod error;
mod ipc;

pub use error::HyprlandError;
pub use ipc::HyprlandIpc;
