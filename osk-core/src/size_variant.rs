//! Size variant of the keyboard layout.

use serde::{Deserialize, Serialize};

/// Size variant of the keyboard layout.
///
/// Determines which key blocks are included in the layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SizeVariant {
    /// 60% compact — main block only, no function row, no nav cluster, no numpad
    Compact60,
    /// 80% tenkeyless — main block + function row + nav cluster, no numpad
    Tenkeyless80,
    /// 100% full-size — all blocks including numpad
    Full100,
}
