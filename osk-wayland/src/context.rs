//! Wayland context managing protocol bindings for the on-screen keyboard.
//!
//! This module provides [`WaylandContext`] which connects to the Wayland
//! display and binds the required protocol globals: `wl_compositor`,
//! `wl_seat`, `wl_shm`, `zwlr_layer_shell_v1`, and
//! `zwp_virtual_keyboard_manager_v1`.

use crate::error::WaylandError;
use crate::protocol::WaylandProtocol;
use crate::state::WaylandState;
use std::sync::Arc;
use wayland_client::Connection;
use wayland_client::QueueHandle;
use wayland_client::globals::GlobalList;
use wayland_client::globals::registry_queue_init;
use wayland_client::protocol::wl_compositor::WlCompositor;
use wayland_client::protocol::wl_seat::WlSeat;
use wayland_client::protocol::wl_shm::WlShm;
use wayland_protocols_misc::zwp_virtual_keyboard_v1::client::zwp_virtual_keyboard_manager_v1::ZwpVirtualKeyboardManagerV1;
use wayland_protocols_wlr::layer_shell::v1::client::zwlr_layer_shell_v1::ZwlrLayerShellV1;

/// Wayland context holding all bound globals required by the on-screen keyboard.
///
/// This struct is created by connecting to the Wayland display and binding
/// the required protocol globals from the registry.
pub struct WaylandContext {
    /// Compositor for creating surfaces
    pub compositor: WlCompositor,
    /// Seat for input handling
    pub seat: WlSeat,
    /// Shared memory manager for buffers
    pub shm: WlShm,
    /// Layer shell for overlay positioning
    pub layer_shell: ZwlrLayerShellV1,
    /// Virtual keyboard manager for creating virtual keyboard devices
    pub vkbd_manager: ZwpVirtualKeyboardManagerV1,
    /// The Wayland connection
    pub connection: Connection,
    /// The event queue handle
    pub queue_handle: QueueHandle<WaylandState>,
    /// The global list contents for late binding
    pub globals: Arc<GlobalList>,
}

impl WaylandContext {
    /// Connect to the Wayland display and bind required globals.
    ///
    /// # Errors
    ///
    /// Returns `WaylandError` if the connection fails or a required
    /// protocol is not available in the registry.
    pub fn connect() -> Result<Self, WaylandError> {
        let connection = Connection::connect_to_env().map_err(|e| WaylandError::ConnectionFailed(e.to_string()))?;

        let (globals, queue) = registry_queue_init::<WaylandState>(&connection).map_err(|e| WaylandError::ConnectionFailed(e.to_string()))?;

        let queue_handle = queue.handle();

        let compositor = globals
            .bind::<WlCompositor, _, _>(&queue_handle, 1..=1, ())
            .map_err(|_| WaylandError::ProtocolNotFound(WaylandProtocol::WlCompositor))?;

        let seat = globals
            .bind::<WlSeat, _, _>(&queue_handle, 1..=7, ())
            .map_err(|_| WaylandError::ProtocolNotFound(WaylandProtocol::WlSeat))?;

        let shm = globals
            .bind::<WlShm, _, _>(&queue_handle, 1..=1, ())
            .map_err(|_| WaylandError::ProtocolNotFound(WaylandProtocol::WlShm))?;

        let layer_shell = globals
            .bind::<ZwlrLayerShellV1, _, _>(&queue_handle, 1..=4, ())
            .map_err(|_| WaylandError::ProtocolNotFound(WaylandProtocol::ZwlrLayerShellV1))?;

        let vkbd_manager = globals
            .bind::<ZwpVirtualKeyboardManagerV1, _, _>(&queue_handle, 1..=1, ())
            .map_err(|_| WaylandError::ProtocolNotFound(WaylandProtocol::ZwpVirtualKeyboardManagerV1))?;

        Ok(Self {
            compositor,
            seat,
            shm,
            layer_shell,
            vkbd_manager,
            connection,
            queue_handle,
            globals: Arc::new(globals),
        })
    }
}
