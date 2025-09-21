#!/bin/bash

# Simple Performance Test - Just the essentials
echo "🚀 Performance Test: Tilde vs Bun"
echo "=================================="

# Build release (suppress warnings)
echo "📦 Building release..."
cargo build --release > /dev/null 2>&1

echo ""
echo "🧪 Testing Tilde (3 runs):"
for i in {1..3}; do
    echo -n "  Run $i: "
    time ./target/release/tilde benchmark_comparison/benchmark_final.tilde > /dev/null
done

echo ""
echo "🟡 Testing Bun (3 runs):"
if [ -f "$HOME/.bun/bin/bun" ]; then
    for i in {1..3}; do
        echo -n "  Run $i: "
        time ~/.bun/bin/bun run benchmark_comparison/benchmark_final.js > /dev/null
    done
else
    echo "❌ Bun not installed. Install with: curl -fsSL https://bun.sh/install | bash"
fi

echo ""
echo "✅ Test complete! Focus on the 'user' times above."
echo "   Typical results: Tilde ~200ms, Bun ~5ms"