# sentri-generator-move

Code generator for Move modules using Sentri invariants.

Generates Move module code and runtime checks to enforce Sentri invariants in Move programs.

## Usage

```toml
[dependencies]
sentri-generator-move = "0.1.3"
sentri-core = "0.1.3"
sentri-ir = "0.1.3"
```

## Key Components

- `MoveCodeGenerator`: Generates Move module code with invariant checks
- `ResourceValidator`: Creates resource safety validators
- `AccessControlGenerator`: Generates access control checks
- `TypeSafetyChecker`: Enforces Move type system constraints

## Example

```rust
use sentri_generator_move::MoveCodeGenerator;

let mut generator = MoveCodeGenerator::new();
let invariants = vec!["balance > 0", "owner != @0x0"];

let generated_module = generator.generate(&invariants)?;
println!("Generated module with {} checks", generated_module.check_count);
```

## Generation Targets

- Aptos Move modules
- Movement ecosystem programs
- Resource lifecycle enforcement
- Type-safe invariant checks

## Features

- Module-level invariant enforcement
- Resource safety validation
- Access control generation
- Type system compliance
- Atomic operation safety

See [Sentri documentation](https://github.com/geekstrancend/Sentri) for Move generation options.

## License

MIT
