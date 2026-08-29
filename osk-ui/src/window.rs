//! OSK window that renders the keyboard as a GTK 4 layer-shell surface.

use crate::UiError;
use gtk4::Application;
use gtk4::ApplicationWindow;
use gtk4::Button;
use gtk4::GestureClick;
use gtk4::Grid;
use gtk4::gdk;
use gtk4::glib;
use gtk4::prelude::*;
use gtk4_layer_shell::Edge;
use gtk4_layer_shell::KeyboardMode;
use gtk4_layer_shell::Layer;
use gtk4_layer_shell::LayerShell;
use osk_core::KeyShape;
use osk_core::KeyState;
use osk_core::KeyType;
use osk_core::LayoutDef;
use osk_input::ModifierState;
use osk_input::VirtualKeyboard;
use std::cell::RefCell;
use std::rc::Rc;
use tracing::error;

/// Default CSS theme for the on-screen keyboard.
pub const DEFAULT_CSS: &str = include_str!("../assets/default.css");

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
    /// Create a new OSK window with the given application and layout.
    ///
    /// # Errors
    ///
    /// Returns `UiError` if the window or layer-shell setup fails.
    pub fn new(app: &Application, layout: LayoutDef) -> Result<Self, UiError> {
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
        // Clear existing children
        while let Some(child) = self.grid.first_child() {
            self.grid.remove(&child);
        }

        for (row_idx, row) in self.layout.rows.iter().enumerate() {
            let mut col_idx: i32 = 0;
            for key in row.iter() {
                let button = Button::builder().label(&key.label).css_classes(["key"]).build();

                // Add custom CSS class if present
                if let Some(ref css_class) = key.css_class {
                    button.add_css_class(css_class.as_str());
                }

                // Set width based on key shape
                let width = match &key.shape {
                    KeyShape::Rect { width_u } => *width_u,
                    KeyShape::LShape { top_width_u, .. } => *top_width_u,
                };

                // For spacers, make them invisible
                if key.label.is_empty() {
                    button.set_visible(false);
                }

                // Attach touch gesture for low-latency key events
                let gesture = GestureClick::builder().touch_only(true).build();

                let keyboard_clone = keyboard.clone();
                let modifier_state_clone = modifier_state.clone();
                let button_clone = button.clone();
                let keycode = key.keycode;
                let key_type = key.key_type.clone();

                gesture.connect_pressed(move |_, _n, _x, _y| {
                    // 1. Send key event IMMEDIATELY — before any UI work
                    if key_type == KeyType::Modifier
                        && let Some(modifier) = keycode.modifier()
                    {
                        modifier_state_clone.borrow_mut().press(modifier);
                        if let Err(e) = keyboard_clone.borrow_mut().send_modifiers(&modifier_state_clone.borrow()) {
                            error!("Failed to send modifier state: {e}");
                        }
                    }

                    if let Err(e) = keyboard_clone.borrow_mut().send_key(keycode, KeyState::Pressed) {
                        error!("Failed to send key event: {e}");
                    }

                    // 2. Defer visual feedback to the next idle cycle
                    let button_widget = button_clone.clone();
                    glib::idle_add_local_once(move || {
                        button_widget.add_css_class("key-pressed");
                    });
                });

                let keyboard_clone = keyboard.clone();
                let modifier_state_clone = modifier_state.clone();
                let button_clone = button.clone();
                let keycode = key.keycode;
                let key_type_release = key.key_type.clone();

                gesture.connect_released(move |_, _n, _x, _y| {
                    if key_type_release == KeyType::Modifier
                        && let Some(modifier) = keycode.modifier()
                    {
                        modifier_state_clone.borrow_mut().release(modifier);
                        if let Err(e) = keyboard_clone.borrow_mut().send_modifiers(&modifier_state_clone.borrow()) {
                            error!("Failed to send modifier state: {e}");
                        }
                    }

                    if let Err(e) = keyboard_clone.borrow_mut().send_key(keycode, KeyState::Released) {
                        error!("Failed to send key release: {e}");
                    }

                    let button_widget = button_clone.clone();
                    glib::idle_add_local_once(move || {
                        button_widget.remove_css_class("key-pressed");
                    });
                });

                button.add_controller(gesture);

                // Place in grid with appropriate column span
                let col_span = width.ceil() as i32;
                self.grid.attach(&button, col_idx, row_idx as i32, col_span.max(1), 1);
                col_idx += col_span.max(1);
            }
        }
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
