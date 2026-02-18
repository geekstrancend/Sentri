//! Move analyzer implementation.

use invar_core::model::ProgramModel;
use invar_core::traits::ChainAnalyzer;
use invar_core::Result;
use std::path::Path;
use tracing::info;

/// Analyzer for Move programs (Aptos/Sui).
pub struct MoveAnalyzer;

impl ChainAnalyzer for MoveAnalyzer {
    fn analyze(&self, path: &Path) -> Result<ProgramModel> {
        info!("Analyzing Move program at {:?}", path);

        let _source = std::fs::read_to_string(path).map_err(invar_core::InvarError::IoError)?;

        // Create a basic program model
        let program = ProgramModel::new(
            "move_module".to_string(),
            "move".to_string(),
            path.to_string_lossy().to_string(),
        );

        // TODO: Parse Move source
        Ok(program)
    }

    fn chain(&self) -> &str {
        "move"
    }
}
