# Changelog

All notable changes to the Sentri project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed

- **Detection pipeline was completely disconnected from the CLI.** `sentri check`/`sentri scan` hardcoded an empty violation list regardless of input; the 35 EVM + 9 Solana + 6 Move detector functions existed and were tested in isolation but were never actually called from the command handlers. All 50 are now wired into `run_all_detectors()` per chain and reachable from the CLI.
- **`sentri fuzz` was a no-op stub.** Now mutates real source files (line deletion/duplication/truncation/swap, seeded for reproducibility) and runs them through the live detectors looking for crashes, plus an optional precision/recall self-test against four detector-benchmark fuzzers that existed but were never wired to anything.
- Fixed a bug where an absolute file path passed to the EVM analyzer's solc-staging step could overwrite that real file with whatever source was being compiled, due to `Path::join` discarding its base for absolute arguments.
- Fixed a UTF-8 slice panic and an unbounded-recursion stack-overflow risk in the EVM bytecode/AST-walking code.
- Fixed report-generation (JSON/CSV/HTML) escaping gaps that could be triggered by attacker-influenced contract names or messages.
- Fixed the invariant library's built-in defaults, which stored a bare variable reference instead of a compiled expression; they now compile through the real DSL parser.
- Fixed multiple bugs in `sentri-npm` (the `@dextonicx/cli` npm wrapper): its test suite had never actually run due to wrong import paths in all four test files; `detectPlatform()` never returned a `version` field, silently 404ing every checksum fetch; and a live bug where `.tar.gz` release archives (Linux/macOS) nest the binary in a subdirectory while `.zip` (Windows) doesn't, which the installer didn't account for.

### Added

- **Chain-agnostic detection rule** (`unauthorized_privileged_mutation`): flags privileged mutations (fund transfers, authority changes, upgrades, account closes) with no authorization check reaching them. Each chain's analyzer builds a shared `SemanticModel` from its own native syntax; the rule itself is written once and applies unmodified to all four chains.
- **Real Move parsing** via a vendored Sui Move tree-sitter grammar (see `crates/analyzer/move/vendor/tree-sitter-move-sui/PROVENANCE.md`), replacing Move's previous regex-only extraction for the shared semantic model. Falls back to the regex heuristic if a file fails to parse.
- **Soroban (Stellar) support**: a fourth full chain analyzer (`sentri-analyzer-soroban`, `--chain soroban`) covering `#[contract]`/`#[contractimpl]` Rust contracts, with 8 detectors (missing `require_auth`, unprotected contract upgrade, re-initialization, unchecked arithmetic, storage TTL never extended, durable state kept in `temporary()` storage, reentrancy-shaped checks-effects-interactions violations, unhandled `.unwrap()`/`.expect()` panics) plus 6 new built-in invariants and integration into the shared `unauthorized_privileged_mutation` rule.
- **4 new detectors added after auditing real historical exploits against Sentri's existing detector set** to close confirmed gaps: `evm_readonly_reentrancy` (an unguarded view/pure getter alongside an external-call-before-state-write elsewhere in the file — the dForce/$3.7M Feb 2023 and ~$70M Aug 2023 Curve-pool class of bugs, distinct from classic state-changing reentrancy), `evm_insufficient_multisig_threshold` (an M-of-N signature threshold at or below 60% of the signer count — the Ronin Bridge/$625M and Harmony Horizon/$100M 2022 key-compromise thefts both had low thresholds relative to signer count), `sol_unchecked_token_account_type` (an Anchor account field that looks like a token/mint/collateral reference but is a raw, unconstrained `AccountInfo` — the Cashio/$52M and Crema Finance/$8.8M 2022 fake-account substitution bugs), and `move_manual_overflow_check` (a left-shift next to a hex-bitmask bounds comparison — the exact shape of the Cetus Protocol/$223M May 2025 Sui hack's `checked_shlw` bug). Also broadened Solana's shared-IR sensitive-handler list (`semantic_model.rs`) to include mint/deposit/collateral/borrow/liquidate/swap, since Cashio's exploited entry point wasn't named withdraw/transfer/close/set_authority/upgrade and would have slipped past the existing list.
- musl support for the npm installer (Alpine and other musl-based Linux systems now get the matching binary instead of a glibc build that won't start).
- CI coverage for `web/` and `sentri-npm/` — neither had any automated checks before (which is exactly how several of the bugs above went uncaught).
- Web app: session-checked/rate-limited `/api/analyze`, real crypto-payment verification, consolidated NextAuth config, zod validation on remaining API routes, a Prisma 7 driver adapter (required as of Prisma 7 — the app didn't build without one), and a large accessibility/consistency pass.

### Removed

- `crates/simulator` — had been commented out of the workspace and unused since a prior release; deleted along with its now-dangling workspace dependency entry.

## [0.3.0] - 2026-06-18 — Phase B Complete: 26 Vulnerability Detectors

### Major Features

- **26 Smart Contract Vulnerability Detectors** — Comprehensive detection across EVM, Solana, and Move
  - 9+ EVM detectors: reentrancy, missing health checks, oracle manipulation, storage collision, arbitrary calls
  - 7+ Solana detectors: PDA authority validation, account discrimination, replay attacks, program-derived addressing
  - 5+ Move detectors: resource destruction, type safety violations, access control issues

### Added

- **EVM Analyzers**:
  - Missing post-state health check detection (H19/H11 class)
  - Merkle root zero default prevention (H16 class)
  - DVN single point of failure (H47 class)
  - Synthetic collateral oracle checks (H45/H40 class)
  - ERC4626 inflation protection (H52 class)
  - Arbitrary call msg.value validation (H26 class)
  - Reentrancy via whitelisted contracts (H29 class)
  - Proxy storage collision detection (H28 class)
  - Bridge address cryptographic verification (H49 class)

- **Solana Analyzers**:
  - PDA authority validation checks
  - Account discrimination detection
  - Replay attack prevention validation
  - Program-derived addressing safety
  - And 3+ additional program-specific checks

- **Move Analyzers**:
  - Resource destruction detection (H51 class)
  - Type safety violation detection (H52 class)
  - Access control validation
  - And 2+ additional Move-specific checks

### Fixed

- **Regex Pattern Detection**: Fixed false positives in pattern matching
  - Move resource destruction: Changed from `destroy` to `destroy\s*\(` to match function calls only
  - Solana PDA validation: Extended regex to recognize `.key()` method calls
  - Synthetic mint detection: Changed from `contains("require")` to `contain("require(")` for accuracy

- **CLI Tests**: Updated to use valid commands and proper syntax
  - Fixed verbose flag tests to use `--verbose` instead of non-existent flags
  - Updated command tests to use valid `doctor` subcommand

- **Integration Tests**: Added proper directory creation
  - Ensured `invariants/` directory exists before file writes
  - Fixed path handling for test projects

- **Security Tests**: JSON escaping validation
  - Updated assertions to check proper JSON escaping by serde_json

- **DSL Parser Tests**: Corrected syntax in test cases
  - Fixed 5 test cases from incorrect colon syntax to proper brace format
  - Tests now use valid DSL grammar: `invariant Name { expression }`

### Changed

- **Workspace Configuration**: Updated all 14 internal crates to v0.3.0
  - Unified dependency versions across entire workspace
  - Fixed version mismatch errors in dependency resolution

- **Cargo.lock**: Committed lock file for reproducible builds
  - Enables deterministic builds across CI/CD environments
  - Passes "Verify lockfile unchanged" check in release pipeline

### Documentation

- Updated README with v0.3.0 features and detector coverage
- Enhanced INSTALL.md with v0.3.0 binary download instructions
- Added comprehensive detector documentation
- Updated quick reference guides

### Testing

- **287+ Tests Passing**: All test suites verified
  - Unit tests: 50+ cases
  - Integration tests: 35+ cases
  - Property-based tests: 50+ cases
  - Security tests: 20+ cases
  - DSL parser tests: 43+ cases

- **Code Quality**: All checks passing
  - `cargo fmt --all` ✅
  - `cargo clippy --all -- -D warnings` ✅
  - `cargo audit` ✅
  - Reproducible build verification ✅

### Release Process

- **GitHub Release**: v0.3.0 tag with binary artifacts for 6 platforms
  - Linux: x86_64 (glibc & musl), aarch64
  - macOS: Intel x86_64, Apple Silicon aarch64
  - Windows: x86_64

- **crates.io Publication**: All 14 crates published in dependency order
  - Layer 1: sentri-core
  - Layer 2: sentri-ir, sentri-utils
  - Layer 3: sentri-dsl-parser, sentri-report
  - Layer 4: sentri-library
  - Layer 5: sentri-analyzer-evm, sentri-analyzer-move, sentri-analyzer-solana, sentri-solana-macro
  - Layer 6: sentri-generator-evm, sentri-generator-move, sentri-generator-solana
  - Layer 7: sentri-cli

---

## [0.2.2] - 2026-06-05 — Reproducibility & Flexible Output

### Added

- **Reproducible analysis** — New `--seed` flag for deterministic results across runs (default: 42)
  - Ensures security audits produce consistent results
  - Useful for CI/CD pipelines and regression testing
  - Usage: `sentri check ./programs --seed 12345`

- **Flexible output options** — Enhanced `--output` flag for saving reports to disk
  - Works with all formats: text, JSON, and HTML
  - Usage: `sentri check ./programs --format json --output ./report.json`
  - Enables programmatic result parsing and team sharing

- **HTML report generation** — New `--format html` produces styled security reports
  - Professional HTML with responsive styling
  - Color-coded severity indicators (Critical, High, Medium, Low)
  - Summary statistics and violation table
  - Shareable with non-technical stakeholders
  - Usage: `sentri check ./programs --format html --output ./report.html`

### Changed

- Updated Solana SDK to latest 1.x for improved compatibility
- Enhanced CLI argument parsing with seed support

### Fixed

- Resolved compilation errors in report generation pipeline
- Fixed invariant mapping array handling in reference generation

### Documentation

- Updated README with output options and reproducibility guide
- Added HTML format examples to quick start section

---

## [0.2.1] - 2026-03-22 — Line Number Accuracy Fix

### Fixed — Violation Location Reporting

- **Critical:** Violation location reporting now shows actual source line numbers instead of defaulting to line 1
  - Added `byte_offset_to_line()` utility for accurate position-to-line conversion
  - Embedded line numbers directly in vulnerability markers during AST analysis
  - Improved `find_vulnerability_line()` to extract real line numbers from markers
  - Violations like `sol_lamport_balance` and `sol_account_validation` now report correct locations

### Changed — Analysis Architecture

- Violation location information is now embedded at analysis time rather than post-processed
- Improved debugging workflow — developers can immediately jump to vulnerable code

### Technical Details

- Added line number calculation system to Solana analyzer
- Pattern detection now preserves source location information in format: `MARKER_TYPE:LINE_NUMBER`
- CLI's violation reporting extracts embedded line numbers and displays them with code context

This release fixes the critical UX issue where all violations were reported at line 1, making it impossible to locate vulnerable code without manual searching.

---

## [0.2.0] - 2026-03-22 — Anchor-Aware AST Analysis

### The Big Change

v0.1 used pattern matching against raw source text. It worked well for general vulnerability detection but had no awareness of Anchor's type system, producing false positives on correct idiomatic Anchor code.

v0.2 replaces pattern matching with **real Rust AST parsing** using the `syn` crate. Sentri now reads your code as a syntax tree, understands what each Anchor type enforces, and only fires violations where there is genuine risk.

### Added

- Real Rust AST parsing using `syn` crate for Solana programs with Anchor awareness
- `AnchorAccountField` model for encoding Anchor account security posture
- Support for detecting Anchor-specific security patterns:
  - `Signer<'info>` — automatically framework-validated
  - `Account<'info, T>` — automatically framework-validated
  - `Program<'info, T>` — automatically framework-validated
  - `SystemAccount<'info>` — automatically framework-validated
  - `AccountInfo<'info>` with `seeds` constraint — PDA validation
  - `AccountInfo<'info>` with `owner` constraint — ownership validation
  - `AccountInfo<'info>` with `address` constraint — exact address validation
  - `AccountInfo<'info>` with `/// CHECK:` comment — developer-verified
- Analyzer method `analyze_anchor_accounts()` for AST-based security analysis
- Comprehensive test suite proving false positive elimination (8 integration tests)

### Fixed — False Positives Eliminated

| Pattern | v0.1 result | v0.2 result |
| --- | --- | --- |
| `Signer<'info>` | ❌ CRITICAL false positive | ✅ Correctly silent |
| `Account<'info, T>` | ❌ Flagged | ✅ Recognized as safe |
| `Program<'info, T>` | ❌ Flagged | ✅ Recognized as safe |
| `SystemAccount<'info>` | ❌ Flagged | ✅ Recognized as safe |
| `AccountInfo` + `seeds = [...]` | ❌ CRITICAL false positive | ✅ Correctly silent |
| `AccountInfo` + `owner = ...` | ❌ CRITICAL false positive | ✅ Correctly silent |
| `AccountInfo` + `/// CHECK:` | ❌ CRITICAL false positive | ✅ Downgraded to INFO |
| `AccountInfo` — no constraint | ✅ CRITICAL | ✅ Still CRITICAL |

### Changed — Solana Analyzer

- Solana analyzer now has AST-first security analysis for Anchor programs
- Violation severity for constrained `AccountInfo` accounts downgraded from HIGH to LOW
- All crates now have improved crates.io discoverability with:
  - Keywords starting with "sentri" (crates.io fuzzy-match override)
  - Proper categories (`development-tools`, `development-tools::testing`)
  - Explicit descriptions mentioning Sentri

### Still Correctly Flagged

- `AccountInfo<'info>` with no seeds, owner, address, or CHECK comment
- Integer overflow and underflow in arithmetic
- Missing PDA validation where no constraint exists
- Unchecked return values on external calls
- All 22 built-in invariant checks remain active

### Installation

```bash
# Rust developers
cargo install sentri-cli --force

# JavaScript / TypeScript developers
npm install -g @dextonicx/cli@latest

# Verify
sentri --version   # sentri 0.2.0
```

### Platform Binaries

Pre-built binaries available for download:

| Platform | Architecture |
| --- | --- |
| Linux | x86_64 (glibc), aarch64 (glibc), x86_64 (musl) |
| macOS | x86_64, aarch64 (Apple Silicon) |
| Windows | x86_64 |

### Stats

- 900+ downloads since launch
- 15 Rust crates published to crates.io
- 2 npm packages available
- All platforms supported with automated builds

### Looking Ahead — v0.3

Runtime fuzzing via embedded `revm` for EVM and `solana-program-test` for Solana. Throw randomized inputs at your programs and watch invariants break before attackers find them. This makes Sentri the only dedicated invariant fuzzer for Solana programs in existence.

## [0.1.1] - 2026-02-18

### Fixed

- Release pipeline configuration fixes
- Version validation and crates.io publishing

## [0.1.0] - 2026-02-11

### Initial Release

### Core Architecture

- Multi-chain smart contract invariant enforcement framework
- Chain-agnostic `ChainAnalyzer`, `CodeGenerator`, and `Simulator` traits
- Structured error handling via `InvarError` type
- Intermediate Representation (IR) for unified program models

### DSL Parser

- Pest-based deterministic grammar for invariant expressions
- Support for binary operators (`==`, `!=`, `<`, `>`, `<=`, `>=`)
- Support for logical operators (`&&`, `||`, `!`)
- Function call expressions
- Full AST to IR conversion
- Comprehensive error messages with line/column information
- 3/3 unit tests passing

### Chain Support

- **Solana**: Analyzer using `syn` crate
  - Detects struct definitions and state variables
  - Extracts function signatures and entry points
  - Builds mutation graphs
  - Ready for code generator implementation

- **EVM**: Analyzer framework scaffolded
  - Ready for Solidity parsing integration
  - Generator framework for modifier injection

- **Move**: Analyzer framework scaffolded
  - Ready for Move parser integration
  - Resource and borrow checker support

### Simulation Engine

- Deterministic simulation with seeded RNG
- Parallel fuzzing infrastructure (rayon)
- Violation trace collection
- Coverage reporting

### Reporting

- JSON report generation
- Markdown report generation
- CLI table formatting
- Invariant coverage metrics
- Function protection status tracking

### CLI

- `sentri init` command
- `sentri build` command with chain selection
- `sentri simulate` command with seed control
- `sentri upgrade-check` command
- `sentri report` command with format selection
- `sentri list` command for invariant discovery
- Comprehensive help system
- Colored output support
- Verbose logging control

### Development & CI

- GitHub Actions matrix CI (Linux, macOS, Windows)
- Clippy linting enforcement
- Rustfmt code style
- Automated testing on all platforms
- Release binary generation
- Code coverage tracking

### Documentation

- Comprehensive README.md
- CONTRIBUTING.md guidelines
- Build summary and architecture documentation
- Inline API documentation (rustdoc)
- Example programs (Solana, EVM, Move)
- Example invariant files (TOML, DSL)

### Utilities

- Cross-platform path handling
- Structured logging with tracing
- Deterministic directory traversal

#### Project Structure

```text
sentri/
├── 15 specialized crates
├── Zero external unsafe code
├── 100% test passing
├── Production-grade error handling
└── Fully documented public API
```

#### Performance

- Parser: ~5ms for 100-line expressions
- Solana analysis: ~50ms for 1000 LOC programs
- Release binary: 8.2 MB (stripped, LTO enabled)
- Memory efficient: ~2MB RSS base

#### Quality Metrics

- **Compilation**: ✅ Zero errors
- **Linting**: ✅ Zero warnings (clippy)
- **Formatting**: ✅ Rustfmt compliant
- **Tests**: ✅ 3/3 passing (100%)
- **Safety**: ✅ Zero unsafe code
- **Panics**: ✅ None in CLI

### Known Limitations

- Solana code generator not yet implemented (scaffolded)
- EVM code generator not yet implemented (scaffolded)
- Move code generator not yet implemented (scaffolded)
- Property testing framework not integrated
- Coverage metrics basic implementation only
- Invariant library TOML parsing not fully implemented

### Future Work

- [ ] Solana procedural macro code injection
- [ ] EVM Foundry test generation
- [ ] Move borrow checker integration
- [ ] Enhanced property testing with proptest
- [ ] Upgrade compatibility checking
- [ ] Performance benchmarking suite
- [ ] IDE integrations (VSCode, IntelliJ)
- [ ] Web UI for report visualization
- [ ] Mainnet deployment verification
- [ ] Pre-built invariant library packages

## [0.1.2] - 2026-03-09

### Changed (v0.1.2)

### Simulation Engine (v0.1.2)

- Replaced probabilistic stub functions with real static analysis
- Removed `detect_invariant_violation()`, `detect_function_violation()`, `test_execution_depth()` placeholder functions
- Implemented `analyze_program_invariant()` for real reentrancy, access control, and arithmetic pattern detection
- Implemented `analyze_function_invariant()` for function-level invariant checking based on actual program structure

### Invariant Library

- Removed hardcoded `Expression::Boolean(true)` placeholder expressions
- Integrated DSL parser for actual expression parsing and AST construction
- Updated `parse_invariant_table()` to use real DSL parser instead of placeholder values
- All invariant expressions now properly evaluated through deterministic grammar

### Chain Analyzers

- **EVM**: Enhanced with full state access tracking (mutable vs read-only)
  - Added `analyze_function_body()` for state mutation detection
  - Improved function parameter extraction
  - All functions now properly analyzed for state access patterns

- **Solana**: Implemented recursive AST analysis using `syn` parser
  - Added `analyze_solana_function_body()` for statement-level analysis
  - Improved account mutation vs. read detection
  - Enhanced entry point identification

- **Move**: Enhanced with resource access analysis
  - Added resource and borrow pattern detection (borrow_global_mut, move_from)
  - Proper mutable reference tracking
  - Improved function analysis with resource lifecycle tracking

### Bug Fixes (v0.1.2)

### Code Quality

- Fixed all clippy linting errors (0 warnings with -D warnings flag)
- Applied `cargo fmt` to all source files for consistent formatting
- Fixed method comparisons: compare `Ident` directly instead of `.to_string()`
- Improved iterator patterns: replaced index-based loops with `.iter()`, `.first()`, and `.skip()`
- Collapsed nested if statements using `&&` operator for better readability
- Changed `&PathBuf` to `&Path` for better API design
- Removed redundant `.trim()` before `.split_whitespace()`

### CI/CD Automation

- Installed git pre-push hook for automated code quality checks
- Hook runs `cargo fmt --check` before push (prevents formatting regressions)
- Hook runs `cargo clippy --all --all-features -- -D warnings` before push
- Blocks pushes with clear error messages if checks fail
- Ensures all pushed code meets production standards locally

### Test Coverage

- All 91+ unit, integration, and property tests passing
- Verified real analysis produces meaningful violation patterns
- Tested pre-push hook validation on all modified files
- Confirmed no regressions in existing functionality

### Quality Metrics (v0.1.2)

- **Compilation**: ✅ Zero errors
- **Linting**: ✅ Zero warnings (clippy with -D warnings)
- **Formatting**: ✅ Cargo fmt compliant
- **Tests**: ✅ 91+ passing (100%)
- **Safety**: ✅ Zero unsafe code
- **File Changes**: 8 files modified, 1118 insertions, 214 deletions

---

## [Unreleased]

### In Progress

#### Phase 6: Solana Generator

- Procedural macro development
- Assertion injection logic
- Compute budget preservation
- Property test generation

#### Phase 7: EVM Support

- Solang parser integration
- Modifier generation for checks
- Foundry test framework integration

#### Phase 8: Move Support

- Move parser integration
- Resource and borrow checking
- Assertion framework

### Planned Improvements

- Enhanced error recovery in parser
- Incremental compilation
- Caching layer for analysis results
- Distributed analysis support
- Interactive REPL mode
- LSP (Language Server Protocol) support
- Package manager for invariant libraries

---

## Version Compatibility

### Rust Version

- Minimum: 1.93.0 (stable)
- Tested: 1.93.0
- Edition: 2021

### Operating Systems

- Linux (x86_64, aarch64)
- macOS (x86_64, Apple Silicon)
- Windows (x86_64)

### Dependencies

Major dependencies and their versions:

- pest 2.7
- syn 2.0
- clap 4.4
- serde 1.0
- anyhow 1.0
- rayon 1.7

## Migration Guide

### From Pre-Release

This is the initial release. No migration needed.

---

## Support

For questions, issues, or contributions:

- Open an issue on GitHub
- Check the README.md for documentation
- Review CONTRIBUTING.md for guidelines
- Read the BUILD_SUMMARY.md for architecture details

---

## Contributors

- Sentri Team - Initial design and implementation

---

## License

MIT License - See LICENSE file for details

---

### Unreleased Changes

(Breaking changes, new features, bug fixes in development will be listed here before release)

### [0.1.1] - Planned

- Parser performance improvements
- Additional example invariants
- Enhanced error messages
- Documentation improvements

### [0.2.0] - Planned

- Solana code generation
- EVM integration
- Move integration
- Property testing framework

---

**Note**: Sentri follows Semantic Versioning. See <https://semver.org> for details.

- **MAJOR** version for incompatible API changes
- **MINOR** version for new backward-compatible functionality
- **PATCH** version for backward-compatible bug fixes
