# sentri-analyzer-evm

EVM (Solidity) analyzer for the Sentri framework.

## Usage

```toml
[dependencies]
sentri-analyzer-evm = "0.3.0"
sentri-core = "0.3.0"
sentri-ir = "0.3.0"
```

## Key Components

- `EvmAnalyzer` — implements `ChainAnalyzer` (`analyze(&self, path: &Path) -> Result<ProgramModel>`); parses via `solc`'s JSON AST when `solc` is installed
- `detectors::run_all_detectors(source, file_path)` — the entry point the CLI calls; runs all 37 EVM pattern detectors (regex/source-text based, no `solc` required) plus the shared cross-chain `unauthorized_privileged_mutation` rule (best-effort, needs `solc`)
- `bytecode::BytecodeAnalyzer` — disassembles and inspects compiled bytecode (not currently wired into `run_all_detectors`)
- `cfg::ControlFlowGraph`, `symbolic` — control-flow/symbolic-value scaffolding used internally by `semantic_model`

## Example

```rust
use sentri_analyzer_evm::detectors::run_all_detectors;

let source = std::fs::read_to_string("Vault.sol")?;
let findings = run_all_detectors(&source, "Vault.sol");
println!("Found {} findings", findings.len());
```

## Detectors (37)

Named historical-exploit patterns (health check, merkle root, DVN single
point of failure, unbacked synthetic mint, LST depeg, oracle self-trade,
ERC4626 inflation, arbitrary call `msg.value`, reentrancy via whitelisted
contracts, proxy storage collision, bridge address verification, read-only
reentrancy, insufficient multisig threshold, and more) plus a base set of
classic patterns (reentrancy, missing signer/access checks, unchecked math,
missing conservation checks). See `src/detectors/mod.rs` for the full list.

## Supported Chains

Any EVM-compatible chain (Ethereum, Arbitrum, Optimism, Polygon, ...) —
detection is source/AST based, not chain-ID specific.

## License

MIT
