//! EVM analyzer implementation.

use invar_core::model::{ProgramModel, FunctionModel};
use invar_core::traits::ChainAnalyzer;
use invar_core::Result;
use std::path::Path;
use std::collections::BTreeSet;
use tracing::info;

/// Analyzer for EVM (Solidity) smart contracts.
pub struct EvmAnalyzer;

impl ChainAnalyzer for EvmAnalyzer {
    fn analyze(&self, path: &Path) -> Result<ProgramModel> {
        info!("Analyzing EVM contract at {:?}", path);

        let source = std::fs::read_to_string(path)
            .map_err(invar_core::InvarError::IoError)?;

        // Parse Solidity source code
        let contract_name = extract_contract_name(&source)
            .unwrap_or_else(|| "UnknownContract".to_string());

        let functions = extract_public_functions(&source);
        info!("Found {} public functions in contract", functions.len());

        let state_vars = extract_state_variables(&source);
        info!("Found {} state variables", state_vars.len());

        // Create program model with analyzed information
        let mut program = ProgramModel::new(
            contract_name,
            "evm".to_string(),
            path.to_string_lossy().to_string(),
        );

        // Add extracted functions to the program model
        for func_name in functions {
            let func = FunctionModel {
                name: func_name,
                parameters: Vec::new(),
                return_type: None,
                mutates: BTreeSet::new(),
                reads: BTreeSet::new(),
                is_entry_point: true,
                is_pure: false,
            };
            program.add_function(func);
        }

        Ok(program)
    }

    fn chain(&self) -> &str {
        "evm"
    }
}

/// Extract contract name from Solidity source code.
fn extract_contract_name(source: &str) -> Option<String> {
    for line in source.lines() {
        if line.trim_start().starts_with("contract ") {
            let contract_part = line.split("contract ").nth(1)?;
            let name = contract_part.split(|c: char| c == '{' || c == '(' || c == ';')
                .next()?
                .trim();
            return Some(name.to_string());
        }
    }
    None
}

/// Extract public and external function names from Solidity source code.
fn extract_public_functions(source: &str) -> Vec<String> {
    let mut functions = Vec::new();
    for line in source.lines() {
        let trimmed = line.trim_start();
        if (trimmed.contains("public ") || trimmed.contains("external ")) && trimmed.contains("function ") {
            if let Some(func_part) = trimmed.split("function ").nth(1) {
                if let Some(name) = func_part.split('(').next() {
                    functions.push(name.trim().to_string());
                }
            }
        }
    }
    functions
}

/// Extract state variable names from Solidity source code.
fn extract_state_variables(source: &str) -> Vec<String> {
    let mut variables = Vec::new();
    for line in source.lines() {
        let trimmed = line.trim_start();
        // Match state variable declarations (e.g., "uint256 public balance;")
        if is_state_variable_declaration(trimmed) {
            if let Some(var_name) = extract_variable_name(trimmed) {
                variables.push(var_name);
            }
        }
    }
    variables
}

/// Determine if a line is a state variable declaration.
fn is_state_variable_declaration(line: &str) -> bool {
    let types = ["uint", "int", "address", "bool", "bytes", "string", "mapping"];
    types.iter().any(|t| line.starts_with(t)) && !line.contains("function")
}

/// Extract variable name from declaration (e.g., "uint256 public balance;" → "balance").
fn extract_variable_name(line: &str) -> Option<String> {
    let name_part = line.split_whitespace()
        .skip_while(|w| w.starts_with("uint") || w.starts_with("int") || 
                        w == &"public" || w == &"private" || w == &"mapping" ||
                        w == &"address" || w == &"bool" || w == &"bytes" || w == &"string")
        .next()?;
    let name = name_part.split(|c: char| c == ';' || c == '=' || c == '(' || c == '[')
        .next()?
        .trim();
    if name.is_empty() {
        None
    } else {
        Some(name.to_string())
    }
}
