#![warn(missing_docs)]
#![deny(unsafe_code)]

//! EVM (Ethereum/Solidity) program analyzer.

pub mod analyzer;

pub use analyzer::EvmAnalyzer;
