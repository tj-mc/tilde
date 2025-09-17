# Contributing to Tails

Thank you for your interest in contributing to Tails! This document provides guidelines and information for contributors.

## Ways to Contribute

- 🐛 **Report bugs** - Help us identify and fix issues
- 💡 **Suggest features** - Share ideas for language improvements
- 📝 **Improve documentation** - Help make Tails more accessible
- 🧪 **Write tests** - Increase code coverage and reliability
- 🔧 **Submit code changes** - Fix bugs or implement features
- 🌐 **Improve examples** - Create helpful sample scripts

## Getting Started

### Development Setup

1. **Clone the repository**
   ```bash
   git clone https://github.com/tj-mc/tails.git
   cd tails
   ```

2. **Install Rust** (if you haven't already)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

3. **Build and test**
   ```bash
   cargo build
   cargo test
   ```

4. **Try the REPL**
   ```bash
   cargo run
   ```

### Project Structure

- `src/lexer.rs` - Tokenizes input source code
- `src/parser.rs` - Builds abstract syntax tree
- `src/evaluator.rs` - Executes parsed code
- `src/ast.rs` - AST data structures
- `src/value.rs` - Runtime value types
- `examples/` - Sample Tails scripts
- `docs/` - Documentation files
- `tests/` - Integration tests

## Submitting Changes

### Before You Start

1. **Check existing issues** to avoid duplicate work
2. **Open an issue** for major changes to discuss the approach
3. **Fork the repository** and create a feature branch

### Pull Request Process

1. **Create a descriptive branch name**
   ```bash
   git checkout -b feature/add-string-functions
   git checkout -b fix/memory-leak-in-parser
   ```

2. **Write clear commit messages**
   ```bash
   git commit -m "Add string manipulation functions (substr, trim, split)"
   ```

3. **Add tests for your changes**
   - Unit tests in the same file as your code
   - Integration tests in `tests/` directory
   - Example scripts in `examples/` if relevant

4. **Ensure all tests pass**
   ```bash
   cargo test
   cargo check
   ```

5. **Update documentation** if needed
   - Update `docs/SYNTAX.md` for language changes
   - Add examples to demonstrate new features

6. **Submit your pull request**
   - Use a clear, descriptive title
   - Reference any related issues
   - Explain what changes you made and why

## Code Style

- Follow standard Rust conventions
- Use `cargo fmt` to format code
- Use `cargo clippy` to check for common issues
- Write clear, self-documenting code
- Add comments for complex logic

## Testing Guidelines

- **Unit tests**: Test individual functions and edge cases
- **Integration tests**: Test complete language features
- **Example scripts**: Demonstrate real-world usage
- **Error handling**: Test both success and failure cases

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test module
cargo test evaluator::tests

# Run with output
cargo test -- --nocapture
```

## Language Design Principles

When contributing to Tails, keep these principles in mind:

1. **Readability First** - Code should be self-explanatory
2. **Simplicity** - Prefer simple solutions over complex ones
3. **Consistency** - Follow existing patterns and conventions
4. **User Experience** - Consider how changes affect script writers
5. **Performance** - Maintain good performance characteristics

## Reporting Issues

### Bug Reports

Please include:
- Tails version (`tails --version`)
- Operating system and version
- Minimal example that reproduces the issue
- Expected vs actual behavior
- Error messages (if any)

### Feature Requests

Please include:
- Clear description of the proposed feature
- Use cases and examples
- How it fits with existing language features
- Any implementation ideas you have

## Version Management

To update the project version across all files:

```bash
# Update to new version (updates all files automatically)
./tools/update_version.sh 0.2.0

# Get current version
./tools/get_version.sh
```

This ensures version consistency across:
- `Cargo.toml` and `Cargo.lock`
- WebAssembly package configuration
- Web REPL interface
- GitHub Actions workflows
- Documentation

## Questions and Support

- **General questions**: Open a discussion on GitHub
- **Bug reports**: Create an issue with the bug template
- **Feature requests**: Create an issue with the feature template

## License

By contributing to Tails, you agree that your contributions will be licensed under the same MIT license that covers the project.

---

Thank you for helping make Tails better! 🐈‍⬛