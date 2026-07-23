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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::{CallOutcome, ParamKind};
    use std::collections::{HashMap, VecDeque};

    // ── fixtures ──

    const OWNER: [u8; 20] = [0xAA; 20];
    const ATTACKER: [u8; 20] = [0xBB; 20];
    const CONTRACT: [u8; 20] = [0xCC; 20];

    fn uint(n: u64) -> Vec<u8> {
        let mut w = [0u8; 32];
        w[24..32].copy_from_slice(&n.to_be_bytes());
        w.to_vec()
    }
    fn addr_ret(a: [u8; 20]) -> Vec<u8> {
        let mut w = [0u8; 32];
        w[12..32].copy_from_slice(&a);
        w.to_vec()
    }
    fn ok(data: Vec<u8>) -> CallOutcome {
        CallOutcome {
            reverted: false,
            return_data: data,
        }
    }
    fn reverted() -> CallOutcome {
        CallOutcome {
            reverted: true,
            return_data: Vec::new(),
        }
    }
    fn read_fn(name: &str, selector: [u8; 4], inputs: Vec<ParamKind>) -> FunctionSpec {
        FunctionSpec::new(name, selector, inputs, false)
    }
    fn call_with(selector: [u8; 4], caller: [u8; 20]) -> EncodedCall {
        EncodedCall {
            function: FunctionSpec::new("x", selector, Vec::new(), true),
            calldata: selector.to_vec(),
            caller,
            value: 0,
        }
    }

    /// Backend that replays a scripted queue of outcomes per selector, so each
    /// invariant branch (including the revert paths) can be driven exactly.
    /// An exhausted or unknown selector reverts — the safe default.
    #[derive(Default)]
    struct MockBackend {
        queues: HashMap<[u8; 4], VecDeque<CallOutcome>>,
    }
    impl MockBackend {
        fn on(mut self, selector: [u8; 4], outcomes: Vec<CallOutcome>) -> Self {
            self.queues.insert(selector, outcomes.into());
            self
        }
    }
    impl ExecutionBackend for MockBackend {
        fn call(&mut self, call: &EncodedCall) -> CallOutcome {
            self.queues
                .get_mut(&call.function.selector)
                .and_then(|q| q.pop_front())
                .unwrap_or_else(reverted)
        }
        fn snapshot(&mut self) -> u64 {
            0
        }
        fn revert_to(&mut self, _snapshot: u64) {}
    }

    fn ctx<'a>(last_call: &'a EncodedCall, trace: &'a [TraceEvent]) -> CheckContext<'a> {
        CheckContext { last_call, trace }
    }

    // ── MonotonicInvariant ──

    const READ: [u8; 4] = [0x11; 4];

    #[test]
    fn monotonic_flags_a_decrease_from_the_reset_baseline() {
        let inv = MonotonicInvariant::new("supply", read_fn("s", READ, vec![]));
        let mut b = MockBackend::default().on(READ, vec![ok(uint(100)), ok(uint(40))]);
        inv.reset(&mut b); // baseline 100
        let call = call_with(READ, OWNER);
        let msg = inv
            .check(&mut b, &ctx(&call, &[]))
            .expect("decrease must fire");
        assert!(msg.contains("100"));
        assert!(msg.contains("40"));
    }

    #[test]
    fn monotonic_increase_is_not_a_violation() {
        let inv = MonotonicInvariant::new("supply", read_fn("s", READ, vec![]));
        let mut b = MockBackend::default().on(READ, vec![ok(uint(10)), ok(uint(99))]);
        inv.reset(&mut b);
        let call = call_with(READ, OWNER);
        assert!(inv.check(&mut b, &ctx(&call, &[])).is_none());
    }

    #[test]
    fn monotonic_reverting_reads_are_ignored_in_reset_and_check() {
        let inv = MonotonicInvariant::new("supply", read_fn("s", READ, vec![]));
        // reset reverts (baseline stays None), then a later check also reverts.
        let mut b = MockBackend::default().on(READ, vec![reverted(), reverted()]);
        inv.reset(&mut b);
        let call = call_with(READ, OWNER);
        assert!(inv.check(&mut b, &ctx(&call, &[])).is_none());
    }

    // ── ConservationInvariant ──

    const TOTAL: [u8; 4] = [0x22; 4];
    const BAL: [u8; 4] = [0x33; 4];

    fn conservation(actors: Vec<[u8; 20]>) -> ConservationInvariant {
        ConservationInvariant::new(
            "erc20",
            read_fn("totalSupply", TOTAL, vec![]),
            read_fn("balanceOf", BAL, vec![ParamKind::Address]),
            actors,
        )
    }

    #[test]
    fn conservation_flags_a_supply_mismatch() {
        let inv = conservation(vec![OWNER]);
        let mut b = MockBackend::default()
            .on(TOTAL, vec![ok(uint(100))])
            .on(BAL, vec![ok(uint(40))]);
        let call = call_with(READ, OWNER);
        let msg = inv
            .check(&mut b, &ctx(&call, &[]))
            .expect("mismatch must fire");
        assert!(msg.contains("40"));
        assert!(msg.contains("100"));
    }

    #[test]
    fn conservation_holds_when_balances_sum_to_supply() {
        let inv = conservation(vec![OWNER, ATTACKER]);
        let mut b = MockBackend::default()
            .on(TOTAL, vec![ok(uint(100))])
            .on(BAL, vec![ok(uint(60)), ok(uint(40))]);
        let call = call_with(READ, OWNER);
        assert!(inv.check(&mut b, &ctx(&call, &[])).is_none());
    }

    #[test]
    fn conservation_is_a_noop_when_total_supply_reverts() {
        let inv = conservation(vec![OWNER]);
        let mut b = MockBackend::default().on(TOTAL, vec![reverted()]);
        let call = call_with(READ, OWNER);
        assert!(inv.check(&mut b, &ctx(&call, &[])).is_none());
    }

    #[test]
    fn conservation_skips_actors_whose_balance_call_reverts() {
        // First actor's balanceOf reverts (skipped); second returns the full
        // supply, so the surviving sum still matches and nothing is flagged.
        let inv = conservation(vec![ATTACKER, OWNER]);
        let mut b = MockBackend::default()
            .on(TOTAL, vec![ok(uint(50))])
            .on(BAL, vec![reverted(), ok(uint(50))]);
        let call = call_with(READ, OWNER);
        assert!(inv.check(&mut b, &ctx(&call, &[])).is_none());
    }

    // ── AccessControlInvariant ──

    const OWNER_FN: [u8; 4] = [0x44; 4];
    const GUARDED: [u8; 4] = [0x55; 4];
    const OTHER: [u8; 4] = [0x66; 4];

    fn access_control() -> AccessControlInvariant {
        AccessControlInvariant::new(
            "ownable",
            read_fn("owner", OWNER_FN, vec![]),
            read_fn("transferOwnership", GUARDED, vec![ParamKind::Address]),
        )
    }

    #[test]
    fn access_control_flags_ownership_seized_by_a_non_owner() {
        let inv = access_control();
        // reset: owner is OWNER. Then a guarded call from ATTACKER leaves
        // ATTACKER as owner.
        let mut b =
            MockBackend::default().on(OWNER_FN, vec![ok(addr_ret(OWNER)), ok(addr_ret(ATTACKER))]);
        inv.reset(&mut b);
        let call = call_with(GUARDED, ATTACKER);
        let msg = inv
            .check(&mut b, &ctx(&call, &[]))
            .expect("unauthorized ownership change must fire");
        assert!(msg.contains("non-owner"));
    }

    #[test]
    fn access_control_allows_a_legitimate_transfer_by_the_owner() {
        let inv = access_control();
        // The owner themselves hands ownership to ATTACKER — authorized.
        let mut b =
            MockBackend::default().on(OWNER_FN, vec![ok(addr_ret(OWNER)), ok(addr_ret(ATTACKER))]);
        inv.reset(&mut b);
        let call = call_with(GUARDED, OWNER);
        assert!(inv.check(&mut b, &ctx(&call, &[])).is_none());
    }

    #[test]
    fn access_control_syncs_baseline_on_non_guarded_calls() {
        let inv = access_control();
        // A call to some other function just refreshes the tracked owner.
        let mut b = MockBackend::default().on(OWNER_FN, vec![ok(addr_ret(OWNER))]);
        let call = call_with(OTHER, ATTACKER);
        assert!(inv.check(&mut b, &ctx(&call, &[])).is_none());
    }

    #[test]
    fn access_control_ignores_a_reverting_owner_read() {
        let inv = access_control();
        // owner() reverts during the guarded-call check → nothing to compare.
        let mut b = MockBackend::default().on(OWNER_FN, vec![reverted()]);
        let call = call_with(GUARDED, ATTACKER);
        assert!(inv.check(&mut b, &ctx(&call, &[])).is_none());
    }

    // ── ReentrancyInvariant ──

    #[test]
    fn reentrancy_flags_a_late_write_after_non_reverting_reentry() {
        let inv = ReentrancyInvariant::new("reentrancy");
        // Textbook write-after-external-call: the re-entered frame's storage
        // write lands after control has already re-entered (see
        // `trace::detect_reentrancy` — the late write is what makes it
        // exploitable).
        let trace = [
            TraceEvent::CallBegin { address: CONTRACT }, // outer
            TraceEvent::CallBegin { address: ATTACKER }, //   external call
            TraceEvent::CallBegin { address: CONTRACT }, //     re-entry
            TraceEvent::StorageWrite { address: CONTRACT },
            TraceEvent::CallEnd { reverted: false }, //     inner returns
            TraceEvent::CallEnd { reverted: false }, //   attacker returns
            TraceEvent::StorageWrite { address: CONTRACT }, // outer writes too late
            TraceEvent::CallEnd { reverted: false }, // outer returns
        ];
        let mut b = MockBackend::default();
        let call = call_with(READ, ATTACKER);
        let msg = inv
            .check(&mut b, &ctx(&call, &trace))
            .expect("exploitable reentrancy must fire");
        assert!(msg.contains("re-entered"));
    }

    #[test]
    fn reentrancy_is_a_noop_on_an_empty_trace() {
        let inv = ReentrancyInvariant::new("reentrancy");
        let mut b = MockBackend::default();
        let call = call_with(READ, ATTACKER);
        assert!(inv.check(&mut b, &ctx(&call, &[])).is_none());
    }
}
