#![warn(missing_docs)]
#![deny(unsafe_code)]

//! Solana code generator: Injects invariant checks into Rust programs.

pub mod generator;

pub use generator::SolanaGenerator;
