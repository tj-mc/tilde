use tilde::{evaluator::Evaluator, parser::Parser, value::Value};

#[test]
fn test_negative_integer_literal() {
    let input = "
        ~negative is -42
        ~positive is 42
    ";

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    assert_eq!(
        evaluator.get_variable("negative"),
        Some(&Value::Number(-42.0))
    );
    assert_eq!(
        evaluator.get_variable("positive"),
        Some(&Value::Number(42.0))
    );
}

#[test]
fn test_negative_float_literal() {
    let input = "
        ~negative_pi is -3.14159
        ~positive_pi is 3.14159
    ";

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    assert_eq!(
        evaluator.get_variable("negative_pi"),
        Some(&Value::Number(-3.14159))
    );
    assert_eq!(
        evaluator.get_variable("positive_pi"),
        Some(&Value::Number(3.14159))
    );
}

#[test]
fn test_negative_zero() {
    let input = "
        ~negative_zero is -0
        ~positive_zero is 0
    ";

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    assert_eq!(
        evaluator.get_variable("negative_zero"),
        Some(&Value::Number(-0.0))
    );
    assert_eq!(
        evaluator.get_variable("positive_zero"),
        Some(&Value::Number(0.0))
    );
}

#[test]
fn test_negative_literals_in_expressions() {
    let input = "
        ~result1 is -5 + 10
        ~result2 is -3.5 * 2
        ~result3 is -10 / -2
    ";

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    assert_eq!(evaluator.get_variable("result1"), Some(&Value::Number(5.0)));
    assert_eq!(
        evaluator.get_variable("result2"),
        Some(&Value::Number(-7.0))
    );
    assert_eq!(evaluator.get_variable("result3"), Some(&Value::Number(5.0)));
}

#[test]
fn test_negative_literals_with_stdlib_functions() {
    let input = "
        ~numbers is [-1, -2, -3, 4, 5]
        ~positives is filter ~numbers is-positive
        ~negatives is filter ~numbers is-negative
        ~absolute_values is map ~numbers absolute
    ";

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    assert_eq!(
        evaluator.get_variable("positives"),
        Some(&Value::List(vec![Value::Number(4.0), Value::Number(5.0)]))
    );
    assert_eq!(
        evaluator.get_variable("negatives"),
        Some(&Value::List(vec![
            Value::Number(-1.0),
            Value::Number(-2.0),
            Value::Number(-3.0)
        ]))
    );
    assert_eq!(
        evaluator.get_variable("absolute_values"),
        Some(&Value::List(vec![
            Value::Number(1.0),
            Value::Number(2.0),
            Value::Number(3.0),
            Value::Number(4.0),
            Value::Number(5.0)
        ]))
    );
}

#[test]
fn test_negative_large_numbers() {
    let input = "
        ~large_negative is -999999999
        ~large_negative_float is -999999.999999
    ";

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    assert_eq!(
        evaluator.get_variable("large_negative"),
        Some(&Value::Number(-999999999.0))
    );
    assert_eq!(
        evaluator.get_variable("large_negative_float"),
        Some(&Value::Number(-999999.999999))
    );
}

#[test]
fn test_very_small_negative_float() {
    let input = "
        ~tiny_negative is -0.000001
    ";

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    assert_eq!(
        evaluator.get_variable("tiny_negative"),
        Some(&Value::Number(-0.000001))
    );
}

#[test]
fn test_negative_literals_in_list() {
    let input = "
        ~mixed_list is [-42, 3.14, -0.5, 100, -999]
    ";

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    assert_eq!(
        evaluator.get_variable("mixed_list"),
        Some(&Value::List(vec![
            Value::Number(-42.0),
            Value::Number(3.14),
            Value::Number(-0.5),
            Value::Number(100.0),
            Value::Number(-999.0)
        ]))
    );
}

#[test]
fn test_negative_literal_comparisons() {
    let input = "
        ~is_less is -5 < 0
        ~is_greater is -10 > -20
        ~is_equal is -42 == -42
    ";

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    assert_eq!(
        evaluator.get_variable("is_less"),
        Some(&Value::Boolean(true))
    );
    assert_eq!(
        evaluator.get_variable("is_greater"),
        Some(&Value::Boolean(true))
    );
    assert_eq!(
        evaluator.get_variable("is_equal"),
        Some(&Value::Boolean(true))
    );
}
