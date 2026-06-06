#![deny(unsafe_code)]
#![allow(missing_docs)]

//! DSL Parser: Compile invariant expressions into IR.

pub mod grammar;
pub mod lexer;
pub mod parser;

pub use parser::{parse_invariant, InvariantParser};
