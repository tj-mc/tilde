# `~tilde` 🐈‍⬛

**Interpreted Programming Language**

Tilde prioritizes **readability** and **simplicity** above all else. 

With simple `~variable` notation, english-like syntax, and powerful built-ins for modern tasks like HTTP requests and data processing, Tilde makes scripts that anyone can understand.

### Made for Humans

**New to scripting?** Tilde is intuitive and readable - anyone can start coding in 30 seconds! 

**Experienced developer?** Tilde won't get in your way. Write simple automation scripts without fighting the language.

## 🧶 See It In Action

### HTTP & Data Processing
```tilde
~user is get "https://api.github.com/users/octocat"
say "Found user: `~user.name` with `~user.public_repos` repos"
```

### Actions
```tilde
# Compose functionality with actions
action greet-user ~name ~age (
    if ~age >= 18 (
        say "Welcome `~name`! You can access all features."
    ) else (
        say "Hi `~name`! Parental guidance recommended."
    )
)

# Call them with *
*greet-user "Alice" 25
```

### Interactive Automation
```tilde
loop (
    ~username is ask "GitHub username (or 'quit'): "
    if ~username == "quit" break-loop

    ~info is get "https://api.github.com/users/`~username`"
    say "`~info.name` has `~info.public_repos` public repositories"
    wait 5
)
```

> 📖 **Want to learn more?** Check out the complete language guide: **[SYNTAX.md](docs/SYNTAX.md)**

## Install & Usage

### 🌐 Try in Browser

**Web REPL**: https://tj-mc.github.io/tilde/

The easiest way to try Tilde! No installation required - runs directly in your browser.

### 💻 Install Locally

**Quick Install:**
```bash
curl -sSL https://raw.githubusercontent.com/tj-mc/tilde/main/install.sh | bash
```

**Usage:**
```bash
tilde                 # Start REPL
tilde script.tilde    # Run a file
tilde --help          # Show help
```

### Download Binary
Download pre-built binaries from [Releases](https://github.com/tj-mc/tilde/releases) for:
- Linux x64
- macOS (Intel & Apple Silicon)
- Windows x64

## Examples & Learning

Explore more examples in the `examples/` folder:
- **`examples/actions.tilde`** - Functions and recursion
- **`examples/comprehensive_objects.tilde`** - Advanced object handling
- **`examples/http_get.tilde`** - Real HTTP requests
- **`examples/fibonacci.tilde`** - Loops and mathematical sequences

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
cargo run --bin performance_main your_program.tilde

# Track performance over time
./tools/production_benchmarks.sh
```
---

## 📚 Ready to Learn More?

**[📖 Complete Language Reference → SYNTAX.md](docs/SYNTAX.md)**

Discover all of Tilde' features including:
- **Standard Library** (`map`, `filter`, `reduce`) for functional programming
- Advanced object manipulation and property access
- Comprehensive control flow (`loop`, `if`/`else`, `break-loop`)
- HTTP operations (`get`, `wait`) and error handling
- Function definitions with `action` and calling with `*`
- String interpolation patterns and data types
- And much more!

**📚 [Standard Library Documentation → STDLIB.md](docs/STDLIB.md)**
