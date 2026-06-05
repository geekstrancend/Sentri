#![warn(missing_docs)]
#![deny(unsafe_code)]

//! Solana program analyzer: Extracts program model from Rust source.

pub mod analyzer;
/// Data models for parsed Anchor account structures and security analysis
pub mod anchor_model;
/// Parser for extracting account information from Anchor source code
pub mod anchor_parser;
/// Vulnerability detectors for Solana programs
pub mod detectors;
pub mod solana_durable_nonce;
pub mod solana_pda_authority_validation;
pub mod solana_rent_exemption;

pub use analyzer::SolanaAnalyzer;
pub use anchor_model::{
    AccountSecurity, AnchorAccountField, AnchorAccountStruct, AnchorConstraint,
};
pub use anchor_parser::parse_anchor_accounts;
pub use detectors::*;
pub use solana_durable_nonce::detect_solana_durable_nonce_validation;
pub use solana_pda_authority_validation::detect_solana_pda_authority_validation;
pub use solana_rent_exemption::detect_solana_rent_exemption;
