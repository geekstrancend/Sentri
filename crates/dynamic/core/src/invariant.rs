//! Invariant oracles: properties checked after every call in a sequence.
//!
//! An invariant only ever *reads* state (through read-only calls on the
//! backend) — it never mutates. That keeps the fuzzing loop's mutation
//! path (the sequence under test) cleanly separated from the oracle path
//! (is the resulting state still valid).

use crate::abi_encode::{decode_uint256, encode_call};
use crate::backend::{EncodedCall, ExecutionBackend, FunctionSpec};
use crate::u256::{u256_add, u256_lt, u256_to_decimal};
use std::cell::RefCell;

pub trait Invariant {
    fn name(&self) -> &str;

    /// Called after every call in a sequence. Returns `Some(message)` if the
    /// property is currently violated.
    fn check(&self, backend: &mut dyn ExecutionBackend) -> Option<String>;

    /// Forget any state remembered from a previous sequence run (see
    /// [`crate::sequence::run_sequence`]). Default no-op for stateless
    /// invariants (e.g. [`ConservationInvariant`], which recomputes
    /// everything fresh on each check).
    fn reset(&self) {}
}

const ZERO_ADDR: [u8; 20] = [0u8; 20];

fn read_call(function: &FunctionSpec, args: &[[u8; 32]]) -> EncodedCall {
    EncodedCall {
        calldata: encode_call(function, args),
        function: function.clone(),
        caller: ZERO_ADDR,
        value: 0,
    }
}

/// A read-only function's `uint256` return value must never decrease across
/// a call sequence. Generalizes a very common real-world property class:
/// total supply, cumulative fees collected, vault share price, checkpoint
/// counters, and withdrawal-epoch indices are all expected to be
/// monotonic in legitimate contracts — a decrease is exactly the shape of
/// bug that lets an attacker roll back accounting state or re-extract value.
pub struct MonotonicInvariant {
    label: String,
    read_fn: FunctionSpec,
    last_seen: RefCell<Option<[u8; 32]>>,
}

impl MonotonicInvariant {
    pub fn new(label: impl Into<String>, read_fn: FunctionSpec) -> Self {
        Self {
            label: label.into(),
            read_fn,
            last_seen: RefCell::new(None),
        }
    }
}

impl Invariant for MonotonicInvariant {
    fn name(&self) -> &str {
        &self.label
    }

    fn reset(&self) {
        *self.last_seen.borrow_mut() = None;
    }

    fn check(&self, backend: &mut dyn ExecutionBackend) -> Option<String> {
        let outcome = backend.call(&read_call(&self.read_fn, &[]));
        if outcome.reverted {
            return None;
        }
        let current = decode_uint256(&outcome.return_data);
        let mut last = self.last_seen.borrow_mut();
        let violated = match *last {
            Some(prev) if u256_lt(&current, &prev) => Some(format!(
                "{} decreased: {} -> {}",
                self.label,
                u256_to_decimal(&prev),
                u256_to_decimal(&current)
            )),
            _ => None,
        };
        *last = Some(current);
        violated
    }
}

/// ERC20-conservation-shaped invariant: `sum(balanceOf(actor) for actor in
/// actors) == totalSupply()`. Catches the "credited a balance without a
/// matching debit/mint" bug class — accounting drift between per-account
/// ledgers and the aggregate figure is the root cause behind a long list of
/// real incidents (fee-on-transfer mishandling, duplicated credit on
/// reentrant transfer, bridge mint/burn mismatches).
pub struct ConservationInvariant {
    label: String,
    total_supply_fn: FunctionSpec,
    balance_of_fn: FunctionSpec,
    actors: Vec<[u8; 20]>,
}

impl ConservationInvariant {
    pub fn new(
        label: impl Into<String>,
        total_supply_fn: FunctionSpec,
        balance_of_fn: FunctionSpec,
        actors: Vec<[u8; 20]>,
    ) -> Self {
        Self {
            label: label.into(),
            total_supply_fn,
            balance_of_fn,
            actors,
        }
    }
}

impl Invariant for ConservationInvariant {
    fn name(&self) -> &str {
        &self.label
    }

    fn check(&self, backend: &mut dyn ExecutionBackend) -> Option<String> {
        let ts_outcome = backend.call(&read_call(&self.total_supply_fn, &[]));
        if ts_outcome.reverted {
            return None;
        }
        let total_supply = decode_uint256(&ts_outcome.return_data);

        let mut sum = [0u8; 32];
        for actor in &self.actors {
            let mut word = [0u8; 32];
            word[12..32].copy_from_slice(actor);
            let outcome = backend.call(&read_call(&self.balance_of_fn, &[word]));
            if outcome.reverted {
                continue;
            }
            sum = u256_add(&sum, &decode_uint256(&outcome.return_data));
        }

        if sum != total_supply {
            return Some(format!(
                "{}: sum(balanceOf) = {} != totalSupply() = {}",
                self.label,
                u256_to_decimal(&sum),
                u256_to_decimal(&total_supply)
            ));
        }
        None
    }
}
