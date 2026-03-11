//! EVM analyzer implementation.

use sentri_core::model::{FunctionModel, ProgramModel};
use sentri_core::traits::ChainAnalyzer;
use sentri_core::Result;
use std::collections::BTreeSet;
use std::path::Path;
use tracing::{debug, info};

/// Analyzer for EVM (Solidity) smart contracts.
///
/// Performs static analysis on Solidity source code to extract:
/// - Contract name and structure
/// - Function signatures and visibility
/// - State variable declarations
/// - State access patterns (reads and writes)
/// - Function modifiers and guards
pub struct EvmAnalyzer;

impl ChainAnalyzer for EvmAnalyzer {
    fn analyze(&self, path: &Path) -> Result<ProgramModel> {
        info!("Analyzing EVM contract at {:?}", path);

        let source = std::fs::read_to_string(path).map_err(sentri_core::InvarError::IoError)?;

        // Extract contract name
        let contract_name =
            extract_contract_name(&source).unwrap_or_else(|| "UnknownContract".to_string());

        // Extract state variables for better analysis
        let state_vars = extract_state_variables(&source);
        debug!("Found {} state variables", state_vars.len());

        // Extract functions with full analysis
        let functions = extract_functions_with_analysis(&source, &state_vars);
        info!("Found {} functions in contract", functions.len());

        // Create program model with analyzed information
        let mut program = ProgramModel::new(
            contract_name,
            "evm".to_string(),
            path.to_string_lossy().to_string(),
        );

        // Add state variables to the program
        for var_name in &state_vars {
            use sentri_core::model::StateVar;
            program.add_state_var(StateVar {
                name: var_name.clone(),
                type_name: "state_var".to_string(),
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
        "evm"
    }
}

/// Extract contract name from Solidity source code.
fn extract_contract_name(source: &str) -> Option<String> {
    for line in source.lines() {
        if line.trim_start().starts_with("contract ") {
            let contract_part = line.split("contract ").nth(1)?;
            let name = contract_part
                .split(|c: char| ['{', '(', ';'].contains(&c))
                .next()?
                .trim();
            return Some(name.to_string());
        }
    }
    None
}

/// Extract state variable names from Solidity source code.
fn extract_state_variables(source: &str) -> Vec<String> {
    let mut variables = Vec::new();
    for line in source.lines() {
        let trimmed = line.trim_start();
        // Skip comments and function/contract declarations
        if trimmed.starts_with("//")
            || trimmed.starts_with("/*")
            || trimmed.contains("function ")
            || trimmed.contains("contract ")
        {
            continue;
        }

        // Match state variable types
        let solidity_types = [
            "uint",
            "int",
            "address",
            "bool",
            "bytes",
            "string",
            "mapping",
            "struct",
            "enum",
            "bool[]",
            "uint[]",
            "address[]",
        ];

        if solidity_types.iter().any(|t| trimmed.starts_with(t)) {
            if let Some(var_name) = extract_variable_name(trimmed) {
                variables.push(var_name);
            }
        }
    }
    variables
}

/// Extract functions with full analysis including state access patterns.
fn extract_functions_with_analysis(source: &str, state_vars: &[String]) -> Vec<FunctionModel> {
    let mut functions = Vec::new();
    let lines: Vec<&str> = source.lines().collect();

    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim_start();

        // Look for function signatures
        if (trimmed.contains("public ")
            || trimmed.contains("external ")
            || trimmed.contains("internal ")
            || trimmed.contains("private "))
            && trimmed.contains("function ")
        {
            // Extract function name and details
            if let Some(func_part) = trimmed.split("function ").nth(1) {
                if let Some(name) = func_part.split('(').next() {
                    let func_name = name.trim().to_string();

                    // Extract modifiers
                    let has_guards = trimmed.contains("require(")
                        || trimmed.contains("onlyOwner")
                        || trimmed.contains("nonReentrant")
                        || trimmed.to_lowercase().contains("modifier");

                    // Check visibility
                    let is_public = trimmed.contains("public ");
                    let is_external = trimmed.contains("external ");

                    // Extract parameters
                    let params = extract_function_params(func_part);

                    // Analyze function body for state access
                    let (reads, mutates) = analyze_function_body(&lines, i, state_vars);

                    // Extract return type
                    let return_type = if trimmed.contains("returns ") {
                        Some("unknown".to_string())
                    } else {
                        None
                    };

                    // Determine if function is pure (no state mutations or views)
                    let is_pure = !trimmed.contains("constant")
                        && !trimmed.contains("view")
                        && mutates.is_empty();

                    let func = FunctionModel {
                        name: func_name,
                        parameters: params,
                        return_type,
                        mutates,
                        reads,
                        is_entry_point: is_public || is_external && !has_guards,
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

/// Extract function parameters from a function signature.
fn extract_function_params(signature: &str) -> Vec<String> {
    // Extract content between parentheses
    if let Some(start) = signature.find('(') {
        if let Some(end) = signature.find(')') {
            let params_str = &signature[start + 1..end];
            if params_str.is_empty() {
                return Vec::new();
            }

            // Split by comma and extract parameter names
            params_str
                .split(',')
                .map(|p| {
                    // Each param is like "uint256 amount" or "address indexed user"
                    let parts: Vec<&str> = p.split_whitespace().collect();
                    if parts.len() >= 2 {
                        parts[parts.len() - 1].to_string()
                    } else {
                        parts.first().unwrap_or(&"").to_string()
                    }
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

/// Analyze a function body to detected state mutations and reads.
fn analyze_function_body(
    lines: &[&str],
    start_idx: usize,
    state_vars: &[String],
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
                    // Analyze accumulated vulnerabilities before returning
                    analyze_evm_vulnerabilities(lines, start_idx, i, &mut mutates);
                    return (reads, mutates);
                }
            }
        }

        if !in_function {
            continue;
        }

        // Look for state variable accesses
        for var in state_vars {
            if trimmed.contains(var) {
                // Check if it's a read or write
                if trimmed.contains(&format!("{} =", var))
                    || trimmed.contains(&format!("{}[", var))
                    || trimmed.contains(&format!("{}.push", var))
                    || trimmed.contains(&format!("{}.pop", var))
                {
                    mutates.insert(var.clone());
                } else {
                    reads.insert(var.clone());
                }
            }
        }

        // Also detect state mutations through common patterns
        if trimmed.contains("require(") || trimmed.contains("assert(") {
            // These are reads of state
            for var in state_vars {
                if trimmed.contains(var) {
                    reads.insert(var.clone());
                }
            }
        }
    }

    (reads, mutates)
}

/// Detect Solidity-specific security vulnerabilities.
fn analyze_evm_vulnerabilities(
    lines: &[&str],
    start_idx: usize,
    end_idx: usize,
    mutates: &mut BTreeSet<String>,
) {
    let body = lines[start_idx..=end_idx].join("\n");
    let body_lower = body.to_lowercase();

    // === REENTRANCY VULNERABILITIES ===
    if (body.contains(".call{")
        || body.contains(".call(")
        || body.contains("delegatecall")
        || body_lower.contains("transfer("))
        && (mutates.iter().any(|m| !m.is_empty())
            || body.contains("state change")
            || body_lower.contains("Balance")
            || body_lower.contains("allowance"))
    {
        // Check if there's state change before call (good) or after (bad)
        let call_pos = body.find(".call").unwrap_or(0);
        let state_change = body[..call_pos].contains("=");
        if !state_change || body[call_pos..].contains("=") {
            mutates.insert("EVM_REENTRANCY".to_string());
        }
    }

    // === UNCHECKED RETURN VALUES ===
    if (body.contains(".call(")
        || body.contains(".transfer(")
        || body.contains(".send(")
        || body_lower.contains("delegatecall"))
        && !body.contains("require(")
        && !body.contains("assert(")
    {
        mutates.insert("EVM_UNCHECKED_CALL".to_string());
    }

    // === INTEGER OVERFLOW/UNDERFLOW ===
    if (body.contains("+") || body.contains("-") || body.contains("*") || body.contains("/"))
        && !body_lower.contains("safemath")
        && !body_lower.contains("checked")
        && !body_lower.contains("openZeppelin")
        && !body_lower.contains("openzeppelin")
    {
        mutates.insert("EVM_UNCHECKED_ARITHMETIC".to_string());
    }

    // === FRONT-RUNNING VULNERABILITY ===
    if (body_lower.contains("gaslimit")
        || body_lower.contains("gasprice")
        || body_lower.contains("nonce"))
        && (body_lower.contains("visible in mempool") || body_lower.contains("pending"))
    {
        mutates.insert("EVM_FRONT_RUNNING".to_string());
    }

    // === FRONT-RUNNING (mempool ordering attacks) ===
    if body_lower.contains("tx.gasprice")
        && (mutates.iter().any(|m| m.contains("allowance"))
            || mutates.iter().any(|m| m.contains("balance")))
    {
        mutates.insert("EVM_FRONT_RUNNING".to_string());
    }

    // === DELEGATECALL VULNERABILITY ===
    if body.contains("delegatecall") {
        // Any delegatecall to untrusted code is dangerous
        if body.contains("delegatecall(") && !body.contains("require(") {
            mutates.insert("EVM_DELEGATECALL_ABUSE".to_string());
        }
    }

    // === TIMESTAMP DEPENDENCY ===
    if body_lower.contains("block.timestamp") || body_lower.contains("now") {
        mutates.insert("EVM_TIMESTAMP_DEPENDENCY".to_string());
    }

    // === ACCESS CONTROL MISSING ===
    if (body_lower.contains("msg.sender") || body_lower.contains("caller"))
        && !body.contains("require(")
        && !body.contains("onlyOwner")
        && !body.contains("modifier")
        && !body.contains("assert(")
    {
        mutates.insert("EVM_ACCESS_CONTROL".to_string());
    }

    // === INSUFFICIENT INPUT VALIDATION ===
    if body.contains("function") && !body.contains("require(") && !body.contains("assert(") {
        mutates.insert("EVM_INPUT_VALIDATION".to_string());
    }
}

/// Extract variable name from declaration (e.g., "uint256 public balance;" → "balance").
fn extract_variable_name(line: &str) -> Option<String> {
    let words: Vec<&str> = line.split_whitespace().collect();

    // Skip type and visibility keywords
    let solidity_keywords = [
        "uint",
        "int",
        "address",
        "bool",
        "bytes",
        "string",
        "mapping",
        "public",
        "private",
        "internal",
        "external",
        "constant",
        "immutable",
        "struct",
        "enum",
        "indexed",
        "memory",
        "storage",
        "calldata",
    ];

    for word in words.iter() {
        // Skip numbers after types (e.g., 256 in uint256)
        if word.chars().all(|c| c.is_ascii_digit()) {
            continue;
        }

        let word_lower = word.to_lowercase();
        let is_keyword = solidity_keywords
            .iter()
            .any(|kw| word_lower.starts_with(kw))
            || word_lower == "mapping"
            || word_lower.starts_with("mapping");

        if !is_keyword {
            // Extract the actual variable name, removing semicolons and other symbols
            let name = word
                .split(|c: char| [';', '=', '(', '[', ','].contains(&c))
                .next()?
                .trim();

            if !name.is_empty() {
                return Some(name.to_string());
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_analyze_vulnerable_contract() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("Token.sol");
        fs::write(
            &path,
            r#"pragma solidity ^0.8.0;
contract Token {
    mapping(address => uint) balances;
    uint supply = 0;
    
    function withdraw() external {
        uint amount = balances[msg.sender];
        (bool success, ) = msg.sender.call{value: amount}("");
        require(success);
        balances[msg.sender] = 0;
    }
    
    function transfer(address to, uint amount) public {
        balances[msg.sender] -= amount;
        balances[to] += amount;
    }
}"#,
        )
        .unwrap();

        let analyzer = EvmAnalyzer;
        let result = analyzer.analyze(&path).unwrap();

        assert_eq!(result.name, "Token");
        assert_eq!(result.chain, "evm");
        assert!(!result.functions.is_empty());
        assert!(result.functions.iter().any(|(_, f)| f.name == "withdraw"));
        assert!(result.functions.iter().any(|(_, f)| f.name == "transfer"));
    }

    #[test]
    fn test_analyze_empty_file() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("empty.sol");
        fs::write(&path, "").unwrap();

        let analyzer = EvmAnalyzer;
        let result = analyzer.analyze(&path).unwrap();

        assert_eq!(result.functions.len(), 0);
    }

    #[test]
    fn test_analyze_nonexistent_path() {
        let analyzer = EvmAnalyzer;
        let result = analyzer.analyze(std::path::Path::new("/nonexistent/path/Contract.sol"));

        assert!(result.is_err());
    }

    #[test]
    fn test_extract_contract_name() {
        let source = r#"pragma solidity ^0.8.0;
contract MyContract {
    function test() public {}
}"#;
        let name = extract_contract_name(source);
        assert_eq!(name, Some("MyContract".to_string()));
    }

    #[test]
    fn test_extract_state_variables() {
        let source = r#"pragma solidity ^0.8.0;
contract Test {
    uint256 public balance;
    address owner;
    mapping(address => uint) balances;
}"#;
        let vars = extract_state_variables(source);
        assert!(vars.contains(&"balance".to_string()));
        assert!(vars.contains(&"owner".to_string()));
    }

    #[test]
    fn test_extract_function_params() {
        let signature = "transfer(address recipient, uint256 amount)";
        let params = extract_function_params(signature);
        assert_eq!(params.len(), 2);
        assert!(params.contains(&"recipient".to_string()));
        assert!(params.contains(&"amount".to_string()));
    }

    #[test]
    fn test_extract_variable_name() {
        assert_eq!(
            extract_variable_name("uint256 public balance;"),
            Some("balance".to_string())
        );
        assert_eq!(
            extract_variable_name("address owner;"),
            Some("owner".to_string())
        );
        assert_eq!(
            extract_variable_name("mapping(address => uint) balances;"),
            Some("balances".to_string())
        );
    }

    #[test]
    fn test_chain_identifier() {
        let analyzer = EvmAnalyzer;
        assert_eq!(analyzer.chain(), "evm");
    }
}
