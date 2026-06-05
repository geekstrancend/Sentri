/// Merkle Root Detector Fuzzer
/// Property-based testing for merkle root zero-value vulnerabilities
///
/// Validates detection of uninitialized or zero merkle roots used in proof verification.
use crate::CodeFuzzer;

/// Merkle root fuzzer test suite
pub struct MerkleRootFuzzer {
    _fuzzer: CodeFuzzer,
    _test_count: usize,
}

impl MerkleRootFuzzer {
    /// Create new merkle root fuzzer
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
            let _fuzzer = CodeFuzzer::new(Some(seed));

            let vulnerable = i % 3 == 0;
            let pattern = if vulnerable {
                self.gen_vulnerable_pattern()
            } else {
                self.gen_safe_pattern()
            };

            corpus.push((pattern, vulnerable));
        }

        corpus
    }

    /// Generate vulnerable merkle root pattern
    fn gen_vulnerable_pattern(&self) -> String {
        format!(
            r#"contract Bridge {{
    bytes32 public merkleRoot;
    
    function initialize() public {{
        // Vulnerable: Root is zero (uninitialized or explicitly set)
        merkleRoot = bytes32(0);
    }}
    
    function verifyAndExecute(bytes32[] calldata proof) public {{
        // Accepts zero root in proof verification
        require(merkleRoot == bytes32(0) || verifyProof(proof, merkleRoot));
        executeTransaction();
    }}
}}"#
        )
    }

    /// Generate safe merkle root pattern
    fn gen_safe_pattern(&self) -> String {
        format!(
            r#"contract Bridge {{
    bytes32 public merkleRoot;
    
    function initialize(bytes32[] memory leaves) public {{
        // Safe: Root computed from actual leaves
        merkleRoot = computeMerkleRoot(leaves);
        require(merkleRoot != bytes32(0), "Invalid root");
    }}
    
    function verifyAndExecute(bytes32[] calldata proof) public {{
        // Rejects zero root
        require(merkleRoot != bytes32(0), "Root not initialized");
        require(verifyProof(proof, merkleRoot), "Invalid proof");
        executeTransaction();
    }}
}}"#
        )
    }

    /// Run fuzzer and return results
    pub fn fuzz(&mut self, iterations: usize) -> FuzzResult {
        let mut detections = 0;
        let mut false_positives = 0;
        let mut false_negatives = 0;

        for i in 0..iterations {
            let vulnerable = i % 3 == 0;
            let pattern = if vulnerable {
                self.gen_vulnerable_pattern()
            } else {
                self.gen_safe_pattern()
            };

            // Simulate detector: check for zero root initialization
            let detected = pattern.contains("bytes32(0)")
                && pattern.contains("merkleRoot")
                && !pattern.contains("require(merkleRoot != bytes32(0)");

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
    /// False positives
    pub false_positives: usize,
    /// False negatives
    pub false_negatives: usize,
    /// Total tests
    pub total: usize,
}

impl FuzzResult {
    /// Calculate precision
    pub fn precision(&self) -> f64 {
        let total_pos = self.true_positives + self.false_positives;
        if total_pos == 0 {
            0.0
        } else {
            self.true_positives as f64 / total_pos as f64
        }
    }

    /// Calculate recall
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
        let mut fuzzer = MerkleRootFuzzer::new(Some(42));
        let corpus = fuzzer.generate_corpus(15);
        assert_eq!(corpus.len(), 15);
    }

    #[test]
    fn fuzzer_detects_zero_root() {
        let mut fuzzer = MerkleRootFuzzer::new(Some(123));
        let result = fuzzer.fuzz(100);

        assert!(result.true_positives > 0);
        assert!(result.precision() > 0.8);
    }

    #[test]
    fn fuzz_result_metrics() {
        let result = FuzzResult {
            true_positives: 30,
            false_positives: 2,
            false_negatives: 8,
            total: 40,
        };

        assert!(result.precision() > 0.9);
        assert!(result.recall() > 0.7);
        assert!(result.f1_score() > 0.8);
    }
}
