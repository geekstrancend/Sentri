# Migration Guide

## Upgrading Invar

This guide helps you upgrade Invar across major versions.

## v0.1.0 → v1.0.0 (Planned)

**Release Timeline:** Q3 2024

### Breaking Changes

#### 1. Type System Changes

**v0.1.0:**
```invar
invariant: test
global: x > 0
```

**v1.0.0:** Explicit type annotations required
```invar
invariant: test [solana]
global: 
    x: uint256 > 0
```

**Migration:**
```bash
# Run migration tool
invar migrate --version 1.0.0 --config invar.toml

# Manually add type annotations
# See: docs/writing-invariants.md#type-annotations
```

#### 2. Configuration File Format

**v0.1.0:**
```toml
[project]
name = "my-project"

[solana]
programs = ["src/**/*.rs"]
```

**v1.0.0:** Explicit chain sections
```toml
[project]
name = "my-project"
chains = ["solana", "evm"]

[chains.solana]
programs = ["src/**/*.rs"]

[chains.evm]
contracts = ["contracts/**/*.sol"]
```

**Migration:**
1. Backup `invar.toml`: `cp invar.toml invar.toml.backup`
2. Run migration: `invar migrate config --config invar.toml`
3. Verify: `invar validate-config --config invar.toml`
4. Test: `invar analyze --config invar.toml --dry-run`

#### 3. CLI Command Changes

**v0.1.0:**
```bash
invar analyze --config invar.toml
invar validate-dsl invariants.invar
```

**v1.0.0:**
```bash
invar check --config invar.toml          # Replaces analyze
invar lint --invariants invariants.invar  # Replaces validate-dsl
invar analyze --config invar.toml        # Detailed output (new)
```

**Migration Script:**
```bash
#!/bin/bash
# migrate-commands.sh

# Update scripts to use new commands
sed -i 's|invar validate-dsl|invar lint|g' .github/workflows/*.yml
sed -i 's|invar analyze|invar check|g' Makefile

# Update CI/CD
# GitHub Actions: see ci-integration.md
# Jenkins: see deployment.md
# GitLab: see deployment.md
```

### Deprecation Timeline

For transitional period (6 months):

**Phase 1: Deprecation** (v1.0.0 release)
- Old commands still work
- Warnings in output: "⚠️ Command 'analyze' is deprecated, use 'check' instead"
- Compatibility layer in place

**Phase 2: Removal** (v1.1.0)
- Old commands removed
- Clear error message: "Command 'analyze' not found. Did you mean 'check'?"

### Backward Compatibility

Not maintained across major versions. Plan 2-3 month migration window.

## v1.0.0 → v1.1.0 (Minor)

**Backward compatible** - No migration needed

New features:
- [ ] Zero-knowledge proof integration
- [ ] Multi-signature invariant approval
- [ ] Enhanced DSL syntax

Existing code works unchanged!

```bash
# Simply upgrade
cargo install invar --version 1.1.0

# No configuration changes needed
invar check --config invar.toml  # Works!
```

## v1.1.0 → v1.1.1 (Patch)

**Backward compatible** - No migration needed

Bug fixes only. No API changes.

```bash
# Update for latest bug fixes
cargo install invar --version 1.1.1
```

## Migration Strategies

### Strategy 1: Gradual Migration

For large projects:

1. **Week 1:** Read migration guide
   ```bash
   cat MIGRATION.md
   ```

2. **Week 2:** Backup and test
   ```bash
   cp invar.toml invar.toml.backup
   cp invariants.invar invariants.invar.backup
   ```

3. **Week 3:** Run migration tool
   ```bash
   invar migrate --version 1.0.0 --config invar.toml
   ```

4. **Week 4:** Verify and test
   ```bash
   invar analyze --config invar.toml --dry-run
   cargo test  # Run your tests
   ```

5. **Week 5:** Update CI/CD
   ```bash
   # Update GitHub Actions
   # See: docs/ci-integration.md
   ```

6. **Week 6:** Deploy
   ```bash
   git commit -m "Upgrade to Invar v1.0.0"
   git push origin main
   ```

### Strategy 2: Rapid Migration

For small projects:

```bash
#!/bin/bash
# quick-migrate.sh

set -e

echo "1. Backup files"
cp invar.toml invar.toml.backup
cp invariants.invar invariants.invar.backup

echo "2. Upgrade Invar"
brew upgrade invar  # or your installation method

echo "3. Run migration"
invar migrate --version 1.0.0 --config invar.toml

echo "4. Validate"
invar validate-config --config invar.toml
invar analyze --config invar.toml --dry-run

echo "5. Test"
cargo test
npm test  # your tests

echo "6. Done!"
echo "✅ Migration successful"
echo "⚠️  Review changes before deploying"
```

## Common Migration Issues

### Issue: "Unknown configuration field"

**Symptom:**
```
Error: Unknown field 'programs' in [solana]
```

**Cause:** Old config format

**Fix:**
```toml
# ❌ Old
[solana]
programs = ["src/**/*.rs"]

# ✅ New v1.0.0
[[chains]]
name = "solana"
programs = ["src/**/*.rs"]
```

### Issue: Type annotation errors

**Symptom:**
```
Error: Expected type annotation
Line: global: x > 0
       ^
```

**Cause:** v1.0.0 requires explicit types

**Fix:**
```invar
# ❌ Old - implicitly uint64
invariant: test
global: x > 0

# ✅ New - explicit type
invariant: test
global: x: uint256 > 0
```

Supported types:
- `int256`, `int128`, `int64`, `int32`, `int16`, `int8`
- `uint256`, `uint128`, `uint64`, `uint32`, `uint16`, `uint8`
- `address`, `string`, `bool`, `bytes`

### Issue: CLI command not found

**Symptom:**
```bash
$ invar analyze
Error: Unknown command 'analyze'
Did you mean 'check'?
```

**Cause:** Command renamed in v1.0.0

**Fix:** Use new command name
```bash
# ❌ Old command (v0.1.0)
invar analyze --config invar.toml

# ✅ New command (v1.0.0)
invar check --config invar.toml
# or for detailed output
invar analyze --config invar.toml  # Available again in v1.0.0
```

### Issue: Exit codes changed

**Symptom:**
```bash
$ invar check
# Returns exit code 4 (unexpected)
```

**Cause:** Exit code semantics updated in v1.0.0

**Fix:** Update scripts to new codes
```bash
# Exit codes v1.0.0+
# 0 = success
# 1 = violation
# 2 = config error
# 3 = internal error

if invar check --config invar.toml; then
    echo "✅ All invariants passed"
else
    exit $?  # Propagate exit code
fi
```

## Testing Migration

### Pre-Migration Checklist

- [ ] Read full migration guide
- [ ] Backup all files
- [ ] Review breaking changes in detail
- [ ] Check for custom scripts needing updates
- [ ] Plan testing approach
- [ ] Schedule communication to team

### Testing Steps

```bash
# 1. Test locally
invar migrate --version 1.0.0 --config invar.toml
invar validate-config --config invar.toml
invar analyze --config invar.toml --dry-run

# 2. Run full test suite
cargo test
npm test
python -m pytest

# 3. Test in staging
git checkout -b migration/v1-upgrade
git add .
git commit -m "Migrate to Invar v1.0.0"
git push origin migration/v1-upgrade
# Create PR, run CI/CD

# 4. Team review
# - Review changes
# - Verify tests pass
# - Approve PR
# - Merge to main

# 5. Deploy
git checkout main
git pull
# Deploy to production
```

### Rollback Plan

If migration fails:

```bash
# 1. Revert to backup
cp invar.toml.backup invar.toml
cp invariants.invar.backup invariants.invar

# 2. Downgrade Invar
brew uninstall invar && brew install invar@0.1.0

# 3. Verify
invar analyze --config invar.toml --dry-run

# 4. Investigate issue
# - Check MIGRATION.md
# - Post in GitHub Discussions
# - File issue if bug

# 5. Try again after fix
```

## Version-Specific Guides

### Upgrading from v0.1.0

- [Complete Upgrade Guide](./versioning.md#v0-to-v1-upgrade)
- [Configuration Migration](./deployment.md#configuration)
- [CLI Changes](./writing-invariants.md#version-compatibility)

### Already on v1.0.0+

No migration needed! New minor/patch versions are backward compatible.

## Future Versions

See [Versioning Policy](versioning.md) for:
- Release schedule
- Support timelines
- Deprecation process
- Long-term support

## Getting Help

Migration stuck?

1. **Check FAQ**: [docs/faq.md](faq.md#upgrading)
2. **Read guide again**: [Full version guide](versioning.md)
3. **Ask community**:
   - GitHub Discussions
   - Discord
   - GitHub Issues with `[migration]` tag
4. **Contact maintainers**: support@invar-project.dev

## Summary

| Version | Type | Backward Compat | Effort |
|---------|------|-----------------|--------|
| v0.1.0 → v1.0.0 | Major | ❌ No | High (1-3 weeks) |
| v1.0.0 → v1.1.0 | Minor | ✅ Yes | None |
| v1.1.0 → v1.1.1 | Patch | ✅ Yes | None |

**Key Principle:** Major versions may break things. Minor/patch versions never do.

---

**Last updated:** 2024-01-15  
**Current version:** v0.1.0  
**Target version:** v1.0.0 (Q3 2024)
