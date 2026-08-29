//! Application entry point for `osk-rs`.

use std::cell::RefCell;
use std::rc::Rc;

use clap::Parser;
use gtk4::prelude::*;
use osk_config::Cli;
use osk_input::{ModifierState, VirtualKeyboard, WaylandVirtualKeyboard};
use osk_layout::qwertz_tkl;
use osk_ui::OskWindow;
use osk_wayland::WaylandContext;

/// Run the `osk-rs` application.
pub fn run() {
    let cli = Cli::parse();

    if cli.verbose {
        tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).init();
    } else {
        tracing_subscriber::fmt().with_max_level(tracing::Level::INFO).init();
    }

    tracing::info!("osk-rs starting up");

    let app = gtk4::Application::builder().application_id("org.example.OSK").build();

    app.connect_activate(move |app| {
        // Connect to Wayland and bind required globals
        let wayland = match WaylandContext::connect() {
            Ok(ctx) => ctx,
            Err(e) => {
                tracing::error!("Wayland connection failed: {e}");
                return;
            }
        };

        // Create virtual keyboard proxy from the Wayland context
        let vkbd_proxy = wayland.vkbd_manager.create_virtual_keyboard(&wayland.seat, &wayland.queue_handle, ());
        let mut vkbd = WaylandVirtualKeyboard::from_proxy(vkbd_proxy);

        // Publish a minimal keymap (Phase 1: hardcoded XKB keymap string)
        // TODO: Generate proper XKB keymap from layout in Phase 2
        let keymap = include_str!("../assets/keymap_de.txt");
        if let Err(e) = vkbd.publish_keymap(keymap) {
            tracing::error!("Keymap setup failed: {e}");
            return;
        }

        // Build the QWERTZ TKL layout
        let layout = qwertz_tkl();

        // Create the OSK window
        let window = match OskWindow::new(app, layout) {
            Ok(w) => w,
            Err(e) => {
                tracing::error!("Window creation failed: {e}");
                return;
            }
        };

        // Wrap virtual keyboard and modifier state in Rc<RefCell>
        let vkbd_rc = Rc::new(RefCell::new(vkbd));
        let modifier_state = Rc::new(RefCell::new(ModifierState::new()));

        // Render keys with touch handlers
        window.render_keys(vkbd_rc, modifier_state);
        window.show();

        tracing::info!("osk-rs keyboard visible");
    });

    let _ = app.run();

    tracing::info!("osk-rs shutting down");
}
