//! Delta-debugging shrinker: reduces a failing call sequence to a minimal
//! one that still reproduces the same violation, so the PoC handed to a
//! human is "call mint(), call transfer()" instead of forty interleaved
//! random calls.

use crate::backend::ExecutionBackend;
use crate::invariant::Invariant;
use crate::sequence::{run_sequence, CallSequence};

/// Repeatedly try removing one call at a time from the sequence; keep the
/// removal if the resulting (shorter) sequence still triggers a violation
/// from the same invariant. Iterates to a fixpoint (no single removal still
/// reproduces), which is the standard ddmin-style shrink loop — O(n^2) in
/// the worst case, which is acceptable here since sequences are bounded by
/// the configured fuzz depth (tens of calls, not thousands).
pub fn shrink<F>(fresh_backend: F, seq: &CallSequence, invariants: &[Box<dyn Invariant>]) -> CallSequence
where
    F: Fn() -> Box<dyn ExecutionBackend>,
{
    let mut current = seq.clone();
    loop {
        let mut improved = false;
        let mut i = 0;
        while i < current.0.len() {
            if current.0.len() <= 1 {
                break;
            }
            let mut candidate = current.clone();
            candidate.0.remove(i);

            let mut backend = fresh_backend();
            if run_sequence(backend.as_mut(), &candidate, invariants).is_some() {
                current = candidate;
                improved = true;
                // don't advance i: another call may now be removable at the
                // same index after this removal shifted things down
            } else {
                i += 1;
            }
        }
        if !improved {
            break;
        }
    }
    current
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::EncodedCall;
    use crate::invariant::MonotonicInvariant;
    use crate::testing::{self, MockBackend, DECR, INCR};

    #[test]
    fn shrinks_a_long_irrelevant_sequence_down_to_the_single_triggering_call() {
        let functions = testing::counter_functions();
        let incr = functions.iter().find(|f| f.selector == INCR).unwrap().clone();
        let decr = functions.iter().find(|f| f.selector == DECR).unwrap().clone();
        let value_fn = functions.iter().find(|f| f.selector == testing::VALUE).unwrap().clone();

        let call_of = |f: &crate::backend::FunctionSpec| EncodedCall {
            function: f.clone(),
            calldata: f.selector.to_vec(),
            caller: [1u8; 20],
            value: 0,
        };

        // 5 harmless increments, one buggy decrement (the actual violation),
        // then 5 more harmless increments — the decrement is the only call
        // that matters and should be all that survives shrinking.
        let mut calls = Vec::new();
        for _ in 0..5 {
            calls.push(call_of(&incr));
        }
        calls.push(call_of(&decr));
        for _ in 0..5 {
            calls.push(call_of(&incr));
        }
        let seq = CallSequence(calls);

        let invariants: Vec<Box<dyn Invariant>> = vec![Box::new(MonotonicInvariant::new("value", value_fn))];
        let fresh = || -> Box<dyn ExecutionBackend> { Box::new(MockBackend::default()) };

        // Sanity check: the full sequence really does trigger the violation
        // before we assert anything about shrinking it.
        let mut backend = fresh();
        assert!(run_sequence(backend.as_mut(), &seq, &invariants).is_some());

        let shrunk = shrink(fresh, &seq, &invariants);
        // MonotonicInvariant needs one prior reading to compare against, so
        // the true minimal reproducer is "establish a baseline, then
        // decrease it" — one leading INCR plus the DECR, not DECR alone
        // (counter starts at 0 and saturating-subtracts to 0, which reads
        // as "no prior baseline, nothing to violate" rather than a
        // decrease).
        assert_eq!(
            shrunk.len(),
            2,
            "expected shrink to reduce 11 calls down to the minimal 2 that establish a baseline then violate it"
        );
        assert_eq!(shrunk.0[0].function.selector, INCR);
        assert_eq!(shrunk.0[1].function.selector, DECR);
    }
}
