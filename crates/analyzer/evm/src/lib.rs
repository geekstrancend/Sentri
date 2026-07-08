#![deny(unsafe_code)]
#![allow(missing_docs)]

//! EVM (Ethereum/Solidity) program analyzer with advanced analysis capabilities.
//!
//! This crate provides comprehensive static and dynamic analysis for EVM smart contracts:
//!
//! - **AST Analysis**: Parse Solidity using solc JSON AST via AstWalker
//! - **Control Flow**: Build and analyze control flow graphs
//! - **Data Flow**: Track variable definitions, uses, and taint propagation
//! - **Bytecode Analysis**: Disassemble and analyze compiled EVM bytecode
//! - **Symbolic Execution**: Verify invariants hold for all inputs

pub mod analyzer;
pub mod ast;
pub mod ast_types;
pub mod ast_walker;
pub mod bytecode;
pub mod cfg;
pub mod dataflow;
pub mod detectors;
pub mod errors;
/// Chain-agnostic semantic-model extraction (Epic 6.1 shared IR)
pub mod semantic_model;
pub mod symbolic;

pub use analyzer::EvmAnalyzer;
pub use ast::{AstContract, SolidityParser, Visibility};
pub use ast_types::{AstNode, SourceUnit};
pub use ast_walker::{AstVisitor, AstWalker};
pub use bytecode::{BytecodeAnalyzer, Instruction, Opcode};
pub use cfg::ControlFlowGraph;
pub use dataflow::DataFlow;
pub use errors::{AnalysisError, AnalysisResult};
pub use semantic_model::build_semantic_model;
pub use symbolic::SymbolicExecutor;
