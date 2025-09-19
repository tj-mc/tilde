# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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

[unreleased]: https://github.com/tj-mc/tails/compare/v0.1.3...HEAD
[0.1.3]: https://github.com/tj-mc/tails/compare/v0.1.0...v0.1.3
[0.1.0]: https://github.com/tj-mc/tails/releases/tag/v0.1.0