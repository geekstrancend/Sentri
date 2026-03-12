# Getting Started with Sentri

## What is Sentri?

Sentri is a **production-grade invariant enforcement system** for smart contracts across Solana, EVM, and Move chains.

It lets you:
- Define invariants in a simple DSL
- Automatically verify them across chain interactions
- Run security checks for common vulnerabilities
- Get clear violation reports
- Prevent bugs before they happen

## Choose Your Path

Sentri supports two main workflows:

1. **Security Analysis** (Recommended for first-time users) — Quickly scan smart contracts for common security vulnerabilities
2. **Invariant Definition** (Advanced) — Define custom properties that your contracts must always satisfy

---

## Path 1: Security Analysis (First-Time Users)

### Installation

### Step 1: Install Sentri

#### **Option A: Cargo (Recommended for Rust Projects)**
```bash
cargo install sentri
```

This installs sentri globally and adds it to your PATH (usually `~/.cargo/bin/sentri`).

#### **Option B: Homebrew (macOS)**
```bash
brew install sentri
```

#### **Option C: Build from Source**
```bash
git clone https://github.com/geekstrancend/sentri.git
cd sentri
cargo install --path .
```

#### **Option D: Download Pre-built Binary**
Visit the [Sentri Releases](https://github.com/geekstrancend/sentri/releases) page and download the binary for your OS (Linux, macOS, Windows).

Extract and add to PATH:
```bash
# Example for Linux
tar -xzf sentri-v0.1.10-x86_64-unknown-linux-gnu.tar.gz
sudo mv sentri /usr/local/bin/
```

### Step 2: Verify Installation
```bash
sentri --version
```

Expected output:
```
sentri 0.1.10
```

Verify sentri is accessible:
```bash
which sentri
# Output: /home/user/.cargo/bin/sentri (or similar)
```

If command not found, add to PATH:
```bash
# For Cargo installation
export PATH="$HOME/.cargo/bin:$PATH"

# Make permanent (add to ~/.bashrc or ~/.zshrc)
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

### Step 3: Verify Dependencies
```bash
sentri doctor
```

This checks that all Sentri components are working correctly. Output example:
```
✓ Analyzer ready
✓ Database loaded
✓ Config parser working
```

### Step 4: Navigate to Your Project
```bash
cd /path/to/your/solana-project
```

Ensure your project has:
- Smart contract code (Solana `.rs` files, EVM `.sol` files, etc.)
- Cargo.toml or appropriate build configuration

### Step 5: Initialize Configuration
```bash
sentri init
```

This creates .sentri.toml in your project root with default settings. Edit it to match your project:

```toml
[checks]
# Solana-specific invariants
enabled = [
  "SOL_001",  # Missing signer checks
  "SOL_002",  # Account validation failures
  "SOL_003",  # Integer overflow
  "SOL_004",  # Rent exemption violations
  "SOL_005",  # PDA derivation errors
  "SOL_006",  # Lamport balance issues
  "SOL_007",  # Instruction parsing failures
]

[report]
format = "json"
output = "sentri-report.json"
fail_on = "high"

[ignore]
files = ["target/**", "node_modules/**", "tests/**"]
```

**Configuration Options:**

| Section | Option | Values | Purpose |
|---------|--------|--------|---------|
| **[checks]** | enabled | `SOL_001` - `SOL_007` (Solana) | Which security checks to run |
| **[report]** | format | `json`, `text`, `html` | Output format |
| **[report]** | output | file path | Where to save report |
| **[report]** | fail_on | `low`, `medium`, `high`, `critical` | Exit code threshold |
| **[ignore]** | files | glob patterns | Directories to skip |

### Step 6: Run Initial Analysis

#### **Basic Command**
```bash
sentri check <PATH> --chain <BLOCKCHAIN>
```

#### **For Solana Projects**
```bash
sentri check ./programs/geekslibrary/src/lib.rs --chain solana
```

#### **With Options**
```bash
# Human-readable output with details
sentri check ./programs/geekslibrary/src/lib.rs --chain solana --verbose

# Save JSON report
sentri check ./programs/geekslibrary/src/lib.rs --chain solana --format json --output sentri-report.json

# Fail if violations found
sentri check ./programs/geekslibrary/src/lib.rs --chain solana --fail-on high
```

**Supported Chains:**
- `solana` — Solana smart contracts
- `evm` — Ethereum and EVM-compatible chains (default)
- `move` — Move (Aptos, Sui)

**Output Format Options:**
- `--format text` — Colored, boxed human-readable (default)
- `--format json` — Machine-readable, one object per line
- `--format html` — HTML report with styling

### Step 7: Analyze Results

Output structure:
```
Sentri · Multi-chain Invariant Checker · v0.1.10

Target  ./programs/geekslibrary/src/lib.rs
Chain  Solana

✓ 22 checks in 0.0s

Violations (7)
├─ 1. Solana invariant: Signer Checks [HIGH]
├─ 2. Solana invariant: Account Validation [HIGH]
├─ 3. Solana invariant: Integer Overflow [HIGH]
├─ 4. Solana invariant: Rent Exemption [HIGH]
├─ 5. Solana invariant: PDA Derivation [HIGH]
├─ 6. Solana invariant: Lamport Balance [HIGH]
└─ 7. Solana invariant: Instruction Parsing [HIGH]

Passed Checks (15)
✓ balance_conservation, access_control_present, etc.

Analysis Summary
├─ Target: ./programs/geekslibrary/src/lib.rs
├─ Chain: Solana
├─ Checks: 22 total · 7 violations · 15 passed
└─ Status: ✗ FAIL — violations found
```

For each violation, examine:
1. **Location** — File path and line number
2. **Severity** — CRITICAL/HIGH/MEDIUM/LOW
3. **CWE** — Common Weakness Enumeration reference
4. **Vulnerable Code** — Actual code snippet showing the issue
5. **Recommendation** — Detailed fix with code examples
6. **Reference** — Links to official documentation

### Step 8: Fix Each Violation

Example fix for **Missing Signer Checks**:

**Before (Vulnerable):**
```rust
#[derive(Accounts)]
pub struct AddBook<'info> {
    #[account(mut)]
    pub book: Account<'info, Book>,
    pub library: Account<'info, Library>,
}
```

**After (Fixed):**
```rust
#[derive(Accounts)]
pub struct AddBook<'info> {
    #[account(mut)]
    pub book: Account<'info, Book>,
    #[account(mut, signer)]  // ← Add signer requirement
    pub library: Account<'info, Library>,
}
```

Common fixes:
- Add `signer` constraint: `#[account(mut, signer)]`
- Add explicit checks: `require!(account.is_signer, ErrorCode::MustBeSigner)?`
- Use checked arithmetic: `amount.checked_add(fee)?`
- Validate ownership: `require!(account.owner == &system_program::ID)?`

### Step 9: Re-run Analysis

After making fixes:
```bash
sentri check ./programs/geekslibrary/src/lib.rs --chain solana --verbose
```

Verify:
- Violation count decreases
- High-severity issues are resolved
- Status changes to ✓ PASS

### Step 10: Generate Final Report

Create a JSON report for documentation/CI:
```bash
sentri check ./programs/geekslibrary/src/lib.rs --chain solana --format json --output sentri-report.json
```

View the report:
```bash
cat sentri-report.json
```

### Step 11: Set Up for CI/CD (Optional)

Add to your build pipeline (GitHub Actions, GitLab CI, etc.):

```yaml
# GitHub Actions example
- name: Run Sentri Security Analysis
  run: |
    sentri check ./programs/geekslibrary/src/lib.rs \
      --chain solana \
      --fail-on high \
      --format json \
      --output sentri-report.json
```

Exit codes:
- `0` — All checks passed
- `1` — Violations found at/above `fail_on` threshold

### Step 12: Continuous Monitoring

Track violations over time:
```bash
# Run daily and save timestamped reports
sentri check ./programs/geekslibrary/src/lib.rs \
  --chain solana \
  --format json \
  --output reports/sentri-$(date +%Y%m%d).json
```

---

## Quick Reference: Security Analysis Commands

```bash
# 1. Install
cargo install sentri

# 2. Verify
sentri --version
sentri doctor

# 3. Navigate to project
cd /path/to/project

# 4. Initialize config
sentri init

# 5. Run analysis
sentri check ./path/to/contract.rs --chain solana --verbose

# 6. Save report
sentri check ./path/to/contract.rs --chain solana --format json --output sentri-report.json

# 7. Re-verify after fixes
sentri check ./path/to/contract.rs --chain solana --fail-on high
```

---

## Troubleshooting

| Problem | Solution |
|---------|----------|
| `sentri: command not found` | Add `~/.cargo/bin` to PATH or reinstall with `cargo install sentri` |
| `Failed to analyze EVM contract` | Use correct `--chain` flag; specify file not directory |
| `FAIL — violations found` | Exit code 1; check sentri-report.json for details |
| `Is a directory` error | Provide path to `.rs` or `.sol` file, not folder |
| Config file not found | Run `sentri init` in project root to create .sentri.toml |

---

## Path 2: Invariant Definition (Advanced Users)

### Initialize New Projects

```bash
sentri init my-project
sentri init --template solana my-solana-vault
sentri init --template evm my-evm-token
```

## Writing Invariants

### Structure

Every invariant has:

```invar
invariant: unique_name
description: "Human-readable description"
category: category_name

[context { ... }]  # Optional

[forall <variable> in <collection>:
    <condition>]   # Optional

[global:
    <condition>]   # Optional (at least one required)
```

### Examples

#### 1. Simple Balance Check

```invar
invariant: positive_balance
description: "Account balance must be non-negative"

global:
    balance >= 0
```

#### 2. Solana Vault with Authority

```invar
invariant: vault_authority_immutable
description: "Vault authority cannot change"

context {
    state: VaultState,
    chain: Solana
}

global:
    state.authority == initial_authority

forall update in state.pending_updates:
    update.type != "authority_change"
```

#### 3. EVM Token Mint

```invar
invariant: total_supply_conservation
description: "Sum of balances equals total supply"
chain: EVM

global:
    sum(balances) == totalSupply &&
    totalSupply <= MAX_UINT256

forall mint_op in recent_mints:
    mint_op.to_account != address(0)
```

#### 4. Governance Quorum

```invar
invariant: governance_quorum_enforcement
description: "Proposals must meet quorum"
category: governance

forall proposal in active_proposals:
    proposal.votes_for + proposal.votes_against <= total_voting_power() &&
    proposal.end_block > current_block()

global:
    count_active_proposals <= MAX_CONCURRENT_PROPOSALS
```

## Integration with CI/CD

### GitHub Actions

Add to `.github/workflows/check.yml`:

```yaml
name: Sentri Check

on: [push, pull_request]

jobs:
  invariants:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Sentri
        run: cargo install sentri
      
      - name: Check invariants
        run: sentri check invariants/
```

### Local Pre-commit Hook

Create `.git/hooks/pre-commit`:

```bash
#!/bin/bash
set -e
sentri check invariants/ || exit 1
echo "Invariants passed"
```

Make executable:
```bash
chmod +x .git/hooks/pre-commit
```

## Configuration

### sentri.toml

```toml
[project]
name = "my-vault"
version = "0.1.0"
chains = ["solana", "evm"]

[invariants]
paths = ["invariants/"]
categories = ["finance", "security"]

[analysis]
coverage_target = 90
strict_mode = true
parallel = true

[output]
format = "json"
colorize = true
```

## Understanding Errors

### Parse Error

```
Error: Failed to parse invariants/vault.invar
  Line 4: Missing colon after 'invariant'
  
  invariant vault_conservation
           ^ Expected ':'
```

**Fix:** Add colon after invariant name:
```invar
invariant: vault_conservation
```

### Type Error

```
Error: Type mismatch in vault.invar:9
  Cannot compare string to integer
  
  deposit.owner == 42
           ^^^^^ type: String
           ^^   type: Integer
```

**Fix:** Use correct types in comparison.

### Evaluation Error

```
Error: Invariant violated in vault.invar:5
  vault_conservation failed
  
  State: {vault_balance: 1000, deposits_sum: 900}
  Condition: sum(deposits) == vault_balance
  Result: 900 == 1000 → FALSE
```

**Fix:** Correct contract logic or invariant definition.

## Troubleshooting

### "Command not found: sentri"

```bash
# Make sure it's installed
which sentri

# If not found, install
cargo install sentri

# Or add to PATH
export PATH="$PATH:$HOME/.cargo/bin"
```

### "No such file or directory"

```bash
# Make sure you're in the right directory
pwd

# List your files
ls -la invariants/

# Use absolute paths if needed
sentri check /full/path/to/vault.invar
```

### "Invalid invariant syntax"

```bash
# Run with verbose mode
sentri check --verbose invariants/

# Check for common mistakes:
# - Missing ':' after 'invariant'
# - Incorrect indentation
# - Typos in keywords
```

## Next Steps

1. **Read [Writing Invariants](writing-invariants.md)** - Deep dive into DSL
2. **Check [Examples](example-invariants.md)** - Real-world patterns
3. **CI Integration** - Add to your pipeline
4. **Security Model** - Understand guarantees
5. **Contributing** - Help improve Sentri

## Getting Help

- **Documentation**: [https://sentri.dev/docs](https://sentri.dev/docs)
- **Issues**: [GitHub Issues](https://github.com/geekstrancend/Sentri/issues)
- **Discussions**: [GitHub Discussions](https://github.com/geekstrancend/Sentri/discussions)
- **Security**: [security@sentri.dev](mailto:security@sentri.dev)

## Key Concepts

### Exit Codes

```bash
sentri check vault.invar
echo $?  # Exit code
```

- **0** = Success (invariants passed)
- **1** = Invariant violation
- **2** = Configuration error
- **3** = Internal error

### Output Formats

```bash
# Machine-readable JSON
sentri check --format json vault.invar

# Human-readable Markdown
sentri check --format markdown vault.invar

# CI-friendly SARIF
sentri check --format sarif vault.invar
```

## Performance Notes

For large analyses:

```bash
# Enable parallel processing (default: on)
sentri check --parallel

# Use specific number of threads
sentri check --threads 4

# Disable parallelism for debugging
sentri check --threads 1
```

## Support Level

| Component | Status |
|-----------|--------|
| Solana | Production Ready |
| EVM | Production Ready |
| Move | Beta |
| DSL | Stable |
| CLI | Stable |
