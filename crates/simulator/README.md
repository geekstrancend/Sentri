# sentri-simulator

Transaction simulator for Sentri invariant testing.

Simulates blockchain transactions against invariant specifications to detect violations before deployment.

## Usage

```toml
[dependencies]
sentri-simulator = "0.1.3"
sentri-core = "0.1.3"
sentri-ir = "0.1.3"
```

## Key Components

- `TransactionSimulator`: Executes transactions in sandboxed environment
- `StateTracker`: Tracks blockchain state changes during simulation
- `InvariantChecker`: Real-time violation detection
- `ReportGenerator`: Produces detailed violation reports

## Example

```rust
use sentri_simulator::{TransactionSimulator, SimulationConfig};
use sentri_core::{Target, Invariant};

let config = SimulationConfig::default();
let mut simulator = TransactionSimulator::new(config);

let tx_data = vec![0x00, 0x01]; // Transaction data
let invariants = vec![/* your invariants */];

let result = simulator.simulate(&tx_data, &invariants, Target::EVM)?;
println!("Violations found: {}", result.violations.len());
```

## Simulation Capabilities

- EVM transaction simulation
- Solana instruction simulation
- Move transaction simulation
- State change tracking
- Invariant violation reporting
- Gas estimation

## Features

- Sandboxed execution (no external state)
- Multi-transaction scenarios
- State rollback/recovery
- Detailed violation reporting
- Performance metrics

See [Sentri documentation](https://github.com/geekstrancend/Sentri) for simulator configuration.

## License

MIT
