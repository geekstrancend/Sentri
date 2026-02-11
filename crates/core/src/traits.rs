//! Core traits defining the analyzer/generator/simulator interface.

use crate::error::Result;
use crate::model::{GenerationOutput, Invariant, ProgramModel, SimulationReport};
use std::path::Path;

/// Analyzes a smart contract program and extracts its model.
///
/// Implementations must be chain-specific but return a chain-agnostic `ProgramModel`.
pub trait ChainAnalyzer: Send + Sync {
    /// Analyze a program at the given path.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The file cannot be read
    /// - The syntax is invalid for the target chain
    /// - Unsupported patterns are encountered
    fn analyze(&self, path: &Path) -> Result<ProgramModel>;

    /// Chain identifier: "solana", "evm", "move".
    fn chain(&self) -> &str;
}

/// Generates instrumented code with invariant checks.
///
/// Implementations inject assertions after state mutations.
pub trait CodeGenerator: Send + Sync {
    /// Generate instrumented code.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Invariant expressions reference undefined state
    /// - Type mismatches are detected
    /// - Code generation fails
    fn generate(
        &self,
        program: &ProgramModel,
        invariants: &[Invariant],
    ) -> Result<GenerationOutput>;

    /// Chain identifier.
    fn chain(&self) -> &str;
}

/// Simulates program execution to find invariant violations.
///
/// Implementations provide deterministic, fuzzing-based simulation.
pub trait Simulator: Send + Sync {
    /// Simulate execution against invariants.
    ///
    /// # Errors
    ///
    /// Returns an error if simulation setup fails.
    fn simulate(
        &self,
        program: &ProgramModel,
        invariants: &[Invariant],
    ) -> Result<SimulationReport>;

    /// Chain identifier.
    fn chain(&self) -> &str;
}
