//! A row of keys on the keyboard.

use crate::Key;
use serde::Deserialize;
use serde::Serialize;
use std::ops::Deref;
use std::ops::DerefMut;

/// A single row of keys on the keyboard.
///
/// Newtype wrapping `Vec<Key>` to provide type safety and semantic clarity
/// when passing key rows between modules.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct KeyRow(Vec<Key>);

impl KeyRow {
    /// Create a new empty `KeyRow`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a `KeyRow` from a vector of keys.
    pub fn from_vec(keys: Vec<Key>) -> Self {
        Self(keys)
    }

    /// Push a key onto the end of this row.
    pub fn push(&mut self, key: Key) {
        self.0.push(key);
    }

    /// Extend this row with the contents of another row.
    pub fn extend(&mut self, other: &KeyRow) {
        self.0.extend(other.0.iter().cloned());
    }

    /// Return the number of keys in this row.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Return `true` if this row contains no keys.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Return an iterator over the keys in this row.
    pub fn iter(&self) -> std::slice::Iter<'_, Key> {
        self.0.iter()
    }

    /// Return a mutable iterator over the keys in this row.
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Key> {
        self.0.iter_mut()
    }

    /// Return the inner vector, consuming this `KeyRow`.
    pub fn into_inner(self) -> Vec<Key> {
        self.0
    }
}

impl From<Vec<Key>> for KeyRow {
    fn from(keys: Vec<Key>) -> Self {
        Self(keys)
    }
}

impl From<KeyRow> for Vec<Key> {
    fn from(row: KeyRow) -> Self {
        row.0
    }
}

impl Deref for KeyRow {
    type Target = Vec<Key>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for KeyRow {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl IntoIterator for KeyRow {
    type Item = Key;
    type IntoIter = std::vec::IntoIter<Key>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a KeyRow {
    type Item = &'a Key;
    type IntoIter = std::slice::Iter<'a, Key>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::keycode;

    #[test]
    fn test_new_empty() {
        let row = KeyRow::new();
        assert!(row.is_empty());
        assert_eq!(row.len(), 0);
    }

    #[test]
    fn test_push_and_len() {
        let mut row = KeyRow::new();
        row.push(Key::char("A", keycode::KEY_A));
        row.push(Key::char("B", keycode::KEY_B));
        assert_eq!(row.len(), 2);
        assert!(!row.is_empty());
    }

    #[test]
    fn test_from_vec_and_into_inner() {
        let keys = vec![Key::char("A", keycode::KEY_A), Key::char("B", keycode::KEY_B)];
        let row = KeyRow::from_vec(keys.clone());
        assert_eq!(row.len(), 2);
        let back = row.into_inner();
        assert_eq!(back, keys);
    }

    #[test]
    fn test_extend() {
        let mut row1 = KeyRow::from_vec(vec![Key::char("A", keycode::KEY_A)]);
        let row2 = KeyRow::from_vec(vec![Key::char("B", keycode::KEY_B)]);
        row1.extend(&row2);
        assert_eq!(row1.len(), 2);
    }

    #[test]
    fn test_deref() {
        let row = KeyRow::from_vec(vec![Key::char("A", keycode::KEY_A), Key::char("B", keycode::KEY_B)]);
        assert_eq!(row[0].label, "A");
        assert_eq!(row[1].label, "B");
    }

    #[test]
    fn test_into_iter() {
        let row = KeyRow::from_vec(vec![Key::char("A", keycode::KEY_A), Key::char("B", keycode::KEY_B)]);
        let labels: Vec<String> = row.into_iter().map(|k| k.label).collect();
        assert_eq!(labels, vec!["A", "B"]);
    }

    #[test]
    fn test_serde_roundtrip() {
        let row = KeyRow::from_vec(vec![Key::char("A", keycode::KEY_A)]);
        let json = serde_json::to_string(&row).unwrap();
        let back: KeyRow = serde_json::from_str(&json).unwrap();
        assert_eq!(row, back);
    }
}
