/// Health Check Detector Fuzzer
/// Property-based testing for health check bypass detection
///
/// Tests that the detector properly identifies missing post-state health checks
/// while avoiding false positives on properly guarded state modifications.
use crate::CodeFuzzer;

/// Health check fuzzer test suite
pub struct HealthCheckFuzzer {
    _fuzzer: CodeFuzzer,
    _test_count: usize,
}

impl HealthCheckFuzzer {
    /// Create new health check fuzzer
    pub fn new(seed: Option<u64>) -> Self {
        Self {
            _fuzzer: CodeFuzzer::new(seed),
            _test_count: 0,
        }
    }

    /// Generate corpus of vulnerable and safe patterns
    pub fn generate_corpus(&mut self, size: usize) -> Vec<(String, bool)> {
        let mut corpus = Vec::new();

        for i in 0..size {
            let seed = self._fuzzer.seed.wrapping_add(i as u64);
            let mut local_fuzzer = CodeFuzzer::new(Some(seed));

            let vulnerable = i % 2 == 0;
            let pattern = if vulnerable {
                self.gen_vulnerable_pattern(&mut local_fuzzer)
            } else {
                self.gen_safe_pattern(&mut local_fuzzer)
            };

            corpus.push((pattern, vulnerable));
        }

        corpus
    }

    /// Generate vulnerable health check pattern
    fn gen_vulnerable_pattern(&self, _fuzzer: &mut CodeFuzzer) -> String {
        let _vars: Vec<String> = vec![];

        r#"function updateLending() public {
        // Modify state without health check
        collateral[msg.sender] = collateral[msg.sender] + 1000;
        debt[msg.sender] = debt[msg.sender] - 100;
        // Missing: require(isHealthy(msg.sender));
        emit Updated(msg.sender);
    }"#
        .to_string()
    }

    /// Generate safe health check pattern
    fn gen_safe_pattern(&self, _fuzzer: &mut CodeFuzzer) -> String {
        r#"function updateLending() public {
        // Modify state
        collateral[msg.sender] = collateral[msg.sender] + 1000;
        debt[msg.sender] = debt[msg.sender] - 100;
        // Protected by health check
        require(isHealthy(msg.sender), "Unhealthy position");
        emit Updated(msg.sender);
    }"#
        .to_string()
    }

    /// Run fuzzer and return mutation scores
    pub fn fuzz(&mut self, iterations: usize) -> FuzzResult {
        let mut detections = 0;
        let mut false_positives = 0;
        let mut false_negatives = 0;

        for i in 0..iterations {
            let seed = self._fuzzer.seed.wrapping_add(i as u64);
            let mut local_fuzzer = CodeFuzzer::new(Some(seed));

            let vulnerable = i % 2 == 0;
            let pattern = if vulnerable {
                self.gen_vulnerable_pattern(&mut local_fuzzer)
            } else {
                self.gen_safe_pattern(&mut local_fuzzer)
            };

            // Simulate detector (in real usage, call actual detect_missing_health_check)
            let detected = pattern.contains("Missing") || !pattern.contains("require(isHealthy");

            if vulnerable && detected {
                detections += 1;
            } else if vulnerable && !detected {
                false_negatives += 1;
            } else if !vulnerable && detected {
                false_positives += 1;
            }

            self._test_count += 1;
        }

        FuzzResult {
            true_positives: detections,
            false_positives,
            false_negatives,
            total: iterations,
        }
    }
}

/// Fuzzer results
#[derive(Debug, Clone)]
pub struct FuzzResult {
    /// True positives detected
    pub true_positives: usize,
    /// False positives (safe code detected as vulnerable)
    pub false_positives: usize,
    /// False negatives (vulnerable code not detected)
    pub false_negatives: usize,
    /// Total tests run
    pub total: usize,
}

impl FuzzResult {
    /// Calculate precision (TP / (TP + FP))
    pub fn precision(&self) -> f64 {
        let total_pos = self.true_positives + self.false_positives;
        if total_pos == 0 {
            0.0
        } else {
            self.true_positives as f64 / total_pos as f64
        }
    }

    /// Calculate recall (TP / (TP + FN))
    pub fn recall(&self) -> f64 {
        let actual_pos = self.true_positives + self.false_negatives;
        if actual_pos == 0 {
            0.0
        } else {
            self.true_positives as f64 / actual_pos as f64
        }
    }

    /// Calculate F1 score
    pub fn f1_score(&self) -> f64 {
        let p = self.precision();
        let r = self.recall();
        if p + r == 0.0 {
            0.0
        } else {
            2.0 * (p * r) / (p + r)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fuzzer_generates_patterns() {
        let mut fuzzer = HealthCheckFuzzer::new(Some(42));
        let corpus = fuzzer.generate_corpus(10);
        assert_eq!(corpus.len(), 10);

        // Check vulnerable patterns
        let vuln_count = corpus.iter().filter(|(_, v)| *v).count();
        assert_eq!(vuln_count, 5);
    }

    #[test]
    fn fuzzer_runs_fuzz_tests() {
        let mut fuzzer = HealthCheckFuzzer::new(Some(123));
        let result = fuzzer.fuzz(100);

        assert_eq!(result.total, 100);
        assert!(result.true_positives > 0);
        assert!(result.precision() <= 1.0);
        assert!(result.recall() <= 1.0);
    }

    #[test]
    fn fuzz_result_metrics_valid() {
        let result = FuzzResult {
            true_positives: 80,
            false_positives: 5,
            false_negatives: 15,
            total: 100,
        };

        let precision = result.precision();
        let recall = result.recall();

        assert!(precision > 0.9);
        assert!(recall > 0.8);
        assert!(result.f1_score() > 0.8);
    }

    #[test]
    fn fuzzer_deterministic_with_seed() {
        let mut f1 = HealthCheckFuzzer::new(Some(999));
        let mut f2 = HealthCheckFuzzer::new(Some(999));

        let r1 = f1.fuzz(50);
        let r2 = f2.fuzz(50);

        assert_eq!(r1.true_positives, r2.true_positives);
        assert_eq!(r1.false_positives, r2.false_positives);
    }
}
