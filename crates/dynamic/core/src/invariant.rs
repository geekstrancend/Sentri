//! Invariant oracles: properties checked after every call in a sequence.
//!
//! An invariant only ever *reads* state (through read-only calls on the
//! backend) — it never mutates. That keeps the fuzzing loop's mutation
//! path (the sequence under test) cleanly separated from the oracle path
//! (is the resulting state still valid).

use crate::abi_encode::{decode_address, decode_uint256, encode_call};
use crate::backend::{EncodedCall, ExecutionBackend, FunctionSpec};
use crate::trace::{detect_reentrancy, TraceEvent};
use crate::u256::{u256_add, u256_lt, u256_to_decimal};
use std::cell::RefCell;

/// Everything an [`Invariant::check`] needs about the call that just
/// executed, beyond the backend's current state. Bundling these into one
/// struct means adding a new observation later (extra trace detail, gas,
/// events) doesn't churn every invariant's signature.
pub struct CheckContext<'a> {
    /// The mutating call that was just executed.
    pub last_call: &'a EncodedCall,
    /// The execution trace of that call (empty on backends that can't
    /// observe execution structure — see
    /// [`ExecutionBackend::last_call_trace`]). Captured once, right after
    /// the mutating call, so it isn't clobbered by the read-only calls
    /// other invariants issue during their own checks.
    pub trace: &'a [TraceEvent],
}

pub trait Invariant {
    fn name(&self) -> &str;

    /// Called after every call in a sequence, given a [`CheckContext`]
    /// describing the call that was just executed. Returns `Some(message)`
    /// if the property is currently violated.
    fn check(&self, backend: &mut dyn ExecutionBackend, ctx: &CheckContext) -> Option<String>;

    /// Called once at the start of each sequence run (see
    /// [`crate::sequence::run_sequence`]), before any calls execute, with
    /// the backend already in its genesis/deployed state. Lets a stateful
    /// invariant seed its baseline from the contract's real
    /// constructor-time state instead of starting from "unknown" — without
    /// this, an invariant that only remembers "the value as of the last
    /// check" would miss a violation on the very first call of a sequence,
    /// since it would have nothing yet to compare against. Default no-op
    /// for stateless invariants (e.g. [`ConservationInvariant`], which
    /// recomputes everything fresh on each check).
    fn reset(&self, backend: &mut dyn ExecutionBackend) {
        let _ = backend;
    }
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

    fn reset(&self, backend: &mut dyn ExecutionBackend) {
        // Seed with the real genesis value (state right after deployment,
        // before any fuzzed calls) rather than `None`. Without this, a
        // decrease on the very first call of a sequence would have nothing
        // to compare against and would silently become the new baseline
        // instead of being flagged — e.g. a constructor that mints an
        // initial totalSupply, immediately followed by a buggy call that
        // reduces it, needs the constructor's value as the reference point.
        let outcome = backend.call(&read_call(&self.read_fn, &[]));
        *self.last_seen.borrow_mut() = if outcome.reverted {
            None
        } else {
            Some(decode_uint256(&outcome.return_data))
        };
    }

    fn check(&self, backend: &mut dyn ExecutionBackend, _ctx: &CheckContext) -> Option<String> {
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

    fn check(&self, backend: &mut dyn ExecutionBackend, _ctx: &CheckContext) -> Option<String> {
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

/// Ownership-transfer authorization check, targeting the extremely common
/// OpenZeppelin-`Ownable` shape: `owner()` (view, no args, returns address)
/// plus a guarded function (typically `transferOwnership(address)`) that's
/// supposed to only succeed when called by the current owner. If ownership
/// changes as a result of a call to the guarded function, and the caller of
/// that call wasn't the owner *before* the change, that's a missing
/// access-control check — a `transferOwnership`-style privileged setter
/// that forgot its `onlyOwner` modifier hands full control to whoever calls
/// it, one of the most common real-world vulnerability classes.
pub struct AccessControlInvariant {
    label: String,
    owner_fn: FunctionSpec,
    guarded_fn: FunctionSpec,
    last_owner: RefCell<Option<[u8; 20]>>,
}

impl AccessControlInvariant {
    pub fn new(label: impl Into<String>, owner_fn: FunctionSpec, guarded_fn: FunctionSpec) -> Self {
        Self {
            label: label.into(),
            owner_fn,
            guarded_fn,
            last_owner: RefCell::new(None),
        }
    }

    fn read_owner(&self, backend: &mut dyn ExecutionBackend) -> Option<[u8; 20]> {
        let outcome = backend.call(&read_call(&self.owner_fn, &[]));
        if outcome.reverted {
            None
        } else {
            Some(decode_address(&outcome.return_data))
        }
    }
}

impl Invariant for AccessControlInvariant {
    fn name(&self) -> &str {
        &self.label
    }

    fn reset(&self, backend: &mut dyn ExecutionBackend) {
        let owner = self.read_owner(backend);
        *self.last_owner.borrow_mut() = owner;
    }

    fn check(&self, backend: &mut dyn ExecutionBackend, ctx: &CheckContext) -> Option<String> {
        // Only the guarded function is supposed to be able to change
        // ownership. For every other call, just keep the baseline synced
        // to current state so a later check against the guarded function
        // compares against an up-to-date "before" value rather than a
        // stale one from several calls ago.
        if ctx.last_call.function.selector != self.guarded_fn.selector {
            if let Some(owner) = self.read_owner(backend) {
                *self.last_owner.borrow_mut() = Some(owner);
            }
            return None;
        }

        let prev_owner = *self.last_owner.borrow();
        let current_owner = self.read_owner(backend)?;

        let violated = match prev_owner {
            Some(prev) if current_owner != prev && ctx.last_call.caller != prev => Some(format!(
                "{}: owner changed from 0x{} to 0x{} via a call from non-owner 0x{}",
                self.label,
                hex_addr(&prev),
                hex_addr(&current_owner),
                hex_addr(&ctx.last_call.caller)
            )),
            _ => None,
        };
        *self.last_owner.borrow_mut() = Some(current_owner);
        violated
    }
}

fn hex_addr(addr: &[u8; 20]) -> String {
    addr.iter().map(|b| format!("{:02x}", b)).collect()
}

/// Reentrancy detector. Unlike the state-comparison invariants, this one
/// reads nothing from the backend after the fact — it inspects the
/// execution *trace* of the call that just ran (see [`crate::trace`]) and
/// flags the classic exploitable-reentrancy pattern: a contract re-entered
/// by a non-reverting nested call that then writes its own storage after
/// the re-entry (a checks-effects-interactions violation).
///
/// It fires only when the trace actually shows re-entry, so it's a no-op on
/// backends that don't produce traces, and on runs where no re-entrant
/// contract was ever in the call path. Triggering it in practice requires a
/// re-entrant contract to be exercised (see the `revm` backend's reentrancy
/// probe) — a purely EOA-driven call sequence can never produce re-entry,
/// which is exactly why this is trace-based rather than state-based.
pub struct ReentrancyInvariant {
    label: String,
}

impl ReentrancyInvariant {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
        }
    }
}

impl Invariant for ReentrancyInvariant {
    fn name(&self) -> &str {
        &self.label
    }

    fn check(&self, _backend: &mut dyn ExecutionBackend, ctx: &CheckContext) -> Option<String> {
        detect_reentrancy(ctx.trace).map(|report| {
            format!(
                "{}: contract 0x{} was re-entered and wrote its own storage after the re-entry (checks-effects-interactions violation)",
                self.label,
                hex_addr(&report.address)
            )
        })
    }
}
