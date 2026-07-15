# sentri

Multi-chain smart contract invariant checker for EVM, Solana, Move, and Soroban.

[![Crates.io](https://img.shields.io/crates/v/sentri-cli)](https://crates.io/crates/sentri-cli)
[![Downloads](https://img.shields.io/crates/d/sentri-cli)](https://crates.io/crates/sentri-cli)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue)](https://github.com/geekstrancend/Sentri/blob/main/LICENSE)

## What is Sentri?

Sentri is a unified invariant checking framework for smart contracts. Define security properties once in Sentri's DSL and verify them across EVM, Solana, Move, and Soroban contracts automatically.

## Quick Start

### Install

```bash
cargo install sentri-cli
```

Verify installation:
```bash
sentri doctor
```

### Usage

Initialize project:
```bash
sentri init
```

Run checks:
```bash
sentri check ./contracts --chain evm
```

Generate report:
```bash
sentri check ./contracts --format json --output report.json
```

## Supported Chains

| Chain | Language | Checks | Status |
|-------|----------|--------|--------|
| **EVM** | Solidity, Vyper | 35 built-in detectors | ✅ Stable |
| **Solana** | Rust (Anchor, native) | 9 built-in detectors | ✅ Stable |
| **Move** | Move (Aptos, Sui) | 6 built-in detectors, real AST via a vendored Sui tree-sitter grammar | ✅ Stable |
| **Soroban** | Rust (Stellar) | 8 built-in detectors | ✅ Stable |

## Features

- 58 smart contract vulnerability detectors, plus a chain-agnostic rule (`unauthorized_privileged_mutation`) shared across all four
- Custom invariant DSL
- JSON/HTML/text reports
- CI/CD integration
- Violation suppression
- Cross-platform binaries
- Production-ready security analysis

## Documentation

Full documentation: [github.com/geekstrancend/Sentri](https://github.com/geekstrancend/Sentri)

## License

MIT

