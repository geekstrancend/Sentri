#![allow(missing_docs)]
//! Error types for EVM analysis operations.

use thiserror::Error;

/// Result type for EVM analyzer operations.
pub type AnalysisResult<T> = Result<T, AnalysisError>;

/// Comprehensive error type for EVM analysis failures.
#[derive(Error, Debug)]
pub enum AnalysisError {
    /// Error compiling Solidity source with solc.
    #[error("Solidity compilation failed: {0}")]
    CompilationError(String),

    /// Error parsing JSON AST.
    #[error("AST parsing error: {0}")]
    AstParsingError(String),

    /// Error analyzing bytecode.
    #[error("Bytecode analysis error: {0}")]
    BytecodeAnalysisError(String),

    /// Error building control flow graph.
    #[error("Control flow graph construction error: {0}")]
    CfgError(String),

    /// Error during data flow analysis.
    #[error("Data flow analysis error: {0}")]
    DataFlowError(String),

    /// Error during symbolic execution.
    #[error("Symbolic execution error: {0}")]
    SymbolicExecutionError(String),

    /// Error during contract execution.
    #[error("Contract execution error: {0}")]
    ExecutionError(String),

    /// Generic I/O error.
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    /// JSON serialization/deserialization error.
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// Generic internal error.
    #[error("Internal error: {0}")]
    Internal(String),
}

impl AnalysisError {
    /// Create a compilation error.
    pub fn compilation(msg: impl Into<String>) -> Self {
        AnalysisError::CompilationError(msg.into())
    }

    /// Create an AST parsing error.
    pub fn ast_parsing(msg: impl Into<String>) -> Self {
        AnalysisError::AstParsingError(msg.into())
    }

    /// Create a bytecode analysis error.
    pub fn bytecode(msg: impl Into<String>) -> Self {
        AnalysisError::BytecodeAnalysisError(msg.into())
    }

    /// Create a CFG construction error.
    pub fn cfg(msg: impl Into<String>) -> Self {
        AnalysisError::CfgError(msg.into())
    }

    /// Create a data flow analysis error.
    pub fn dataflow(msg: impl Into<String>) -> Self {
        AnalysisError::DataFlowError(msg.into())
    }

    /// Create a symbolic execution error.
    pub fn symbolic(msg: impl Into<String>) -> Self {
        AnalysisError::SymbolicExecutionError(msg.into())
    }

    /// Create an execution error.
    pub fn execution(msg: impl Into<String>) -> Self {
        AnalysisError::ExecutionError(msg.into())
    }

    /// Create an internal error.
    pub fn internal(msg: impl Into<String>) -> Self {
        AnalysisError::Internal(msg.into())
    }
}
