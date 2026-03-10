# @dextonicx/cli

[![npm version](https://img.shields.io/npm/v/@dextonicx/cli.svg)](https://www.npmjs.com/package/@dextonicx/cli)
[![npm downloads](https://img.shields.io/npm/dm/@dextonicx/cli.svg)](https://www.npmjs.com/package/@dextonicx/cli)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

**Multi-chain smart contract invariant checker** for **EVM** (Solidity), **Solana** (Rust/Anchor), and **Move** (Aptos/Sui).

Runs static analysis on your blockchain code before deployment. Checks 22 built-in security patterns across all three blockchain ecosystems.

> **✅ v0.1.8+**: Fixed critical hang issue. Use the latest version for stable operation.

## Table of Contents

1. [Installation](#installation)
2. [Quick Start](#quick-start)
3. [Implementation Guide](#implementation-guide)
4. [Project Integration](#project-integration)
5. [CI/CD Setup](#cicd-setup)
6. [Troubleshooting](#troubleshooting)

---

## Installation

### Step 1: Choose Your Installation Method

#### **Option A: Global Install (Easiest)**
Use Sentri from anywhere on your machine:

```bash
npm install -g @dextonicx/cli@latest
```

Verify installation:
```bash
sentri --version
```

#### **Option B: Local Project Install**
Install as a project dependency:

```bash
npm install --save-dev @dextonicx/cli@latest
```

Use with npm script:
```bash
npx sentri check ./contracts --chain evm
```

#### **Option C: Cargo Install (If Rust is Available)**

```bash
cargo install sentri-cli
```

Then configure npm package to use it:
```bash
export SENTRI_BINARY_PATH=$(which sentri)
npx @dextonicx/cli check ./contracts --chain evm
```

### Step 2: Verify Binary Download

On first run, Sentri downloads the binary (one-time, requires network):

```bash
sentri --version  # Should show: sentri 0.1.3
sentri doctor     # Should show: ✓ All components healthy
```

If you see errors:
- Check internet connection (download requires GitHub access)
- Run with verbose: `sentri --version --verbose`
- See [Troubleshooting](#troubleshooting) below

---

## Quick Start

### 1. Run on EVM Contracts

```bash
sentri check ./contracts --chain evm
```

### 2. Check Solana Programs

```bash
sentri check ./programs --chain solana
```

### 3. Analyze Move Modules

```bash
sentri check ./sources --chain move
```

### 4. Get JSON Output

```bash
sentri check ./contracts --chain evm --format json --output report.json
```

### 5. Fail on High Severity Issues

```bash
sentri check ./contracts --chain evm --fail-on high
```

---

## Implementation Guide

### For Solidity/Hardhat Projects

**Step 1: Install**
```bash
npm install --save-dev @dextonicx/cli@latest
```

**Step 2: Add NPM script** (`package.json`)
```json
{
  "scripts": {
    "analyze": "sentri check ./contracts --chain evm",
    "analyze:strict": "sentri check ./contracts --chain evm --fail-on high"
  }
}
```

**Step 3: Run**
```bash
npm run analyze
```

**Step 4: Integrate into Hardhat** (`hardhat.config.js`)
```javascript
const { analyze } = require("@dextonicx/cli");

task("sentri", "Run invariant checks")
  .addParam("chain", "Blockchain", "evm")
  .setAction(async ({ chain }) => {
    const report = await analyze({
      path: "./contracts",
      chain,
    });
    
    console.log(`✓ Found ${report.summary.violations} violations`);
    if (report.summary.critical > 0) {
      throw new Error("Critical vulnerabilities found!");
    }
  });
```

Run: `npx hardhat sentri`

### For Anchor/Solana Projects

**Step 1: Install**
```bash
npm install --save-dev @dextonicx/cli@latest
```

**Step 2: Add NPM script** (`package.json`)
```json
{
  "scripts": {
    "analyze": "sentri check ./programs --chain solana"
  }
}
```

**Step 3: Run**
```bash
npm run analyze
```

### For Move (Aptos/Sui) Projects

**Step 1: Install globally** (Move CLI needs external tool)
```bash
npm install -g @dextonicx/cli@latest
```

**Step 2: Run from project root**
```bash
sentri check ./sources --chain move
```

### Node.js/JavaScript Programmatic Usage

**Create `analyze.js`:**
```javascript
const { analyze, doctor } = require("@dextonicx/cli");

async function checkSecurity() {
  // Check system health first
  const health = await doctor();
  console.log(`System status: ${health.status}`);

  // Run analysis
  const report = await analyze({
    path: "./contracts",
    chain: "evm",
    failOn: "high",
  });

  console.log(`Found ${report.summary.violations} violations`);
  
  // View violations
  report.violations.forEach(v => {
    console.log(`[${v.severity}] ${v.title}`);
    console.log(`  at ${v.location}`);
    console.log(`  ${v.message}`);
  });

  // Fail if critical
  if (report.summary.critical > 0) {
    process.exit(1);
  }
}

checkSecurity().catch(err => {
  console.error("Analysis failed:", err);
  process.exit(1);
});
```

**Run:** `node analyze.js`

---

## CI Integration

### GitHub Actions

```yaml
name: Invariant Checks

on: [push, pull_request]

jobs:
  sentri:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - uses: actions/setup-node@v4
        with:
          node-version: "20"

      - name: Install Sentri
        run: npm install -g @dextonicx/cli

      - name: Run invariant checks
        run: sentri check ./contracts --chain evm --fail-on high

      - name: Generate JSON report
        if: always()
        run: sentri check ./contracts --chain evm --format json --output sentri-report.json

      - name: Upload report
        if: always()
        uses: actions/upload-artifact@v3
        with:
          name: sentri-report
          path: sentri-report.json
```

### GitLab CI

```yaml
sentri:
  image: node:20
  script:
    - npm install -g @dextonicx/cli
    - sentri check ./contracts --chain evm --fail-on high
  artifacts:
    reports:
      codequality: sentri-report.json
```

### Local Testing

```bash
npm install @dextonicx/cli
npx sentri check ./contracts --chain evm
```

## Supported Platforms

| Platform | Architecture | Status |
|----------|--------------|--------|
| Linux    | x86_64       | ✅ Supported |
| Linux    | ARM64        | ✅ Supported |
| macOS    | x86_64       | ✅ Supported |
| macOS    | ARM64 (M1/M2)| ✅ Supported |
| Windows  | x86_64       | ✅ Supported |

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `SENTRI_SKIP_DOWNLOAD` | (unset) | Set to `1` to skip binary download in postinstall |
| `SENTRI_BINARY_PATH` | (auto-detect) | Override path to Sentri binary |
| `HTTPS_PROXY` | (unset) | HTTP proxy for binary download |
| `HTTP_PROXY` | (unset) | HTTP proxy (fallback) |

Example — use an existing Cargo install instead of downloading:

```bash
export SENTRI_BINARY_PATH=/usr/local/bin/sentri
npx @dextonicx/cli check ./contracts --chain evm
```

## Invariants

Sentri checks 22 built-in security invariants across three blockchains.

### EVM (10 invariants)

- **EVM_001**: Reentrancy checks
- **EVM_002**: Integer overflow protection
- **EVM_003**: Integer underflow protection
- **EVM_004**: Unchecked return values
- **EVM_005**: Delegatecall injection
- **EVM_006**: Access control violations
- **EVM_007**: Timestamp dependence
- **EVM_008**: Front-running vulnerabilities
- **EVM_009**: Uninitialized pointers
- **EVM_010**: Division by zero

### Solana (7 invariants)

- **SOL_001**: Missing signer checks
- **SOL_002**: Account validation failures
- **SOL_003**: Integer overflow
- **SOL_004**: Rent exemption violations
- **SOL_005**: PDA derivation errors
- **SOL_006**: Lamport balance issues
- **SOL_007**: Instruction parsing failures

### Move (5 invariants)

- **MOVE_001**: Access control issues
- **MOVE_002**: Integer overflow
- **MOVE_003**: Resource leaks
- **MOVE_004**: Type mismatches
- **MOVE_005**: Missing signer requirements

See the [full invariants reference](https://github.com/geekstrancend/Sentri#invariants) for detailed descriptions.

## Configuration

Create a `.sentri.toml` file to configure analysis:

```toml
# .sentri.toml
[checks]
enabled = [
  "EVM_001",  # Reentrancy
  "EVM_002",  # Integer overflow
  "EVM_008",  # Front-running
]

[report]
format = "json"
output = "sentri-report.json"
fail_on = "medium"

[ignore]
files = ["node_modules/**", "build/**"]
violations = [
  { id = "EVM_001", location = "contracts/LegacyContract.sol" },
]
```

Then run:

```bash
sentri check ./contracts --chain evm --config .sentri.toml
```

## Build Your Own Plugin

The programmatic API allows building custom tools:

```javascript
const { analyze } = require("@dextonicx/cli");

async function customAnalyzer(contractPath) {
  const report = await analyze({
    path: contractPath,
    chain: "evm",
  });

  // Do custom processing
  const criticalViolations = report.violations.filter(
    (v) => v.severity === "Critical"
  );

  return {
    passed: report.summary.passed === report.summary.total_checks,
    critical: criticalViolations.length,
    violations: report.violations,
  };
}

module.exports = { customAnalyzer };
```

## Troubleshooting

### npm install hangs or times out

**Issue**: `npm install @dextonicx/cli` hangs during the postinstall script.

**Solution**: Use **v0.1.8 or later**:

```bash
npm install @dextonicx/cli@latest
```

Versions before v0.1.8 had a critical hang issue in the binary path resolution. v0.1.8+ includes:
- ✅ Fixed infinite recursion in binary detection
- ✅ Download timeout handling (30s socket, 60s total)
- ✅ Works with proven v0.1.3 binary from GitHub

If you're still experiencing hangs:

```bash
# Option 1: Use cargo binary + env var
cargo install sentri-cli
export SENTRI_BINARY_PATH=$(which sentri)
npm install @dextonicx/cli@latest

# Option 2: Skip download and provide binary manually
npm install @dextonicx/cli@latest --no-optional
mkdir -p node_modules/@dextonicx/cli/.sentri-bin
cp /path/to/sentri node_modules/@dextonicx/cli/.sentri-bin/sentri
chmod +x node_modules/@dextonicx/cli/.sentri-bin/sentri
```

### sentri command hangs when I run it

**Issue**: `sentri check` or `sentri doctor` hangs indefinitely.

**Solution**: This was a critical bug fixed in v0.1.8. **Update to the latest version**:

```bash
npm install -g @dextonicx/cli@latest
```

The hanging was caused by infinite recursion in binary path resolution. v0.1.8+ completely fixes this.

### Binary not found after install

The postinstall script may have been skipped (e.g., `npm install --ignore-scripts`).

**Solution**: Reinstall with postinstall enabled:

```bash
npm install @dextonicx/cli@latest
```

Or provide your own binary:

```bash
export SENTRI_BINARY_PATH=/path/to/sentri
npx @dextonicx/cli check ./contracts --chain evm
```

### Permission denied on Linux/macOS

The extracted binary may have lost executable permission.

**Solution**: Reinstall:

```bash
npm uninstall @dextonicx/cli
npm install @dextonicx/cli@latest
```

### Unsupported platform error

Your OS/architecture combination is not yet supported for automatic download.

**Solution**: Install from source using Rust:

```bash
cargo install sentri-cli
export SENTRI_BINARY_PATH=$(which sentri)
npx @dextonicx/cli check ./contracts --chain evm
```

## Performance

Sentri uses static analysis — it runs without executing code:

- **EVM**: ~1-5 seconds for typical contracts
- **Solana**: ~2-10 seconds for anchor programs
- **Move**: ~2-8 seconds for modules

Times vary with code size and system speed.

## Documentation

- **GitHub**: https://github.com/geekstrancend/Sentri
- **Crates.io**: https://crates.io/crates/sentri-cli
- **API Docs**: https://docs.rs/sentri-cli

## License

MIT — See [LICENSE](LICENSE)

## Support

- **Issues**: https://github.com/geekstrancend/Sentri/issues
- **Discussions**: https://github.com/geekstrancend/Sentri/discussions
- **Security**: https://github.com/geekstrancend/Sentri/security/policy

---

Built with ❤️ by Sentri Contributors
