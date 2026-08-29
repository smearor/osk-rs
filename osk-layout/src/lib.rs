//! XKB layout parsing and size variant builder for `osk-rs`.
//!
//! This crate reads XKB layout definitions and builds visual key grids
//! for different size variants (60%, TKL, Full-Size).

mod error;
mod layout_builder;
mod qwertz_tkl;
mod size_variant_builder;
mod standard_layout_builder;

pub use error::LayoutError;
pub use layout_builder::LayoutBuilder;
pub use qwertz_tkl::qwertz_tkl;
pub use size_variant_builder::SizeVariantBuilder;
pub use standard_layout_builder::StandardLayoutBuilder;
