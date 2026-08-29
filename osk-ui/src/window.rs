//! OSK window that renders the keyboard as a GTK 4 layer-shell surface.

use crate::KeyVisualState;
use crate::UiError;
use gtk4::Application;
use gtk4::ApplicationWindow;
use gtk4::Button;
use gtk4::GestureClick;
use gtk4::GestureLongPress;
use gtk4::Grid;
use gtk4::Label;
use gtk4::Popover;
use gtk4::gdk;
use gtk4::glib;
use gtk4::glib::SourceId;
use gtk4::prelude::*;
use gtk4_layer_shell::Edge;
use gtk4_layer_shell::KeyboardMode;
use gtk4_layer_shell::Layer;
use gtk4_layer_shell::LayerShell;
use osk_config::DisplayConfig;
use osk_config::KeyboardHeight;
use osk_core::Key;
use osk_core::KeyCode;
use osk_core::KeyShape;
use osk_core::KeyState;
use osk_core::KeyType;
use osk_core::LayoutDef;
use osk_core::Modifier;
use osk_input::ModifierState;
use osk_input::VirtualKeyboard;
use std::cell::RefCell;
use std::rc::Rc;
use tracing::debug;
use tracing::error;

/// Default CSS theme for the on-screen keyboard.
pub const DEFAULT_CSS: &str = include_str!("../assets/default.css");

/// Key repeat delay in milliseconds before auto-repeat begins.
const KEY_REPEAT_DELAY_MS: u32 = 300;

/// Key repeat interval in milliseconds between repeated key events.
const KEY_REPEAT_INTERVAL_MS: u32 = 50;

/// Main OSK window managing the GTK application window and layer-shell surface.
///
/// This struct owns the GTK window, the current layout definition, and
/// a reference to the virtual keyboard for input injection.
pub struct OskWindow {
    /// GTK application window
    pub window: ApplicationWindow,
    /// Current keyboard layout definition
    pub layout: LayoutDef,
    /// Grid widget holding the key buttons
    pub grid: Grid,
}

impl OskWindow {
    /// Create a new OSK window with the given application, layout, and display config.
    ///
    /// # Errors
    ///
    /// Returns `UiError` if the window or layer-shell setup fails.
    pub fn new(app: &Application, layout: LayoutDef, display: &DisplayConfig) -> Result<Self, UiError> {
        let window = ApplicationWindow::builder().application(app).build();

        let grid = Grid::builder()
            .column_spacing(2)
            .row_spacing(2)
            .margin_start(4)
            .margin_end(4)
            .margin_top(4)
            .margin_bottom(4)
            .build();

        window.set_child(Some(&grid));

        let osk_window = Self { window, layout, grid };

        osk_window.setup_layer_shell();
        osk_window.set_window_height(display.height_percent);
        osk_window.apply_css();
        Ok(osk_window)
    }

    /// Initialize the layer-shell surface via gtk4-layer-shell.
    pub fn setup_layer_shell(&self) {
        self.window.init_layer_shell();
        self.window.set_layer(Layer::Overlay);
        self.window.set_anchor(Edge::Bottom, true);
        self.window.set_anchor(Edge::Left, true);
        self.window.set_anchor(Edge::Right, true);
        self.window.set_keyboard_mode(KeyboardMode::Exclusive);
        self.window.set_exclusive_zone(0);
    }

    /// Set the window height as a percentage of the screen height.
    ///
    /// Uses the default monitor's geometry to calculate the pixel height.
    /// Falls back to a default of 300px if the monitor cannot be queried.
    pub fn set_window_height(&self, height_percent: KeyboardHeight) {
        let percent = height_percent.value();
        let default_height = 300;
        let height = gdk::Display::default()
            .and_then(|display| display.monitors().n_items().checked_sub(1).and_then(|i| display.monitors().item(i)))
            .and_then(|item| item.downcast::<gdk::Monitor>().ok())
            .and_then(|monitor| {
                let geometry = monitor.geometry();
                let computed = (geometry.height() as f32 * percent / 100.0).round() as i32;
                if computed > 0 { Some(computed) } else { None }
            })
            .unwrap_or(default_height);

        self.window.set_default_size(-1, height);
    }

    /// Apply the default CSS theme to the window.
    pub fn apply_css(&self) {
        let provider = gtk4::CssProvider::new();
        provider.load_from_data(DEFAULT_CSS);
        let display = gdk::Display::default();
        if let Some(display) = display {
            gtk4::style_context_add_provider_for_display(&display, &provider, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);
        }
    }

    /// Render the keyboard layout into the grid.
    ///
    /// Each key becomes a `gtk4::Button` with a touch gesture attached.
    /// Touch events send key events immediately via the virtual keyboard,
    /// with visual feedback deferred to the next idle cycle.
    pub fn render_keys<V: VirtualKeyboard + 'static>(&self, keyboard: Rc<RefCell<V>>, modifier_state: Rc<RefCell<ModifierState>>) {
        while let Some(child) = self.grid.first_child() {
            self.grid.remove(&child);
        }

        for (row_idx, row) in self.layout.rows.iter().enumerate() {
            let mut col_idx: i32 = 0;
            for key in row.iter() {
                let (button, popover, width) = self.create_key_button(key);
                self.attach_touch_handlers(&button, &popover, key, keyboard.clone(), modifier_state.clone());
                self.attach_long_press(&button, key.keycode);

                let col_span = width.ceil() as i32;
                self.grid.attach(&button, col_idx, row_idx as i32, col_span.max(1), 1);
                col_idx += col_span.max(1);
            }
        }
    }

    /// Create a GTK button for a key with CSS classes, visibility, and preview popover.
    ///
    /// Returns the button, its preview popover, and the key width in grid units.
    fn create_key_button(&self, key: &Key) -> (Button, Popover, f32) {
        let button = Button::builder().label(&key.label).css_classes(["key"]).build();

        if let Some(ref css_class) = key.css_class {
            button.add_css_class(css_class.as_str());
        }

        let width = match &key.shape {
            KeyShape::Rect { width_u } => *width_u,
            KeyShape::LShape { top_width_u, .. } => *top_width_u,
        };

        if key.label.is_empty() {
            button.set_visible(false);
        }

        let preview_label = Label::builder().label(&key.label).css_classes(["key-preview-label"]).build();
        let preview_popover = Popover::builder().child(&preview_label).autohide(false).build();
        preview_popover.set_parent(&button);

        (button, preview_popover, width)
    }

    /// Attach touch press/release handlers to a key button.
    ///
    /// On press: sends key event immediately, defers visual feedback, starts key repeat.
    /// On release: cancels repeat, sends key release, removes visual feedback.
    fn attach_touch_handlers<V: VirtualKeyboard + 'static>(
        &self,
        button: &Button,
        popover: &Popover,
        key: &Key,
        keyboard: Rc<RefCell<V>>,
        modifier_state: Rc<RefCell<ModifierState>>,
    ) {
        let gesture = GestureClick::builder().touch_only(true).build();
        let repeat_source: Rc<RefCell<Option<SourceId>>> = Rc::new(RefCell::new(None));
        let keycode = key.keycode;
        let key_type = key.key_type.clone();

        // --- Pressed handler ---
        let keyboard_pressed = keyboard.clone();
        let modifier_state_pressed = modifier_state.clone();
        let button_pressed = button.clone();
        let key_type_pressed = key_type.clone();
        let repeat_source_pressed = repeat_source.clone();
        let popover_pressed = popover.clone();

        gesture.connect_pressed(move |_, _n, _x, _y| {
            // 1. Send key event IMMEDIATELY — before any UI work
            if key_type_pressed == KeyType::Modifier
                && let Some(modifier) = keycode.modifier()
            {
                modifier_state_pressed.borrow_mut().press(modifier);
                if let Err(e) = keyboard_pressed.borrow_mut().send_modifiers(&modifier_state_pressed.borrow()) {
                    error!("Failed to send modifier state: {e}");
                }
            }

            if let Err(e) = keyboard_pressed.borrow_mut().send_key(keycode, KeyState::Pressed) {
                error!("Failed to send key event: {e}");
            }

            // 2. Defer visual feedback to the next idle cycle
            let button_widget = button_pressed.clone();
            let key_type_idle = key_type_pressed.clone();
            let popover_idle = popover_pressed.clone();
            glib::idle_add_local_once(move || {
                button_widget.add_css_class(KeyVisualState::Pressed.css_class());
                if key_type_idle == KeyType::Modifier {
                    button_widget.add_css_class(KeyVisualState::Active.css_class());
                }
                popover_idle.popup();
            });

            // 3. Start key repeat timer (non-modifier keys only)
            if key_type_pressed != KeyType::Modifier {
                Self::start_key_repeat(keyboard_pressed.clone(), keycode, repeat_source_pressed.clone());
            }
        });

        // --- Released handler ---
        let keyboard_released = keyboard.clone();
        let modifier_state_released = modifier_state.clone();
        let button_released = button.clone();
        let key_type_released = key_type.clone();
        let repeat_source_released = repeat_source.clone();
        let popover_released = popover.clone();

        gesture.connect_released(move |_, _n, _x, _y| {
            // Cancel key repeat timer
            if let Some(source_id) = repeat_source_released.borrow_mut().take() {
                source_id.remove();
            }

            if key_type_released == KeyType::Modifier
                && let Some(modifier) = keycode.modifier()
            {
                modifier_state_released.borrow_mut().release(modifier);
                if let Err(e) = keyboard_released.borrow_mut().send_modifiers(&modifier_state_released.borrow()) {
                    error!("Failed to send modifier state: {e}");
                }
            }

            if let Err(e) = keyboard_released.borrow_mut().send_key(keycode, KeyState::Released) {
                error!("Failed to send key release: {e}");
            }

            let button_widget = button_released.clone();
            let key_type_idle = key_type_released.clone();
            let popover_idle = popover_released.clone();
            glib::idle_add_local_once(move || {
                button_widget.remove_css_class(KeyVisualState::Pressed.css_class());
                // Keep key-active for CapsLock while locked
                if key_type_idle == KeyType::Modifier
                    && let Some(modifier) = keycode.modifier()
                    && modifier != Modifier::CapsLock
                {
                    button_widget.remove_css_class(KeyVisualState::Active.css_class());
                }
                popover_idle.popdown();
            });
        });

        button.add_controller(gesture);
    }

    /// Start a key repeat timer for a non-modifier key.
    ///
    /// After `KEY_REPEAT_DELAY_MS`, sends repeated press/release cycles
    /// at `KEY_REPEAT_INTERVAL_MS` intervals until cancelled on release.
    fn start_key_repeat<V: VirtualKeyboard + 'static>(keyboard: Rc<RefCell<V>>, keycode: KeyCode, repeat_source: Rc<RefCell<Option<SourceId>>>) {
        let repeat_source_inner = repeat_source.clone();
        let source_id = glib::timeout_add_local_once(std::time::Duration::from_millis(KEY_REPEAT_DELAY_MS as u64), move || {
            // Send first repeat immediately
            if let Err(e) = keyboard.borrow_mut().send_key(keycode, KeyState::Pressed) {
                error!("Failed to send key repeat: {e}");
            }
            if let Err(e) = keyboard.borrow_mut().send_key(keycode, KeyState::Released) {
                error!("Failed to send key repeat release: {e}");
            }
            // Then set up recurring interval
            let keyboard_interval = keyboard.clone();
            let source_id = glib::timeout_add_local(std::time::Duration::from_millis(KEY_REPEAT_INTERVAL_MS as u64), move || {
                if let Err(e) = keyboard_interval.borrow_mut().send_key(keycode, KeyState::Pressed) {
                    error!("Failed to send key repeat: {e}");
                }
                if let Err(e) = keyboard_interval.borrow_mut().send_key(keycode, KeyState::Released) {
                    error!("Failed to send key repeat release: {e}");
                }
                glib::ControlFlow::Continue
            });
            *repeat_source_inner.borrow_mut() = Some(source_id);
        });
        *repeat_source.borrow_mut() = Some(source_id);
    }

    /// Attach a long-press gesture stub for future features (dead keys, accent variants).
    fn attach_long_press(&self, button: &Button, keycode: KeyCode) {
        let long_press = GestureLongPress::builder().touch_only(true).build();
        long_press.connect_pressed(move |_, _x, _y| {
            debug!("Long press detected on keycode {:?}", keycode);
        });
        button.add_controller(long_press);
    }

    /// Show the window.
    pub fn show(&self) {
        self.window.show();
    }

    /// Hide the window.
    pub fn hide(&self) {
        self.window.hide();
    }
}
