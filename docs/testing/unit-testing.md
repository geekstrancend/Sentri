# Unit Testing Guide

## Overview

Unit tests validate the correctness of individual components in isolation. They form the foundation of the testing pyramid and should comprise the bulk of tests.

**Target Coverage:** 90%+ for all core modules

## Test Structure

```
tests/unit/
├── mod.rs            # Main unit test module
├── parser_tests      # DSL parser tests
├── type_checker_tests # Type system tests
├── evaluator_tests   # Expression evaluator tests
└── ast_tests         # AST construction tests
```

## Key Testing Principles

### 1. **Determinism**
Every test must be deterministic. Given the same input, it must always produce the same output.

**Bad:**
```rust
#[test]
fn parse_with_randomness() {
    let input = generate_random_dsl();  // ❌ Non-deterministic
    let result = parse(input);
    assert!(result.is_ok());
}
```

**Good:**
```rust
#[test]
fn parse_fixed_invariant() {
    let input = r#"invariant: test
    forall x in items: x > 0"#;
    let result = parse(input);
    assert!(result.is_ok());
}
```

### 2. **Isolation**
Tests should not depend on global state or other tests.

**Bad:**
```rust
static mut GLOBAL_STATE: Option<State> = None;

#[test]
fn test_with_global() {
    unsafe { GLOBAL_STATE = Some(init_state()); }  // ❌ Global mutation
    // ...
}
```

**Good:**
```rust
#[test]
fn test_with_local_state() {
    let state = init_state();  // ✅ Local state
    // ...
}
```

### 3. **Explicit Error Cases**
Test both success and failure paths.

```rust
#[test]
fn test_parse_valid_input() {
    let result = parse("invariant: test\ntrue");
    assert!(result.is_ok());
}

#[test]
fn test_parse_invalid_input() {
    let result = parse("invalid");
    assert!(result.is_err(), "Should explicitly reject invalid input");
}
```

## Running Tests

```bash
# Run all unit tests
cargo test --lib

# Run specific test
cargo test parser_tests::test_parse_simple_invariant

# Run with output
cargo test -- --nocapture

# Run with multiple threads disabled (for debugging)
cargo test -- --test-threads=1
```

## Coverage Measurement

```bash
# Generate coverage report
cargo tarpaulin --all --out Html

# View coverage
open tarpaulin-report.html
```

## Common Test Patterns

### Testing Parser Correctness

```rust
#[test]
fn test_parse_with_context() {
    let input = r#"
    context {
        state: AccountState,
        chain: Solana
    }
    
    invariant: test
    true
    "#;
    
    let result = parse(input);
    assert!(result.is_ok());
    // Verify AST structure
}
```

### Testing Type Consistency

```rust
#[test]
fn test_type_mismatch_detection() {
    let mut checker = TypeChecker::new();
    let expr = "\"string\" + 42";  // Type error
    
    let result = checker.check_expr(expr);
    assert!(result.is_err(), "Type checker should catch string + integer");
}
```

### Testing Evaluation Correctness

```rust
#[test]
fn test_arithmetic_precedence() {
    let evaluator = Evaluator::new();
    let result = evaluator.eval("2 + 3 * 4");
    
    // Assuming result is 14 (not 20)
    assert_eq!(result.ok(), Some(14));
}
```

## Extending Unit Tests

When adding new functionality:

1. Write tests **before** implementation (TDD)
2. Test **both** happy paths and error cases
3. Ensure tests are **deterministic**
4. Verify **coverage > 90%**
5. Run full suite multiple times to confirm determinism

```bash
for i in {1..5}; do cargo test --lib || exit 1; done
```

## Best Practices

- **One assertion per test** (when possible)
- **Descriptive test names** that explain what's being tested
- **Test edge cases**: empty input, maximum values, invalid syntax
- **No unwrap()** in test setup (use expect with message)
- **Always cleanup** temporary resources

## Troubleshooting

**Test passes locally but fails in CI:**
- Check for system-specific paths or timing issues
- Verify tests don't depend on execution order
- Run tests multiple times: `cargo test -- --test-threads=1`

**Non-deterministic failures:**
- Remove any randomness (use fixed seeds if needed)
- Check for HashMap/HashSet ordering (use BTreeMap/BTreeSet)
- Eliminate timer-dependent logic

**Slow tests:**
- Profile with `cargo test -- --nocapture --test-threads=1`
- Consider moving to integration tests
- Use `#[ignore]` for slow tests and run separately
