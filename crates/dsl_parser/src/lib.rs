#![warn(missing_docs)]
#![deny(unsafe_code)]

//! DSL Parser: Compile invariant expressions into IR.

pub mod grammar;
pub mod lexer;
pub mod parser;

pub use parser::{parse_invariant, InvariantParser};
