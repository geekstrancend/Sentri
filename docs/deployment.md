# Deployment and Operations Guide

## Overview

This guide covers deploying, configuring, and operating Sentri in production environments.

**Quick Start:**

```bash
# Install latest release
curl -fsSL https://install.invar.dev | bash

# Initialize project
sentri init --project my-project

# Run analysis
sentri analyze --config my-project/invar.toml

# Monitor in CI/CD
sentri check --strict --output json
```

## Installation

### Pre-Built Binaries

**Latest Release:**
```bash
# Linux
curl -fsSL https://releases.github.com/geekstrancend/sentri/latest/linux-x86_64.tar.gz | tar xz
sudo mv sentri /usr/local/bin/

# macOS
curl -fsSL https://releases.github.com/geekstrancend/sentri/latest/macos-x86_64.tar.gz | tar xz
sudo mv sentri /usr/local/bin/

# Windows
curl -fsSL https://releases.github.com/geekstrancend/sentri/latest/windows-x86_64.zip -o sentri.zip
unzip sentri.zip
# Add to PATH
```

**Verify Installation:**
```bash
sentri --version
sentri --help
```

### Homebrew (macOS/Linux)

```bash
brew tap geekstrancend/sentri
brew install sentri
```

Update:
```bash
brew upgrade sentri
```

### Build from Source

```bash
git clone https://github.com/geekstrancend/Sentri.git
cd Sentri

# Build release binary
cargo build --release

# Binary at target/release/sentri
```

### Docker

```bash
# Pull image
docker pull geekstrancend/sentri:latest

# Run analysis
docker run -v /path/to/project:/project geekstrancend/sentri:latest \
  analyze --config /project/sentri.toml

# Tag and push to registry
docker tag geekstrancend/sentri:latest myregistry/sentri:v0.1.0
docker push myregistry/sentri:v0.1.0
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
sentri init --project myproject

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
sentri analyze --config invar.toml

# Specify directory
sentri analyze --path /path/to/project

# Multiple paths
sentri analyze --path ./src --path ./contracts
```

### Output Formats

```bash
# JSON (for parsing)
sentri analyze --output json > report.json

# Markdown (for reading)
sentri analyze --output markdown > report.md

# Text (for console)
sentri analyze --output text

# Pretty color output
sentri analyze --pretty
```

### Filtering

```bash
# Analyze specific chain
sentri analyze --chain solana

# Analyze specific invariants
sentri analyze --include vault_conservation
sentri analyze --exclude experimental_*

# Severity threshold
sentri analyze --min-severity warning
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
sentri analyze --config invar.toml
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
        run: sentri analyze --config invar.toml --output json
      
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
    - sentri analyze --config invar.toml --output json --output-file report.json
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

if ! sentri analyze --config invar.toml --strict; then
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
RUST_LOG=debug sentri analyze --config invar.toml

# Specific module
RUST_LOG=invar_core=debug sentri analyze

# Tracing with spans
RUST_LOG=invar=trace sentri analyze
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
sentri analyze --output json | jq '.summary | (.passed / .total_invariants) * 100'

# Alert on violations
if sentri analyze --output json | jq '.summary.failed > 0'; then
  send_alert "Invariant violations detected"
fi
```

### Health Checks

```bash
#!/bin/bash
# health_check.sh

# Check installation
sentri --version || exit 1

# Check config
sentri analyze --config invar.toml --dry-run || exit 1

# Quick smoke test
sentri analyze --chain solana --timeout 30s || exit 1

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
sentri validate-config --config invar.toml

# Common issues:
# - Missing [project] section
# - Invalid chain name
# - Missing program/contract paths
```

### Performance Issues

```bash
# Profile analysis
time sentri analyze --config invar.toml

# Baseline metrics
sentri analyze --config invar.toml --benchmark
```

**Optimization:**
- Skip non-critical invariants: `--exclude experimental_*`
- Use specific chains: `--chain solana` (not all)
- Cache results: `--cache /tmp/invar`

### Memory Usage

```bash
# Monitor memory
/usr/bin/time -v sentri analyze --config invar.toml

# Reduce memory for large projects
sentri analyze --streaming --max-buffer 256M
```

### Debugging

```bash
# Verbose output
sentri analyze --config invar.toml -vv

# Generate debug info
sentri analyze --config invar.toml --debug-output debug.log

# Backtrace on error
RUST_BACKTRACE=1 sentri analyze --config invar.toml
```

## Upgrading

### Check Current Version

```bash
sentri --version
# sentri 0.1.0
```

### Update Process

```bash
# Check for updates
sentri update check

# Install update
sentri update --yes

# Verify
sentri --version
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
sentri analyze --config invar.toml --dry-run
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
sentri analyze --config invar.toml
```

### Audit Trail

```bash
# Log all runs
sentri analyze --config invar.toml --audit-log /var/log/invar.log

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
