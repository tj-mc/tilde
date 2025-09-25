use tilde::{evaluator::Evaluator, parser::Parser, value::Value};

#[test]
fn test_contains_on_strings() {
    let input = r#"
    ~email is "user@example.com"
    ~has_at is contains ~email "@"
    ~has_dot is contains ~email "."
    ~has_xyz is contains ~email "xyz"
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_ok());

    assert_eq!(
        evaluator.get_variable("has_at"),
        Some(&Value::Boolean(true))
    );
    assert_eq!(
        evaluator.get_variable("has_dot"),
        Some(&Value::Boolean(true))
    );
    assert_eq!(
        evaluator.get_variable("has_xyz"),
        Some(&Value::Boolean(false))
    );
}

#[test]
fn test_contains_on_lists_still_works() {
    let input = r#"
    ~numbers is [1, 2, 3, 4, 5]
    ~has_three is contains ~numbers 3
    ~has_ten is contains ~numbers 10
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_ok());

    assert_eq!(
        evaluator.get_variable("has_three"),
        Some(&Value::Boolean(true))
    );
    assert_eq!(
        evaluator.get_variable("has_ten"),
        Some(&Value::Boolean(false))
    );
}

#[test]
fn test_contains_string_type_error() {
    let input = r#"
    ~text is "hello world"
    ~result is contains ~text 123
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .contains("search value must also be a string")
    );
}

#[test]
fn test_contains_with_attempt_rescue() {
    let input = r#"
    ~error_caught is false
    attempt (
        ~result is contains "hello" 123
    ) rescue (
        ~error_caught is true
    )
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_ok());

    assert_eq!(
        evaluator.get_variable("error_caught"),
        Some(&Value::Boolean(true))
    );
}

#[test]
fn test_index_of_on_strings() {
    let input = r#"
    ~email is "user@example.com"
    ~at_pos is index-of ~email "@"
    ~dot_pos is index-of ~email "."
    ~missing is index-of ~email "xyz"
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_ok());

    assert_eq!(evaluator.get_variable("at_pos"), Some(&Value::Number(4.0)));
    assert_eq!(
        evaluator.get_variable("dot_pos"),
        Some(&Value::Number(12.0))
    );
    assert_eq!(evaluator.get_variable("missing"), Some(&Value::Null));
}

#[test]
fn test_index_of_on_lists_still_works() {
    let input = r#"
    ~fruits is ["apple", "banana", "cherry"]
    ~banana_pos is index-of ~fruits "banana"
    ~missing_pos is index-of ~fruits "grape"
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_ok());

    assert_eq!(
        evaluator.get_variable("banana_pos"),
        Some(&Value::Number(1.0))
    );
    assert_eq!(evaluator.get_variable("missing_pos"), Some(&Value::Null));
}
