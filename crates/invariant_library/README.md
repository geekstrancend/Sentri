# sentri-library

Standard library of built-in invariants for the Sentri framework.

Provides 22 pre-configured security checks for EVM (10), Solana (7), and Move (5).

## Usage

```toml
[dependencies]
sentri-library = "0.1.3"
sentri-core = "0.1.3"
```

## Built-in Invariants

Access the full library of invariants programmatically:

```rust
use sentri_library::InvariantLibrary;

let lib = InvariantLibrary::new();
let evm_checks = lib.evm_invariants();
println!("EVM has {} checks", evm_checks.len());
```

## Available Invariants

**EVM** (10):
- Reentrancy (EVM_001)
- Integer Overflow / Underflow (EVM_002, EVM_003)
- Unchecked Return Values (EVM_004)
- Delegatecall Injection (EVM_005)
- Access Control (EVM_006)
- Timestamp Dependence (EVM_007)
- Front-running (EVM_008)
- Uninitialized Pointers (EVM_009)
- Division by Zero (EVM_010)

**Solana** (7):
- Signer Checks (SOL_001)
- Account Validation (SOL_002)
- Integer Overflow (SOL_003)
- Rent Exemption (SOL_004)
- PDA Derivation (SOL_005)
- Lamport Balance (SOL_006)
- Instruction Parsing (SOL_007)

**Move** (5):
- Access Control (MOVE_001)
- Integer Overflow (MOVE_002)
- Resource Leaks (MOVE_003)
- Type Mismatches (MOVE_004)
- Signer Requirements (MOVE_005)

See [Sentri documentation](https://github.com/geekstrancend/Sentri) for detailed descriptions.

## License

MIT

