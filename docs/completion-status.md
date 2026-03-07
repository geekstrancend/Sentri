# Sentri Crate Completion Status

**Last Updated**: March 5, 2026  
**Analysis Method**: Source LOC audit + feature completeness

---

## Executive Summary

| Category | Status | LOC | Priority |
|----------|--------|-----|----------|
| **CLI** | ⚠️ Partial | 531 | P1 |
| **Core** | ⚠️ Partial | 35+ | P1 |
| **DSL Parser** | 🔴 Stub | ~10 | **P0** |
| **IR** | 🔴 Stub | ~19 | P1 |
| **Analyzers** (EVM/Solana/Move) | 🔴 Stub | 8 ea | **P1** |
| **Generators** | 🔴 Stub | 8 ea | **P2** |
| **Simulator** | 🔴 Stub | 8 | **P2** |
| **Invariant Library** | 🔴 Stub | 10 | P2 |
| **Report** | 🔴 Stub | 10 | **P1** |
| **Utils** | 🔴 Stub | 13 | P1 |

**Legend**: 🟢 Complete | ⚠️ Partial | 🔴 Stub (< 30 LOC)

---

## Crate-by-Crate Analysis

### Core (`crates/core/`)

**Status**: ⚠️ Partial  
**LOC**: 35+ (lib.rs only; many re-exported modules)

**Implemented**:
- Public module structure and re-exports
- Type definitions (Type, TypeError, TypedValue, etc.)
- Core traits framework (ChainAnalyzer, CodeGenerator, Simulator)

**Stubs/Incomplete**:
- Module files exist but content not verified
- Actual implementations in evaluator.rs, security_validator.rs, etc. likely incomplete
- No end-to-end flow implemented

**Impact**: 🔴 Blocking. Core engine isn't functional yet.

---

### CLI (`crates/cli/src/main.rs`)

**Status**: ⚠️ Partial  
**LOC**: 531

**Implemented**:
- Clap-based CLI argument parsing
- Command structure: init, build, simulate, upgrade-check, report, list
- Basic command routing to handler functions
- Project initialization with scaffolding
- Mock implementations of some handlers (init_project, etc.)

**Stubs/Incomplete**:
- `build_invariants()` calls SecurityValidator but no actual analysis
- `simulate_program()` not implemented
- `check_upgrade()` not implemented
- `generate_report()` not implemented
- `list_invariants()` not implemented
- No `check` command (critical for Phase 1 verification)
- No `watch` command (critical for Phase 3 daemon mode)
- No config file parsing (`--config` flag exists but unused)

**Impact**: 🔴 Blocking. CLI handlers are stubs; core functions don't work.

---

### DSL Parser (`crates/dsl_parser/`)

**Status**: 🔴 Stub  
**LOC**: ~10

**Modules**:
- `grammar.rs` (exists, not examined)
- `lexer.rs` (exists, not examined)
- `parser.rs` (exists, not examined)
- Public API: `parse_invariant()`, `InvariantParser`

**Notes**: Module structure is in place but file contents not verified. Likely incomplete given small lib.rs.

**Impact**: 🔴 Blocking. Cannot parse `.invar` files.

---

### Analyzers (`crates/analyzer/{solana,evm,move}/`)

**Status**: 🔴 Stub  
**LOC**: ~8 each

- All three chain analyzers are nearly empty stubs
- No RPC adapters implemented
- No state fetching logic

**Impact**: 🔴 Critical Blocking. No chain integration at all.

**Required (Phase 2)**:
- `invar-analyzer-evm`: JSON-RPC via ethers-rs/alloy
- `invar-analyzer-solana`: solana-client integration
- `invar-analyzer-move`: Move VM integration (if prioritized)

---

### Generators (`crates/generator/{solana,evm,move}/`)

**Status**: 🔴 Stub  
**LOC**: ~8 each (except `solana_macro` at 218 LOC)

- All base generators are minimal stubs
- `solana_macro` (218 LOC) is the only substantial implementation

**Impact**: 🟡 High Priority but deferred to Phase 2.

---

### IR (`crates/ir/`)

**Status**: 🔴 Stub  
**LOC**: ~19

**Modules**:
- `ast.rs` (exists)
- `analyzer_result.rs` (exists)
- `lib.rs` (minimal exports)

**Impact**: 🔴 Blocking. IR is the internal representation format; needed for all downstream processing.

---

### Report (`crates/report/`)

**Status**: 🔴 Stub  
**LOC**: ~10

**Modules**:
- `formatter.rs` (exists)
- `report.rs` (exists)
- `lib.rs` (minimal)

**Impact**: 🟡 High Priority for Phase 9 (documentation & output formatting).

---

### Simulator (`crates/simulator/`)

**Status**: 🔴 Stub  
**LOC**: ~8

**Impact**: 🟡 Medium Priority (Phase 2 optimization after core works).

---

### Invariant Library (`crates/invariant_library/`)

**Status**: 🔴 Stub  
**LOC**: ~10

**Modules**:
- `library.rs` (1170 LOC - likely has actual content!)
- `loader.rs` (exists)

**Notes**: The library.rs file is substantial; worthy of deeper investigation.

**Impact**: 🟡 Medium Priority (defines reusable invariant patterns).

---

### Utils (`crates/utils/`)

**Status**: 🔴 Stub  
**LOC**: ~13

**Modules**:
- `logging.rs`, `path_utils.rs`, `release.rs`, `version.rs`

**Impact**: 🟡 Low Priority (utility crates).

---

## Critical Path to MVP (Phase 0–3 Focus)

### Phase 0: ✅ COMPLETE
- Deleted repo artifacts (push.sh, push_output.txt)
- Fixed .gitignore to NOT ignore Cargo.lock
- Updated all fork URLs to hackdex-max
- Declared MSRV 1.70

### Phase 1: Core Architecture (In Progress)
- [ ] Implement invariant definition parser (Phase 6 priority)
- [ ] Implement invariant evaluator (stub analysis below)
- [ ] Implement chain state fetcher (requires Phase 2 adapters)
- [ ] Verify `sentri check --config examples/invar.toml.example --once` works

### Phase 2: Multi-Chain Adapters (Not Started)
- [ ] EVM adapter (ethers-rs JSON-RPC, eth_subscribe, storage access)
- [ ] Solana adapter (solana-client, WebSocket subscriptions)
- [ ] Cosmos adapter (optional, deferred if needed)

### Phase 3: Watcher Daemon (Not Started)
- [ ] `sentri watch` long-running mode
- [ ] Block subscription loop
- [ ] Alert integration
- [ ] Graceful shutdown (SIGTERM)

### Phase 4–5: Observability & Alerting (Not Started)
- [ ] Tracing integration
- [ ] Prometheus metrics
- [ ] Webhook sinks
- [ ] Slack integration

---

## Recommended Immediate Actions

1. **Phase 6 First** (before Phase 1 completion):
   - Design and implement `invar.toml` config format
   - Implement config parser with env var substitution
   - Add `sentri validate-config` subcommand
   - This unblocks testing of all downstream components

2. **Phase 1 Core Path** (after config):
   - Implement `sentri check` command end-to-end
   - Mock local node if real RPCs unavailable
   - Get ONE working example (ERC-20 supply check on test network)

3. **Phase 8 CI/CD** (parallel with code):
   - Add GitHub Actions release workflow
   - Cross-platform binary builds
   - Automated testing

---

## Known Gaps

| Gap | Impact | Recovery |
|-----|--------|----------|
| No config file parsing | **Critical** | Phase 6 task |
| Chain adapters are stubs | **Critical** | Phase 2 task |
| No alert integration | High | Phase 5 task |
| No structured logging | High | Phase 4 task |
| Minimal test coverage | High | Phase 7 task |
| No metrics exported | Medium | Phase 4 task |

---

## Notes for Implementation Team

1. **Order matters**: Config → Evaluator → Chain Adapters → Watcher → Observability.
2. **Module imports are correct** but implementations are missing.
3. **CLI structure is sound** – just needs command implementations.
4. **Start with mocks**: E.g., mock RPC responses, test data, before integrating real chains.
5. **Use `examples/invar.toml.example`** as the north star for all config work.

---

**Generated by Phase 1 audit. Next phase: Phase 6 (Configuration).**
