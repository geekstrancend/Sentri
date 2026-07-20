//! Anchor IDL → [`InstructionSpec`] front-end.
//!
//! Deriving the instruction surface is the mechanical half of setting up a
//! Solana fuzz run: an IDL already names every instruction, its discriminator,
//! its argument types, and each account's signer/writable role. This module
//! turns that into the specs the generator drives.
//!
//! Both Anchor IDL layouts are supported:
//!
//! * **0.30+** — carries an explicit `discriminator` byte array, and marks
//!   accounts with `signer` / `writable`.
//! * **legacy** — has no discriminator (Anchor derives it at runtime as
//!   `sha256("global:<snake_case_name>")[..8]`, which we recompute), and marks
//!   accounts with `isSigner` / `isMut`.
//!
//! What an IDL *cannot* tell us is the genesis state (which concrete accounts
//! exist) or which invariants should hold — those come from the fuzz config,
//! not from here.
//!
//! ## On skipping
//!
//! The generator emits fixed-width Borsh scalars. An instruction taking a
//! `String`, `Vec<T>`, or a user-defined struct cannot be encoded correctly,
//! and emitting *something* would just produce garbage the program rejects —
//! silently wasting the whole run. Such instructions are excluded and reported
//! in [`IdlProgram::skipped`], so a caller can see exactly what is not being
//! fuzzed rather than assume full coverage.

use crate::model::{AccountRole, ArgKind, InstructionSpec, Pubkey};
use serde::Deserialize;
use sha2::{Digest, Sha256};

/// A parsed program surface.
#[derive(Debug, Clone)]
pub struct IdlProgram {
    pub name: String,
    /// From the IDL's `address` (0.30+) or `metadata.address`, base58-decoded.
    /// Legacy IDLs often omit it, so the caller must supply one.
    pub program_id: Option<Pubkey>,
    /// Instructions the generator can drive.
    pub instructions: Vec<InstructionSpec>,
    /// Instructions deliberately excluded, with the reason.
    pub skipped: Vec<SkippedInstruction>,
}

/// An instruction that could not be represented, and why.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkippedInstruction {
    pub name: String,
    pub reason: String,
}

#[derive(Debug, thiserror::Error)]
pub enum IdlError {
    #[error("IDL is not valid JSON: {0}")]
    Json(#[from] serde_json::Error),
    #[error("IDL declares no instructions")]
    NoInstructions,
    #[error("program address {0:?} is not valid base58 or is not 32 bytes")]
    BadAddress(String),
}

// ── Raw IDL shapes (both layouts, tolerant) ──

#[derive(Deserialize)]
struct RawIdl {
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    address: Option<String>,
    #[serde(default)]
    metadata: Option<RawMetadata>,
    #[serde(default)]
    instructions: Vec<RawInstruction>,
}

#[derive(Deserialize)]
struct RawMetadata {
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    address: Option<String>,
}

#[derive(Deserialize)]
struct RawInstruction {
    name: String,
    #[serde(default)]
    discriminator: Option<Vec<u8>>,
    #[serde(default)]
    accounts: Vec<RawAccount>,
    #[serde(default)]
    args: Vec<RawArg>,
}

#[derive(Deserialize)]
struct RawAccount {
    #[serde(default)]
    name: String,
    // 0.30+ spelling
    #[serde(default)]
    signer: Option<bool>,
    #[serde(default)]
    writable: Option<bool>,
    // legacy spelling
    #[serde(default, rename = "isSigner")]
    is_signer: Option<bool>,
    #[serde(default, rename = "isMut")]
    is_mut: Option<bool>,
}

impl RawAccount {
    fn is_signer(&self) -> bool {
        self.signer.or(self.is_signer).unwrap_or(false)
    }
    fn is_writable(&self) -> bool {
        self.writable.or(self.is_mut).unwrap_or(false)
    }
    fn role(&self) -> AccountRole {
        // Signer wins: an account that is both signer and writable is an
        // authority, and the generator must draw it from the signer pool for
        // access-control checks to mean anything.
        if self.is_signer() {
            AccountRole::Signer
        } else if self.is_writable() {
            AccountRole::Writable
        } else {
            AccountRole::Readonly
        }
    }
}

#[derive(Deserialize)]
struct RawArg {
    #[serde(default)]
    #[allow(dead_code)]
    name: String,
    #[serde(rename = "type")]
    ty: serde_json::Value,
}

/// Map an IDL type to a fixed-width [`ArgKind`], or `None` if it is not
/// fixed-width (String, Vec, Option, defined structs, …).
fn map_arg_type(ty: &serde_json::Value) -> Option<ArgKind> {
    // Fixed-width primitives arrive as plain strings; everything composite
    // (`{"vec": …}`, `{"defined": …}`, `{"option": …}`, `{"array": …}`)
    // arrives as an object and is not representable.
    let name = ty.as_str()?;
    match name {
        "u8" => Some(ArgKind::U8),
        "i8" => Some(ArgKind::U8),
        "bool" => Some(ArgKind::Bool),
        "u16" | "i16" => Some(ArgKind::U16),
        "u32" | "i32" => Some(ArgKind::U32),
        "u64" => Some(ArgKind::U64),
        "i64" => Some(ArgKind::I64),
        "u128" | "i128" => Some(ArgKind::U128),
        // Anchor spells this `publicKey` (legacy) or `pubkey` (0.30+).
        "publicKey" | "pubkey" => Some(ArgKind::Pubkey),
        _ => None,
    }
}

/// Anchor's legacy global-instruction discriminator:
/// `sha256("global:<snake_case_name>")[..8]`.
pub fn anchor_discriminator(instruction_name: &str) -> Vec<u8> {
    let preimage = format!("global:{}", to_snake_case(instruction_name));
    let mut hasher = Sha256::new();
    hasher.update(preimage.as_bytes());
    hasher.finalize()[..8].to_vec()
}

/// Anchor names instructions in camelCase in legacy IDLs but hashes the
/// snake_case Rust identifier, so the conversion is part of the contract.
fn to_snake_case(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 4);
    for (i, ch) in s.chars().enumerate() {
        if ch.is_ascii_uppercase() {
            if i != 0 {
                out.push('_');
            }
            out.push(ch.to_ascii_lowercase());
        } else {
            out.push(ch);
        }
    }
    out
}

fn decode_base58_pubkey(s: &str) -> Option<Pubkey> {
    let bytes = bs58::decode(s).into_vec().ok()?;
    let arr: [u8; 32] = bytes.try_into().ok()?;
    Some(arr)
}

/// Parse an Anchor IDL into a drivable program surface.
pub fn parse_idl(json: &str) -> Result<IdlProgram, IdlError> {
    let raw: RawIdl = serde_json::from_str(json)?;
    if raw.instructions.is_empty() {
        return Err(IdlError::NoInstructions);
    }

    let name = raw
        .name
        .clone()
        .or_else(|| raw.metadata.as_ref().and_then(|m| m.name.clone()))
        .unwrap_or_else(|| "unknown".to_string());

    let address = raw
        .address
        .clone()
        .or_else(|| raw.metadata.as_ref().and_then(|m| m.address.clone()));
    let program_id = match address {
        Some(a) => Some(decode_base58_pubkey(&a).ok_or(IdlError::BadAddress(a))?),
        None => None,
    };

    let mut instructions = Vec::new();
    let mut skipped = Vec::new();

    for ix in &raw.instructions {
        // Arguments: bail on the first non-fixed-width one rather than
        // silently dropping it (dropping would shift every later argument).
        let mut args = Vec::with_capacity(ix.args.len());
        let mut unsupported = None;
        for arg in &ix.args {
            match map_arg_type(&arg.ty) {
                Some(kind) => args.push(kind),
                None => {
                    unsupported = Some(describe_type(&arg.ty));
                    break;
                }
            }
        }
        if let Some(ty) = unsupported {
            skipped.push(SkippedInstruction {
                name: ix.name.clone(),
                reason: format!("argument type `{ty}` is not fixed-width Borsh"),
            });
            continue;
        }

        let discriminator = ix
            .discriminator
            .clone()
            .unwrap_or_else(|| anchor_discriminator(&ix.name));

        let account_roles: Vec<AccountRole> = ix.accounts.iter().map(|a| a.role()).collect();
        // An instruction with no writable account cannot change state, so it
        // is not worth spending sequence slots on.
        let mutates_state = ix.accounts.iter().any(|a| a.is_writable());

        let account_names: Vec<String> =
            ix.accounts.iter().map(|a| a.name.clone()).collect();

        instructions.push(
            InstructionSpec::new(&ix.name, discriminator, args, account_roles, mutates_state)
                .with_account_names(account_names),
        );
    }

    Ok(IdlProgram {
        name,
        program_id,
        instructions,
        skipped,
    })
}

fn describe_type(ty: &serde_json::Value) -> String {
    match ty {
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Object(map) => map
            .keys()
            .next()
            .cloned()
            .unwrap_or_else(|| "object".to_string()),
        other => other.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_anchor_030_idl_with_explicit_discriminator() {
        let idl = r#"{
          "address": "11111111111111111111111111111111",
          "metadata": { "name": "vault" },
          "instructions": [
            {
              "name": "deposit",
              "discriminator": [1,2,3,4,5,6,7,8],
              "accounts": [
                { "name": "authority", "signer": true },
                { "name": "vault", "writable": true },
                { "name": "mint" }
              ],
              "args": [ { "name": "amount", "type": "u64" } ]
            }
          ]
        }"#;
        let program = parse_idl(idl).unwrap();
        assert_eq!(program.name, "vault");
        assert_eq!(program.program_id, Some([0u8; 32])); // system program
        assert_eq!(program.instructions.len(), 1);
        let ix = &program.instructions[0];
        assert_eq!(ix.discriminator, vec![1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(ix.args, vec![ArgKind::U64]);
        assert_eq!(
            ix.account_roles,
            vec![AccountRole::Signer, AccountRole::Writable, AccountRole::Readonly]
        );
        assert!(ix.mutates_state);
    }

    #[test]
    fn legacy_idl_derives_the_anchor_discriminator_and_reads_ismut_issigner() {
        let idl = r#"{
          "version": "0.1.0",
          "name": "legacy_program",
          "instructions": [
            {
              "name": "mintTo",
              "accounts": [
                { "name": "authority", "isMut": false, "isSigner": true },
                { "name": "tokenAccount", "isMut": true, "isSigner": false }
              ],
              "args": [ { "name": "amount", "type": "u64" } ]
            }
          ]
        }"#;
        let program = parse_idl(idl).unwrap();
        assert_eq!(program.program_id, None);
        let ix = &program.instructions[0];
        // Anchor hashes the snake_case name, not the camelCase IDL spelling.
        assert_eq!(ix.discriminator, anchor_discriminator("mint_to"));
        assert_eq!(ix.discriminator.len(), 8);
        assert_eq!(
            ix.account_roles,
            vec![AccountRole::Signer, AccountRole::Writable]
        );
    }

    #[test]
    fn anchor_discriminator_matches_known_value() {
        // Ground truth: sha256("global:initialize")[..8], the discriminator
        // every Anchor program uses for `initialize`.
        assert_eq!(
            anchor_discriminator("initialize"),
            vec![175, 175, 109, 31, 13, 152, 155, 237]
        );
    }

    #[test]
    fn skips_instructions_with_non_fixed_width_args_and_reports_why() {
        let idl = r#"{
          "name": "p",
          "instructions": [
            { "name": "setName", "accounts": [{"name":"a","isMut":true}],
              "args": [ { "name": "name", "type": "string" } ] },
            { "name": "setList", "accounts": [{"name":"a","isMut":true}],
              "args": [ { "name": "xs", "type": { "vec": "u64" } } ] },
            { "name": "ok", "accounts": [{"name":"a","isMut":true}],
              "args": [ { "name": "n", "type": "u64" } ] }
          ]
        }"#;
        let program = parse_idl(idl).unwrap();
        assert_eq!(program.instructions.len(), 1);
        assert_eq!(program.instructions[0].name, "ok");
        assert_eq!(program.skipped.len(), 2);
        assert_eq!(program.skipped[0].name, "setName");
        assert!(program.skipped[0].reason.contains("string"));
        assert!(program.skipped[1].reason.contains("vec"));
    }

    #[test]
    fn maps_every_fixed_width_scalar_and_both_pubkey_spellings() {
        let idl = r#"{
          "name": "p",
          "instructions": [
            { "name": "wide", "accounts": [{"name":"a","isMut":true}],
              "args": [
                { "name": "a", "type": "u8" },
                { "name": "b", "type": "u16" },
                { "name": "c", "type": "u32" },
                { "name": "d", "type": "u64" },
                { "name": "e", "type": "u128" },
                { "name": "f", "type": "i64" },
                { "name": "g", "type": "bool" },
                { "name": "h", "type": "publicKey" },
                { "name": "i", "type": "pubkey" }
              ] }
          ]
        }"#;
        let program = parse_idl(idl).unwrap();
        assert!(program.skipped.is_empty());
        assert_eq!(
            program.instructions[0].args,
            vec![
                ArgKind::U8,
                ArgKind::U16,
                ArgKind::U32,
                ArgKind::U64,
                ArgKind::U128,
                ArgKind::I64,
                ArgKind::Bool,
                ArgKind::Pubkey,
                ArgKind::Pubkey,
            ]
        );
    }

    #[test]
    fn read_only_instruction_is_parsed_but_not_marked_state_mutating() {
        let idl = r#"{
          "name": "p",
          "instructions": [
            { "name": "getBalance",
              "accounts": [ { "name": "acct", "isMut": false, "isSigner": false } ],
              "args": [] }
          ]
        }"#;
        let program = parse_idl(idl).unwrap();
        assert!(!program.instructions[0].mutates_state);
    }

    #[test]
    fn rejects_malformed_input() {
        assert!(matches!(
            parse_idl("not json"),
            Err(IdlError::Json(_))
        ));
        assert!(matches!(
            parse_idl(r#"{"name":"p","instructions":[]}"#),
            Err(IdlError::NoInstructions)
        ));
        assert!(matches!(
            parse_idl(r#"{"name":"p","address":"not-base58!!","instructions":[{"name":"i","accounts":[],"args":[]}]}"#),
            Err(IdlError::BadAddress(_))
        ));
    }
}
