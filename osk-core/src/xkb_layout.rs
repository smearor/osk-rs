//! XKB layout name.
//!
//! Represents the layout component of an XKB keyboard layout specification,
//! e.g. `de`, `us`, `fr`. This identifies the base keyboard mapping.

use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use std::fmt;
use std::str::FromStr;

/// XKB layout name.
///
/// Identifies the base keyboard layout. Named variants cover the most common
/// layouts; [`XkbLayout::Custom`] allows arbitrary XKB layout strings for
/// less common or future layouts.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum XkbLayout {
    /// US English QWERTY layout
    #[default]
    Us,
    /// German QWERTZ layout
    De,
    /// French AZERTY layout
    Fr,
    /// Italian layout
    It,
    /// Spanish layout
    Es,
    /// British English layout
    Gb,
    /// Russian layout
    Ru,
    /// Norwegian layout
    No,
    /// Swedish layout
    Se,
    /// Danish layout
    Dk,
    /// Finnish layout
    Fi,
    /// Dutch layout
    Nl,
    /// Belgian layout
    Be,
    /// Swiss layout
    Ch,
    /// Polish layout
    Pl,
    /// Turkish layout
    Tr,
    /// Czech layout
    Cz,
    /// Custom XKB layout string (e.g. `"ua"`, `"gr"`)
    Custom(String),
}

impl XkbLayout {
    /// Return the XKB layout string used by `xkbcommon`.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Us => "us",
            Self::De => "de",
            Self::Fr => "fr",
            Self::It => "it",
            Self::Es => "es",
            Self::Gb => "gb",
            Self::Ru => "ru",
            Self::No => "no",
            Self::Se => "se",
            Self::Dk => "dk",
            Self::Fi => "fi",
            Self::Nl => "nl",
            Self::Be => "be",
            Self::Ch => "ch",
            Self::Pl => "pl",
            Self::Tr => "tr",
            Self::Cz => "cz",
            Self::Custom(s) => s,
        }
    }

    /// Return the common alias for this layout (e.g. "qwertz" for "de"),
    /// or `None` if no alias exists.
    pub fn alias(&self) -> Option<&'static str> {
        match self {
            Self::Us => Some("qwerty"),
            Self::De => Some("qwertz"),
            Self::Fr => Some("azerty"),
            _ => None,
        }
    }
}

impl fmt::Display for XkbLayout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<&str> for XkbLayout {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "us" | "qwerty" => Self::Us,
            "de" | "qwertz" => Self::De,
            "fr" | "azerty" => Self::Fr,
            "it" => Self::It,
            "es" => Self::Es,
            "gb" => Self::Gb,
            "ru" => Self::Ru,
            "no" => Self::No,
            "se" => Self::Se,
            "dk" => Self::Dk,
            "fi" => Self::Fi,
            "nl" => Self::Nl,
            "be" => Self::Be,
            "ch" => Self::Ch,
            "pl" => Self::Pl,
            "tr" => Self::Tr,
            "cz" => Self::Cz,
            "" => Self::Us,
            other => Self::Custom(other.to_string()),
        }
    }
}

impl From<String> for XkbLayout {
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}

impl FromStr for XkbLayout {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s))
    }
}

impl Serialize for XkbLayout {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for XkbLayout {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Self::from(s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_str() {
        assert_eq!(XkbLayout::Us.as_str(), "us");
        assert_eq!(XkbLayout::De.as_str(), "de");
        assert_eq!(XkbLayout::Fr.as_str(), "fr");
    }

    #[test]
    fn test_display() {
        assert_eq!(XkbLayout::Us.to_string(), "us");
        assert_eq!(XkbLayout::De.to_string(), "de");
    }

    #[test]
    fn test_from_str_known() {
        assert_eq!(XkbLayout::from("us"), XkbLayout::Us);
        assert_eq!(XkbLayout::from("de"), XkbLayout::De);
        assert_eq!(XkbLayout::from("fr"), XkbLayout::Fr);
    }

    #[test]
    fn test_from_str_alias() {
        assert_eq!(XkbLayout::from("qwerty"), XkbLayout::Us);
        assert_eq!(XkbLayout::from("qwertz"), XkbLayout::De);
        assert_eq!(XkbLayout::from("azerty"), XkbLayout::Fr);
    }

    #[test]
    fn test_from_str_custom() {
        assert_eq!(XkbLayout::from("ua"), XkbLayout::Custom("ua".to_string()));
    }

    #[test]
    fn test_alias() {
        assert_eq!(XkbLayout::Us.alias(), Some("qwerty"));
        assert_eq!(XkbLayout::De.alias(), Some("qwertz"));
        assert_eq!(XkbLayout::Fr.alias(), Some("azerty"));
        assert_eq!(XkbLayout::It.alias(), None);
    }

    #[test]
    fn test_serde_roundtrip() {
        let layout = XkbLayout::De;
        let json = serde_json::to_string(&layout).unwrap();
        let back: XkbLayout = serde_json::from_str(&json).unwrap();
        assert_eq!(layout, back);
    }

    #[test]
    fn test_serde_custom_roundtrip() {
        let layout = XkbLayout::Custom("ua".to_string());
        let json = serde_json::to_string(&layout).unwrap();
        let back: XkbLayout = serde_json::from_str(&json).unwrap();
        assert_eq!(layout, back);
    }
}
