//! Wayland event queue state type.
//!
//! This module provides [`WaylandState`], the state type used by the
//! Wayland event queue. In Phase 1, event handling is done via GTK's
//! main loop, so this type exists only to satisfy `Dispatch` trait
//! requirements.

use wayland_client::Connection;
use wayland_client::Dispatch;
use wayland_client::QueueHandle;
use wayland_client::delegate_noop;
use wayland_client::globals::GlobalListContents;
use wayland_client::protocol::wl_compositor::WlCompositor;
use wayland_client::protocol::wl_registry;
use wayland_client::protocol::wl_registry::WlRegistry;
use wayland_client::protocol::wl_seat::WlSeat;
use wayland_client::protocol::wl_shm::WlShm;
use wayland_protocols_misc::zwp_virtual_keyboard_v1::client::zwp_virtual_keyboard_manager_v1::ZwpVirtualKeyboardManagerV1;
use wayland_protocols_misc::zwp_virtual_keyboard_v1::client::zwp_virtual_keyboard_v1::ZwpVirtualKeyboardV1;
use wayland_protocols_wlr::layer_shell::v1::client::zwlr_layer_shell_v1::ZwlrLayerShellV1;

/// Empty state type for the Wayland event queue.
///
/// Event handling is done via GTK's main loop in Phase 1; this type
/// exists only to satisfy the `Dispatch` trait requirements.
#[derive(Default)]
pub struct WaylandState;

delegate_noop!(WaylandState: WlCompositor);
delegate_noop!(WaylandState: WlSeat);
delegate_noop!(WaylandState: WlShm);
delegate_noop!(WaylandState: ZwlrLayerShellV1);
delegate_noop!(WaylandState: ZwpVirtualKeyboardManagerV1);
delegate_noop!(WaylandState: ZwpVirtualKeyboardV1);

impl Dispatch<WlRegistry, GlobalListContents> for WaylandState {
    fn event(
        _state: &mut Self,
        _proxy: &WlRegistry,
        _event: wl_registry::Event,
        _data: &GlobalListContents,
        _conn: &Connection,
        _qh: &QueueHandle<WaylandState>,
    ) {
    }
}
