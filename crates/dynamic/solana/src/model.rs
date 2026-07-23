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
        Self {
            pubkey,
            is_signer: true,
            is_writable: true,
        }
    }
    pub fn writable(pubkey: Pubkey) -> Self {
        Self {
            pubkey,
            is_signer: false,
            is_writable: true,
        }
    }
    pub fn readonly(pubkey: Pubkey) -> Self {
        Self {
            pubkey,
            is_signer: false,
            is_writable: false,
        }
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
    /// The IDL's name for each account position (`mint`, `vault`,
    /// `authority`, …), parallel to `account_roles`. Empty when unknown.
    ///
    /// Names are what let a run *pin* a position to a specific account. Left
    /// unpinned, the generator draws any writable account for the `mint`
    /// slot, which lets a *correct* program corrupt a conservation invariant —
    /// a false positive, not a finding.
    pub account_names: Vec<String>,
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
    U8,
    U16,
    U32,
    U64,
    U128,
    I64,
    Bool,
    Pubkey,
}

impl ArgKind {
    /// Encoded width in bytes. Borsh encodes these fixed-width and
    /// little-endian, so the generator must emit exactly this many bytes or
    /// the program will misparse every argument after it.
    pub fn width(self) -> usize {
        match self {
            ArgKind::U8 | ArgKind::Bool => 1,
            ArgKind::U16 => 2,
            ArgKind::U32 => 4,
            ArgKind::U64 | ArgKind::I64 => 8,
            ArgKind::U128 => 16,
            ArgKind::Pubkey => 32,
        }
    }
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
            account_names: Vec::new(),
            mutates_state,
        }
    }

    /// Attach the IDL's per-position account names, enabling pinning.
    pub fn with_account_names(mut self, names: Vec<String>) -> Self {
        self.account_names = names;
        self
    }

    /// The IDL name of account position `i`, if known.
    pub fn account_name(&self, i: usize) -> Option<&str> {
        self.account_names.get(i).map(String::as_str)
    }
}
