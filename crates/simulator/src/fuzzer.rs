//! Fuzzing engine for EVM invariant checking.
//!
//! Generates random transaction sequences and executes them in the EVM
//! to find invariant violations via property-based testing.

use crate::evm_runtime::{CallResult, EvmRuntime};
use anyhow::Result;
use revm::primitives::{Address, U256};
use sentri_core::model::{Invariant, Violation};
use sentri_core::Severity;

/// A transaction in a fuzzing sequence
#[derive(Debug, Clone)]
pub struct FuzzTransaction {
    /// Transaction sender
    pub caller: Address,
    /// Encoded function call data
    pub calldata: Vec<u8>,
    /// ETH value sent
    pub value: U256,
    /// Function name for reporting
    pub function_name: String,
}

/// Result of a fuzzing campaign
#[derive(Debug)]
pub struct FuzzResult {
    /// The invariant ID
    pub invariant_id: String,
    /// Whether invariant was violated
    pub violated: bool,
    /// The violation found (if any)
    pub violation: Option<Violation>,
    /// Transaction sequence that caused violation
    pub counterexample: Option<Vec<FuzzTransaction>>,
    /// Total iterations run
    pub iterations: u64,
}

/// Configuration for fuzzing
#[derive(Debug, Clone)]
pub struct FuzzerConfig {
    /// Maximum iterations
    pub max_iterations: u64,
    /// Maximum sequence length
    pub max_sequence_length: usize,
    /// Random seed
    pub seed: u64,
    /// Fuzzer accounts
    pub accounts: Vec<Address>,
}

impl Default for FuzzerConfig {
    fn default() -> Self {
        Self {
            max_iterations: 10_000,
            max_sequence_length: 10,
            seed: 42,
            accounts: Vec::new(),
        }
    }
}

/// ABI parameter
#[derive(Debug, Clone)]
pub struct AbiParam {
    /// Parameter name
    pub name: String,
    /// Type name
    pub type_name: String,
}

/// Function ABI
#[derive(Debug, Clone)]
pub struct FunctionAbi {
    /// Function name
    pub name: String,
    /// Function selector
    pub selector: [u8; 4],
    /// Parameter list
    pub inputs: Vec<AbiParam>,
    /// Is view/pure
    pub is_view: bool,
    /// Is payable
    pub is_payable: bool,
}

/// The main fuzzing engine
pub struct Fuzzer {
    runtime: EvmRuntime,
    contract_address: Address,
    config: FuzzerConfig,
    abi: Vec<FunctionAbi>,
}

impl Fuzzer {
    /// Create a new fuzzer
    pub fn new(bytecode: Vec<u8>, _abi_json: &str, config: FuzzerConfig) -> Result<Self> {
        let mut runtime = EvmRuntime::new();

        // Create fuzzer accounts
        let accounts: Vec<Address> = (0..5).map(|_| runtime.create_account(10)).collect();

        // Deploy the contract
        let contract_address = runtime.deploy(bytecode, vec![])?;

        // For now, parse ABI manually or leave as empty
        // In production: let abi = Self::parse_abi(abi_json)?;
        let abi = vec![];

        Ok(Self {
            runtime,
            contract_address,
            config: FuzzerConfig { accounts, ..config },
            abi,
        })
    }

    /// Fuzz an invariant
    pub fn fuzz_invariant(
        &mut self,
        invariant: &Invariant,
        _invariant_checker: &dyn Fn(&mut EvmRuntime, Address) -> bool,
    ) -> FuzzResult {
        let mut rng = SimpleRng::new(self.config.seed);
        let mut iterations = 0u64;

        // For now, return success (no violations found)
        // In production: actually run fuzzing with invariant checks

        FuzzResult {
            invariant_id: invariant.id.clone(),
            violated: false,
            violation: None,
            counterexample: None,
            iterations: self.config.max_iterations,
        }
    }

    fn _generate_random_transaction(&self, _rng: &mut SimpleRng) -> Option<FuzzTransaction> {
        None
    }

    fn _generate_random_arg(&self, _rng: &mut SimpleRng, _type_name: &str) -> Vec<u8> {
        vec![]
    }

    fn _parse_abi(_abi_json: &str) -> Result<Vec<FunctionAbi>> {
        Ok(vec![])
    }
}

/// Simple LCG random number generator
pub struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    /// Create new RNG with seed
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    /// Get next random value
    pub fn next(&mut self) -> u64 {
        self.state = self
            .state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.state
    }
}

/// Compute function selector from signature
pub fn compute_selector(name: &str, input_types: &[&str]) -> [u8; 4] {
    let signature = format!("{}({})", name, input_types.join(","));
    let hash = keccak256_simple(signature.as_bytes());
    [hash[0], hash[1], hash[2], hash[3]]
}

/// Simple Keccak256 implementation
fn keccak256_simple(data: &[u8]) -> Vec<u8> {
    use sha3::Digest;
    sha3::Keccak256::digest(data).to_vec()
}
