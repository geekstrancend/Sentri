//! The Solana fuzzing loop: generate instruction sequences, execute each
//! against a fresh backend, check invariants after every instruction, and on
//! a violation shrink the sequence to a minimal reproduction. Same search +
//! delta-debug shape as the EVM engine, expressed over the account model.

use crate::backend::SvmBackend;
use crate::generator::{random_instruction, AccountPool};
use crate::invariant::{CheckContext, SolanaInvariant};
use crate::model::{Instruction, Pubkey};
use rand::rngs::SmallRng;
use rand::SeedableRng;

/// One step of a sequence: the instruction plus the signer set it was sent
/// with (Solana needs both to reproduce).
pub type Step = (Instruction, Vec<Pubkey>);

/// A confirmed invariant violation with the minimal reproducing sequence.
#[derive(Debug, Clone)]
pub struct Violation {
    pub invariant_name: String,
    pub message: String,
    pub failing_step: usize,
    pub sequence: Vec<Step>,
}

/// Search parameters (reproducible via `seed`).
#[derive(Debug, Clone)]
pub struct FuzzConfig {
    pub seed: u64,
    pub max_runs: usize,
    pub sequence_depth: usize,
}

impl Default for FuzzConfig {
    fn default() -> Self {
        Self {
            seed: 0,
            max_runs: 500,
            sequence_depth: 12,
        }
    }
}

/// Execute `sequence` against `backend`, checking every invariant after every
/// instruction. Returns the first violation, pinned to the failing step.
pub fn run_sequence(
    backend: &mut dyn SvmBackend,
    sequence: &[Step],
    invariants: &[Box<dyn SolanaInvariant>],
) -> Option<Violation> {
    for inv in invariants {
        inv.reset(backend);
    }
    for (idx, (ix, signers)) in sequence.iter().enumerate() {
        backend.execute(ix, signers);
        let ctx = CheckContext {
            last_ix: ix,
            signers,
        };
        for inv in invariants {
            if let Some(message) = inv.check(backend, &ctx) {
                return Some(Violation {
                    invariant_name: inv.name().to_string(),
                    message,
                    failing_step: idx,
                    sequence: sequence[..=idx].to_vec(),
                });
            }
        }
    }
    None
}

/// Delta-debug: drop one step at a time; keep the removal if the shorter
/// sequence still reproduces. Iterates to a fixpoint.
pub fn shrink<F>(
    fresh_backend: F,
    seq: &[Step],
    invariants: &[Box<dyn SolanaInvariant>],
) -> Vec<Step>
where
    F: Fn() -> Box<dyn SvmBackend>,
{
    let mut current = seq.to_vec();
    loop {
        let mut improved = false;
        let mut i = 0;
        while i < current.len() {
            if current.len() <= 1 {
                break;
            }
            let mut candidate = current.clone();
            candidate.remove(i);
            let mut backend = fresh_backend();
            if run_sequence(backend.as_mut(), &candidate, invariants).is_some() {
                current = candidate;
                improved = true;
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

/// Run the search: generate random instruction sequences from `specs`, execute
/// each against a fresh backend, and on the first violation shrink it to a
/// minimal reproduction.
pub fn fuzz<F>(
    fresh_backend: F,
    program_id: Pubkey,
    specs: &[crate::model::InstructionSpec],
    pool: &AccountPool,
    invariants: Vec<Box<dyn SolanaInvariant>>,
    config: FuzzConfig,
) -> Option<Violation>
where
    F: Fn() -> Box<dyn SvmBackend>,
{
    let mutators: Vec<&crate::model::InstructionSpec> =
        specs.iter().filter(|s| s.mutates_state).collect();
    if mutators.is_empty() {
        return None;
    }
    let mut rng = SmallRng::seed_from_u64(config.seed);

    for _ in 0..config.max_runs {
        let mut sequence: Vec<Step> = Vec::with_capacity(config.sequence_depth);
        for _ in 0..config.sequence_depth {
            let spec = mutators[gen_index(&mut rng, mutators.len())];
            sequence.push(random_instruction(&mut rng, program_id, spec, pool));
        }

        let mut backend = fresh_backend();
        if run_sequence(backend.as_mut(), &sequence, &invariants).is_none() {
            continue;
        }

        let shrunk = shrink(&fresh_backend, &sequence, &invariants);
        let mut replay = fresh_backend();
        return run_sequence(replay.as_mut(), &shrunk, &invariants).or_else(|| {
            // Extremely unlikely (shrink only keeps reproducing sequences),
            // but never lose a real violation to a replay miss.
            let mut b = fresh_backend();
            run_sequence(b.as_mut(), &sequence, &invariants)
        });
    }
    None
}

fn gen_index(rng: &mut SmallRng, n: usize) -> usize {
    use rand::Rng;
    rng.gen_range(0..n)
}

/// Render a violation as a human-readable proof-of-concept.
pub fn format_poc(v: &Violation) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "Invariant violated: {}\n{}\n\nReproduction ({} instruction{}):\n",
        v.invariant_name,
        v.message,
        v.sequence.len(),
        if v.sequence.len() == 1 { "" } else { "s" }
    ));
    for (i, (ix, signers)) in v.sequence.iter().enumerate() {
        let disc = ix
            .data
            .iter()
            .take(1)
            .map(|b| format!("{:02x}", b))
            .collect::<String>();
        out.push_str(&format!(
            "  {}. program {} · ix[0x{}] · {} accounts · {} signer(s)\n",
            i + 1,
            hex8(&ix.program_id),
            disc,
            ix.accounts.len(),
            signers.len(),
        ));
    }
    out.push_str(&format!("\nFailing step: #{}\n", v.failing_step + 1));
    out
}

fn hex8(p: &Pubkey) -> String {
    p.iter().take(8).map(|b| format!("{:02x}", b)).collect()
}
