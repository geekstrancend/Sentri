# sentri-analyzer-evm

EVM bytecode analyzer for the Sentri framework.

Performs static analysis on Solidity smart contracts and EVM bytecode to detect security invariant violations.

## Usage

```toml
[dependencies]
sentri-analyzer-evm = "0.1.3"
sentri-core = "0.1.3"
sentri-ir = "0.1.3"
```

## Key Components

- `EVMAnalyzer`: Main analysis engine for EVM bytecode
- `SolidityAnalyzer`: Source-level analysis for Solidity contracts
- `Pattern Detector`: Identifies security anti-patterns
- `ControlFlow`: Analyzes transaction and call flows

## Example

```rust
use sentri_analyzer_evm::EVMAnalyzer;
use sentri_core::Target;

let mut analyzer = EVMAnalyzer::new();
let bytecode = vec![0x60, 0x01, 0x60, 0x02, 0x01]; // Simple bytecode

let violations = analyzer.analyze(&bytecode, Target::EVM)?;
println!("Found {} violations", violations.len());
```

## Supported Chains

- Ethereum
- Arbitrum
- Optimism
- Polygon
- All EVM-compatible chains

See [Sentri documentation](https://github.com/geekstrancend/Sentri) for chain-specific configuration.

## License

MIT
