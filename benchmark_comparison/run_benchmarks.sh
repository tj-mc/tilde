#!/bin/bash

echo "=== Performance Benchmark: Tilde vs Bun ==="
echo ""

# Build Tilde in release mode for fair comparison
echo "Building Tilde in release mode..."
cd ..
cargo build --release
cd benchmark_comparison

echo ""
echo "=== Running Tilde Benchmark ==="
echo "Running 3 iterations..."
for i in {1..3}; do
    echo "Run $i:"
    time ../target/release/tilde benchmark_final.tilde
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