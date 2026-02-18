# Invar Architecture Overview

## System Architecture

```
┌─────────────────────────────────────────────────────┐
│                   CLI Interface                      │
│  (init, check, build, simulate, report, list)       │
└──────────────────┬──────────────────────────────────┘
                   │
        ┌──────────┴────────────┐
        │                       │
┌───────▼──────┐    ┌──────────▼──────┐
│ DSL Parser   │    │ Configuration   │
│ - Lexer      │    │ - TOML Parser   │
│ - Parser     │    │ - Schema Valid. │
│ - AST Build  │    │ - Environment   │
└───────┬──────┘    └──────────┬──────┘
        │                      │
        └──────────┬───────────┘
                   │
        ┌──────────▼────────────┐
        │   Core Engine         │
        │ ┌────────────────────┐│
        │ │ Type Checker       ││
        │ │ - Type Inference   ││
        │ │ - Constraint Check ││
        │ └────────┬───────────┘│
        │          │            │
        │ ┌────────▼────────────┐
        │ │ Evaluator          │
        │ │ - Expression Eval   │
        │ │ - State Simulation  │
        │ │ - Invariant Check   │
        │ └────────┬────────────┘
        │          │
        │ ┌────────▼────────────┐
        │ │ Threat Model        │
        │ │ - Injection Check    │
        │ │ - Tamper Detection   │
        │ │ - Sandbox Enforce    │
        │ └────────┬────────────┘
        │          │
        │ ┌────────▼────────────┐
        │ │ Security Validator  │
        │ │ - Issue Rating       │
        │ │ - Recommendation     │
        │ └────────────────────┘
        │
        └──────────┬────────────────────────┐
                   │                        │
        ┌──────────▼──────────┐  ┌─────────▼────────┐
        │ Chain Analyzers     │  │ Code Generators  │
        ├─────────────────────┤  ├──────────────────┤
        │ - Solana Analyzer   │  │ - Solana Gen      │
        │ - EVM Analyzer      │  │ - EVM Gen         │
        │ - Move Analyzer     │  │ - Move Gen        │
        └──────────┬──────────┘  └─────────┬────────┘
                   │                      │
        ┌──────────▼──────────────────────▼──────┐
        │           IR (Intermediate Rep)         │
        │  - Analyzer Results                    │
        │  - Generated Code                      │
        │  - Transformation Rules                │
        └──────────┬──────────────────────────────┘
                   │
        ┌──────────▼────────────┐
        │ Report Generator      │
        │ - JSON Formatter      │
        │ - Markdown Formatter  │
        │ - SARIF Formatter     │
        └─────────┬─────────────┘
                  │
        ┌─────────▼──────────────┐
        │  Output (stdout/file)  │
        │  - Structured Reports  │
        │  - Error Messages      │
        │  - Log Output          │
        └────────────────────────┘
```

## Module Structure

```
crates/
├── cli/                      # Command-line interface
│   └── src/
│       ├── main.rs          # Entry point, command parsing
│       ├── commands/        # Subcommand implementations
│       └── output/          # Output formatting
│
├── core/                     # Core analysis engine
│   └── src/
│       ├── lib.rs           # Module exports
│       ├── traits.rs        # Protocol definitions
│       ├── model.rs         # AST and types
│       ├── type_checker.rs  # Type inference & checking
│       ├── evaluator.rs     # Expression evaluation
│       ├── threat_model.rs  # Threat detection
│       ├── attack_patterns.rs # Known patterns
│       ├── security_validator.rs # Issue classification
│       └── error.rs         # Error types
│
├── dsl_parser/              # DSL parsing
│   └── src/
│       ├── lib.rs
│       ├── lexer.rs         # Tokenization
│       ├── parser.rs        # Token → AST
│       └── grammar.rs       # Grammar rules
│
├── ir/                       # Intermediate representation
│   └── src/
│       ├── lib.rs
│       ├── ast.rs           # Internal AST
│       └── analyzer_result.rs # Analysis output
│
├── analyzer/                # Chain-specific analysis
│   ├── solana/
│   ├── evm/
│   └── move/
│
├── generator/               # Code generation
│   ├── solana/
│   ├── evm/
│   ├── move/
│   └── solana_macro/        # Macro-based generation
│
├── report/                  # Report formatting
│   └── src/
│       ├── lib.rs
│       ├── formatter.rs     # Base formatter
│       └── report.rs        # Report structure
│
├── simulator/               # Simulation engine
│   └── src/
│       ├── lib.rs
│       ├── engine.rs        # Simulation loop
│
├── invariant_library/       # Built-in invariants
│   └── src/
│       ├── lib.rs
│       ├── library.rs       # Invariant definitions
│       └── loader.rs        # Loading system
│
└── utils/                   # Utilities
    └── src/
        ├── lib.rs
        ├── logging.rs       # Logging setup
        ├── path_utils.rs    # Path operations
        ├── version.rs       # Version info
        └── release.rs       # Release utilities
```

## Data Flow

### Analysis Workflow

```
User Input (CLI)
    ↓
[Config Parsing] → invar.toml
    ↓
[DSL Loading] → invariants.invar
    ↓
[DSL Parser] → AST
    ↓
[Type Checker] → Typed AST
    ↓
[Security Analysis] → Threat Model
    ↓
[Chain-Specific Analysis] → Analyzer Results
    ↓
[Evaluation Engine] → Invariant Results
    ↓
[Report Generation] → JSON/Markdown/SARIF
    ↓
Output (stdout/file)
```

### Component Interactions

```
┌─────────────┐
│    CLI      │ Coordinates user commands
└──────┬──────┘
       │
       ├──→ [Config Parser] ──→ Configuration
       │
       ├──→ [DSL Parser] ──→ AST
       │       ↓
       │    [Type Checker] ──→ Typed AST
       │       ↓
       │    [Threat Model] ──→ Issues
       │
       ├──→ [Chain Analyzer] ──→ Analysis Results
       │
       └──→ [Evaluator] ──→ Evaluation Results
               ↓
           [Report Generator] ──→ Output
```

## Key Design Decisions

### 1. **Chain Agnosticism**

Core engine is chain-agnostic. Chain adapters implement domain logic.

```rust
pub trait ChainAnalyzer {
    fn analyze(&self, program: &Program) -> Result<Analysis>;
}

pub struct SolanaAnalyzer { ... }
pub struct EvmAnalyzer { ... }
pub struct MoveAnalyzer { ... }
```

### 2. **Explicit Error Handling**

No panics in production paths. All errors are explicit types.

```rust
pub type Result<T> = std::result::Result<T, InvarError>;

pub enum InvarError {
    ParseError(String),
    TypeError(String),
    EvalError(String),
    ConfigError(String),
    // ... more variants
}
```

### 3. **Type Safety**

Complete type system validation before evaluation.

```
Input DSL
    ↓
Parse (syntax check)
    ↓
Type Check (semantic check)
    ↓
Evaluate (safe execution)
```

### 4. **Deterministic by Design**

- No global state
- Deterministic collections (BTreeMap, not HashMap)
- Reproducible seeds for randomness
- No platform-dependent behavior

### 5. **Layered Security**

Multiple security layers:
1. Type system prevents type confusion
2. Parser rejects invalid syntax
3. Sandboxed evaluation prevents escape
4. Threat model detects attacks
5. Report validator ensures output safety

## Critical Paths

### Zero-Panics Path

Core engine path must never panic:
- Parser returns `Err`
- Type checker returns `Err`
- Evaluator returns `Err`
- Never `unwrap()` or `panic!()`

### Type-Safe Path

Type system must prevent runtime type errors:
- Expression types inferred at parse time
- Type mismatches caught before evaluation
- All operations type-checked

### Deterministic Path

All operations must be deterministic:
- Same input → same output
- No randomness in results
- Consistent ordering

## Testing Strategy

Multi-layered testing validates each level:

```
Unit Tests        ← Test individual components
Property Tests    ← Test invariants hold
Integration Tests ← Test full workflows
CLI Tests         ← Test user interface
Security Tests    ← Test safety constraints
Fuzz Tests        ← Test robustness
```

## Performance Characteristics

### Expected Complexity

| Operation | Complexity | Notes |
|-----------|-----------|-------|
| Parse DSL | O(n) | Linear in DSL size |
| Type check | O(n) | Linear in expression tree |
| Evaluate | O(n) | Linear in iteration count |
| Full analysis | O(n·m) | n=program size, m=invariant complexity |

### Optimization Points

- Expression caching
- Parallel evaluation
- Early violation detection
- Lazy evaluation where safe

## Extension Points

### Adding New Chain

1. Implement `ChainAnalyzer` trait
2. Add analyzer to `analyzer/` crate
3. Add code generator to `generator/` crate
4. Register in CLI (`--chain` option)
5. Add tests

### Adding DSL Feature

1. Extend grammar in `dsl_parser/grammar.rs`
2. Update parser in `dsl_parser/parser.rs`
3. Add AST variants in `ir/ast.rs`
4. Implement evaluation in `core/evaluator.rs`
5. Add type checking rules in `core/type_checker.rs`
6. Add tests

### Adding Output Format

1. Implement report in `report/formatter.rs`
2. Register in CLI (`--format` option)
3. Add snapshot tests
4. Document in API

## Dependencies

### Core Dependencies

- `serde`/`serde_json` - Serialization
- `thiserror` - Error handling
- `tracing` - Structured logging
- `clap` - CLI argument parsing
- `toml` - Configuration format

### Optional Dependencies

- `solana-sdk` - Solana functionality
- `rayon` - Parallel processing

### Test Dependencies

- `proptest` - Property-based testing
- `assert_cmd` - CLI testing
- `insta` - Snapshot testing
- `criterion` - Benchmarking

## Thread Safety

- ✅ No global mutable state
- ✅ All types are `Send + Sync` where needed
- ✅ Message passing for parallelism
- ✅ `Arc<Mutex<T>>` for shared mutable state

## Future Extensibility

### Planned Additions

- [ ] Move to persistent caching
- [ ] Incremental analysis
- [ ] Machine learning for threat detection
- [ ] IDE integration (Language Server Protocol)
- [ ] Web interface

### Architecture Supports

- Plugin system (via dynamic loading)
- Custom chain adapters
- External parsers
- Remote evaluation
