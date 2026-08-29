//! Application entry point for `osk-rs`.

use clap::Parser;
use gtk4::prelude::*;
use osk_config::Cli;
use osk_config::Config;
use osk_core::SizeVariant;
use osk_core::XkbLayout;
use osk_input::ModifierState;
use osk_input::VirtualKeyboard;
use osk_input::WaylandVirtualKeyboard;
use osk_layout::LayoutRegistry;
use osk_layout::XkbLayoutParser;
use osk_ui::OskWindow;
use osk_wayland::WaylandContext;
use std::cell::RefCell;
use std::rc::Rc;
use tracing::error;
use tracing::info;
use tracing::warn;

/// Run the `osk-rs` application.
pub fn run() {
    let cli = Cli::parse();

    if cli.verbose {
        tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).init();
    } else {
        tracing_subscriber::fmt().with_max_level(tracing::Level::INFO).init();
    }

    info!("osk-rs starting up");

    // Load configuration from ~/.config/osk-rs/config.toml (or defaults)
    let config = match Config::load() {
        Ok(c) => {
            info!("Configuration loaded successfully");
            c
        }
        Err(e) => {
            warn!("Config load failed, using defaults: {e}");
            Config::default()
        }
    };

    // Determine layout: CLI override > config > default "us"
    let layout = cli.layout.map(XkbLayout::from).or(config.layout.name.clone()).unwrap_or_default();

    // Determine variant: config > default
    let variant = config.layout.variant.clone().unwrap_or_default();

    // Determine size variant: CLI override > config > default TKL
    let size_variant = cli
        .size
        .and_then(|s| s.parse().ok())
        .or(config.display.size)
        .unwrap_or(SizeVariant::Tenkeyless80);

    info!("Using layout: {layout}, variant: {variant}, size: {size_variant}");

    // Build the layout using the layout registry
    let layout_def = LayoutRegistry::build_with_variant(layout.clone(), variant.clone(), size_variant).unwrap_or_default();

    let app = gtk4::Application::builder().application_id("org.example.OSK").build();

    app.connect_shutdown(move |_| {
        info!("osk-rs shutting down — releasing resources");
    });

    let layout_for_log = layout.clone();
    let variant_for_keymap = variant.clone();
    app.connect_activate(move |app| {
        // Connect to Wayland and bind required globals
        info!("Connecting to Wayland...");
        let wayland = match WaylandContext::connect() {
            Ok(ctx) => ctx,
            Err(e) => {
                error!("Wayland connection failed: {e}");
                return;
            }
        };
        info!("Wayland connected");

        // Create virtual keyboard proxy from the Wayland context
        info!("Creating virtual keyboard proxy...");
        let vkbd_proxy = wayland.vkbd_manager.create_virtual_keyboard(&wayland.seat, &wayland.queue_handle, ());
        let mut virtual_keyboard = WaylandVirtualKeyboard::from_proxy(vkbd_proxy);
        info!("Virtual keyboard proxy created");

        // Generate XKB keymap from the selected layout and variant
        info!("Generating XKB keymap...");
        let keymap_string = match XkbLayoutParser::new() {
            Ok(parser) => match parser.parse(&layout_for_log, &variant_for_keymap) {
                Ok(keymap) => keymap.keymap_string,
                Err(e) => {
                    error!("XKB keymap generation failed for layout '{layout_for_log}': {e}");
                    return;
                }
            },
            Err(e) => {
                error!("XKB parser initialization failed: {e}");
                return;
            }
        };
        info!("XKB keymap generated");

        if let Err(e) = virtual_keyboard.publish_keymap(&keymap_string) {
            error!("Keymap setup failed: {e}");
            return;
        }
        info!("Keymap published");

        // Create the OSK window
        info!("Creating OSK window...");
        let window = match OskWindow::new(app, layout_def.clone(), &config.display) {
            Ok(w) => w,
            Err(e) => {
                error!("Window creation failed: {e}");
                return;
            }
        };
        info!("OSK window created");

        // Wrap virtual keyboard and modifier state in Rc<RefCell>
        let keyboard = Rc::new(RefCell::new(virtual_keyboard));
        let modifier_state = Rc::new(RefCell::new(ModifierState::new()));

        // Render keys with touch handlers
        info!("Rendering keys...");
        window.render_keys(keyboard, modifier_state);
        info!("Keys rendered");

        window.show();

        info!("osk-rs keyboard visible (layout={layout_for_log}, size={size_variant})");
    });

    // Strip our custom CLI args so GTK's Application::run() doesn't choke on them.
    // clap already parsed what we need; pass only the program name to GTK.
    let gtk_args = std::env::args().take(1).collect::<Vec<_>>();
    let _ = app.run_with_args(&gtk_args);
}
