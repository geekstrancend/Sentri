//! Invariant library management.

use invar_core::model::Invariant;
use std::collections::BTreeMap;

/// A collection of invariants organized by category.
pub struct InvariantLibrary {
    /// Invariants by category.
    pub categories: BTreeMap<String, Vec<Invariant>>,
}

impl InvariantLibrary {
    /// Create a new empty library.
    pub fn new() -> Self {
        Self {
            categories: BTreeMap::new(),
        }
    }

    /// Add an invariant to the library.
    pub fn add(&mut self, category: String, invariant: Invariant) {
        self.categories
            .entry(category)
            .or_insert_with(Vec::new)
            .push(invariant);
    }

    /// Get all invariants in a category.
    pub fn get_category(&self, category: &str) -> Option<&[Invariant]> {
        self.categories.get(category).map(|v| v.as_slice())
    }

    /// Get all invariants.
    pub fn all(&self) -> Vec<&Invariant> {
        self.categories
            .values()
            .flat_map(|v| v.iter())
            .collect()
    }

    /// Count total invariants.
    pub fn count(&self) -> usize {
        self.categories.values().map(|v| v.len()).sum()
    }
}

impl Default for InvariantLibrary {
    fn default() -> Self {
        Self::new()
    }
}
