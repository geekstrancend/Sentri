//! Report formatting (JSON, Markdown, CLI).

use super::Report;

/// Formats reports in various output formats.
pub struct ReportFormatter;

impl ReportFormatter {
    /// Format as JSON.
    pub fn to_json(report: &Report) -> serde_json::Result<String> {
        serde_json::to_string_pretty(&report)
    }

    /// Format as Markdown.
    pub fn to_markdown(report: &Report) -> String {
        format!(
            "# {}\n\n**Generated:** {}\n**Program:** {}\n\n## Summary\n- Invariants Checked: {}\n- Violations: {}\n- Coverage: {}%\n",
            report.title,
            report.generated_at,
            report.program,
            report.invariants_checked,
            report.violations_found,
            report.coverage_percent
        )
    }

    /// Format for CLI table.
    pub fn to_cli_table(report: &Report) -> String {
        format!(
            "┌─────────────────────────────────────────┐\n│ {} (Coverage: {}%) │\n├─────────────────────────────────────────┤\n│ Invariants: {} | Violations: {} │\n└─────────────────────────────────────────┘\n",
            report.program, report.coverage_percent, report.invariants_checked, report.violations_found
        )
    }
}
