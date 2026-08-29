# osk-rs

A Wayland on-screen keyboard written in Rust, optimized for touchscreen
operation on machines without a physical keyboard.

## Features

- **Touchscreen-first** — large touch targets, multi-touch, visual feedback,
  long-press alternatives, key pop-up previews
- **Multiple layouts** — QWERTZ, QWERTY, AZERTY, Dvorak via XKB, with automatic
  layout detection from the active desktop environment
- **Size variants** — 60% (Compact), TKL (Tenkeyless), Full-Size (100%)
- **Correct key shapes** — ISO L-shaped Enter (de, fr, uk) vs. ANSI wide Enter
  (us), based on the active XKB layout
- **Wayland-native** — `zwlr_layer_shell_v1` overlay, `zwp_virtual_keyboard_v1`
  key injection, `zwp_input_method_v2` focus tracking
- **Compositor compatibility** — Hyprland (primary), Sway, River, Wayfire
- **Configurable** — TOML configuration with hot-reload, CLI overrides, runtime
  layout/size switching, theme support
- **Auto-show / auto-hide** — appears automatically when a text field is focused

## Quick Start

### Prerequisites

- Rust toolchain (stable or nightly)
- GTK 4 development libraries: `libgtk-4-dev`, `libglib2.0-dev`, `libgl-dev`
- D-Bus development libraries: `libdbus-1-dev`
- A Wayland compositor with `zwlr_layer_shell_v1` and
  `zwp_virtual_keyboard_manager_v1` support (Hyprland, Sway, River, Wayfire)

### Build

```sh
cargo build --workspace
```

### Run

```sh
cargo run -p osk-app
```

### Configuration

The configuration file is located at `~/.config/osk-rs/config.toml`.

## Documentation

- [Book](https://smearor.github.io/osk-rs/)
- [API Docs](https://smearor.github.io/osk-rs/docs/osk_rs/)
- [Concept Draft](concepts/planned/OSK_RS.md)

## License

MIT