# Performance Testing & Profiling Guide

This document covers all performance testing, benchmarking, and profiling tools available for the Tails interpreter.

## Overview

The Tails project has comprehensive performance testing infrastructure to:
- Track performance regressions
- Identify bottlenecks
- Measure optimization impact
- Compare against other runtimes

## Current Performance Baseline

**Benchmark**: Complex recursive computation (Fibonacci 18, Factorial 8, nested loops, 30 iterations)

- **Tails**: ~340ms average
- **Bun**: ~9ms average
- **Performance Gap**: 38x slower

**Primary Bottleneck**: 99.4% of execution time spent in evaluator (not parsing/lexing)

## Performance Testing Tools

### 1. Quick Comparison Benchmarks

#### Tails vs Bun Comparison
```bash
cd benchmark_comparison
./run_benchmarks.sh
```

**Purpose**: Direct performance comparison with industry-standard JavaScript runtime
**Files**:
- `benchmark_final.tails` - Tails benchmark
- `benchmark_final.js` - Equivalent Bun benchmark
- `run_benchmarks.sh` - Automated runner

### 2. Detailed Performance Profiling

#### Performance Bottleneck Analysis
```bash
cargo run --bin performance_main benchmark_comparison/profile_benchmark.tails
```

**Output Example**:
```
=== Performance Breakdown ===
evaluation                       104.95ms total (104.95ms avg × 1 calls)
tokenization                       0.24ms total (  0.24ms avg × 1 calls)
parsing                            0.17ms total (  0.17ms avg × 1 calls)
lexing                             0.14ms total (  0.14ms avg × 1 calls)
```

**Purpose**: Identifies which phase (lexing/parsing/evaluation) consumes the most time
**Implementation**: `tools/performance_main.rs` + `tools/performance_analysis.rs`

### 3. Production Benchmarking

#### Structured CSV Benchmarks (Recommended)
```bash
./tools/production_benchmarks.sh
cargo run --release --bin structured_benchmarks
```

**Purpose**: Git-friendly performance tracking over time
**Output**: `benchmark_results/benchmarks.csv`
**Format**: `timestamp,commit,test_name,iterations,avg_time_ns,ops_per_sec`

#### Memory Profiling
```bash
cargo run --release --bin memory_profiler
```

**Purpose**: Track memory allocation patterns and usage
**Output**: `benchmark_results/memory.csv`
**Format**: `timestamp,commit,test_name,duration_ns,final_allocated,peak_allocated,allocation_count,avg_allocation_size`

### 4. Development Benchmarks

#### Human-Readable Benchmarks
```bash
cargo run --release --bin benchmarks
```

**Purpose**: Quick performance check during development
**Output**: Human-readable text with timing information

#### Comprehensive Benchmark Suite
```bash
./tools/benchmark_runner.sh
```

**Purpose**: Full benchmark suite with system information
**Output**: `benchmark_results/bench_YYYYMMDD_HHMMSS_commit.txt`

### 5. Stress Testing
```bash
cargo run --release --bin stress_test
```

**Purpose**: Edge case testing and stability under load

## Performance Testing Workflow

### For New Features

1. **Capture Baseline**:
   ```bash
   ./tools/production_benchmarks.sh
   cp benchmark_results/benchmarks.csv benchmark_results/before_feature.csv
   ```

2. **Implement Feature**

3. **Measure Impact**:
   ```bash
   ./tools/production_benchmarks.sh
   diff benchmark_results/before_feature.csv benchmark_results/benchmarks.csv
   ```

4. **Acceptance Criteria**:
   - **No more than 10% performance regression** for core operations
   - All existing benchmarks must still pass

### For Performance Optimizations

1. **Identify Bottleneck**:
   ```bash
   cargo run --bin performance_main your_test_program.tails
   ```

2. **Implement Optimization**

3. **Measure Improvement**:
   ```bash
   # Before optimization
   time cargo run --release your_test_program.tails

   # After optimization
   time cargo run --release your_test_program.tails
   ```

4. **Validate No Regressions**:
   ```bash
   cargo test  # Ensure all tests pass
   ./tools/production_benchmarks.sh  # Ensure other benchmarks don't regress
   ```

## Key Performance Files

### Benchmark Programs
- `benchmark_comparison/benchmark_final.tails` - Main performance benchmark
- `benchmark_comparison/profile_benchmark.tails` - Smaller profiling benchmark
- `examples/fibonacci_action.tails` - Recursive algorithm test
- `examples/actions.tails` - Action call overhead test

### Performance Tools Source
- `tools/performance_main.rs` - Phase-level profiler (lexing/parsing/evaluation)
- `tools/performance_analysis.rs` - Performance measurement infrastructure
- `tools/benchmarks.rs` - General benchmarking suite
- `tools/structured_benchmarks.rs` - CSV output benchmarks
- `tools/memory_profiler.rs` - Memory allocation tracking
- `tools/stress_test.rs` - Stress testing suite

### Benchmark Scripts
- `benchmark_comparison/run_benchmarks.sh` - Tails vs Bun comparison
- `tools/production_benchmarks.sh` - Structured CSV benchmarks
- `tools/benchmark_runner.sh` - Comprehensive text benchmarks
- `tools/build_release.sh` - Release build script

## Interpreting Results

### Performance Profiling Output
- **evaluation**: Time spent executing the AST (main bottleneck)
- **parsing**: Time spent building AST from tokens
- **tokenization**: Time spent converting tokens
- **lexing**: Time spent creating lexer

### CSV Benchmark Fields
- `avg_time_ns`: Average execution time in nanoseconds
- `ops_per_sec`: Operations per second (higher is better)
- `final_allocated`: Memory usage at end of test
- `peak_allocated`: Maximum memory usage during test

### Performance Targets
- **Immediate**: 5-10x improvement (Target: ~50ms)
- **Medium-term**: 20-35x improvement (Target: ~10ms)
- **Long-term**: Match Bun performance (Target: ~5-9ms)

## Adding New Benchmarks

### Quick Test
Add to `tools/benchmarks.rs` in the `run_benchmarks()` function.

### Production Tracking
Add to `tools/structured_benchmarks.rs` for CSV tracking over time.

### Comparison Benchmark
Create equivalent `.tails` and `.js` files in `benchmark_comparison/`.

## Troubleshooting

### Slow Benchmarks
If benchmarks take too long:
1. Reduce iteration counts in benchmark programs
2. Use smaller input values (e.g., fibonacci(15) instead of fibonacci(25))
3. Use `profile_benchmark.tails` instead of `benchmark_final.tails`

### Memory Issues
If running out of memory:
1. Check `cargo run --release --bin memory_profiler`
2. Look for memory leaks in evaluator
3. Reduce benchmark complexity

### Inconsistent Results
1. Run benchmarks multiple times
2. Use release builds (`cargo build --release`)
3. Close other applications
4. Check system load with `top` or Activity Monitor

## Performance Optimization Principles

1. **Measure First**: Always profile before optimizing
2. **Syntax Preservation**: Zero changes to language syntax allowed
3. **Test Coverage**: All 73 existing tests must pass
4. **Incremental**: Make measurable improvements step by step
5. **Data-Driven**: Focus on actual bottlenecks, not theoretical ones