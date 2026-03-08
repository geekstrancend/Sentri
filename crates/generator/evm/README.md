# sentri-generator-evm

Code generator for EVM smart contracts using Sentri invariants.

Generates Solidity code and EVM bytecode instrumentation to enforce Sentri invariants at runtime.

## Usage

```toml
[dependencies]
sentri-generator-evm = "0.1.3"
sentri-core = "0.1.3"
sentri-ir = "0.1.3"
```

## Key Components

- `SolidityGenerator`: Generates Solidity inline checks
- `BytecodeInstrumentor`: Inserts runtime verification into bytecode
- `GasEstimator`: Calculates gas costs of instrumentation
- `ASTTransformer`: Modifies contract AST for invariant enforcement

## Example

```rust
use sentri_generator_evm::SolidityGenerator;

let mut generator = SolidityGenerator::new();
let invariants = vec!["balance != 0", "owner != address(0)"];

let instrumented_code = generator.generate(&invariants)?;
println!("Generated {} lines", instrumented_code.lines().count());
```

## Generation Targets

- Solidity 0.8.x contracts
- EVM bytecode (via ethers-rs)
- Runtime invariant enforcement
- Gas-optimized checks

## Output Formats

- Pure Solidity (.sol files)
- Solidity with external caller contracts
- Standalone verification contracts

See [Sentri documentation](https://github.com/geekstrancend/Sentri) for generation options.

## License

MIT
