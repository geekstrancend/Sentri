//! Component for displaying passed checks in verbose mode.

use crate::ui::constants::{color_dim, color_success, ICON_PASS};

/// Render passed checks in a compact multi-column layout.
///
/// Shows passed checks in up to 3 columns when `--verbose` is used.
/// Only displayed if there are passed checks to show.
///
/// # Arguments
/// * `passed_checks` - List of check names that passed
///
/// # Returns
/// The formatted passed checks display, or empty string if none
pub fn render_passed_checks(passed_checks: &[String]) -> String {
    if passed_checks.is_empty() {
        return String::new();
    }

    let mut output = String::new();
    output.push('\n');
    output.push_str(&format!(
        "{}  ({})\n\n",
        "Passed Checks",
        passed_checks.len()
    ));

    // Calculate column width based on longest check name
    let max_width = passed_checks.iter().map(|c| c.len()).max().unwrap_or(20) + 6;
    let num_cols = 3;

    // Display in columns
    for (idx, check) in passed_checks.iter().enumerate() {
        let formatted = format!("{}  {}", color_success(ICON_PASS), color_dim(check));

        // Print with padding
        if (idx + 1) % num_cols == 0 {
            // End of line
            output.push_str(&format!("{}\n", formatted));
        } else {
            // Middle of line - pad
            let padding = max_width.saturating_sub(check.len() + 3);
            output.push_str(&formatted);
            output.push_str(&" ".repeat(padding));
        }
    }

    // Final newline if needed
    if !passed_checks.is_empty() && !passed_checks.len().is_multiple_of(num_cols) {
        output.push('\n');
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_passed_checks_empty() {
        let checks: Vec<String> = vec![];
        let output = render_passed_checks(&checks);
        assert_eq!(output, "");
    }

    #[test]
    fn test_passed_checks_single() {
        let checks = vec!["balance_conservation".to_string()];
        let output = render_passed_checks(&checks);
        assert!(output.contains("Passed Checks"));
        assert!(output.contains("balance_conservation"));
    }

    #[test]
    fn test_passed_checks_multiple() {
        let checks = vec![
            "balance_conservation".to_string(),
            "no_negative_balance".to_string(),
            "access_control".to_string(),
        ];
        let output = render_passed_checks(&checks);
        assert!(output.contains("Passed Checks"));
        assert_eq!(output.matches("✓").count(), 3);
    }
}
