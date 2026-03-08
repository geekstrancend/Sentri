# sentri-analyzer-solana

Solana program analyzer for the Sentri framework.

Performs static analysis on Solana programs (Rust/Anchor) and transaction data to detect security invariant violations.

## Usage

```toml
[dependencies]
sentri-analyzer-solana = "0.1.3"
sentri-core = "0.1.3"
sentri-ir = "0.1.3"
```

## Key Components

- `SolanaAnalyzer`: Main analysis engine for Solana programs
- `AccountValidator`: Validates account constraints and permissions
- `SignerChecker`: Verifies signer requirements
- `PDAValidator`: Analyzes Program Derived Address derivations

## Example

```rust
use sentri_analyzer_solana::SolanaAnalyzer;
use sentri_core::Target;

let mut analyzer = SolanaAnalyzer::new();
let program_id = "11111111111111111111111111111111";

let violations = analyzer.analyze(program_id.as_bytes(), Target::Solana)?;
println!("Found {} violations", violations.len());
```

## Supported Features

- Anchor framework analysis
- Native Solana program inspection
- Signer verification
- Account rent (lamport) validation
- PDA safety checks
- Cross-program invocation tracking

See [Sentri documentation](https://github.com/geekstrancend/Sentri) for detailed analysis capabilities.

## License

MIT
