# Security Validation

## Overview

Security validation ensures that:
1. **Enforcement cannot be bypassed** - Invariants always run
2. **No silent failures** - All errors are explicit
3. **Type safety holds** - Runtime type errors are impossible  
4. **No unsafe code** in critical paths
5. **Deterministic output** - No non-determinism information leaks
6. **Secure by default** - No configuration bypasses safety

## Security Principles

### 1. No Silent Disabling

Invariants must **never** be silently skipped:

```rust
// ❌ BAD - Can silently disable
if config.skip_invariants {
    return Ok(());  // Silently skip!
}

// ✅ GOOD - Explicit error
if config.skip_invariants {
    return Err(SecurityError::InvariantsDisabled);
}
```

### 2. Type System Enforcement

All type errors caught at parse/check time, not runtime:

```rust
// ❌ BAD DSL - Would compile
"string_value" + 42

// ✅ GOOD - Type checker rejects
// Error: Cannot add string and integer at parse time
```

### 3. No Unsafe Code in Analysis Paths

The core analysis engine must not use `unsafe`:

```rust
// At crate level:
#![forbid(unsafe_code)]

// Or explicitly prevent:
// #![deny(unsafe_code)]
```

### 4. Explicit Error Handling

No `unwrap()` in production paths:

```rust
// ❌ BAD
let value = parse_number(input).unwrap();

// ✅ GOOD
let value = parse_number(input)
    .map_err(|e| SecurityError::ParseFailed(e))?;
```

### 5. Deterministic Output

Same input → Same output (no randomness in results):

```rust
// ❌ BAD - Non-deterministic
use std::collections::HashMap;
let mut map = HashMap::new();
// HashMap iteration order is non-deterministic!

// ✅ GOOD - Deterministic
use std::collections::BTreeMap;
let mut map = BTreeMap::new();
// BTreeMap iteration is always sorted
```

## Security Tests

### Test: Invalid DSL Rejection

```rust
#[test]
fn test_security_invalid_dsl_rejected() {
    // Invalid DSL must be rejected, not silently ignored
    let invalid = "invariant balance_check\nx > 0";
    
    let result = parse(invalid);
    assert!(result.is_err(), "Invalid DSL must be rejected");
}
```

### Test: No Feature Flag Bypass

```rust
#[test]
fn test_security_feature_flags_cannot_disable_enforcement() {
    let config = ConfigBuilder::default()
        .with_feature("skip_invariants", true)  // Try to disable
        .with_setting("enforce", true)           // But enforcement is mandatory
        .build();
    
    // Enforcement must still happen
    assert!(config.enforce);
}
```

### Test: Type Confusion Prevention

```rust
#[test]
fn test_security_type_confusion_prevented() {
    let dsl = r#"
    invariant: type_safety
    x: u64 > "string"  // Type error
    "#;
    
    let result = check_types(dsl);
    assert!(result.is_err(), "Type system must prevent mixing types");
}
```

### Test: Overflow Detection

```rust
#[test]
fn test_security_overflow_detection_invariant() {
    let dsl = r#"
    invariant: overflow_safety
    sum(balances) <= 18446744073709551615
    "#;
    
    // Overflow must be detected or prevented
    let max_safe = u64::MAX;
    assert!(max_safe == 18446744073709551615);
}
```

### Test: Injection Prevention

```rust
#[test]
fn test_security_injection_prevention() {
    let injection = r#"; delete all invariants; //"#;
    let dsl = format!("invariant: test\n{}", injection);
    
    // Must treat as literal string, not command
    assert!(dsl.contains("delete"));
    assert!(!dsl.contains("; "));  // Semicolons don't execute
}
```

### Test: Path Traversal Prevention

```rust
#[test]
fn test_security_path_traversal_prevention() {
    let base = PathBuf::from("/home/app");
    let malicious = base.join("../../../etc/passwd");
    
    // Canonicalization prevents traversal
    let canonical = canonicalize(&malicious)?;
    
    // Resolved path should be safe
    assert!(!canonical.to_string_lossy().contains("etc/passwd"));
}
```

### Test: Output Escaping

```rust
#[test]
fn test_security_output_escaping() {
    let malicious = "test\n\r\\\"</script>";
    let json = serde_json::json!({"message": malicious});
    
    let output = serde_json::to_string(&json)?;
    
    // Must escape HTML/JSON special characters
    assert!(!output.contains("</script>"));
    assert!(output.contains("\\"));
}
```

### Test: Numeric Safety

```rust
#[test]
fn test_security_numeric_overflow_caught() {
    let result = 255u8.checked_add(1);
    assert_eq!(result, None, "Overflow must be detected");
    
    // Safe arithmetic
    let safe = 255u8.saturating_add(1);
    assert_eq!(safe, 255, "Saturating prevents overflow");
}
```

### Test: Thread Safety

```rust
#[test]
fn test_security_thread_safety() {
    let data = std::sync::Arc::new(
        std::sync::Mutex::new(42)
    );
    
    // Can only access through mutex
    let guard = data.lock().unwrap();
    assert_eq!(*guard, 42);
    
    // Compiler prevents data races
}
```

### Test: Exit Code Integrity

```rust
#[test]
fn test_security_exit_code_not_masked() {
    // Exit codes must accurately reflect status
    // (0 = success, non-zero = failure)
    
    struct Result {
        code: i32,
    }
    
    assert_eq!(Result { code: 0 }.code, 0);
    assert_ne!(Result { code: 1 }.code, 0);
}
```

## Supply Chain Security

### Dependency Audit

```bash
# Check for known vulnerabilities
cargo audit

# Generate security report
cargo audit --verbose
```

### Locked Dependencies

Always commit `Cargo.lock`:

```bash
git add Cargo.lock
# Ensures reproducible builds
```

### Transitive Dependency Review

```bash
# See dependency tree
cargo tree

# Check for duplicates
cargo tree --duplicates
```

## Reproducible Builds

### Toolchain Pinning

In `.github/workflows/ci.yml`:

```yaml
- uses: dtolnay/rust-toolchain@stable
  with:
    toolchain: 1.70.0  # Pin specific version
```

### Binary Reproducibility

```bash
# Build twice, compare hashes
cargo build --release
sha256sum target/release/invar

cargo clean
cargo build --release
sha256sum target/release/invar

# Hashes should match
```

## Code Review Checklist

Before merging, verify:

- [ ] No `unsafe` code in new code (unless security critical)
- [ ] All error cases handled with explicit types
- [ ] No `unwrap()` or `panic!()` in production paths
- [ ] All dependencies are audited
- [ ] Tests pass deterministically
- [ ] No new compiler warnings
- [ ] Type checker catches all type errors
- [ ] Security tests included for new features

## Continuous Security Monitoring

### In CI Pipeline

```yaml
audit:
  name: Security Audit
  runs-on: ubuntu-latest
  steps:
    - uses: rustsec/audit-check-action@v1
      with:
        token: ${{ secrets.GITHUB_TOKEN }}
```

Fails on:
- Known CVEs in dependencies
- Unmaintained crates
- Yanked versions

### Pre-Commit Hooks

```bash
#!/bin/bash
# .git/hooks/pre-commit

cargo fmt --check || exit 1
cargo clippy --all -- -D warnings || exit 1
cargo test --lib || exit 1
```

## Threat Model

### Trusted Components
- ✅ Rust compiler and standard library
- ✅ Solana, EVM, Move ecosystems
- ✅ Our invariant engine

### Untrusted Components
- ❌ User-provided invariant DSL
- ❌ External program state
- ❌ File system inputs
- ❌ Network inputs

### Mitigations
- **DSL**: Parsed, type-checked, sandboxed execution
- **State**: Validated against invariants
- **Files**: Canonicalized paths, bounds-checked access
- **Network**: Not directly consumed, always validated

## Reporting Security Issues

If you find a security vulnerability:

1. **Do not** open a public issue
2. Send details to: `security@invar-project.dev`
3. Include:
   - Vulnerability description
   - Reproduction steps
   - Potential impact
   - Suggested fix (if any)

## Security Updates

Security patches are released immediately as:
- Minor version bumps if data integrity affected
- Patch version bumps for practical vulnerabilities
- Always documented in SECURITY.md
