//! Solana analyzer implementation.

use sentri_core::model::{FunctionModel, ProgramModel, StateVar};
use sentri_core::traits::ChainAnalyzer;
use sentri_core::Result;
use std::collections::BTreeSet;
use std::path::Path;
use tracing::{info, debug};

/// Analyzer for Solana Rust programs.
///
/// Performs static analysis on Solana/Anchor programs to extract:
/// - Program entry points (instruction handlers)
/// - Account accesses and state mutations
/// - Context structures and parameters
pub struct SolanaAnalyzer;

impl ChainAnalyzer for SolanaAnalyzer {
    fn analyze(&self, path: &Path) -> Result<ProgramModel> {
        info!("Analyzing Solana program at {:?}", path);

        // Read the Rust source file
        let source = std::fs::read_to_string(path).map_err(sentri_core::InvarError::IoError)?;

        debug!("Source file size: {} bytes", source.len());

        // Parse using syn to get proper AST
        let file = syn::parse_file(&source).map_err(|e| {
            sentri_core::InvarError::AnalysisFailed(format!("Failed to parse Rust: {}", e))
        })?;

        // Create a basic program model
        let mut program = ProgramModel::new(
            "solana_program".to_string(),
            "solana".to_string(),
            path.to_string_lossy().to_string(),
        );

        // Extract structs (account data and context structures)
        for item in &file.items {
            if let syn::Item::Struct(item_struct) = item {
                let struct_name = item_struct.ident.to_string();
                
                // Add as state variable
                let state_var = StateVar {
                    name: struct_name.clone(),
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
                
                // Check if this is an entrypoint (has #[entrypoint] or #[program] macro)
                let is_entrypoint = item_fn
                    .attrs
                    .iter()
                    .any(|attr| {
                        attr.path().is_ident("entrypoint") 
                            || attr.path().is_ident("program")
                            || attr.path().is_ident("instruction")
                    });

                // Extract parameters to determine account access
                let params: Vec<String> = item_fn
                    .sig
                    .inputs
                    .iter()
                    .filter_map(|inp| {
                        if let syn::FnArg::Typed(pat_type) = inp {
                            if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                                Some(pat_ident.ident.to_string())
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                    .collect();

                // Analyze function body for state accesses
                let (reads, mutates) = analyze_solana_function_body(&item_fn.block);

                let func = FunctionModel {
                    name: func_name,
                    parameters: params,
                    return_type: None,
                    mutates,
                    reads,
                    is_entry_point: is_entrypoint,
                    is_pure: false,
                };
                program.add_function(func);
            }
        }

        info!(
            "Extracted {} state structs and {} functions",
            program.state_vars.len(),
            program.functions.len()
        );
        Ok(program)
    }

    fn chain(&self) -> &str {
        "solana"
    }
}

/// Analyze a Solana function body for state access patterns.
fn analyze_solana_function_body(block: &syn::Block) -> (BTreeSet<String>, BTreeSet<String>) {
    let mut reads = BTreeSet::new();
    let mut mutates = BTreeSet::new();

    // Walk through statements to find account accesses
    for stmt in &block.stmts {
        analyze_statement(stmt, &mut reads, &mut mutates);
    }

    (reads, mutates)
}

/// Recursively analyze statements for state access patterns.
fn analyze_statement(
    stmt: &syn::Stmt,
    reads: &mut BTreeSet<String>,
    mutates: &mut BTreeSet<String>,
) {
    match stmt {
        syn::Stmt::Expr(expr, _) => {
            analyze_expression(expr, reads, mutates);
        }
        syn::Stmt::Local(local) => {
            if let syn::Pat::Ident(pat_ident) = &local.pat {
                let var_name = pat_ident.ident.to_string();
                // Check if variable is marked as mutable
                if pat_ident.mutability.is_some() {
                    mutates.insert(var_name);
                } else if let Some(init) = &local.init {
                    analyze_expression(&init.expr, reads, mutates);
                }
            }
        }
        syn::Stmt::Item(_) => {
            // Skip item declarations
        }
        syn::Stmt::Macro(_) => {
            // Macro calls - assume potential state access
            mutates.insert("macro_call".to_string());
        }
    }
}

/// Recursively analyze expressions for state access patterns.
fn analyze_expression(
    expr: &syn::Expr,
    reads: &mut BTreeSet<String>,
    mutates: &mut BTreeSet<String>,
) {
    match expr {
        syn::Expr::MethodCall(call) => {
            // Method calls might mutate state if mutable reference
            if let syn::Expr::Path(path_expr) = &*call.receiver {
                let path = &path_expr.path;
                if let Some(last_segment) = path.segments.last() {
                    let name = last_segment.ident.to_string();
                    
                    // Check if method name suggests mutation
                    if call.method.to_string().contains("mut") 
                        || call.method.to_string() == "set_inner"
                        || call.method.to_string() == "serialize" 
                        || call.method.to_string() == "save" {
                        mutates.insert(name);
                    } else {
                        reads.insert(name);
                    }
                }
            }
        }
        syn::Expr::Binary(binary) => {
            analyze_expression(&binary.left, reads, mutates);
            analyze_expression(&binary.right, reads, mutates);
        }
        syn::Expr::Block(expr_block) => {
            for stmt in &expr_block.block.stmts {
                analyze_statement(stmt, reads, mutates);
            }
        }
        syn::Expr::If(expr_if) => {
            analyze_expression(&expr_if.cond, reads, mutates);
            for stmt in &expr_if.then_branch.stmts {
                analyze_statement(stmt, reads, mutates);
            }
            if let Some((_, else_expr)) = &expr_if.else_branch {
                analyze_expression(else_expr, reads, mutates);
            }
        }
        syn::Expr::Path(path_expr) => {
            // Variable reference - count as read
            if let Some(last_segment) = path_expr.path.segments.last() {
                reads.insert(last_segment.ident.to_string());
            }
        }
        _ => {
            // Other expression types - skip detailed analysis
        }
    }
}
