# Performance Benchmarking Guide

## Overview

Performance benchmarks measure how Invar scales with respect to:
- **Input size**: How larger programs affect analysis time
- **DSL complexity**: How complex invariants affect parsing/evaluation
- **Memory usage**: Peak and sustained memory consumption
- **Parallel scalability**: How well multi-threaded evaluation works

## Benchmark Structure

```
benches/
├── benchmarks.rs          # Main benchmark suite
├── parser_bench.rs        # Parser performance
├── evaluator_bench.rs     # Evaluator performance
└── memory_bench.rs        # Memory profiling
```

## Running Benchmarks

### Basic Benchmark Run

```bash
cargo bench
```

Creates `target/criterion/` directory with HTML reports.

### Specific Benchmark

```bash
cargo bench -- bench_parser
```

### With Baseline Comparison

```bash
cargo bench -- --baseline main
```

Compares against previously saved baseline.

### Extended Run for Stability

```bash
CRITERION_MEASUREMENT_TIME=10 cargo bench
```

Runs each benchmark for longer (better accuracy).

## Benchmark Categories

### 1. Parser Performance

```rust
fn bench_parser(c: &mut Criterion) {
    c.bench_function("parse_simple_invariant", |b| {
        let input = r#"
invariant: simple_test
forall x in items:
    x > 0
"#;
        b.iter(|| {
            let lexer = Lexer::new(black_box(input));
            // Measure parsing time
        });
    });

    c.bench_function("parse_complex_invariant", |b| {
        let input = r#"
context { state: SystemState, chain: Solana }
invariant: complex_test
forall tx in state.transactions:
    (tx.amount > 0 && tx.fee >= MIN_FEE) ||
    (tx.priority == HIGH && tx.fee >= MIN_PRIORITY_FEE)
global:
    sum(state.balances) == state.total &&
    max(state.balance) <= MAX_ALLOWED
"#;
        b.iter(|| {
            let lexer = Lexer::new(black_box(input));
        });
    });
}
```

### 2. Type Checking Performance

```rust
fn bench_type_checker(c: &mut Criterion) {
    c.bench_function("type_check_simple", |b| {
        let input = "x > 0 && y < 100";
        b.iter(|| {
            let mut checker = TypeChecker::new();
            let _ = checker.check_expr(black_box(input));
        });
    });

    let mut group = c.benchmark_group("type_check_depth");
    for depth in [1, 5, 10, 20].iter() {
        let input = (0..*depth)
            .map(|i| format!("x{} > 0", i))
            .collect::<Vec<_>>()
            .join(" && ");
        
        group.bench_with_input(
            BenchmarkId::from_parameter(depth), 
            depth, 
            |b, _| {
                b.iter(|| {
                    let mut checker = TypeChecker::new();
                    let _ = checker.check_expr(black_box(&input));
                });
            }
        );
    }
    group.finish();
}
```

### 3. Expression Evaluation

```rust
fn bench_evaluator(c: &mut Criterion) {
    c.bench_function("eval_literal", |b| {
        b.iter(|| {
            let evaluator = Evaluator::new();
            let _ = evaluator.eval(black_box("42"));
        });
    });

    c.bench_function("eval_arithmetic", |b| {
        b.iter(|| {
            let evaluator = Evaluator::new();
            let _ = evaluator.eval(black_box("2 + 3 * 4 - 1"));
        });
    });

    // Scaling test
    let mut group = c.benchmark_group("eval_expression_length");
    for len in [10, 50, 100, 500].iter() {
        let input = (0..*len)
            .map(|i| format!("x{}", i))
            .collect::<Vec<_>>()
            .join(" + ");
        
        group.bench_with_input(
            BenchmarkId::from_parameter(len),
            len,
            |b, _| {
                b.iter(|| {
                    let evaluator = Evaluator::new();
                    let _ = evaluator.eval(black_box(&input));
                });
            }
        );
    }
    group.finish();
}
```

## Performance Targets

| Operation | Target | Current |
|-----------|--------|---------|
| Parse simple invariant | < 1ms | TBD |
| Type check expression | < 0.5ms | TBD |
| Evaluate expression | < 0.1ms | TBD |
| Full analysis (1MB program) | < 5s | TBD |
| Memory for 1MB analysis | < 100MB | TBD |

## Scaling Characteristics

### Linear Scaling (Good)
```
Time/Memory
      ^
      │     ╱
      │   ╱
      │ ╱
      └─────────────► Input Size
```

Characteristic: Time ∝ Input Size

### Quadratic Scaling (Acceptable)
```
Time
      ^
      │      ╱╱
      │    ╱╱
      │  ╱╱
      └────────────► Input Size
```

Acceptable if constant is small.

### Exponential Scaling (Bad)
```
Time
      ^
      │        ╱
      │      ╱
      │    ╱
      └────────────► Input Size
```

Must be optimized or limited.

## Memory Profiling

### Heap Allocations

```bash
# On MacOS with detailed profiling
CARGO_PROFILE_RELEASE_DEBUG=true \
cargo bench --bench benchmarks

# Use Valgrind on Linux
valgrind --tool=massif ./target/release/invar
```

### Memory Targets

- **Base overhead**: < 1MB
- **Per-invariant**: < 10KB
- **Peak during analysis**: < 500MB (for 1MB program)

## Continuous Benchmarking

### Store Baseline

```bash
cargo bench -- --save-baseline main
```

### Compare Against Baseline

```bash
cargo bench -- --baseline main
```

### CI Integration

```yaml
benchmark:
  name: Benchmark Smoke Test
  steps:
    - run: cargo bench --no-run
```

Smoke test ensures benchmarks compile and basic functionality works.

## Interpreting Results

### Criterion Output

```
parse_simple_invariant          time:   [485.23 us 487.45 us 489.78 us]
parse_complex_invariant         time:   [2.1234 ms 2.1445 ms 2.1687 ms]
type_check_simple              time:   [125.34 us 126.78 us 128.23 us]
```

### Performance Regression Detection

Changes > 5% trigger investigation:

```bash
# This will show detailed comparison
cargo bench -- --verbose --baseline main
```

## Optimization Guide

### 1. Identify Bottleneck

```bash
cargo bench -- --verbose 2>&1 | grep -A5 "regressed"
```

### 2. Profile the Code

```bash
# Generate flamegraph
cargo install flamegraph
cargo flamegraph --bin invar -- check test.invar

# View with Firefox
firefox flamegraph.svg
```

### 3. Optimize and Re-benchmark

```bash
# Make changes
cargo bench -- --baseline main
```

### 4. Common Optimizations

**Avoid allocations:**
```rust
// ❌ Creates vector on every call
fn process(items: Vec<Item>) { ... }

// ✅ Reuse buffers
fn process(items: &[Item]) { ... }
```

**Use iteration instead of indexing:**
```rust
// ❌ Slower with bounds checks
for i in 0..vec.len() {
    println!("{}", vec[i]);
}

// ✅ Faster, no bounds checks
for item in &vec {
    println!("{}", item);
}
```

**Prefer references:**
```rust
// ❌ Clones strings
fn parse(input: String) -> Result { ... }

// ✅ Borrows strings
fn parse(input: &str) -> Result { ... }
```

## Benchmark Examples

### Scaling Test Pattern

```rust
let mut group = c.benchmark_group("invarient_count_scaling");

group.sample_size(20);  // Reduce samples for long-running test
group.measurement_time(Duration::from_secs(30));

for num_invariants in [1, 10, 50, 100, 500].iter() {
    let input = create_program_with_invariants(*num_invariants);
    
    group.bench_with_input(
        BenchmarkId::from_parameter(num_invariants),
        num_invariants,
        |b, _| {
            b.iter(|| {
                analyze(black_box(&input))
            });
        }
    );
}
group.finish();
```

### Memory Pressure Test

```rust
c.bench_function("memory_large_project", |b| {
    let large_input = create_1mb_program();
    b.iter(|| {
        analyze(black_box(&large_input))
    });
});
```

## Performance Regression Prevention

### 1. Benchmark Before Optimization

```bash
cargo bench -- --save-baseline before_optimization
```

### 2. Make Changes

Edit code to improve performance.

### 3. Benchmark After

```bash
cargo bench -- --baseline before_optimization
```

### 4. Verify Improvement

Should show percentage improvement.

## Hardware Specifications

Benchmarks should note hardware:

```rust
/// Benchmark assumptions:
/// - CPU: Intel Xeon (2.3 GHz)
/// - RAM: 16GB DDR4
/// - Disk: SSD
/// - OS: Linux 5.15
```

When hardware changes significantly, re-baseline.

## Monthly Review

1. Check for regressions
2. Review new benchmarks
3. Update performance targets
4. Document any optimizations
5. File issues for slowdowns > 10%

## Tools

- **Criterion**: Primary benchmarking framework
- **Flamegraph**: CPU profiling
- **Valgrind/Heaptrack**: Memory profiling
- **Perf**: Linux performance analysis

## Example Report

```
Benchmarking parser/simple: Collecting 100 samples
parser/simple                   time:   [485.23 us 487.45 us 489.78 us]
                        change: [+2.3% +3.1% +4.0%] (p = 0.00 < 0.05)
                        performance has regressed.

Recommendations:
- Investigate new allocations
- Profile with flamegraph
- Compare with baseline implementation
```
