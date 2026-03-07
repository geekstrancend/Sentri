//! Solana generator implementation.

use sentri_core::model::{GenerationOutput, Invariant, ProgramModel};
use sentri_core::traits::CodeGenerator;
use sentri_core::Result;
use tracing::info;

/// Code generator for Solana Rust programs.
pub struct SolanaGenerator;

impl CodeGenerator for SolanaGenerator {
    fn generate(
        &self,
        program: &ProgramModel,
        invariants: &[Invariant],
    ) -> Result<GenerationOutput> {
        info!(
            "Generating code for {} with {} invariants",
            program.name,
            invariants.len()
        );

        let mut assertions = Vec::new();
        for inv in invariants {
            assertions.push(format!(
                "assert!({}, \"Invariant {} violated\");",
                inv.expression, inv.name
            ));
        }

        let code = format!(
            "// Generated invariant checks for {}\n// {} invariants injected\n",
            program.name,
            assertions.len()
        );

        let coverage_percent = if program.functions.is_empty() {
            0
        } else {
            (assertions.len() as u8).min(100)
        };

        Ok(GenerationOutput {
            code,
            assertions,
            tests: None,
            coverage_percent,
        })
    }

    fn chain(&self) -> &str {
        "solana"
    }
}
