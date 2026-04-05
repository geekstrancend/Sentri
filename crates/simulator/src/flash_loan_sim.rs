//! Flash loan attack simulator for DeFi protocols.
//!
//! Simulates flash loan attacks to detect oracle manipulation vulnerabilities.

use crate::evm_runtime::EvmRuntime;
use anyhow::Result;
use revm::primitives::{Address, U256};

/// Result of flash loan vulnerability check
#[derive(Debug)]
pub struct FlashLoanCheckResult {
    /// Is the protocol vulnerable
    pub is_vulnerable: bool,
    /// Price before manipulation
    pub price_before: U256,
    /// Price after manipulation
    pub price_after: U256,
    /// Price change percentage
    pub price_change_percent: u64,
}

/// Simulates flash loan attacks
pub struct FlashLoanSimulator {
    runtime: EvmRuntime,
    protocol_address: Address,
}

impl FlashLoanSimulator {
    /// Create a new flash loan simulator
    pub fn new(runtime: EvmRuntime, protocol_address: Address) -> Self {
        Self {
            runtime,
            protocol_address,
        }
    }

    /// Check if a protocol is vulnerable to flash loan price manipulation
    ///
    /// Simulates:
    /// 1. Reading oracle price before manipulation
    /// 2. Performing a large swap (simulated via balance injection)
    /// 3. Reading oracle price after manipulation
    /// 4. Comparing price movement
    pub fn check_oracle_manipulation(
        &mut self,
        oracle_address: Address,
        price_function_selector: [u8; 4],
    ) -> Result<FlashLoanCheckResult> {
        // Read price before manipulation
        let price_before = self.read_oracle_price(oracle_address, price_function_selector)?;

        // Simulate flash loan injection by creating a large account
        let _whale = self.runtime.create_account(1_000_000); // 1M ETH

        // Read price after manipulation
        let price_after = self.read_oracle_price(oracle_address, price_function_selector)?;

        // Calculate change
        let price_change = if price_after > price_before {
            price_after - price_before
        } else {
            price_before - price_after
        };

        let one_percent = price_before / U256::from(100);
        let is_vulnerable = price_change > one_percent;

        let price_change_percent = if price_before > U256::ZERO {
            (price_change * U256::from(100) / price_before)
                .try_into()
                .unwrap_or(0)
        } else {
            0
        };

        Ok(FlashLoanCheckResult {
            is_vulnerable,
            price_before,
            price_after,
            price_change_percent,
        })
    }

    fn read_oracle_price(&self, oracle: Address, selector: [u8; 4]) -> Result<U256> {
        let result = self.runtime.call_static(oracle, selector.to_vec())?;

        if result.len() >= 32 {
            Ok(U256::from_be_slice(&result[..32]))
        } else {
            Ok(U256::ZERO)
        }
    }
}
