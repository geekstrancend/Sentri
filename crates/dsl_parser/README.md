# sentri-dsl-parser

Parser for Sentri's invariant DSL.

Parses `.invar` files into the Sentri intermediate representation (IR) using the `pest` parser generator.

## Usage

```toml
[dependencies]
sentri-dsl-parser = "0.1.3"
sentri-ir = "0.1.3"
```

## Parsing Invariant Files

```rust
use sentri_dsl_parser::Parser;

let parser = Parser::new();
let spec = parser.parse_file("invariants.invar")?;
println!("Loaded {} checks", spec.checks.len());
```

## DSL Syntax

The invariant DSL provides a readable, declarative way to express security properties:

```
invariant_check no_reentrancy {
  description: "Detect reentrancy patterns"
  chain: evm
  severity: critical
  check {
    NO_EXTERNAL_CALLS_BEFORE_STATE_CHANGE
  }
}
```

See [Sentri documentation](https://github.com/geekstrancend/Sentri) for complete DSL reference.

## License

MIT

