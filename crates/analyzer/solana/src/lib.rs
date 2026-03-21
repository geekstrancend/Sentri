#![warn(missing_docs)]
#![deny(unsafe_code)]

//! Solana program analyzer: Extracts program model from Rust source.

pub mod analyzer;
pub mod anchor_model;
pub mod anchor_parser;

pub use analyzer::SolanaAnalyzer;
pub use anchor_model::{
    AccountSecurity, AnchorAccountField, AnchorAccountStruct, AnchorConstraint,
};
pub use anchor_parser::parse_anchor_accounts;
