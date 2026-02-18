//! Solana analyzer implementation.

use invar_core::model::{FunctionModel, ProgramModel, StateVar};
use invar_core::traits::ChainAnalyzer;
use invar_core::Result;
use std::path::Path;
use tracing::{debug, info};

/// Analyzer for Solana Rust programs.
pub struct SolanaAnalyzer;

impl ChainAnalyzer for SolanaAnalyzer {
    fn analyze(&self, path: &Path) -> Result<ProgramModel> {
        info!("Analyzing Solana program at {:?}", path);

        // Read the Rust source file
        let source = std::fs::read_to_string(path).map_err(invar_core::InvarError::IoError)?;

        debug!("Source file size: {} bytes", source.len());

        // Create a basic program model
        let mut program = ProgramModel::new(
            "solana_program".to_string(),
            "solana".to_string(),
            path.to_string_lossy().to_string(),
        );

        // Parse using syn
        let file = syn::parse_file(&source).map_err(|e| {
            invar_core::InvarError::AnalysisFailed(format!("Failed to parse Rust: {}", e))
        })?;

        // Extract structs (potential state)
        for item in &file.items {
            if let syn::Item::Struct(item_struct) = item {
                let state_var = StateVar {
                    name: item_struct.ident.to_string(),
                    type_name: "struct".to_string(),
                    is_mutable: false,
                    visibility: None,
                };
                program.add_state_var(state_var);
            }
        }

        // Extract functions/entry points
        for item in &file.items {
            if let syn::Item::Fn(item_fn) = item {
                let func_name = item_fn.sig.ident.to_string();
                let is_entry = item_fn
                    .attrs
                    .iter()
                    .any(|attr| attr.path().is_ident("solana_program::entrypoint"));

                let func = FunctionModel {
                    name: func_name,
                    parameters: item_fn
                        .sig
                        .inputs
                        .iter()
                        .map(|_inp| "param".to_string())
                        .collect(),
                    return_type: None,
                    mutates: Default::default(),
                    reads: Default::default(),
                    is_entry_point: is_entry,
                    is_pure: false,
                };
                program.add_function(func);
            }
        }

        info!(
            "Extracted {} state vars and {} functions",
            program.state_vars.len(),
            program.functions.len()
        );
        Ok(program)
    }

    fn chain(&self) -> &str {
        "solana"
    }
}
