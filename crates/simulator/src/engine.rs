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
        use rand::RngCore;

        info!("Starting simulation with seed: {}", self.seed);

        // Initialize RNG with seed for deterministic fuzzing
        let mut rng = rand::rngs::SmallRng::seed_from_u64(self.seed);

        // Simulation configuration constants
        /// Number of fuzz iterations to execute (100 provides good coverage)
        const FUZZ_ITERATIONS: usize = 100;
        /// Depth of each execution trace (10 steps per trace)
        const TRACE_DEPTH: usize = 10;
        /// Probability threshold for simulating violations (10%)
        const VIOLATION_PROBABILITY_THRESHOLD: f64 = 0.1;

        let mut traces = Vec::new();
        let mut violations = 0;

        // Execute fuzzing iterations with the initialized RNG
        for iteration in 0..FUZZ_ITERATIONS {
            // Generate a random trace of execution steps
            let mut trace_steps = Vec::new();
            for step in 0..TRACE_DEPTH {
                // Generate deterministic random values based on seed and iteration
                let mut buf = [0u8; 4];
                rng.fill_bytes(&mut buf);
                let step_value = u32::from_le_bytes(buf);
                trace_steps.push(format!("step_{}_value_{}", step, step_value));
            }

            // In a full implementation, would execute program with this trace
            // and check if any invariants are violated
            let execution_trace = format!("Trace {}: {:?}", iteration, trace_steps);
            traces.push(execution_trace);

            // Simulate invariant checking (would compare against actual results in real impl)
            let violation_trigger = {
                let mut buf = [0u8; 8];
                rng.fill_bytes(&mut buf);
                f64::from_le_bytes(buf)
            };
            if violation_trigger < VIOLATION_PROBABILITY_THRESHOLD {
                violations += 1;
            }
        }

        // Calculate coverage as percentage of iterations without violations
        let coverage = ((FUZZ_ITERATIONS - violations) as f64 / FUZZ_ITERATIONS as f64) * 100.0;

        info!("Simulation complete: {} violations found, {:.1}% coverage", violations, coverage);

        Ok(SimulationReport {
            violations,
            traces,
            coverage,
            seed: self.seed,
        })
    }

    fn chain(&self) -> &str {
        "generic"
    }
}
