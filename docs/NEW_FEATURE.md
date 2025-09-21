# Adding a New Language Feature to Tilde

This document outlines the standard procedure for adding new features to the Tilde programming language.

## Overview

Adding a new feature requires modifications across multiple layers of the interpreter. Follow this systematic approach to ensure consistency and maintainability.

> Important: To avoid every-growing files, try to make self-contained features as individual modules

## Step-by-Step Procedure

### 1. Define the Feature Specification

Before implementation, clearly define:
- **Syntax**: How users will write the feature
- **Semantics**: What the feature does
- **Edge cases**: How it handles errors or special conditions
- **Examples**: Clear usage examples

Document this in `SYNTAX.md` under the appropriate section.

### 2. Update the Lexer (`src/lexer.rs`)

#### 2.1 Add Token Type
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // ... existing tokens
    YourNewToken,
}
```

#### 2.2 Add Keyword Recognition
In the `read_word()` method, add your keyword:
```rust
match word.as_str() {
    // ... existing keywords
    "yourfeature" => Token::YourNewToken,
    // ...
}
```

### 3. Update the AST (`src/ast.rs`)

Add the appropriate AST node:
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    // ... existing statements
    YourFeature(Vec<Expression>), // or appropriate structure
}
```

### 4. Update the Parser (`src/parser.rs`)

#### 4.1 Add Parsing Logic
In `parse_statement()`, add a case for your token:
```rust
Token::YourNewToken => {
    self.advance();
    // Parse arguments or parameters
    let args = self.parse_your_feature_args()?;
    Ok(Statement::YourFeature(args))
}
```

#### 4.2 Update Terminator Checks
Ensure your token is included in statement terminator checks:
```rust
while !matches!(self.current_token(), 
    Token::Eof | Token::Newline | Token::YourNewToken | /* other tokens */) {
    // parsing logic
}
```

### 5. Implement Evaluation (`src/evaluator.rs`)

Add the execution logic:
```rust
Statement::YourFeature(args) => {
    // Evaluate arguments
    let evaluated_args = /* evaluate args */;
    
    // Perform the feature's action
    // Return appropriate Value
    Ok(Value::String("result".to_string()))
}
```

### 6. Write Tests

#### 6.1 Unit Tests
Add tests in the relevant module:
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_your_feature() {
        let mut parser = Parser::new("yourfeature args");
        let program = parser.parse().unwrap();
        // Assert expected AST structure
    }
}
```

#### 6.2 Integration Tests
Create integration tests in `tests/`:
```rust
#[test]
fn test_your_feature_integration() {
    let input = "yourfeature test";
    // Test end-to-end functionality
}
```

### 7. Update Documentation

1. **SYNTAX.md**: Add complete syntax documentation
2. **README.md**: Update if it's a major feature
3. **Examples**: Create example programs demonstrating the feature

### 8. Test and Validate

Run the complete test suite:
```bash
cargo test
cargo fmt
cargo clippy
```

Test the feature manually in the REPL:
```bash
cargo run
```

### 9. Performance Impact Analysis

Before finalizing the feature, assess its performance impact:

#### 9.1 Run Baseline Benchmarks
```bash
# Get current performance baseline
./tools/production_benchmarks.sh
cp benchmark_results/benchmarks.csv benchmark_results/before_feature.csv
```

#### 9.2 Test Memory Impact
```bash
# Check memory usage patterns
cargo run --release --bin memory_profiler
cp benchmark_results/memory.csv benchmark_results/memory_before_feature.csv
```

#### 9.3 Post-Implementation Validation
After implementing your feature:
```bash
# Run benchmarks again
./tools/production_benchmarks.sh

# Compare performance impact
diff benchmark_results/before_feature.csv benchmark_results/benchmarks.csv

# Check memory impact
cargo run --release --bin memory_profiler
diff benchmark_results/memory_before_feature.csv benchmark_results/memory.csv
```

#### 9.4 Performance Regression Policy
- **No more than 10% performance regression** for core operations (lexing, parsing, simple evaluation)
- **Memory usage increases should be justified** - document why additional allocations are necessary  
- **New features should not slow down existing features** - focus optimization on the new code paths
- If performance impact is significant, consider:
  - Lazy evaluation strategies
  - Caching frequently used results
  - More efficient data structures
  - Code path optimization

#### 9.5 Benchmark New Features
If your feature adds significant new functionality:
```bash
# Add specific benchmarks for your feature in tools/structured_benchmarks.rs
# Example: Add a case for your new control structure, function, etc.
```

## Example: Adding the `ask` Command

Here's how the `ask` command was added following this procedure:

### 1. Specification
- **Purpose**: Get user input from stdin
- **Syntax**: `ask [prompt_expression...]`
- **Returns**: Number if parseable, otherwise String

### 2. Lexer Changes
- Added `Token::Ask`
- Added keyword mapping: `"ask" => Token::Ask`

### 3. AST Changes
- Added `Statement::Ask(Vec<Expression>)`

### 4. Parser Changes
- Added parsing case for `Token::Ask`
- Updated terminator checks to include `Token::Ask`

### 5. Evaluator Changes
- Implemented stdin reading with optional prompt
- Added type detection (number vs string)

### 6. Tests
- Parser tests for various `ask` syntaxes
- Integration test demonstrating usage

## Best Practices

1. **Keep It Simple**: Follow Tilde' philosophy of simplicity
2. **Consistency**: Match existing code patterns and style
3. **Error Handling**: Provide clear error messages
4. **Test Coverage**: Write comprehensive tests
5. **Documentation**: Keep docs in sync with implementation

## Checklist

Before considering a feature complete:

- [ ] Token added to lexer
- [ ] AST node defined
- [ ] Parser implementation complete
- [ ] Evaluator implementation complete
- [ ] Unit tests written and passing
- [ ] Integration tests written and passing
- [ ] Baseline benchmarks captured
- [ ] Feature implemented
- [ ] Post-implementation benchmarks run
- [ ] Performance impact assessed (<10% regression)
- [ ] Memory impact justified if significant
- [ ] SYNTAX.md updated
- [ ] Example code created
- [ ] Code formatted (`cargo fmt`)
- [ ] Linter passing (`cargo clippy`)
- [ ] All tests passing (`cargo test`)
- [ ] Where practical, the new feature is a distinct module

## Common Pitfalls

1. **Forgetting terminator checks**: Ensure your token is recognized as a statement boundary
2. **Missing test cases**: Test edge cases and error conditions
3. **Inconsistent spacing**: Follow existing patterns for multi-argument commands
4. **Type handling**: Consider how your feature handles different value types

## Questions?

Refer to existing implementations (`say`, `ask`) as reference examples. The codebase follows consistent patterns that should guide new implementations.