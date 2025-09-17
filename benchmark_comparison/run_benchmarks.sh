#!/bin/bash

echo "=== Performance Benchmark: Tails vs Bun ==="
echo ""

# Build Tails in release mode for fair comparison
echo "Building Tails in release mode..."
cd ..
cargo build --release
cd benchmark_comparison

echo ""
echo "=== Running Tails Benchmark ==="
echo "Running 3 iterations..."
for i in {1..3}; do
    echo "Run $i:"
    time ../target/release/tails benchmark_final.tails
    echo ""
done

echo ""
echo "=== Running Bun Benchmark ==="
echo "Running 3 iterations..."
for i in {1..3}; do
    echo "Run $i:"
    time ~/.bun/bin/bun run benchmark_final.js
    echo ""
done

echo "=== Benchmark Complete ==="