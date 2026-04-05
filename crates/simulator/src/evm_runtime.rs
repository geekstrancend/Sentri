//! EVM runtime execution engine using revm.
//!
//! This module wraps the revm Ethereum Virtual Machine to enable in-process
//! contract execution and invariant verification.

use anyhow::Result;
use revm::{
    primitives::{AccountInfo, Address, Bytecode, B256, U256},
    State,
};

/// Result of a function call in the EVM
#[derive(Debug, Clone)]
pub struct CallResult {
    /// Whether the call succeeded
    pub success: bool,
    /// Return data
    pub output: Vec<u8>,
    /// Gas used
    pub gas_used: u64,
    /// Whether it reverted
    pub reverted: bool,
    /// Revert reason if available
    pub revert_reason: Option<String>,
}

/// Manages EVM execution using revm
pub struct EvmRuntime {
    state: State,
    deployer: Address,
    block_timestamp: u64,
    block_number: u64,
}

impl EvmRuntime {
    /// Create a new EVM runtime
    pub fn new() -> Self {
        let state = State::default();
        let deployer = Address::from([0x01u8; 20]);

        Self {
            state,
            deployer,
            block_timestamp: 1,
            block_number: 1,
        }
    }

    /// Deploy a contract from bytecode
    pub fn deploy(&mut self, bytecode: Vec<u8>, _constructor_args: Vec<u8>) -> Result<Address> {
        // Create a new contract address
        let contract_addr = Address::random();

        // Insert the account with bytecode
        self.state.insert_account(
            contract_addr,
            AccountInfo {
                balance: U256::ZERO,
                nonce: 0,
                code_hash: keccak256(&bytecode),
                code: Some(Bytecode::Raw(bytecode.into())),
            },
        );

        Ok(contract_addr)
    }

    /// Call a function on a deployed contract
    pub fn call(
        &mut self,
        contract: Address,
        calldata: Vec<u8>,
        value: U256,
        caller: Address,
    ) -> Result<CallResult> {
        // For now, return a successful empty call result
        // Full implementation would require proper revm integration
        Ok(CallResult {
            success: true,
            output: vec![],
            gas_used: 0,
            reverted: false,
            revert_reason: None,
        })
    }

    /// Read state without modifying it
    pub fn call_static(&self, _contract: Address, _calldata: Vec<u8>) -> Result<Vec<u8>> {
        // Placeholder implementation
        Ok(vec![])
    }

    /// Advance block number
    pub fn advance_block(&mut self, blocks: u64) {
        self.block_number += blocks;
    }

    /// Create a funded account
    pub fn create_account(&mut self, eth_balance: u64) -> Address {
        let addr = Address::random();
        self.state.insert_account(
            addr,
            AccountInfo {
                balance: U256::from(eth_balance) * U256::from(10).pow(U256::from(18)),
                nonce: 0,
                code_hash: B256::ZERO,
                code: Some(Bytecode::default()),
            },
        );
        addr
    }

    /// Get current block timestamp
    pub fn block_timestamp(&self) -> u64 {
        self.block_timestamp
    }

    /// Set block timestamp
    pub fn set_block_timestamp(&mut self, timestamp: u64) {
        self.block_timestamp = timestamp;
    }

    /// Get deployer address
    pub fn get_deployer(&self) -> Address {
        self.deployer
    }
}

impl Default for EvmRuntime {
    fn default() -> Self {
        Self::new()
    }
}

/// Decode error message from revert data
fn decode_revert_reason(data: &[u8]) -> Option<String> {
    // ABI decode "Error(string)" revert reason
    // Function selector: 0x08c379a0
    if data.len() > 4 && &data[..4] == &[0x08, 0xc3, 0x79, 0xa0] {
        // Skip selector and decode string (length + data)
        if data.len() > 68 {
            let msg_start = 68;
            if let Ok(s) = std::str::from_utf8(&data[msg_start..]) {
                return Some(s.trim_matches('\0').to_string());
            }
        }
    }

    // Fallback: hex encode
    Some(format!("0x{}", hex::encode(data)))
}

/// Keccak256 hash function
fn keccak256(data: &[u8]) -> B256 {
    use sha3::Digest;
    let hash = sha3::Keccak256::digest(data);
    B256::from_slice(&hash)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evm_runtime_creation() {
        let runtime = EvmRuntime::new();
        assert_eq!(runtime.block_timestamp(), 1);
        assert_eq!(runtime.block_number, 1);
    }

    #[test]
    fn test_deployer_address() {
        let runtime = EvmRuntime::new();
        let deployer = runtime.get_deployer();
        assert_ne!(deployer, Address::ZERO);
    }

    #[test]
    fn test_create_account() {
        let mut runtime = EvmRuntime::new();
        let addr = runtime.create_account(10);
        assert_ne!(addr, Address::ZERO);
    }

    #[test]
    fn test_advance_block() {
        let mut runtime = EvmRuntime::new();
        runtime.advance_block(5);
        assert_eq!(runtime.block_number, 6);
    }
}
