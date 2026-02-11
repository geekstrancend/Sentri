//! Move generator implementation.

use invar_core::model::{GenerationOutput, Invariant, ProgramModel};
use invar_core::traits::CodeGenerator;
use invar_core::Result;
use tracing::info;

/// Code generator for Move programs.
pub struct MoveGenerator;

impl CodeGenerator for MoveGenerator {
    fn generate(
        &self,
        program: &ProgramModel,
        invariants: &[Invariant],
    ) -> Result<GenerationOutput> {
        info!(
            "Generating Move assertions for {} with {} invariants",
            program.name,
            invariants.len()
        );

        let mut assertions = Vec::new();
        for inv in invariants {
            assertions.push(format!(
                "assert!({}, E_INVARIANT_{});",
                inv.expression,
                inv.name.to_uppercase()
            ));
        }

        let code = format!(
            "// Generated Move invariant checks for {}\n// {} assertions\n",
            program.name,
            assertions.len()
        );

        Ok(GenerationOutput {
            code,
            assertions,
            tests: None,
            coverage_percent: 0,
        })
    }

    fn chain(&self) -> &str {
        "move"
    }
}
