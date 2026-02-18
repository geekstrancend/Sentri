//! Simulation engine.

use invar_core::model::{Invariant, ProgramModel, SimulationReport};
use invar_core::traits::Simulator;
use invar_core::Result;
use rand::SeedableRng;
use tracing::info;

/// Deterministic simulation engine for invariant testing.
pub struct SimulationEngine {
    /// RNG seed for reproducibility.
    pub seed: u64,
}

impl SimulationEngine {
    /// Create a new simulation engine with a seed.
    pub fn new(seed: u64) -> Self {
        Self { seed }
    }
}

impl Default for SimulationEngine {
    fn default() -> Self {
        Self { seed: 42 }
    }
}

impl Simulator for SimulationEngine {
    fn simulate(
        &self,
        _program: &ProgramModel,
        _invariants: &[Invariant],
    ) -> Result<SimulationReport> {
        info!("Starting simulation with seed: {}", self.seed);

        // Initialize RNG with seed
        let _rng = rand::rngs::SmallRng::seed_from_u64(self.seed);

        // TODO: Implement fuzzing
        Ok(SimulationReport {
            violations: 0,
            traces: Vec::new(),
            coverage: 0.0,
            seed: self.seed,
        })
    }

    fn chain(&self) -> &str {
        "generic"
    }
}
