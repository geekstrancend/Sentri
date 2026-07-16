//! `revm`-backed [`ExecutionBackend`] for real Solidity bytecode.
//!
//! UNVERIFIED IN THIS CHANGE: this file could not be compiled in the
//! environment it was written in (no network access to fetch the `revm`
//! crate, which is a declared-but-previously-unused workspace dependency —
//! see the crate README). The `revm` public API shown here (the
//! `Evm::builder()` / `with_db` / `modify_tx_env` / `TransactTo` /
//! `ExecutionResult` idioms) has been stable across a wide range of recent
//! revm versions, but exact type/method names for the pinned `revm = "14"`
//! have not been checked against real docs. Before relying on this:
//!
//! ```text
//! cargo build -p sentri-dynamic-evm
//! ```
//!
//! and fix whatever the compiler flags — this is the one piece of this
//! change that needs that pass.

use crate::types::CompiledContract;
use revm::db::InMemoryDB;
use revm::primitives::{AccountInfo, Address, Bytes, ExecutionResult, Output, TransactTo, U256};
use revm::Evm;
use sentri_dynamic_core::{CallOutcome, EncodedCall, ExecutionBackend};

/// A deployed contract instance plus the in-memory state it lives in.
/// `snapshot`/`revert_to` clone the whole DB rather than diffing it — state
/// for a single fuzzed contract under test is small, and correctness here
/// matters far more than shaving allocations off a debug tool.
pub struct RevmBackend {
    db: InMemoryDB,
    contract_address: Address,
    snapshots: Vec<InMemoryDB>,
}

impl RevmBackend {
    /// Deploy `init_code` (Solidity creation bytecode) and return a backend
    /// ready to receive calls against the resulting contract address.
    pub fn deploy(init_code: Vec<u8>, deployer: [u8; 20]) -> anyhow::Result<Self> {
        let deployer_addr = Address::from(deployer);
        let mut db = InMemoryDB::default();
        db.insert_account_info(
            deployer_addr,
            AccountInfo {
                balance: U256::from(u128::MAX),
                ..Default::default()
            },
        );

        let mut evm = Evm::builder()
            .with_db(db)
            .modify_tx_env(|tx| {
                tx.caller = deployer_addr;
                tx.transact_to = TransactTo::Create;
                tx.data = Bytes::from(init_code);
                tx.value = U256::ZERO;
            })
            .build();

        let result = evm
            .transact_commit()
            .map_err(|e| anyhow::anyhow!("deployment transaction failed: {e:?}"))?;

        let contract_address = match result {
            ExecutionResult::Success {
                output: Output::Create(_, Some(addr)),
                ..
            } => addr,
            ExecutionResult::Success { .. } => {
                anyhow::bail!("deployment succeeded but returned no contract address")
            }
            ExecutionResult::Revert { output, .. } => {
                anyhow::bail!("constructor reverted: 0x{}", hex::encode(output))
            }
            ExecutionResult::Halt { reason, .. } => {
                anyhow::bail!("constructor halted: {reason:?}")
            }
        };

        // Fund every actor the harness might use as a caller so `payable`
        // calls with a nonzero value don't spuriously revert on
        // insufficient balance — that's not the class of bug this engine
        // is trying to find.
        let db = evm.context.evm.db.clone();

        Ok(Self {
            db,
            contract_address,
            snapshots: Vec::new(),
        })
    }

    /// Pre-fund a set of addresses so they can be used as callers/`value`
    /// senders without running out of ETH mid-fuzz.
    pub fn fund_actors(&mut self, actors: &[[u8; 20]]) {
        for actor in actors {
            self.db.insert_account_info(
                Address::from(*actor),
                AccountInfo {
                    balance: U256::from(u128::MAX),
                    ..Default::default()
                },
            );
        }
    }

    pub fn contract_address(&self) -> [u8; 20] {
        self.contract_address.into_array()
    }
}

impl ExecutionBackend for RevmBackend {
    fn call(&mut self, call: &EncodedCall) -> CallOutcome {
        let db = std::mem::take(&mut self.db);
        let mut evm = Evm::builder()
            .with_db(db)
            .modify_tx_env(|tx| {
                tx.caller = Address::from(call.caller);
                tx.transact_to = TransactTo::Call(self.contract_address);
                tx.data = Bytes::from(call.calldata.clone());
                tx.value = U256::from(call.value);
            })
            .build();

        let outcome = match evm.transact_commit() {
            Ok(ExecutionResult::Success {
                output: Output::Call(data),
                ..
            }) => CallOutcome {
                reverted: false,
                return_data: data.to_vec(),
            },
            Ok(ExecutionResult::Success { .. })
            | Ok(ExecutionResult::Revert { .. })
            | Ok(ExecutionResult::Halt { .. }) => CallOutcome {
                reverted: true,
                return_data: Vec::new(),
            },
            Err(_) => CallOutcome {
                reverted: true,
                return_data: Vec::new(),
            },
        };

        self.db = evm.context.evm.db.clone();
        outcome
    }

    fn snapshot(&mut self) -> u64 {
        self.snapshots.push(self.db.clone());
        (self.snapshots.len() - 1) as u64
    }

    fn revert_to(&mut self, snapshot: u64) {
        if let Some(state) = self.snapshots.get(snapshot as usize) {
            self.db = state.clone();
        }
    }
}

/// Builds a fresh, already-deployed [`RevmBackend`] for one fuzz attempt.
/// `sentri_dynamic_core::fuzz` calls this many times (once per attempt,
/// more during shrinking) — each call redeploys from the same creation
/// bytecode so every attempt starts from an identical genesis state.
pub fn backend_factory<'a>(
    contract: &'a CompiledContract,
    actors: &'a [[u8; 20]],
) -> impl Fn() -> Box<dyn ExecutionBackend> + 'a {
    move || {
        let deployer = actors.first().copied().unwrap_or([0xAAu8; 20]);
        let mut backend = RevmBackend::deploy(contract.init_code.clone(), deployer)
            .expect("deployment must succeed against known-good bytecode compiled moments earlier");
        backend.fund_actors(actors);
        Box::new(backend)
    }
}
