# Deployment and Operations Guide

## Overview

This guide covers deploying, configuring, and operating Invar in production environments.

**Quick Start:**

```bash
# Install latest release
curl -fsSL https://install.invar.dev | bash

# Initialize project
invar init --project my-project

# Run analysis
invar analyze --config my-project/invar.toml

# Monitor in CI/CD
invar check --strict --output json
```

## Installation

### Pre-Built Binaries

**Latest Release:**
```bash
# Linux
curl -fsSL https://releases.github.com/zelius/invar/latest/linux-x86_64.tar.gz | tar xz
sudo mv invar /usr/local/bin/

# macOS
curl -fsSL https://releases.github.com/zelius/invar/latest/macos-x86_64.tar.gz | tar xz
sudo mv invar /usr/local/bin/

# Windows
curl -fsSL https://releases.github.com/zelius/invar/latest/windows-x86_64.zip -o invar.zip
unzip invar.zip
# Add to PATH
```

**Verify Installation:**
```bash
invar --version
invar --help
```

### Homebrew (macOS/Linux)

```bash
brew tap zelius/invar
brew install invar
```

Update:
```bash
brew upgrade invar
```

### Build from Source

```bash
git clone https://github.com/zelius/invar
cd invar

# Build release binary
cargo build --release

# Binary at target/release/invar
```

### Docker

```bash
# Pull image
docker pull zelius/invar:latest

# Run analysis
docker run -v /path/to/project:/project zelius/invar:latest \
  analyze --config /project/invar.toml

# Tag and push to registry
docker tag zelius/invar:latest myregistry/invar:v0.1.0
docker push myregistry/invar:v0.1.0
```

Dockerfile:
```dockerfile
FROM rust:latest as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/invar /usr/local/bin/
ENTRYPOINT ["invar"]
```

## Configuration

### Project initialization

```bash
invar init --project myproject

# Creates:
# myproject/invar.toml
# myproject/invariants.invar
# myproject/.invarignore
```

### Configuration File (invar.toml)

```toml
[project]
name = "my-dapp"
version = "0.1.0"

[chains]
enabled = ["solana", "evm", "move"]

[solana]
programs = ["src/bin/**/*.rs"]

[evm]
contracts = ["contracts/**/*.sol"]

[move]
packages = ["packages/*/Move.toml"]

[reporting]
format = "json"
output = "reports/"
include_warnings = true

[security]
# Fail on violations
strict = false
# Fail on warnings
fail_on_warnings = false
```

### Environment Variables

```bash
# Logging level
export RUST_LOG=debug

# Color output
export RUST_LOG_STYLE=always

# Temporary directory
export TMPDIR=/tmp/invar

# Security: Disable risky features (none available)
export INVAR_STRICT=1
```

### Ignoring Files

`.invarignore`:
```
# Skip test files
**/test/**
**/tests/**

# Skip examples
examples/

# Platform-specific
*.tmp
*.bak
```

Pattern syntax (gitignore-compatible):
- `*` - Match anything in current directory
- `**` - Match anything, recursively
- `!` - Negation (un-ignore)

## Running Analysis

### Basic Analysis

```bash
# Use config file
invar analyze --config invar.toml

# Specify directory
invar analyze --path /path/to/project

# Multiple paths
invar analyze --path ./src --path ./contracts
```

### Output Formats

```bash
# JSON (for parsing)
invar analyze --output json > report.json

# Markdown (for reading)
invar analyze --output markdown > report.md

# Text (for console)
invar analyze --output text

# Pretty color output
invar analyze --pretty
```

### Filtering

```bash
# Analyze specific chain
invar analyze --chain solana

# Analyze specific invariants
invar analyze --include vault_conservation
invar analyze --exclude experimental_*

# Severity threshold
invar analyze --min-severity warning
```

### Exit Codes

Invar uses exit codes for CI/CD integration:

| Code | Meaning | Action |
|------|---------|--------|
| 0 | Success | Continue |
| 1 | Violation found | Fail build |
| 2 | Config error | Fix config, retry |
| 3 | Internal error | File bug report |

**CI/CD Pattern:**
```bash
invar analyze --config invar.toml
case $? in
  0) echo "All invariants satisfied" ;;
  1) echo "Violation detected - halting deploy" && exit 1 ;;
  2) echo "Configuration error" && exit 1 ;;
  3) echo "Internal error - escalate" && exit 1 ;;
esac
```

## CI/CD Integration

### GitHub Actions

```yaml
name: Invariant Check
on: [push, pull_request]

jobs:
  invar:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Install Invar
        run: |
          curl -fsSL https://install.invar.dev | bash
          echo "$HOME/.invar/bin" >> $GITHUB_PATH
      
      - name: Run Analysis
        run: invar analyze --config invar.toml --output json
      
      - name: Upload Report
        if: always()
        uses: actions/upload-artifact@v3
        with:
          name: invar-report
          path: invar-report.json
```

### GitLab CI

```yaml
check_invariants:
  image: zelius/invar:latest
  script:
    - invar analyze --config invar.toml --output json --output-file report.json
  artifacts:
    reports:
      dotenv: report.json
  coverage: '/Coverage: (\d+\.\d+)%/'
```

### Jenkins

```groovy
stage('Check Invariants') {
    steps {
        sh '''
            curl -fsSL https://install.invar.dev | bash
            ~/.invar/bin/invar analyze --config invar.toml
        '''
    }
    post {
        always {
            archiveArtifacts artifacts: 'invar-report.*'
        }
        failure {
            currentBuild.result = 'FAILURE'
        }
    }
}
```

### Pre-Commit Hook

`.git/hooks/pre-commit`:
```bash
#!/bin/bash
set -e

echo "Checking invariants..."

if ! invar analyze --config invar.toml --strict; then
    echo "Invariant violations detected"
    exit 1
fi

echo "Invariants satisfied"
```

Install:
```bash
chmod +x .git/hooks/pre-commit
```

## Monitoring & Observability

### Logging

```bash
# Debug level logging
RUST_LOG=debug invar analyze --config invar.toml

# Specific module
RUST_LOG=invar_core=debug invar analyze

# Tracing with spans
RUST_LOG=invar=trace invar analyze
```

### Metrics

Invar produces JSON output for monitoring:

```json
{
  "status": "success",
  "summary": {
    "total_invariants": 42,
    "passed": 40,
    "failed": 2,
    "skipped": 0
  },
  "violations": [
    {
      "invariant": "vault_conservation",
      "severity": "critical",
      "chain": "solana"
    }
  ]
}
```

Parse for monitoring:
```bash
# Extract pass rate
invar analyze --output json | jq '.summary | (.passed / .total_invariants) * 100'

# Alert on violations
if invar analyze --output json | jq '.summary.failed > 0'; then
  send_alert "Invariant violations detected"
fi
```

### Health Checks

```bash
#!/bin/bash
# health_check.sh

# Check installation
invar --version || exit 1

# Check config
invar analyze --config invar.toml --dry-run || exit 1

# Quick smoke test
invar analyze --chain solana --timeout 30s || exit 1

echo "Invar is healthy"
```

Run periodically:
```bash
# In cron
*/5 * * * * /path/to/health_check.sh
```

## Troubleshooting

### Configuration Errors

```bash
# Validate config
invar validate-config --config invar.toml

# Common issues:
# - Missing [project] section
# - Invalid chain name
# - Missing program/contract paths
```

### Performance Issues

```bash
# Profile analysis
time invar analyze --config invar.toml

# Baseline metrics
invar analyze --config invar.toml --benchmark
```

**Optimization:**
- Skip non-critical invariants: `--exclude experimental_*`
- Use specific chains: `--chain solana` (not all)
- Cache results: `--cache /tmp/invar`

### Memory Usage

```bash
# Monitor memory
/usr/bin/time -v invar analyze --config invar.toml

# Reduce memory for large projects
invar analyze --streaming --max-buffer 256M
```

### Debugging

```bash
# Verbose output
invar analyze --config invar.toml -vv

# Generate debug info
invar analyze --config invar.toml --debug-output debug.log

# Backtrace on error
RUST_BACKTRACE=1 invar analyze --config invar.toml
```

## Upgrading

### Check Current Version

```bash
invar --version
# invar 0.1.0
```

### Update Process

```bash
# Check for updates
invar update check

# Install update
invar update --yes

# Verify
invar --version
```

### Breaking Changes

Invar follows [semantic versioning](./versioning.md).

**Migration Guide:**
```bash
# v0.1.0 → v0.2.0
# Review MIGRATION.md before upgrading

# Backup current config
cp invar.toml invar.toml.v0.1.0

# Upgrade
cargo install invar@0.2.0

# Test with dry-run
invar analyze --config invar.toml --dry-run
```

## Security Best Practices

### Access Control

```bash
# Restrict binary permissions
chmod 755 /usr/local/bin/invar

# Only allow specific users
chmod 700 /path/to/invar.toml
chown analyzer:analyzer /path/to/invar.toml
```

### Secret Management

Never commit secrets to config:

```toml
# Bad
[solana]
rpc_url = "http://localhost:8899"
keypair = "secret_key.json"  # Never commit!

# Good
[solana]
rpc_url = "${SOLANA_RPC_URL}"  # From environment
keypair = "${SOLANA_KEYPAIR}"  # From environment
```

Set in CI/CD:
```bash
export SOLANA_RPC_URL="http://localhost:8899"
export SOLANA_KEYPAIR="/secure/path/to/keypair.json"
invar analyze --config invar.toml
```

### Audit Trail

```bash
# Log all runs
invar analyze --config invar.toml --audit-log /var/log/invar.log

# Parse logs
grep "violation" /var/log/invar.log | jq
```

## Production Checklist

Before deploying to production:

- [ ] Test locally with production config
- [ ] Run through CI/CD pipeline
- [ ] Review all invariants are correct
- [ ] Verify reporting/alerts work
- [ ] Check performance on production data
- [ ] Have rollback plan if needed
- [ ] Document any custom configurations
- [ ] Set up monitoring and alerting
- [ ] Train team on tool usage
- [ ] Create runbook for common issues

## Support

- **Issues**: GitHub Issues with `[deployment]` tag
- **Questions**: GitHub Discussions
- **Security**: security@invar-project.dev
- **Community**: Discord (link in README)

## Summary

**Deployment is straightforward:**
1. Install binary
2. Initialize project config
3. Integrate with CI/CD
4. Monitor results
5. Update regularly

**Success requires:**
- Clear configuration
- Integration with existing CI/CD
- Monitoring for violations
- Regular updates
