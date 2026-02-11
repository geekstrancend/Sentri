#![warn(missing_docs)]
#![deny(unsafe_code)]

//! Move (Aptos/Sui) program analyzer.

pub mod analyzer;

pub use analyzer::MoveAnalyzer;
