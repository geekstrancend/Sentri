# Getting Started with Invar

## What is Invar?

Invar is a **production-grade invariant enforcement system** for smart contracts across Solana, EVM, and Move chains.

It lets you:
- Define invariants in a simple DSL
- Automatically verify them across chain interactions
- Get clear violation reports
- Prevent bugs before they happen

## Installation

### From GitHub Releases

```bash
# Download latest release
curl -L https://github.com/invar/invar/releases/download/latest/invar-linux-x86_64 \
  -o /usr/local/bin/invar
chmod +x /usr/local/bin/invar

# Verify installation
invar --version
```

### From Source

```bash
git clone https://github.com/invar/invar.git
cd invar
cargo install --path crates/cli

# Verify
invar --version
```

### With Cargo

```bash
cargo install invar
```

## Quick Start

### 1. Initialize a Project

```bash
invar init my-vault
cd my-vault
```

Creates:
```
my-vault/
├── invar.toml          # Project configuration
├── invariants/         # Invariant definitions
│   └── vault.invar
├── src/                # Your contract code
└── tests/              # Test files
```

### 2. Write an Invariant

Create `invariants/vault.invar`:

```invar
invariant: vault_conservation
description: "Total deposits always equal vault balance"
category: finance

context {
    state: VaultState,
    chain: Solana
}

// The core invariant: conservation law
global:
    sum(state.deposits[*].amount) == state.vault_balance

// Individual deposit constraints
forall deposit in state.deposits:
    deposit.amount > 0 &&
    deposit.created_at <= now()
```

### 3. Add Your Contract Code

Place your Rust/Solidity/Move code in `src/`:

```bash
# Copy your contract
cp ~/my-contract/lib.rs src/

# Or write a new one
cat > src/vault.rs << 'EOF'
pub struct Vault {
    pub deposits: Vec<Deposit>,
}

pub struct Deposit {
    pub owner: String,
    pub amount: u64,
}
EOF
```

### 4. Run Analysis

```bash
invar check invariants/vault.invar
```

Output:
```
vault_conservation
   Status: PASS
   Time: 2.3ms

Analysis complete: 1 invariant, 1 passed
```

## Common Commands

### Check Invariants

```bash
# Check single file
invar check vault.invar

# Check entire directory
invar check invariants/

# Verbose output
invar check --verbose invariants/
```

### Generate Reports

```bash
# JSON report
invar report --format json invariants/vault.invar > report.json

# Markdown report
invar report --format markdown invariants/vault.invar > report.md

# Plain text
invar report --format text invariants/vault.invar
```

### Initialize New Projects

```bash
invar init my-project
invar init --template solana my-solana-vault
invar init --template evm my-evm-token
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
name: Invar Check

on: [push, pull_request]

jobs:
  invariants:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Invar
        run: cargo install invar
      
      - name: Check invariants
        run: invar check invariants/
```

### Local Pre-commit Hook

Create `.git/hooks/pre-commit`:

```bash
#!/bin/bash
set -e
invar check invariants/ || exit 1
echo "Invariants passed"
```

Make executable:
```bash
chmod +x .git/hooks/pre-commit
```

## Configuration

### invar.toml

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

### "Command not found: invar"

```bash
# Make sure it's installed
which invar

# If not found, install
cargo install invar

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
invar check /full/path/to/vault.invar
```

### "Invalid invariant syntax"

```bash
# Run with verbose mode
invar check --verbose invariants/

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
5. **Contributing** - Help improve Invar

## Getting Help

- **Documentation**: [https://invar.sh/docs](https://invar.sh/docs)
- **Issues**: [GitHub Issues](https://github.com/invar/invar/issues)
- **Discussions**: [GitHub Discussions](https://github.com/invar/invar/discussions)
- **Security**: [security@invar-project.dev](mailto:security@invar-project.dev)

## Key Concepts

### Exit Codes

```bash
invar check vault.invar
echo $?  # Exit code
```

- **0** = Success (invariants passed)
- **1** = Invariant violation
- **2** = Configuration error
- **3** = Internal error

### Output Formats

```bash
# Machine-readable JSON
invar check --format json vault.invar

# Human-readable Markdown
invar check --format markdown vault.invar

# CI-friendly SARIF
invar check --format sarif vault.invar
```

## Performance Notes

For large analyses:

```bash
# Enable parallel processing (default: on)
invar check --parallel

# Use specific number of threads
invar check --threads 4

# Disable parallelism for debugging
invar check --threads 1
```

## Support Level

| Component | Status |
|-----------|--------|
| Solana | Production Ready |
| EVM | Production Ready |
| Move | Beta |
| DSL | Stable |
| CLI | Stable |
