//! The fuzz plan: everything an IDL *cannot* tell us.
//!
//! An Anchor IDL describes a program's instruction surface, but says nothing
//! about the world the program runs in — which accounts exist, what is in
//! them, which one is *the* mint, or what must stay true. That is this file: a
//! small JSON document the caller writes once per target, combined with the
//! parsed IDL to produce a runnable plan.
//!
//! ```json
//! {
//!   "program_id": "Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS",
//!   "program_so": "target/deploy/my_program.so",
//!   "accounts": [
//!     { "name": "mint",  "pubkey": "…", "lamports": 1000000, "space": 82 },
//!     { "name": "alice", "pubkey": "…", "lamports": 1000000, "space": 165 }
//!   ],
//!   "signers":  ["…authority…"],
//!   "writable": ["…mint…", "…alice…"],
//!   "pin": { "mint": "…mint…" },
//!   "invariants": [
//!     { "type": "token_conservation", "mint": "…", "token_accounts": ["…"],
//!       "amount_offset": 64, "supply_offset": 36 },
//!     { "type": "account_owner", "account": "…" }
//!   ],
//!   "seed": 1, "runs": 500, "depth": 10
//! }
//! ```

use crate::fuzz::FuzzConfig;
use crate::generator::AccountPool;
use crate::invariant::{AccountOwnerInvariant, SolanaInvariant, TokenConservationInvariant};
use crate::model::Pubkey;
use serde::Deserialize;

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("fuzz plan is not valid JSON: {0}")]
    Json(#[from] serde_json::Error),
    #[error("`{field}`: {value:?} is not a valid base58 32-byte pubkey")]
    BadPubkey { field: String, value: String },
    #[error("`data_hex` for account {0:?} is not valid hex")]
    BadHex(String),
    #[error("unknown invariant type {0:?} (expected `token_conservation` or `account_owner`)")]
    UnknownInvariant(String),
    #[error("the plan declares no invariants — a fuzz run with nothing to check proves nothing")]
    NoInvariants,
}

/// A genesis account to seed before fuzzing.
#[derive(Debug, Clone, Deserialize)]
pub struct RawAccount {
    #[serde(default)]
    pub name: String,
    pub pubkey: String,
    #[serde(default = "default_lamports")]
    pub lamports: u64,
    /// Zero-filled data length. Ignored when `data_hex` is given.
    #[serde(default)]
    pub space: usize,
    /// Exact initial account data, hex-encoded.
    #[serde(default)]
    pub data_hex: Option<String>,
    /// Owning program; defaults to the program under test.
    #[serde(default)]
    pub owner: Option<String>,
}

fn default_lamports() -> u64 {
    1_000_000_000
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RawInvariant {
    /// `sum(token account amounts) == mint supply`.
    TokenConservation {
        #[serde(default)]
        name: Option<String>,
        mint: String,
        token_accounts: Vec<String>,
        amount_offset: usize,
        supply_offset: usize,
    },
    /// An account's owner must never change.
    AccountOwner {
        #[serde(default)]
        name: Option<String>,
        account: String,
    },
}

#[derive(Debug, Clone, Deserialize)]
pub struct RawPlan {
    #[serde(default)]
    pub program_id: Option<String>,
    #[serde(default)]
    pub program_so: Option<String>,
    #[serde(default)]
    pub accounts: Vec<RawAccount>,
    #[serde(default)]
    pub signers: Vec<String>,
    #[serde(default)]
    pub writable: Vec<String>,
    #[serde(default)]
    pub readonly: Vec<String>,
    #[serde(default)]
    pub pin: std::collections::BTreeMap<String, String>,
    #[serde(default)]
    pub invariants: Vec<RawInvariant>,
    #[serde(default)]
    pub seed: Option<u64>,
    #[serde(default)]
    pub runs: Option<usize>,
    #[serde(default)]
    pub depth: Option<usize>,
}

/// A resolved genesis account.
#[derive(Debug, Clone)]
pub struct GenesisAccount {
    pub name: String,
    pub pubkey: Pubkey,
    pub lamports: u64,
    pub data: Vec<u8>,
    /// `None` means "owned by the program under test".
    pub owner: Option<Pubkey>,
}

/// A runnable plan: pool, invariants, genesis and search parameters.
pub struct FuzzPlan {
    pub program_id: Option<Pubkey>,
    pub program_so: Option<String>,
    pub accounts: Vec<GenesisAccount>,
    pub pool: AccountPool,
    pub invariants: Vec<Box<dyn SolanaInvariant>>,
    pub config: FuzzConfig,
}

fn pubkey(field: &str, value: &str) -> Result<Pubkey, ConfigError> {
    let bytes = bs58::decode(value)
        .into_vec()
        .map_err(|_| ConfigError::BadPubkey {
            field: field.to_string(),
            value: value.to_string(),
        })?;
    bytes.try_into().map_err(|_| ConfigError::BadPubkey {
        field: field.to_string(),
        value: value.to_string(),
    })
}

fn pubkeys(field: &str, values: &[String]) -> Result<Vec<Pubkey>, ConfigError> {
    values.iter().map(|v| pubkey(field, v)).collect()
}

fn decode_hex(name: &str, s: &str) -> Result<Vec<u8>, ConfigError> {
    let s = s.strip_prefix("0x").unwrap_or(s);
    if !s.len().is_multiple_of(2) {
        return Err(ConfigError::BadHex(name.to_string()));
    }
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|_| ConfigError::BadHex(name.to_string())))
        .collect()
}

/// Parse a fuzz plan from JSON.
pub fn parse_plan(json: &str) -> Result<FuzzPlan, ConfigError> {
    let raw: RawPlan = serde_json::from_str(json)?;
    if raw.invariants.is_empty() {
        return Err(ConfigError::NoInvariants);
    }

    let program_id = match &raw.program_id {
        Some(p) => Some(pubkey("program_id", p)?),
        None => None,
    };

    let mut accounts = Vec::with_capacity(raw.accounts.len());
    for a in &raw.accounts {
        let pk = pubkey("accounts[].pubkey", &a.pubkey)?;
        let data = match &a.data_hex {
            Some(h) => decode_hex(&a.name, h)?,
            None => vec![0u8; a.space],
        };
        let owner = match &a.owner {
            Some(o) => Some(pubkey("accounts[].owner", o)?),
            None => None,
        };
        accounts.push(GenesisAccount {
            name: a.name.clone(),
            pubkey: pk,
            lamports: a.lamports,
            data,
            owner,
        });
    }

    let mut pool = AccountPool::new(
        pubkeys("signers", &raw.signers)?,
        pubkeys("writable", &raw.writable)?,
        pubkeys("readonly", &raw.readonly)?,
    );
    for (name, value) in &raw.pin {
        pool = pool.pin(name, pubkey("pin", value)?);
    }

    let mut invariants: Vec<Box<dyn SolanaInvariant>> = Vec::new();
    for inv in &raw.invariants {
        match inv {
            RawInvariant::TokenConservation {
                name,
                mint,
                token_accounts,
                amount_offset,
                supply_offset,
            } => {
                let label = name
                    .clone()
                    .unwrap_or_else(|| "token conservation: sum(amounts) == mint supply".into());
                invariants.push(Box::new(TokenConservationInvariant::new(
                    &label,
                    pubkey("invariants[].mint", mint)?,
                    pubkeys("invariants[].token_accounts", token_accounts)?,
                    *amount_offset,
                    *supply_offset,
                )));
            }
            RawInvariant::AccountOwner { name, account } => {
                let label = name.clone().unwrap_or_else(|| "account owner integrity".into());
                invariants.push(Box::new(AccountOwnerInvariant::track(
                    &label,
                    pubkey("invariants[].account", account)?,
                )));
            }
        }
    }

    let defaults = FuzzConfig::default();
    let config = FuzzConfig {
        seed: raw.seed.unwrap_or(defaults.seed),
        max_runs: raw.runs.unwrap_or(defaults.max_runs),
        sequence_depth: raw.depth.unwrap_or(defaults.sequence_depth),
    };

    Ok(FuzzPlan {
        program_id,
        program_so: raw.program_so.clone(),
        accounts,
        pool,
        invariants,
        config,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // base58 of 32 zero bytes / other fixed arrays, for deterministic tests.
    fn b58(bytes: [u8; 32]) -> String {
        bs58::encode(bytes).into_string()
    }

    #[test]
    fn parses_a_full_plan() {
        let mint = b58([0x30; 32]);
        let acct = b58([0x01; 32]);
        let auth = b58([0xA0; 32]);
        let json = format!(
            r#"{{
              "program_id": "{p}",
              "program_so": "target/deploy/x.so",
              "accounts": [
                {{ "name": "mint", "pubkey": "{mint}", "space": 82 }},
                {{ "name": "alice", "pubkey": "{acct}", "lamports": 5, "data_hex": "0x00ff" }}
              ],
              "signers": ["{auth}"],
              "writable": ["{mint}", "{acct}"],
              "pin": {{ "mint": "{mint}" }},
              "invariants": [
                {{ "type": "token_conservation", "mint": "{mint}",
                   "token_accounts": ["{acct}"], "amount_offset": 64, "supply_offset": 36 }},
                {{ "type": "account_owner", "account": "{acct}" }}
              ],
              "seed": 9, "runs": 42, "depth": 5
            }}"#,
            p = b58([0x70; 32]),
        );

        let plan = parse_plan(&json).unwrap();
        assert_eq!(plan.program_id, Some([0x70; 32]));
        assert_eq!(plan.program_so.as_deref(), Some("target/deploy/x.so"));

        assert_eq!(plan.accounts.len(), 2);
        assert_eq!(plan.accounts[0].data, vec![0u8; 82]);
        assert_eq!(plan.accounts[0].lamports, default_lamports());
        assert_eq!(plan.accounts[1].data, vec![0x00, 0xff]);
        assert_eq!(plan.accounts[1].lamports, 5);

        assert_eq!(plan.pool.signers, vec![[0xA0; 32]]);
        assert_eq!(plan.pool.pinned.get("mint"), Some(&[0x30; 32]));

        assert_eq!(plan.invariants.len(), 2);
        assert_eq!(plan.config.seed, 9);
        assert_eq!(plan.config.max_runs, 42);
        assert_eq!(plan.config.sequence_depth, 5);
    }

    #[test]
    fn search_params_fall_back_to_defaults() {
        let json = format!(
            r#"{{ "invariants": [ {{ "type": "account_owner", "account": "{a}" }} ] }}"#,
            a = b58([0x01; 32])
        );
        let plan = parse_plan(&json).unwrap();
        let d = FuzzConfig::default();
        assert_eq!(plan.config.seed, d.seed);
        assert_eq!(plan.config.max_runs, d.max_runs);
        assert_eq!(plan.config.sequence_depth, d.sequence_depth);
        assert_eq!(plan.program_id, None);
    }

    #[test]
    fn a_plan_with_no_invariants_is_rejected() {
        // Silently running a fuzzer that checks nothing would report "no
        // findings" and look like a clean audit.
        assert!(matches!(
            parse_plan(r#"{ "invariants": [] }"#),
            Err(ConfigError::NoInvariants)
        ));
    }

    #[test]
    fn rejects_bad_pubkeys_and_hex() {
        let bad = r#"{ "program_id": "0OIl", "invariants": [ { "type": "account_owner", "account": "x" } ] }"#;
        assert!(matches!(parse_plan(bad), Err(ConfigError::BadPubkey { .. })));

        let json = format!(
            r#"{{ "accounts": [ {{ "name": "a", "pubkey": "{a}", "data_hex": "zz" }} ],
                  "invariants": [ {{ "type": "account_owner", "account": "{a}" }} ] }}"#,
            a = b58([0x01; 32])
        );
        assert!(matches!(parse_plan(&json), Err(ConfigError::BadHex(_))));
    }

    #[test]
    fn rejects_an_unknown_invariant_type() {
        let json = format!(
            r#"{{ "invariants": [ {{ "type": "wishful_thinking", "account": "{a}" }} ] }}"#,
            a = b58([0x01; 32])
        );
        // serde's tagged-enum error surfaces as a JSON error naming the tag.
        // (`FuzzPlan` holds trait objects, so it isn't `Debug`/`unwrap_err`able.)
        let err = match parse_plan(&json) {
            Err(e) => e,
            Ok(_) => panic!("an unknown invariant type must be rejected"),
        };
        assert!(
            format!("{err}").contains("wishful_thinking"),
            "error should name the unknown type, got: {err}"
        );
    }
}
