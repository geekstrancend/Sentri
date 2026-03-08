# sentri-core

Core types, configuration, and error handling for the Sentri invariant checking framework.

This crate provides the fundamental types and utilities you need to build tools within the Sentri ecosystem.

## Usage

As a library dependency:

```toml
[dependencies]
sentri-core = "0.1.3"
```

## Key Types

- `Invariant` — represents a security invariant specification
- `Violation` — a detected invariant violation
- `Config` — project and analysis configuration
- `SeverityLevel` — violation severity (Critical, High, Medium, Low, Info)

## Example

```rust
use sentri_core::{Invariant, Violation, SeverityLevel};

let violation = Violation {
    id: "EVM_001".to_string(),
    severity: SeverityLevel::Critical,
    title: "Reentrancy Vulnerability".to_string(),
    message: "External call before state change detected".to_string(),
};
```

See [Sentri documentation](https://github.com/geekstrancend/Sentri) for more examples.

## License

MIT

