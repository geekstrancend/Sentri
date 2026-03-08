# sentri

Multi-chain smart contract invariant checker for EVM, Solana, and Move.

[![Crates.io](https://img.shields.io/crates/v/sentri-cli)](https://crates.io/crates/sentri-cli)
[![Downloads](https://img.shields.io/crates/d/sentri-cli)](https://crates.io/crates/sentri-cli)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue)](https://github.com/geekstrancend/Sentri/blob/main/LICENSE)

## What is Sentri?

Sentri is a unified invariant checking framework for smart contracts. Define security properties once in Sentri's DSL and verify them across EVM, Solana, and Move contracts automatically.

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
| **EVM** | Solidity, Vyper | 10 built-in invariants | ✅ Stable |
| **Solana** | Rust (Anchor, native) | 7 built-in invariants | ✅ Stable |
| **Move** | Move (Aptos, Sui) | 5 built-in invariants | ✅ Stable |

## Features

- 22 built-in security invariants
- Custom invariant DSL
- JSON/HTML/text reports
- CI/CD integration
- Violation suppression
- Cross-platform binaries

## Documentation

Full documentation: [github.com/geekstrancend/Sentri](https://github.com/geekstrancend/Sentri)

## License

MIT

