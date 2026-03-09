//! Invariant library management.

use sentri_core::model::Invariant;
use std::collections::BTreeMap;

/// A collection of invariants organized by category.
pub struct InvariantLibrary {
    /// Invariants by category.
    pub categories: BTreeMap<String, Vec<Invariant>>,
}

impl InvariantLibrary {
    /// Create a new empty library.
    pub fn new() -> Self {
        Self {
            categories: BTreeMap::new(),
        }
    }

    /// Create a library with default built-in invariants for a chain.
    pub fn with_defaults(chain: &str) -> Self {
        let mut lib = Self::new();
        lib.add_defaults(chain);
        lib
    }

    /// Add default invariants for a specific blockchain.
    fn add_defaults(&mut self, chain: &str) {
        match chain.to_lowercase().as_str() {
            "evm" => self.add_evm_defaults(),
            "solana" => self.add_solana_defaults(),
            "move" => self.add_move_defaults(),
            _ => {}
        }
    }

    /// Add EVM-specific invariants.
    fn add_evm_defaults(&mut self) {
        let evm_invariants = vec![
            ("evm_reentrancy_protection", "Reentrancy Protection", "call_order_respected AND no_recursive_calls"),
            ("evm_integer_overflow", "Integer Overflow", "arithmetic_values_bounded AND checked_arithmetic"),
            ("evm_integer_underflow", "Integer Underflow", "subtraction_checked AND minimum_value_respected"),
            ("evm_unchecked_returns", "Unchecked Return Values", "all_external_calls_checked AND return_validation"),
            ("evm_delegatecall_injection", "Delegatecall Injection", "delegatecall_target_whitelist AND no_user_delegatecall"),
            ("evm_access_control", "Access Control", "caller_authentication AND permission_respected"),
            ("evm_timestamp_dependence", "Timestamp Dependence", "no_timestamp_for_security AND block_properties_consistent"),
            ("evm_frontrunning", "Front-running", "state_order_independent AND tx_ordering_irrelevant"),
            ("evm_uninitialized_pointers", "Uninitialized Pointers", "memory_initialized_before_use AND storage_initialized"),
            ("evm_division_by_zero", "Division by Zero", "divisor_nonzero AND modulo_nonzero"),
        ];

        for (id, name, _constraint) in evm_invariants {
            self.add(
                "EVM".to_string(),
                Invariant {
                    name: id.to_string(),
                    description: Some(format!("EVM invariant: {}", name)),
                    expression: sentri_core::model::Expression::Var(id.to_string()),
                    severity: "high".to_string(),
                    category: "evm".to_string(),
                    is_always_true: true,
                    layers: vec!["control_flow".to_string(), "data_flow".to_string()],
                    phases: vec!["execution".to_string(), "finalization".to_string()],
                },
            );
        }
    }

    /// Add Solana-specific invariants.
    fn add_solana_defaults(&mut self) {
        let solana_invariants = vec![
            ("sol_signer_checks", "Signer Checks", "all_required_signers_present AND signatures_valid"),
            ("sol_account_validation", "Account Validation", "expected_accounts_provided AND account_state_valid"),
            ("sol_integer_overflow", "Integer Overflow", "arithmetic_checked AND values_within_bounds"),
            ("sol_rent_exemption", "Rent Exemption", "rent_paid_or_exempt AND account_cleanup_proper"),
            ("sol_pda_derivation", "PDA Derivation", "pda_seeds_deterministic AND derivation_consistent"),
            ("sol_lamport_balance", "Lamport Balance", "lamports_conserved AND no_lamport_leaks"),
            ("sol_instruction_parsing", "Instruction Parsing", "instruction_data_valid AND account_order_correct"),
        ];

        for (id, name, _constraint) in solana_invariants {
            self.add(
                "Solana".to_string(),
                Invariant {
                    name: id.to_string(),
                    description: Some(format!("Solana invariant: {}", name)),
                    expression: sentri_core::model::Expression::Var(id.to_string()),
                    severity: "high".to_string(),
                    category: "solana".to_string(),
                    is_always_true: true,
                    layers: vec!["account_layer".to_string(), "instruction_layer".to_string()],
                    phases: vec!["parsing".to_string(), "execution".to_string()],
                },
            );
        }
    }

    /// Add Move-specific invariants.
    fn add_move_defaults(&mut self) {
        let move_invariants = vec![
            ("move_access_control", "Access Control", "caller_has_required_capability AND resource_protected"),
            ("move_integer_overflow", "Integer Overflow", "addition_checked AND values_bounded"),
            ("move_resource_leaks", "Resource Leaks", "all_resources_returned AND no_abort_without_cleanup"),
            ("move_type_safety", "Type Safety", "types_match_at_boundaries AND resources_typed"),
            ("move_signer_requirement", "Signer Requirement", "signer_passed_and_verified AND signer_not_optional"),
        ];

        for (id, name, _constraint) in move_invariants {
            self.add(
                "Move".to_string(),
                Invariant {
                    name: id.to_string(),
                    description: Some(format!("Move invariant: {}", name)),
                    expression: sentri_core::model::Expression::Var(id.to_string()),
                    severity: "high".to_string(),
                    category: "move".to_string(),
                    is_always_true: true,
                    layers: vec!["module_layer".to_string(), "transaction_layer".to_string()],
                    phases: vec!["execution".to_string()],
                },
            );
        }
    }

    /// Add an invariant to the library.
    pub fn add(&mut self, category: String, invariant: Invariant) {
        self.categories.entry(category).or_default().push(invariant);
    }

    /// Get all invariants in a category.
    pub fn get_category(&self, category: &str) -> Option<&[Invariant]> {
        self.categories.get(category).map(|v| v.as_slice())
    }

    /// Get all invariants.
    pub fn all(&self) -> Vec<&Invariant> {
        self.categories.values().flat_map(|v| v.iter()).collect()
    }

    /// Count total invariants.
    pub fn count(&self) -> usize {
        self.categories.values().map(|v| v.len()).sum()
    }
}

impl Default for InvariantLibrary {
    fn default() -> Self {
        Self::new()
    }
}
