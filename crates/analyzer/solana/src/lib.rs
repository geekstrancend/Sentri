#![warn(missing_docs)]
#![deny(unsafe_code)]

//! Solana program analyzer: Extracts program model from Rust source.

pub mod analyzer;

pub use analyzer::SolanaAnalyzer;
