//! The Solana execution backend abstraction.
//!
//! Mirrors the EVM crate's `ExecutionBackend`, but the state surface is
//! account-shaped: invariants read an account's lamports and raw data (then
//! decode), rather than calling `balanceOf`. One implementation is the
//! in-memory `MockSvm` (in `testing.rs`) that proves the engine logic without
//! a VM; the real implementation (feature `litesvm-backend`) runs actual
//! Solana bytecode.

use crate::model::{Instruction, IxOutcome, Pubkey};

/// "A deployed program plus the account state it operates on, that I can
/// drive with instructions and snapshot/restore." Object-safe so the fuzzer
/// can hold `&mut dyn SvmBackend`.
pub trait SvmBackend {
    /// Execute one instruction, signed by `signers`, mutating account state.
    fn execute(&mut self, ix: &Instruction, signers: &[Pubkey]) -> IxOutcome;

    /// Lamport balance of an account (0 if it doesn't exist).
    fn lamports(&self, pubkey: &Pubkey) -> u64;

    /// Raw account data (empty if the account doesn't exist). Invariants
    /// decode program-specific layouts from this.
    fn account_data(&self, pubkey: &Pubkey) -> Vec<u8>;

    /// The program that owns an account (`None` if the account doesn't exist).
    fn account_owner(&self, pubkey: &Pubkey) -> Option<Pubkey>;

    /// Snapshot current state; returns an id the shrinker restores with
    /// [`SvmBackend::revert_to`].
    fn snapshot(&mut self) -> u64;

    /// Restore state to a snapshot.
    fn revert_to(&mut self, snapshot: u64);
}

/// Decode a little-endian `u64` from the first 8 bytes at `offset` of account
/// data (Solana/Borsh scalars are little-endian). Returns 0 if the slice is
/// too short — invariants must not panic on a malformed/uninitialized
/// account, exactly the defensive posture the EVM decoders take.
pub fn decode_u64_le(data: &[u8], offset: usize) -> u64 {
    let end = offset.saturating_add(8);
    if data.len() < end {
        return 0;
    }
    let mut buf = [0u8; 8];
    buf.copy_from_slice(&data[offset..end]);
    u64::from_le_bytes(buf)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_u64_le_reads_little_endian_and_is_bounds_safe() {
        let data = vec![0u8; 8].into_iter().chain([0xFF, 0, 0, 0, 0, 0, 0, 0]).collect::<Vec<_>>();
        assert_eq!(decode_u64_le(&data, 8), 255);
        // offset past the end must not panic
        assert_eq!(decode_u64_le(&data, 100), 0);
        // partial tail must not panic
        assert_eq!(decode_u64_le(&[1, 2, 3], 0), 0);
    }
}
