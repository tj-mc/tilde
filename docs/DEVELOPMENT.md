# Development
## Project Overview

This is a Rust project implementing the Tails scripting language - a simple, readable language. The interpreter provides a full REPL experience with multiline support and file execution.

## Design Philosophy

Our design priorities (in order):
1. **Readability**: Code should be self-documenting and intuitive to read
2. **Simplicity**: Minimal syntax with an easy mental model
3. **Performance**: Fast enough for practical use, but not at the expense of the first two

## Technical Constraints

- **Zero Dependencies Policy**: Pure Rust implementation with no external crates
- **Memory Safety**: Leverages Rust's ownership system for safe memory management
- **Error Handling**: Comprehensive error reporting with clear messages

## Development Commands

### Build and Run
- `cargo build` - Build the project
- `cargo run` - Run the main program
- `cargo test` - Run tests (61 tests implemented, all passing)

### Code Quality
- `cargo fmt` - Format code according to Rust standards
- `cargo clippy` - Run the Rust linter for code improvements
- **Zero warnings policy**: All compiler warnings must be fixed. Warnings are not acceptable.

### Performance Analysis (Zero Dependencies)
- `cargo build --release` - Optimized builds for performance testing
- `RUSTFLAGS=-g cargo build --release` - Release with debug symbols for profiling
- `time cargo run large_program.tails` - Basic execution timing
- Native OS tools: `time`, `top`, Activity Monitor for system profiling

### Performance Testing & Benchmarking

**📊 See [PERFORMANCE_TESTING.md](PERFORMANCE_TESTING.md) for comprehensive performance testing guide**

#### Quick Commands:
- `./tools/perf_test.sh` - **FASTEST**: Quick Tails vs Bun comparison with auto-build
- `cargo run --bin performance_main <file.tails>` - **NEW**: Identify bottlenecks (lexing vs parsing vs evaluation)
- `./tools/production_benchmarks.sh` - **TRACKING**: CSV benchmarks for tracking over time
- `cd benchmark_comparison && ./run_benchmarks.sh` - Full automated comparison with detailed analysis

#### Current Performance Status:
- **Tails**: ~200ms (⬇️ was ~340ms) - **1.6x improvement achieved**
- **Bun**: ~6ms (same benchmark)
- **Performance Gap**: 33x slower (was 57x slower)
- **Bottleneck**: 99.4% of time spent in evaluator (not parsing/lexing)
- **Next Target**: 5-10x additional improvement maintaining identical syntax

## Architecture

The interpreter is organized into clean, testable modules:

### Core Modules (src/)
- **lib.rs**: Module declarations
- **main.rs**: REPL interface
- **lexer.rs**: Tokenization (converts text → tokens)
- **parser.rs**: AST generation (converts tokens → syntax tree)
- **ast.rs**: Abstract Syntax Tree type definitions
- **evaluator.rs**: Execution engine (evaluates AST)
- **value.rs**: Runtime value types (Number, String, Boolean, List, Object, Null)

### Testing Structure
- **Unit tests**: Each module contains `#[cfg(test)]` blocks with focused tests
- **Integration tests**: `tests/integration_test.rs` for end-to-end testing
- Run all tests: `cargo test`
- Run specific test: `cargo test test_name`

### Reference Files
- **syntax.txt**: Complete language specification
- **plan.txt**: Implementation roadmap

### Procedures (procedures/)
- **new-feature.md**: Standard procedure for adding new language features to Tails

## Current Implementation Status

### ✅ Fully Implemented Features
- **Variables**: Assignment (`~var is value`) and retrieval with prefix notation
- **Data Types**: Numbers (f64), Strings (both single and double quotes), Booleans, Lists, Objects, Null
- **Arithmetic**: All operations - Addition (`+`), subtraction (`-`), multiplication (`*`), division (`/`), string concatenation
- **Comparisons**: All operators (`<`, `<=`, `>`, `>=`, `==`, `!=`) with mixed-type support
- **Control Flow**: If/else-if/else chains, loop constructs with break-loop termination
- **Objects**: Complete object system with creation (`{key: value}`), property access (`~obj.property`), nested assignment
- **I/O Functions**: `say` (multi-argument output), `ask` (input with auto-typing)
- **Web Operations**: `get` and `wait` statements with HTTP requests via ureq dependency
- **REPL**: Interactive shell with multiline mode, file execution, error handling
- **Expressions**: Parenthesized expressions, variable references, function calls, object literals


## Implementation Plan

See `./PLAN.md` for the detailed implementation roadmap.