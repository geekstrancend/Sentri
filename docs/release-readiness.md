# Release Readiness Checklist

This checklist ensures Invar is production-ready before each release.

---

## Pre-Release Verification

### Code Quality

- [ ] **Zero compiler warnings**
  ```bash
  cargo clippy --all --all-targets -- -D warnings
  ```

- [ ] **Code formatting compliant**
  ```bash
  cargo fmt --all -- --check
  ```

- [ ] **All tests pass**
  ```bash
  cargo test --all
  ```

- [ ] **No unsafe code** (except explicitly documented)
  ```bash
  grep -r "unsafe {" crates/core crates/dsl_parser
  ```

- [ ] **No unwrap() in production paths**
  ```bash
  grep -r "\.unwrap()" crates/ | grep -v "test" | grep -v "example"
  ```

### Test Coverage

- [ ] **Core module coverage > 90%**
  ```bash
  cargo tarpaulin --all --out Html
  # Review tarpaulin-report.html
  ```

- [ ] **All unit tests pass**
  ```bash
  cargo test --lib
  ```

- [ ] **Property tests stable** (run 3x)
  ```bash
  for i in {1..3}; do cargo test --test property || exit 1; done
  ```

- [ ] **Integration tests pass**
  ```bash
  cargo test --test integration
  ```

- [ ] **CLI tests pass**
  ```bash
  cargo test --test cli
  ```

- [ ] **Security tests pass**
  ```bash
  cargo test --test security
  ```

- [ ] **Benchmark compiles**
  ```bash
  cargo bench --no-run
  ```

### Security

- [ ] **Dependency audit clean**
  ```bash
  cargo audit
  ```

- [ ] **No vulnerable dependencies**
  ```bash
  cargo outdated
  ```

- [ ] **Security tests pass**
  ```bash
  cargo test --test security --verbose
  ```

- [ ] **No injection vulnerabilities**
  - Manual code review of DSL parser
  - Verify path canonicalization

- [ ] **No information disclosure**
  - Error messages don't leak internals
  - Output properly escaped for JSON

- [ ] **Cryptographic operations verified**
  - If any crypto used, review algorithm choice
  - Ensure reproducibility

### Determinism

- [ ] **Tests run deterministically**
  ```bash
  for i in {1..5}; do cargo test --lib || exit 1; done
  ```

- [ ] **Output is deterministic**
  - Same input always produces same output
  - No HashMap/HashSet (use BTreeMap/BTreeSet)
  - No randomized output

- [ ] **Benchmark results stable**
  ```bash
  CRITERION_MEASUREMENT_TIME=60 cargo bench
  ```

### Documentation

- [ ] **README is current**
  ```bash
  cargo build --bin invar
  invar --version  # Test examples from README
  ```

- [ ] **API documentation complete**
  ```bash
  cargo doc --all --no-deps --open
  # Review all public items have doc comments
  ```

- [ ] **Testing guide present**
  - [ ] [docs/testing/unit-testing.md](../testing/unit-testing.md)
  - [ ] [docs/testing/property-testing.md](../testing/property-testing.md)
  - [ ] [docs/testing/cli-testing.md](../testing/cli-testing.md)
  - [ ] [docs/testing/integration-testing.md](../testing/integration-testing.md)

- [ ] **Security documentation present**
  - [ ] [docs/security/security-validation.md](../security/security-validation.md)
  - [ ] Threat model documented
  - [ ] Security contact information

- [ ] **CI/CD documented**
  - [ ] [docs/ci/ci-pipeline.md](../ci/ci-pipeline.md)
  - [ ] All jobs explained
  - [ ] Troubleshooting guide

- [ ] **Getting started complete**
  - [ ] [docs/getting-started.md](../getting-started.md)
  - [ ] Installation works
  - [ ] Quick start runs successfully

- [ ] **Performance guide complete**
  - [ ] [docs/performance/benchmarking.md](../performance/benchmarking.md)
  - [ ] Benchmark targets documented
  - [ ] Scaling characteristics described

### Release Artifacts

- [ ] **Binaries build successfully**
  ```bash
  cargo build --release --bin invar
  ```

- [ ] **Binary works correctly**
  ```bash
  ./target/release/invar --version
  ./target/release/invar --help
  ./target/release/invar check --help
  ```

- [ ] **All platforms build**
  - [ ] Ubuntu
  - [ ] macOS
  - [ ] Windows

- [ ] **Binary size reasonable**
  ```bash
  ls -lh target/release/invar
  # Should be < 50MB (after stripping)
  ```

### Changelog

- [ ] **CHANGELOG.md updated**
  - [ ] All breaking changes documented
  - [ ] New features listed
  - [ ] Bug fixes catalogued
  - [ ] Migration guide if needed

- [ ] **Version bumped appropriately**
  - [ ] Follows semantic versioning
  - [ ] Matches changelog entries
  - [ ] All crate versions synchronized

### Reproducibility

- [ ] **Cargo.lock committed**
  ```bash
  git ls-files Cargo.lock
  ```

- [ ] **Build is reproducible**
  ```bash
  cargo build --release
  sha256sum target/release/invar > hash1.txt
  cargo clean
  cargo build --release
  sha256sum target/release/invar > hash2.txt
  diff hash1.txt hash2.txt
  ```

- [ ] **Toolchain pinned**
  - Review .github/workflows/ci.yml
  - Rust version explicit (not "latest")

---

## Pre-Release Fixes

If any item is unchecked, **do not proceed** until fixed.

### Fix Template

1. Create issue: "Release blocker: [specific item]"
2. Create fix branch: `fix/release-blocker-[issue-number]`
3. Implement fix with tests
4. Verify fix passes all checks
5. Merge to develop
6. Verify again from start of checklist

---

## Release Process

### 1. Create Release Branch

```bash
git checkout develop
git pull origin develop
git checkout -b release/v0.2.0
```

### 2. Update Version

Update version in:
```
Cargo.toml (workspace)
crates/*/Cargo.toml (dependencies)
docs/ (any hardcoded versions)
```

### 3. Update CHANGELOG.md

```markdown
## [0.2.0] - 2026-02-18

### Added
- Feature one
- Feature two

### Changed
- Breaking change one

### Fixed
- Bug fix one

### Security
- Security issue one
```

### 4. Create Release Commit

```bash
git add Cargo.toml Cargo.lock CHANGELOG.md
git commit -m "Release v0.2.0"
git tag v0.2.0
```

### 5. Create Pull Request

```bash
git push origin release/v0.2.0
# Open PR on GitHub
```

### 6. Final Verification

Run complete test suite:

```bash
cargo test --all
cargo check --all
cargo clippy --all --all-targets -- -D warnings
cargo fmt --all -- --check
cargo audit
cargo tarpaulin --all
```

### 7. Merge to Main

Once all checks pass:

```bash
# In GitHub UI or terminal
git checkout main
git pull origin main
git merge --no-ff release/v0.2.0
git push origin main
git push origin v0.2.0  # Push tag
```

### 8. Merge Back to Develop

```bash
git checkout develop
git merge --no-ff main
git push origin develop
```

### 9. Create GitHub Release

In GitHub UI:
1. Go to Releases
2. Click "Draft a new release"
3. Select tag `v0.2.0`
4. Copy from CHANGELOG.md
5. Upload binaries
6. Mark as latest

### 10. Publish Crates

```bash
# From workspace root
cargo publish -p invar-core
cargo publish -p invar-dsl-parser
# ... all public crates

# Publishing to crates.io requires verification
# Follow prompts
```

---

## Post-Release Verification

Within 24 hours of release:

- [ ] **Downloads working**
  ```bash
  # Download from release page
  # Verify binary works
  ```

- [ ] **Crate published**
  ```bash
  cargo install invar --version 0.2.0
  invar --version
  ```

- [ ] **Documentation updated**
  - Released docs at invar.sh match code
  - No 404s in documentation

- [ ] **No critical issues reported**
  - Monitor GitHub issues
  - Check security contact

- [ ] **Announce release**
  - Tweet/social media
  - Newsletter if applicable
  - Community forums

---

## Version Numbering

Invar follows **Semantic Versioning**.

```
MAJOR.MINOR.PATCH
  0    1      0

0 = Breaking changes only
1 = New features, backward compatible
0 = Bug fixes, no new features
```

### When to bump

- **MAJOR**: Breaking DSL changes, API changes, security fixes
- **MINOR**: New DSL features, new chain support, new commands
- **PATCH**: Bug fixes, performance improvements

### Stability Guarantees

| Version | Stability |
|---------|-----------|
| < 1.0.0 | Beta, breaking changes expected |
| 1.0.0 - 1.x.x | Stable DSL, breaking changes rare |
| 2.0.0+ | Major versions for breaking changes |

---

## Pre-Release Sign-Off

Before release, obtain sign-off from:

- [ ] **Code Owner**: Code quality, tests
- [ ] **Security**: No vulnerabilities, safe defaults
- [ ] **DevOps**: Build process, artifacts
- [ ] **Documentation**: Docs complete and correct

---

## Rollback Procedure

If critical issue found post-release:

```bash
# Create hotfix branch
git checkout -b hotfix/v0.2.1 v0.2.0

# Fix the issue
# ... tests pass ...

# Create release
git tag v0.2.1
git push origin v0.2.1

# Announce deprecation of v0.2.0
# Direct users to v0.2.1
```

---

## Monitoring

After release, monitor for 72 hours:

```bash
# Check issue tracker
curl https://api.github.com/repos/invar/invar/issues

# Check crate downloads
# https://crates.io/crates/invar

# Check crash reports
# (configured in error reporting system)
```

If critical issues found, execute hotfix.

---

## Sign-Off

| Role | Name | Date | Signature |
|------|------|------|-----------|
| Release Manager | _____ | _____ | _____ |
| Code Owner | _____ | _____ | _____ |
| Security Lead | _____ | _____ | _____ |

---

**Release Version:** ____________

**Release Date:** ____________

**Status:** ☐ Pre-Release ☐ Released ☐ Rolled Back

**Notes:**
_________________________________
_________________________________
