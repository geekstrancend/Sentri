# sentri-analyzer-move

Move language analyzer for the Sentri framework.

Performs static analysis on Move modules and transactions to detect security invariant violations and unsafe patterns.

## Usage

```toml
[dependencies]
sentri-analyzer-move = "0.1.3"
sentri-core = "0.1.3"
sentri-ir = "0.1.3"
```

## Key Components

- `MoveAnalyzer`: Main analysis engine for Move bytecode
- `TypeChecker`: Validates Move type system compliance
- `AccessControl`: Analyzes visibility and access patterns
- `ResourceAnalyzer`: Tracks resource creation and movement

## Example

```rust
use sentri_analyzer_move::MoveAnalyzer;
use sentri_core::Target;

let mut analyzer = MoveAnalyzer::new();
let module_data = vec![0x00, 0x01]; // Move compiled module

let violations = analyzer.analyze(&module_data, Target::Move)?;
println!("Found {} violations", violations.len());
```

## Supported Platforms

- Aptos
- Movement
- Other Move-compatible networks

## Analysis Capabilities

- Move module bytecode inspection
- Type safety verification
- Resource leak detection
- Access control validation
- Integer overflow checking

See [Sentri documentation](https://github.com/geekstrancend/Sentri) for detailed Move analysis documentation.

## License

MIT
