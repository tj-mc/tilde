use tails::evaluator::Evaluator;
use tails::parser::Parser;
use tails::value::Value;

// ============================================================================
// PREDICATE FUNCTION TESTS
// ============================================================================

#[test]
fn test_is_even() {
    let mut evaluator = Evaluator::new();

    // Test even numbers
    let program_text = "~result is is-even 4\nsay ~result";
    let mut parser = Parser::new(program_text);
    let program = parser.parse().unwrap();
    let result = evaluator.eval_program(program).unwrap();
    if let Value::String(output) = result {
        assert!(output.contains("true"));
    }

    // Test odd numbers
    let mut evaluator = Evaluator::new();
    let program_text = "~result is is-even 3\nsay ~result";
    let mut parser = Parser::new(program_text);
    let program = parser.parse().unwrap();
    let result = evaluator.eval_program(program).unwrap();
    if let Value::String(output) = result {
        assert!(output.contains("false"));
    }

    // Test zero
    let mut evaluator = Evaluator::new();
    let program_text = "~result is is-even 0\nsay ~result";
    let mut parser = Parser::new(program_text);
    let program = parser.parse().unwrap();
    let result = evaluator.eval_program(program).unwrap();
    if let Value::String(output) = result {
        assert!(output.contains("true"));
    }
}

#[test]
fn test_is_odd() {
    let mut evaluator = Evaluator::new();

    // Test odd numbers
    let program_text = "~result is is-odd 3\nsay ~result";
    let mut parser = Parser::new(program_text);
    let program = parser.parse().unwrap();
    let result = evaluator.eval_program(program).unwrap();
    if let Value::String(output) = result {
        assert!(output.contains("true"));
    }

    // Test even numbers
    let mut evaluator = Evaluator::new();
    let program_text = "~result is is-odd 4\nsay ~result";
    let mut parser = Parser::new(program_text);
    let program = parser.parse().unwrap();
    let result = evaluator.eval_program(program).unwrap();
    if let Value::String(output) = result {
        assert!(output.contains("false"));
    }
}

#[test]
fn test_is_positive() {
    let mut evaluator = Evaluator::new();

    // Test positive number
    let program_text = "~result is is-positive 5\nsay ~result";
    let mut parser = Parser::new(program_text);
    let program = parser.parse().unwrap();
    let result = evaluator.eval_program(program).unwrap();
    if let Value::String(output) = result {
        assert!(output.contains("true"));
    }

    // Test zero
    let mut evaluator = Evaluator::new();
    let program_text = "~result is is-positive 0\nsay ~result";
    let mut parser = Parser::new(program_text);
    let program = parser.parse().unwrap();
    let result = evaluator.eval_program(program).unwrap();
    if let Value::String(output) = result {
        assert!(output.contains("false"));
    }
}

#[test]
fn test_is_negative() {
    let mut evaluator = Evaluator::new();

    // Test negative number - need to use expression since literals don't support negatives
    let program_text = "~num is 0 - 5\n~result is is-negative ~num\nsay ~result";
    let mut parser = Parser::new(program_text);
    let program = parser.parse().unwrap();
    let result = evaluator.eval_program(program).unwrap();
    if let Value::String(output) = result {
        assert!(output.contains("true"));
    }

    // Test positive number
    let mut evaluator = Evaluator::new();
    let program_text = "~result is is-negative 5\nsay ~result";
    let mut parser = Parser::new(program_text);
    let program = parser.parse().unwrap();
    let result = evaluator.eval_program(program).unwrap();
    if let Value::String(output) = result {
        assert!(output.contains("false"));
    }
}

#[test]
fn test_is_zero() {
    let mut evaluator = Evaluator::new();

    // Test zero
    let program_text = "~result is is-zero 0\nsay ~result";
    let mut parser = Parser::new(program_text);
    let program = parser.parse().unwrap();
    let result = evaluator.eval_program(program).unwrap();
    if let Value::String(output) = result {
        assert!(output.contains("true"));
    }

    // Test non-zero
    let mut evaluator = Evaluator::new();
    let program_text = "~result is is-zero 5\nsay ~result";
    let mut parser = Parser::new(program_text);
    let program = parser.parse().unwrap();
    let result = evaluator.eval_program(program).unwrap();
    if let Value::String(output) = result {
        assert!(output.contains("false"));
    }
}

// ============================================================================
// TRANSFORMATION FUNCTION TESTS
// ============================================================================

#[test]
fn test_double() {
    let mut evaluator = Evaluator::new();

    let program_text = "~result is double 5\nsay ~result";
    let mut parser = Parser::new(program_text);
    let program = parser.parse().unwrap();
    let result = evaluator.eval_program(program).unwrap();
    if let Value::String(output) = result {
        assert!(output.contains("10"));
    }
}

#[test]
fn test_triple() {
    let mut evaluator = Evaluator::new();

    let program_text = "~result is triple 4\nsay ~result";
    let mut parser = Parser::new(program_text);
    let program = parser.parse().unwrap();
    let result = evaluator.eval_program(program).unwrap();
    if let Value::String(output) = result {
        assert!(output.contains("12"));
    }
}

#[test]
fn test_quadruple() {
    let mut evaluator = Evaluator::new();

    let program_text = "~result is quadruple 3\nsay ~result";
    let mut parser = Parser::new(program_text);
    let program = parser.parse().unwrap();
    let result = evaluator.eval_program(program).unwrap();
    if let Value::String(output) = result {
        assert!(output.contains("12"));
    }
}

#[test]
fn test_half() {
    let mut evaluator = Evaluator::new();

    let program_text = "~result is half 10\nsay ~result";
    let mut parser = Parser::new(program_text);
    let program = parser.parse().unwrap();
    let result = evaluator.eval_program(program).unwrap();
    if let Value::String(output) = result {
        assert!(output.contains("5"));
    }
}

#[test]
fn test_square() {
    let mut evaluator = Evaluator::new();

    let program_text = "~result is square 4\nsay ~result";
    let mut parser = Parser::new(program_text);
    let program = parser.parse().unwrap();
    let result = evaluator.eval_program(program).unwrap();
    if let Value::String(output) = result {
        assert!(output.contains("16"));
    }
}

#[test]
fn test_increment() {
    let mut evaluator = Evaluator::new();

    let program_text = "~result is increment 5\nsay ~result";
    let mut parser = Parser::new(program_text);
    let program = parser.parse().unwrap();
    let result = evaluator.eval_program(program).unwrap();
    if let Value::String(output) = result {
        assert!(output.contains("6"));
    }
}

#[test]
fn test_decrement() {
    let mut evaluator = Evaluator::new();

    let program_text = "~result is decrement 5\nsay ~result";
    let mut parser = Parser::new(program_text);
    let program = parser.parse().unwrap();
    let result = evaluator.eval_program(program).unwrap();
    if let Value::String(output) = result {
        assert!(output.contains("4"));
    }
}

// ============================================================================
// REDUCTION FUNCTION TESTS
// ============================================================================

#[test]
fn test_add() {
    let mut evaluator = Evaluator::new();

    let program_text = "~result is add 3 7\nsay ~result";
    let mut parser = Parser::new(program_text);
    let program = parser.parse().unwrap();
    let result = evaluator.eval_program(program).unwrap();
    if let Value::String(output) = result {
        assert!(output.contains("10"));
    }
}

#[test]
fn test_multiply() {
    let mut evaluator = Evaluator::new();

    let program_text = "~result is multiply 4 5\nsay ~result";
    let mut parser = Parser::new(program_text);
    let program = parser.parse().unwrap();
    let result = evaluator.eval_program(program).unwrap();
    if let Value::String(output) = result {
        assert!(output.contains("20"));
    }
}

#[test]
fn test_max() {
    let mut evaluator = Evaluator::new();

    let program_text = "~result is max 3 7\nsay ~result";
    let mut parser = Parser::new(program_text);
    let program = parser.parse().unwrap();
    let result = evaluator.eval_program(program).unwrap();
    if let Value::String(output) = result {
        assert!(output.contains("7"));
    }

    // Test equal values
    let mut evaluator = Evaluator::new();
    let program_text = "~result is max 5 5\nsay ~result";
    let mut parser = Parser::new(program_text);
    let program = parser.parse().unwrap();
    let result = evaluator.eval_program(program).unwrap();
    if let Value::String(output) = result {
        assert!(output.contains("5"));
    }
}

#[test]
fn test_min() {
    let mut evaluator = Evaluator::new();

    let program_text = "~result is min 3 7\nsay ~result";
    let mut parser = Parser::new(program_text);
    let program = parser.parse().unwrap();
    let result = evaluator.eval_program(program).unwrap();
    if let Value::String(output) = result {
        assert!(output.contains("3"));
    }
}

// ============================================================================
// STRING FUNCTION TESTS
// ============================================================================

#[test]
fn test_uppercase() {
    let mut evaluator = Evaluator::new();

    let program_text = "~result is uppercase \"hello world\"\nsay ~result";
    let mut parser = Parser::new(program_text);
    let program = parser.parse().unwrap();
    let result = evaluator.eval_program(program).unwrap();
    if let Value::String(output) = result {
        assert!(output.contains("HELLO WORLD"));
    }
}

#[test]
fn test_lowercase() {
    let mut evaluator = Evaluator::new();

    let program_text = "~result is lowercase \"HELLO WORLD\"\nsay ~result";
    let mut parser = Parser::new(program_text);
    let program = parser.parse().unwrap();
    let result = evaluator.eval_program(program).unwrap();
    if let Value::String(output) = result {
        assert!(output.contains("hello world"));
    }
}

#[test]
fn test_is_empty_alternative() {
    let mut evaluator = Evaluator::new();

    // Test empty string using length (since is-empty is redundant)
    let program_text = "~result is (length \"\") == 0\nsay ~result";
    let mut parser = Parser::new(program_text);
    let program = parser.parse().unwrap();
    let result = evaluator.eval_program(program).unwrap();
    if let Value::String(output) = result {
        assert!(output.contains("true"));
    }

    // Test non-empty string
    let mut evaluator = Evaluator::new();
    let program_text = "~result is (length \"hello\") == 0\nsay ~result";
    let mut parser = Parser::new(program_text);
    let program = parser.parse().unwrap();
    let result = evaluator.eval_program(program).unwrap();
    if let Value::String(output) = result {
        assert!(output.contains("false"));
    }
}

#[test]
fn test_length_with_strings() {
    let mut evaluator = Evaluator::new();

    // Test with built-in length function (not stdlib)
    let program_text = "~result is length \"hello\"\nsay ~result";
    let mut parser = Parser::new(program_text);
    let program = parser.parse().unwrap();
    let result = evaluator.eval_program(program).unwrap();
    if let Value::String(output) = result {
        assert!(output.contains("5"));
    }

    // Test empty string
    let mut evaluator = Evaluator::new();
    let program_text = "~result is length \"\"\nsay ~result";
    let mut parser = Parser::new(program_text);
    let program = parser.parse().unwrap();
    let result = evaluator.eval_program(program).unwrap();
    if let Value::String(output) = result {
        assert!(output.contains("0"));
    }
}

// ============================================================================
// INTEGRATION TESTS WITH MAP/FILTER/REDUCE
// ============================================================================

#[test]
fn test_stdlib_helper_functions_direct_usage() {
    let mut evaluator = Evaluator::new();

    let program_text = r#"
~numbers is [2, 4, 6, 8]
~first_doubled is double (~numbers.0)
~first_squared is square (~numbers.0)
~first_is_even is is-even (~numbers.0)
say "Double of 2: " ~first_doubled " Square of 2: " ~first_squared " 2 is even: " ~first_is_even
"#;
    let mut parser = Parser::new(program_text);
    let program = parser.parse().unwrap();
    let result = evaluator.eval_program(program).unwrap();

    if let Value::String(output) = result {
        assert!(output.contains("Double of 2: 4 Square of 2: 4 2 is even: true"));
    }
}

#[test]
fn test_helper_functions_with_filter() {
    let mut evaluator = Evaluator::new();

    let program_text = r#"
~numbers is [1, 2, 3, 4, 5, 0]
~evens is filter ~numbers is-even
~positives is filter ~numbers is-positive
say "Evens: " ~evens " Positives: " ~positives
"#;
    let mut parser = Parser::new(program_text);
    let program = parser.parse().unwrap();
    let result = evaluator.eval_program(program).unwrap();

    if let Value::String(output) = result {
        assert!(output.contains("Evens: [2, 4, 0] Positives: [1, 2, 3, 4, 5]"));
    }
}

#[test]
fn test_helper_functions_with_reduce() {
    let mut evaluator = Evaluator::new();

    let program_text = r#"
~numbers is [1, 2, 3, 4, 5]
~sum is reduce ~numbers add 0
~product is reduce ~numbers multiply 1
~maximum is reduce ~numbers max 0
say "Sum: " ~sum " Product: " ~product " Max: " ~maximum
"#;
    let mut parser = Parser::new(program_text);
    let program = parser.parse().unwrap();
    let result = evaluator.eval_program(program).unwrap();

    if let Value::String(output) = result {
        assert!(output.contains("Sum: 15 Product: 120 Max: 5"));
    }
}

#[test]
fn test_string_functions_with_map() {
    let mut evaluator = Evaluator::new();

    let program_text = r#"
~words is ["hello", "WORLD", "TeSt"]
~upper is map ~words uppercase
~lower is map ~words lowercase
say "Upper: " ~upper " Lower: " ~lower
"#;
    let mut parser = Parser::new(program_text);
    let program = parser.parse().unwrap();
    let result = evaluator.eval_program(program).unwrap();

    if let Value::String(output) = result {
        assert!(output.contains("Upper: [HELLO, WORLD, TEST] Lower: [hello, world, test]"));
    }
}

// ============================================================================
// ERROR TESTS
// ============================================================================

#[test]
fn test_helper_function_errors() {
    let mut evaluator = Evaluator::new();

    // Test wrong argument count
    let program_text = "~result is double 1 2";
    let mut parser = Parser::new(program_text);
    let program = parser.parse().unwrap();
    let result = evaluator.eval_program(program);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("double requires exactly 1 argument"));

    // Test wrong argument type
    let mut evaluator = Evaluator::new();
    let program_text = "~result is is-even \"not a number\"";
    let mut parser = Parser::new(program_text);
    let program = parser.parse().unwrap();
    let result = evaluator.eval_program(program);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("is-even argument must be a number"));

    // Test string function with wrong type
    let mut evaluator = Evaluator::new();
    let program_text = "~result is uppercase 123";
    let mut parser = Parser::new(program_text);
    let program = parser.parse().unwrap();
    let result = evaluator.eval_program(program);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("uppercase argument must be a string"));
}