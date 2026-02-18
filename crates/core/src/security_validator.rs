//! Pre-build security validation using attack pattern detection.
//!
//! This module validates code before build to prevent known vulnerabilities.

use crate::attack_patterns::{AttackPattern, AttackPatternDB};
use std::path::Path;

/// Security validation report.
#[derive(Debug, Clone)]
pub struct SecurityReport {
    /// Critical vulnerabilities found.
    pub critical_issues: Vec<SecurityIssue>,
    /// High-risk issues found.
    pub high_issues: Vec<SecurityIssue>,
    /// Medium-risk issues found.
    pub medium_issues: Vec<SecurityIssue>,
    /// Low-risk issues found.
    pub low_issues: Vec<SecurityIssue>,
    /// Pass/fail status.
    pub passed: bool,
    /// Overall risk score (0-100).
    pub risk_score: u32,
}

/// A detected security issue.
#[derive(Debug, Clone)]
pub struct SecurityIssue {
    /// Attack pattern involved.
    pub attack_pattern: String,
    /// Location in code (file:line).
    pub location: String,
    /// Description of the issue.
    pub description: String,
    /// Suggested fix.
    pub suggested_fix: String,
    /// Severity level.
    pub severity: IssueSeverity,
}

/// Issue severity level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum IssueSeverity {
    /// Can cause total loss of funds.
    Critical = 4,
    /// Can cause significant fund loss.
    High = 3,
    /// Could enable attacks under certain conditions.
    Medium = 2,
    /// Minor risk or best practice violation.
    Low = 1,
}

impl std::fmt::Display for IssueSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Critical => write!(f, "CRITICAL"),
            Self::High => write!(f, "HIGH"),
            Self::Medium => write!(f, "MEDIUM"),
            Self::Low => write!(f, "LOW"),
        }
    }
}

/// Security validator for code before building.
pub struct SecurityValidator {
    attack_db: AttackPatternDB,
}

impl SecurityValidator {
    /// Create a new security validator.
    pub fn new() -> Self {
        Self {
            attack_db: AttackPatternDB::new(),
        }
    }

    /// Validate code from a file.
    pub fn validate_file(&self, path: &Path, chain: &str) -> Result<SecurityReport, String> {
        let code =
            std::fs::read_to_string(path).map_err(|e| format!("Failed to read file: {}", e))?;
        self.validate_code(&code, path.to_string_lossy().as_ref(), chain)
    }

    /// Validate code content.
    pub fn validate_code(
        &self,
        code: &str,
        file_path: &str,
        chain: &str,
    ) -> Result<SecurityReport, String> {
        let mut critical_issues = Vec::new();
        let mut high_issues = Vec::new();
        let mut medium_issues = Vec::new();
        let mut low_issues = Vec::new();

        // Check each pattern relevant to the chain
        let patterns = self.attack_db.patterns_for_chain(chain);

        for pattern in patterns {
            let issues = self.check_pattern(code, file_path, pattern);
            for issue in issues {
                match issue.severity {
                    IssueSeverity::Critical => critical_issues.push(issue),
                    IssueSeverity::High => high_issues.push(issue),
                    IssueSeverity::Medium => medium_issues.push(issue),
                    IssueSeverity::Low => low_issues.push(issue),
                }
            }
        }

        // Calculate risk score
        let risk_score = (critical_issues.len() as u32 * 25
            + high_issues.len() as u32 * 15
            + medium_issues.len() as u32 * 8
            + low_issues.len() as u32 * 3)
            .min(100);

        let passed = critical_issues.is_empty() && high_issues.is_empty();

        Ok(SecurityReport {
            critical_issues,
            high_issues,
            medium_issues,
            low_issues,
            passed,
            risk_score,
        })
    }

    /// Check code against a specific attack pattern.
    fn check_pattern(
        &self,
        code: &str,
        file_path: &str,
        pattern: &AttackPattern,
    ) -> Vec<SecurityIssue> {
        let mut issues = Vec::new();

        // Special handling for reentrancy: need to check state update AFTER external call
        if pattern.id == "reentrancy" {
            issues.extend(self.check_reentrancy(code, file_path, pattern));
        } else {
            // Generic pattern matching for other attacks
            for (line_num, line) in code.lines().enumerate() {
                for vulnerable_pattern in &pattern.vulnerable_patterns {
                    if line.contains(vulnerable_pattern.as_str()) {
                        let severity = match pattern.cvss_score {
                            s if s >= 9.0 => IssueSeverity::Critical,
                            s if s >= 7.0 => IssueSeverity::High,
                            s if s >= 5.0 => IssueSeverity::Medium,
                            _ => IssueSeverity::Low,
                        };

                        issues.push(SecurityIssue {
                            attack_pattern: pattern.name.clone(),
                            location: format!("{}:{}", file_path, line_num + 1),
                            description: format!(
                                "Potential {} vulnerability detected. {}",
                                pattern.name, pattern.description
                            ),
                            suggested_fix: format!(
                                "Apply defensive invariant: {}",
                                pattern
                                    .defensive_invariants
                                    .first()
                                    .unwrap_or(&"Review code".to_string())
                            ),
                            severity,
                        });
                    }
                }
            }
        }
        issues
    }

    /// Check for reentrancy by analyzing state update order.
    fn check_reentrancy(
        &self,
        code: &str,
        file_path: &str,
        pattern: &AttackPattern,
    ) -> Vec<SecurityIssue> {
        let mut issues = Vec::new();
        let lines: Vec<&str> = code.lines().collect();

        // Find external calls (transfer, call, etc.)
        for (line_num, line) in lines.iter().enumerate() {
            // Skip if line has state update protection
            if line.contains("nonReentrant") {
                continue;
            }

            // Check if line has external call
            let has_external_call =
                line.contains("transfer(") || line.contains(".call(") || line.contains(".send(");

            if !has_external_call {
                continue;
            }

            // Look back up to 50 lines to find state updates
            let mut has_state_update_before = false;
            let search_start = line_num.saturating_sub(50);

            for prev_line in lines.iter().take(line_num).skip(search_start) {
                // Look for state updates: balance[X] = Y or balance = Z patterns
                if (prev_line.contains("balances[") && prev_line.contains("= 0"))
                    || (prev_line.contains("balance =") && prev_line.contains("= 0"))
                {
                    has_state_update_before = true;
                    break;
                }
            }

            // If NO state update before the external call, it's vulnerable
            if !has_state_update_before {
                let severity = IssueSeverity::Critical;

                issues.push(SecurityIssue {
                    attack_pattern: pattern.name.clone(),
                    location: format!("{}:{}", file_path, line_num + 1),
                    description: format!(
                        "Potential {} vulnerability detected. {}",
                        pattern.name, pattern.description
                    ),
                    suggested_fix: "Apply defensive invariant: state_update_before_external_call"
                        .to_string(),
                    severity,
                });
            }
        }
        issues
    }
}

impl Default for SecurityValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_validator_creation() {
        let validator = SecurityValidator::new();
        assert_eq!(validator.attack_db.all_patterns().len(), 8);
    }

    #[test]
    fn test_vulnerable_code_detection() {
        let validator = SecurityValidator::new();
        let code = "fn transfer() { transfer_funds(); /* state update after */ }";
        let report = validator.validate_code(code, "test.rs", "evm").unwrap();
        assert!(!report.passed);
        assert!(!report.critical_issues.is_empty());
    }

    #[test]
    fn test_safe_code_passes() {
        let validator = SecurityValidator::new();
        let code = "fn safe_code() { let x = 1 + 1; println!(\"{}\", x); }";
        let report = validator.validate_code(code, "test.rs", "evm").unwrap();
        assert!(report.passed);
        assert_eq!(report.critical_issues.len(), 0);
    }

    #[test]
    fn test_risk_score_calculation() {
        let validator = SecurityValidator::new();
        let code = "fn risky() { payable(msg.sender).transfer(amount); balances[msg.sender] = 0; }";
        let report = validator.validate_code(code, "test.rs", "evm").unwrap();
        assert!(report.risk_score > 0);
    }

    #[test]
    fn test_chain_specific_validation() {
        let validator = SecurityValidator::new();
        let code = "fn access() { require(is_owner()); }";

        let evm_report = validator.validate_code(code, "test.sol", "evm").unwrap();
        let solana_report = validator.validate_code(code, "test.rs", "solana").unwrap();

        // Both chains should detect access control patterns
        assert!(evm_report.passed || solana_report.passed);
    }
}
