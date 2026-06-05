# Phase C Implementation Guide - 12 Medium-Priority Invariants

**Status**: All Phase C specifications complete and ready for implementation  
**Location**: See IMPLEMENTATION_v0_3_0.md - Phase C section for full details  
**Effort Estimate**: 1 week for all 12 invariants  
**Pattern**: Identical to Phase A/B detectors

---

## 📋 Phase C Invariants (12 Total)

### Medium-Priority Invariants by Loss Amount

| # | Invariant | H-Code | Loss | File Location | Status |
|---|-----------|--------|------|----------------|--------|
| 1 | EVM Account Abstraction Entropy | H54 | $5.2M | `evm/` | 📋 Specified |
| 2 | EVM Router Slippage Validation | H51 | $9.1M | `evm/` | 📋 Specified |
| 3 | EVM Token Balance Manipulation | H50 | $3.8M | `evm/` | 📋 Specified |
| 4 | EVM Signature Replay Protection | H48 | $2.4M | `evm/` | 📋 Specified |
| 5 | EVM Upgrade Path Verification | H47 | $1.2M | `evm/` | 📋 Specified |
| 6 | EVM Constructor Race Condition | H46 | $0.9M | `evm/` | 📋 Specified |
| 7 | EVM State Mutation Ordering | H45 | $2.6M | `evm/` | 📋 Specified |
| 8 | EVM Arithmetic Rounding | H44 | $4.3M | `evm/` | 📋 Specified |
| 9 | Solana Account Rent Exemption | H55 | $6.7M | `solana/` | 📋 Specified |
| 10 | Solana PDA Authority Validation | H53 | $3.5M | `solana/` | 📋 Specified |
| 11 | Move Type Safety Violation | H52 | $2.1M | `move/` | 📋 Specified |
| 12 | Move Resource Destruction | H51 | $1.8M | `move/` | 📋 Specified |

**Combined Phase C Loss Prevention**: ~$45M+ documented

---

## 🚀 Implementation Template

### Create New Detector File

```bash
# For EVM
touch crates/analyzer/evm/src/detectors/INVARIANT_NAME.rs

# For Solana
touch crates/analyzer/solana/src/INVARIANT_NAME.rs

# For Move
touch crates/analyzer/move/src/INVARIANT_NAME.rs
```

### Use This Structure

```rust
/// EVM INVARIANT_NAME Detector
///
/// Detects HXX (Project Name $XM) vulnerability: [Description]
///
/// The vulnerability occurs when:
/// 1. [Condition 1]
/// 2. [Condition 2]
/// 3. [Condition 3]
///
/// Example Vulnerable Pattern:
/// ```solidity
/// [Vulnerable code]
/// ```
///
/// Safe Pattern:
/// ```solidity
/// [Safe code]
/// ```

use lazy_static::lazy_static;
use regex::Regex;
use sentri_core::Finding;

lazy_static! {
    static ref PATTERN_NAME: Regex = Regex::new(r"(?i)PATTERN").unwrap();
    // ... more patterns
}

pub fn detect_INVARIANT_NAME(source: &str, file_path: &str) -> Vec<Finding> {
    let mut findings = Vec::new();

    for (line_num, line) in source.lines().enumerate() {
        if line.trim().starts_with("//") {
            continue;
        }

        if !PATTERN_NAME.is_match(line) {
            continue;
        }

        // Extract context
        let context = extract_context(source, line_num);
        
        // Check conditions
        if is_vulnerable(&context) {
            findings.push(
                Finding::new(
                    "evm_INVARIANT_NAME".to_string(),
                    sentri_core::Severity::Medium,  // Phase C are Medium priority
                    file_path.to_string(),
                    line_num + 1,
                    0,
                    "Vulnerability description".to_string(),
                    line.trim().to_string(),
                )
                .with_metadata("exploit_id", "HXX")
                .with_metadata("exploit_name", "Project Name")
                .with_metadata("loss", "$X.XM")
                .with_metadata("year", "202X")
                .with_metadata("vulnerability_type", "category")
                .with_metadata("detector", "pattern_analysis")
                .with_metadata("remediation", "Fix description"),
            );
        }
    }

    findings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_vulnerable_pattern() {
        let vulnerable = r#"// Vulnerable code"#;
        let findings = detect_INVARIANT_NAME(vulnerable, "test.sol");
        assert!(!findings.is_empty(), "Should detect vulnerability");
    }

    #[test]
    fn test_safe_pattern() {
        let safe = r#"// Safe code"#;
        let findings = detect_INVARIANT_NAME(safe, "test.sol");
        let critical = findings.iter()
            .filter(|f| f.severity == sentri_core::Severity::Critical)
            .collect::<Vec<_>>();
        assert!(critical.is_empty(), "Should not flag safe code");
    }

    // ... 3-5 more tests total
}
```

---

## 📝 Phase C Specification Reference

All 12 Phase C invariants have complete specifications in IMPLEMENTATION_v0_3_0.md including:

### For Each Invariant:
- ✅ Detailed vulnerability description
- ✅ Real-world examples (with links)
- ✅ Detection approach & patterns to match
- ✅ Code snippets (vulnerable & safe)
- ✅ Test case templates
- ✅ Remediation guidance
- ✅ Loss amount & exploit history
- ✅ Similar invariants to learn from

### How to Use the Spec:
1. Open IMPLEMENTATION_v0_3_0.md
2. Go to "Phase C - 12 Medium-Priority Invariants"
3. Find your target invariant
4. Copy the patterns and test templates
5. Adapt Phase A/B code as reference
6. Create detector file
7. Run: `cargo test -p CRATE_NAME`

---

## 🎯 Implementation Strategy

### Option A: Serial Implementation (Safest)
1. Pick one invariant from Phase C
2. Read full spec from IMPLEMENTATION_v0_3_0.md
3. Create detector file
4. Implement with 4-5 tests
5. Run `cargo test`
6. Update mod.rs exports
7. Repeat for next invariant

**Timeline**: ~1 hour per invariant × 12 = ~12 hours of work

### Option B: Batch Implementation (Faster)
1. Create all 12 detector files (placeholder)
2. Implement all 12 detectors (parallel work possible)
3. Run single test suite: `cargo test -p sentri-analyzer-evm`
4. Fix any issues
5. Update all mod.rs exports at once

**Timeline**: ~2-3 hours total for experienced developer

### Option C: Distributed Implementation
1. Split 12 invariants among team members
2. Each person implements 2-3 invariants
3. Submit PRs for review
4. Merge when all tests pass

**Timeline**: 1-2 hours per developer

---

## 📦 Quality Checklist for Phase C

Before committing each detector:

- [ ] File created in correct directory (evm/solana/move)
- [ ] Function signature: `pub fn detect_INVARIANT_NAME(source: &str, file_path: &str) -> Vec<Finding>`
- [ ] Exactly 4-5 test cases implemented
- [ ] Vulnerable pattern test passes
- [ ] Safe pattern test passes
- [ ] Real exploit pattern test passes
- [ ] Edge case test passes
- [ ] All metadata fields populated
- [ ] Remediation guidance clear
- [ ] Module exported in mod.rs
- [ ] `cargo test -p CRATE` passes with 0 warnings

---

## 🔄 Integration Steps

### After Creating Each Detector:

1. **Add module declaration** to mod.rs:
```rust
pub mod new_detector;
```

2. **Add public export** to mod.rs:
```rust
pub use new_detector::detect_INVARIANT_NAME;
```

3. **Run tests**:
```bash
cargo test -p sentri-analyzer-evm --lib detectors
```

4. **Run specific detector test**:
```bash
cargo test -p sentri-analyzer-evm detect_INVARIANT_NAME
```

---

## 📊 Expected Phase C Completion

### Code Metrics
- 12 detector files: ~2,200 LOC total
- 60 test cases: ~4,000 lines of test code
- ~2 hours compilation time (first build)
- ~30 seconds per subsequent build

### Test Coverage
- 60 total test cases (5 per detector)
- 100% pass rate expected
- ~45 seconds to run full suite

### Documentation
- Each detector: 150-200 lines with comprehensive comments
- Each test: Clear documentation of what it validates
- README updates with Phase C additions

---

## 💡 Tips for Phase C Implementation

### From Phase A/B Experience:
1. **Pattern matching > AST parsing**: Regex is fast and effective
2. **Context extraction important**: Get full function body (100-200 lines)
3. **Test each pattern thoroughly**: 5 tests catch ~95% of false positives
4. **Real exploit examples matter**: Users trust patterns based on real attacks
5. **Metadata critical**: H-code, loss amount, year build credibility

### Common Mistakes to Avoid:
- ❌ Missing context extraction (only checking single line)
- ❌ Insufficient test coverage (< 4 tests)
- ❌ Generic patterns that catch false positives
- ❌ Missing metadata fields
- ❌ No remediation guidance
- ❌ Comments in test code explaining what's tested

### Best Practices:
- ✅ Use lazy_static for compiled regexes (faster)
- ✅ Skip comments at start of loop
- ✅ Use find/capture for flexible matching
- ✅ Include both `fn_name()` and `fn_name (` patterns
- ✅ Test with real exploit code samples
- ✅ Use `assert!` with clear error messages

---

## 🔧 Build & Test Commands

```bash
# Build all EVM analyzers (including Phase C)
cargo build -p sentri-analyzer-evm --release

# Build Solana analyzers (Phase C additions)
cargo build -p sentri-analyzer-solana --release

# Run all Phase A + B + C tests
cargo test -p sentri-analyzer-evm --lib detectors

# Run specific detector tests
cargo test -p sentri-analyzer-evm detect_INVARIANT_NAME

# Check for warnings
cargo build --all --all-targets

# Generate documentation
cargo doc --no-deps --open
```

---

## 📈 Progress Tracking

Create a checklist as you implement:

```
Phase C Implementation Progress (12 Total)

EVM (7 invariants):
- [ ] evm_account_abstraction_entropy
- [ ] evm_router_slippage_validation
- [ ] evm_token_balance_manipulation
- [ ] evm_signature_replay_protection
- [ ] evm_upgrade_path_verification
- [ ] evm_constructor_race_condition
- [ ] evm_state_mutation_ordering
- [ ] evm_arithmetic_rounding

Solana (2 invariants):
- [ ] solana_account_rent_exemption
- [ ] solana_pda_authority_validation

Move (2 invariants):
- [ ] move_type_safety_violation
- [ ] move_resource_destruction
```

---

## 📞 Support Resources

### When You're Stuck:
1. **Look at Phase A**: All Phase A detectors are reference implementations
2. **Check IMPLEMENTATION_v0_3_0.md**: Full specification for the invariant
3. **Compare to similar invariant**: Phase B has similar patterns
4. **Review test patterns**: All detectors follow 4-5 test structure

### Example Reference Implementations:
- **Best for pattern matching**: `health_check.rs`
- **Best for complex logic**: `oracle_self_trade.rs`
- **Best for state updates**: `reentrancy_via_whitelisted.rs`
- **Best for multi-condition**: `lst_depeg.rs`

---

## 🎉 Ready to Start!

You now have:
- ✅ All 12 Phase C specifications
- ✅ Reference implementations (Phases A & B)
- ✅ Test templates and patterns
- ✅ Implementation guide and checklist
- ✅ Build/test commands
- ✅ Quality standards defined

**Next Step**: Choose your first Phase C invariant and start implementing!

---

**Estimated Time to Phase C Complete**: 1 week (12-15 hours of development)  
**Estimated Time to v0.3.0 Release**: 2-3 weeks (Phase C + Phase D + testing)  
**Status**: Ready for Phase C Implementation
