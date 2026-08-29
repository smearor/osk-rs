//! Virtual keyboard protocol client for `zwp_virtual_keyboard_v1`.

use std::ffi::CString;
use std::os::fd::{AsFd, FromRawFd, OwnedFd};
use std::ptr;

use libc::{MAP_SHARED, PROT_READ, close, ftruncate, mmap, munmap};
use osk_core::{KeyCode, KeyState};
use wayland_protocols_misc::zwp_virtual_keyboard_v1::client::zwp_virtual_keyboard_v1::ZwpVirtualKeyboardV1;

use crate::{InputError, ModifierState};

/// Trait for virtual keyboard protocol clients.
///
/// Implementations send key events and modifier state to the compositor
/// via `zwp_virtual_keyboard_v1`.
pub trait VirtualKeyboard {
    /// Publish the XKB keymap string to the compositor.
    ///
    /// # Errors
    ///
    /// Returns `InputError` if the keymap cannot be published.
    fn publish_keymap(&mut self, keymap_str: &str) -> Result<(), InputError>;

    /// Send a key event to the compositor.
    ///
    /// # Errors
    ///
    /// Returns `InputError` if the key event cannot be sent.
    fn send_key(&mut self, keycode: KeyCode, state: KeyState) -> Result<(), InputError>;

    /// Send the current modifier state to the compositor.
    ///
    /// # Errors
    ///
    /// Returns `InputError` if the modifier state cannot be sent.
    fn send_modifiers(&mut self, modifiers: &ModifierState) -> Result<(), InputError>;
}

/// Concrete virtual keyboard client wrapping `zwp_virtual_keyboard_v1`.
///
/// Created from a `wl_seat` and `zwp_virtual_keyboard_manager_v1`.
/// Manages keymap publication, key event sending, and modifier state.
pub struct WaylandVirtualKeyboard {
    /// The Wayland virtual keyboard protocol object
    vkbd: ZwpVirtualKeyboardV1,
    /// File descriptor for the published keymap (kept alive)
    _keymap_fd: Option<OwnedFd>,
    /// Current modifier state
    pub modifier_state: ModifierState,
    /// Serial counter for key events
    serial: u32,
}

impl WaylandVirtualKeyboard {
    /// Create a virtual keyboard from an existing protocol object.
    pub fn from_proxy(vkbd: ZwpVirtualKeyboardV1) -> Self {
        Self {
            vkbd,
            _keymap_fd: None,
            modifier_state: ModifierState::new(),
            serial: 0,
        }
    }

    /// Get the underlying Wayland proxy.
    pub fn proxy(&self) -> &ZwpVirtualKeyboardV1 {
        &self.vkbd
    }

    /// Create a memfd with the given content and return the file descriptor.
    fn create_memfd(content: &str) -> Result<(OwnedFd, usize), InputError> {
        let cstr = CString::new(content).map_err(|e| InputError::KeymapFailed(e.to_string()))?;

        unsafe {
            let name = CString::new("osk-keymap").unwrap();
            let fd = libc::memfd_create(name.as_ptr(), 0);
            if fd < 0 {
                return Err(InputError::KeymapFailed("memfd_create failed".to_string()));
            }

            let len = cstr.as_bytes().len();
            if ftruncate(fd, len as libc::off_t) < 0 {
                close(fd);
                return Err(InputError::KeymapFailed("ftruncate failed".to_string()));
            }

            let ptr = mmap(ptr::null_mut(), len, PROT_READ, MAP_SHARED, fd, 0);
            if ptr == libc::MAP_FAILED {
                close(fd);
                return Err(InputError::KeymapFailed("mmap failed".to_string()));
            }

            std::ptr::copy_nonoverlapping(cstr.as_ptr() as *const u8, ptr as *mut u8, len);
            munmap(ptr, len);

            let owned = OwnedFd::from_raw_fd(fd);
            Ok((owned, len))
        }
    }
}

impl Drop for WaylandVirtualKeyboard {
    fn drop(&mut self) {
        self.vkbd.destroy();
    }
}

impl VirtualKeyboard for WaylandVirtualKeyboard {
    fn publish_keymap(&mut self, keymap_str: &str) -> Result<(), InputError> {
        let (fd, size) = Self::create_memfd(keymap_str)?;
        self.vkbd.keymap(1, fd.as_fd(), size as u32);
        self._keymap_fd = Some(fd);
        Ok(())
    }

    fn send_key(&mut self, keycode: KeyCode, state: KeyState) -> Result<(), InputError> {
        self.serial = self.serial.wrapping_add(1);
        let wl_state = match state {
            KeyState::Pressed => 1,
            KeyState::Released => 0,
        };
        self.vkbd.key(self.serial, keycode.raw(), wl_state);
        Ok(())
    }

    fn send_modifiers(&mut self, modifiers: &ModifierState) -> Result<(), InputError> {
        self.serial = self.serial.wrapping_add(1);
        self.vkbd.modifiers(self.serial, modifiers.depressed, modifiers.latched, modifiers.locked);
        Ok(())
    }
}
