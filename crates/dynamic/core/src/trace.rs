//! Execution-trace model and reentrancy analysis.
//!
//! Reentrancy is fundamentally different from the state-comparison
//! invariants (Monotonic/Conservation/AccessControl): you can't detect it
//! by reading state before and after a call, because the damage happens
//! *during* a single call, in a nested re-entrant frame. Detecting it needs
//! visibility into the call structure of that one call — which contract
//! called which, in what order, whether inner calls reverted, and when
//! storage was written.
//!
//! An [`crate::backend::ExecutionBackend`] that can observe execution (the
//! `revm` backend, via an inspector) reports that structure as a flat
//! `Vec<TraceEvent>` after each call. The analysis in this module
//! ([`detect_reentrancy`]) is pure — it takes a trace and decides whether
//! it exhibits the exploitable-reentrancy pattern — so it is fully testable
//! offline against hand-constructed traces, independent of any VM.

/// A single observed execution event, in execution order. `CallBegin`/
/// `CallEnd` bracket a call frame (they nest); `StorageWrite` records that
/// a contract mutated its own storage at that point in the trace.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TraceEvent {
    CallBegin {
        address: [u8; 20],
    },
    /// `reverted` distinguishes a call that completed normally from one that
    /// was rolled back — crucial for reentrancy: a `nonReentrant`-guarded
    /// contract still *enters* the re-entrant call (so `CallBegin` fires),
    /// but that inner frame reverts, which is exactly what makes it safe.
    CallEnd {
        reverted: bool,
    },
    StorageWrite {
        address: [u8; 20],
    },
}

/// The finding produced when [`detect_reentrancy`] flags a trace.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReentrancyReport {
    /// The contract that was re-entered.
    pub address: [u8; 20],
}

/// Decides whether `trace` exhibits *exploitable* reentrancy, using a
/// deliberately conservative two-part rule that mirrors what a real
/// reentrancy PoC actually demonstrates — not just "a contract appears
/// twice on the call stack", which alone produces false positives on
/// safe-but-reentrant contracts and on guarded contracts whose re-entry is
/// blocked.
///
/// A trace is flagged when both hold for some contract `A`:
///
/// 1. **A non-reverting re-entrant frame:** a `CallBegin{A}` occurs while an
///    earlier `A` frame is still open, and that inner frame's matching
///    `CallEnd` reports `reverted: false`. A `nonReentrant` guard makes the
///    inner frame revert, so guarded contracts fail this test — they are
///    not flagged.
///
/// 2. **A storage write to `A` at or after the re-entry:** `A` mutates its
///    own storage after control has already re-entered it. This is the
///    checks-effects-interactions violation that makes the reentrancy
///    *exploitable* — the classic "update balance after the external call"
///    bug. A contract that writes its state *before* the external call
///    (CEI-correct) has no such late write and is not flagged, even if it
///    technically permits re-entry.
pub fn detect_reentrancy(trace: &[TraceEvent]) -> Option<ReentrancyReport> {
    // Track the addresses of currently-open frames as a stack, so we can
    // tell when a CallBegin re-enters an address that's already executing.
    let mut open_frames: Vec<[u8; 20]> = Vec::new();

    // For each trace index where a *successful-so-far* re-entrant frame
    // began, remember the re-entered address. We only confirm "successful"
    // at CallEnd, so we stash the candidate keyed by its position in the
    // open-frame stack and resolve it on pop.
    let mut reentrant_frame_addr: Vec<Option<[u8; 20]>> = Vec::new();

    // Addresses that were confirmed re-entered by a non-reverting inner
    // frame, paired with the trace index at which that re-entry began.
    let mut confirmed_reentries: Vec<([u8; 20], usize)> = Vec::new();

    for (idx, event) in trace.iter().enumerate() {
        match event {
            TraceEvent::CallBegin { address } => {
                let is_reentrant = open_frames.contains(address);
                open_frames.push(*address);
                reentrant_frame_addr.push(if is_reentrant { Some(*address) } else { None });
            }
            TraceEvent::CallEnd { reverted } => {
                open_frames.pop();
                if let Some(Some(addr)) = reentrant_frame_addr.pop() {
                    if !reverted {
                        confirmed_reentries.push((addr, idx));
                    }
                }
            }
            TraceEvent::StorageWrite { .. } => {}
        }
    }

    // Part 2: for each confirmed non-reverting re-entry, was there a storage
    // write to that same address at or after the re-entry began?
    for (addr, reentry_idx) in confirmed_reentries {
        let wrote_after = trace.iter().enumerate().any(|(i, ev)| {
            i >= reentry_idx
                && matches!(ev, TraceEvent::StorageWrite { address } if *address == addr)
        });
        if wrote_after {
            return Some(ReentrancyReport { address: addr });
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const VAULT: [u8; 20] = [0xC0; 20];
    const ATTACKER: [u8; 20] = [0xA7; 20];

    fn begin(addr: [u8; 20]) -> TraceEvent {
        TraceEvent::CallBegin { address: addr }
    }
    fn end(reverted: bool) -> TraceEvent {
        TraceEvent::CallEnd { reverted }
    }
    fn write(addr: [u8; 20]) -> TraceEvent {
        TraceEvent::StorageWrite { address: addr }
    }

    #[test]
    fn flags_classic_write_after_external_call_reentrancy() {
        // Vault.withdraw: calls attacker (sends ETH), attacker re-enters
        // withdraw, and the balance is only zeroed *after* the external
        // call — the textbook exploitable pattern.
        let trace = vec![
            begin(VAULT),    // outer withdraw
            begin(ATTACKER), //   external call to attacker
            begin(VAULT),    //     attacker re-enters withdraw
            write(VAULT),    //       inner withdraw zeroes balance
            end(false),      //     inner withdraw returns
            end(false),      //   attacker returns
            write(VAULT),    // outer withdraw zeroes balance (too late)
            end(false),      // outer withdraw returns
        ];
        assert_eq!(
            detect_reentrancy(&trace),
            Some(ReentrancyReport { address: VAULT })
        );
    }

    #[test]
    fn does_not_flag_a_guarded_contract_whose_reentry_reverts() {
        // nonReentrant guard: the re-entrant inner frame reverts, so it's
        // not a successful re-entry.
        let trace = vec![
            begin(VAULT),
            begin(ATTACKER),
            begin(VAULT), // attacker tries to re-enter
            end(true),    // guard reverts the inner frame
            end(false),   // attacker returns
            write(VAULT), // outer withdraw completes safely
            end(false),
        ];
        assert_eq!(detect_reentrancy(&trace), None);
    }

    #[test]
    fn does_not_flag_cei_correct_contract_even_if_reentry_succeeds() {
        // Checks-effects-interactions: state is written *before* the
        // external call, so re-entry sees updated state and does no further
        // damage — there is no storage write at or after the re-entry.
        let trace = vec![
            begin(VAULT),
            write(VAULT), // effect happens first
            begin(ATTACKER),
            begin(VAULT), // re-entry succeeds but finds nothing left to take
            end(false),
            end(false),
            end(false),
        ];
        assert_eq!(detect_reentrancy(&trace), None);
    }

    #[test]
    fn does_not_flag_a_plain_external_call_with_no_reentry() {
        let trace = vec![
            begin(VAULT),
            begin(ATTACKER), // external call, but attacker never calls back
            end(false),
            write(VAULT),
            end(false),
        ];
        assert_eq!(detect_reentrancy(&trace), None);
    }

    #[test]
    fn empty_trace_is_not_flagged() {
        assert_eq!(detect_reentrancy(&[]), None);
    }
}
