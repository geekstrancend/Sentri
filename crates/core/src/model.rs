//! Core domain models for invariant analysis.

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

/// A compiled invariant expression with metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invariant {
    /// Unique identifier for the invariant.
    pub name: String,

    /// Human-readable description.
    pub description: Option<String>,

    /// The invariant expression in IR form.
    pub expression: Expression,

    /// Severity level: "critical", "high", "medium", "low".
    pub severity: String,

    /// Category: "core", "defi", "bridge", "governance", "account-abstraction", etc.
    pub category: String,

    /// Whether this invariant should always hold.
    pub is_always_true: bool,

    /// Layer scopes for cross-layer analysis (e.g., ["bundler", "account", "paymaster"]).
    /// If empty, applies to all layers.
    pub layers: Vec<String>,
}

/// An expression tree representing invariant conditions.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Expression {
    /// Boolean literal.
    Boolean(bool),

    /// Variable reference.
    Var(String),

    /// Layer-qualified variable reference (e.g., bundler::nonce).
    LayerVar {
        /// Layer name (bundler, account, paymaster, protocol, entrypoint).
        layer: String,
        /// Variable name within the layer.
        var: String,
    },

    /// Integer constant.
    Int(i128),

    /// Comparison: left op right.
    BinaryOp {
        /// Left operand.
        left: Box<Expression>,
        /// Operator: ==, !=, <, >, <=, >=.
        op: BinaryOp,
        /// Right operand.
        right: Box<Expression>,
    },

    /// Logical operation: &&, ||.
    Logical {
        /// Left operand.
        left: Box<Expression>,
        /// Operator: And, Or.
        op: LogicalOp,
        /// Right operand.
        right: Box<Expression>,
    },

    /// Logical negation.
    Not(Box<Expression>),

    /// Function call.
    FunctionCall {
        /// Function name.
        name: String,
        /// Arguments.
        args: Vec<Expression>,
    },

    /// Tuple of expressions.
    Tuple(Vec<Expression>),
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Boolean(b) => write!(f, "{}", b),
            Self::Var(v) => write!(f, "{}", v),
            Self::LayerVar { layer, var } => write!(f, "{}::{}", layer, var),
            Self::Int(i) => write!(f, "{}", i),
            Self::BinaryOp { left, op, right } => {
                write!(f, "({} {} {})", left, op, right)
            }
            Self::Logical { left, op, right } => {
                write!(f, "({} {} {})", left, op, right)
            }
            Self::Not(e) => write!(f, "!({})", e),
            Self::FunctionCall { name, args } => {
                write!(f, "{}(", name)?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", arg)?;
                }
                write!(f, ")")
            }
            Self::Tuple(exprs) => {
                write!(f, "(")?;
                for (i, e) in exprs.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", e)?;
                }
                write!(f, ")")
            }
        }
    }
}

/// Binary operators for expressions.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum BinaryOp {
    /// Equality.
    Eq,
    /// Not equal.
    Neq,
    /// Less than.
    Lt,
    /// Greater than.
    Gt,
    /// Less than or equal.
    Lte,
    /// Greater than or equal.
    Gte,
}

impl std::fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Eq => write!(f, "=="),
            Self::Neq => write!(f, "!="),
            Self::Lt => write!(f, "<"),
            Self::Gt => write!(f, ">"),
            Self::Lte => write!(f, "<="),
            Self::Gte => write!(f, ">="),
        }
    }
}

/// Logical operators.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogicalOp {
    /// Logical AND.
    And,
    /// Logical OR.
    Or,
}

impl std::fmt::Display for LogicalOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::And => write!(f, "&&"),
            Self::Or => write!(f, "||"),
        }
    }
}

/// A state variable in a program.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateVar {
    /// Variable name.
    pub name: String,

    /// Data type (chain-specific).
    pub type_name: String,

    /// Whether it's mutable.
    pub is_mutable: bool,

    /// Visibility: "public", "private", "internal", etc.
    pub visibility: Option<String>,
}

/// A function or entry point in a program.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionModel {
    /// Function name.
    pub name: String,

    /// Function signature/parameters.
    pub parameters: Vec<String>,

    /// Return type.
    pub return_type: Option<String>,

    /// State variables this function mutates.
    pub mutates: BTreeSet<String>,

    /// State variables this function reads.
    pub reads: BTreeSet<String>,

    /// Whether this is an entry point.
    pub is_entry_point: bool,

    /// Whether it's pure/view (doesn't mutate state).
    pub is_pure: bool,
}

/// A complete program model extracted from source code.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramModel {
    /// Program/contract/module name.
    pub name: String,

    /// State variables.
    pub state_vars: BTreeMap<String, StateVar>,

    /// Functions/entry points.
    pub functions: BTreeMap<String, FunctionModel>,

    /// Functions → Mutations mapping (deterministic).
    pub mutation_graph: BTreeMap<String, BTreeSet<String>>,

    /// Chains supported: "solana", "evm", "move".
    pub chain: String,

    /// Source file path.
    pub source_path: String,
}

impl ProgramModel {
    /// Create a new program model.
    pub fn new(name: String, chain: String, source_path: String) -> Self {
        Self {
            name,
            chain,
            source_path,
            state_vars: BTreeMap::new(),
            functions: BTreeMap::new(),
            mutation_graph: BTreeMap::new(),
        }
    }

    /// Add a state variable to the model.
    pub fn add_state_var(&mut self, var: StateVar) {
        self.state_vars.insert(var.name.clone(), var);
    }

    /// Add a function to the model.
    pub fn add_function(&mut self, func: FunctionModel) {
        self.mutation_graph
            .insert(func.name.clone(), func.mutates.clone());
        self.functions.insert(func.name.clone(), func);
    }
}

/// Output from code generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationOutput {
    /// The generated code.
    pub code: String,

    /// Assertions injected.
    pub assertions: Vec<String>,

    /// Generated tests (if any).
    pub tests: Option<String>,

    /// Coverage percentage (0-100).
    pub coverage_percent: u8,
}

/// Report from a simulation run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationReport {
    /// Number of violations found.
    pub violations: usize,

    /// Violation traces.
    pub traces: Vec<String>,

    /// Coverage percentage.
    pub coverage: f64,

    /// Deterministic seed used.
    pub seed: u64,
}
