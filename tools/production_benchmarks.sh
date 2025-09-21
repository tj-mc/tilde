#!/bin/bash

# Production-Grade Tilde Language Benchmarks
# Creates structured data for performance tracking over time

echo "🚀 Production Tilde Language Benchmarks"
echo "========================================"
echo

# Ensure results directory exists
mkdir -p benchmark_results

# Build all benchmark binaries in release mode
echo "🔨 Building benchmark suite..."
cargo build --release --bin structured_benchmarks --quiet
cargo build --release --bin memory_profiler --quiet
cargo build --release --bin stress_test --quiet

echo "✅ Build complete"
echo

# Run structured benchmarks (CSV + JSON output)
echo "📊 Running performance benchmarks..."
./target/release/structured_benchmarks

echo
# Run memory profiling
echo "🧠 Running memory profiler..."
./target/release/memory_profiler

echo
# Quick stress test validation
echo "💥 Running stress tests..."
./target/release/stress_test | tail -5

echo
echo "📈 Performance History:"
echo "======================"

# Show recent performance trends if CSV exists
if [ -f "benchmark_results/benchmarks.csv" ]; then
    echo "Recent performance data (last 5 runs):"
    echo
    echo "Test Name                | Avg Time (μs) | Ops/Sec"
    echo "-------------------------|---------------|--------"
    tail -30 benchmark_results/benchmarks.csv | grep "lexer_simple" | tail -5 | \
    while IFS=, read timestamp commit test_name iterations avg_time_ns ops_per_sec; do
        avg_time_us=$(echo "scale=2; $avg_time_ns / 1000" | bc 2>/dev/null || echo "N/A")
        printf "%-24s | %13s | %7s\n" "$test_name" "$avg_time_us" "$ops_per_sec"
    done
    
    echo
    echo "📊 Run: git log --oneline | head -5    # to see recent commits"
    echo "📈 Run: python3 -c \"
import pandas as pd
df = pd.read_csv('benchmark_results/benchmarks.csv')
print('Performance trend for lexer_simple:')
print(df[df.test_name=='lexer_simple'].tail()[['commit','ops_per_sec']])
\"    # for trend analysis (if pandas available)"
else
    echo "No historical data yet. Run a few times to see trends!"
fi

echo
echo "✅ Benchmark complete!"
echo "📁 Results in: benchmark_results/"
echo "📊 CSV data:  benchmark_results/benchmarks.csv"
echo "🗂️  JSON data: benchmark_results/bench_*.json"