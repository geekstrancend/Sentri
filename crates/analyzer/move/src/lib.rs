#![warn(missing_docs)]
#![deny(unsafe_code)]

//! Move (Aptos/Sui) program analyzer.

pub mod analyzer;
/// Vulnerability detectors for Move modules
pub mod detectors;
pub mod move_type_safety_violation;
pub mod move_resource_destruction;

pub use analyzer::MoveAnalyzer;
pub use detectors::*;
pub use move_type_safety_violation::detect_move_type_safety_violation;
pub use move_resource_destruction::detect_move_resource_destruction;
