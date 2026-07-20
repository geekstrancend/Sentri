//! End-to-end verification against **real SPL Token bytecode**.
//!
//! Every other test in this crate proves the engine against a mock we wrote.
//! This one runs the whole pipeline — generator → real SVM → invariant oracle
//! — over the genuine, audited SPL Token program that litesvm embeds. It is
//! the strongest statement available offline: the fuzzer executes real Solana
//! bytecode, and does **not** report a violation on correct code.
//!
//! A false-positive test is the right shape here. SPL Token is correct, so a
//! conservation violation would be our bug, not its. (The catching side is
//! proven separately against the mock's deliberately buggy airdrop.)

use crate::backend::SvmBackend;
use crate::fuzz::{fuzz, FuzzConfig};
use crate::generator::AccountPool;
use crate::invariant::{SolanaInvariant, TokenConservationInvariant};
use crate::litesvm_backend::LiteSvmGenesis;
use crate::model::{AccountRole, ArgKind, InstructionSpec, Pubkey};

/// The real SPL Token program id.
const SPL_TOKEN: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";

const AUTHORITY: Pubkey = [0xA1; 32];
const MINT: Pubkey = [0x31; 32];
const ACCT_A: Pubkey = [0x11; 32];
const ACCT_B: Pubkey = [0x12; 32];

// Real SPL Token account layouts.
//
// Mint (82 bytes): mint_authority COption<Pubkey> (4 tag + 32), supply u64,
// decimals u8, is_initialized bool, freeze_authority COption<Pubkey>.
const MINT_LEN: usize = 82;
const SUPPLY_OFFSET: usize = 36;
// Account (165 bytes): mint (32), owner (32), amount u64 @64, delegate
// COption<Pubkey> (36), state u8 @108, …
const ACCOUNT_LEN: usize = 165;
const AMOUNT_OFFSET: usize = 64;
const STATE_OFFSET: usize = 108;
const STATE_INITIALIZED: u8 = 1;

fn spl_token_id() -> Pubkey {
    let v = bs58::decode(SPL_TOKEN).into_vec().expect("valid base58");
    v.try_into().expect("32 bytes")
}

/// A initialized SPL mint with `supply = 0` and `AUTHORITY` as mint authority.
fn mint_data(supply: u64) -> Vec<u8> {
    let mut d = vec![0u8; MINT_LEN];
    d[0..4].copy_from_slice(&1u32.to_le_bytes()); // COption::Some
    d[4..36].copy_from_slice(&AUTHORITY);
    d[SUPPLY_OFFSET..SUPPLY_OFFSET + 8].copy_from_slice(&supply.to_le_bytes());
    d[44] = 0; // decimals
    d[45] = 1; // is_initialized
    d
}

/// An initialized SPL token account of `MINT`, owned by `AUTHORITY`.
fn token_account_data(amount: u64) -> Vec<u8> {
    let mut d = vec![0u8; ACCOUNT_LEN];
    d[0..32].copy_from_slice(&MINT);
    d[32..64].copy_from_slice(&AUTHORITY);
    d[AMOUNT_OFFSET..AMOUNT_OFFSET + 8].copy_from_slice(&amount.to_le_bytes());
    d[STATE_OFFSET] = STATE_INITIALIZED;
    d
}

fn genesis() -> LiteSvmGenesis {
    let token = spl_token_id();
    // Seeded directly rather than via InitializeMint/InitializeAccount: the
    // layouts above are the program's own, so this is the same end state with
    // fewer moving parts.
    LiteSvmGenesis::default()
        .with_spl_programs()
        .account(MINT, 10_000_000, mint_data(0), token)
        .account(ACCT_A, 10_000_000, token_account_data(0), token)
        .account(ACCT_B, 10_000_000, token_account_data(0), token)
}

/// SPL Token instructions, by their real tag bytes.
fn spl_specs() -> Vec<InstructionSpec> {
    vec![
        // Transfer = 3: [source(w), destination(w), authority(s)]
        InstructionSpec::new(
            "transfer",
            vec![3],
            vec![ArgKind::U64],
            vec![AccountRole::Writable, AccountRole::Writable, AccountRole::Signer],
            true,
        )
        .with_account_names(vec!["source".into(), "destination".into(), "authority".into()]),
        // MintTo = 7: [mint(w), destination(w), authority(s)]
        InstructionSpec::new(
            "mint_to",
            vec![7],
            vec![ArgKind::U64],
            vec![AccountRole::Writable, AccountRole::Writable, AccountRole::Signer],
            true,
        )
        .with_account_names(vec!["mint".into(), "destination".into(), "authority".into()]),
        // Burn = 8: [account(w), mint(w), authority(s)]
        InstructionSpec::new(
            "burn",
            vec![8],
            vec![ArgKind::U64],
            vec![AccountRole::Writable, AccountRole::Writable, AccountRole::Signer],
            true,
        )
        .with_account_names(vec!["account".into(), "mint".into(), "authority".into()]),
        ]
}

fn pool() -> AccountPool {
    // `mint` is pinned: SPL Token would reject a non-mint there anyway, but
    // pinning spends the run's budget on sequences that actually execute.
    AccountPool::new(vec![AUTHORITY], vec![ACCT_A, ACCT_B], vec![])
        .pin("mint", MINT)
}

fn conservation() -> Box<dyn SolanaInvariant> {
    Box::new(TokenConservationInvariant::new(
        "SPL conservation: sum(amount) == mint supply",
        MINT,
        vec![ACCT_A, ACCT_B],
        AMOUNT_OFFSET,
        SUPPLY_OFFSET,
    ))
}

#[test]
fn genesis_seeds_real_spl_accounts_readable_through_the_backend() {
    // Guards the whole e2e: if the layout or seeding were wrong, the
    // false-positive test below would pass vacuously.
    let backend = genesis().build();
    assert_eq!(backend.account_owner(&MINT), Some(spl_token_id()));
    assert_eq!(backend.account_data(&MINT).len(), MINT_LEN);
    assert_eq!(backend.account_data(&ACCT_A).len(), ACCOUNT_LEN);
    assert!(backend.lamports(&ACCT_A) > 0);
}

#[test]
fn mint_to_executes_on_real_spl_bytecode_and_moves_supply() {
    // Proves instructions actually execute in the SVM — a run where
    // everything reverted would also show "no violations".
    let mut backend = genesis().build();
    let (ix, signers) = {
        use crate::model::{AccountMeta, Instruction};
        let mut data = vec![7u8];
        data.extend_from_slice(&1_000u64.to_le_bytes());
        (
            Instruction {
                program_id: spl_token_id(),
                accounts: vec![
                    AccountMeta::writable(MINT),
                    AccountMeta::writable(ACCT_A),
                    AccountMeta::signer_writable(AUTHORITY),
                ],
                data,
            },
            vec![AUTHORITY],
        )
    };

    let outcome = backend.execute(&ix, &signers);
    assert!(
        !outcome.reverted,
        "real SPL MintTo must succeed; logs: {:?}",
        outcome.logs
    );

    let supply = crate::backend::decode_u64_le(&backend.account_data(&MINT), SUPPLY_OFFSET);
    let amount = crate::backend::decode_u64_le(&backend.account_data(&ACCT_A), AMOUNT_OFFSET);
    assert_eq!(supply, 1_000, "mint supply must reflect the mint");
    assert_eq!(amount, 1_000, "destination balance must reflect the mint");
}

#[test]
fn no_false_positive_fuzzing_real_spl_token() {
    // The headline claim: drive real SPL Token bytecode with generated
    // transfer/mint/burn sequences and find nothing, because there is nothing
    // to find. A hit here would be a bug in our oracle.
    let config = FuzzConfig {
        seed: 11,
        // Modest: each run rebuilds a full SVM with the SPL programs loaded.
        max_runs: 15,
        sequence_depth: 6,
    };
    let result = fuzz(
        || Box::new(genesis().build()),
        spl_token_id(),
        &spl_specs(),
        &pool(),
        vec![conservation()],
        config,
    );
    assert!(
        result.is_none(),
        "SPL Token conserves supply; a violation means our invariant is wrong"
    );
}
