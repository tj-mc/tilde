# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.0] - 2025-09-21

### Changed
- **Function Definition**: `action` keyword renamed to `function`
- **Block Syntax**: Added `:block:` syntax for explicit scoping

### Added
- Matrix rain graphics example

### Fixed
- Various bug fixes and improvements

## [0.3.0] - 2025-01-20

### Added
- **Advanced List Operations**: New powerful list processing functions
  - **Predicate Functions**: `partition`, `group-by`, `sort-by` for complex data organization
  - **Set Operations**: `union`, `difference`, `intersection` for mathematical set operations
  - **Collection Functions**: `length`, `append` now available as stdlib functions
- **Enhanced Predicate Support**: All list functions now support both custom actions and stdlib predicates
  - Functions like `filter`, `find`, `every`, `some` can use `is-even`, `is-odd`, etc. directly
  - Improved function resolution system for seamless stdlib integration

### Enhanced
- **Function Call System**: Refactored how predicates and transformers are evaluated
  - Unified helper system for consistent function resolution
  - Better error messages for function parameter mismatches
  - Support for mixed stdlib and custom action usage

### Technical Improvements
- **Code Organization**: New modular stdlib structure with dedicated modules
  - `collection.rs` for basic collection operations
  - `set_operations.rs` for mathematical set operations
  - Enhanced `list.rs` with improved predicate handling
- **Test Coverage**: 55+ new tests for advanced list operations and set functions
- **Documentation**: Updated STDLIB.md with comprehensive examples and usage patterns

### Changed
- **Token Cleanup**: Removed hardcoded `Length` and `Append` tokens from lexer
  - These are now proper stdlib functions with full functionality
  - Better consistency with other stdlib functions

## [0.2.0] - 2025-01-20

### Added
- **Comprehensive Standard Library**: Complete functional programming toolkit
  - **List Operations**: `map`, `filter`, `reduce`, `sort`, `reverse`
  - **Helper Functions**: 20+ functional programming helpers
    - **Predicates**: `is-even`, `is-odd`, `is-positive`, `is-negative`, `is-zero`
    - **Transformations**: `double`, `triple`, `quadruple`, `half`, `square`, `increment`, `decrement`
    - **Reductions**: `add`, `multiply`, `max`, `min`
    - **String Operations**: `uppercase`, `lowercase`
    - **Math Functions**: `absolute`, `square-root`
  - **String Functions**: `join`, `split`, `trim`
  - **Math Operations**: `min`, `max`, `absolute`, `square-root`
- **Enhanced Variable Operations**:
  - `~var up amount` - Increment variables by specified amount
  - `~var down amount` - Decrement variables by specified amount
- **Collection Iteration**: `for-each` for processing lists with custom actions
- **Terminal Control**: `clear` function for screen clearing in animations/games
- **Negative Number Literals**: Direct support for negative numbers (e.g., `-42`, `-3.14`)
- **Advanced Examples**:
  - `stdlib_demo.tails` - Comprehensive standard library showcase
  - `functional_programming.tails` - Real-world functional programming patterns
  - `foreach_advanced.tails` - Advanced iteration techniques
  - Multiple graphics examples (bouncing ball, matrix rain, simple game)

### Enhanced
- **Function Reference System**: Functions can be passed by name directly
- **Parser Improvements**: Enhanced handling of function references
- **Error Messages**: More descriptive error messages for stdlib function usage
- **Code Organization**: Refactored stdlib with utility functions to reduce duplication

### Technical Improvements
- **Test Coverage**: 60+ new tests covering all stdlib functionality
- **Documentation**: Complete stdlib documentation with examples
- **Performance**: Optimized stdlib functions with shared validation utilities
- **Type Safety**: Enhanced argument validation for all stdlib functions

## [0.1.3] - 2025-01-19

### Added
- **File I/O Operations**: Complete file reading and writing functionality
  - `read "path"` - Read files with metadata (content, size, exists, error)
  - `write "path" content` - Write any data type to files with result status
  - Support for all data types in file operations
  - Comprehensive error handling and file metadata
- **Web REPL Improvements**: Version display now updates automatically

### Fixed
- Version consistency across all project files and web interface

## [0.1.0] - 2025-01-17

### Added
- Initial public release of Tails scripting language
- Core language features:
  - `~` prefix variable notation
  - String interpolation with backticks
  - Built-in HTTP operations (`get`, `wait`)
  - Object and list manipulation
  - Control flow (`if`/`else`, `loop`, `break-loop`)
  - Function definitions with `action` and calls with `*`
- Comprehensive standard library:
  - Mathematical operations (`+`, `-`, `*`, `/`, `\`, `%`)
  - Comparison operators (`<`, `>`, `<=`, `>=`, `==`, `!=`)
  - Object functions (`keys`, `values`, `has-key`, `length`)
  - List functions (`append`, `length`)
  - Interactive functions (`ask`, `say`)
- Multi-platform binary releases:
  - Linux x64
  - macOS (Intel & Apple Silicon)
  - Windows x64
- Web REPL at https://tj-mc.github.io/tails/
- WebAssembly support for browser execution
- Comprehensive documentation and examples
- Performance testing and benchmarking tools
- Automated CI/CD with GitHub Actions

### Technical Details
- Written in Rust with full type safety
- WASM-compatible architecture
- Comprehensive test suite (121+ tests)
- Memory-efficient interpreter design
- Cross-platform HTTP client support

[unreleased]: https://github.com/tj-mc/tails/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/tj-mc/tails/compare/v0.1.3...v0.2.0
[0.1.3]: https://github.com/tj-mc/tails/compare/v0.1.0...v0.1.3
[0.1.0]: https://github.com/tj-mc/tails/releases/tag/v0.1.0