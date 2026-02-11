#![warn(missing_docs)]
#![deny(unsafe_code)]

//! Invar Core: Base abstractions for multi-chain invariant analysis.
//!
//! This module defines the core traits and types that are chain-agnostic
//! and form the foundation for all analyzers and generators.

pub mod error;
pub mod model;
pub mod traits;
pub mod types;
pub mod type_checker;
pub mod evaluator;
pub mod threat_model;

pub use error::{InvarError, Result};
pub use model::{FunctionModel, Invariant, ProgramModel, StateVar};
pub use traits::{ChainAnalyzer, CodeGenerator, Simulator};
pub use types::{Type, TypedExpr, TypedValue, TypeError, TypeResult};
pub use type_checker::TypeChecker;
pub use evaluator::{Evaluator, ExecutionContext, Value, EvaluationError, EvalResult};
pub use threat_model::{
    ThreatModelConfig, ThreatModelError, ThreatResult,
    InjectionVerifier, TamperDetector, DSLSandbox, StrictModeAnalyzer, SimulationIsolation,
};
