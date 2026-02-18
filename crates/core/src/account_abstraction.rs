//! Account Abstraction cross-layer invariant support.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

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
    /// Layer-specific state variables.
    pub layer_state: BTreeMap<String, BTreeMap<String, serde_json::Value>>,

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
}
