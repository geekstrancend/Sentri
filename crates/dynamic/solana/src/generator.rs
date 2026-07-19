//! Random instruction generation from an [`InstructionSpec`] surface.
//!
//! The account-model twin of the EVM argument generator: it fills each
//! instruction's discriminator, edge-biased scalar args, and — the part with
//! no EVM analogue — its account list, resolving each role to a concrete
//! account from the pool. It also returns the signer set the backend needs.

use crate::model::{AccountMeta, AccountRole, ArgKind, Instruction, InstructionSpec, Pubkey};
use rand::Rng;

/// The account universe the fuzzer draws from: signer authorities and the
/// writable/readonly state accounts an instruction can touch. Closed on
/// purpose, for the same reason the EVM fuzzer closes its address set —
/// conservation-style invariants sum over exactly these accounts.
#[derive(Debug, Clone)]
pub struct AccountPool {
    pub signers: Vec<Pubkey>,
    pub writable: Vec<Pubkey>,
    pub readonly: Vec<Pubkey>,
}

/// Encode instruction data: discriminator followed by each scalar arg in
/// Solana/Borsh little-endian layout.
pub fn encode_ix_data<R: Rng>(rng: &mut R, spec: &InstructionSpec, pool: &AccountPool) -> Vec<u8> {
    let mut data = spec.discriminator.clone();
    for kind in &spec.args {
        match kind {
            ArgKind::U64 => data.extend_from_slice(&random_u64(rng).to_le_bytes()),
            ArgKind::U8 => data.push(rng.gen()),
            ArgKind::Bool => data.push(rng.gen_range(0..2)),
            ArgKind::Pubkey => data.extend_from_slice(&pick_pubkey(rng, pool)),
        }
    }
    data
}

/// Edge-value-biased `u64` — boundary values (0, 1, u64::MAX) find far more
/// arithmetic bugs than uniform sampling.
fn random_u64<R: Rng>(rng: &mut R) -> u64 {
    match rng.gen_range(0..8u8) {
        0 => 0,
        1 => 1,
        2 => u64::MAX,
        3 => 1 << 63,
        4..=5 => rng.gen::<u32>() as u64, // small, the common case
        _ => rng.gen(),
    }
}

fn pick_pubkey<R: Rng>(rng: &mut R, pool: &AccountPool) -> Pubkey {
    // Draw pubkey-typed args from the writable+signer accounts so they refer
    // to real state the invariants track, not addresses nothing else touches.
    let mut choices: Vec<Pubkey> = Vec::new();
    choices.extend_from_slice(&pool.writable);
    choices.extend_from_slice(&pool.signers);
    if choices.is_empty() {
        let mut p = [0u8; 32];
        rng.fill(&mut p);
        return p;
    }
    choices[rng.gen_range(0..choices.len())]
}

fn pick_role<R: Rng>(rng: &mut R, role: AccountRole, pool: &AccountPool) -> AccountMeta {
    match role {
        AccountRole::Signer => {
            let pk = pick(rng, &pool.signers);
            AccountMeta::signer_writable(pk)
        }
        AccountRole::Writable => {
            let pk = pick(rng, &pool.writable);
            AccountMeta::writable(pk)
        }
        AccountRole::Readonly => {
            let pk = pick(rng, &pool.readonly);
            AccountMeta::readonly(pk)
        }
    }
}

fn pick<R: Rng>(rng: &mut R, xs: &[Pubkey]) -> Pubkey {
    if xs.is_empty() {
        let mut p = [0u8; 32];
        rng.fill(&mut p);
        p
    } else {
        xs[rng.gen_range(0..xs.len())]
    }
}

/// Build one random instruction for `spec`, plus the set of signer pubkeys the
/// transaction must carry (derived from the instruction's signer accounts).
pub fn random_instruction<R: Rng>(
    rng: &mut R,
    program_id: Pubkey,
    spec: &InstructionSpec,
    pool: &AccountPool,
) -> (Instruction, Vec<Pubkey>) {
    let accounts: Vec<AccountMeta> = spec
        .account_roles
        .iter()
        .map(|r| pick_role(rng, *r, pool))
        .collect();
    let signers: Vec<Pubkey> = accounts
        .iter()
        .filter(|a| a.is_signer)
        .map(|a| a.pubkey)
        .collect();
    let data = encode_ix_data(rng, spec, pool);
    (
        Instruction {
            program_id,
            accounts,
            data,
        },
        signers,
    )
}
