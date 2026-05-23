#![warn(missing_docs)]
#![deny(unsafe_code)]

//! Move (Aptos/Sui) program analyzer.

pub mod analyzer;
/// Vulnerability detectors for Move modules
pub mod detectors;

pub use analyzer::MoveAnalyzer;
pub use detectors::*;
