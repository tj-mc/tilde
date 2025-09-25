use tilde::{evaluator::Evaluator, parser::Parser, value::Value};

#[test]
fn test_env_function_basic() {
    // Set test environment variable
    unsafe {
        std::env::set_var("TILDE_TEST_VAR", "hello_world");
    }

    let input = r#"~result is env "TILDE_TEST_VAR""#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_ok());

    let env_value = evaluator.get_variable("result").unwrap();
    assert_eq!(env_value, &Value::String("hello_world".to_string()));
}

#[test]
fn test_env_function_missing_variable() {
    // Make sure this variable doesn't exist
    unsafe {
        std::env::remove_var("TILDE_NONEXISTENT_VAR");
    }

    let input = r#"~result is env "TILDE_NONEXISTENT_VAR""#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_ok());

    let env_value = evaluator.get_variable("result").unwrap();
    assert_eq!(env_value, &Value::Null);
}

#[test]
fn test_env_function_with_or_fallback() {
    // Make sure this variable doesn't exist
    unsafe {
        std::env::remove_var("TILDE_MISSING_PORT");
    }

    let input = r#"~port is env "TILDE_MISSING_PORT" or 8080"#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_ok());

    let port_value = evaluator.get_variable("port").unwrap();
    assert_eq!(port_value, &Value::Number(8080.0));
}

#[test]
fn test_env_function_with_or_fallback_existing() {
    // Set test environment variable
    unsafe {
        std::env::set_var("TILDE_EXISTING_PORT", "3000");
    }

    let input = r#"~port is env "TILDE_EXISTING_PORT" or 8080"#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_ok());

    let port_value = evaluator.get_variable("port").unwrap();
    assert_eq!(port_value, &Value::String("3000".to_string()));
}

#[test]
fn test_env_function_with_string_fallback() {
    // Make sure this variable doesn't exist
    unsafe {
        std::env::remove_var("TILDE_MISSING_REGION");
    }

    let input = r#"~region is env "TILDE_MISSING_REGION" or "us-east-1""#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_ok());

    let region_value = evaluator.get_variable("region").unwrap();
    assert_eq!(region_value, &Value::String("us-east-1".to_string()));
}

#[test]
fn test_env_function_error_cases() {
    // Test with no arguments
    let input = r#"~result is env"#;
    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_program(program);
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .contains("env requires exactly 1 argument")
    );

    // Test with non-string argument
    let input = r#"~result is env 123"#;
    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_program(program);
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .contains("env argument must be a string")
    );
}

#[test]
fn test_env_function_complex_usage() {
    // Set multiple test environment variables
    unsafe {
        std::env::set_var("TILDE_DATABASE_URL", "postgres://localhost:5432/mydb");
        std::env::set_var("TILDE_DEBUG", "true");
        std::env::remove_var("TILDE_API_KEY");
    }

    let input = r#"
        ~db_url is env "TILDE_DATABASE_URL"
        ~debug is env "TILDE_DEBUG"
        ~api_key is env "TILDE_API_KEY" or "default-key"
        ~timeout is env "TILDE_TIMEOUT" or 30000
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_ok());

    assert_eq!(
        evaluator.get_variable("db_url").unwrap(),
        &Value::String("postgres://localhost:5432/mydb".to_string())
    );
    assert_eq!(
        evaluator.get_variable("debug").unwrap(),
        &Value::String("true".to_string())
    );
    assert_eq!(
        evaluator.get_variable("api_key").unwrap(),
        &Value::String("default-key".to_string())
    );
    assert_eq!(
        evaluator.get_variable("timeout").unwrap(),
        &Value::Number(30000.0)
    );
}
