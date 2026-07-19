//! Invariant oracles for the Solana account model.
//!
//! On Solana an invariant *reads account state directly* (no view call is
//! needed — the data is right there in the account), so `check` takes an
//! immutable backend reference. Otherwise the shape mirrors the EVM crate:
//! after every instruction in a sequence, each invariant gets a chance to
//! report a violation.

use crate::backend::{decode_u64_le, SvmBackend};
use crate::model::{Instruction, Pubkey};
use std::cell::RefCell;

/// Context describing the instruction that just executed.
pub struct CheckContext<'a> {
    pub last_ix: &'a Instruction,
    pub signers: &'a [Pubkey],
}

pub trait SolanaInvariant {
    fn name(&self) -> &str;

    /// Return `Some(message)` if the property is currently violated.
    fn check(&self, backend: &dyn SvmBackend, ctx: &CheckContext) -> Option<String>;

    /// Seed any baseline from genesis state before the sequence runs.
    fn reset(&self, backend: &dyn SvmBackend) {
        let _ = backend;
    }
}

/// SPL-token-style conservation: the sum of `amount` across a set of token
/// accounts must equal the mint's `supply`. This is the account-model twin of
/// the EVM `sum(balanceOf) == totalSupply()` — and it catches the same
/// real-world bug class: a program that credits a token account without a
/// matching supply update (the "mint out of thin air" that has drained real
/// Solana programs).
pub struct TokenConservationInvariant {
    label: String,
    mint: Pubkey,
    token_accounts: Vec<Pubkey>,
    /// Byte offset of the `amount` field inside a token account's data.
    amount_offset: usize,
    /// Byte offset of the `supply` field inside the mint's data.
    supply_offset: usize,
}

impl TokenConservationInvariant {
    pub fn new(
        label: impl Into<String>,
        mint: Pubkey,
        token_accounts: Vec<Pubkey>,
        amount_offset: usize,
        supply_offset: usize,
    ) -> Self {
        Self {
            label: label.into(),
            mint,
            token_accounts,
            amount_offset,
            supply_offset,
        }
    }
}

impl SolanaInvariant for TokenConservationInvariant {
    fn name(&self) -> &str {
        &self.label
    }

    fn check(&self, backend: &dyn SvmBackend, _ctx: &CheckContext) -> Option<String> {
        let supply = decode_u64_le(&backend.account_data(&self.mint), self.supply_offset);
        let sum: u128 = self
            .token_accounts
            .iter()
            .map(|a| decode_u64_le(&backend.account_data(a), self.amount_offset) as u128)
            .sum();
        if sum != supply as u128 {
            return Some(format!(
                "{}: sum(token amounts) = {} != mint supply = {}",
                self.label, sum, supply
            ));
        }
        None
    }
}

/// Account-ownership integrity: an account's owning program must not change to
/// an unexpected program over the run. Reassigning an account's owner is a
/// privileged, rarely-legitimate operation; an unexpected change is the
/// Solana analogue of an EVM ownership takeover and a known attack surface
/// (e.g. via a mis-scoped CPI or a missing owner check).
pub struct AccountOwnerInvariant {
    label: String,
    account: Pubkey,
    expected_owner: RefCell<Option<Pubkey>>,
    fixed_expected: Option<Pubkey>,
}

impl AccountOwnerInvariant {
    /// Track that `account`'s owner never changes from whatever it is at
    /// genesis (seeded in `reset`).
    pub fn track(label: impl Into<String>, account: Pubkey) -> Self {
        Self {
            label: label.into(),
            account,
            expected_owner: RefCell::new(None),
            fixed_expected: None,
        }
    }

    /// Assert `account`'s owner is always exactly `owner`.
    pub fn expect(label: impl Into<String>, account: Pubkey, owner: Pubkey) -> Self {
        Self {
            label: label.into(),
            account,
            expected_owner: RefCell::new(Some(owner)),
            fixed_expected: Some(owner),
        }
    }
}

impl SolanaInvariant for AccountOwnerInvariant {
    fn name(&self) -> &str {
        &self.label
    }

    fn reset(&self, backend: &dyn SvmBackend) {
        if self.fixed_expected.is_none() {
            *self.expected_owner.borrow_mut() = backend.account_owner(&self.account);
        }
    }

    fn check(&self, backend: &dyn SvmBackend, _ctx: &CheckContext) -> Option<String> {
        let expected = *self.expected_owner.borrow();
        let current = backend.account_owner(&self.account);
        match (expected, current) {
            (Some(e), Some(c)) if e != c => Some(format!(
                "{}: account owner changed from {} to {}",
                self.label,
                hex16(&e),
                hex16(&c)
            )),
            _ => None,
        }
    }
}

fn hex16(p: &Pubkey) -> String {
    p.iter().take(8).map(|b| format!("{:02x}", b)).collect()
}
