/// Fuzzer utilities for property-based vulnerability detection testing.
///
/// Generates code patterns to test detector robustness and minimize false positives.

/// Deterministic pseudo-random number generator for fuzz testing
pub struct CodeFuzzer {
    seed: u64,
    state: u64,
}

impl CodeFuzzer {
    /// Create new fuzzer with optional seed
    pub fn new(seed: Option<u64>) -> Self {
        let s = seed.unwrap_or_else(|| {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        });
        Self {
            seed: s,
            state: s,
        }
    }

    /// Deterministic random number generator using xorshift
    fn next_u64(&mut self) -> u64 {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 7;
        self.state ^= self.state << 17;
        self.state
    }

    /// Generate random number in range [0, max)
    fn gen_range(&mut self, max: usize) -> usize {
        (self.next_u64() as usize) % max
    }

    /// Generate random boolean
    fn gen_bool(&mut self, numerator: usize, denominator: usize) -> bool {
        (self.next_u64() as usize) % denominator < numerator
    }

    /// Generate random function name
    pub fn random_function_name(&mut self) -> String {
        const PREFIXES: &[&str] = &["process_", "execute_", "handle_", "check_", "validate_"];
        const SUFFIXES: &[&str] = &["transaction", "deposit", "withdraw", "transfer", "swap", "mint", "burn"];
        let prefix = PREFIXES[self.gen_range(PREFIXES.len())];
        let suffix = SUFFIXES[self.gen_range(SUFFIXES.len())];
        format!("{}{}_{}", prefix, suffix, self.next_u64() % 1000)
    }

    /// Generate random variable names
    pub fn random_variable_name(&mut self) -> String {
        const NAMES: &[&str] = &["amount", "balance", "reserve", "collateral", "debt", "price", "rate", "value", "token", "user"];
        let name = NAMES[self.gen_range(NAMES.len())];
        format!("{}_{}", name, self.next_u64() % 100)
    }

    /// Generate random Solidity function with variable control flow
    pub fn generate_solidity_function(&mut self, include_vulnerable_pattern: bool) -> String {
        let func_name = self.random_function_name();
        let var1 = self.random_variable_name();
        let var2 = self.random_variable_name();
        
        let health_check = if include_vulnerable_pattern {
            "// Missing health check".to_string()
        } else {
            "require(isHealthy(), \"Not healthy\");".to_string()
        };

        format!(
            r#"    function {}(uint256 {} ) public {{
        {} = {} + 100;
        {} = {} - 50;
        {}
        emit Transfer({}, {});
    }}"#,
            func_name, var1, var1, var1, var2, var2, health_check, var1, var2
        )
    }

    /// Generate random Solidity with merkle root pattern
    pub fn generate_merkle_pattern(&mut self, vulnerable: bool) -> String {
        let init_value = if vulnerable {
            "bytes32 root = bytes32(0);"
        } else {
            "bytes32 root = keccak256(abi.encodePacked(leaves));"
        };

        format!(
            r#"    function setupMerkleRoot() public {{
        {}
        merkleRoot = root;
    }}"#,
            init_value
        )
    }

    /// Generate random oracle usage pattern
    pub fn generate_oracle_pattern(&mut self, vulnerable: bool) -> String {
        let price_source = if vulnerable {
            "uint256 price = token.balanceOf(address(this));"
        } else {
            "uint256 price = priceOracle.getPrice(token); require(price > 0, \"Invalid price\");"
        };

        let state_modification = self.random_variable_name();
        
        format!(
            r#"    function updatePrice() public {{
        {}
        {} = price;
        emit PriceUpdated(price);
    }}"#,
            price_source, state_modification
        )
    }

    /// Generate random collateral check pattern
    pub fn generate_collateral_pattern(&mut self, vulnerable: bool) -> String {
        let check = if vulnerable {
            "// No collateral validation"
        } else {
            "require(collateral >= amount * RATIO, \"Insufficient collateral\");"
        };

        format!(
            r#"    function mintSynthetic(uint256 amount) public {{
        {}
        totalMinted += amount;
    }}"#,
            check
        )
    }

    /// Generate random contract with multiple functions
    pub fn generate_contract(&mut self, function_count: usize) -> String {
        let mut contract = "contract TestContract {\n".to_string();
        
        for _ in 0..function_count {
            let vulnerable = self.gen_bool(3, 10);
            let func = self.generate_solidity_function(vulnerable);
            contract.push_str(&func);
            contract.push('\n');
        }

        contract.push('}');
        contract
    }

    /// Generate random Rust code pattern
    pub fn generate_rust_pattern(&mut self, vulnerable: bool) -> String {
        let check = if vulnerable {
            "// Missing validation"
        } else {
            "if !self.validate_state() { return Err(InvalidState); }"
        };

        format!(
            r#"impl Handler {{
    pub fn process(&mut self) -> Result<()> {{
        self.update_state();
        {}
        Ok(())
    }}
}}"#,
            check
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fuzzer_generates_valid_solidity() {
        let mut fuzzer = CodeFuzzer::new(Some(42));
        let contract = fuzzer.generate_contract(5);
        assert!(contract.contains("contract TestContract"));
        assert!(contract.contains("function"));
    }

    #[test]
    fn fuzzer_generates_different_outputs() {
        let mut fuzzer1 = CodeFuzzer::new(Some(100));
        let mut fuzzer2 = CodeFuzzer::new(Some(200));
        
        let func1 = fuzzer1.generate_solidity_function(true);
        let func2 = fuzzer2.generate_solidity_function(true);
        assert_ne!(func1, func2);
    }

    #[test]
    fn fuzzer_deterministic_with_seed() {
        let mut f1 = CodeFuzzer::new(Some(999));
        let mut f2 = CodeFuzzer::new(Some(999));
        
        assert_eq!(
            f1.generate_solidity_function(true),
            f2.generate_solidity_function(true)
        );
    }

    #[test]
    fn fuzzer_generates_merkle_patterns() {
        let mut fuzzer = CodeFuzzer::new(Some(123));
        let vuln = fuzzer.generate_merkle_pattern(true);
        let safe = fuzzer.generate_merkle_pattern(false);
        
        assert!(vuln.contains("bytes32(0)"));
        assert!(safe.contains("keccak256"));
    }
}
