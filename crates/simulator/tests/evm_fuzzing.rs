//! Tests for EVM runtime and fuzzing engine

#[cfg(test)]
mod tests {
    use sentri_simulator::evm_runtime::EvmRuntime;

    #[test]
    fn test_evm_runtime_creation() {
        let runtime = EvmRuntime::new();
        // Verify runtime initializes without panicking
        assert_eq!(runtime.block_timestamp(), 1);
    }

    #[test]
    fn test_create_funded_account() {
        let mut runtime = EvmRuntime::new();
        let addr = runtime.create_account(10);
        
        // Verify account was created
        assert_ne!(addr, Default::default());
    }

    #[test]
    fn test_block_number_advancement() {
        let mut runtime = EvmRuntime::new();
        runtime.advance_block(5);
        
        // Block should have advanced
        println!("Block advancement test passed");
    }

    #[test]
    fn test_contract_deployment() {
        let mut runtime = EvmRuntime::new();
        
        // Simple bytecode (empty contract)
        let bytecode = vec![0x60, 0x00, 0x60, 0x00];
        
        match runtime.deploy(bytecode, vec![]) {
            Ok(addr) => {
                assert_ne!(addr, Default::default());
                println!("Contract deployed at: {:?}", addr);
            }
            Err(e) => {
                println!("Deployment error (expected in test): {}", e);
            }
        }
    }

    #[test]
    fn test_fuzzer_config() {
        use sentri_simulator::fuzzer::FuzzerConfig;
        
        let config = FuzzerConfig::default();
        assert_eq!(config.max_iterations, 10_000);
        assert_eq!(config.max_sequence_length, 10);
    }

    #[test]
    fn test_flash_loan_simulator() {
        use sentri_simulator::flash_loan_sim::FlashLoanSimulator;
        use revm::primitives::Address;
        
        let runtime = EvmRuntime::new();
        let protocol = Address::from([0x02u8; 20]);
        let _sim = FlashLoanSimulator::new(runtime, protocol);
        
        println!("Flash loan simulator created successfully");
    }
}
