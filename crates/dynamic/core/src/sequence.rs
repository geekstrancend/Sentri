//! Call-sequence generation and execution.
//!
//! A "sequence" is the fuzzing engine's unit of work: an ordered list of
//! calls into the contract under test. Invariants are re-checked after
//! every call, not just at the end, so a violation is pinned to the exact
//! step that caused it — the shrinker then reduces the sequence around
//! that step.

use crate::abi_encode::{encode_call, random_args};
use crate::backend::{EncodedCall, ExecutionBackend, FunctionSpec};
use crate::invariant::Invariant;
use rand::Rng;

#[derive(Debug, Clone, Default)]
pub struct CallSequence(pub Vec<EncodedCall>);

impl CallSequence {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// A confirmed invariant violation, with the minimal-so-far sequence that
/// reproduces it and which invariant/step caught it.
#[derive(Debug, Clone)]
pub struct Violation {
    pub invariant_name: String,
    pub message: String,
    pub failing_step: usize,
    pub sequence: CallSequence,
}

/// Generate a random sequence of calls against `functions`, drawing callers
/// and address arguments from `actors`. Only functions with `mutates_state`
/// are used to drive state transitions — view functions are called by
/// invariants directly, not injected into the sequence under test.
pub fn generate_random_sequence<R: Rng>(
    rng: &mut R,
    functions: &[FunctionSpec],
    actors: &[[u8; 20]],
    depth: usize,
) -> CallSequence {
    let mutators: Vec<&FunctionSpec> = functions.iter().filter(|f| f.mutates_state).collect();
    if mutators.is_empty() || actors.is_empty() {
        return CallSequence::default();
    }
    let mut calls = Vec::with_capacity(depth);
    for _ in 0..depth {
        let function = mutators[rng.gen_range(0..mutators.len())].clone();
        let args = random_args(rng, &function.inputs, actors);
        let calldata = encode_call(&function, &args);
        let caller = actors[rng.gen_range(0..actors.len())];
        let value = if function.payable && rng.gen_bool(0.3) {
            rng.gen_range(0..=1_000_000u128)
        } else {
            0
        };
        calls.push(EncodedCall {
            function,
            calldata,
            caller,
            value,
        });
    }
    CallSequence(calls)
}

/// Execute `sequence` against `backend`, checking every invariant after
/// every call. Returns the first violation found, pinned to the step and
/// sequence-prefix that triggered it.
pub fn run_sequence(
    backend: &mut dyn ExecutionBackend,
    sequence: &CallSequence,
    invariants: &[Box<dyn Invariant>],
) -> Option<Violation> {
    // Each call to run_sequence is expected to start from the same baseline
    // (genesis) state — callers revert the backend to that snapshot first.
    // Invariants that track "previous value" across steps (e.g. monotonic
    // checks) must forget whatever they remembered from the last time this
    // function ran, or a later sequence would be compared against a stale
    // value from an unrelated earlier run.
    for invariant in invariants {
        invariant.reset(backend);
    }
    for (step_idx, call) in sequence.0.iter().enumerate() {
        backend.call(call);
        for invariant in invariants {
            if let Some(message) = invariant.check(backend, call) {
                return Some(Violation {
                    invariant_name: invariant.name().to_string(),
                    message,
                    failing_step: step_idx,
                    sequence: CallSequence(sequence.0[..=step_idx].to_vec()),
                });
            }
        }
    }
    None
}
