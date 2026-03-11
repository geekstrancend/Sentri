//! Simulation engine for analyzing program invariants.

use sentri_core::model::{Invariant, ProgramModel, SimulationReport};
use sentri_core::traits::Simulator;
use sentri_core::Result;
use tracing::info;

/// Deterministic simulation engine for invariant testing.
///
/// This engine performs static analysis of program models against invariants.
/// It analyzes function control flow, state access patterns, and structural properties
/// to detect potential violations.
pub struct SimulationEngine;

impl SimulationEngine {
    /// Create a new simulation engine.
    pub fn new(_seed: u64) -> Self {
        Self
    }
}

impl Default for SimulationEngine {
    fn default() -> Self {
        Self
    }
}

impl Simulator for SimulationEngine {
    fn simulate(
        &self,
        program: &ProgramModel,
        invariants: &[Invariant],
    ) -> Result<SimulationReport> {
        info!(
            "Starting analysis of {} with {} invariants",
            program.name,
            invariants.len()
        );

        let mut traces = Vec::new();
        let mut detected_violations = 0;

        // Analyze each invariant against the program
        for invariant in invariants {
            // Check if invariant is applicable to this program type
            if !is_invariant_applicable(invariant, program) {
                continue;
            }

            // Analyze program-level patterns
            if let Some(violation) = analyze_program_invariant(invariant, program) {
                detected_violations += 1;
                traces.push(violation);
            }

            // Analyze each function against the invariant
            for (func_name, function) in &program.functions {
                if let Some(violation) =
                    analyze_function_invariant(invariant, program, function, func_name)
                {
                    detected_violations += 1;
                    traces.push(violation);
                }
            }
        }

        // Calculate coverage based on violations found
        let total_checks = invariants.len();
        let coverage = if total_checks > 0 {
            ((total_checks - detected_violations.min(total_checks)) as f64 / total_checks as f64)
                * 100.0
        } else {
            100.0
        };

        info!(
            "Analysis complete: {} violations detected in {} invariants, {:.1}% coverage",
            detected_violations, total_checks, coverage
        );

        Ok(SimulationReport {
            violations: detected_violations,
            traces,
            coverage: coverage.max(0.0),
            seed: 0,
        })
    }

    fn chain(&self) -> &str {
        "analysis"
    }
}

/// Check if an invariant is applicable to this program type.
///
/// An invariant applies if:
/// - It's a cross-platform invariant (access control, overflow), OR
/// - Its category matches the program's blockchain platform
fn is_invariant_applicable(invariant: &Invariant, program: &ProgramModel) -> bool {
    let program_chain = program.chain.to_lowercase();
    let invariant_category = invariant.category.to_lowercase();

    // Cross-platform invariants apply to all programs
    if invariant_category.contains("access")
        || invariant_category.contains("overflow")
        || invariant_category.contains("general")
    {
        return true;
    }

    // Platform-specific invariants must match the program's chain
    program_chain.contains(&invariant_category)
}

/// Analyze program-level invariants to detect violations.
///
/// Performs static analysis of:
/// - Reentrancy risks (multiple entry points with state mutations)
/// - Access control patterns (entry point functions without state checks)
/// - Arithmetic overflow risks (functions with numeric parameters)
fn analyze_program_invariant(invariant: &Invariant, program: &ProgramModel) -> Option<String> {
    let invariant_name_lower = invariant.name.to_lowercase();

    // Reentrancy risk: multiple entry points with state mutations
    if invariant_name_lower.contains("reentrancy") {
        let entry_points: Vec<_> = program
            .functions
            .iter()
            .filter(|(_, f)| f.is_entry_point)
            .collect();

        let mutating_functions: Vec<_> = program
            .functions
            .iter()
            .filter(|(_, f)| !f.mutates.is_empty())
            .collect();

        if entry_points.len() > 1 && !mutating_functions.is_empty() {
            return Some(format!(
                "Invariant '{}' violated in {}: Reentrancy risk detected - {} entry points with state mutations across {} functions",
                invariant.name,
                program.name,
                entry_points.len(),
                mutating_functions.len()
            ));
        }
    }

    // Access control risk: entry points with public state access
    if invariant_name_lower.contains("access") {
        let public_entry_points: Vec<_> = program
            .functions
            .iter()
            .filter(|(_, f)| f.is_entry_point && !f.reads.is_empty())
            .collect();

        if !public_entry_points.is_empty() {
            return Some(format!(
                "Invariant '{}' violated in {}: Access control risk - {} entry points access state without guards",
                invariant.name,
                program.name,
                public_entry_points.len()
            ));
        }
    }

    // Overflow/underflow risk: functions with numeric operations
    if invariant_name_lower.contains("overflow") || invariant_name_lower.contains("underflow") {
        let numeric_functions: Vec<_> = program
            .functions
            .iter()
            .filter(|(_, f)| {
                f.parameters.iter().any(|p| {
                    let p_lower = p.to_lowercase();
                    p_lower.contains("u") || p_lower.contains("i") || p_lower.contains("int")
                })
            })
            .collect();

        if numeric_functions.len() > 2 {
            return Some(format!(
                "Invariant '{}' violated in {}: Arithmetic risk - {} functions with numeric parameters may have unchecked operations",
                invariant.name,
                program.name,
                numeric_functions.len()
            ));
        }
    }

    None
}

/// Analyze function-level invariants to detect violations.
///
/// Performs static analysis of:
/// - Complex state interactions (reads + mutates)
/// - Entry point authorization patterns
/// - Pure function expectations
/// - Solana-specific vulnerabilities (lamport manipulation, unsafe arithmetic)
fn analyze_function_invariant(
    invariant: &Invariant,
    program: &ProgramModel,
    function: &sentri_core::model::FunctionModel,
    func_name: &str,
) -> Option<String> {
    let invariant_name_lower = invariant.name.to_lowercase();
    let state_interaction_count = function.reads.len() + function.mutates.len();

    // Solana-specific: Unsafe lamport manipulation
    if program.chain.to_lowercase().contains("solana")
        && function
            .mutates
            .iter()
            .any(|m| m.contains("SOLANA_LAMPORT_UNSAFE"))
    {
        return Some(format!(
            "Function '{}' in {} violates '{}': Unsafe direct lamport manipulation detected",
            func_name, program.name, "Solana: Safe Lamport Operations"
        ));
    }

    // EVM-specific: Reentrancy vulnerability
    if program.chain.to_lowercase().contains("evm")
        && function
            .mutates
            .iter()
            .any(|m| m.contains("EVM_REENTRANCY"))
    {
        return Some(format!(
            "Function '{}' in {} violates '{}': Reentrancy vulnerability - state mutation after external call",
            func_name, program.name, "EVM: Reentrancy Guard"
        ));
    }

    // EVM-specific: Unchecked external calls
    if program.chain.to_lowercase().contains("evm")
        && function
            .mutates
            .iter()
            .any(|m| m.contains("EVM_UNCHECKED_CALL"))
    {
        return Some(format!(
            "Function '{}' in {} violates '{}': Unchecked external call return value",
            func_name, program.name, "EVM: Checked Calls"
        ));
    }

    // EVM-specific: Integer overflow/underflow
    if program.chain.to_lowercase().contains("evm")
        && function
            .mutates
            .iter()
            .any(|m| m.contains("EVM_UNCHECKED_ARITHMETIC"))
    {
        return Some(format!(
            "Function '{}' in {} violates '{}': Integer overflow/underflow risk",
            func_name, program.name, "EVM: Safe Arithmetic"
        ));
    }

    // EVM-specific: Delegatecall abuse
    if program.chain.to_lowercase().contains("evm")
        && function
            .mutates
            .iter()
            .any(|m| m.contains("EVM_DELEGATECALL_ABUSE"))
    {
        return Some(format!(
            "Function '{}' in {} violates '{}': Unsafe delegatecall to untrusted contract",
            func_name, program.name, "EVM: Delegatecall Safety"
        ));
    }

    // EVM-specific: Timestamp dependency
    if program.chain.to_lowercase().contains("evm")
        && function
            .mutates
            .iter()
            .any(|m| m.contains("EVM_TIMESTAMP_DEPENDENCY"))
    {
        return Some(format!(
            "Function '{}' in {} violates '{}': Logic depends on block.timestamp (mutable)",
            func_name, program.name, "EVM: Timestamp Independence"
        ));
    }

    // EVM-specific: Front-running vulnerability
    if program.chain.to_lowercase().contains("evm")
        && function
            .mutates
            .iter()
            .any(|m| m.contains("EVM_FRONT_RUNNING"))
    {
        return Some(format!(
            "Function '{}' in {} violates '{}': Vulnerable to front-running attacks",
            func_name, program.name, "EVM: Front-run Protection"
        ));
    }

    // EVM-specific: Missing access control
    if program.chain.to_lowercase().contains("evm")
        && function
            .mutates
            .iter()
            .any(|m| m.contains("EVM_ACCESS_CONTROL"))
    {
        return Some(format!(
            "Function '{}' in {} violates '{}': Missing access control checks",
            func_name, program.name, "EVM: Access Control"
        ));
    }

    // EVM-specific: Input validation
    if program.chain.to_lowercase().contains("evm")
        && function
            .mutates
            .iter()
            .any(|m| m.contains("EVM_INPUT_VALIDATION"))
    {
        return Some(format!(
            "Function '{}' in {} violates '{}': Missing input validation",
            func_name, program.name, "EVM: Input Validation"
        ));
    }

    // Move-specific: Resource leak
    if program.chain.to_lowercase().contains("move")
        && function
            .mutates
            .iter()
            .any(|m| m.contains("MOVE_RESOURCE_LEAK"))
    {
        return Some(format!(
            "Function '{}' in {} violates '{}': Potential resource leak - move_from result not bound",
            func_name, program.name, "Move: Resource Safety"
        ));
    }

    // Move-specific: Missing ability checks
    if program.chain.to_lowercase().contains("move")
        && function
            .mutates
            .iter()
            .any(|m| m.contains("MOVE_MISSING_ABILITY"))
    {
        return Some(format!(
            "Function '{}' in {} violates '{}': Resource moves without required abilities",
            func_name, program.name, "Move: Ability Requirements"
        ));
    }

    // Move-specific: Unchecked arithmetic
    if program.chain.to_lowercase().contains("move")
        && function
            .mutates
            .iter()
            .any(|m| m.contains("MOVE_UNCHECKED_ARITHMETIC"))
    {
        return Some(format!(
            "Function '{}' in {} violates '{}': Unchecked arithmetic operation",
            func_name, program.name, "Move: Safe Arithmetic"
        ));
    }

    // Move-specific: Missing signer verification
    if program.chain.to_lowercase().contains("move")
        && function
            .mutates
            .iter()
            .any(|m| m.contains("MOVE_MISSING_SIGNER"))
    {
        return Some(format!(
            "Function '{}' in {} violates '{}': Resource moved without signer verification",
            func_name, program.name, "Move: Signer Verification"
        ));
    }

    // Move-specific: Unguarded state mutation
    if program.chain.to_lowercase().contains("move")
        && function
            .mutates
            .iter()
            .any(|m| m.contains("MOVE_UNGUARDED_MUTATION"))
    {
        return Some(format!(
            "Function '{}' in {} violates '{}': Unguarded global state mutation",
            func_name, program.name, "Move: Guarded Mutations"
        ));
    }

    // Move-specific: Privilege escalation
    if program.chain.to_lowercase().contains("move")
        && function
            .mutates
            .iter()
            .any(|m| m.contains("MOVE_PRIVILEGE_ESCALATION"))
    {
        return Some(format!(
            "Function '{}' in {} violates '{}': Potential privilege escalation through signer extraction",
            func_name, program.name, "Move: Privilege Control"
        ));
    }

    // Move-specific: Unsafe abort
    if program.chain.to_lowercase().contains("move")
        && function
            .mutates
            .iter()
            .any(|m| m.contains("MOVE_UNSAFE_ABORT"))
    {
        return Some(format!(
            "Function '{}' in {} violates '{}': Abort without error code explanation",
            func_name, program.name, "Move: Error Handling"
        ));
    }

    // Solana-specific: Missing signer check
    if program.chain.to_lowercase().contains("solana")
        && function
            .mutates
            .iter()
            .any(|m| m.contains("SOLANA_MISSING_SIGNER"))
    {
        return Some(format!(
            "Function '{}' in {} violates '{}': Missing Signer requirement - SOL_001 (Missing Signer Check)",
            func_name,
            program.name,
            "Solana: Required Signers"
        ));
    }

    // Solana-specific: Unchecked arithmetic with overflow risk
    if program.chain.to_lowercase().contains("solana")
        && function
            .mutates
            .iter()
            .any(|m| m.contains("SOLANA_UNCHECKED_ARITHMETIC"))
    {
        return Some(format!(
            "Function '{}' in {} violates '{}': Unchecked arithmetic operation detected - SOL_003 (Integer Overflow)",
            func_name,
            program.name,
            "Solana: Safe Arithmetic Operations"
        ));
    }

    // Solana-specific: Missing account validation
    if program.chain.to_lowercase().contains("solana")
        && function
            .mutates
            .iter()
            .any(|m| m.contains("SOLANA__MISSING_VALIDATION"))
    {
        return Some(format!(
            "Function '{}' in {} violates '{}': Missing account validation - SOL_002 (Lack of Account Ownership Validation)",
            func_name,
            program.name,
            "Solana: Account Validation"
        ));
    }

    // Solana-specific: Rent exemption violations
    if program.chain.to_lowercase().contains("solana")
        && function
            .mutates
            .iter()
            .any(|m| m.contains("SOLANA_RENT_EXEMPTION"))
    {
        return Some(format!(
            "Function '{}' in {} violates '{}': Potential rent exemption violation - SOL_004 (Rent Exemption)",
            func_name,
            program.name,
            "Solana: Rent Exemption"
        ));
    }

    // Solana-specific: PDA derivation issues
    if program.chain.to_lowercase().contains("solana")
        && function
            .mutates
            .iter()
            .any(|m| m.contains("SOLANA_PDA_DERIVATION"))
    {
        return Some(format!(
            "Function '{}' in {} violates '{}': Potential PDA derivation issue - SOL_005 (PDA Derivation)",
            func_name,
            program.name,
            "Solana: PDA Derivation"
        ));
    }

    // Solana-specific: Unsafe deserialization
    if program.chain.to_lowercase().contains("solana")
        && function
            .mutates
            .iter()
            .any(|m| m.contains("SOLANA_UNSAFE_DESERIALIZATION"))
    {
        return Some(format!(
            "Function '{}' in {} violates '{}': Unsafe deserialization of account data - SOL_007 (Account Deserialization)",
            func_name,
            program.name,
            "Solana: Safe Deserialization"
        ));
    }

    // Solana-specific: Unchecked token transfers
    if program.chain.to_lowercase().contains("solana")
        && function
            .mutates
            .iter()
            .any(|m| m.contains("SOLANA_UNCHECKED_TOKEN_TRANSFER"))
    {
        return Some(format!(
            "Function '{}' in {} violates '{}': Unchecked token transfer operation - SOL_008 (Token Transfer Overflow)",
            func_name,
            program.name,
            "Solana: Safe Token Operations"
        ));
    }

    // Reentrancy: entry points with complex state interactions
    if invariant_name_lower.contains("reentrancy")
        && function.is_entry_point
        && state_interaction_count > 2
    {
        return Some(format!(
            "Function '{}' in {} violates '{}': Complex state interactions (reads: {}, writes: {}) without reentrancy guards",
            func_name,
            program.name,
            invariant.name,
            function.reads.len(),
            function.mutates.len()
        ));
    }

    // Access control: public entry points without authorization
    if invariant_name_lower.contains("access")
        && function.is_entry_point
        && !function.reads.is_empty()
        && !function.is_pure
    {
        return Some(format!(
            "Function '{}' in {} violates '{}': Entry point accesses {} state variables without authorization checks",
            func_name,
            program.name,
            invariant.name,
            function.reads.len()
        ));
    }

    // Arithmetic: functions with multiple numeric parameters
    if (invariant_name_lower.contains("overflow") || invariant_name_lower.contains("underflow"))
        && function.parameters.len() > 1
    {
        let numeric_params = function
            .parameters
            .iter()
            .filter(|p| {
                let p_lower = p.to_lowercase();
                p_lower.contains("u") || p_lower.contains("i") || p_lower.contains("int")
            })
            .count();

        if numeric_params > 1 {
            return Some(format!(
                "Function '{}' in {} violates '{}': {} numeric parameters without overflow checks",
                func_name, program.name, invariant.name, numeric_params
            ));
        }
    }

    None
}
