<h1 align="center">Sentri</h1>

<p align="center">Multi-chain smart contract invariant checker for EVM, Solana, and Move</p>

<p align="center">
  <a href="https://img.shields.io/crates/v/sentri-cli"><img src="https://img.shields.io/crates/v/sentri-cli" alt="Crates.io"></a>
  <a href="https://img.shields.io/crates/d/sentri-cli"><img src="https://img.shields.io/crates/d/sentri-cli" alt="Downloads"></a>
  <a href="https://github.com/geekstrancend/Sentri/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue" alt="License"></a>
  <a href="https://github.com/geekstrancend/Sentri/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/geekstrancend/Sentri/ci.yml?branch=main" alt="CI"></a>
  <a href="#"><img src="https://img.shields.io/badge/rust-stable-orange" alt="Rust"></a>
</p>

---

## What is Sentri?

Sentri is a unified invariant checking framework that lets you define security properties once and verify them across multiple blockchain platforms. Write invariants in Sentri's DSL—a human-readable format designed for security engineers—and automatically analyze EVM smart contracts, Solana programs, and Move modules against those specifications.

Unlike monolithic tools that specialize in a single chain, Sentri abstracts away platform differences. You catch vulnerabilities before deployment with the same level of rigor across all three major ecosystems, reducing the cognitive load of multi-chain development and eliminating the need to learn three different security analysis tools.

---

## Features

- **22 built-in security invariants** covering reentrancy, access control, arithmetic overflow, state consistency, and more across EVM (10), Solana (7), and Move (5)
- **Custom invariant DSL** — define complex multi-condition security rules in readable, declarative syntax
- **Multi-format reports** — generate JSON, HTML, or colored text output with detailed violation context
- **Violation suppression** — inline comments and `.sentri.toml` configuration for managing false positives
- **Professional CLI** — color-coded severity levels, progress spinners, and detailed error messages
- **CI/CD integration** — exit codes and thresholds for automated enforcement in pipelines
- **Cross-platform** — Linux, macOS (Intel and Apple Silicon), and Windows binaries pre-built
- **Health checks** — `sentri doctor` command verifies environment and dependencies

---

## Installation

### Via Cargo (Recommended)

```bash
cargo install sentri-cli
```

### Pre-built Binaries

Download from [the latest release](https://github.com/geekstrancend/Sentri/releases/latest):

| Platform | Download |
|----------|----------|
| Linux x86_64 (glibc) | `sentri-x86_64-unknown-linux-gnu` |
| Linux x86_64 (musl) | `sentri-x86_64-linux-musl` |
| Linux aarch64 | `sentri-aarch64-unknown-linux-gnu` |
| macOS Intel (x86_64) | `sentri-x86_64-apple-darwin` |
| macOS Apple Silicon | `sentri-aarch64-apple-darwin` |
| Windows x86_64 | `sentri-x86_64-pc-windows-msvc.exe` |

After downloading, mark executable and add to PATH:
```bash
chmod +x sentri-x86_64-unknown-linux-gnu
sudo mv sentri-x86_64-unknown-linux-gnu /usr/local/bin/sentri
```

### Verify Installation

```bash
sentri --version
sentri doctor
```

Expected output:
```
sentri doctor

Sentri Diagnostic Report
─────────────────────────────────────────────────────────────────

│ Sentri Version: 0.1.3                                         │
│ Rust Version: 1.75.0  ✓                                       │
│ Platform: Linux x86_64  ✓                                     │
│ Built-in Invariants: 22  ✓                                    │
│                                                               │
│ Status: All systems operational                              │
```

---

## Quick Start

### Step 1: Initialize Your Project

```bash
sentri init
```

This creates a `.sentri.toml` file in your current directory:

```toml
[project]
name = "my-protocol"
chain = "evm"

[analysis]
severity_threshold = "low"
ignore = []

[invariants]
built_in = []
custom = ["invariants/"]

[output]
format = "text"
```

### Step 2: Run Your First Check

```bash
sentri check ./contracts --chain evm
```

Example output:

```
Sentri Invariant Checker
────────────────────────────────────────────────────────────

✓ Scanning contracts...
  Found: Token.sol, Staking.sol

✓ Parsing invariants...
  Built-in: 10 EVM invariants

▶ Analyzing Token.sol...
  [████████] 100% complete

⚠ CRITICAL Violation Found

╭─ Token.sol:201 ─ Reentrancy Vulnerability ─────────────────╮
│                                                             │
│  ✗ CRITICAL                                                │
│                                                             │
│ Invariant   test_no_reentrancy                             │
│ Location    Token.sol:201-215 (function transfer)          │
│ Severity    Critical                                       │
│ CWE         CWE-252 · Unchecked Return Value               │
│                                                             │
│ → Use checks-effects-interactions pattern                 │
│ → Move external calls to end of function                  │
│ → Use OpenZeppelin ReentrancyGuard                         │
│                                                             │
│ Reference   docs.sentri.dev/invariants/reentrancy         │
│                                                             │
╰─────────────────────────────────────────────────────────────╯

═══════════════════════════════════════════════════════════════
SUMMARY
═══════════════════════════════════════════════════════════════

  Critical:  1
  High:      0
  Medium:    0
  Low:       0

Status: FAILED (1 critical violation)
```

### Step 3: Generate a JSON Report

```bash
sentri check ./contracts --chain evm --format json --output report.json
```

### Step 4: Add to GitHub Actions

Create `.github/workflows/sentri.yml`:

```yaml
name: Sentri Invariant Checker

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]

jobs:
  sentri:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Sentri
        run: cargo install sentri-cli
      
      - name: Run invariant checks
        run: sentri check ./contracts --chain evm --fail-on high
```

---

## Built-in Invariants

Sentri includes 22 pre-configured security checks covering common vulnerability patterns:

### EVM (Solidity/Vyper) — 10 Invariants

| ID | Name | Severity | CWE | Description |
|----|------|----------|-----|-------------|
| EVM_001 | Reentrancy | Critical | CWE-252 | External calls before state changes |
| EVM_002 | Integer Overflow | High | CWE-190 | Unchecked arithmetic operations |
| EVM_003 | Integer Underflow | High | CWE-191 | Unsigned subtraction below zero |
| EVM_004 | Unchecked Return Value | High | CWE-252 | Ignoring external call failures |
| EVM_005 | Delegatecall Injection | Critical | CWE-95 | Unsafe delegatecall to user input |
| EVM_006 | Access Control | High | CWE-276 | Missing or incorrect permission checks |
| EVM_007 | Timestamp Dependence | Medium | CWE-829 | Relying on block.timestamp for randomness |
| EVM_008 | Front-running | Medium | CWE-362 | Predictable order of execution |
| EVM_009 | Uninitialized Storage Pointer | High | CWE-824 | Uninitialized local variables |
| EVM_010 | Division by Zero | High | CWE-369 | Denominator not validated |

### Solana (Rust Programs) — 7 Invariants

| ID | Name | Severity | CWE | Description |
|----|------|----------|-----|-------------|
| SOL_001 | Missing Signer Check | Critical | CWE-276 | Instruction signer not verified |
| SOL_002 | Missing Account Validation | High | CWE-476 | Account ownership not checked |
| SOL_003 | Integer Overflow | High | CWE-190 | Unchecked arithmetic in program |
| SOL_004 | Rent Exemption | Medium | CWE-200 | Account rent not validated |
| SOL_005 | PDA Derivation Mismatch | High | CWE-347 | Incorrect PDA seed validation |
| SOL_006 | Insufficient Lamports | Medium | CWE-607 | Insufficient lamports for operation |
| SOL_007 | Instruction Parsing Error | High | CWE-20 | Insufficient input validation |

### Move (Aptos/Sui) — 5 Invariants

| ID | Name | Severity | CWE | Description |
|----|------|----------|-----|-------------|
| MOVE_001 | Unauthorized Access | Critical | CWE-276 | Caller not authorized for operation |
| MOVE_002 | Integer Overflow | High | CWE-190 | Unchecked arithmetic operations |
| MOVE_003 | Resource Leak | High | CWE-772 | Resource not properly destroyed |
| MOVE_004 | Type Mismatch | Medium | CWE-686 | Type safety violation in generics |
| MOVE_005 | Missing Signer | Critical | CWE-276 | Signer proof not verified |

---

## Configuration Reference

Complete `.sentri.toml` with all configurable options:

```toml
[project]
name = "my-protocol"              # Project name for reports
chain = "evm"                     # Target chain: evm | solana | move

[analysis]
severity_threshold = "low"        # Report violations at or above this level
ignore = [                        # Invariant IDs to skip
  "EVM_007",
  "EVM_008",
]

[invariants]
built_in = []                     # Empty = use all; list specific IDs to use subset
custom = [                        # Paths to custom .invar files
  "invariants/",
  "security/rules.invar"
]

[output]
format = "text"                   # text | json | html
# file = "report.json"            # Optional: write to file instead of stdout

# Suppress specific violations
[[suppress]]
id = "EVM_007"
location = "Token.sol:89"
reason = "Intentional design decision — documented in ADR-042"
expires = "2026-12-31"

[[suppress]]
id = "EVM_001"
function = "emergencyWithdraw"
reason = "Protected by timelocks and external monitoring"
```

---

## CLI Reference

### Main Commands

**check** — Analyze contracts against invariants

```bash
sentri check <path> [OPTIONS]

Options:
  --chain <CHAIN>           Target chain [evm|solana|move]
  --format <FORMAT>         Output format [text|json|html]
  --output <FILE>           Write report to file
  --fail-on <SEVERITY>      Fail if violations at this level or higher
  --exclude-ids <IDS>       Skip invariant IDs (comma-separated)
  -v, --verbose             Verbose output
```

**init** — Initialize a new Sentri project

```bash
sentri init [--chain evm] [--path ./]
```

**doctor** — Run diagnostic checks

```bash
sentri doctor [-v|--verbose]
```

**--version** — Show version

```bash
sentri --version
```

---

## CI/CD Integration

### GitHub Actions

```yaml
name: Sentri Invariant Check

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  sentri:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: cargo install sentri-cli
      - run: sentri check ./contracts --chain evm --fail-on high
```

### Fail Thresholds by Environment

```bash
# Strict on main branch
if [ "$GITHUB_REF" = "refs/heads/main" ]; then
  sentri check ./contracts --fail-on high
else
  sentri check ./contracts --fail-on critical
fi
```

---

## Roadmap

| Version | Focus | Status |
|---------|-------|--------|
| **0.1** | Multi-chain baseline, 22 checks, DSL, reports | ✅ Released |
| **0.2** | AST analysis, call graphs, taint tracking | 🔄 In Progress |
| **0.3** | Runtime fuzzing (revm, solana-program-test) | 📋 Planned |
| **0.4** | SMT solver integration, bounded model checking | 📋 Planned |
| **0.5** | Z3 symbolic execution, formal verification | 📋 Planned |

---

## Contributing

### Running Tests

```bash
cargo test --all
cargo clippy --all -- -D warnings
cargo fmt --all
```

### Submitting Changes

1. Fork repository
2. Create feature branch: `git checkout -b feature/name`
3. Make changes and test
4. Commit: `git commit -m "feat: description"`
5. Push and open PR

### Reporting Issues

- **Security**: Email security@sentri.dev
- **Bugs**: [GitHub Issues](https://github.com/geekstrancend/Sentri/issues)
- **False Positives**: Especially valuable — include code samples
- **Features**: [GitHub Discussions](https://github.com/geekstrancend/Sentri/discussions)

---

## License

MIT License — see [LICENSE](LICENSE) file

---

**GitHub**: [geekstrancend/Sentri](https://github.com/geekstrancend/Sentri)  
**Crates.io**: [sentri-cli](https://crates.io/crates/sentri-cli)  
**Discussions**: [GitHub Discussions](https://github.com/geekstrancend/Sentri/discussions)
