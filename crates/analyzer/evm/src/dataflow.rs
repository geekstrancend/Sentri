#![allow(missing_docs)]
//! Data Flow Analysis for value and taint tracking.
//!
//! Enables tracking of:
//! - Variable definitions and uses (def-use chains)
//! - Tainted values (user input, untrusted sources)
//! - Value propagation through assignments and calls
//! - Dependency detection and analysis

use crate::cfg::ControlFlowGraph;
use crate::errors::AnalysisResult;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use tracing::{debug, info};

/// Represents a variable's origin and taint status.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ValueOrigin {
    /// Variable is a constant.
    Constant { value: String },
    /// Variable comes from a parameter.
    Parameter { index: usize, tainted: bool },
    /// Variable comes from state storage.
    StateVar { name: String, tainted: bool },
    /// Variable comes from function call.
    FunctionCall { function: String, tainted: bool },
    /// Variable comes from external call (always tainted).
    ExternalCall { address: String },
    /// Variable comes from arithmetic operation.
    Arithmetic {
        operands: Vec<String>,
        tainted: bool,
    },
    /// Variable comes from untrusted source.
    Untrusted { source: String },
    /// Unknown origin.
    Unknown,
}

impl ValueOrigin {
    /// Check if value is tainted.
    pub fn is_tainted(&self) -> bool {
        matches!(
            self,
            ValueOrigin::Parameter { tainted: true, .. }
                | ValueOrigin::StateVar { tainted: true, .. }
                | ValueOrigin::FunctionCall { tainted: true, .. }
                | ValueOrigin::ExternalCall { .. }
                | ValueOrigin::Arithmetic { tainted: true, .. }
                | ValueOrigin::Untrusted { .. }
        )
    }
}

/// Definition-Use chain entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefUse {
    /// Variable name.
    pub variable: String,
    /// Where it's defined.
    pub definition: Location,
    /// Where it's used.
    pub uses: Vec<Location>,
}

/// Location of definition/use in code.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    /// Block ID.
    pub block: usize,
    /// Statement index in block.
    pub statement_idx: usize,
    /// Line number (if available).
    pub line: Option<usize>,
}

/// DataFlow analysis result for a function.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataFlow {
    /// Variable origins.
    pub origins: HashMap<String, ValueOrigin>,
    /// Definition-use chains.
    pub def_use_chains: Vec<DefUse>,
    /// Tainted variables.
    pub tainted_vars: HashSet<String>,
    /// Data dependencies.
    pub dependencies: HashMap<String, HashSet<String>>,
}

impl DataFlow {
    /// Create empty data flow analysis.
    pub fn new() -> Self {
        Self {
            origins: HashMap::new(),
            def_use_chains: Vec::new(),
            tainted_vars: HashSet::new(),
            dependencies: HashMap::new(),
        }
    }

    /// Check if a variable is tainted.
    pub fn is_tainted(&self, var: &str) -> bool {
        self.tainted_vars.contains(var)
    }

    /// Get all variables that depend on a given variable.
    pub fn transitive_dependents(&self, var: &str) -> HashSet<String> {
        let mut result = HashSet::new();
        let mut queue = vec![var.to_string()];

        while let Some(current) = queue.pop() {
            if !result.insert(current.clone()) {
                continue;
            }

            // Find all variables that depend on current
            for (dependent, depends_on) in &self.dependencies {
                if depends_on.contains(&current) {
                    queue.push(dependent.clone());
                }
            }
        }

        result.remove(var);
        result
    }

    /// Check if variable `a` can affect variable `b`.
    pub fn can_affect(&self, a: &str, b: &str) -> bool {
        self.transitive_dependents(a).contains(b)
    }
}

impl Default for DataFlow {
    fn default() -> Self {
        Self::new()
    }
}

/// Data flow analysis engine.
pub struct DataFlowAnalyzer;

impl DataFlowAnalyzer {
    /// Analyze data flow in a control flow graph.
    pub fn analyze(cfg: &ControlFlowGraph) -> AnalysisResult<DataFlow> {
        info!("Performing data flow analysis");

        let mut analysis = DataFlow::new();

        // Compute reaching definitions
        let reaching_defs = Self::compute_reaching_definitions(cfg)?;
        debug!(
            "Computed reaching definitions for {} blocks",
            reaching_defs.len()
        );

        // Compute live variables
        let _live_vars = Self::compute_live_variables(cfg)?;
        debug!("Computed live variables");

        // Extract definition-use chains
        analysis = Self::extract_def_use_chains(cfg, analysis)?;
        debug!("Extracted {} def-use chains", analysis.def_use_chains.len());

        // Identify tainted variables
        analysis = Self::identify_tainted(cfg, analysis)?;
        debug!(
            "Identified {} tainted variables",
            analysis.tainted_vars.len()
        );

        // Build dependency graph
        analysis = Self::build_dependencies(cfg, analysis)?;

        Ok(analysis)
    }

    /// Compute reaching definitions for each block.
    fn compute_reaching_definitions(
        cfg: &ControlFlowGraph,
    ) -> AnalysisResult<HashMap<usize, HashSet<String>>> {
        let mut reaching = HashMap::new();

        // TODO: Implement iterative reaching definitions algorithm
        // For now, return empty map
        let blocks = vec![cfg.entry()];
        for block in blocks {
            reaching.insert(block, HashSet::new());
        }

        Ok(reaching)
    }

    /// Compute live variables for each block (backwards analysis).
    fn compute_live_variables(
        cfg: &ControlFlowGraph,
    ) -> AnalysisResult<HashMap<usize, HashSet<String>>> {
        let mut live = HashMap::new();

        // TODO: Implement live variable analysis
        // For now, return empty map
        let blocks = vec![cfg.entry()];
        for block in blocks {
            live.insert(block, HashSet::new());
        }

        Ok(live)
    }

    /// Extract definition-use chains from control flow graph.
    fn extract_def_use_chains(
        _cfg: &ControlFlowGraph,
        analysis: DataFlow,
    ) -> AnalysisResult<DataFlow> {
        // TODO: Walk through CFG and extract def-use chains

        Ok(analysis)
    }

    /// Identify tainted variables through data flow.
    fn identify_tainted(_cfg: &ControlFlowGraph, analysis: DataFlow) -> AnalysisResult<DataFlow> {
        // Sources of taint:
        // - External calls
        // - Function parameters marked as untrusted
        // - User input
        // - Unchecked values

        info!("Identifying tainted variables");

        // TODO: Taint propagation analysis

        Ok(analysis)
    }

    /// Build data dependency graph.
    fn build_dependencies(cfg: &ControlFlowGraph, analysis: DataFlow) -> AnalysisResult<DataFlow> {
        // TODO: Construct dependency graph from def-use chains

        Ok(analysis)
    }
}

/// Taint propagation engine.
pub struct TaintAnalyzer {
    /// Tainted source identifiers.
    tainted_sources: HashSet<String>,
}

impl TaintAnalyzer {
    /// Create new taint analyzer.
    pub fn new() -> Self {
        let mut sources = HashSet::new();
        // Mark common untrusted sources
        sources.insert("msg.sender".to_string());
        sources.insert("tx.origin".to_string());
        sources.insert("calldata".to_string());
        sources.insert("input".to_string());

        Self {
            tainted_sources: sources,
        }
    }

    /// Add custom tainted source.
    pub fn add_tainted_source(&mut self, source: String) {
        self.tainted_sources.insert(source);
    }

    /// Check if a value origin is tainted.
    pub fn is_tainted(&self, origin: &ValueOrigin) -> bool {
        origin.is_tainted()
            || (match origin {
                ValueOrigin::FunctionCall { function, .. } => {
                    self.tainted_sources.contains(function)
                }
                _ => false,
            })
    }

    /// Propagate taint through assignments.
    pub fn propagate(&self, source_tainted: bool, _assignment_op: &str) -> bool {
        // Most operations propagate taint
        // Except: hash operations (keccak256, sha256) are taint sinks
        source_tainted
    }
}

impl Default for TaintAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// Use-Def Analysis for variable tracking.
pub struct UseDefAnalyzer;

impl UseDefAnalyzer {
    /// Compute use-def chains for all variables.
    pub fn compute_chains(cfg: &ControlFlowGraph) -> AnalysisResult<HashMap<String, Vec<DefUse>>> {
        let chains: HashMap<String, Vec<DefUse>> = HashMap::new();

        // TODO: Walk CFG and compute use-def chains

        Ok(chains)
    }

    /// Find all uses of a variable definition.
    pub fn find_uses(def_use: &DefUse) -> Vec<Location> {
        def_use.uses.clone()
    }

    /// Find the definition of a variable use.
    pub fn find_definition(def_uses: &[DefUse], var: &str) -> Option<Location> {
        def_uses
            .iter()
            .find(|du| du.variable == var)
            .map(|du| du.definition.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_origin_taint() {
        let constant = ValueOrigin::Constant {
            value: "42".to_string(),
        };
        assert!(!constant.is_tainted());

        let untrusted = ValueOrigin::Untrusted {
            source: "user_input".to_string(),
        };
        assert!(untrusted.is_tainted());

        let external = ValueOrigin::ExternalCall {
            address: "0x...".to_string(),
        };
        assert!(external.is_tainted());
    }

    #[test]
    fn test_taint_analyzer_creation() {
        let analyzer = TaintAnalyzer::new();
        assert!(analyzer.tainted_sources.contains("msg.sender"));
        assert!(analyzer.tainted_sources.contains("calldata"));
    }

    #[test]
    fn test_data_flow_taint_tracking() {
        let mut data_flow = DataFlow::new();
        data_flow.tainted_vars.insert("user_input".to_string());

        assert!(data_flow.is_tainted("user_input"));
        assert!(!data_flow.is_tainted("safe_var"));
    }

    #[test]
    fn test_data_flow_dependencies() {
        let mut data_flow = DataFlow::new();
        let mut deps = HashSet::new();
        deps.insert("a".to_string());
        deps.insert("b".to_string());

        data_flow.dependencies.insert("c".to_string(), deps);

        assert!(data_flow.dependencies.contains_key("c"));
        assert_eq!(data_flow.dependencies["c"].len(), 2);
    }

    #[test]
    fn test_transitive_dependents() {
        let mut data_flow = DataFlow::new();

        let mut deps_b = HashSet::new();
        deps_b.insert("a".to_string());
        data_flow.dependencies.insert("b".to_string(), deps_b);

        let mut deps_c = HashSet::new();
        deps_c.insert("b".to_string());
        data_flow.dependencies.insert("c".to_string(), deps_c);

        let dependents = data_flow.transitive_dependents("a");
        assert!(dependents.contains("b"));
        assert!(dependents.contains("c"));
    }
}
