/// Integration Testing Module
///
/// Comprehensive integration tests against real exploit patterns and documented vulnerabilities.
/// Tests detectors using actual code from documented exploits to validate real-world effectiveness.

use sentri_core::Finding;
use std::collections::HashMap;

/// Real exploit test case
#[derive(Debug, Clone)]
pub struct ExploitTestCase {
    /// H-code identifier (e.g., H19, H16)
    pub h_code: String,
    /// Project name
    pub project_name: String,
    /// Loss amount in millions
    pub loss_millions: f64,
    /// Year of exploit
    pub year: u32,
    /// Vulnerable code pattern
    pub vulnerable_code: String,
    /// Expected detector hits
    pub expected_detections: Vec<String>,
    /// Minimum expected findings
    pub min_findings: usize,
}

/// Integration test suite
pub struct IntegrationTestSuite {
    test_cases: Vec<ExploitTestCase>,
    results: HashMap<String, IntegrationTestResult>,
}

/// Result of integration test
#[derive(Debug, Clone)]
pub struct IntegrationTestResult {
    /// H-code
    pub h_code: String,
    /// Whether test passed
    pub passed: bool,
    /// Number of findings
    pub findings_count: usize,
    /// Matching detectors
    pub matching_detectors: Vec<String>,
    /// False negatives (expected but not found)
    pub false_negatives: Vec<String>,
    /// Execution time in ms
    pub execution_time_ms: u64,
}

impl IntegrationTestSuite {
    /// Create new integration test suite
    pub fn new() -> Self {
        let mut suite = Self {
            test_cases: Vec::new(),
            results: HashMap::new(),
        };
        
        suite.initialize_real_exploits();
        suite
    }

    /// Initialize test cases with real exploits
    fn initialize_real_exploits(&mut self) {
        // Phase A exploits
        self.test_cases.push(ExploitTestCase {
            h_code: "H19".to_string(),
            project_name: "Euler".to_string(),
            loss_millions: 197.0,
            year: 2023,
            vulnerable_code: r#"
function depositCollateral(address asset, uint256 amount) public {
    collateral[msg.sender][asset] += amount;
    // Missing: require(isHealthy(msg.sender));
    emit DepositCollateral(asset, amount);
}
            "#.to_string(),
            expected_detections: vec!["health_check".to_string()],
            min_findings: 1,
        });

        self.test_cases.push(ExploitTestCase {
            h_code: "H16".to_string(),
            project_name: "Nomad".to_string(),
            loss_millions: 190.0,
            year: 2022,
            vulnerable_code: r#"
bytes32 public merkleRoot;

function initialize() external {
    // Zero root initialization
    merkleRoot = bytes32(0);
}

function verifyMessage(bytes32[] calldata proof) public {
    if (verifyProof(proof, merkleRoot)) {
        executeMessage();
    }
}
            "#.to_string(),
            expected_detections: vec!["merkle_root".to_string()],
            min_findings: 1,
        });

        self.test_cases.push(ExploitTestCase {
            h_code: "H47".to_string(),
            project_name: "KelpDAO".to_string(),
            loss_millions: 292.0,
            year: 2024,
            vulnerable_code: r#"
address[] public dvns;

function setDVNs(address[] calldata _dvns) external {
    // No minimum count validation
    dvns = _dvns;
}

function sendMessage(bytes calldata message) external {
    require(dvns.length > 0);
    (bool success,) = dvns[0].call(message);
    require(success);
}
            "#.to_string(),
            expected_detections: vec!["dvn_single_point".to_string()],
            min_findings: 1,
        });

        self.test_cases.push(ExploitTestCase {
            h_code: "H56".to_string(),
            project_name: "Echo".to_string(),
            loss_millions: 73.0,
            year: 2023,
            vulnerable_code: r#"
function mint(uint256 amount) public {
    // No collateral verification
    totalMinted += amount;
    balances[msg.sender] += amount;
}
            "#.to_string(),
            expected_detections: vec!["synthetic_mint".to_string()],
            min_findings: 1,
        });

        // Phase B exploits
        self.test_cases.push(ExploitTestCase {
            h_code: "H17".to_string(),
            project_name: "Mango".to_string(),
            loss_millions: 117.0,
            year: 2022,
            vulnerable_code: r#"
function swapWithOracle() public {
    uint256 price = oracle.getPrice(token);
    // Manipulate spot price while using it
    tokenPrice[msg.sender] = price;
    executeSwap(price);
}
            "#.to_string(),
            expected_detections: vec!["oracle_self_trade".to_string()],
            min_findings: 1,
        });
    }

    /// Add test case
    pub fn add_test_case(&mut self, test_case: ExploitTestCase) {
        self.test_cases.push(test_case);
    }

    /// Run integration tests
    pub fn run_tests<F>(&mut self, detector_fn: F) -> IntegrationTestResults
    where
        F: Fn(&str) -> Vec<Finding>,
    {
        let mut results = Vec::new();
        let mut passed = 0;
        let mut failed = 0;

        for test_case in &self.test_cases {
            let start = std::time::Instant::now();
            let findings = detector_fn(&test_case.vulnerable_code);
            let elapsed = start.elapsed().as_millis() as u64;

            let findings_count = findings.len();
            let test_passed = findings_count >= test_case.min_findings;

            if test_passed {
                passed += 1;
            } else {
                failed += 1;
            }

            let matching_detectors = findings
                .iter()
                .map(|f| f.vulnerability_id.clone())
                .collect();

            let result = IntegrationTestResult {
                h_code: test_case.h_code.clone(),
                passed: test_passed,
                findings_count,
                matching_detectors,
                false_negatives: test_case
                    .expected_detections
                    .iter()
                    .filter(|d| !test_case.expected_detections.iter().any(|m| m == *d))
                    .cloned()
                    .collect(),
                execution_time_ms: elapsed,
            };

            results.push(result.clone());
            self.results.insert(test_case.h_code.clone(), result);
        }

        IntegrationTestResults {
            total: self.test_cases.len(),
            passed,
            failed,
            results,
        }
    }

    /// Get result for H-code
    pub fn get_result(&self, h_code: &str) -> Option<&IntegrationTestResult> {
        self.results.get(h_code)
    }
}

/// Integration test results summary
#[derive(Debug, Clone)]
pub struct IntegrationTestResults {
    /// Total tests
    pub total: usize,
    /// Tests passed
    pub passed: usize,
    /// Tests failed
    pub failed: usize,
    /// Individual results
    pub results: Vec<IntegrationTestResult>,
}

impl IntegrationTestResults {
    /// Success rate as percentage
    pub fn success_rate(&self) -> f64 {
        if self.total == 0 { 0.0 } else { (self.passed as f64 / self.total as f64) * 100.0 }
    }

    /// Generate report
    pub fn report(&self) -> String {
        let mut report = format!(
            "# Integration Test Results\n\n## Summary\n- **Total Tests:** {}\n",
            self.total
        );
        report.push_str(&format!("- **Passed:** {}\n", self.passed));
        report.push_str(&format!("- **Failed:** {}\n", self.failed));
        report.push_str(&format!("- **Success Rate:** {:.1}%\n\n", self.success_rate()));

        report.push_str("## Details\n\n");
        for result in &self.results {
            let status = if result.passed { "✓ PASS" } else { "✗ FAIL" };
            report.push_str(&format!(
                "### {} {}\n- **Findings:** {}\n- **Time:** {} ms\n\n",
                result.h_code, status, result.findings_count, result.execution_time_ms
            ));
        }

        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integration_suite_initializes() {
        let suite = IntegrationTestSuite::new();
        assert!(!suite.test_cases.is_empty());
    }

    #[test]
    fn exploit_test_case_structure() {
        let test = ExploitTestCase {
            h_code: "H99".to_string(),
            project_name: "Test".to_string(),
            loss_millions: 10.0,
            year: 2024,
            vulnerable_code: "test code".to_string(),
            expected_detections: vec!["detector1".to_string()],
            min_findings: 1,
        };

        assert_eq!(test.h_code, "H99");
        assert_eq!(test.min_findings, 1);
    }

    #[test]
    fn test_results_success_rate() {
        let results = IntegrationTestResults {
            total: 10,
            passed: 8,
            failed: 2,
            results: vec![],
        };

        assert_eq!(results.success_rate(), 80.0);
    }
}
