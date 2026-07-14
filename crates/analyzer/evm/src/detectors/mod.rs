//! AST-based vulnerability detectors for EVM smart contracts.
//!
//! Each detector implements analysis for a specific vulnerability pattern.
//!
//! Phase A (Critical): 5 high-impact invariants
//! - evm_missing_post_state_health_check (H19 Euler - $197M)
//! - evm_merkle_root_zero_default (H16 Nomad - $190M)
//! - evm_dvn_single_point_failure (H47 KelpDAO - $292M)
//! - evm_unbacked_synthetic_mint (H56 Echo - $73M)
//! - evm_lst_depeg_collateral_risk (H47 KelpDAO - $292M)
//!
//! Phase B (High-Priority): 8 additional invariants
//! - evm_oracle_self_trade (H17 Mango - $117M)
//! - evm_synthetic_collateral_oracle (H45 Rhea, H40 Makina)
//! - evm_erc4626_inflation_protection (Theoretical)
//! - evm_arbitrary_call_msg_value (H26 Unizen - $2.1M)
//! - evm_reentrancy_via_whitelisted (H29 Penpie - $27M)
//! - evm_proxy_storage_collision (H28 Pike - $1.68M)
//! - evm_bridge_address_cryptographic_verify (H49 Purrlend)
//! - More...

pub mod aa_entropy_weakness;
// DEPRECATED: Old detector using legacy Violation struct, disabled for v0.3.0
// pub mod access_control;
pub mod arbitrary_call_msg_value;
pub mod arithmetic_rounding;
pub mod bridge_address_cryptographic_verify;
pub mod constructor_race_condition;
pub mod dvn_single_point;
pub mod erc4626_inflation_protection;
// DEPRECATED: Old detector using legacy Violation struct, disabled for v0.3.0
// pub mod flash_loan;
pub mod health_check;
pub mod implementations;
pub mod lst_depeg;
pub mod merkle_root;
pub mod oracle_self_trade;
// DEPRECATED: Old detector using legacy Violation struct, disabled for v0.3.0
// pub mod overflow;
pub mod proxy_storage_collision;
// DEPRECATED: Old detector using legacy Violation struct, disabled for v0.3.0
// pub mod reentrancy;
pub mod reentrancy_via_whitelisted;
pub mod router_slippage_validation;
pub mod signature_replay_protection;
pub mod state_mutation_ordering;
pub mod synthetic_collateral_oracle;
pub mod synthetic_mint;
pub mod token_balance_manipulation;
pub mod upgrade_path_verification;

pub use aa_entropy_weakness::detect_aa_entropy_weakness;
// pub use access_control::AccessControlDetector;
pub use arbitrary_call_msg_value::detect_arbitrary_call_msg_value;
pub use arithmetic_rounding::detect_arithmetic_rounding;
pub use bridge_address_cryptographic_verify::detect_bridge_address_cryptographic_verify;
pub use constructor_race_condition::detect_constructor_race_condition;
pub use dvn_single_point::detect_dvn_single_point_failure;
pub use erc4626_inflation_protection::detect_erc4626_inflation_protection;
// pub use flash_loan::FlashLoanDetector;
pub use health_check::detect_missing_health_check;
pub use implementations::*;
pub use lst_depeg::detect_lst_depeg_collateral_risk;
pub use merkle_root::detect_merkle_root_zero_default;
pub use oracle_self_trade::detect_oracle_self_trade;
// pub use overflow::OverflowDetector;
pub use proxy_storage_collision::detect_proxy_storage_collision;
// pub use reentrancy::ReentrancyDetector;
pub use reentrancy_via_whitelisted::detect_reentrancy_via_whitelisted;
pub use router_slippage_validation::detect_router_slippage_validation;
pub use signature_replay_protection::detect_signature_replay_protection;
pub use state_mutation_ordering::detect_state_mutation_ordering;
pub use synthetic_collateral_oracle::detect_synthetic_collateral_oracle;
pub use synthetic_mint::detect_unbacked_synthetic_mint;
pub use token_balance_manipulation::detect_token_balance_manipulation;
pub use upgrade_path_verification::detect_upgrade_path_verification;

/// Run every live EVM detector (the base pattern set in `implementations::detect_all`
/// plus every standalone named-exploit detector) against the given source text.
///
/// This is the single entry point the CLI should use for EVM analysis: each detector
/// operates directly on raw source text, so no solc/AST availability is required.
pub fn run_all_detectors(source: &str, file_path: &str) -> Vec<sentri_core::Finding> {
    let mut findings = implementations::detect_all(source, file_path);

    findings.extend(aa_entropy_weakness::detect_aa_entropy_weakness(
        source, file_path,
    ));
    findings.extend(arbitrary_call_msg_value::detect_arbitrary_call_msg_value(
        source, file_path,
    ));
    findings.extend(arithmetic_rounding::detect_arithmetic_rounding(
        source, file_path,
    ));
    findings.extend(
        bridge_address_cryptographic_verify::detect_bridge_address_cryptographic_verify(
            source, file_path,
        ),
    );
    findings
        .extend(constructor_race_condition::detect_constructor_race_condition(source, file_path));
    findings.extend(dvn_single_point::detect_dvn_single_point_failure(
        source, file_path,
    ));
    findings.extend(
        erc4626_inflation_protection::detect_erc4626_inflation_protection(source, file_path),
    );
    findings.extend(health_check::detect_missing_health_check(source, file_path));
    findings.extend(lst_depeg::detect_lst_depeg_collateral_risk(
        source, file_path,
    ));
    findings.extend(merkle_root::detect_merkle_root_zero_default(
        source, file_path,
    ));
    findings.extend(oracle_self_trade::detect_oracle_self_trade(
        source, file_path,
    ));
    findings.extend(proxy_storage_collision::detect_proxy_storage_collision(
        source, file_path,
    ));
    findings
        .extend(reentrancy_via_whitelisted::detect_reentrancy_via_whitelisted(source, file_path));
    findings
        .extend(router_slippage_validation::detect_router_slippage_validation(source, file_path));
    findings
        .extend(signature_replay_protection::detect_signature_replay_protection(source, file_path));
    findings.extend(state_mutation_ordering::detect_state_mutation_ordering(
        source, file_path,
    ));
    findings
        .extend(synthetic_collateral_oracle::detect_synthetic_collateral_oracle(source, file_path));
    findings.extend(synthetic_mint::detect_unbacked_synthetic_mint(
        source, file_path,
    ));
    findings
        .extend(token_balance_manipulation::detect_token_balance_manipulation(source, file_path));
    findings.extend(upgrade_path_verification::detect_upgrade_path_verification(
        source, file_path,
    ));

    // Chain-agnostic shared-IR rule (Epic 6.1): flags privileged mutations
    // with no authorization guard, using the same rule Solana and Move share.
    // This one needs a real solc AST (unlike every detector above, which work
    // on raw source text), so it's best-effort: if solc isn't installed or
    // parsing fails, this rule simply contributes nothing rather than
    // breaking the rest of the scan.
    if let Ok(contract) = crate::ast::SolidityParser::parse_source(source, file_path) {
        let model = crate::semantic_model::build_semantic_model(&contract, source, file_path);
        findings.extend(sentri_ir::rules::find_unauthorized_privileged_mutations(
            &model,
        ));
    }

    // A handful of detectors can overlap in coverage (e.g. merkle_root vs.
    // merkle_root_zero_default) - dedupe on (invariant_id, file, line) as a safety net.
    let mut seen = std::collections::HashSet::new();
    findings.retain(|f| seen.insert(f.dedup_key()));

    findings.sort_by(|a, b| match b.severity.cmp(&a.severity) {
        std::cmp::Ordering::Equal => a.line.cmp(&b.line),
        other => other,
    });

    findings
}
