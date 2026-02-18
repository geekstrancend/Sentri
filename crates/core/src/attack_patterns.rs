//! Known attack patterns and corresponding defensive invariants.
//!
//! This module documents historical exploits and provides protective
//! invariants to prevent similar attacks.

use std::collections::BTreeMap;

/// A known attack pattern with defensive invariants.
#[derive(Debug, Clone)]
pub struct AttackPattern {
    /// Unique identifier for the attack.
    pub id: String,
    /// Attack name (e.g., "Reentrancy", "Integer Overflow").
    pub name: String,
    /// Description of how the attack works.
    pub description: String,
    /// Year the attack was discovered/first exploited.
    pub year: u32,
    /// Notable incidents where this attack occurred.
    pub incidents: Vec<String>,
    /// Code patterns that indicate vulnerability.
    pub vulnerable_patterns: Vec<String>,
    /// Defensive invariants to prevent the attack.
    pub defensive_invariants: Vec<String>,
    /// Affected chains: "solana", "evm", "move".
    pub affected_chains: Vec<String>,
    /// CVSS severity score (1-10).
    pub cvss_score: f32,
}

/// Attack pattern database.
pub struct AttackPatternDB {
    patterns: BTreeMap<String, AttackPattern>,
}

impl AttackPatternDB {
    /// Create a new attack pattern database with known patterns.
    pub fn new() -> Self {
        let mut patterns = BTreeMap::new();

        // Attack 1: Reentrancy
        patterns.insert(
            "reentrancy".to_string(),
            AttackPattern {
                id: "reentrancy".to_string(),
                name: "Reentrancy".to_string(),
                description:
                    "Attacker calls back into contract during execution, modifying state before \
                    previous execution completes"
                        .to_string(),
                year: 2016,
                incidents: vec!["The DAO (2016) - $50M loss".to_string()],
                vulnerable_patterns: vec![
                    "transfer_funds(); /* state update after */".to_string(),
                    "transfer(amount)".to_string(),
                    "delegatecall".to_string(),
                    "state update AFTER external call".to_string(),
                    "payable(msg.sender).transfer".to_string(),
                    "call.value()() without checking re-entry".to_string(),
                    "state_change_after_external_call".to_string(),
                ],
                defensive_invariants: vec![
                    "state_update_before_external_call".to_string(),
                    "mutex_lock_during_transfer".to_string(),
                    "checks_effects_interactions_order".to_string(),
                    "balance_matches_sum_before_and_after".to_string(),
                ],
                affected_chains: vec!["evm".to_string()],
                cvss_score: 9.8,
            },
        );

        // Attack 2: Integer Overflow/Underflow
        patterns.insert(
            "integer_overflow".to_string(),
            AttackPattern {
                id: "integer_overflow".to_string(),
                name: "Integer Overflow/Underflow".to_string(),
                description:
                    "Arithmetic operations exceed max/min bounds, wrapping to opposite extreme"
                        .to_string(),
                year: 2018,
                incidents: vec![
                    "BEC Token (2018) - $7.6M frozen".to_string(),
                    "BeautyChain (2018) - batch transfer bug".to_string(),
                ],
                vulnerable_patterns: vec![
                    "unchecked_addition".to_string(),
                    "unchecked_subtraction".to_string(),
                    "balance + amount without overflow check".to_string(),
                ],
                defensive_invariants: vec![
                    "addition_with_overflow_check".to_string(),
                    "subtraction_with_underflow_check".to_string(),
                    "total_supply_constant".to_string(),
                    "balance_never_negative".to_string(),
                ],
                affected_chains: vec!["evm".to_string(), "move".to_string()],
                cvss_score: 8.5,
            },
        );

        // Attack 3: Access Control Bypass
        patterns.insert(
            "access_control_bypass".to_string(),
            AttackPattern {
                id: "access_control_bypass".to_string(),
                name: "Access Control Bypass".to_string(),
                description:
                    "Attacker circumvents permission checks to perform privileged operations"
                        .to_string(),
                year: 2017,
                incidents: vec!["Parity Wallet (2017) - $30M frozen".to_string()],
                vulnerable_patterns: vec![
                    "missing_require(is_owner())".to_string(),
                    "tx.origin != msg.sender".to_string(),
                    "no_signature_validation".to_string(),
                    "public_function_without_auth".to_string(),
                ],
                defensive_invariants: vec![
                    "only_owner_can_transfer".to_string(),
                    "multisig_required_for_critical_ops".to_string(),
                    "all_privileged_ops_checked".to_string(),
                    "authorization_before_state_change".to_string(),
                ],
                affected_chains: vec!["evm".to_string(), "solana".to_string(), "move".to_string()],
                cvss_score: 9.9,
            },
        );

        // Attack 4: Flash Loan Attack
        patterns.insert(
            "flash_loan".to_string(),
            AttackPattern {
                id: "flash_loan".to_string(),
                name: "Flash Loan Attack".to_string(),
                description:
                    "Attacker borrows large amount in single transaction to manipulate price"
                        .to_string(),
                year: 2020,
                incidents: vec![
                    "bZx (2020) - $350K + $600K losses".to_string(),
                    "Harvest Finance (2020) - $34M loss".to_string(),
                ],
                vulnerable_patterns: vec![
                    "price_oracle_single_source".to_string(),
                    "no_price_validation".to_string(),
                    "lending_without_collateral_check".to_string(),
                ],
                defensive_invariants: vec![
                    "price_from_multiple_sources".to_string(),
                    "collateral_check_before_lending".to_string(),
                    "price_deviation_limits".to_string(),
                    "no_same_block_operations".to_string(),
                ],
                affected_chains: vec!["evm".to_string()],
                cvss_score: 8.7,
            },
        );

        // Attack 5: Frontrunning/MEV
        patterns.insert(
            "frontrunning".to_string(),
            AttackPattern {
                id: "frontrunning".to_string(),
                name: "Frontrunning / MEV Extraction".to_string(),
                description:
                    "Attacker observes pending transaction and places own transaction first"
                        .to_string(),
                year: 2018,
                incidents: vec!["General vulnerability since Ethereum inception".to_string()],
                vulnerable_patterns: vec![
                    "price_depends_on_order".to_string(),
                    "state_visible_in_mempool".to_string(),
                    "no_slippage_protection".to_string(),
                ],
                defensive_invariants: vec![
                    "slippage_limits_enforced".to_string(),
                    "atomic_swap_no_intermediate_states".to_string(),
                    "timestamp_deadline_checks".to_string(),
                    "sorted_by_priority_not_order".to_string(),
                ],
                affected_chains: vec!["evm".to_string()],
                cvss_score: 7.5,
            },
        );

        // Attack 6: Type Confusion
        patterns.insert(
            "type_confusion".to_string(),
            AttackPattern {
                id: "type_confusion".to_string(),
                name: "Type Confusion / Implicit Conversion".to_string(),
                description: "Implicit type conversions cause incorrect comparisons or operations"
                    .to_string(),
                year: 2019,
                incidents: vec!["Multiplier Finance (2021) - $1M loss".to_string()],
                vulnerable_patterns: vec![
                    "implicit_type_conversion".to_string(),
                    "comparison_different_types".to_string(),
                    "address_to_uint_conversion".to_string(),
                ],
                defensive_invariants: vec![
                    "no_implicit_conversions".to_string(),
                    "explicit_type_matching_required".to_string(),
                    "type_checked_before_comparison".to_string(),
                ],
                affected_chains: vec!["evm".to_string()],
                cvss_score: 7.2,
            },
        );

        // Attack 7: Delegatecall Misuse
        patterns.insert(
            "delegatecall_misuse".to_string(),
            AttackPattern {
                id: "delegatecall_misuse".to_string(),
                name: "Delegatecall to Untrusted Code".to_string(),
                description: "Contract delegatecalls to address that can be controlled by attacker"
                    .to_string(),
                year: 2016,
                incidents: vec!["King of the Ether (2016) - theft of contract funds".to_string()],
                vulnerable_patterns: vec![
                    "delegatecall(attacker_address)".to_string(),
                    "delegatecall_to_user_input".to_string(),
                    "no_validation_before_delegatecall".to_string(),
                ],
                defensive_invariants: vec![
                    "delegatecall_target_hardcoded".to_string(),
                    "delegatecall_target_audited".to_string(),
                    "no_delegatecall_to_untrusted".to_string(),
                    "delegatecall_results_validated".to_string(),
                ],
                affected_chains: vec!["evm".to_string()],
                cvss_score: 9.8,
            },
        );

        // Attack 8: Timestamp Dependence
        patterns.insert(
            "timestamp_dependence".to_string(),
            AttackPattern {
                id: "timestamp_dependence".to_string(),
                name: "Timestamp Dependence".to_string(),
                description: "Miner/validator can manipulate block timestamp for advantage"
                    .to_string(),
                year: 2015,
                incidents: vec!["Various lottery and randomness exploits".to_string()],
                vulnerable_patterns: vec![
                    "random_number = block.timestamp".to_string(),
                    "critical_logic_depends_on_block.timestamp".to_string(),
                    "no_time_bounds_checking".to_string(),
                ],
                defensive_invariants: vec![
                    "no_randomness_from_timestamp".to_string(),
                    "randomness_from_external_oracle".to_string(),
                    "time_bounds_enforced".to_string(),
                    "timestamp_within_reasonable_bounds".to_string(),
                ],
                affected_chains: vec!["evm".to_string()],
                cvss_score: 6.5,
            },
        );

        Self { patterns }
    }

    /// Get all attack patterns.
    pub fn all_patterns(&self) -> Vec<&AttackPattern> {
        self.patterns.values().collect()
    }

    /// Get patterns affecting a specific chain.
    pub fn patterns_for_chain(&self, chain: &str) -> Vec<&AttackPattern> {
        self.patterns
            .values()
            .filter(|p| p.affected_chains.contains(&chain.to_string()))
            .collect()
    }

    /// Get pattern by ID.
    pub fn get_pattern(&self, id: &str) -> Option<&AttackPattern> {
        self.patterns.get(id)
    }

    /// Check if code might be vulnerable to a pattern.
    pub fn check_code(&self, code: &str, attack_id: &str) -> Vec<String> {
        let mut issues = Vec::new();

        if let Some(pattern) = self.get_pattern(attack_id) {
            for vulnerable_pattern in &pattern.vulnerable_patterns {
                if code.contains(vulnerable_pattern) {
                    issues.push(format!(
                        "Found vulnerable pattern '{}' from {} attack",
                        vulnerable_pattern, pattern.name
                    ));
                }
            }
        }

        issues
    }
}

impl Default for AttackPatternDB {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attack_db_creation() {
        let db = AttackPatternDB::new();
        assert_eq!(db.all_patterns().len(), 8);
    }

    #[test]
    fn test_get_pattern() {
        let db = AttackPatternDB::new();
        let pattern = db.get_pattern("reentrancy").unwrap();
        assert_eq!(pattern.name, "Reentrancy");
        assert_eq!(pattern.year, 2016);
    }

    #[test]
    fn test_patterns_for_chain() {
        let db = AttackPatternDB::new();
        let evm_patterns = db.patterns_for_chain("evm");
        assert!(!evm_patterns.is_empty());

        let solana_patterns = db.patterns_for_chain("solana");
        assert!(solana_patterns
            .iter()
            .any(|p| p.id == "access_control_bypass"));
    }

    #[test]
    fn test_code_vulnerability_check() {
        let db = AttackPatternDB::new();
        let vulnerable_code = "transfer_funds(); /* state update after */";
        let issues = db.check_code(vulnerable_code, "reentrancy");
        assert!(!issues.is_empty());
    }

    #[test]
    fn test_cvss_scores() {
        let db = AttackPatternDB::new();
        for pattern in db.all_patterns() {
            assert!(pattern.cvss_score > 0.0 && pattern.cvss_score <= 10.0);
        }
    }
}
