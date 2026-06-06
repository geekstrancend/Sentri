#![deny(unsafe_code)]
#![allow(missing_docs)]

//! Solana code generator: Injects invariant checks into Rust programs.

pub mod generator;

pub use generator::SolanaGenerator;
