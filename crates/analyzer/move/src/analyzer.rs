//! Move analyzer implementation.

use sentri_core::model::{FunctionModel, ProgramModel};
use sentri_core::traits::ChainAnalyzer;
use sentri_core::Result;
use std::collections::BTreeSet;
use std::path::Path;
use tracing::info;

/// Analyzer for Move programs (Aptos/Sui).
///
/// Performs static analysis on Move code to extract:
/// - Module names and entry points
/// - Function signatures and visibility
/// - Resource types and access patterns
/// - Mutation and read operations
pub struct MoveAnalyzer;

impl ChainAnalyzer for MoveAnalyzer {
    fn analyze(&self, path: &Path) -> Result<ProgramModel> {
        info!("Analyzing Move program at {:?}", path);

        let source = std::fs::read_to_string(path).map_err(sentri_core::InvarError::IoError)?;

        // Extract module name
        let module_name = extract_module_name(&source).unwrap_or_else(|| "move_module".to_string());

        // Extract resource types
        let resources = extract_resource_types(&source);
        info!("Found {} resource types in Move module", resources.len());

        // Extract functions with proper analysis
        let functions = extract_functions_with_analysis(&source, &resources);
        info!("Found {} functions in Move module", functions.len());

        // Create program model with analyzed information
        let mut program = ProgramModel::new(
            module_name,
            "move".to_string(),
            path.to_string_lossy().to_string(),
        );

        // Add resource types as state variables
        for resource in &resources {
            use sentri_core::model::StateVar;
            program.add_state_var(StateVar {
                name: resource.clone(),
                type_name: "resource".to_string(),
                is_mutable: true,
                visibility: None,
            });
        }

        // Add extracted functions to the program model
        for func in functions {
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
            let name = module_part
                .split(|c: char| [':', '{', ';'].contains(&c))
                .next()?
                .trim();
            return Some(name.to_string());
        }
    }
    None
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
                if let Some(name) = struct_part
                    .split(|c: char| ['{', '(', '<', ';'].contains(&c))
                    .next()
                {
                    resources.push(name.trim().to_string());
                }
            }
        }
    }
    resources
}

/// Extract functions with full analysis including state access patterns.
fn extract_functions_with_analysis(source: &str, resources: &[String]) -> Vec<FunctionModel> {
    let mut functions = Vec::new();
    let lines: Vec<&str> = source.lines().collect();

    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim_start();

        // Look for function declarations
        if (trimmed.contains("public fun ") 
            || trimmed.contains("fun ")
            || trimmed.contains("entry fun "))
            && !trimmed.contains("//") {
            
            // Extract visibility
            let is_public = trimmed.contains("public ");
            let is_entry = trimmed.contains("entry ");

            // Extract function name
            let func_keyword = if trimmed.contains("entry fun ") {
                "entry fun "
            } else if trimmed.contains("public fun ") {
                "public fun "
            } else {
                "fun "
            };

            if let Some(func_part) = trimmed.split(func_keyword).nth(1) {
                if let Some(name) = func_part.split('(').next() {
                    let func_name = name.trim().to_string();

                    // Extract parameters
                    let params = extract_move_function_params(func_part);

                    // Check if function has mutable parameter (acquires or &mut)
                    let has_mutable_ref = func_part.contains("&mut ") || func_part.contains("acquires ");

                    // Analyze function body for resource access
                    let (reads, mutates) = analyze_move_function_body(&lines, i, resources);

                    // Check if function is pure (no mutation)
                    let is_pure = !has_mutable_ref && mutates.is_empty();
                    let is_entry_point = is_entry || (is_public && !reads.is_empty());

                    let func = FunctionModel {
                        name: func_name,
                        parameters: params,
                        return_type: None,
                        mutates,
                        reads,
                        is_entry_point,
                        is_pure,
                    };

                    functions.push(func);
                }
            }
        }

        i += 1;
    }

    functions
}

/// Extract Move function parameters.
fn extract_move_function_params(signature: &str) -> Vec<String> {
    if let Some(start) = signature.find('(') {
        if let Some(end) = signature.find(')') {
            let params_str = &signature[start + 1..end];
            if params_str.is_empty() {
                return Vec::new();
            }

            params_str
                .split(',')
                .map(|p| {
                    // Each param is like "account: &mut signer" or "amount: u64"
                    let parts: Vec<&str> = p.trim().split_whitespace().collect();
                    parts.get(0).unwrap_or(&"").to_string()
                })
                .filter(|p| !p.is_empty())
                .collect()
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    }
}

/// Analyze a Move function body to detect resource access patterns.
fn analyze_move_function_body(
    lines: &[&str],
    start_idx: usize,
    resources: &[String],
) -> (BTreeSet<String>, BTreeSet<String>) {
    let mut reads = BTreeSet::new();
    let mut mutates = BTreeSet::new();

    let mut brace_count = 0;
    let mut in_function = false;

    for i in start_idx..lines.len() {
        let line = lines[i];
        let trimmed = line.trim();

        // Count braces to detect function body
        for ch in line.chars() {
            if ch == '{' {
                in_function = true;
                brace_count += 1;
            } else if ch == '}' {
                brace_count -= 1;
                if in_function && brace_count == 0 {
                    return (reads, mutates);
                }
            }
        }

        if !in_function {
            continue;
        }

        // Look for resource accesses
        for resource in resources {
            if trimmed.contains(resource) {
                // Check for mutations
                if trimmed.contains("move_from") 
                    || trimmed.contains("borrow_global_mut")
                    || trimmed.contains("global_mut") {
                    mutates.insert(resource.clone());
                } else if trimmed.contains("borrow_global")
                    || trimmed.contains("global")
                    || trimmed.contains("assert!") {
                    reads.insert(resource.clone());
                }
            }
        }
    }

    (reads, mutates)
}
