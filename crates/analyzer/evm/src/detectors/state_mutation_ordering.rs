/// EVM State Mutation Ordering Detector
///
/// Detects H45 vulnerability: Unsafe state mutation order leading to exploits
///
/// The vulnerability occurs when:
/// 1. State is updated after external calls
/// 2. Reentrancy possible before state is finalized
/// 3. Incorrect order exposes intermediate states
/// 4. Can lead to double-spending or fund loss
///

use lazy_static::lazy_static;
use regex::Regex;
use sentri_core::Finding;

lazy_static! {
    static ref EXTERNAL_CALL: Regex = 
        Regex::new(r"(?i)\.call\(|\.delegatecall\(|\.transfer\(|\.send\(|safeTransfer|safeTransferFrom").unwrap();
    static ref STATE_UPDATE: Regex = 
        Regex::new(r"(?i)(balances|amounts|supply|shares|reserves)\s*\[\s*\w+\s*\]\s*(-=|\+=|=)").unwrap();
    static ref EXTERNAL_CALL_BEFORE_STATE: Regex = 
        Regex::new(r"(?i)\.call.*?balances|transfer.*?balances\[.*?\]\s*-=").unwrap();
}

pub fn detect_state_mutation_ordering(source: &str, file_path: &str) -> Vec<Finding> {
    let mut findings = Vec::new();

    for (line_num, line) in source.lines().enumerate() {
        if line.trim().starts_with("//") || !EXTERNAL_CALL.is_match(line) {
            continue;
        }

        let context_end = std::cmp::min(line_num + 100, source.lines().count());
        let function_body = source
            .lines()
            .skip(line_num)
            .take(context_end - line_num)
            .collect::<Vec<_>>()
            .join("\n");

        if EXTERNAL_CALL_BEFORE_STATE.is_match(&function_body) {
            findings.push(
                Finding::new(
                    "evm_state_mutation_ordering".to_string(),
                    sentri_core::Severity::Medium,
                    file_path.to_string(),
                    line_num + 1,
                    0,
                    "External call precedes state update. Reorder to update state before calling external functions (CEI pattern).".to_string(),
                    line.trim().to_string(),
                )
                .with_metadata("exploit_id", "H45")
                .with_metadata("exploit_name", "State Mutation Order")
                .with_metadata("loss", "$2.6M")
                .with_metadata("year", "2023")
                .with_metadata("vulnerability_type", "ordering_vulnerability")
                .with_metadata("detector", "pattern_analysis")
                .with_metadata("remediation", "Update state before external calls (CEI)"),
            );
        }
    }

    findings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_call_before_state_update() {
        let vulnerable = r#"
        function withdraw(uint amount) external {
            (bool success, ) = msg.sender.call{value: amount}("");
            require(success, "Transfer failed");
            balances[msg.sender] -= amount;  // State update AFTER call!
        }
        "#;
        let findings = detect_state_mutation_ordering(vulnerable, "test.sol");
        assert!(!findings.is_empty());
    }

    #[test]
    fn test_correct_ordering() {
        let safe = r#"
        function withdraw(uint amount) external {
            balances[msg.sender] -= amount;  // Update state FIRST
            (bool success, ) = msg.sender.call{value: amount}("");
            require(success, "Transfer failed");
        }
        "#;
        let findings = detect_state_mutation_ordering(safe, "test.sol");
        assert!(findings.is_empty());
    }

    #[test]
    fn test_transfer_before_balance_update() {
        let vulnerable = r#"
        function swap(address token, uint amount) external {
            token.transfer(msg.sender, amount);
            balances[msg.sender] -= amount;
        }
        "#;
        let findings = detect_state_mutation_ordering(vulnerable, "test.sol");
        assert!(!findings.is_empty());
    }

    #[test]
    fn test_safe_transfer_pattern() {
        let safe = r#"
        function swap(address token, uint amount) external {
            balances[msg.sender] -= amount;
            token.transfer(msg.sender, amount);
        }
        "#;
        let findings = detect_state_mutation_ordering(safe, "test.sol");
        assert!(findings.is_empty());
    }

    #[test]
    fn test_safe_transfer_from() {
        let safe = r#"
        function deposit(address token, uint amount) external {
            balances[msg.sender] += amount;
            token.transferFrom(msg.sender, address(this), amount);
        }
        "#;
        let findings = detect_state_mutation_ordering(safe, "test.sol");
        assert!(findings.is_empty());
    }
}
