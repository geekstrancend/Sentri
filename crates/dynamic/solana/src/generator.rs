//! Random instruction generation from an [`InstructionSpec`] surface.
//!
//! The account-model twin of the EVM argument generator: it fills each
//! instruction's discriminator, edge-biased scalar args, and — the part with
//! no EVM analogue — its account list, resolving each role to a concrete
//! account from the pool. It also returns the signer set the backend needs.

use crate::model::{AccountMeta, AccountRole, ArgKind, Instruction, InstructionSpec, Pubkey};
use rand::Rng;
use std::collections::BTreeMap;

/// The account universe the fuzzer draws from: signer authorities and the
/// writable/readonly state accounts an instruction can touch. Closed on
/// purpose, for the same reason the EVM fuzzer closes its address set —
/// conservation-style invariants sum over exactly these accounts.
#[derive(Debug, Clone, Default)]
pub struct AccountPool {
    pub signers: Vec<Pubkey>,
    pub writable: Vec<Pubkey>,
    pub readonly: Vec<Pubkey>,
    /// Account positions bound to a fixed account by IDL name (`mint` → the
    /// real mint). Any position whose name appears here always resolves to
    /// that account instead of a random draw; everything else stays fuzzed.
    ///
    /// This is what keeps singleton accounts (a mint, a global config, a
    /// vault authority) from being aliased to an arbitrary writable account,
    /// which would break conservation-style invariants on correct programs.
    pub pinned: BTreeMap<String, Pubkey>,
}

impl AccountPool {
    pub fn new(signers: Vec<Pubkey>, writable: Vec<Pubkey>, readonly: Vec<Pubkey>) -> Self {
        Self {
            signers,
            writable,
            readonly,
            pinned: BTreeMap::new(),
        }
    }

    /// Pin the account position named `name` to `pubkey`.
    pub fn pin(mut self, name: &str, pubkey: Pubkey) -> Self {
        self.pinned.insert(name.to_string(), pubkey);
        self
    }
}

/// Encode instruction data: discriminator followed by each scalar arg in
/// Solana/Borsh little-endian layout.
pub fn encode_ix_data<R: Rng>(rng: &mut R, spec: &InstructionSpec, pool: &AccountPool) -> Vec<u8> {
    let mut data = spec.discriminator.clone();
    for kind in &spec.args {
        // Every arm must emit exactly `kind.width()` little-endian bytes —
        // Borsh is positional, so a short write corrupts every later argument.
        match kind {
            ArgKind::U8 => data.push(random_u64(rng) as u8),
            ArgKind::U16 => data.extend_from_slice(&(random_u64(rng) as u16).to_le_bytes()),
            ArgKind::U32 => data.extend_from_slice(&(random_u64(rng) as u32).to_le_bytes()),
            ArgKind::U64 => data.extend_from_slice(&random_u64(rng).to_le_bytes()),
            ArgKind::U128 => data.extend_from_slice(&random_u128(rng).to_le_bytes()),
            ArgKind::I64 => data.extend_from_slice(&(random_u64(rng) as i64).to_le_bytes()),
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

/// Edge-biased `u128`, same rationale as [`random_u64`]: saturating and
/// overflow boundaries are where the bugs are.
fn random_u128<R: Rng>(rng: &mut R) -> u128 {
    match rng.gen_range(0..8u8) {
        0 => 0,
        1 => 1,
        2 => u128::MAX,
        3 => u64::MAX as u128,
        4 => 1u128 << 127,
        _ => random_u64(rng) as u128,
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

/// Build the `AccountMeta` for `role` at a known pubkey.
fn meta_for_role(role: AccountRole, pubkey: Pubkey) -> AccountMeta {
    match role {
        AccountRole::Signer => AccountMeta::signer_writable(pubkey),
        AccountRole::Writable => AccountMeta::writable(pubkey),
        AccountRole::Readonly => AccountMeta::readonly(pubkey),
    }
}

fn pick_role<R: Rng>(rng: &mut R, role: AccountRole, pool: &AccountPool) -> AccountMeta {
    let candidates = match role {
        AccountRole::Signer => &pool.signers,
        AccountRole::Writable => &pool.writable,
        AccountRole::Readonly => &pool.readonly,
    };
    meta_for_role(role, pick(rng, candidates))
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
        .enumerate()
        .map(|(i, role)| {
            // A pinned name always wins over a random draw, but keeps the
            // role's signer/writable flags so the instruction stays well-formed.
            match spec.account_name(i).and_then(|n| pool.pinned.get(n)) {
                Some(pk) => meta_for_role(*role, *pk),
                None => pick_role(rng, *role, pool),
            }
        })
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

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::SmallRng;
    use rand::SeedableRng;

    const AUTH: Pubkey = [0xA0; 32];
    const ACCT_A: Pubkey = [0x01; 32];
    const ACCT_B: Pubkey = [0x02; 32];
    const MINT: Pubkey = [0x30; 32];

    fn spec() -> InstructionSpec {
        InstructionSpec::new(
            "mint_to",
            vec![0x10],
            vec![ArgKind::U64],
            vec![AccountRole::Signer, AccountRole::Writable, AccountRole::Writable],
            true,
        )
        .with_account_names(vec![
            "authority".into(),
            "token_account".into(),
            "mint".into(),
        ])
    }

    #[test]
    fn pinned_position_always_resolves_to_the_pinned_account() {
        // Without pinning the `mint` slot is drawn from the writable pool and
        // can alias a token account — the false-positive source that pinning
        // exists to remove.
        let pool = AccountPool::new(vec![AUTH], vec![ACCT_A, ACCT_B, MINT], vec![]).pin("mint", MINT);
        let mut rng = SmallRng::seed_from_u64(42);

        let mut saw_varied_token_account = false;
        for _ in 0..200 {
            let (ix, signers) = random_instruction(&mut rng, [0x70; 32], &spec(), &pool);
            // position 2 is `mint`, pinned
            assert_eq!(ix.accounts[2].pubkey, MINT, "pinned mint slot must never vary");
            assert!(ix.accounts[2].is_writable, "pinning must keep the role's flags");
            // position 0 is the signer authority
            assert_eq!(ix.accounts[0].pubkey, AUTH);
            assert!(ix.accounts[0].is_signer);
            assert_eq!(signers, vec![AUTH]);
            if ix.accounts[1].pubkey != MINT {
                saw_varied_token_account = true;
            }
        }
        assert!(
            saw_varied_token_account,
            "unpinned positions must still be fuzzed"
        );
    }

    #[test]
    fn unpinned_mint_slot_can_alias_another_account() {
        // Documents precisely why pinning is needed: with no pin, the `mint`
        // position does get drawn as a non-mint account.
        let pool = AccountPool::new(vec![AUTH], vec![ACCT_A, ACCT_B, MINT], vec![]);
        let mut rng = SmallRng::seed_from_u64(7);
        let aliased = (0..200).any(|_| {
            let (ix, _) = random_instruction(&mut rng, [0x70; 32], &spec(), &pool);
            ix.accounts[2].pubkey != MINT
        });
        assert!(aliased);
    }

    #[test]
    fn encoded_data_width_matches_the_declared_arg_widths() {
        // Borsh is positional: a wrong width corrupts every later argument.
        let all = vec![
            ArgKind::U8,
            ArgKind::U16,
            ArgKind::U32,
            ArgKind::U64,
            ArgKind::U128,
            ArgKind::I64,
            ArgKind::Bool,
            ArgKind::Pubkey,
        ];
        let expected: usize = 1 + all.iter().map(|k| k.width()).sum::<usize>();
        let s = InstructionSpec::new("wide", vec![0xAB], all, vec![], true);
        let pool = AccountPool::new(vec![AUTH], vec![ACCT_A], vec![]);
        let mut rng = SmallRng::seed_from_u64(1);
        for _ in 0..50 {
            assert_eq!(encode_ix_data(&mut rng, &s, &pool).len(), expected);
        }
    }
}
