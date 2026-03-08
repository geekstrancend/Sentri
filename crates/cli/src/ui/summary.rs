//! Summary dashboard for analysis results.

use crate::ui::constants::*;
use crate::ui::utils::{box_line, divider, empty_box_line, severity_bar, term_width};

/// Represents the analysis summary report.
#[derive(Debug, Clone)]
pub struct AnalysisSummary {
    /// The target file or path analyzed
    pub target: String,
    /// The blockchain being analyzed (EVM, Solana, Move)
    pub chain: String,
    /// Total number of checks performed
    pub total_checks: usize,
    /// Number of violations found
    pub violations: usize,
    /// Number of checks that passed
    pub passed: usize,
    /// Number of suppressed violations
    pub suppressed: usize,
    /// Analysis duration in seconds
    pub duration_secs: f64,
    /// Breakdown of violations by severity
    pub severity_breakdown: SeverityBreakdown,
}

/// Breakdown of violations by severity level.
#[derive(Debug, Clone)]
pub struct SeverityBreakdown {
    pub critical: usize,
    pub high: usize,
    pub medium: usize,
    pub low: usize,
}

/// Render the analysis summary dashboard.
///
/// Produces a bordered panel showing:
/// - Target, chain, checks, duration
/// - Severity breakdown with proportional bars
/// - Overall pass/fail status
///
/// # Arguments
/// * `summary` - The analysis summary to display
///
/// # Returns
/// The formatted summary as a string
pub fn render_summary(summary: &AnalysisSummary) -> String {
    let width = term_width();
    let mut output = String::new();

    output.push('\n');

    // Top border
    output.push_str(&format!(
        "{}{}{}\n",
        color_border("╭"),
        divider(width.saturating_sub(2)),
        color_border("╮")
    ));

    // Header line
    let header_line = "─ Analysis Summary ─".to_string();
    output.push_str(&format!("{}\n", box_line(&header_line, width)));

    // Empty line
    output.push_str(&format!("{}\n", empty_box_line(width)));

    // Target line
    let target_line = format!(
        "{}  {}",
        color_label("Target"),
        color_value(&summary.target)
    );
    output.push_str(&format!("{}\n", box_line(&target_line, width)));

    // Chain line
    let chain_line = format!("{}  {}", color_label("Chain"), color_value(&summary.chain));
    output.push_str(&format!("{}\n", box_line(&chain_line, width)));

    // Checks summary line
    let checks_line = format!(
        "{}  {} total  {}  {} violations  {}  {} passed  {}  {} suppressed",
        color_label("Checks"),
        color_value(&summary.total_checks.to_string()),
        color_dim("·"),
        color_value(&summary.violations.to_string()),
        color_dim("·"),
        color_value(&summary.passed.to_string()),
        color_dim("·"),
        color_value(&summary.suppressed.to_string()),
    );
    output.push_str(&format!("{}\n", box_line(&checks_line, width)));

    // Duration line
    let duration_line = format!("{}  {:.2}s", color_label("Duration"), summary.duration_secs);
    output.push_str(&format!("{}\n", box_line(&duration_line, width)));

    // Empty line
    output.push_str(&format!("{}\n", empty_box_line(width)));

    // Middle divider
    output.push_str(&format!(
        "{}{}{}     \n",
        color_border("├"),
        divider(width.saturating_sub(4)),
        color_border("┤")
    ));

    // Empty line
    output.push_str(&format!("{}\n", empty_box_line(width)));

    // Severity breakdown header
    let severity_header = "Severity Breakdown";
    output.push_str(&format!("{}\n", box_line(severity_header, width)));

    // Empty line
    output.push_str(&format!("{}\n", empty_box_line(width)));

    // Top row of severity bars (critical + high)
    let max_severity = summary
        .severity_breakdown
        .critical
        .max(summary.severity_breakdown.high)
        .max(summary.severity_breakdown.medium)
        .max(summary.severity_breakdown.low);

    let critical_bar = severity_bar(summary.severity_breakdown.critical, max_severity, "█", "░");
    let critical_count = summary.severity_breakdown.critical.to_string();
    let critical_line = format!(
        "{}  {}  {}    {}  {}  {}",
        color_critical("CRITICAL"),
        critical_bar,
        color_value(&critical_count),
        color_high("HIGH"),
        severity_bar(summary.severity_breakdown.high, max_severity, "█", "░"),
        color_value(&summary.severity_breakdown.high.to_string()),
    );
    output.push_str(&format!("{}\n", box_line(&critical_line, width)));

    // Bottom row of severity bars (medium + low)
    let medium_bar = severity_bar(summary.severity_breakdown.medium, max_severity, "█", "░");
    let medium_count = summary.severity_breakdown.medium.to_string();
    let low_bar = severity_bar(summary.severity_breakdown.low, max_severity, "█", "░");
    let low_count = summary.severity_breakdown.low.to_string();
    let medium_line = format!(
        "{}  {}  {}    {}  {}  {}",
        color_medium("MEDIUM"),
        medium_bar,
        color_value(&medium_count),
        color_low("LOW"),
        low_bar,
        color_value(&low_count),
    );
    output.push_str(&format!("{}\n", box_line(&medium_line, width)));

    // Empty line
    output.push_str(&format!("{}\n", empty_box_line(width)));

    // Middle divider
    output.push_str(&format!(
        "{}{}{}    \n",
        color_border("├"),
        divider(width.saturating_sub(4)),
        color_border("┤")
    ));

    // Empty line
    output.push_str(&format!("{}\n", empty_box_line(width)));

    // Status line
    let (status_icon, status_color_text) = if summary.violations == 0 {
        (ICON_PASS, color_success("PASS — all checks passed"))
    } else {
        (
            ICON_CRITICAL,
            color_failure("FAIL — violations found at or above 'low' threshold"),
        )
    };

    let status_line = format!(
        "{}  {} {}",
        color_label("Status"),
        status_icon,
        status_color_text
    );
    output.push_str(&format!("{}\n", box_line(&status_line, width)));

    // Empty line
    output.push_str(&format!("{}\n", empty_box_line(width)));

    // Bottom border
    output.push_str(&format!(
        "{}{}{}\n",
        color_border("╰"),
        divider(width.saturating_sub(2)),
        color_border("╯")
    ));

    output
}

/// Render a simple success message when no violations found.
///
/// # Returns
/// A formatted success message
#[allow(dead_code)]
pub fn render_no_violations() -> String {
    let _width = term_width();
    let mut output = String::new();

    output.push('\n');
    output.push_str(&format!(
        "  {}\n",
        color_success(&format!(
            "{} No violations found. All checks passed.",
            ICON_PASS
        ))
    ));

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_summary_with_violations() {
        let summary = AnalysisSummary {
            target: "./contracts/Token.sol".to_string(),
            chain: "EVM".to_string(),
            total_checks: 47,
            violations: 3,
            passed: 44,
            suppressed: 0,
            duration_secs: 1.24,
            severity_breakdown: SeverityBreakdown {
                critical: 1,
                high: 1,
                medium: 1,
                low: 0,
            },
        };

        let rendered = render_summary(&summary);
        assert!(rendered.contains("Analysis Summary"));
        assert!(rendered.contains("./contracts/Token.sol"));
        assert!(rendered.contains("EVM"));
        assert!(rendered.contains("47"));
        assert!(rendered.contains("3"));
        assert!(rendered.contains("44"));
        assert!(rendered.contains("1.24"));
        assert!(rendered.contains("FAIL"));
    }

    #[test]
    fn test_render_summary_no_violations() {
        let summary = AnalysisSummary {
            target: "./contracts/Safe.sol".to_string(),
            chain: "EVM".to_string(),
            total_checks: 100,
            violations: 0,
            passed: 100,
            suppressed: 0,
            duration_secs: 2.5,
            severity_breakdown: SeverityBreakdown {
                critical: 0,
                high: 0,
                medium: 0,
                low: 0,
            },
        };

        let rendered = render_summary(&summary);
        assert!(rendered.contains("Analysis Summary"));
        assert!(rendered.contains("PASS"));
        assert!(rendered.contains("100"));
    }

    #[test]
    fn test_render_no_violations_message() {
        let msg = render_no_violations();
        assert!(msg.contains("No violations found"));
        assert!(msg.contains("All checks passed"));
    }
}
