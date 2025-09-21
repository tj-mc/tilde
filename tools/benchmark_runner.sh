#!/bin/bash

# Tilde Language Benchmark Runner
# Runs benchmarks and commits results to performance log

echo "🚀 Running Tilde Language Performance Tests"
echo "============================================="
echo

# Get system info
RUST_VERSION=$(rustc --version)
OS_INFO=$(uname -a)
DATE=$(date)
COMMIT_HASH=$(git rev-parse --short HEAD 2>/dev/null || echo "no-git")

# Create results directory
mkdir -p benchmark_results

# Generate result filename
RESULT_FILE="benchmark_results/bench_$(date +%Y%m%d_%H%M%S)_${COMMIT_HASH}.txt"

# Run benchmarks and capture output
echo "📊 Running benchmarks..."
echo "Date: $DATE" > "$RESULT_FILE"
echo "Commit: $COMMIT_HASH" >> "$RESULT_FILE"
echo "Rust: $RUST_VERSION" >> "$RESULT_FILE"
echo "System: $OS_INFO" >> "$RESULT_FILE"
echo "======================================" >> "$RESULT_FILE"
echo >> "$RESULT_FILE"

# Build in release mode for accurate performance testing
echo "🔨 Building in release mode..."
cargo build --release --bin benchmarks --quiet
cargo build --release --bin stress_test --quiet

# Run main benchmarks
echo "⚡ Running performance benchmarks..."
echo "PERFORMANCE BENCHMARKS:" >> "$RESULT_FILE"
echo "-----------------------" >> "$RESULT_FILE"
./target/release/benchmarks >> "$RESULT_FILE" 2>&1

echo >> "$RESULT_FILE"
echo "STRESS TESTS:" >> "$RESULT_FILE" 
echo "-------------" >> "$RESULT_FILE"
./target/release/stress_test >> "$RESULT_FILE" 2>&1

# Show results
echo
echo "✅ Benchmark complete! Results saved to: $RESULT_FILE"
echo
echo "📈 Latest Results Summary:"
echo "=========================="
tail -n 20 "$RESULT_FILE"

# Optional: Add to git if in a git repo
if git status >/dev/null 2>&1; then
    echo
    echo "📝 Adding results to git..."
    git add "$RESULT_FILE"
    echo "Results staged for commit. Run 'git commit' to save."
fi

echo
echo "🔍 View full results: cat $RESULT_FILE"
echo "📊 Compare with previous: diff benchmark_results/bench_*.txt"