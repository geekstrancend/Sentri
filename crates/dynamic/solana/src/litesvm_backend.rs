//! Real Solana execution backend, backed by [`litesvm`] (an in-process SVM
//! that runs actual BPF bytecode). This is the Solana counterpart to the
//! revm-backed EVM backend: the engine (`generator`/`invariant`/`fuzz`) is
//! proven against the in-memory mock, and this adapter swaps in a real VM so
//! the same invariant search runs over compiled programs.
//!
//! Gated behind the `litesvm-backend` feature because it pulls the full Solana
//! runtime. Signature verification and blockhash checks are disabled: the
//! fuzzer drives the program directly and reasons about state, not consensus,
//! so requiring real keypairs per generated signer would add nothing but cost.

use crate::backend::SvmBackend;
use crate::model::{Instruction as SentriIx, IxOutcome, Pubkey as SentriPubkey};

use litesvm::LiteSVM;
use solana_account::Account;
use solana_instruction::{AccountMeta, Instruction};
use solana_message::Message;
use solana_pubkey::Pubkey;
use solana_transaction::Transaction;

fn to_pk(pk: SentriPubkey) -> Pubkey {
    Pubkey::new_from_array(pk)
}
fn from_pk(pk: Pubkey) -> SentriPubkey {
    pk.to_bytes()
}

/// A reproducible genesis: the programs to deploy and the accounts to seed.
///
/// The fuzz loop needs a `Fn() -> Box<dyn SvmBackend>` that yields a *fresh*
/// VM per run; a real SVM isn't cheaply clonable, so the harness captures a
/// `LiteSvmGenesis` (plain data, `Clone`) and rebuilds from it:
///
/// ```ignore
/// let genesis = LiteSvmGenesis::default()
///     .program(TOKEN_PROGRAM, &program_so)
///     .account(MINT, 1_000_000, vec![0u8; 8], TOKEN_PROGRAM);
/// let violation = fuzz(|| Box::new(genesis.build()), TOKEN_PROGRAM, &specs,
///                      &pool, invariants, config);
/// ```
#[derive(Clone, Default)]
pub struct LiteSvmGenesis {
    programs: Vec<(SentriPubkey, Vec<u8>)>,
    accounts: Vec<(SentriPubkey, u64, Vec<u8>, SentriPubkey)>,
    spl_programs: bool,
}

impl LiteSvmGenesis {
    /// Load the builtin SPL programs (Token, Associated Token, …) into the
    /// SVM, for targets that CPI into them — or to fuzz SPL Token itself.
    pub fn with_spl_programs(mut self) -> Self {
        self.spl_programs = true;
        self
    }

    /// Deploy a BPF program at `program_id` from its `.so` bytes.
    pub fn program(mut self, program_id: SentriPubkey, bytes: &[u8]) -> Self {
        self.programs.push((program_id, bytes.to_vec()));
        self
    }

    /// Seed an account (owner is the program that owns it, e.g. the token
    /// program for token accounts). Seeded accounts are tracked for snapshots.
    pub fn account(
        mut self,
        pubkey: SentriPubkey,
        lamports: u64,
        data: Vec<u8>,
        owner: SentriPubkey,
    ) -> Self {
        self.accounts.push((pubkey, lamports, data, owner));
        self
    }

    /// Build a genesis from a parsed [`FuzzPlan`]: deploy the program and
    /// seed every declared account. Accounts that name no owner default to
    /// being owned by the program under test, which is the common case.
    pub fn from_plan(
        plan: &crate::config::FuzzPlan,
        program_id: SentriPubkey,
        program_bytes: &[u8],
    ) -> Self {
        let mut genesis = Self::default().program(program_id, program_bytes);
        for a in &plan.accounts {
            genesis = genesis.account(
                a.pubkey,
                a.lamports,
                a.data.clone(),
                a.owner.unwrap_or(program_id),
            );
        }
        genesis
    }

    /// Construct a fresh backend from this genesis.
    pub fn build(&self) -> LiteSvmBackend {
        let mut svm = LiteSVM::new()
            .with_sigverify(false)
            .with_blockhash_check(false);
        if self.spl_programs {
            svm = svm.with_spl_programs();
        }
        let fee_payer = Pubkey::new_unique();
        svm.airdrop(&fee_payer, 1_000_000_000)
            .expect("fee payer airdrop must succeed on a fresh SVM");

        for (id, bytes) in &self.programs {
            svm.add_program(to_pk(*id), bytes);
        }

        let mut tracked = Vec::with_capacity(self.accounts.len());
        for (pk, lamports, data, owner) in &self.accounts {
            svm.set_account(
                to_pk(*pk),
                Account {
                    lamports: *lamports,
                    data: data.clone(),
                    owner: to_pk(*owner),
                    executable: false,
                    rent_epoch: 0,
                },
            )
            .expect("seeding a genesis account must succeed");
            tracked.push(*pk);
        }

        LiteSvmBackend {
            svm,
            fee_payer,
            tracked,
            snapshots: Vec::new(),
        }
    }
}

/// A [`SvmBackend`] that executes instructions on a real in-process SVM.
pub struct LiteSvmBackend {
    svm: LiteSVM,
    fee_payer: Pubkey,
    /// Accounts captured on `snapshot`/restored on `revert_to`. litesvm has no
    /// native snapshot, so we save/restore exactly the seeded state accounts.
    tracked: Vec<SentriPubkey>,
    snapshots: Vec<Vec<(SentriPubkey, Option<Account>)>>,
}

impl SvmBackend for LiteSvmBackend {
    fn execute(&mut self, ix: &SentriIx, _signers: &[SentriPubkey]) -> IxOutcome {
        let accounts: Vec<AccountMeta> = ix
            .accounts
            .iter()
            .map(|m| AccountMeta {
                pubkey: to_pk(m.pubkey),
                is_signer: m.is_signer,
                is_writable: m.is_writable,
            })
            .collect();
        let instruction = Instruction {
            program_id: to_pk(ix.program_id),
            accounts,
            data: ix.data.clone(),
        };
        let mut tx = Transaction::new_unsigned(Message::new(
            std::slice::from_ref(&instruction),
            Some(&self.fee_payer),
        ));
        tx.message.recent_blockhash = self.svm.latest_blockhash();

        match self.svm.send_transaction(tx) {
            Ok(meta) => IxOutcome {
                reverted: false,
                logs: meta.logs,
            },
            Err(fail) => IxOutcome {
                reverted: true,
                logs: fail.meta.logs,
            },
        }
    }

    fn lamports(&self, pubkey: &SentriPubkey) -> u64 {
        self.svm.get_balance(&to_pk(*pubkey)).unwrap_or(0)
    }

    fn account_data(&self, pubkey: &SentriPubkey) -> Vec<u8> {
        self.svm
            .get_account(&to_pk(*pubkey))
            .map(|a| a.data)
            .unwrap_or_default()
    }

    fn account_owner(&self, pubkey: &SentriPubkey) -> Option<SentriPubkey> {
        self.svm.get_account(&to_pk(*pubkey)).map(|a| from_pk(a.owner))
    }

    fn snapshot(&mut self) -> u64 {
        let snap: Vec<(SentriPubkey, Option<Account>)> = self
            .tracked
            .iter()
            .map(|pk| (*pk, self.svm.get_account(&to_pk(*pk))))
            .collect();
        self.snapshots.push(snap);
        (self.snapshots.len() - 1) as u64
    }

    fn revert_to(&mut self, snapshot: u64) {
        if let Some(snap) = self.snapshots.get(snapshot as usize).cloned() {
            for (pk, acct) in snap {
                if let Some(a) = acct {
                    let _ = self.svm.set_account(to_pk(pk), a);
                }
            }
        }
    }
}
