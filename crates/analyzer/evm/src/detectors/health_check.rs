//! Detector for missing post-state health checks in lending protocols.
//!
//! This detector identifies when state-modifying functions in lending/collateral
//! contracts don't verify account health after state updates. This pattern was
//! exploited in H19 Euler Finance ($197M, 2023) and H11 Cream Finance V2 ($130M, 2021).

use lazy_static::lazy_static;
use regex::Regex;
use sentri_core::Finding;

lazy_static! {
    /// Patterns that indicate a function modifies critical financial state
    static ref STATE_MODIFICATION_PATTERNS: Vec<&'static str> = vec![
        "collateral",
        "debt",
        "borrow",
        "principal",
        "balance",
        "reserve",
        "supply",
        "borrowed",
    ];

    /// Patterns that indicate health check verification
    static ref HEALTH_CHECK_PATTERNS: [&'static str; 12] = [
        "checkHealth",
        "isHealthy",
        "health",
        "checkLiquidity",
        "checkCollateral",
        "liquidity",
        "getHealthFactor",
        "healthFactor",
        "require.*health",
        "require.*liquidity",
        "MIN_HEALTH",
        "healthThreshold",
    ];

    /// Regex for external/public functions
    static ref FUNCTION_REGEX: Regex =
        Regex::new(r"(?i)\bfunction\s+\w+\s*\([^)]*\)\s*(public|external)").unwrap();

    /// Regex for state modifications
    static ref STATE_MOD_REGEX: Regex =
        Regex::new(r"(?i)collateral|debt|borrow|reserve|balance").unwrap();
}

/// Detects missing health checks after state modifications in lending protocols.
pub fn detect_missing_health_check(source: &str, file_path: &str) -> Vec<Finding> {
    let mut findings = Vec::new();

    // Pattern 1: Identify if this is a lending-like contract or has state modifications we care about
    let is_lending = source.to_lowercase().contains("collateral")
        || source.to_lowercase().contains("health")
        || source.to_lowercase().contains("borrow")
        || source.to_lowercase().contains("lend")
        || source.to_lowercase().contains("liquidat");

    if !is_lending {
        return findings;
    }

    // Pattern 2: Find state-modifying functions
    for (func_line_num, func_line) in source.lines().enumerate() {
        if !FUNCTION_REGEX.is_match(func_line) {
            continue;
        }

        let func_name = extract_function_name(func_line);
        if func_name.is_empty() {
            continue;
        }

        // Extract function body (simplified - look ahead ~50 lines)
        let func_start = func_line_num;
        let func_end = (func_line_num + 50).min(source.lines().count());
        let func_body = source
            .lines()
            .skip(func_start)
            .take(func_end - func_start)
            .collect::<Vec<&str>>()
            .join("\n");

        // Pattern 3: Check if function modifies critical state
        let modifies_state = STATE_MOD_REGEX.is_match(&func_body);
        if !modifies_state {
            continue;
        }

        // Pattern 4: Check if function has health check before return
        if !has_health_check_before_return(&func_body) {
            let message = format!(
                "Missing health check after state modification in {}(). \
                 Function modifies collateral/debt/balance but does not verify \
                 account health before returning. This pattern was exploited in \
                 H19 Euler Finance ($197M) when users could donate collateral \
                 without triggering a health check, then self-liquidate at profit. \
                 \
                 Required fix: Add health check validation (e.g., require(isHealthy(msg.sender))) \
                 before the function returns.",
                func_name
            );

            findings.push(
                Finding::new(
                    "evm_missing_post_state_health_check".to_string(),
                    sentri_core::Severity::Critical,
                    file_path.to_string(),
                    func_line_num + 1,
                    0,
                    message,
                    func_line.trim().to_string(),
                )
                .with_metadata("exploit_id".to_string(), "H19".to_string())
                .with_metadata("exploit_name".to_string(), "Euler Finance".to_string())
                .with_metadata("loss".to_string(), "$197M".to_string())
                .with_metadata("year".to_string(), "2023".to_string())
                .with_metadata(
                    "also_affects".to_string(),
                    "H11 Cream Finance V2 ($130M, 2021)".to_string(),
                )
                .with_metadata("detector".to_string(), "pattern_analysis".to_string())
                .with_source_fragment(func_body),
            );
        }
    }

    findings
}

/// Check if contract contains lending-related functions
fn is_lending_like_contract(source: &str) -> bool {
    let lending_functions = vec![
        "borrow",
        "liquidate",
        "deposit",
        "withdraw",
        "repay",
        "lend",
        "collateral",
        "health",
        "liquidation",
    ];

    let source_lower = source.to_lowercase();
    lending_functions
        .iter()
        .filter(|func| source_lower.contains(**func))
        .count()
        >= 2
}

/// Extract function name from function declaration
fn extract_function_name(line: &str) -> String {
    if let Some(start) = line.find("function ") {
        let after_function = &line[start + 9..];
        if let Some(end) = after_function.find('(') {
            return after_function[..end].trim().to_string();
        }
    }
    String::new()
}

/// Check if function has health check before return statements
fn has_health_check_before_return(func_body: &str) -> bool {
    let func_lower = func_body.to_lowercase();

    // Check for any health-related patterns
    let health_patterns = vec![
        "checkhealthor_status",
        "isHealthy",
        "health",
        "checkLiquidity",
        "checkCollateral",
        "liquidity",
        "getHealthFactor",
        "healthFactor",
        "require",
        "MIN_HEALTH",
        "healthThreshold",
    ];

    for pattern in health_patterns {
        if func_lower.contains(&pattern.to_lowercase()) {
            // If it has "require" combined with "health" or "liquidity", count it
            if pattern == "require" {
                if func_lower.contains("require")
                    && (func_lower.contains("health") || func_lower.contains("liquidity"))
                {
                    return true;
                }
            } else {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vulnerable_donation_pattern_euler() {
        let code = r#"
            contract Euler {
                mapping(address => uint) collateral;
                mapping(address => uint) debt;

                function donateToReserves(uint amount) external {
                    require(msg.sender == owner, "Not owner");
                    collateral[msg.sender] -= amount;
                    reserves.balance += amount;
                    // MISSING: require(isHealthy(msg.sender))
                }
            }
        "#;

        let findings = detect_missing_health_check(code, "test.sol");
        assert!(!findings.is_empty(), "Should detect missing health check");
        assert_eq!(findings[0].severity, sentri_core::Severity::Critical);
        assert!(findings[0].invariant_id.contains("health_check"));
    }

    #[test]
    fn test_safe_with_health_check() {
        let code = r#"
            contract SafeProtocol {
                function deposit(uint amount) external {
                    collateral[msg.sender] += amount;
                    reserves -= amount;
                    require(
                        calculateHealthFactor(msg.sender) >= MIN_HEALTH,
                        "Account would be unhealthy"
                    );
                }
            }
        "#;

        let findings = detect_missing_health_check(code, "test.sol");
        assert!(
            findings.is_empty(),
            "Should not flag when health check present"
        );
    }

    #[test]
    fn test_safe_with_liquidity_check() {
        let code = r#"
            contract SafeProtocol {
                function borrow(uint amount) external {
                    debt[msg.sender] += amount;
                    _checkLiquidity(msg.sender);
                }
            }
        "#;

        let findings = detect_missing_health_check(code, "test.sol");
        assert!(findings.is_empty(), "Should not flag with _checkLiquidity");
    }

    #[test]
    fn test_non_financial_contract() {
        let code = r#"
            contract ERC20 {
                function transfer(address to, uint amount) external {
                    balances[msg.sender] -= amount;
                    balances[to] += amount;
                }
            }
        "#;

        let findings = detect_missing_health_check(code, "test.sol");
        assert!(findings.is_empty(), "Should not flag non-lending contracts");
    }

    #[test]
    fn test_safe_with_implicit_health_check() {
        let code = r#"
            contract SafeProtocol {
                function donateToReserves(uint amount) external {
                    collateral[msg.sender] -= amount;
                    reserves += amount;
                    require(isHealthy(msg.sender), "Health check failed");
                }
            }
        "#;

        let findings = detect_missing_health_check(code, "test.sol");
        assert!(
            findings.is_empty(),
            "Should detect implicit isHealthy check"
        );
    }
}
