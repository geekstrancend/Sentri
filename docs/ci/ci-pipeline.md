# CI Pipeline Documentation

## Overview

The Invar CI pipeline enforces **production-grade quality** through **automated**, **deterministic** checks that run on every push and pull request.

## Pipeline Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Push / Pull Request                       │
└────────────────────────┬────────────────────────────────────┘
                         │
        ┌────────────────┼────────────────┐
        │                │                │
        ▼                ▼                ▼
    ┌────────┐      ┌────────┐      ┌─────────┐
    │ Format │      │ Clippy │      │ Audit   │
    │ Check  │      │ Lint   │      │         │
    └────────┘      └────────┘      └─────────┘
        │                │                │
        └────────────────┼────────────────┘
                         │
        ┌────────────────┼────────────────┐
        │                │                │
        ▼                ▼                ▼
    ┌─────────┐    ┌─────────┐    ┌──────────┐
    │ Unit    │    │Property │    │Security  │
    │ Tests   │    │Tests    │    │Tests     │
    └─────────┘    └─────────┘    └──────────┘
        │                │                │
        └────────────────┼────────────────┘
                         │
        ┌────────────────┼────────────────┐
        │                │                │
        ▼                ▼                ▼
    ┌─────────────┐ ┌──────────┐ ┌──────────┐
    │Integration  │ │CLI Tests │ │Benchmark │
    │Tests        │ │          │ │Smoke Test│
    └─────────────┘ └──────────┘ └──────────┘
        │                │                │
        └────────────────┼────────────────┘
                         │
                    ┌────▼────┐
                    │ Coverage │
                    │ Report   │
                    └────┬────┘
                         │
                    ┌────▼────┐
                    │Build     │
                    │Release   │
                    └──────────┘
```

## Job Definitions

### 1. Format Check (Strict)

Ensures consistent code style:

```yaml
format:
  name: Format Check (Strict)
  steps:
    - name: Check formatting
      run: cargo fmt --all -- --check
```

**What it checks:**
- Rust formatting compliance
- Consistent indentation
- Code style standards

**Fails if:** Any file is not properly formatted

### 2. Clippy Lint (Strict)

Enforces code quality:

```yaml
clippy:
  name: Clippy Lint (Strict)
  steps:
    - name: Run clippy strict
      run: cargo clippy --all --all-targets --all-features -- -D warnings -D clippy::all
```

**What it checks:**
- Compiler warnings
- Common mistakes
- Performance issues
- Style recommendations

**Fails if:** Any clippy warning encountered

### 3. Security Audit

Checks for vulnerable dependencies:

```yaml
audit:
  name: Security Audit
  steps:
    - uses: rustsec/audit-check-action@v1
```

**What it checks:**
- Known CVEs in dependencies
- Unmaintained crates
- Yanked versions

**Fails if:** Vulnerability found

### 4. Unit Tests (Cross-Platform)

Tests core functionality:

```yaml
test:
  name: Unit Tests
  strategy:
    matrix:
      os: [ubuntu-latest, macos-latest, windows-latest]
      rust: [stable, beta]
  steps:
    - run: cargo test --lib --all --verbose
```

**Coverage:**
- ✅ Ubuntu (Stable & Beta)
- ✅ macOS (Stable & Beta)
- ✅ Windows (Stable & Beta)

**Fails if:** Any test fails

### 5. Property Tests

Generative testing:

```yaml
property-tests:
  name: Property Tests
  steps:
    - run: cargo test --test property --verbose
```

**Tests:**
- Parser never panics
- Evaluation is deterministic
- Type checking is sound
- Collections are safe

### 6. Security Tests

Validates security constraints:

```yaml
security-tests:
  name: Security Tests
  steps:
    - run: cargo test --test security --verbose
```

**Validates:**
- No silent failures
- No injection vulnerabilities
- No overflow issues
- Type safety holds

### 7. Integration Tests

End-to-end workflow validation:

```yaml
integration-tests:
  name: Integration Tests
  steps:
    - run: cargo test --test integration --verbose
```

**Validates:**
- Multi-file projects
- Cross-chain workflows
- Real-world invariants
- File integrity

### 8. CLI Tests

Command-line behavior:

```yaml
cli-tests:
  name: CLI Behavior Tests
  steps:
    - run: cargo build --release --bin invar
    - run: cargo test --test cli --verbose
```

**Validates:**
- Help output
- Exit codes
- JSON/Markdown output
- Error handling

### 9. Code Coverage

Tracks test coverage:

```yaml
coverage:
  name: Code Coverage
  steps:
    - run: cargo tarpaulin --all --out Xml --timeout 300
```

**Requirement:** > 80% coverage

### 10. Benchmark Smoke Test

Ensures benchmarks compile:

```yaml
benchmark:
  name: Benchmark Smoke Test
  steps:
    - run: cargo bench --no-run
```

### 11. Determinism Check

Validates consistency:

```yaml
determinism:
  name: Determinism Check
  steps:
    - run: for i in {1..3}; do cargo test --lib --all || exit 1; done
```

**Ensures:** Tests produce identical results across runs

### 12. Build Release Binary

Creates platform-specific binaries:

```yaml
build-release:
  needs: [format, clippy, test, security-tests, audit]
  strategy:
    matrix:
      os: [ubuntu-latest, macos-latest, windows-latest]
  steps:
    - run: cargo build --release --bin invar --verbose
```

**Produces:**
- `invar-linux-x86_64`
- `invar-macos-x86_64`
- `invar-windows-x86_64.exe`

## Environment Configuration

```yaml
env:
  CARGO_TERM_COLOR: always          # Colored output
  RUST_BACKTRACE: full              # Full error traces
  RUSTFLAGS: -D warnings            # Treat warnings as errors
```

## Caching Strategy

```yaml
- uses: Swatinem/rust-cache@v2     # Cache build artifacts
```

Dramatically speeds up CI by reusing:
- Downloaded dependencies
- Compiled crates
- Build artifacts

## Platform Coverage

| Platform | Rust | Tests |
|----------|------|-------|
| Ubuntu | Stable, Beta | All |
| macOS | Stable, Beta | All |
| Windows | Stable | All |

## Dependency Management

All jobs use the same stable Rust toolchain via:

```yaml
- uses: dtolnay/rust-toolchain@stable
```

Ensures consistency across all environments.

## Failure Modes

CI **fails** on:

```rust
// 1. Any formatting issue
cargo fmt --check  // must pass

// 2. Any clippy warning
cargo clippy -- -D warnings  // must pass

// 3. Any security vulnerability
cargo audit  // must be clean

// 4. Any test failure
cargo test --all  // all must pass

// 5. Any security issue
// security tests must pass

// 6. Non-deterministic behavior
// must pass 3 consecutive runs

// 7. Coverage below threshold
// coverage > 80%
```

## Local Development

To run the same checks locally before pushing:

```bash
#!/bin/bash
set -e

echo "🔍 Format check..."
cargo fmt --all -- --check

echo "📋 Clippy lint..."
cargo clippy --all --all-targets -- -D warnings

echo "🔒 Security audit..."
cargo audit

echo "✅ Unit tests..."
cargo test --lib

echo "🔬 Property tests..."
cargo test --test property

echo "🛡️  Security tests..."
cargo test --test security

echo "🔄 Integration tests..."
cargo test --test integration

echo "💻 CLI tests..."
cargo test --test cli

echo "✨ All checks passed!"
```

## Performance Optimizations

### Parallel Testing
```yaml
# Tests run in parallel by default
cargo test --all
```

### Artifact Caching
```yaml
- uses: Swatinem/rust-cache@v2
```

### Matrix Strategy
```yaml
matrix:
  os: [ubuntu-latest, macos-latest, windows-latest]
  rust: [stable, beta]
```

Tests run in parallel across matrix dimensions.

## Debugging CI Failures

### Local Reproduction

```bash
# Exact same command as CI
cargo fmt --all -- --check
cargo clippy --all --all-targets -- -D warnings
cargo test --lib --all --verbose
```

### View Full Logs

In GitHub Actions: Click "View raw logs" to see complete output.

### Re-run Jobs

After fix:
- Push a new commit
- CI automatically re-runs
- Or click "Re-run all jobs" in GitHub UI

## Status Checks

The final job `all-checks-pass` depends on:

```yaml
needs: [
  format, clippy, test, property-tests, 
  security-tests, integration-tests, cli-tests, 
  coverage, benchmark, determinism, audit
]
```

All must pass for PR to be mergeable.

## Release Automation

When all checks pass, release job:

```yaml
build-release:
  needs: [all checks above]
```

Creator artifacts suitable for:
- GitHub releases
- Package managers
- Distribution

## Monitoring and Alerts

### Branch Protection

Enable in GitHub Settings:

- ✅ Require CI checks pass
- ✅ Require code reviews
- ✅ Dismiss stale PR approvals
- ✅ Require branches be up to date

### Status Badges

Add to README:

```markdown
![CI Status](https://github.com/invar/invar/workflows/CI%20-%20Production%20Grade/badge.svg)
```

## Cost Optimization

### GitHub Actions Costs

- Ubuntu: $0.008/minute
- macOS: $0.08/minute  
- Windows: $0.016/minute

**Optimization strategies:**
1. Use Ubuntu as primary testing platform
2. Run macOS/Windows only on main branch
3. Use caching to reduce build time
4. Parallelize independent jobs

## Maintenance

### Monthly Review

- [ ] Audit CI logs for flakes
- [ ] Update toolchain versions
- [ ] Check for new clippy rules
- [ ] Review coverage trends

### Toolchain Updates

```yaml
- uses: dtolnay/rust-toolchain@stable  # Auto-tracks latest
```

Or pin specific version:

```yaml
- uses: dtolnay/rust-toolchain@1.75.0
```
