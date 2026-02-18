#![warn(missing_docs)]
#![deny(unsafe_code)]

//! Intermediate Representation: Chain-agnostic program model and invariant IR.
//!
//! This module re-exports core types and extends them with additional
//! IR-specific utilities while remaining chain-agnostic.

pub use invar_core::model::{
    BinaryOp, Expression, FunctionModel, GenerationOutput, Invariant, LogicalOp, ProgramModel,
    SimulationReport, StateVar,
};
pub use invar_core::{InvarError, Result};

pub mod analyzer_result;
pub mod ast;

pub use analyzer_result::AnalysisContext;
pub use ast::DependencyGraph;
