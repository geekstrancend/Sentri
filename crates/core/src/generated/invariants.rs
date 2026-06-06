// Empty invariant registry (no .sinv files found)
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompiledInvariant {
    pub id: &'static str,
    pub severity: &'static str,
    pub chain: &'static str,
    pub description: &'static str,
    pub message: &'static str,
}

pub static INVARIANT_REGISTRY: Lazy<BTreeMap<&'static str, CompiledInvariant>> = 
    Lazy::new(BTreeMap::new);

pub fn get_invariant(id: &str) -> Option<&'static CompiledInvariant> {
    INVARIANT_REGISTRY.get(id)
}

pub fn invariants_for_chain(_chain: &str) -> Vec<&'static CompiledInvariant> {
    vec![]
}

pub fn invariant_count() -> usize {
    0
}
