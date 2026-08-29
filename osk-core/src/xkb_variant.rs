//! XKB layout variant.
//!
//! Represents the variant component of an XKB keyboard layout specification,
//! e.g. `nodeadkeys`, `intl`, `qwerty`.

use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use std::fmt;

/// XKB layout variant.
///
/// Selects a specific variant within an XKB layout. Most layouts use
/// [`XkbVariant::Default`] (no variant). Named variants cover the most
/// common options; [`XkbVariant::Custom`] allows arbitrary XKB variant
/// strings for less common or future variants.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum XkbVariant {
    /// No variant — the layout's default configuration
    #[default]
    Default,
    /// No dead keys (e.g. German `nodeadkeys`)
    Nodeadkeys,
    /// International layout with AltGr dead keys (e.g. `intl`)
    Intl,
    /// QWERTY mapping within a non-QWERTY layout (e.g. German `qwerty`)
    Qwerty,
    /// Dead grave accent key
    Deadgraveacute,
    /// Dead acute accent key
    Deadacute,
    /// Macintosh variant
    Mac,
    /// Neo layout (German ergonomic)
    Neo,
    /// Dvorak layout variant
    Dvorak,
    /// Custom XKB variant string (e.g. `"T3"`, `"sun_type6"`)
    Custom(String),
}

impl XkbVariant {
    /// Return the XKB variant string, or `None` for the default.
    ///
    /// This returns the string that would be passed to `xkbcommon` as
    /// the `XKB_VARIANT_NAME` component.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::Default => None,
            Self::Nodeadkeys => Some("nodeadkeys"),
            Self::Intl => Some("intl"),
            Self::Qwerty => Some("qwerty"),
            Self::Deadgraveacute => Some("deadgraveacute"),
            Self::Deadacute => Some("deadacute"),
            Self::Mac => Some("mac"),
            Self::Neo => Some("neo"),
            Self::Dvorak => Some("dvorak"),
            Self::Custom(s) => Some(s),
        }
    }
}

impl From<&str> for XkbVariant {
    fn from(s: &str) -> Self {
        match s {
            "nodeadkeys" => Self::Nodeadkeys,
            "intl" => Self::Intl,
            "qwerty" => Self::Qwerty,
            "deadgraveacute" => Self::Deadgraveacute,
            "deadacute" => Self::Deadacute,
            "mac" => Self::Mac,
            "neo" => Self::Neo,
            "dvorak" => Self::Dvorak,
            "" => Self::Default,
            other => Self::Custom(other.to_string()),
        }
    }
}

impl From<String> for XkbVariant {
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}

impl fmt::Display for XkbVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.as_str() {
            Some(s) => write!(f, "{s}"),
            None => write!(f, "default"),
        }
    }
}

impl Serialize for XkbVariant {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.as_str() {
            Some(s) => serializer.serialize_str(s),
            None => serializer.serialize_str("default"),
        }
    }
}

impl<'de> Deserialize<'de> for XkbVariant {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Self::from(s))
    }
}
