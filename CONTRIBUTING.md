# Contributing to Invar

Thank you for your interest in contributing to Invar! This document provides guidelines for contributing to the project.

## Code of Conduct

We are committed to providing a welcoming and inclusive environment. Please be respectful and professional in all interactions.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/your-username/invar.git`
3. Create a feature branch: `git checkout -b feature/your-feature`
4. Make your changes
5. Commit with clear messages: `git commit -m "Add feature: description"`
6. Push to your fork: `git push origin feature/your-feature`
7. Open a Pull Request

## Code Standards

### Rust Code

All code must meet these standards before being merged:

#### 1. **Compilation**
```bash
cargo build --all
```
Must compile without errors on Rust stable (1.93+).

#### 2. **Clippy Linting**
```bash
cargo clippy --all --all-targets -- -D warnings
```
All clippy warnings must be resolved. No exceptions.

#### 3. **Formatting**
```bash
cargo fmt --all -- --check
```
Code must be formatted with `rustfmt`. Run `cargo fmt --all` to auto-format.

#### 4. **Tests**
```bash
cargo test --all
```
All tests must pass. New features must include tests with >90% coverage.

#### 5. **Documentation**
```bash
cargo doc --all --no-deps
```
All public items must have documentation comments.
```rust
/// Analyzes a smart contract program.
///
/// # Arguments
///
/// * `path` - Path to the program source file
///
/// # Errors
///
/// Returns an error if the file cannot be read or parsed.
pub fn analyze(&self, path: &Path) -> Result<ProgramModel>
```

#### 6. **No Panics in CLI**
- Library code may use `unwrap()` only in unreachable branches with explicit comments
- CLI code must never panic - use `Result` types instead
- All user-facing operations must return `Result<T, InvarError>`

#### 7. **No Unsafe Code**
- All unsafe code must be explicitly justified with a comment
- Unsafe blocks must be minimized and isolated
- Document the safety invariant being upheld

#### 8. **Determinism**
- No global mutable state
- Use seeded RNGs where randomness is needed
- Use sorted collections (BTreeMap, BTreeSet) instead of unordered ones
- Tests must be deterministic and reproducible

### Example PR Checklist

- [ ] Runs `cargo build` without errors
- [ ] Runs `cargo clippy` without warnings
- [ ] Runs `cargo fmt` (all code formatted)
- [ ] Runs `cargo test` (all tests pass)
- [ ] Added tests for new functionality
- [ ] Added documentation for public APIs
- [ ] Commits have clear, descriptive messages
- [ ] PR description explains the changes

## Commit Message Guidelines

Use clear, descriptive commit messages:

```
Brief summary (50 chars max)

Longer explanation (72 chars per line) describing:
- What changed
- Why it changed
- Any relevant context

Fixes #123
```

Examples:
- ❌ "fix stuff"
- ✅ "Fix invariant parser handling of logical operators"
- ❌ "update"
- ✅ "Add cross-chain invariant validation"

## Feature Requests

Before implementing a new feature:

1. Open a GitHub issue with the label `enhancement`
2. Describe the use case and expected behavior
3. Discuss with maintainers
4. Get consensus before starting implementation

## Bug Reports

When reporting bugs:

1. Include your Rust version (`rustc --version`)
2. Provide a minimal reproduction example
3. Describe expected vs actual behavior
4. Include relevant logs (enable with `RUST_LOG=debug`)

Example:
```
**Describe the bug**
Parser fails on valid logical operator syntax.

**To Reproduce**
```
invariant Test {
    (a || b) && c
}
```

**Expected behavior**
Should parse successfully.

**Actual behavior**
Error: expected unary at ...

**Environment**
- Rust version: 1.93.0
- Invar version: 0.1.0
- OS: Linux
```

## Testing Guidelines

### Unit Tests

Test individual functions:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_expression() {
        let input = "balance >= 0";
        let result = parse_expr(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().name, "balance");
    }

    #[test]
    fn test_invalid_syntax_fails() {
        let input = "balance >= ";
        let result = parse_expr(input);
        assert!(result.is_err());
    }
}
```

### Integration Tests

Place in `tests/` directory:
```rust
// tests/integration_test.rs
#[test]
fn test_end_to_end_analysis() {
    let analyzer = SolanaAnalyzer::new();
    let result = analyzer.analyze("examples/solana_token_transfer.rs");
    assert!(result.is_ok());
    let program = result.unwrap();
    assert_eq!(program.functions.len(), 2);
}
```

### Performance Tests

For performance-critical code:
```rust
#[bench]
fn bench_parse_large_expression(b: &mut Bencher) {
    let expr = /* large expression */;
    b.iter(|| parse_expr(&expr));
}
```

## Documentation Guidelines

### Module-Level Docs

```rust
//! Module description.
//!
//! This module handles X and is responsible for Y.
//!
//! # Examples
//!
//! ```
//! let item = ModuleName::new();
//! item.do_something()?;
//! ```
```

### Function Docs

```rust
/// Short description (one sentence).
///
/// Longer description explaining behavior, constraints, etc.
///
/// # Arguments
///
/// * `param1` - Description of param1
/// * `param2` - Description of param2
///
/// # Returns
///
/// Returns X when successful.
///
/// # Errors
///
/// Returns `InvarError::InvalidInput` if param validation fails.
///
/// # Examples
///
/// ```
/// let result = function(arg1, arg2)?;
/// assert_eq!(result.value, expected);
/// ```
pub fn function(param1: Type1, param2: Type2) -> Result<ReturnType>
```

## Release Process

Maintainers handle releases. When ready:

1. Update version in `Cargo.toml` (semantic versioning)
2. Update `CHANGELOG.md`
3. Create a git tag: `git tag v0.2.0`
4. Push tag: `git push origin v0.2.0`
5. GitHub Actions builds and uploads release artifacts

## Performance Considerations

When contributing performance-sensitive code:

- Profile before optimizing
- Benchmark improvements with `cargo bench`
- Consider memory allocation patterns
- Use iterators instead of collecting vectors when possible
- Avoid cloning where feasible

## Architecture Decisions

Major architectural changes should:

1. Be discussed in a GitHub discussion first
2. Include design rationale
3. Consider backward compatibility
4. Include migration guide if breaking

## Questions?

- Open a GitHub discussion
- Check existing issues
- Review documentation
- Ask in pull request comments

## Thank You

We appreciate your contributions to making Invar better! 🚀
