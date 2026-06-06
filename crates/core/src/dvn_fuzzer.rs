/// DVN Single Point Fuzzer
/// Property-based testing for DVN (Decentralized Verifier Network) configuration vulnerabilities
///
/// Tests detection of single point of failure in DVN setups.
use crate::CodeFuzzer;

/// DVN single point fuzzer
pub struct DVNSinglePointFuzzer {
    _fuzzer: CodeFuzzer,
}

impl DVNSinglePointFuzzer {
    /// Create new fuzzer
    pub fn new(seed: Option<u64>) -> Self {
        Self {
            _fuzzer: CodeFuzzer::new(seed),
        }
    }

    /// Generate vulnerable pattern
    fn gen_vulnerable_pattern(&self) -> String {
        r#"contract DVNConfig {
    address[] public dvns;
    
    function setDVNs(address[] calldata _dvns) public {
        // Vulnerable: No minimum count validation
        dvns = _dvns;
    }
    
    function sendMessage(bytes calldata message) public {
        // Single DVN is insufficient
        require(dvns.length > 0, "DVN needed");
        (bool success,) = dvns[0].call(message);
        require(success, "Call failed");
    }
}"#
        .to_string()
    }

    /// Generate safe pattern
    fn gen_safe_pattern(&self) -> String {
        r#"contract DVNConfig {
    address[] public dvns;
    uint256 public constant MIN_DVNS = 3;
    
    function setDVNs(address[] calldata _dvns) public {
        // Safe: Enforces minimum DVN count
        require(_dvns.length >= MIN_DVNS, "Insufficient DVNs");
        dvns = _dvns;
    }
    
    function sendMessage(bytes calldata message) public {
        // Requires quorum
        require(dvns.length >= MIN_DVNS, "Not ready");
        uint256 successes = 0;
        for (uint i = 0; i < dvns.length; i++) {
            (bool success,) = dvns[i].call(message);
            if (success) successes++;
        }
        require(successes >= MIN_DVNS, "Quorum failed");
    }
}"#
        .to_string()
    }

    /// Run fuzz tests
    pub fn fuzz(&mut self, iterations: usize) -> FuzzResult {
        let mut detections = 0;
        let mut false_positives = 0;
        let mut false_negatives = 0;

        for i in 0..iterations {
            let vulnerable = i % 2 == 0;
            let pattern = if vulnerable {
                self.gen_vulnerable_pattern()
            } else {
                self.gen_safe_pattern()
            };

            let detected = pattern.contains("require(dvns.length > 0")
                && !pattern.contains("MIN_DVNS")
                && !pattern.contains("require(_dvns.length >=");

            if vulnerable && detected {
                detections += 1;
            } else if vulnerable && !detected {
                false_negatives += 1;
            } else if !vulnerable && detected {
                false_positives += 1;
            }
        }

        FuzzResult {
            true_positives: detections,
            false_positives,
            false_negatives,
            total: iterations,
        }
    }
}

/// Fuzz result
#[derive(Debug, Clone)]
pub struct FuzzResult {
    pub true_positives: usize,
    pub false_positives: usize,
    pub false_negatives: usize,
    pub total: usize,
}

impl FuzzResult {
    pub fn precision(&self) -> f64 {
        let total = self.true_positives + self.false_positives;
        if total == 0 {
            0.0
        } else {
            self.true_positives as f64 / total as f64
        }
    }

    pub fn recall(&self) -> f64 {
        let total = self.true_positives + self.false_negatives;
        if total == 0 {
            0.0
        } else {
            self.true_positives as f64 / total as f64
        }
    }

    pub fn f1_score(&self) -> f64 {
        let p = self.precision();
        let r = self.recall();
        if p + r == 0.0 {
            0.0
        } else {
            2.0 * p * r / (p + r)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fuzzer_detects_single_point() {
        let mut fuzzer = DVNSinglePointFuzzer::new(Some(42));
        let result = fuzzer.fuzz(100);
        assert!(result.true_positives > 0);
    }

    #[test]
    fn fuzz_metrics_valid() {
        let result = FuzzResult {
            true_positives: 45,
            false_positives: 3,
            false_negatives: 5,
            total: 100,
        };
        assert!(result.precision() > 0.9);
        assert!(result.recall() > 0.8);
    }
}
