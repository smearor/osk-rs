//! Size variant of the keyboard layout.

use serde::Deserialize;
use serde::Serialize;

/// Size variant of the keyboard layout.
///
/// Determines which key blocks are included in the layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum SizeVariant {
    /// 60% compact — main block only, no function row, no nav cluster, no numpad
    #[default]
    Compact60,
    /// 80% tenkeyless — main block + function row + nav cluster, no numpad
    Tenkeyless80,
    /// 100% full-size — all blocks including numpad
    Full100,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_variant_equality() {
        assert_eq!(SizeVariant::Compact60, SizeVariant::Compact60);
        assert_ne!(SizeVariant::Compact60, SizeVariant::Tenkeyless80);
    }
}
