//! Size variant of the keyboard layout.

use serde::Deserialize;
use serde::Serialize;
use std::fmt;
use std::str::FromStr;

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

impl fmt::Display for SizeVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Compact60 => write!(f, "compact"),
            Self::Tenkeyless80 => write!(f, "tkl"),
            Self::Full100 => write!(f, "full"),
        }
    }
}

impl FromStr for SizeVariant {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "compact" | "60" | "60%" => Ok(Self::Compact60),
            "tkl" | "tenkeyless" | "80" | "80%" => Ok(Self::Tenkeyless80),
            "full" | "fullsize" | "100" | "100%" => Ok(Self::Full100),
            _ => Err(format!("unknown size variant: {s}")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_variant_equality() {
        assert_eq!(SizeVariant::Compact60, SizeVariant::Compact60);
        assert_ne!(SizeVariant::Compact60, SizeVariant::Tenkeyless80);
    }

    #[test]
    fn test_display() {
        assert_eq!(SizeVariant::Compact60.to_string(), "compact");
        assert_eq!(SizeVariant::Tenkeyless80.to_string(), "tkl");
        assert_eq!(SizeVariant::Full100.to_string(), "full");
    }

    #[test]
    fn test_from_str() {
        assert_eq!("compact".parse::<SizeVariant>().unwrap(), SizeVariant::Compact60);
        assert_eq!("tkl".parse::<SizeVariant>().unwrap(), SizeVariant::Tenkeyless80);
        assert_eq!("full".parse::<SizeVariant>().unwrap(), SizeVariant::Full100);
        assert_eq!("60".parse::<SizeVariant>().unwrap(), SizeVariant::Compact60);
        assert_eq!("80".parse::<SizeVariant>().unwrap(), SizeVariant::Tenkeyless80);
        assert_eq!("100".parse::<SizeVariant>().unwrap(), SizeVariant::Full100);
    }

    #[test]
    fn test_from_str_invalid() {
        assert!("invalid".parse::<SizeVariant>().is_err());
    }
}
