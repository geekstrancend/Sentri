#![deny(unsafe_code)]
#![allow(missing_docs)]

//! revm-backed dynamic invariant fuzzing for EVM/Solidity contracts.
//!
//! Ties three things together: `sentri_utils::SolcManager` (compiles
//! source, already existed), `solc_bridge` (turns its ABI/bytecode output
//! into the chain-agnostic shapes `sentri-dynamic-core` understands), and
//! `backend::RevmBackend` (executes calls against real EVM bytecode via
//! `revm`, gated behind the `revm-backend` feature — see that module's doc
//! comment for why it's split out and what's unverified about it).

#[cfg(feature = "revm-backend")]
pub mod backend;
pub mod solc_bridge;
pub mod types;

pub use types::CompiledContract;

#[cfg(feature = "revm-backend")]
use sentri_dynamic_core::{fuzz, FuzzConfig, Violation};
use sentri_dynamic_core::{FunctionSpec, Invariant};

/// Auto-detects standard invariants from a contract's function surface:
/// currently, ERC20-shaped conservation (`totalSupply()` + `balanceOf(address)`
/// both present) and monotonicity for any no-argument view function whose
/// name suggests an accumulator (`totalSupply`, `totalAssets`,
/// `exchangeRate`, `sharePrice`, cumulative/checkpoint-style getters).
/// Returns an empty list if nothing recognizable is present — dynamic
/// fuzzing without an oracle can't find anything, so callers should treat
/// an empty result as "nothing to check yet", not "contract is safe".
///
/// Pure data in, data out — doesn't touch `revm`, so unlike the rest of
/// this crate it's built and tested unconditionally (see the tests module
/// below), not gated behind the `revm-backend` feature.
pub fn auto_detect_invariants(
    functions: &[FunctionSpec],
    actors: &[[u8; 20]],
) -> Vec<Box<dyn Invariant>> {
    use sentri_dynamic_core::{ConservationInvariant, MonotonicInvariant};

    let mut invariants: Vec<Box<dyn Invariant>> = Vec::new();

    let total_supply = functions
        .iter()
        .find(|f| f.name == "totalSupply" && f.inputs.is_empty());
    let balance_of = functions
        .iter()
        .find(|f| f.name == "balanceOf" && f.inputs.len() == 1);
    if let (Some(ts), Some(bo)) = (total_supply, balance_of) {
        invariants.push(Box::new(ConservationInvariant::new(
            "ERC20 conservation: sum(balanceOf) == totalSupply()",
            ts.clone(),
            bo.clone(),
            actors.to_vec(),
        )));
    }

    const MONOTONIC_NAME_HINTS: &[&str] = &[
        "totalSupply",
        "totalAssets",
        "exchangeRate",
        "sharePrice",
        "totalDeposits",
        "checkpoint",
        "cumulative",
    ];
    for f in functions {
        if f.mutates_state || !f.inputs.is_empty() {
            continue;
        }
        let lower = f.name.to_lowercase();
        if MONOTONIC_NAME_HINTS
            .iter()
            .any(|hint| lower.contains(hint.to_lowercase().as_str()))
        {
            invariants.push(Box::new(MonotonicInvariant::new(
                format!("{} monotonicity", f.name),
                f.clone(),
            )));
        }
    }

    invariants
}

/// Compiles `source` with `solc`, auto-detects invariants from its ABI, and
/// runs the dynamic fuzzer against it. Returns `Ok(None)` if no violation
/// was found within the configured search budget, `Ok(Some(violation))`
/// with a minimal reproduction if one was, and `Err` for compilation
/// failures.
#[cfg(feature = "revm-backend")]
pub fn fuzz_solidity_source(source: &str, config: FuzzConfig) -> anyhow::Result<Option<Violation>> {
    let solc = sentri_utils::SolcManager::new()?;
    let output = solc.get_ast_for_source(source, "fuzz_target.sol")?;

    let Some((_, contract_json)) = output.contracts.iter().find(|(_, v)| {
        v.get("bin")
            .and_then(|b| b.as_str())
            .map(|b| !b.trim_start_matches("0x").is_empty())
            .unwrap_or(false)
    }) else {
        anyhow::bail!("no deployable contract found in source (no non-empty bytecode — is this an interface/abstract contract?)");
    };

    let contract = solc_bridge::compiled_contract_from_solc_entry(contract_json)?;
    if contract.functions.is_empty() {
        anyhow::bail!("compiled successfully but no callable functions with supported argument types were found");
    }

    let invariants = auto_detect_invariants(&contract.functions, &config.actors);
    if invariants.is_empty() {
        anyhow::bail!(
            "no auto-detectable invariant on this ABI (looked for ERC20-shaped totalSupply/balanceOf, or monotonic accumulator getters) — nothing to check"
        );
    }

    // `backend_factory` borrows `actors` for as long as the returned
    // closure lives; `config` is about to be moved into `fuzz` below, so
    // the factory needs its own copy rather than borrowing config.actors
    // directly (a borrow of config.actors can't outlive config itself
    // being consumed in the very same call).
    let actors = config.actors.clone();
    let factory = backend::backend_factory(&contract, &actors);
    Ok(fuzz(factory, &contract.functions, invariants, config))
}

#[cfg(test)]
mod tests {
    use super::*;
    use sentri_dynamic_core::ParamKind;

    fn view_fn(name: &str, inputs: Vec<ParamKind>) -> FunctionSpec {
        FunctionSpec::new(name, [0u8; 4], inputs, false)
    }

    #[test]
    fn detects_erc20_conservation_when_both_functions_present() {
        let functions = vec![
            view_fn("totalSupply", vec![]),
            view_fn("balanceOf", vec![ParamKind::Address]),
        ];
        let invariants = auto_detect_invariants(&functions, &[[1u8; 20]]);
        assert!(invariants.iter().any(|i| i.name().contains("conservation")));
    }

    #[test]
    fn skips_conservation_when_only_one_of_the_pair_is_present() {
        let functions = vec![view_fn("totalSupply", vec![])];
        let invariants = auto_detect_invariants(&functions, &[[1u8; 20]]);
        assert!(!invariants.iter().any(|i| i.name().contains("conservation")));
    }

    #[test]
    fn detects_monotonic_hints_case_insensitively() {
        let functions = vec![view_fn("TotalAssets", vec![])];
        let invariants = auto_detect_invariants(&functions, &[]);
        assert!(invariants.iter().any(|i| i.name().contains("monotonicity")));
    }

    #[test]
    fn ignores_mutating_functions_and_functions_with_arguments_for_monotonic_hints() {
        let functions = vec![
            FunctionSpec::new("totalAssets", [0u8; 4], vec![], true), // mutates -> not a getter
            view_fn("checkpointFor", vec![ParamKind::Address]), // takes args -> not a plain accumulator getter
        ];
        let invariants = auto_detect_invariants(&functions, &[]);
        assert!(invariants.is_empty());
    }

    #[test]
    fn returns_empty_on_unrecognized_abi() {
        let functions = vec![view_fn("frobnicate", vec![])];
        assert!(auto_detect_invariants(&functions, &[]).is_empty());
    }
}
