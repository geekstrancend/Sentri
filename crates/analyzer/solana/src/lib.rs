#![warn(missing_docs)]
#![deny(unsafe_code)]

//! Solana program analyzer: Extracts program model from Rust source.

pub mod analyzer;
/// Data models for parsed Anchor account structures and security analysis
pub mod anchor_model;
/// Parser for extracting account information from Anchor source code
pub mod anchor_parser;

pub use analyzer::SolanaAnalyzer;
pub use anchor_model::{
    AccountSecurity, AnchorAccountField, AnchorAccountStruct, AnchorConstraint,
};
pub use anchor_parser::parse_anchor_accounts;
