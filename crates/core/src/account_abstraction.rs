//! Account Abstraction cross-layer invariant support.
//!
//! Supports phase-qualified invariants for ERC-4337 execution:
//! - Validation Phase: validateUserOp, account signature checks
//! - Execution Phase: account code execution, state mutations
//! - Settlement Phase: bundles with other ops, fund transfers

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Execution phases in the ERC-4337 UserOp lifecycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub enum ExecutionPhase {
    /// Validation phase: validateUserOp, signature checks, paymaster verification.
    Validation,
    /// Execution phase: account code runs, state mutations, call execution.
    Execution,
    /// Settlement phase: bundled with other ops, balances transferred.
    Settlement,
}

impl ExecutionPhase {
    /// Get string representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Validation => "validation",
            Self::Execution => "execution",
            Self::Settlement => "settlement",
        }
    }
}

impl std::str::FromStr for ExecutionPhase {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "validation" => Ok(Self::Validation),
            "execution" => Ok(Self::Execution),
            "settlement" => Ok(Self::Settlement),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for ExecutionPhase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Identifies different layers in account abstraction execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub enum AALayer {
    /// Bundler mempool and aggregation layer.
    Bundler,
    /// Account smart contract execution layer.
    Account,
    /// Optional paymaster sponsorship layer.
    Paymaster,
    /// Protocol and EntryPoint layer.
    Protocol,
    /// EntryPoint contract layer.
    EntryPoint,
}

impl AALayer {
    /// Get string representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Bundler => "bundler",
            Self::Account => "account",
            Self::Paymaster => "paymaster",
            Self::Protocol => "protocol",
            Self::EntryPoint => "entrypoint",
        }
    }
}

impl std::str::FromStr for AALayer {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bundler" => Ok(Self::Bundler),
            "account" => Ok(Self::Account),
            "paymaster" => Ok(Self::Paymaster),
            "protocol" => Ok(Self::Protocol),
            "entrypoint" => Ok(Self::EntryPoint),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for AALayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Cross-layer context for account abstraction analysis.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AAContext {
    /// Current execution phase (Validation, Execution, or Settlement).
    pub current_phase: Option<ExecutionPhase>,

    /// Layer-specific state variables: layer -> (variable -> value).
    pub layer_state: BTreeMap<String, BTreeMap<String, serde_json::Value>>,

    /// Phase-specific state snapshots for cross-phase analysis: phase -> layer_state.
    pub phase_snapshots: BTreeMap<String, BTreeMap<String, BTreeMap<String, serde_json::Value>>>,

    /// UserOperation data from bundler layer.
    pub user_op: Option<UserOpData>,

    /// Account state from account layer.
    pub account_state: Option<AccountState>,

    /// Paymaster state from paymaster layer (if applicable).
    pub paymaster_state: Option<PaymasterState>,

    /// EntryPoint state from protocol layer.
    pub entry_point_state: Option<EntryPointState>,
}

impl AAContext {
    /// Set the current execution phase.
    pub fn set_phase(&mut self, phase: ExecutionPhase) {
        self.current_phase = Some(phase);
    }

    /// Get the current execution phase.
    pub fn get_phase(&self) -> Option<ExecutionPhase> {
        self.current_phase
    }

    /// Check if currently in a specific phase.
    pub fn in_phase(&self, phase: ExecutionPhase) -> bool {
        self.current_phase == Some(phase)
    }

    /// Snapshot layer state at current phase.
    pub fn snapshot_phase(&mut self, phase: ExecutionPhase) {
        self.phase_snapshots
            .insert(phase.to_string(), self.layer_state.clone());
    }

    /// Get snapshots from a specific phase for comparison.
    pub fn get_phase_snapshot(
        &self,
        phase: ExecutionPhase,
    ) -> Option<&BTreeMap<String, BTreeMap<String, serde_json::Value>>> {
        self.phase_snapshots.get(phase.as_str())
    }

    /// Get variable value from a specific layer.
    pub fn get_layer_var(&self, layer: &str, var: &str) -> Option<&serde_json::Value> {
        self.layer_state.get(layer)?.get(var)
    }

    /// Set variable value in a specific layer.
    pub fn set_layer_var(&mut self, layer: String, var: String, value: serde_json::Value) {
        self.layer_state
            .entry(layer)
            .or_default()
            .insert(var, value);
    }

    /// Get variable value from a specific layer at a specific phase.
    pub fn get_layer_var_at_phase(
        &self,
        phase: ExecutionPhase,
        layer: &str,
        var: &str,
    ) -> Option<&serde_json::Value> {
        self.phase_snapshots
            .get(phase.as_str())?
            .get(layer)?
            .get(var)
    }
}

/// UserOperation data from bundler layer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserOpData {
    /// Account address.
    pub sender: String,
    /// Operation nonce.
    pub nonce: u128,
    /// Init code if deploying new account.
    pub init_code: Vec<u8>,
    /// Call data.
    pub call_data: Vec<u8>,
    /// Call gas limit.
    pub call_gas_limit: u128,
    /// Verification gas limit.
    pub verification_gas_limit: u128,
    /// Pre-operation gas.
    pub pre_op_gas: u128,
    /// Maximum gas price account will accept.
    pub max_gas_price: u128,
    /// Maximum priority gas price.
    pub max_priority_fee_per_gas: u128,
    /// Paymaster address (if applicable).
    pub paymaster_and_data: Vec<u8>,
    /// Signature data.
    pub signature: Vec<u8>,
}

/// Account smart contract state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountState {
    /// Account nonce value.
    pub nonce: u128,
    /// Account balance.
    pub balance: u128,
    /// Expected signer address.
    pub expected_signer: String,
    /// Whether signature is valid.
    pub signature_valid: bool,
    /// Reentrancy guard state.
    pub reentrancy_locked: bool,
    /// Execution failed flag.
    pub execution_failed: bool,
    /// State hash before execution.
    pub state_hash_before: String,
    /// State hash after execution.
    pub state_hash_after: String,
}

/// Paymaster contract state (optional).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymasterState {
    /// Paymaster address.
    pub address: String,
    /// Deposit amount.
    pub deposit: u128,
    /// Paymaster nonce.
    pub nonce: u128,
    /// Verification status.
    pub status: String,
}

/// EntryPoint contract state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntryPointState {
    /// EntryPoint address.
    pub address: String,
    /// Current block number.
    pub block_number: u128,
    /// Current block timestamp.
    pub block_timestamp: u128,
    /// Authenticated caller (msg.sender during handleOps).
    pub authenticated_caller: String,
}

/// Cross-layer invariant check result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossLayerCheckResult {
    /// Invariant name.
    pub invariant_name: String,
    /// Layers involved in this check.
    pub layers_involved: Vec<String>,
    /// Whether the invariant holds.
    pub holds: bool,
    /// Detailed reason if invariant fails.
    pub failure_reason: Option<String>,
    /// Variables used in evaluation.
    pub variables_used: BTreeMap<String, serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_execution_phase_from_str() {
        assert_eq!(
            ExecutionPhase::from_str("validation"),
            Ok(ExecutionPhase::Validation)
        );
        assert_eq!(
            ExecutionPhase::from_str("execution"),
            Ok(ExecutionPhase::Execution)
        );
        assert_eq!(
            ExecutionPhase::from_str("settlement"),
            Ok(ExecutionPhase::Settlement)
        );
        assert_eq!(ExecutionPhase::from_str("invalid"), Err(()));
    }

    #[test]
    fn test_aa_layer_from_str() {
        assert_eq!(AALayer::from_str("bundler"), Ok(AALayer::Bundler));
        assert_eq!(AALayer::from_str("account"), Ok(AALayer::Account));
        assert_eq!(AALayer::from_str("paymaster"), Ok(AALayer::Paymaster));
        assert_eq!(AALayer::from_str("protocol"), Ok(AALayer::Protocol));
        assert_eq!(AALayer::from_str("invalid"), Err(()));
    }

    #[test]
    fn test_aa_context_layer_vars() {
        let mut ctx = AAContext::default();
        ctx.set_layer_var(
            "bundler".to_string(),
            "nonce".to_string(),
            serde_json::json!(42),
        );

        let value = ctx.get_layer_var("bundler", "nonce");
        assert_eq!(value, Some(&serde_json::json!(42)));
    }

    #[test]
    fn test_phase_tracking() {
        let mut ctx = AAContext::default();
        assert_eq!(ctx.get_phase(), None);

        ctx.set_phase(ExecutionPhase::Validation);
        assert!(ctx.in_phase(ExecutionPhase::Validation));
        assert!(!ctx.in_phase(ExecutionPhase::Execution));

        ctx.set_layer_var(
            "account".to_string(),
            "balance".to_string(),
            serde_json::json!(1000),
        );
        ctx.snapshot_phase(ExecutionPhase::Validation);

        ctx.set_phase(ExecutionPhase::Execution);
        ctx.set_layer_var(
            "account".to_string(),
            "balance".to_string(),
            serde_json::json!(500),
        );

        let pre_exec_balance =
            ctx.get_layer_var_at_phase(ExecutionPhase::Validation, "account", "balance");
        assert_eq!(pre_exec_balance, Some(&serde_json::json!(1000)));
    }
}
