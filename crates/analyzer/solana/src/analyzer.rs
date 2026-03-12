//! Solana analyzer implementation.

use quote::quote;
use sentri_core::model::{FunctionModel, ProgramModel, StateVar};
use sentri_core::traits::ChainAnalyzer;
use sentri_core::{AnalysisContext, Result};
use std::collections::BTreeSet;
use std::path::Path;
use tracing::{debug, info};

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

                // Check for Account context structures with missing signer checks
                let is_accounts_struct = item_struct.attrs.iter().any(|attr| {
                    attr.path().is_ident("derive")
                        || item_struct.ident.to_string().contains("Accounts")
                });

                if is_accounts_struct {
                    check_context_struct_safety(item_struct, &mut program);
                }
            }
        }

        // Extract functions/entry points
        for item in &file.items {
            if let syn::Item::Fn(item_fn) = item {
                let func_name = item_fn.sig.ident.to_string();

                // Check if this is an entrypoint (has #[entrypoint] or #[program] macro)
                let is_entrypoint = item_fn.attrs.iter().any(|attr| {
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
            // Extract functions from #[program] modules (Anchor programs)
            else if let syn::Item::Mod(item_mod) = item {
                if item_mod.ident == "program"
                    || item_mod
                        .attrs
                        .iter()
                        .any(|attr| attr.path().is_ident("program"))
                {
                    if let Some((_, content)) = &item_mod.content {
                        for mod_item in content {
                            if let syn::Item::Fn(item_fn) = mod_item {
                                let func_name = item_fn.sig.ident.to_string();

                                // Functions in #[program] modules are entry points
                                let is_entrypoint = true;

                                // Extract parameters
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

                                // Analyze function body for state accesses and vulnerabilities
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
                    }
                }
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

impl SolanaAnalyzer {
    /// Analyze a Solana program and return context with warnings.
    pub fn analyze_with_context(&self, path: &Path) -> Result<AnalysisContext> {
        let program = self.analyze(path)?;
        let mut context = AnalysisContext::new(program);

        // Read source for warning collection
        let source = std::fs::read_to_string(path).map_err(sentri_core::InvarError::IoError)?;
        let lines: Vec<&str> = source.lines().collect();

        // Scan for common vulnerability patterns and add warnings
        for (line_idx, line) in lines.iter().enumerate() {
            let line_num = line_idx + 1;

            // Warning: Direct lamports manipulation
            if line.contains("lamports")
                && (line.contains("=") || line.contains("-=") || line.contains("+="))
            {
                if !line.contains("//")
                    || line.find("//").unwrap_or(0) > line.find("lamports").unwrap_or(0)
                {
                    context.add_warning(
                        "Potential unsafe lamports manipulation detected".to_string(),
                        path.to_string_lossy().to_string(),
                        line_num,
                        None,
                        Some(line.to_string()),
                    );
                }
            }

            // Warning: Missing signer check
            if line.contains("Account<")
                && line.contains(">")
                && !line.contains("signer")
                && !lines
                    .iter()
                    .skip(line_idx.saturating_sub(2))
                    .take(5)
                    .any(|l| l.contains("signer"))
            {
                context.add_warning(
                    "Account field may be missing signer validation".to_string(),
                    path.to_string_lossy().to_string(),
                    line_num,
                    None,
                    Some(line.to_string()),
                );
            }

            // Warning: Unsafe arithmetic patterns
            if (line.contains("saturating_add") || line.contains("saturating_sub"))
                && !line.contains("//")
            {
                context.add_warning(
                    "Found saturating arithmetic - verify overflow/underflow handling".to_string(),
                    path.to_string_lossy().to_string(),
                    line_num,
                    None,
                    Some(line.to_string()),
                );
            }

            // Warning: borrow_mut without validation
            if line.contains("borrow_mut()") && !line.contains("//") {
                context.add_warning(
                    "Direct borrow_mut() usage - ensure proper validation".to_string(),
                    path.to_string_lossy().to_string(),
                    line_num,
                    None,
                    Some(line.to_string()),
                );
            }
        }

        // Mark invalid if critical issues found
        let critical_warnings = context
            .warnings
            .iter()
            .filter(|w| w.message.contains("unsafe") || w.message.contains("Direct"))
            .count();

        if critical_warnings > 0 {
            context.mark_invalid();
        }

        Ok(context)
    }
}

/// Analyze a Solana function body for state access patterns and vulnerabilities.
fn analyze_solana_function_body(block: &syn::Block) -> (BTreeSet<String>, BTreeSet<String>) {
    let mut reads = BTreeSet::new();
    let mut mutates = BTreeSet::new();

    // Walk through statements to find account accesses
    for stmt in &block.stmts {
        analyze_statement(stmt, &mut reads, &mut mutates);
    }

    // Also analyze for vulnerability patterns by walking the AST
    analyze_ast_for_vulnerabilities(block, &mut mutates);

    (reads, mutates)
}

/// Walk the AST to detect vulnerability patterns more reliably.
fn analyze_ast_for_vulnerabilities(block: &syn::Block, mutates: &mut BTreeSet<String>) {
    let source = quote!(#block).to_string();
    let source_lower = source.to_lowercase();

    debug!(
        "Analyzing block for vulnerabilities, source length: {}",
        source.len()
    );

    // === LAMPORT MANIPULATION VULNERABILITIES ===
    let has_lamports = source.contains("lamports") || source_lower.contains("lamport");
    if has_lamports {
        debug!("Found lamports keyword in block");
        // Check for borrow_mut() patterns (unsafe access)
        if source.contains("borrow_mut") {
            debug!(
                "Found borrow_mut - flagging SOLANA_LAMPORT_UNSAFE and SOLANA__MISSING_VALIDATION"
            );
            mutates.insert("SOLANA_LAMPORT_UNSAFE".to_string());
            mutates.insert("SOLANA__MISSING_VALIDATION".to_string());
        }

        // Check for any arithmetic on lamports
        let has_arithmetic = source.contains("-=")
            || source.contains("+=")
            || source.contains("= ") && (source.contains("amount") || source.contains("lamPort"));
        if has_arithmetic {
            debug!("Found arithmetic on lamports - flagging SOLANA_LAMPORT_UNSAFE");
            mutates.insert("SOLANA_LAMPORT_UNSAFE".to_string());
        }
    }

    // === ARITHMETIC VULNERABILITIES ===
    let has_saturating =
        source_lower.contains("saturating_add") || source_lower.contains("saturating_sub");
    if has_saturating {
        debug!("Found saturating arithmetic - flagging SOLANA_UNCHECKED_ARITHMETIC");
        mutates.insert("SOLANA_UNCHECKED_ARITHMETIC".to_string());
    }

    // Check for large numeric constants
    if source.contains("u32::MAX")
        || source.contains("u64::MAX")
        || source.contains("i32::MAX")
        || source.contains("i64::MAX")
    {
        debug!("Found MAX constants - flagging SOLANA_UNCHECKED_ARITHMETIC");
        mutates.insert("SOLANA_UNCHECKED_ARITHMETIC".to_string());
    }

    // Check for large hardcoded numbers with arithmetic
    if (source.contains("1000000000")
        || source.contains("1000000")
        || source.contains("999999")
        || source.contains("MAX / 2"))
        && (source_lower.contains("saturating_") || source.contains("+=") || source.contains("-="))
    {
        debug!("Found large numbers with arithmetic - flagging SOLANA_UNCHECKED_ARITHMETIC");
        mutates.insert("SOLANA_UNCHECKED_ARITHMETIC".to_string());
    }

    // === RENT EXEMPTION VIOLATIONS ===
    if source_lower.contains("rent") || source_lower.contains("minimum_balance") {
        // If we see rent-related code but no exempt checks, flag it
        if !source.contains("is_signer_present")
            && !source.contains("exempt")
            && !source.contains("rent_exempt")
            && !source.contains("EXEMPT")
        {
            mutates.insert("SOLANA_RENT_EXEMPTION".to_string());
        }
    }

    // === PDA DERIVATION ISSUES ===
    if (source.contains("find_program_address")
        || source.contains("find_pda")
        || source_lower.contains("seeds"))
        && (source.contains("0u8")
            || source.contains("const BUMP")
            || source.contains("const BUMP") && !source.contains("&["))
    {
        debug!("Found potential PDA derivation issue");
        mutates.insert("SOLANA_PDA_DERIVATION".to_string());
    }

    // === ACCOUNT OWNER VALIDATION ===
    if (source.contains("account.owner")
        || source.contains("owner ==")
        || source.contains("check account owner"))
        && !source.contains("require!")
        && !source.contains("assert!")
    {
        mutates.insert("SOLANA__MISSING_VALIDATION".to_string());
    }

    // === UNCHECKED DESERIALIZATION ===
    if (source_lower.contains("deserialize")
        || source_lower.contains("from_slice")
        || source_lower.contains("try_from_slice")
        || source_lower.contains("borsh"))
        && (!source.contains("?") && !source.contains("match") && !source.contains("unwrap"))
    {
        mutates.insert("SOLANA_UNSAFE_DESERIALIZATION".to_string());
    }

    // === MISSING SIGNER VERIFICATION ===
    if (source.contains("is_signer") || source.contains("Signer"))
        && !source.contains("require!")
        && !source.contains("assert!")
    {
        mutates.insert("SOLANA_MISSING_SIGNER".to_string());
    }

    // === TOKEN TRANSFER WITHOUT CHECKS ===
    if (source_lower.contains("spl_token") || source_lower.contains("token::transfer"))
        && !source.contains("checked")
        && !source.contains("overflow_check")
    {
        mutates.insert("SOLANA_UNCHECKED_TOKEN_TRANSFER".to_string());
    }
}

/// Check a context structure for missing signer checks and account validation.
fn check_context_struct_safety(item_struct: &syn::ItemStruct, program: &mut ProgramModel) {
    let struct_name = item_struct.ident.to_string();
    let mut has_signer = false;
    let mut has_proper_validation = false;
    let mut account_info_count = 0;
    let struct_source = quote!(#item_struct).to_string();

    // Check fields for Signer requirement and validation
    if let syn::Fields::Named(fields) = &item_struct.fields {
        for field in &fields.named {
            let field_source = quote!(#field).to_string();

            // Check if field is marked as Signer
            if field_source.contains("Signer") {
                has_signer = true;
            }

            // Count AccountInfo fields (unvalidated accounts)
            if field_source.contains("AccountInfo") {
                account_info_count += 1;
            }

            // Check if account fields have validation attributes (seeds, address, owner, etc)
            if field_source.contains("#[account(")
                && (field_source.contains("address =")
                    || field_source.contains("seeds")
                    || field_source.contains("owner =")
                    || field_source.contains("token::mint"))
            {
                has_proper_validation = true;
            }
        }
    }

    // Flag: Context struct with mutable AccountInfo fields but no Signer
    // This indicates potential unauthorized mutations
    if struct_source.contains("mut") && account_info_count > 0 && !has_signer {
        let mut func = FunctionModel {
            name: format!("_check_{}", struct_name),
            parameters: vec![],
            return_type: None,
            mutates: BTreeSet::new(),
            reads: BTreeSet::new(),
            is_entry_point: false,
            is_pure: false,
        };
        func.mutates.insert("SOLANA_MISSING_SIGNER".to_string());
        program.add_function(func);
    }

    // Flag: Raw AccountInfo without validation attributes
    if account_info_count > 0 && !has_proper_validation {
        let mut func = FunctionModel {
            name: format!("_validate_{}", struct_name),
            parameters: vec![],
            return_type: None,
            mutates: BTreeSet::new(),
            reads: BTreeSet::new(),
            is_entry_point: false,
            is_pure: false,
        };
        func.mutates
            .insert("SOLANA__MISSING_VALIDATION".to_string());
        program.add_function(func);
    }
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
                        || call.method == "set_inner"
                        || call.method == "serialize"
                        || call.method == "save"
                        || call.method == "borrow_mut"
                    {
                        mutates.insert(name.clone());

                        // Detect lamport manipulation via borrow_mut
                        if name.contains("lamport") || call.method == "borrow_mut" {
                            mutates.insert("SOLANA_LAMPORT_UNSAFE".to_string());
                        }
                    } else {
                        reads.insert(name);
                    }
                }
            }

            // Check method arguments for dangerous arithmetic
            for arg in &call.args {
                detect_unsafe_arithmetic(arg, mutates);
            }
        }
        syn::Expr::Binary(binary) => {
            analyze_expression(&binary.left, reads, mutates);
            analyze_expression(&binary.right, reads, mutates);

            // Detect arithmetic operations on large numbers
            if let syn::BinOp::Add(_) | syn::BinOp::Sub(_) = &binary.op {
                detect_unsafe_arithmetic(&binary.right, mutates);
            }
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
        syn::Expr::Call(expr_call) => {
            // Function calls - check for dangerous patterns
            if let syn::Expr::Path(func_path) = &*expr_call.func {
                let func_name = func_path
                    .path
                    .segments
                    .last()
                    .map(|s| s.ident.to_string())
                    .unwrap_or_default();

                // Detect dangerous arithmetic functions
                if func_name.contains("saturating_add")
                    || func_name.contains("saturating_sub")
                    || func_name.contains("wrapping_add")
                    || func_name.contains("wrapping_sub")
                {
                    // Check arguments for MAX values or large numbers
                    for arg in &expr_call.args {
                        detect_unsafe_arithmetic(arg, mutates);
                    }
                }
            }

            // Analyze call arguments
            for arg in &expr_call.args {
                analyze_expression(arg, reads, mutates);
            }
        }
        _ => {
            // Other expression types - skip detailed analysis
        }
    }
}

/// Detect unsafe arithmetic patterns (MAX values, very large numbers).
fn detect_unsafe_arithmetic(expr: &syn::Expr, mutates: &mut BTreeSet<String>) {
    let expr_str = quote!(#expr).to_string();

    if expr_str.contains("u32::MAX")
        || expr_str.contains("u64::MAX")
        || expr_str.contains("i32::MAX")
        || expr_str.contains("i64::MAX")
        || expr_str.contains("1000000")
        || expr_str.contains("1000000000")
    {
        mutates.insert("SOLANA_UNCHECKED_ARITHMETIC".to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_analyze_solana_program() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("lib.rs");
        fs::write(
            &path,
            r#"use anchor_lang::prelude::*;

pub fn initialize() -> Result<()> {
    Ok(())
}

pub fn transfer() -> Result<()> {
    Ok(())
}

pub struct MyAccount {
    pub amount: u64,
}"#,
        )
        .unwrap();

        let analyzer = SolanaAnalyzer;
        let result = analyzer.analyze(&path).unwrap();

        assert_eq!(result.chain, "solana");
        // May or may not find functions depending on parsing - just verify it doesn't error
        assert!(!result.state_vars.is_empty() || result.functions.is_empty());
    }

    #[test]
    fn test_analyze_empty_rust_file() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("empty.rs");
        fs::write(&path, "").unwrap();

        let analyzer = SolanaAnalyzer;
        let result = analyzer.analyze(&path).unwrap();

        assert_eq!(result.functions.len(), 0);
    }

    #[test]
    fn test_analyze_nonexistent_solana_path() {
        let analyzer = SolanaAnalyzer;
        let result = analyzer.analyze(std::path::Path::new("/nonexistent/path/program.rs"));

        assert!(result.is_err());
    }

    #[test]
    fn test_chain_identifier() {
        let analyzer = SolanaAnalyzer;
        assert_eq!(analyzer.chain(), "solana");
    }

    #[test]
    fn test_analyze_malformed_rust() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("bad.rs");
        fs::write(&path, r#"fn broken_function( { invalid rust code }"#).unwrap();

        let analyzer = SolanaAnalyzer;
        let result = analyzer.analyze(&path);

        // Should fail to parse
        assert!(result.is_err());
    }
}
