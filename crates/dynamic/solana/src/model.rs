//! The Solana call model.
//!
//! An EVM "call" is flat: one caller, one target, one blob of calldata. A
//! Solana instruction is not — it names a program, carries opaque instruction
//! data, and (crucially) an ordered list of *accounts*, each flagged
//! signer/writable. State lives in those accounts, not in the program. This
//! module is that model, kept deliberately separate from the EVM
//! `EncodedCall` shape because forcing one onto the other is exactly the
//! mismatch that makes naive "multi-chain" fuzzers wrong.

/// A 32-byte account/program address.
pub type Pubkey = [u8; 32];

/// One account referenced by an instruction, with its role for this call.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountMeta {
    pub pubkey: Pubkey,
    /// Must have signed the transaction (authority checks depend on this).
    pub is_signer: bool,
    /// The instruction is permitted to mutate this account's data/lamports.
    pub is_writable: bool,
}

impl AccountMeta {
    pub fn signer_writable(pubkey: Pubkey) -> Self {
        Self { pubkey, is_signer: true, is_writable: true }
    }
    pub fn writable(pubkey: Pubkey) -> Self {
        Self { pubkey, is_signer: false, is_writable: true }
    }
    pub fn readonly(pubkey: Pubkey) -> Self {
        Self { pubkey, is_signer: false, is_writable: false }
    }
}

/// A single instruction: the Solana unit of execution the fuzzer drives.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instruction {
    pub program_id: Pubkey,
    pub accounts: Vec<AccountMeta>,
    /// Opaque instruction data. Convention: the first byte(s) are the
    /// instruction discriminator (Anchor uses an 8-byte discriminator; native
    /// programs often use a single tag byte). The generator treats this as a
    /// tag prefix + argument bytes.
    pub data: Vec<u8>,
}

/// The result of executing one instruction.
#[derive(Debug, Clone)]
pub struct IxOutcome {
    /// The transaction was rolled back (a guard/`require`/`assert` failed, a
    /// missing signer, a constraint violation, …). As in EVM, a revert is an
    /// expected, informative outcome — not an error.
    pub reverted: bool,
    /// Program log lines, for diagnostics / PoC rendering.
    pub logs: Vec<String>,
}

/// Describes one callable instruction on a program, for the generator:
/// its discriminator, argument shape, and the account roles it expects. The
/// analogue of the EVM `FunctionSpec`, but account-aware.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstructionSpec {
    pub name: String,
    /// Leading discriminator/tag bytes that select this instruction.
    pub discriminator: Vec<u8>,
    /// Fixed-width scalar args appended after the discriminator.
    pub args: Vec<ArgKind>,
    /// The account roles this instruction reads/writes, in order. The
    /// generator fills concrete pubkeys from the actor/account pool.
    pub account_roles: Vec<AccountRole>,
    /// Whether this instruction mutates on-chain state (drives sequence
    /// generation; read-only instructions are used by invariants, not the
    /// sequence under test).
    pub mutates_state: bool,
}

/// A scalar instruction argument. Fixed-width only, mirroring the EVM
/// encoder's discipline — enough for amounts, flags, and pubkey args, which
/// covers the bulk of real instructions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArgKind {
    U64,
    U8,
    Bool,
    Pubkey,
}

/// The role an instruction expects an account to play. The generator resolves
/// each to a concrete account from the pool.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccountRole {
    /// The transaction authority (a signer) — used for access-control checks.
    Signer,
    /// A writable state account (a token account, a vault, …).
    Writable,
    /// A read-only account (a mint, a config, …).
    Readonly,
}

impl InstructionSpec {
    pub fn new(
        name: &str,
        discriminator: Vec<u8>,
        args: Vec<ArgKind>,
        account_roles: Vec<AccountRole>,
        mutates_state: bool,
    ) -> Self {
        Self {
            name: name.to_string(),
            discriminator,
            args,
            account_roles,
            mutates_state,
        }
    }
}
