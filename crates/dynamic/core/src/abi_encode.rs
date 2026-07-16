//! Minimal static-type ABI encoding and edge-biased random argument
//! generation.
//!
//! Only fixed-width types are supported (see [`crate::backend::ParamKind`]);
//! each argument occupies exactly one 32-byte word, so encoding a call is
//! just `selector ++ word_0 ++ word_1 ++ ...` with no tail/offset handling.
//! That covers a large share of real functions (transfers, mints, approvals,
//! access-controlled setters) and keeps the encoder trivially correct.

use crate::backend::{FunctionSpec, ParamKind};
use rand::Rng;

/// Encode a fully-formed call: selector followed by each argument
/// right-aligned (big-endian) in its own 32-byte word, matching Solidity
/// ABI encoding for static types.
pub fn encode_call(function: &FunctionSpec, args: &[[u8; 32]]) -> Vec<u8> {
    debug_assert_eq!(function.inputs.len(), args.len());
    let mut out = Vec::with_capacity(4 + args.len() * 32);
    out.extend_from_slice(&function.selector);
    for word in args {
        out.extend_from_slice(word);
    }
    out
}

pub fn decode_uint256(return_data: &[u8]) -> [u8; 32] {
    let mut out = [0u8; 32];
    let len = return_data.len().min(32);
    // Return data is big-endian; right-align if the caller returned fewer
    // than 32 bytes (shouldn't happen for a real uint256 return, but don't
    // panic on malformed/truncated data from a buggy contract under test).
    out[32 - len..].copy_from_slice(&return_data[..len]);
    out
}

/// Edge-value-biased random argument generation: property fuzzers
/// (Foundry/Echidna included) find far more real bugs by weighting
/// generation toward boundary values (0, 1, max) than by sampling
/// uniformly, since off-by-one and overflow/underflow bugs cluster there.
pub fn random_args<R: Rng>(rng: &mut R, kinds: &[ParamKind], actors: &[[u8; 20]]) -> Vec<[u8; 32]> {
    kinds.iter().map(|k| random_word(rng, *k, actors)).collect()
}

fn random_word<R: Rng>(rng: &mut R, kind: ParamKind, actors: &[[u8; 20]]) -> [u8; 32] {
    match kind {
        ParamKind::Bool => {
            let mut word = [0u8; 32];
            word[31] = rng.gen_range(0..2);
            word
        }
        ParamKind::Address => {
            let mut word = [0u8; 32];
            if !actors.is_empty() {
                // Always draw from the known actor pool, never a fully
                // random address. This isn't just about hitting
                // cross-actor interactions more often — invariants like
                // ConservationInvariant sum balances over exactly this
                // pool, so an address argument that lands outside it would
                // make a *correct* contract look broken (funds "missing"
                // from the sum purely because the fuzzer sent them
                // somewhere it never checks). Real invariant fuzzers
                // (Echidna, Foundry) close the address universe the same
                // way by default for this reason.
                let addr = actors[rng.gen_range(0..actors.len())];
                word[12..32].copy_from_slice(&addr);
            } else {
                rng.fill(&mut word[12..32]);
            }
            word
        }
        ParamKind::Bytes32 => {
            let mut word = [0u8; 32];
            rng.fill(&mut word);
            word
        }
        ParamKind::Uint256 => random_uint256(rng),
    }
}

fn random_uint256<R: Rng>(rng: &mut R) -> [u8; 32] {
    let roll: u8 = rng.gen_range(0..10);
    match roll {
        0 => [0u8; 32],                        // zero
        1 => {
            let mut w = [0u8; 32];
            w[31] = 1;
            w
        } // one
        2 => [0xFFu8; 32],                      // u256::MAX
        3 => {
            let mut w = [0u8; 32];
            w[0] = 0x80;
            w
        } // 2^255 (sign-bit-adjacent, common overflow boundary)
        4..=6 => {
            // small value, the common case for amounts/counters
            let mut w = [0u8; 32];
            w[28..32].copy_from_slice(&rng.gen::<u32>().to_be_bytes());
            w
        }
        _ => {
            // fully random large value
            let mut w = [0u8; 32];
            rng.fill(&mut w);
            w
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::FunctionSpec;
    use rand::SeedableRng;

    #[test]
    fn encode_call_lays_out_selector_then_words() {
        let f = FunctionSpec::new("f", [0xAA, 0xBB, 0xCC, 0xDD], vec![ParamKind::Uint256], true);
        let mut word = [0u8; 32];
        word[31] = 7;
        let encoded = encode_call(&f, &[word]);
        assert_eq!(&encoded[0..4], &[0xAA, 0xBB, 0xCC, 0xDD]);
        assert_eq!(encoded.len(), 36);
        assert_eq!(encoded[35], 7);
    }

    #[test]
    fn decode_uint256_right_aligns_short_return_data() {
        let decoded = decode_uint256(&[5]);
        assert_eq!(decoded[31], 5);
        assert_eq!(&decoded[..31], &[0u8; 31]);
    }

    #[test]
    fn random_args_covers_edge_values_over_many_samples() {
        let mut rng = rand::rngs::SmallRng::seed_from_u64(42);
        let mut saw_zero = false;
        let mut saw_max = false;
        for _ in 0..200 {
            let word = random_uint256(&mut rng);
            if word == [0u8; 32] {
                saw_zero = true;
            }
            if word == [0xFFu8; 32] {
                saw_max = true;
            }
        }
        assert!(saw_zero, "edge-biased generator should hit zero over 200 samples");
        assert!(saw_max, "edge-biased generator should hit u256::MAX over 200 samples");
    }

    #[test]
    fn random_address_always_draws_from_known_actor_pool() {
        let mut rng = rand::rngs::SmallRng::seed_from_u64(1);
        let actors = [[0x11u8; 20], [0x22u8; 20]];
        for _ in 0..100 {
            let word = random_word(&mut rng, ParamKind::Address, &actors);
            let addr: [u8; 20] = word[12..32].try_into().unwrap();
            assert!(
                actors.contains(&addr),
                "address arguments must stay within the closed actor pool or ConservationInvariant-style checks get spurious failures"
            );
        }
    }
}
