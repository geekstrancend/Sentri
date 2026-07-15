# Sentri

[![crates.io](https://img.shields.io/crates/v/sentri-cli.svg)](https://crates.io/crates/sentri-cli)
[![npm](https://img.shields.io/npm/v/@dextonicx/cli.svg)](https://www.npmjs.com/package/@dextonicx/cli)
[![Downloads](https://img.shields.io/crates/d/sentri-cli.svg)](https://crates.io/crates/sentri-cli)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![CI](https://github.com/geekstrancend/Sentri/actions/workflows/ci.yml/badge.svg)](https://github.com/geekstrancend/Sentri/actions)

**Multi-chain smart contract security analyzer for EVM, Solana, and Move.**

Sentri checks your smart contracts and programs for vulnerabilities before
deployment. Define what should always be true — invariants — and Sentri
verifies your code cannot violate them.

One tool. Three chains. One DSL.

---

## What's new in v0.3.0

v0.3.0 reconnects the full detection pipeline into the CLI and adds a
chain-agnostic detection layer on top of it.

**Key improvements:**

- ✅ **50 Smart Contract Vulnerability Detectors** — Comprehensive coverage of critical and high-priority exploits, wired end-to-end into `sentri check`/`sentri scan`
- ✅ **Chain-agnostic shared rule** — `unauthorized_privileged_mutation` runs against a common semantic model built by each chain's own analyzer, so one rule (missing an authorization check on a privileged mutation) is written once and applies to all three chains
- ✅ **Real Move parsing** — a vendored Sui Move tree-sitter grammar backs Move's semantic extraction, with the original regex heuristic kept as an automatic fallback if a file fails to parse
- ✅ **Real fuzzing** — `sentri fuzz` mutates real source files and runs them through the live detectors looking for crashes, instead of a no-op stub
- ✅ **Production Ready** — All tests passing, security audit complete, reproducible builds

**Detector Coverage:**
- **EVM**: 35 detectors (reentrancy, missing checks, oracle manipulation, proxy issues, and 20+ named historical-exploit patterns)
- **Solana**: 9 detectors (PDA validation, authority checks, replay attacks, durable nonce, rent exemption)
- **Move**: 6 detectors (resource destruction, type safety, access control)

Sentri also ships a web dashboard (`web/`) — sign-up, scan submission, and
report viewing on top of the same CLI engine — alongside the `sentri` CLI
and its npm wrapper (`@dextonicx/cli`).

---

## What's new in v0.2.2

v0.2.2 adds reproducibility and flexible output options for better CI integration and reporting.

**Key improvements:**

- ✅ **Reproducible analysis** — `--seed` flag for deterministic results across runs
- ✅ **File output** — `--output` flag to write reports to disk (text, JSON, HTML)
- ✅ **HTML reports** — Beautiful formatted security reports with styled tables and summaries
- ✅ **Solana SDK 1.x** — Updated to latest stable Solana SDK

---

## What's new in v0.2.1

v0.2.1 fixes violation location reporting — all violations now show their actual source line numbers instead of defaulting to line 1. This dramatically improves debugging workflow.

**Key improvements:**

- ✅ **Accurate violation locations** — Real line numbers from the vulnerable code
- ✅ **Code context** — Shows 2 lines before/after violation for quick reference
- ✅ **Embedded line tracking** — Line numbers calculated during AST analysis, not post-processing

---

## What's new in v0.2.0

v0.2 replaces pattern matching with real Rust AST parsing via the `syn`
crate. Sentri now understands Anchor's type system and eliminates false
positives on idiomatic Anchor programs.

| Pattern | v0.1 | v0.2 |
| --- | --- | --- |
| `Signer<'info>` | ❌ False positive | ✅ Correctly silent |
| `Account<'info, T>` | ❌ Over-flagged | ✅ Recognized as safe |
| `AccountInfo` with `seeds = [...]` | ❌ False positive | ✅ Correctly silent |
| `AccountInfo` with `/// CHECK:` | ❌ False positive | ✅ Downgraded to INFO |
| `AccountInfo` with no constraint | ✅ CRITICAL | ✅ Still CRITICAL |

> **Upgrading from v0.1/v0.2.0?** Run `cargo install sentri-cli --force`

---

## Install

```bash
# Rust developers
cargo install sentri-cli

# JavaScript / TypeScript developers
npm install -g @dextonicx/cli

# Verify installation
sentri --version   # sentri 0.2.1
sentri doctor
```

Or download a pre-built binary directly from
[GitHub Releases](https://github.com/geekstrancend/Sentri/releases).

**Supported platforms:**

- Linux x86_64, aarch64, musl
- macOS x86_64, aarch64 (Apple Silicon)
- Windows x86_64

---

## Quick start

```bash
# Check a Solana program
sentri check ./programs --chain solana

# Check Solidity contracts
sentri check ./contracts --chain evm

# Check Move modules
sentri check ./sources --chain move

# Output as JSON
sentri check ./programs --chain solana --format json

# Output as HTML
sentri check ./programs --chain solana --format html --output ./report.html

# Reproducible analysis with fixed seed
sentri check ./programs --chain solana --seed 42

# Fail CI if high or critical violations found
sentri check ./programs --chain solana --fail-on high

# Run health check
sentri doctor

# Initialize config
sentri init
```

---

## Output options

### Report formats

```bash
# Text report (default, human-readable)
sentri check ./programs --chain solana --format text

# JSON report (machine-readable, for parsing/CI)
sentri check ./programs --chain solana --format json

# HTML report (styled, shareable with team)
sentri check ./programs --chain solana --format html
```

### Saving reports to disk

```bash
# Save any format to file
sentri check ./programs --chain solana --format json --output ./report.json
sentri check ./programs --chain solana --format html --output ./report.html
sentri check ./programs --chain solana --format text --output ./report.txt
```

### Reproducible analysis

For deterministic results (useful in CI or security audits), use `--seed`:

```bash
# Always uses seed 42 by default
sentri check ./programs --chain solana

# Use a custom seed
sentri check ./programs --chain solana --seed 12345

# Results will be identical on the same code with the same seed
```

---

## GitHub Actions

Add one step to your workflow:

```yaml
- name: Sentri security check
  run: |
    cargo install sentri-cli
    sentri check ./programs --chain solana --fail-on high
```

CI fails automatically on high or critical violations. Zero additional
configuration required.

---

## Built-in invariants

Sentri ships with 22 built-in security checks across all three chains.

### EVM (10 invariants)

| ID | Name | Severity |
| --- | --- | --- |
| `evm_reentrancy_protection` | Reentrancy Protection | Critical |
| `evm_integer_overflow` | Integer Overflow | High |
| `evm_integer_underflow` | Integer Underflow | High |
| `evm_unchecked_returns` | Unchecked Return Values | Medium |
| `evm_delegatecall_injection` | Delegatecall Injection | Critical |
| `evm_access_control` | Access Control | High |
| `evm_timestamp_dependence` | Timestamp Dependence | Medium |
| `evm_frontrunning` | Front-running | Medium |
| `evm_uninitialized_pointers` | Uninitialized Pointers | High |
| `evm_division_by_zero` | Division by Zero | Medium |

### Solana (7 invariants)

| ID | Name | Severity |
| --- | --- | --- |
| `sol_signer_checks` | Signer Checks | Critical |
| `sol_account_validation` | Account Validation | Critical |
| `sol_integer_overflow` | Integer Overflow | High |
| `sol_rent_exemption` | Rent Exemption | Medium |
| `sol_pda_derivation` | PDA Derivation | High |
| `sol_lamport_balance` | Lamport Balance | Critical |
| `sol_instruction_parsing` | Instruction Parsing | Medium |

### Move (5 invariants)

| ID | Name | Severity |
| --- | --- | --- |
| `move_access_control` | Access Control | Critical |
| `move_integer_overflow` | Integer Overflow | High |
| `move_resource_leaks` | Resource Leaks | High |
| `move_type_safety` | Type Safety | High |
| `move_signer_requirement` | Signer Requirement | Critical |

---

## Configuration

Create `.sentri.toml` in your project root:

```toml
[project]
name = "my-project"
chain = "solana"

[analysis]
severity_threshold = "low"
# suppress = ["sol_rent_exemption"]

[output]
format = "text"   # text | json | html
```

Or run `sentri init` to generate a config automatically.

### Inline suppression

```rust
// sentri: ignore sol_account_validation — external VRF oracle account
pub oracle_queue: AccountInfo<'info>,
```

---

## Anchor false positive guide (v0.2+)

Sentri v0.2 understands Anchor's type system. These patterns are
correctly handled:

```rust
// SAFE — Anchor enforces signer automatically
pub authority: Signer<'info>,

// SAFE — Anchor validates ownership and discriminator
pub arena: Account<'info, Arena>,

// SAFE — seeds constraint validates PDA derivation
#[account(seeds = [b"vault", user.key().as_ref()], bump)]
pub vault: AccountInfo<'info>,

// SAFE — developer has verified this external account
/// CHECK: This is the Switchboard VRF oracle. Address validated off-chain.
pub oracle_queue: AccountInfo<'info>,

// CRITICAL — genuinely unchecked, Sentri correctly fires
pub mystery: AccountInfo<'info>,
```

---

## Roadmap

| Version | Focus | Status |
| --- | --- | --- |
| v0.1 | Pattern-based analysis, 22 invariants, full CLI | ✅ Shipped |
| v0.2 | Real AST parsing, Anchor-aware analysis | ✅ Shipped |
| v0.3 | Runtime fuzzing — revm + solana-program-test | 🔨 Next |
| v0.4 | Bounded model checking | 📋 Planned |
| v0.5 | Symbolic execution via Z3 | 📋 Planned |
| v1.0 | Slither + Echidna + Mythril for every chain | 🎯 Goal |

---

## Links

- **GitHub**: [geekstrancend/Sentri](https://github.com/geekstrancend/Sentri)
- **crates.io**: [sentri-cli](https://crates.io/crates/sentri-cli)
- **npm**: [@dextonicx/cli](https://www.npmjs.com/package/@dextonicx/cli)
- **Docs**: [docs.rs/sentri-cli](https://docs.rs/sentri-cli)

---

## Contributing

Issues, PRs, and feedback are welcome.

If you are a Rust engineer familiar with `syn` or Anchor internals,
the v0.3 fuzzing work is the highest-impact contribution area right now.

If you are a smart contract auditor, help expand the invariant library
with real attack patterns you have encountered.

---

## License

MIT — see [LICENSE](LICENSE)
