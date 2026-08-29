//! OSK IPC service stub.

/// Stub for the OSK IPC service.
///
/// In Phase 1, this will provide a D-Bus interface and Unix socket server
/// for runtime control of the keyboard (show, hide, toggle, set layout, etc.).
pub struct OskIpc;

impl OskIpc {
    /// Create a new IPC service stub.
    pub fn new() -> Self {
        Self
    }
}

impl Default for OskIpc {
    fn default() -> Self {
        Self::new()
    }
}
