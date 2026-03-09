# @dextonicx/cli

[![npm version](https://img.shields.io/npm/v/@dextonicx/cli.svg)](https://www.npmjs.com/package/@dextonicx/cli)
[![npm downloads](https://img.shields.io/npm/dm/@dextonicx/cli.svg)](https://www.npmjs.com/package/@dextonicx/cli)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

Multi-chain smart contract invariant checker for **EVM** (Solidity), **Solana** (Rust/Anchor), and **Move** (Aptos/Sui).

Run static analysis on your blockchain code before deployment. Sentri checks invariants against 22 built-in security patterns across three major blockchain ecosystems.

## Installation

### NPM (Recommended)

```bash
npm install -g @dextonicx/cli
```

Then use globally:

```bash
sentri check ./contracts --chain evm
```

Or use with npx without installing:

```bash
npx @dextonicx/cli check ./contracts --chain evm
```

### From Cargo (Alternative)

If you have Rust installed:

```bash
cargo install sentri-cli
```

## Quick Start

### 1. Run on EVM Contracts

```bash
sentri check ./contracts --chain evm
```

Output:
```
Analyzing Solidity contracts...
✓ Completed analysis

Summary
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Total checks:     10
  Violations:        2
    ⚠ High:        2

Violations
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  [High] EVM_008 Front-running vulnerability
    Location: contracts/Auction.sol:45
    Message: Function modifies state after external call
    Recommendation: Use checks-effects-interactions pattern
```

### 2. Analyze Solana Programs

```bash
sentri check ./programs --chain solana
```

### 3. Check Move Modules

```bash
sentri check ./sources --chain move
```

### 4. Get JSON Output for CI Integration

```bash
sentri check ./contracts --chain evm --format json --output report.json
```

### 5. Fail CI if Violations Found

```bash
sentri check ./contracts --chain evm --fail-on high
```

When `--fail-on` is set, Sentri exits with code 1 if violations at or above the threshold are found:

```bash
$ sentri check ./contracts --chain evm --fail-on high
exit code: 1  # ← Fails CI pipeline
```

## Usage

### CLI

```bash
sentri check <PATH> --chain <CHAIN> [OPTIONS]

Options:
  --chain <CHAIN>           evm, solana, or move
  --format <FORMAT>         text (default), json, html
  --output <FILE>           Write report to file
  --config <FILE>           Path to .sentri.toml configuration
  --fail-on <SEVERITY>      Fail if violations found: low, medium, high, critical
  -v, --verbose             Verbose output
  --version                 Show version
  --help                    Show this help
```

### Node.js API

Use Sentri programmatically in JavaScript/TypeScript:

```javascript
const { analyze } = require("@dextonicx/cli");

async function checkContracts() {
  const report = await analyze({
    path: "./contracts",
    chain: "evm",
    failOn: "high",
  });

  console.log(`Found ${report.summary.violations} violations`);

  if (report.summary.critical > 0) {
    console.error("❌ Critical vulnerabilities detected!");
    process.exit(1);
  }

  for (const violation of report.violations) {
    console.log(
      `[${violation.severity}] ${violation.title} at ${violation.location}`
    );
  }

  console.log(`✓ Analysis complete`);
}

checkContracts().catch(console.error);
```

### Hardhat Integration

Use Sentri in Hardhat tasks:

```javascript
// hardhat.config.js
const { analyze } = require("@dextonicx/cli");

task("sentri", "Run Sentri invariant checks")
  .addParam("chain", "Blockchain: evm, solana, move", "evm")
  .setAction(async ({ chain }) => {
    const report = await analyze({
      path: "./contracts",
      chain,
    });

    console.log(`Found ${report.summary.violations} violations`);
    if (report.summary.critical > 0) {
      throw new Error(`Critical vulnerabilities found!`);
    }
  });
```

Then run:

```bash
npx hardhat sentri --chain evm
```

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

### Binary not found after install

The postinstall script may have been skipped (e.g., `npm install --ignore-scripts`).

**Solution**: Reinstall:

```bash
npm install @dextonicx/cli
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
npm install @dextonicx/cli
```

### Unsupported platform error

Your OS/architecture combination is not yet supported.

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
