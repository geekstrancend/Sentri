# sentri-ir

Intermediate representation (IR) for Sentri invariant specifications.

Defines the AST and IR structures used to represent and analyze invariant specifications across EVM, Solana, and Move platforms.

## Usage

```toml
[dependencies]
sentri-ir = "0.3.0"
sentri-core = "0.3.0"
```

## Key Types

- `Spec` — complete invariant specification
- `Check` — individual check/invariant
- `Expression` — semantic expressions in invariants
- `Target` — chain target (EVM, Solana, Move)

## Example

```rust
use sentri_ir::{Spec, Target};

let spec = Spec {
    name: "balance_check".to_string(),
    target: Target::EVM,
    description: "Validate balance consistency".to_string(),
};
```

See [Sentri documentation](https://github.com/geekstrancend/Sentri) for IR details.

## License

MIT

