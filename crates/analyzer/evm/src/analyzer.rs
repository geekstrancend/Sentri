//! EVM analyzer implementation.

use invar_core::model::ProgramModel;
use invar_core::traits::ChainAnalyzer;
use invar_core::Result;
use std::path::Path;
use tracing::info;

/// Analyzer for EVM (Solidity) smart contracts.
pub struct EvmAnalyzer;

impl ChainAnalyzer for EvmAnalyzer {
    fn analyze(&self, path: &Path) -> Result<ProgramModel> {
        info!("Analyzing EVM contract at {:?}", path);

        let _source = std::fs::read_to_string(path)
            .map_err(|e| invar_core::InvarError::IoError(e))?;

        // Create a basic program model
        let program = ProgramModel::new(
            "evm_contract".to_string(),
            "evm".to_string(),
            path.to_string_lossy().to_string(),
        );

        // TODO: Parse Solidity source with solang parser
        Ok(program)
    }

    fn chain(&self) -> &str {
        "evm"
    }
}
