use tilde::{evaluator::Evaluator, parser::Parser, value::Value};

#[test]
fn test_random_float_literal_detection() {
    // Test that 0.0 1 returns float (not integer)
    let input = "~val is random 0.0 1";
    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_program(program);

    assert!(result.is_ok());

    let val = evaluator.get_variable("val").unwrap();
    if let Value::Number(n) = val {
        assert!(*n >= 0.0 && *n <= 1.0);
        // This is the key test - it should be a float, not just 0 or 1
        // Run multiple times to ensure we sometimes get non-integer values
        let mut found_non_integer = false;
        for _ in 0..50 {
            let mut parser = Parser::new("~val is random 0.0 1");
            let program = parser.parse().unwrap();
            let mut evaluator = Evaluator::new();
            evaluator.eval_program(program).unwrap();

            if let Some(Value::Number(n)) = evaluator.get_variable("val") {
                if n.fract() != 0.0 {
                    found_non_integer = true;
                    break;
                }
            }
        }
        assert!(found_non_integer, "random 0.0 1 should return floats, not just integers");
    } else {
        panic!("Expected number value");
    }
}

#[test]
fn test_random_integer_vs_float_literal() {
    // Test all combinations to ensure proper behavior
    let test_cases = vec![
        ("random 0 1", true),     // Both integers -> should return int (0 or 1)
        ("random 0.0 1", false), // Float literal -> should return float
        ("random 0 1.0", false), // Float literal -> should return float
        ("random 0.0 1.0", false), // Both float literals -> should return float
    ];

    for (expr, should_be_integer_only) in test_cases {
        let input = format!("~val is {}", expr);
        let mut found_non_integer = false;

        // Test multiple times to see the range of outputs
        for _ in 0..100 {
            let mut parser = Parser::new(&input);
            let program = parser.parse().unwrap();
            let mut evaluator = Evaluator::new();
            evaluator.eval_program(program).unwrap();

            if let Some(Value::Number(n)) = evaluator.get_variable("val") {
                if n.fract() != 0.0 {
                    found_non_integer = true;
                    break;
                }
            }
        }

        if should_be_integer_only {
            assert!(!found_non_integer, "{} should only return integers (0 or 1)", expr);
        } else {
            assert!(found_non_integer, "{} should return floats (including non-integers)", expr);
        }
    }
}