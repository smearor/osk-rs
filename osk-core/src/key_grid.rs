//! A grid of keys — multiple rows forming a keyboard layout.

use crate::Key;
use crate::key_row::KeyRow;
use serde::Deserialize;
use serde::Serialize;
use std::ops::Deref;
use std::ops::DerefMut;

/// A grid of keys representing a complete keyboard layout.
///
/// Newtype wrapping `Vec<KeyRow>` to provide type safety and semantic clarity.
/// Each [`KeyRow`] represents one horizontal row of keys.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct KeyGrid(Vec<KeyRow>);

impl KeyGrid {
    /// Create a new empty `KeyGrid`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a `KeyGrid` from a vector of key rows.
    pub fn from_rows(rows: Vec<KeyRow>) -> Self {
        Self(rows)
    }

    /// Create a `KeyGrid` from a vector of vectors of keys.
    pub fn from_vec(rows: Vec<Vec<Key>>) -> Self {
        Self(rows.into_iter().map(KeyRow::from).collect())
    }

    /// Push a row onto the end of this grid.
    pub fn push(&mut self, row: KeyRow) {
        self.0.push(row);
    }

    /// Return the number of rows in this grid.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Return `true` if this grid contains no rows.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Return an iterator over the rows in this grid.
    pub fn iter(&self) -> std::slice::Iter<'_, KeyRow> {
        self.0.iter()
    }

    /// Return a mutable iterator over the rows in this grid.
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, KeyRow> {
        self.0.iter_mut()
    }

    /// Return the inner vector of rows, consuming this `KeyGrid`.
    pub fn into_inner(self) -> Vec<KeyRow> {
        self.0
    }

    /// Convert this `KeyGrid` into a `Vec<Vec<Key>>`.
    pub fn into_vec_vec(self) -> Vec<Vec<Key>> {
        self.0.into_iter().map(KeyRow::into_inner).collect()
    }
}

impl From<Vec<KeyRow>> for KeyGrid {
    fn from(rows: Vec<KeyRow>) -> Self {
        Self(rows)
    }
}

impl From<Vec<Vec<Key>>> for KeyGrid {
    fn from(rows: Vec<Vec<Key>>) -> Self {
        Self::from_vec(rows)
    }
}

impl From<KeyGrid> for Vec<Vec<Key>> {
    fn from(grid: KeyGrid) -> Self {
        grid.into_vec_vec()
    }
}

impl Deref for KeyGrid {
    type Target = Vec<KeyRow>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for KeyGrid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl IntoIterator for KeyGrid {
    type Item = KeyRow;
    type IntoIter = std::vec::IntoIter<KeyRow>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a KeyGrid {
    type Item = &'a KeyRow;
    type IntoIter = std::slice::Iter<'a, KeyRow>;

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
        let grid = KeyGrid::new();
        assert!(grid.is_empty());
        assert_eq!(grid.len(), 0);
    }

    #[test]
    fn test_push_and_len() {
        let mut grid = KeyGrid::new();
        grid.push(KeyRow::from_vec(vec![Key::char("A", keycode::KEY_A)]));
        grid.push(KeyRow::from_vec(vec![Key::char("B", keycode::KEY_B), Key::char("C", keycode::KEY_C)]));
        assert_eq!(grid.len(), 2);
        assert!(!grid.is_empty());
    }

    #[test]
    fn test_from_vec() {
        let rows = vec![vec![Key::char("A", keycode::KEY_A)], vec![Key::char("B", keycode::KEY_B)]];
        let grid = KeyGrid::from_vec(rows);
        assert_eq!(grid.len(), 2);
        assert_eq!(grid[0].len(), 1);
        assert_eq!(grid[1].len(), 1);
    }

    #[test]
    fn test_into_vec_vec() {
        let rows = vec![vec![Key::char("A", keycode::KEY_A)], vec![Key::char("B", keycode::KEY_B)]];
        let grid = KeyGrid::from_vec(rows.clone());
        let back = grid.into_vec_vec();
        assert_eq!(back, rows);
    }

    #[test]
    fn test_deref() {
        let grid = KeyGrid::from_vec(vec![vec![Key::char("A", keycode::KEY_A)]]);
        assert_eq!(grid[0][0].label, "A");
    }

    #[test]
    fn test_into_iter() {
        let grid = KeyGrid::from_vec(vec![vec![Key::char("A", keycode::KEY_A)], vec![Key::char("B", keycode::KEY_B)]]);
        let labels: Vec<String> = grid.into_iter().flat_map(|row| row.into_iter().map(|k| k.label)).collect();
        assert_eq!(labels, vec!["A", "B"]);
    }

    #[test]
    fn test_serde_roundtrip() {
        let grid = KeyGrid::from_vec(vec![vec![Key::char("A", keycode::KEY_A)]]);
        let json = serde_json::to_string(&grid).unwrap();
        let back: KeyGrid = serde_json::from_str(&json).unwrap();
        assert_eq!(grid, back);
    }
}
