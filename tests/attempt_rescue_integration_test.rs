use tilde::{evaluator::Evaluator, parser::Parser, value::Value};

#[test]
fn test_attempt_rescue_basic_error_handling() {
    let input = r#"
    attempt (
        ~undefined_var is ~nonexistent + 1
    ) rescue ~error (
        ~result is "caught error"
    )
    "#;

    let mut parser = Parser::new(input);
    let parse_result = parser.parse();
    if let Err(e) = &parse_result {
        eprintln!("Parse error: {}", e);
    }
    let program = parse_result.unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    if let Err(e) = &result {
        eprintln!("Eval error: {}", e);
    }
    assert!(result.is_ok());

    let result_value = evaluator.get_variable("result").unwrap();
    assert_eq!(result_value, &Value::String("caught error".to_string()));
}

#[test]
fn test_attempt_rescue_no_error() {
    let input = r#"
    ~success_value is 42
    attempt (
        ~result is ~success_value * 2
    ) rescue (
        ~result is "error occurred"
    )
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_ok());

    let result_value = evaluator.get_variable("result").unwrap();
    assert_eq!(result_value, &Value::Number(84.0));
}

#[test]
fn test_attempt_rescue_error_variable_access() {
    let input = r#"
    attempt (
        ~result is 10 / 0
    ) rescue ~err (
        ~error_message is ~err
    )
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_ok());

    let error_var = evaluator.get_variable("error_message").unwrap();
    if let Value::Error(error_value) = error_var {
        assert!(error_value.message.contains("Division by zero"));
    } else {
        panic!(
            "Expected error_message to be an Error value, got: {:?}",
            error_var
        );
    }
}

#[test]
fn test_attempt_rescue_without_error_variable() {
    let input = r#"
    ~handled is false
    attempt (
        ~undefined_result is ~missing_var + 1
    ) rescue (
        ~handled is true
    )
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_ok());

    let handled_value = evaluator.get_variable("handled").unwrap();
    assert_eq!(handled_value, &Value::Boolean(true));
}

#[test]
fn test_attempt_rescue_nested_blocks() {
    let input = r#"

    attempt (
        attempt (
            ~x is ~undefined_var
        ) rescue (
            ~inner_result is "inner catch"
        )
        ~outer_result is "outer success"
    ) rescue (
        ~outer_result is "outer catch"
    )
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_ok());

    let inner_result = evaluator.get_variable("inner_result").unwrap();
    assert_eq!(inner_result, &Value::String("inner catch".to_string()));

    let outer_result = evaluator.get_variable("outer_result").unwrap();
    assert_eq!(outer_result, &Value::String("outer success".to_string()));
}

#[test]
fn test_attempt_rescue_error_in_rescue_block() {
    let input = r#"
    attempt (
        ~result is ~undefined_var
    ) rescue (
        ~another_undefined is ~also_undefined
    )
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    // Error in rescue block should still propagate
    assert!(result.is_err());
}

#[test]
fn test_attempt_rescue_with_function_call_error() {
    let input = r#"
    attempt (
        ~value is unknown_function 123
    ) rescue ~error (
        ~result is "function error caught"
    )
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_ok());

    let result_value = evaluator.get_variable("result").unwrap();
    assert_eq!(
        result_value,
        &Value::String("function error caught".to_string())
    );
}

#[test]
fn test_attempt_rescue_date_arithmetic_error() {
    let input = r#"
    ~error_caught is false
    attempt (
        ~bad_date is date-add "not-a-date" 5
    ) rescue ~err (
        ~error_caught is true
    )
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_ok());

    let error_caught = evaluator.get_variable("error_caught").unwrap();
    assert_eq!(error_caught, &Value::Boolean(true));
}

#[test]
fn test_error_value_is_truthy() {
    let input = r#"
    attempt (
        ~undefined_result is ~missing_var
    ) rescue ~err (
        ~error_var is ~err
    )

    if ~error_var (
        ~is_error_truthy is false
    ) else (
        ~is_error_truthy is true
    )
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_ok());

    // Error values should be falsy according to our design
    let is_truthy = evaluator.get_variable("is_error_truthy").unwrap();
    assert_eq!(is_truthy, &Value::Boolean(true));
}
