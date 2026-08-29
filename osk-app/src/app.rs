//! Application entry point for `osk-rs`.

use clap::Parser;
use gtk4::prelude::*;
use osk_config::Cli;
use osk_input::ModifierState;
use osk_input::VirtualKeyboard;
use osk_input::WaylandVirtualKeyboard;
use osk_layout::qwertz_tkl;
use osk_ui::OskWindow;
use osk_wayland::WaylandContext;
use std::cell::RefCell;
use std::rc::Rc;
use tracing::error;
use tracing::info;

/// Run the `osk-rs` application.
pub fn run() {
    let cli = Cli::parse();

    if cli.verbose {
        tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).init();
    } else {
        tracing_subscriber::fmt().with_max_level(tracing::Level::INFO).init();
    }

    info!("osk-rs starting up");

    let app = gtk4::Application::builder().application_id("org.example.OSK").build();

    app.connect_activate(move |app| {
        // Connect to Wayland and bind required globals
        let wayland = match WaylandContext::connect() {
            Ok(ctx) => ctx,
            Err(e) => {
                error!("Wayland connection failed: {e}");
                return;
            }
        };

        // Create virtual keyboard proxy from the Wayland context
        let vkbd_proxy = wayland.vkbd_manager.create_virtual_keyboard(&wayland.seat, &wayland.queue_handle, ());
        let mut virtual_keyboard = WaylandVirtualKeyboard::from_proxy(vkbd_proxy);

        // Publish a minimal keymap (Phase 1: hardcoded XKB keymap string)
        // TODO: Generate proper XKB keymap from layout in Phase 2
        let keymap = include_str!("../assets/keymap_de.txt");
        if let Err(e) = virtual_keyboard.publish_keymap(keymap) {
            error!("Keymap setup failed: {e}");
            return;
        }

        // Build the QWERTZ TKL layout
        let layout = qwertz_tkl();

        // Create the OSK window
        let window = match OskWindow::new(app, layout) {
            Ok(w) => w,
            Err(e) => {
                error!("Window creation failed: {e}");
                return;
            }
        };

        // Wrap virtual keyboard and modifier state in Rc<RefCell>
        let keyboard = Rc::new(RefCell::new(virtual_keyboard));
        let modifier_state = Rc::new(RefCell::new(ModifierState::new()));

        // Render keys with touch handlers
        window.render_keys(keyboard, modifier_state);
        window.show();

        info!("osk-rs keyboard visible");
    });

    let _ = app.run();

    info!("osk-rs shutting down");
}
