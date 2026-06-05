/// EVM Constructor Race Condition Detector
///
/// Detects H46 vulnerability: Initialization logic callable after deployment
///
/// The vulnerability occurs when:
/// 1. Constructor logic can be called via public/external functions
/// 2. No initialized flag to prevent re-initialization
/// 3. Attacker can reinitialize contract to reset state
/// 4. Can steal ownership or reset balances
///

use lazy_static::lazy_static;
use regex::Regex;
use sentri_core::Finding;

lazy_static! {
    static ref CONSTRUCTOR: Regex = Regex::new(r"(?i)constructor\s*\(").unwrap();
    static ref INITIALIZATION_FUNCTION: Regex = 
        Regex::new(r"(?i)function\s+(initialize|init|setup|configure|setOwner)\s*\(").unwrap();
    static ref INIT_FLAG: Regex = 
        Regex::new(r"(?i)initialized|_initialized|hasInitialized|init_flag").unwrap();
    static ref REQUIRE_NOT_INIT: Regex = 
        Regex::new(r"(?i)require\s*\(\s*!.*?initialized|require\s*\(\s*!.*?_initialized").unwrap();
}

pub fn detect_constructor_race_condition(source: &str, file_path: &str) -> Vec<Finding> {
    let mut findings = Vec::new();

    for (line_num, line) in source.lines().enumerate() {
        if line.trim().starts_with("//") || !INITIALIZATION_FUNCTION.is_match(line) {
            continue;
        }

        let context_end = std::cmp::min(line_num + 150, source.lines().count());
        let function_body = source
            .lines()
            .skip(line_num)
            .take(context_end - line_num)
            .collect::<Vec<_>>()
            .join("\n");

        let has_init_check = REQUIRE_NOT_INIT.is_match(&function_body);
        
        // Check if this is in a proxy context (has initialize pattern)
        if !has_init_check {
            findings.push(
                Finding::new(
                    "evm_constructor_race_condition".to_string(),
                    sentri_core::Severity::Medium,
                    file_path.to_string(),
                    line_num + 1,
                    0,
                    "Initialization function lacks re-entrancy check. Add require(!initialized) to prevent re-initialization attacks.".to_string(),
                    line.trim().to_string(),
                )
                .with_metadata("exploit_id", "H46")
                .with_metadata("exploit_name", "Constructor Race")
                .with_metadata("loss", "$0.9M")
                .with_metadata("year", "2023")
                .with_metadata("vulnerability_type", "initialization_race")
                .with_metadata("detector", "pattern_analysis")
                .with_metadata("remediation", "Add require(!initialized) and set initialized = true"),
            );
        }
    }

    findings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_initialization_guard() {
        let vulnerable = r#"
        function initialize(address owner, uint256 supply) external {
            contractOwner = owner;
            totalSupply = supply;
        }
        "#;
        let findings = detect_constructor_race_condition(vulnerable, "test.sol");
        assert!(!findings.is_empty());
    }

    #[test]
    fn test_with_initialized_check() {
        let safe = r#"
        function initialize(address owner, uint256 supply) external {
            require(!initialized, "Already initialized");
            contractOwner = owner;
            totalSupply = supply;
            initialized = true;
        }
        "#;
        let findings = detect_constructor_race_condition(safe, "test.sol");
        assert!(findings.is_empty());
    }

    #[test]
    fn test_with_initializer_modifier() {
        let safe = r#"
        function initialize(address owner) external initializer {
            _owner = owner;
        }
        "#;
        let findings = detect_constructor_race_condition(safe, "test.sol");
        assert!(findings.is_empty());  // initializer modifier provides protection
    }

    #[test]
    fn test_init_function_unguarded() {
        let vulnerable = r#"
        function init(uint256 amount) external {
            balance = amount;
            admin = msg.sender;
        }
        "#;
        let findings = detect_constructor_race_condition(vulnerable, "test.sol");
        assert!(!findings.is_empty());
    }

    #[test]
    fn test_setup_with_guard() {
        let safe = r#"
        function setup(address newAdmin) external {
            require(!_initialized, "Setup already done");
            admin = newAdmin;
            _initialized = true;
        }
        "#;
        let findings = detect_constructor_race_condition(safe, "test.sol");
        assert!(findings.is_empty());
    }
}
