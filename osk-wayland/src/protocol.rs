//! Wayland protocols required by the on-screen keyboard.
//!
//! Each variant of [`WaylandProtocol`] corresponds to a Wayland global
//! that must be present in the registry for the OSK to function.

/// Wayland protocols required by the on-screen keyboard.
///
/// Each variant corresponds to a Wayland global that must be present
/// in the registry for the OSK to function.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WaylandProtocol {
    /// `wl_compositor` — surface creation
    WlCompositor,
    /// `wl_seat` — input handling
    WlSeat,
    /// `wl_shm` — shared memory buffers
    WlShm,
    /// `zwlr_layer_shell_v1` — layer-shell overlay positioning
    ZwlrLayerShellV1,
    /// `zwp_virtual_keyboard_manager_v1` — virtual keyboard device creation
    ZwpVirtualKeyboardManagerV1,
}

impl std::fmt::Display for WaylandProtocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WlCompositor => write!(f, "wl_compositor"),
            Self::WlSeat => write!(f, "wl_seat"),
            Self::WlShm => write!(f, "wl_shm"),
            Self::ZwlrLayerShellV1 => write!(f, "zwlr_layer_shell_v1"),
            Self::ZwpVirtualKeyboardManagerV1 => write!(f, "zwp_virtual_keyboard_manager_v1"),
        }
    }
}
