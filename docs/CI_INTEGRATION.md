# Sentri CI/CD Integration Guide

Integrate Sentri into your continuous integration pipeline to catch invariant violations before they reach production.

## Quick Start

### GitHub Actions

Add this workflow to `.github/workflows/sentri.yml`:

```yaml
name: Sentri Invariant Check

on:
  push:
    branches: [main, develop]
    paths:
      - 'contracts/**'
      - 'programs/**'
  pull_request:
    branches: [main, develop]
    paths:
      - 'contracts/**'
      - 'programs/**'

jobs:
  sentri-check:
    runs-on: ubuntu-latest
    
    steps:
      - uses: actions/checkout@v3
      
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
      
      - name: Install Sentri
        run: cargo install --git https://github.com/geekstrancend/Sentri --bin sentri
      
      - name: Run Sentri Checks
        run: sentri check ./contracts --chain evm --fail-on high --format json --output sentri-report.json
      
      - name: Upload Report
        if: always()
        uses: actions/upload-artifact@v3
        with:
          name: sentri-report
          path: sentri-report.json
      
      - name: Comment PR
        if: failure() && github.event_name == 'pull_request'
        uses: actions/github-script@v6
        with:
          script: |
            github.rest.issues.createComment({
              issue_number: context.issue.number,
              owner: context.repo.owner,
              repo: context.repo.repo,
              body: '❌ Sentri invariant checks failed. See workflow run for details.'
            })
```

### GitLab CI

Add to `.gitlab-ci.yml`:

```yaml
sentri:
  image: rust:latest
  stage: test
  script:
    - cargo install --git https://github.com/geekstrancend/Sentri --bin sentri
    - sentri check ./contracts --chain evm --fail-on high --quiet
  artifacts:
    reports:
      junit: sentri-report.xml
    paths:
      - sentri-report.json
    expire_in: 30 days
  on:
    - main
    - develop
    - merge_requests
```

### CircleCI

Add to `.circleci/config.yml`:

```yaml
jobs:
  sentri-check:
    docker:
      - image: rust:latest
    steps:
      - checkout
      - run:
          name: Install Sentri
          command: cargo install --git https://github.com/geekstrancend/Sentri --bin sentri
      - run:
          name: Run Sentri Checks
          command: sentri check ./contracts --chain evm --fail-on high
      - store_artifacts:
          path: sentri-report.json

workflows:
  version: 2
  test:
    jobs:
      - sentri-check:
          filters:
            branches:
              only:
                - main
                - develop
```

## Configuration

### Basic Configuration (`.sentri.toml`)

```toml
[project]
name = "my-contracts"
version = "1.0.0"

[chains]
enabled = ["evm"]

[invariants]
# Categories: balance_arithmetic, access_control, state_consistency, etc.
categories = ["balance_arithmetic", "access_control"]

# Or explicitly list:
enabled = [
    "balance_conservation",
    "no_reentrancy",
    "owner_only_function"
]

fail_on = "high"  # Fail on CRITICAL/HIGH
```

### Chain-Specific Configuration

```toml
[chains.evm]
enabled = true
invariants = [
    "balance_conservation",
    "no_reentrancy",
    "safe_delegatecall",
    "safe_selfdestruct"
]
target_directory = "contracts/"

[chains.solana]
enabled = false  # Not using Solana

[chains.move]
enabled = false  # Not using Move
```

## CLI Options for CI

### Output Formats

**Text (default)** - Colored output for humans:
```bash
sentri check ./contracts --format text
```

**JSON** - Machine-readable format:
```bash
sentri check ./contracts --format json --output results.json
```

**HTML** - Styled report for web viewing:
```bash
sentri check ./contracts --format html --output report.html
```

### Retry Behavior

**Fail on any violation:**
```bash
sentri check ./contracts --fail-on low
```

**Fail only on critical/high:**
```bash
sentri check ./contracts --fail-on high
```

**Never fail (just report):**
```bash
sentri check ./contracts --fail-on none
```

### Quiet Mode

Suppress all output except errors:
```bash
sentri check ./contracts --quiet
```

Useful for CI to reduce log spam.

### Verbose Mode

Show all checks including passed ones:
```bash
sentri check ./contracts --verbose
```

## Pre-commit Hook

Ensure developers run checks locally before committing.

Install `.git/hooks/pre-commit`:

```bash
#!/bin/bash

set -e

# Check if sentri is installed
if ! command -v sentri &> /dev/null; then
    echo "sentri not found. Install with: cargo install --bin sentri"
    exit 1
fi

# Run sentri on staged contracts
STAGED_FILES=$(git diff --cached --name-only --diff-filter=ACM | grep -E '\.(sol|rs)$' || true)

if [ -n "$STAGED_FILES" ]; then
    echo "Running Sentri checks on staged files..."
    
    for file in $STAGED_FILES; do
        if [ -f "$file" ]; then
            sentri check "$file" --quiet
            if [ $? -ne 0 ]; then
                echo "❌ Sentri check failed for $file"
                echo "Fix violations and try again."
                exit 1
            fi
        fi
    done
    
    echo "✓ All Sentri checks passed"
fi
```

Make executable:
```bash
chmod +x .git/hooks/pre-commit
```

## Suppression Patterns

### Inline Suppression

Suppress a specific check for a function:

```solidity
// @sentri-suppress: no_reentrancy
function withdraw() public {
    // ... implementation ...
}
```

Suppress multiple checks:

```solidity
// @sentri-suppress: no_reentrancy, safe_delegatecall
function complexOperation() public {
    // ...
}
```

### Config-level Suppression

In `.sentri.toml`:

```toml
[[suppression]]
file = "contracts/Legacy.sol"
invariant = "no_reentrancy"
reason = "Legacy code - reentrancy not applicable"
until = "2024-12-31"  # Optional expiration

[[suppression]]
file = "contracts/Risky.sol"
invariants = ["no_reentrancy", "safe_delegatecall"]
reason = "Intentional design pattern"
approved_by = "security-team"
```

## GitHub Actions Examples

### Matrix Testing Multiple Chains

```yaml
strategy:
  matrix:
    chain: [evm, solana, move]

steps:
  - run: sentri check ./contracts --chain ${{ matrix.chain }}
```

### Report Posting

```yaml
- name: Run Sentri
  id: sentri
  run: sentri check ./contracts --format json --output report.json
  continue-on-error: true

- name: Post Report Comment
  if: github.event_name == 'pull_request'
  uses: actions/github-script@v6
  with:
    script: |
      const fs = require('fs');
      const report = JSON.parse(fs.readFileSync('report.json', 'utf8'));
      
      const summary = `
      ## Sentri Analysis
      - **Total violations:** ${report.summary.violations}
      - **Critical:** ${report.summary.critical}
      - **High:** ${report.summary.high}
      - **Status:** ${report.summary.violations === 0 ? '✅ PASS' : '❌ FAIL'}
      `;
      
      github.rest.issues.createComment({
        issue_number: context.issue.number,
        owner: context.repo.owner,
        repo: context.repo.repo,
        body: summary
      });
```

### Comparison with Main Branch

```yaml
- name: Checkout main
  run: git fetch origin main:main

- name: Check current branch
  run: sentri check ./contracts --format json --output current.json

- name: Check main branch
  run: |
    git stash
    git checkout main
    sentri check ./contracts --format json --output main.json
    git checkout -

- name: Compare results
  run: python3 scripts/compare-reports.py main.json current.json
```

## Docker Integration

### Dockerfile

```dockerfile
FROM rust:latest

RUN cargo install --git https://github.com/geekstrancend/Sentri --bin sentri

WORKDIR /workspace

ENTRYPOINT ["sentri"]
CMD ["doctor"]
```

Usage:

```bash
docker build -t sentri-check .

docker run -v $(pwd):/workspace sentri-check check ./contracts
```

### Docker Compose

```yaml
version: '3'
services:
  sentri:
    image: sentri-check:latest
    volumes:
      - ./contracts:/workspace/contracts
      - ./sentri.toml:/workspace/.sentri.toml
    command: check ./contracts --format html --output report.html
```

## Performance Optimization

### Caching

GitHub Actions caching:

```yaml
- name: Cache cargo registry
  uses: actions/cache@v3
  with:
    path: ~/.cargo/registry
    key: ${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}

- name: Cache Sentri installation
  uses: actions/cache@v3
  with:
    path: ~/.cargo/bin/sentri
    key: ${{ runner.os }}-sentri-bin
```

### Limiting Scope

Only check changed files:

```bash
# Get changed files from main
git diff --name-only main...HEAD -- '*.sol' | xargs sentri check
```

### Parallel Checks

Check multiple chains in parallel (GitHub Actions):

```yaml
jobs:
  sentri:
    strategy:
      matrix:
        chain: [evm, solana]
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - run: cargo install --bin sentri
      - run: sentri check ./contracts --chain ${{ matrix.chain }}
```

## Monitoring & Analytics

### Report Storage

Store reports over time:

```bash
# On successful run
mkdir -p reports/$(date +%Y-%m-%d)
sentri check ./contracts --format json --output reports/$(date +%Y-%m-%d)/report.json
```

### Trend Analysis

```python
import json
import glob

reports = sorted(glob.glob('reports/*/report.json'))

for report_file in reports:
    with open(report_file) as f:
        data = json.load(f)
        print(f"{report_file}: {data['summary']['violations']} violations")
```

### Grafana Integration

Export metrics for Grafana:

```bash
sentri check ./contracts --format json | jq '
.summary |
"sentri_violations " + (.violations | tostring) + "\n" +
"sentri_critical " + (.critical | tostring) + "\n" +
"sentri_high " + (.high | tostring)'
```

## Troubleshooting

### "sentri: command not found"

Ensure installation in CI environment:

```bash
# Check if installed
which sentri

# Install if missing
cargo install --git https://github.com/geekstrancend/Sentri --bin sentri

# Verify
sentri --version
```

### Timeout Issues

Increase timeouts for large contracts:

```yaml
timeout-minutes: 30  # GitHub Actions
```

### Memory Issues

Set memory limits:

```bash
# Limit to 2GB
sentri check ./contracts --max-memory 2GB
```

### Configuration Not Found

Ensure `.sentri.toml` is in working directory:

```bash
ls -la .sentri.toml
sentri init .  # Create if missing
```

## Best Practices

1. **Fail Fast** - Use `--fail-on high` to catch critical issues immediately
2. **Review Reports** - Always review the full HTML report
3. **Suppressions** - Document why checks are suppressed with `approved_by`
4. **Regular Updates** - Update Sentri regularly for new invariants
5. **Monitoring** - Track violations over time to detect trends
6. **Local Testing** - Run `sentri check` locally before pushing
7. **Pre-commit Hooks** - Prevent commits with violations
8. **Documentation** - Link to Sentri docs when violations are found

## Example Workflow

Complete GitHub Actions workflow:

```yaml
name: Smart Contract Analysis

on: [push, pull_request]

jobs:
  sentri:
    runs-on: ubuntu-latest
    
    steps:
      - uses: actions/checkout@v3
      
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
      
      - name: Cache Rust
        uses: Swatinem/rust-cache@v2
      
      - name: Install Sentri
        run: cargo install --git https://github.com/geekstrancend/Sentri --bin sentri
      
      - name: Run Sentri
        id: sentri
        run: |
          sentri check ./contracts \
            --chain evm \
            --fail-on high \
            --format json \
            --output sentri-report.json
        continue-on-error: true
      
      - name: Upload Report
        uses: actions/upload-artifact@v3
        if: always()
        with:
          name: sentri-report
          path: sentri-report.json
      
      - name: Check Results
        if: steps.sentri.outcome == 'failure'
        run: |
          echo "❌ Sentri checks failed"
          cat sentri-report.json | jq .
          exit 1
  
  test:
    runs-on: ubuntu-latest
    needs: sentri
    steps:
      - uses: actions/checkout@v3
      - run: cargo test --lib
```

## Support

For issues with CI integration:
- Check `.sentri.toml` syntax: `sentri doctor`
- View full output: Remove `--quiet` flag
- See debug info: Add `--verbose`
- Check logs in CI dashboard for error details
