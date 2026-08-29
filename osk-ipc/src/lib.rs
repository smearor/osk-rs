//! D-Bus session bus service, Unix socket server, and signal handling for `osk-rs`.
//!
//! This crate provides IPC interfaces for controlling the OSK at runtime:
//! - D-Bus session bus service (`org.example.OSK`)
//! - Unix domain socket server for CLI commands
//! - POSIX signal handling (SIGUSR1 for toggle, SIGTERM for shutdown)

mod error;
mod ipc;

pub use error::IpcError;
pub use ipc::OskIpc;
