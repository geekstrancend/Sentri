#![warn(missing_docs)]
#![deny(unsafe_code)]

//! Simulation engine for finding invariant violations.
//!
//! This module provides:
//! - EVM runtime execution using revm
//! - Fuzzing engine for invariant testing
//! - Flash loan attack simulation for DeFi protocols

pub mod engine;
pub mod evm_runtime;
pub mod flash_loan_sim;
pub mod fuzzer;

pub use engine::SimulationEngine;
pub use evm_runtime::EvmRuntime;
pub use flash_loan_sim::FlashLoanSimulator;
pub use fuzzer::{Fuzzer, FuzzerConfig, FuzzResult, FuzzTransaction};

