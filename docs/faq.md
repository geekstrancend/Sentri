# Frequently Asked Questions

## Installation

### Q: How do I install Sentri?

**A:** Multiple options:

```bash
# Using curl (recommended)
curl -fsSL https://install.sentri.dev | bash

# Using Homebrew
brew install sentri

# From source
git clone https://github.com/geekstrancend/Sentri.git
cd Sentri
cargo install --path crates/cli

# Using Docker
docker pull geekstrancend/sentri:latest
```

### Q: I get "command not found: sentri"

**A:** Add Sentri to your PATH:

```bash
# Find where it's installed
which sentri

# If empty, add to PATH in ~/.bashrc or ~/.zshrc
export PATH="$PATH:$HOME/.sentri/bin"

# Reload shell
source ~/.bashrc  # or ~/.zshrc
```

### Q: How do I update Sentri?

**A:** Use the update command:

```bash
sentri update --yes

# Or reinstall
curl -fsSL https://install.sentri.dev | bash
```

## Configuration

### Q: How do I initialize a project?

**A:** Run init command:

```bash
sentri init --project myproject

# Creates:
# myproject/sentri.toml
# myproject/invariants.invar
# myproject/.invarignore
```

### Q: Where should I put sentri.toml?

**A:** In your project root:

```
my-project/
├── sentri.toml          ← Here
├── invariants.invar    ← Or in subdirectory
├── src/
└── contracts/
```

Can also reference from anywhere:

```bash
sentri analyze --config /path/to/sentri.toml
```

### Q: How do I exclude files?

**A:** Use `.invarignore`:

```
# Skip test and example files
**/test/**
**/tests/**
**/examples/**

# Skip specific chains
**/solana/**
contracts/experimental/**
```

## Writing Invariants

### Q: What's the difference between `invariant` and `global`?

**A:** 

- `invariant: name` - Declares the invariant
- `global: condition` - Specifies what must always be true
- `context: block` - Optional conditional context

Example:
```invar
invariant: vault_conservation
context:
    type: Solana
    program: SPL_TOKEN_VAULT
global:
    vault.lamports == sum(accounts.lamports)
```

### Q: How do I check multiple conditions?

**A:** Use logical operators:

```invar
invariant: complex_condition
global:
    (amount > 0 && amount < MAX) ||
    (status == "approved" && amount <= LIMIT)
```

### Q: Can I use comments in DSL?

**A:** Yes, use `#`:

```invar
# This is a comment
invariant: example
global:
    x > 0  # x must be positive
```

### Q: What types are supported?

**A:** Basic types:
- `int` - 256-bit signed integer
- `uint` - 256-bit unsigned integer
- `address` - Blockchain address
- `string` - Text string
- `bool` - Boolean
- `bytes` - Raw bytes

Collections:
- `[T]` - Array of type T
- `{K: V}` - Map from K to V

## Running Analysis

### Q: What's the difference between analyze and check?

**A:**

```bash
# Full analysis with report
sentri analyze --config sentri.toml

# Quick check (pass/fail)
sentri check --config sentri.toml --strict
```

### Q: How do I run only specific invariants?

**A:** Use filtering:

```bash
# Include specific invariant
sentri analyze --include vault_conservation

# Exclude patterns
sentri analyze --exclude experimental_*

# Only specific chain
sentri analyze --chain solana
```

### Q: What do the exit codes mean?

**A:**

| Code | Meaning |
|------|---------|
| 0 | ✅ All invariants satisfied |
| 1 | ❌ Violation detected |
| 2 | ⚠️ Configuration error |
| 3 | 🔥 Internal error |

Use in scripts:

```bash
sentri analyze --config sentri.toml
case $? in
  0) echo "Success" ;;
  1) exit 1  # Halt build
  ;;
  *) exit 1  # Error
  ;;
esac
```

### Q: How do I get JSON output?

**A:** Use output flag:

```bash
sentri analyze --output json

# Or pipe to jq
sentri analyze --output json | jq '.summary'
```

## Performance

### Q: Sentri is slow, how do I speed it up?

**A:** Several strategies:

```bash
# 1. Skip slow invariants
sentri analyze --exclude slow_*

# 2. Analyze specific chain only
sentri analyze --chain solana

# 3. Limit analysis scope
sentri analyze --path src/ --path contracts/

# 4. Skip caching
sentri analyze --no-cache
```

### Q: How do I profile performance?

**A:** Check timing:

```bash
time sentri analyze --config sentri.toml

# More detailed
RUST_LOG=debug sentri analyze --config sentri.toml
```

### Q: What's taking the most time?

**A:** Run with timing output:

```bash
sentri analyze --config sentri.toml --verbose --verbose
# Shows timing for each phase
```

## Errors and Debugging

### Q: I get a parse error but my DSL looks correct

**A:** Common issues:

```invar
# ❌ Missing global block
invariant: test

# ✅ Add global block
invariant: test
global: x > 0

# ❌ Invalid type
global: "string" + 42

# ✅ Type variables correctly
global: "string_" + string_value
```

Check syntax at https://sentri-dsl.dev/syntax

### Q: How do I debug my invariant?

**A:** Use logging:

```bash
RUST_LOG=debug sentri analyze --config sentri.toml --verbose

# See what's being evaluated
```

Print intermediate values using separate invariants:

```invar
# Check step by step
invariant: debug_step_1
global: x > 0

invariant: debug_step_2
global: x < 100

invariant: debug_step_3
global: x % 2 == 0
```

### Q: "Unknown variable" error

**A:** Variable not defined in context:

```invar
# ❌ undefined_var doesn't exist
invariant: test
global: undefined_var > 0

# ✅ Use variables from context
invariant: test
context:
    type: Solana
    account: token_account
global:
    account.amount > 0
```

### Q: Type mismatch error

**A:** Ensure type consistency:

```invar
# ❌ Can't compare string to number
global: accounts[0].name == 42

# ✅ Compare same types
global: accounts[0].owner == expected_owner
```

## CI/CD

### Q: How do I add Sentri to GitHub Actions?

**A:** Simple workflow:

```yaml
name: Invariant Check
on: [push, pull_request]

jobs:
  sentri:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Run Invariants
        run: |
          curl -fsSL https://install.sentri.dev | bash
          sentri analyze --config sentri.toml --strict
```

### Q: How do I prevent merge if invariants fail?

**A:** Use branch protection rules in GitHub:
1. Go to Settings → Branches
2. Add rule for main branch
3. Require "Invariant Check" to pass

### Q: Can I run Sentri in Docker?

**A:** Yes:

```bash
docker run -v /path/to/project:/project zelius/sentri:latest \
  analyze --config /project/sentri.toml
```

Or in docker-compose:

```yaml
services:
  sentri:
    image: geekstrancend/sentri:latest
    volumes:
      - ./invariants.invar:/app/invariants.invar
    command: analyze --config /app/sentri.toml
```

## Security

### Q: Is it safe to run Sentri on untrusted code?

**A:** Yes, Sentri uses sandboxing:
- No filesystem access
- No network access
- No external execution
- Bounded memory and time

See [Security Model](docs/security-model.md) for details.

### Q: Should I use Sentri instead of audits?

**A:** No, use together:
1. **Sentri** - Checks invariants automatically
2. **Audit** - Reviews logic and design
3. **Testing** - Unit and integration tests
4. **Monitoring** - Runtime observation

### Q: How do I report security issues?

**A:** Don't use GitHub Issues for security.

Email: security@sentri.dev

Include:
- Description of vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (optional)

## Technical

### Q: What's the difference between Sentri versions?

**A:** Check [Versioning Policy](docs/versioning.md):

- **v0.x** - Unstable API (changing)
- **v1.x** - Stable API (backward compatible)
- **Patch** (v1.0.x) - Bug fixes only

### Q: Can I use Sentri with language X?

**A:** Depends on the blockchain:

| Language | Status |
|----------|--------|
| Solidity (EVM) | ✅ Full support |
| Rust (Solana) | ✅ Full support |
| Move (Aptos/SUI) | ✅ Full support |
| AssemblyScript | ⚠️ Experimental |
| Other | ❌ Not supported |

Request new language support in GitHub Issues.

### Q: Can I extend Sentri?

**A:** Limited extension support:

- [Write custom invariants](docs/writing-invariants.md)
- [Contribute new features](CONTRIBUTING.md)

For plugin system, see [Roadmap](README.md#Roadmap).

### Q: What resources does Sentri use?

**A:** Typical usage:

- **Memory**: 10-100MB for small projects
- **CPU**: Single-thread analysis
- **Disk**: ~50MB for binary + dependencies
- **Network**: Only for updates (opt-in)

## Still Have Questions?

### Resources

- [Documentation](docs/README.md) - Complete guides
- [Examples](examples/) - Real-world usage
- [GitHub Issues](https://github.com/geekstrancend/Sentri/issues) - Ask community
- [GitHub Discussions](https://github.com/geekstrancend/Sentri/discussions) - Discussions
- [Discord](https://discord.gg/sentri) - Real-time chat

### Getting Help

1. Search existing issues/discussions
2. Check documentation relevant to your problem
3. Post in GitHub Discussions
4. If critical bug, report as issue with `[bug]` tag

## Contributing

Found a bug or want to improve Sentri?

- [Contributing Guide](CONTRIBUTING_EXTENDED.md)
- [Code of Conduct](CODE_OF_CONDUCT.md)
- Pull requests welcome!

---

**Last updated:** 2024-01-15  
**Sentri version:** 0.1.0
