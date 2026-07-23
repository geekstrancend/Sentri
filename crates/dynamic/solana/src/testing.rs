//! A pure-Rust, in-memory `SvmBackend` simulating a tiny SPL-token-like
//! program. It exists to prove the Solana fuzzing engine (generation,
//! execution, invariant checking, shrinking) is correct without a real VM —
//! exactly how the EVM crate proved its engine with a mock before wiring
//! revm. The real backend (feature `litesvm-backend`) runs actual bytecode.

use crate::backend::{decode_u64_le, SvmBackend};
use crate::model::{AccountRole, ArgKind, Instruction, InstructionSpec, IxOutcome, Pubkey};
use std::collections::HashMap;

// ── Program + account fixtures ──
pub const TOKEN_PROGRAM: Pubkey = [0x70; 32];
pub const MINT: Pubkey = [0x30; 32];
pub const AUTHORITY: Pubkey = [0xA0; 32];
pub const TOKEN_ACCT_1: Pubkey = [0x01; 32];
pub const TOKEN_ACCT_2: Pubkey = [0x02; 32];
pub const TOKEN_ACCT_3: Pubkey = [0x03; 32];

// ── Instruction discriminators ──
pub const IX_MINT: u8 = 0x10;
/// Buggy: credits a token account's amount without updating mint supply —
/// the "mint out of thin air" that breaks conservation.
pub const IX_AIRDROP: u8 = 0x11;
/// Buggy: reassigns an account's owner with no authority check.
pub const IX_REASSIGN_UNCHECKED: u8 = 0x20;

// Account-data layout used by this mock: `amount`/`supply` at offset 0.
pub const AMOUNT_OFFSET: usize = 0;
pub const SUPPLY_OFFSET: usize = 0;

/// The instruction surface exposing the *correct* mint alongside the buggy
/// airdrop — the fuzzer will find that airdrop breaks conservation.
pub fn token_functions_vulnerable() -> Vec<InstructionSpec> {
    vec![
        InstructionSpec::new(
            "mint",
            vec![IX_MINT],
            vec![ArgKind::U64],
            vec![
                AccountRole::Signer,
                AccountRole::Writable,
                AccountRole::Writable,
            ],
            true,
        ),
        InstructionSpec::new(
            "airdrop",
            vec![IX_AIRDROP],
            vec![ArgKind::U64],
            vec![AccountRole::Signer, AccountRole::Writable],
            true,
        ),
    ]
}

/// Only the correct mint — conservation must hold, proving no false positive.
pub fn token_functions_safe() -> Vec<InstructionSpec> {
    vec![InstructionSpec::new(
        "mint",
        vec![IX_MINT],
        vec![ArgKind::U64],
        vec![
            AccountRole::Signer,
            AccountRole::Writable,
            AccountRole::Writable,
        ],
        true,
    )]
}

#[derive(Clone, Default)]
struct Account {
    lamports: u64,
    data: Vec<u8>,
    owner: Pubkey,
}

#[derive(Clone)]
struct MockState {
    accounts: HashMap<Pubkey, Account>,
}

impl Default for MockState {
    fn default() -> Self {
        let mut accounts = HashMap::new();
        // mint (supply 0) and three token accounts (amount 0), all owned by
        // the token program.
        for pk in [MINT, TOKEN_ACCT_1, TOKEN_ACCT_2, TOKEN_ACCT_3] {
            accounts.insert(
                pk,
                Account {
                    lamports: 1_000_000,
                    data: vec![0u8; 8],
                    owner: TOKEN_PROGRAM,
                },
            );
        }
        Self { accounts }
    }
}

#[derive(Default)]
pub struct MockSvm {
    state: MockState,
    snapshots: Vec<MockState>,
}

fn read_u64(acct: &Account) -> u64 {
    decode_u64_le(&acct.data, 0)
}
fn write_u64(acct: &mut Account, v: u64) {
    if acct.data.len() < 8 {
        acct.data.resize(8, 0);
    }
    acct.data[0..8].copy_from_slice(&v.to_le_bytes());
}
fn arg_u64(data: &[u8]) -> u64 {
    // discriminator is 1 byte; the u64 arg follows.
    decode_u64_le(data, 1)
}

/// The canonical token accounts of this mint. Real SPL `mint_to` checks the
/// destination is a token account *of the given mint*; the mock mirrors that
/// so a correct mint can't be tricked into writing the mint's supply slot into
/// a random account (or vice-versa).
fn is_token_account(pk: &Pubkey) -> bool {
    matches!(*pk, TOKEN_ACCT_1 | TOKEN_ACCT_2 | TOKEN_ACCT_3)
}
fn is_mint(pk: &Pubkey) -> bool {
    *pk == MINT
}

impl MockSvm {
    fn revert() -> IxOutcome {
        IxOutcome {
            reverted: true,
            logs: vec!["reverted".into()],
        }
    }
    fn ok(log: &str) -> IxOutcome {
        IxOutcome {
            reverted: false,
            logs: vec![log.into()],
        }
    }
}

impl SvmBackend for MockSvm {
    fn execute(&mut self, ix: &Instruction, signers: &[Pubkey]) -> IxOutcome {
        let disc = *ix.data.first().unwrap_or(&0xFF);
        match disc {
            IX_MINT => {
                // accounts: [signer authority, token_account, mint]
                if signers.is_empty() || ix.accounts.len() < 3 {
                    return Self::revert();
                }
                let amount = arg_u64(&ix.data);
                let ta = ix.accounts[1].pubkey;
                let mint = ix.accounts[2].pubkey;
                // SPL-style identity checks: destination must be a token
                // account, the mint account must be the mint. Without these a
                // correct mint could still corrupt conservation.
                if !is_token_account(&ta) || !is_mint(&mint) {
                    return Self::revert();
                }
                // checked arithmetic → revert on overflow (real 0.8-style safety)
                let (new_ta, new_supply) = {
                    let ta_amt = self.state.accounts.get(&ta).map(read_u64).unwrap_or(0);
                    let supply = self.state.accounts.get(&mint).map(read_u64).unwrap_or(0);
                    match (ta_amt.checked_add(amount), supply.checked_add(amount)) {
                        (Some(a), Some(s)) => (a, s),
                        _ => return Self::revert(),
                    }
                };
                if let Some(a) = self.state.accounts.get_mut(&ta) {
                    write_u64(a, new_ta);
                }
                if let Some(m) = self.state.accounts.get_mut(&mint) {
                    write_u64(m, new_supply);
                }
                Self::ok("mint")
            }
            IX_AIRDROP => {
                // accounts: [signer, token_account] — BUG: no mint update.
                if ix.accounts.len() < 2 {
                    return Self::revert();
                }
                let amount = arg_u64(&ix.data);
                let ta = ix.accounts[1].pubkey;
                if !is_token_account(&ta) {
                    return Self::revert();
                }
                let new_ta = {
                    let ta_amt = self.state.accounts.get(&ta).map(read_u64).unwrap_or(0);
                    match ta_amt.checked_add(amount) {
                        Some(a) => a,
                        None => return Self::revert(),
                    }
                };
                if let Some(a) = self.state.accounts.get_mut(&ta) {
                    write_u64(a, new_ta);
                }
                Self::ok("airdrop")
            }
            IX_REASSIGN_UNCHECKED => {
                // accounts: [account]; data: disc ++ new_owner(pubkey). BUG:
                // no authority check.
                if ix.accounts.is_empty() || ix.data.len() < 1 + 32 {
                    return Self::revert();
                }
                let acct = ix.accounts[0].pubkey;
                let mut new_owner = [0u8; 32];
                new_owner.copy_from_slice(&ix.data[1..33]);
                if let Some(a) = self.state.accounts.get_mut(&acct) {
                    a.owner = new_owner;
                }
                Self::ok("reassign")
            }
            _ => Self::revert(),
        }
    }

    fn lamports(&self, pubkey: &Pubkey) -> u64 {
        self.state
            .accounts
            .get(pubkey)
            .map(|a| a.lamports)
            .unwrap_or(0)
    }

    fn account_data(&self, pubkey: &Pubkey) -> Vec<u8> {
        self.state
            .accounts
            .get(pubkey)
            .map(|a| a.data.clone())
            .unwrap_or_default()
    }

    fn account_owner(&self, pubkey: &Pubkey) -> Option<Pubkey> {
        self.state.accounts.get(pubkey).map(|a| a.owner)
    }

    fn snapshot(&mut self) -> u64 {
        self.snapshots.push(self.state.clone());
        (self.snapshots.len() - 1) as u64
    }

    fn revert_to(&mut self, snapshot: u64) {
        if let Some(s) = self.snapshots.get(snapshot as usize) {
            self.state = s.clone();
        }
    }
}
