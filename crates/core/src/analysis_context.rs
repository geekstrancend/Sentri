//! Analysis context tracking.

use crate::model::ProgramModel;

/// Represents a warning with source location.
#[derive(Debug, Clone)]
pub struct AnalysisWarning {
    /// The warning message.
    pub message: String,
    /// The file where the issue was found.
    pub file: String,
    /// The line number of the issue.
    pub line: usize,
    /// The column position (optional).
    pub column: Option<usize>,
    /// The exact source line content.
    pub source_line: Option<String>,
}

/// Context information from analysis phase.
#[derive(Debug, Clone)]
pub struct AnalysisContext {
    /// The analyzed program model.
    pub program: ProgramModel,
    /// Whether the analysis found no critical issues.
    pub is_valid: bool,
    /// All warnings encountered during analysis.
    pub warnings: Vec<AnalysisWarning>,
}

impl AnalysisContext {
    /// Create a new analysis context for a program.
    pub fn new(program: ProgramModel) -> Self {
        Self {
            program,
            is_valid: true,
            warnings: Vec::new(),
        }
    }

    /// Add a warning with source location.
    pub fn add_warning(
        &mut self,
        message: String,
        file: String,
        line: usize,
        column: Option<usize>,
        source_line: Option<String>,
    ) {
        self.warnings.push(AnalysisWarning {
            message,
            file,
            line,
            column,
            source_line,
        });
    }

    /// Mark the context as invalid due to critical issues.
    pub fn mark_invalid(&mut self) {
        self.is_valid = false;
    }

    /// Get the number of warnings.
    pub fn warning_count(&self) -> usize {
        self.warnings.len()
    }

    /// Check if there are any warnings.
    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }
}
