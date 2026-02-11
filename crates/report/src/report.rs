//! Report data structures.

use serde::{Deserialize, Serialize};

/// A complete analysis report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    /// Report title.
    pub title: String,

    /// Timestamp of generation.
    pub generated_at: String,

    /// Program analyzed.
    pub program: String,

    /// Total invariants checked.
    pub invariants_checked: usize,

    /// Violations found.
    pub violations_found: usize,

    /// Coverage percentage.
    pub coverage_percent: u8,

    /// Functions protected.
    pub protected_functions: Vec<String>,

    /// Functions not protected.
    pub unprotected_functions: Vec<String>,

    /// Severity breakdown.
    pub severity_breakdown: SeverityBreakdown,
}

/// Breakdown by severity.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SeverityBreakdown {
    /// Critical violations.
    pub critical: usize,
    /// High severity violations.
    pub high: usize,
    /// Medium severity violations.
    pub medium: usize,
    /// Low severity violations.
    pub low: usize,
}

impl Report {
    /// Create a new report.
    pub fn new(title: String, program: String) -> Self {
        Self {
            title,
            generated_at: chrono::Utc::now().to_rfc3339(),
            program,
            invariants_checked: 0,
            violations_found: 0,
            coverage_percent: 0,
            protected_functions: Vec::new(),
            unprotected_functions: Vec::new(),
            severity_breakdown: SeverityBreakdown::default(),
        }
    }
}
