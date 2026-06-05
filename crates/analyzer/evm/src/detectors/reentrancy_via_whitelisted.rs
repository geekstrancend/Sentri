/// EVM Reentrancy via Whitelisted Contract Detector
///
/// Detects H29 (Penpie $27M) vulnerability: Reentrancy through whitelisted token/contract calls
///
/// The vulnerability occurs when:
/// 1. Function transfers whitelisted tokens without checking for reentrancy
/// 2. Whitelisted contract is assumed safe but contains malicious fallback/hooks
/// 3. Reentrancy guards only check for external callers, not whitelisted contracts
/// 4. Attacker can create malicious token that reenters via callback
///
/// Example Vulnerable Pattern:
/// ```solidity
/// function withdraw(uint amount) external nonReentrant {
///     require(whitelistedTokens[token], "Token not whitelisted");
///     token.transfer(msg.sender, amount);  // Assumes whitelisted = safe
///     balances[msg.sender] -= amount;
/// }
/// ```
///
/// Safe Pattern:
/// ```solidity
/// function withdraw(uint amount) external nonReentrant {
///     require(whitelistedTokens[token], "Token not whitelisted");
///     balances[msg.sender] -= amount;  // Update state FIRST
///     token.transfer(msg.sender, amount);  // Then transfer (CEI pattern)
/// }
/// ```

use lazy_static::lazy_static;
use regex::Regex;
use sentri_core::Finding;

lazy_static! {
    static ref TRANSFER_PATTERN: Regex = Regex::new(
        r"(?i)(transfer|transferFrom|safeTransfer|safeTransferFrom|send|call)\s*\(\s*(?:to|recipient|msg\.sender|user)"
    ).unwrap();
    static ref WHITELIST_CHECK: Regex =
        Regex::new(r"(?i)require\s*\(\s*(?:whitelisted|approved|verified)\s*\[")
            .unwrap();
    static ref STATE_UPDATE_PATTERN: Regex = Regex::new(
        r"(?i)(balances|amount|supply|shares|value)\s*\[\s*\w+\s*\]\s*(-=|\+=|=\s*0|=\s*\w+\s*-)"
    ).unwrap();
    static ref TRANSFER_AFTER_BEFORE: Regex =
        Regex::new(r"transfer.*?balances|transfer.*?amount.*?-=")
            .unwrap();
    static ref STATE_BEFORE_TRANSFER: Regex = Regex::new(r"balances.*?-=.*?transfer|amount.*?-=.*?transfer").unwrap();
    static ref REENTRANCY_GUARD: Regex = Regex::new(r"(?i)nonReentrant|noReentrant|ReentrancyGuard").unwrap();
}

pub fn detect_reentrancy_via_whitelisted(source: &str, file_path: &str) -> Vec<Finding> {
    let mut findings = Vec::new();

    for (line_num, line) in source.lines().enumerate() {
        // Skip comments
        if line.trim().starts_with("//") {
            continue;
        }

        // Look for transfer operations
        if !TRANSFER_PATTERN.is_match(line) {
            continue;
        }

        // Extract function context (200 lines backward)
        let start = if line_num > 200 { line_num - 200 } else { 0 };
        let end = std::cmp::min(line_num + 50, source.lines().count());
        let function_context = source
            .lines()
            .skip(start)
            .take(end - start)
            .collect::<Vec<_>>()
            .join("\n");

        // Check for whitelist check
        let has_whitelist = WHITELIST_CHECK.is_match(&function_context);
        if !has_whitelist {
            continue;
        }

        // Check CEI pattern: state update BEFORE transfer
        let has_proper_order = STATE_BEFORE_TRANSFER.is_match(&function_context);

        // Check for reentrancy guard
        let has_guard = REENTRANCY_GUARD.is_match(&function_context);

        if !has_proper_order {
            findings.push(
                Finding::new(
                    "evm_reentrancy_via_whitelisted".to_string(),
                    sentri_core::Severity::Critical,
                    file_path.to_string(),
                    line_num + 1,
                    0,
                    "Transfer of whitelisted token happens before state update. \
                     Vulnerable to reentrancy even though token is whitelisted (whitelist doesn't guarantee safety)."
                        .to_string(),
                    line.trim().to_string(),
                )
                .with_metadata("exploit_id", "H29")
                .with_metadata("exploit_name", "Penpie Whitelisted Reentrancy")
                .with_metadata("loss", "$27M")
                .with_metadata("year", "2023")
                .with_metadata("vulnerability_type", "reentrancy")
                .with_metadata("detector", "pattern_analysis")
                .with_metadata("remediation", "Update state before transfer (CEI pattern)"),
            );
        } else if !has_guard {
            // Has proper order but no reentrancy guard
            findings.push(
                Finding::new(
                    "evm_reentrancy_via_whitelisted".to_string(),
                    sentri_core::Severity::High,
                    file_path.to_string(),
                    line_num + 1,
                    0,
                    "Transfer of whitelisted token lacks reentrancy guard. \
                     While CEI pattern is used, consider adding nonReentrant modifier for defense in depth."
                        .to_string(),
                    line.trim().to_string(),
                )
                .with_metadata("exploit_id", "H29")
                .with_metadata("exploit_name", "Penpie - Weak Guard")
                .with_metadata("loss", "$27M")
                .with_metadata("year", "2023")
                .with_metadata("vulnerability_type", "reentrancy")
                .with_metadata("detector", "pattern_analysis")
                .with_metadata("remediation", "Add nonReentrant modifier"),
            );
        }
    }

    findings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_transfer_before_state_update() {
        let vulnerable = r#"
        function withdraw(uint amount) external {
            require(whitelistedTokens[tokenAddress], "Token not whitelisted");
            tokenAddress.transfer(msg.sender, amount);  // Transfer BEFORE state update!
            balances[msg.sender] -= amount;
        }
        "#;

        let findings = detect_reentrancy_via_whitelisted(vulnerable, "test.sol");
        assert!(!findings.is_empty(), "Should detect transfer before state update");
        assert_eq!(
            findings[0].metadata.get("exploit_id"),
            Some(&"H29".to_string())
        );
    }

    #[test]
    fn test_safe_cei_pattern() {
        let safe = r#"
        function withdraw(uint amount) external nonReentrant {
            require(whitelistedTokens[tokenAddress], "Token not whitelisted");
            balances[msg.sender] -= amount;  // State update FIRST
            tokenAddress.transfer(msg.sender, amount);  // Then transfer
        }
        "#;

        let findings = detect_reentrancy_via_whitelisted(safe, "test.sol");
        let critical_findings: Vec<_> =
            findings.iter().filter(|f| f.severity == sentri_core::Severity::Critical).collect();
        assert!(
            critical_findings.is_empty(),
            "Should allow proper CEI pattern with guard"
        );
    }

    #[test]
    fn test_proper_order_without_guard() {
        let weak = r#"
        function withdraw(uint amount) external {
            require(whitelistedTokens[tokenAddress], "Token not whitelisted");
            balances[msg.sender] -= amount;  // State update first
            tokenAddress.transfer(msg.sender, amount);  // Then transfer (but no guard)
        }
        "#;

        let findings = detect_reentrancy_via_whitelisted(weak, "test.sol");
        let high_findings: Vec<_> =
            findings.iter().filter(|f| f.severity == sentri_core::Severity::High).collect();
        assert!(
            !high_findings.is_empty(),
            "Should flag missing reentrancy guard despite proper order"
        );
    }

    #[test]
    fn test_no_whitelist_check_ignored() {
        let ignored = r#"
        function transfer(address to, uint amount) external {
            balances[msg.sender] -= amount;
            tokenAddress.transfer(to, amount);
        }
        "#;

        let findings = detect_reentrancy_via_whitelisted(ignored, "test.sol");
        assert!(
            findings.is_empty(),
            "Should ignore transfers without whitelist check"
        );
    }

    #[test]
    fn test_penpie_pattern() {
        let penpie_vulnerable = r#"
        function withdraw(uint amount) external {
            require(whitelistedTokens[msg.sender], "Not whitelisted");
            _vault.transfer(msg.sender, amount);
            userDeposits[msg.sender] -= amount;
        }
        "#;

        let findings = detect_reentrancy_via_whitelisted(penpie_vulnerable, "test.sol");
        assert!(
            !findings.is_empty(),
            "Should detect Penpie-style reentrancy vulnerability"
        );
    }

    #[test]
    fn test_safe_transfer_from() {
        let safe = r#"
        function withdraw(uint amount) external nonReentrant {
            require(whitelistedTokens[token], "Not whitelisted");
            balances[msg.sender] -= amount;
            token.safeTransfer(msg.sender, amount);
        }
        "#;

        let findings = detect_reentrancy_via_whitelisted(safe, "test.sol");
        let critical_findings: Vec<_> =
            findings.iter().filter(|f| f.severity == sentri_core::Severity::Critical).collect();
        assert!(critical_findings.is_empty(), "Safe pattern should not trigger");
    }
}
