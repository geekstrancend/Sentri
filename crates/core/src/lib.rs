#![warn(missing_docs)]
#![deny(unsafe_code)]

//! Invar Core: Base abstractions for multi-chain invariant analysis.
//!
//! This module defines the core traits and types that are chain-agnostic
//! and form the foundation for all analyzers and generators.

pub mod account_abstraction;
pub mod attack_patterns;
pub mod error;
pub mod evaluator;
pub mod model;
pub mod security_validator;
pub mod threat_model;
pub mod traits;
pub mod type_checker;
pub mod types;

pub use account_abstraction::{
    AAContext, AALayer, AccountState, CrossLayerCheckResult, EntryPointState, PaymasterState,
    UserOpData,
};
pub use attack_patterns::AttackPatternDB;
pub use error::{InvarError, Result};
pub use evaluator::{EvalResult, EvaluationError, Evaluator, ExecutionContext, Value};
pub use model::{FunctionModel, Invariant, ProgramModel, StateVar};
pub use security_validator::{IssueSeverity, SecurityIssue, SecurityReport, SecurityValidator};
pub use threat_model::{
    DSLSandbox, InjectionVerifier, SimulationIsolation, StrictModeAnalyzer, TamperDetector,
    ThreatModelConfig, ThreatModelError, ThreatResult,
};
pub use traits::{ChainAnalyzer, CodeGenerator, Simulator};
pub use type_checker::TypeChecker;
pub use types::{Type, TypeError, TypeResult, TypedExpr, TypedValue};
