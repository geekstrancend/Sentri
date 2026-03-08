# sentri-generator-solana

Code generator for Solana programs using Sentri invariants.

Generates Rust/Anchor code and runtime instrumentation to enforce Sentri invariants in Solana programs.

## Usage

```toml
[dependencies]
sentri-generator-solana = "0.1.3"
sentri-core = "0.1.3"
sentri-ir = "0.1.3"
```

## Key Components

- `AnchorGenerator`: Generates Anchor instruction handlers with checks
- `InstructionInstrumentor`: Adds safety checks to instruction handlers
- `AccountValidator`: Creates account constraint validators
- `PDASafetyChecker`: Generates PDA validation code

## Example

```rust
use sentri_generator_solana::AnchorGenerator;

let mut generator = AnchorGenerator::new();
let invariants = vec!["is_signer == true", "lamports > 0"];

let generated_handler = generator.generate(&invariants)?;
println!("Generated handler with {} checks", generated_handler.checks);
```

## Generation Targets

- Anchor framework programs
- Native Solana program instructions
- Account constraint checks
- Signer verification code

## Features

- Anchor instruction macro instrumentation
- Account constraint generation
- Signer requirement enforcement
- Rent/lamport validation
- Cross-program invocation safety

See [Sentri documentation](https://github.com/geekstrancend/Sentri) for Solana generation options.

## License

MIT
