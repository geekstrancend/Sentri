#![warn(missing_docs)]
#![deny(unsafe_code)]

//! EVM (Ethereum/Solidity) program analyzer with advanced analysis capabilities.
//!
//! This crate provides comprehensive static and dynamic analysis for EVM smart contracts:
//!
//! - **AST Analysis**: Parse Solidity using solc JSON AST
//! - **Control Flow**: Build and analyze control flow graphs
//! - **Data Flow**: Track variable definitions, uses, and taint propagation
//! - **Bytecode Analysis**: Disassemble and analyze compiled EVM bytecode
//! - **Symbolic Execution**: Verify invariants hold for all inputs

pub mod analyzer;
pub mod ast;
pub mod bytecode;
pub mod cfg;
pub mod dataflow;
pub mod errors;
pub mod symbolic;

pub use analyzer::EvmAnalyzer;
pub use ast::{AstContract, SolidityParser, Visibility};
pub use bytecode::{BytecodeAnalyzer, Instruction, Opcode};
pub use cfg::ControlFlowGraph;
pub use dataflow::DataFlow;
pub use errors::{AnalysisError, AnalysisResult};
pub use symbolic::SymbolicExecutor;
