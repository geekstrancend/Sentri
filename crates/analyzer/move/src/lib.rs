#![deny(unsafe_code)]
#![allow(missing_docs)]

//! Move (Aptos/Sui) program analyzer.

pub mod analyzer;
/// Vulnerability detectors for Move modules
pub mod detectors;
pub mod move_resource_destruction;
pub mod move_type_safety_violation;
/// Chain-agnostic semantic-model extraction (Epic 6.1 shared IR)
pub mod semantic_model;

pub use analyzer::MoveAnalyzer;
pub use detectors::*;
pub use move_resource_destruction::detect_move_resource_destruction;
pub use move_type_safety_violation::detect_move_type_safety_violation;
pub use semantic_model::build_semantic_model;
