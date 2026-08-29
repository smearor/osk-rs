//! XKB layout parsing and size variant builder for `osk-rs`.
//!
//! This crate reads XKB layout definitions and builds visual key grids
//! for different size variants (60%, TKL, Full-Size).

mod error;
mod layout_builder;
mod layout_registry;
mod layout_sections;
mod layouts;
mod size_variant_builder;
mod standard_layout_builder;

pub use error::LayoutError;
pub use layout_builder::LayoutBuilder;
pub use layout_registry::LayoutRegistry;
pub use layout_sections::LayoutSections;
pub use layouts::Azerty;
pub use layouts::Dvorak;
pub use layouts::LayoutDefinition;
pub use layouts::Qwerty;
pub use layouts::Qwertz;
pub use layouts::qwertz_tkl;
pub use size_variant_builder::SizeVariantBuilder;
pub use standard_layout_builder::StandardLayoutBuilder;
