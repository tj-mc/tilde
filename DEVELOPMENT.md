## Development

### Build from Source
```bash
git clone https://github.com/tj-mc/tilde.git
cd tilde
cargo build --release
./target/release/tilde
```

### Run Tests
```bash
cargo test
```

### Web REPL Development

**One-command setup:**
```bash
./web-dev.sh    # Installs dependencies, builds WASM, starts server
```

**Manual steps:**
```bash
./build-web.sh  # Build WebAssembly package
./serve-web.sh  # Start development server
```

See [web/README.md](web/README.md) for detailed documentation.

### Create Release Builds
```bash
./tools/build_release.sh
```

### Project Structure
- `src/lexer.rs` - Tokenizes input
- `src/parser.rs` - Builds syntax tree
- `src/evaluator.rs` - Executes code
- `src/ast.rs` - Data structures
- `src/value.rs` - Runtime values

## Performance Testing

For comprehensive performance testing, benchmarking, and profiling documentation, see:

📊 **[PERFORMANCE_TESTING.md](docs/PERFORMANCE_TESTING.md)**

### Quick Performance Commands:
```bash
# Compare Tilde vs Bun performance
cd benchmark_comparison && ./run_benchmarks.sh

# Identify bottlenecks (lexing vs parsing vs evaluation)
cargo run --bin performance_main your_program.tde

# Track performance over time
./tools/production_benchmarks.sh
```
---
