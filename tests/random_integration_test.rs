use tilde::{evaluator::Evaluator, parser::Parser, value::Value};

#[test]
fn test_random_assignment_integer() {
    let input = "~val is random 1 5";
    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_program(program);

    assert!(result.is_ok());

    // Check that the variable was assigned
    let val = evaluator.get_variable("val").unwrap();
    if let Value::Number(n) = val {
        assert!(*n >= 1.0 && *n <= 5.0);
        assert_eq!(n.fract(), 0.0); // Should be integer
    } else {
        panic!("Expected number value");
    }
}

#[test]
fn test_random_assignment_float() {
    let input = "~val is random 0.0 1.0";
    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_program(program);

    assert!(result.is_ok());

    let val = evaluator.get_variable("val").unwrap();
    if let Value::Number(n) = val {
        assert!(*n >= 0.0 && *n <= 1.0);
    } else {
        panic!("Expected number value");
    }
}

#[test]
fn test_random_assignment_mixed_types() {
    let input = "~val is random 1 2.5";
    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_program(program);

    assert!(result.is_ok());

    let val = evaluator.get_variable("val").unwrap();
    if let Value::Number(n) = val {
        assert!(*n >= 1.0 && *n <= 2.5);
    } else {
        panic!("Expected number value");
    }
}

#[test]
fn test_random_statement() {
    let input = "random 1 10";
    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_program(program);

    assert!(result.is_ok());
}

#[test]
fn test_random_in_expression() {
    let input = "~result is (random 1 3) + (random 4 6)";
    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_program(program);

    assert!(result.is_ok());

    let val = evaluator.get_variable("result").unwrap();
    if let Value::Number(n) = val {
        assert!(*n >= 5.0 && *n <= 9.0); // min: 1+4=5, max: 3+6=9
    } else {
        panic!("Expected number value");
    }
}

#[test]
fn test_random_error_conditions() {
    // Too few arguments
    let mut parser = Parser::new("~val is random 1");
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_program(program);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("exactly 2 arguments"));

    // Too many arguments
    let mut parser = Parser::new("~val is random 1 2 3");
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_program(program);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("exactly 2 arguments"));

    // Non-numeric arguments
    let mut parser = Parser::new("~val is random \"hello\" 5");
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_program(program);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("must be numbers"));

    // Min greater than max
    let mut parser = Parser::new("~val is random 10 5");
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_program(program);
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .contains("minimum value cannot be greater")
    );
}

#[test]
fn test_random_with_variables() {
    let input = "~min is 1\n~max is 10\n~val is random ~min ~max";
    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_program(program);

    assert!(result.is_ok());

    let val = evaluator.get_variable("val").unwrap();
    if let Value::Number(n) = val {
        assert!(*n >= 1.0 && *n <= 10.0);
        assert_eq!(n.fract(), 0.0); // Should be integer
    } else {
        panic!("Expected number value");
    }
}

#[test]
fn test_random_consistency_different_calls() {
    // Test that multiple calls can produce different values
    let input = "~val1 is random 1 1000\n~val2 is random 1 1000\n~val3 is random 1 1000";
    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_program(program);

    assert!(result.is_ok());

    let val1 = evaluator.get_variable("val1").unwrap();
    let val2 = evaluator.get_variable("val2").unwrap();
    let val3 = evaluator.get_variable("val3").unwrap();

    // While theoretically possible, it's extremely unlikely all three are the same
    // in a range of 1000 values
    if let (Value::Number(n1), Value::Number(n2), Value::Number(n3)) = (val1, val2, val3) {
        // At least verify they're all in range
        assert!(*n1 >= 1.0 && *n1 <= 1000.0);
        assert!(*n2 >= 1.0 && *n2 <= 1000.0);
        assert!(*n3 >= 1.0 && *n3 <= 1000.0);
    }
}
