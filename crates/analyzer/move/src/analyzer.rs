//! Move analyzer implementation.

use invar_core::model::{ProgramModel, FunctionModel};
use invar_core::traits::ChainAnalyzer;
use invar_core::Result;
use std::path::Path;
use std::collections::BTreeSet;
use tracing::info;

/// Analyzer for Move programs (Aptos/Sui).
pub struct MoveAnalyzer;

impl ChainAnalyzer for MoveAnalyzer {
    fn analyze(&self, path: &Path) -> Result<ProgramModel> {
        info!("Analyzing Move program at {:?}", path);

        let source = std::fs::read_to_string(path)
            .map_err(invar_core::InvarError::IoError)?;

        // Parse Move source code
        let module_name = extract_module_name(&source)
            .unwrap_or_else(|| "move_module".to_string());

        let functions = extract_public_functions(&source);
        info!("Found {} public functions in Move module", functions.len());

        let structs = extract_resource_types(&source);
        info!("Found {} resource types", structs.len());

        // Create program model with analyzed information
        let mut program = ProgramModel::new(
            module_name,
            "move".to_string(),
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
        "move"
    }
}

/// Extract module name from Move source code.
fn extract_module_name(source: &str) -> Option<String> {
    for line in source.lines() {
        if line.trim_start().starts_with("module ") {
            let module_part = line.split("module ").nth(1)?;
            let name = module_part.split(|c: char| c == ':' || c == '{' || c == ';')
                .next()?
                .trim();
            return Some(name.to_string());
        }
    }
    None
}

/// Extract public function names from Move source code.
fn extract_public_functions(source: &str) -> Vec<String> {
    let mut functions = Vec::new();
    for line in source.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("public fun ") {
            if let Some(func_part) = trimmed.split("public fun ").nth(1) {
                if let Some(name) = func_part.split('(').next() {
                    functions.push(name.trim().to_string());
                }
            }
        }
    }
    functions
}

/// Extract resource type names from Move source code.
fn extract_resource_types(source: &str) -> Vec<String> {
    let mut resources = Vec::new();
    for line in source.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("struct ") || trimmed.starts_with("resource struct ") {
            let key = if trimmed.starts_with("resource struct ") {
                "resource struct "
            } else {
                "struct "
            };
            if let Some(struct_part) = trimmed.split(key).nth(1) {
                if let Some(name) = struct_part.split(|c: char| c == '{' || c == '(' || c == '<')
                    .next() {
                    resources.push(name.trim().to_string());
                }
            }
        }
    }
    resources
}
