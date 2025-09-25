use tilde::{evaluator::Evaluator, parser::Parser};

#[test]
fn test_basic_increment() {
    let input = r#"
        ~counter is 5
        ~counter up 3
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    evaluator.eval_program(program).unwrap();

    let counter = evaluator.get_variable("counter").unwrap();
    assert_eq!(counter.to_string(), "8");
}

#[test]
fn test_basic_decrement() {
    let input = r#"
        ~counter is 10
        ~counter down 4
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    evaluator.eval_program(program).unwrap();

    let counter = evaluator.get_variable("counter").unwrap();
    assert_eq!(counter.to_string(), "6");
}

#[test]
fn test_increment_with_variable_amount() {
    let input = r#"
        ~counter is 5
        ~amount is 7
        ~counter up ~amount
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    evaluator.eval_program(program).unwrap();

    let counter = evaluator.get_variable("counter").unwrap();
    assert_eq!(counter.to_string(), "12");
}

#[test]
fn test_decrement_with_variable_amount() {
    let input = r#"
        ~counter is 20
        ~amount is 8
        ~counter down ~amount
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    evaluator.eval_program(program).unwrap();

    let counter = evaluator.get_variable("counter").unwrap();
    assert_eq!(counter.to_string(), "12");
}

#[test]
fn test_increment_with_expression() {
    let input = r#"
        ~counter is 5
        ~base is 3
        ~counter up (~base + 2)
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    evaluator.eval_program(program).unwrap();

    let counter = evaluator.get_variable("counter").unwrap();
    assert_eq!(counter.to_string(), "10");
}

#[test]
fn test_multiple_operations() {
    let input = r#"
        ~counter is 0
        ~counter up 5
        ~counter up 3
        ~counter down 2
        ~counter up 1
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    evaluator.eval_program(program).unwrap();

    let counter = evaluator.get_variable("counter").unwrap();
    assert_eq!(counter.to_string(), "7");
}

#[test]
fn test_float_operations() {
    let input = r#"
        ~value is 10.5
        ~value up 2.3
        ~value down 1.8
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    evaluator.eval_program(program).unwrap();

    let value = evaluator.get_variable("value").unwrap();
    assert_eq!(value.to_string(), "11");
}

#[test]
fn test_increment_in_loop() {
    let input = r#"
        ~total is 0
        ~i is 0
        loop (
            if ~i >= 5 break-loop
            ~total up ~i
            ~i up 1
        )
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    evaluator.eval_program(program).unwrap();

    let total = evaluator.get_variable("total").unwrap();
    assert_eq!(total.to_string(), "10"); // 0+1+2+3+4 = 10
}

#[test]
fn test_increment_in_foreach() {
    let input = r#"
        ~numbers is [1, 2, 3, 4]
        ~sum is 0
        for-each ~num in ~numbers (
            ~sum up ~num
        )
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    evaluator.eval_program(program).unwrap();

    let sum = evaluator.get_variable("sum").unwrap();
    assert_eq!(sum.to_string(), "10");
}

#[test]
fn test_increment_undefined_variable() {
    let input = r#"
        ~undefined_var up 5
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .contains("Undefined variable: undefined_var")
    );
}

#[test]
fn test_increment_non_number_variable() {
    let input = r#"
        ~text is "hello"
        ~text up 5
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .contains("both variable and amount must be numbers")
    );
}

#[test]
fn test_increment_with_non_number_amount() {
    let input = r#"
        ~counter is 5
        ~counter up "invalid"
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .contains("both variable and amount must be numbers")
    );
}

#[test]
fn test_decrement_undefined_variable() {
    let input = r#"
        ~undefined_var down 5
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .contains("Undefined variable: undefined_var")
    );
}

#[test]
fn test_negative_results() {
    let input = r#"
        ~value is 5
        ~value down 10
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    evaluator.eval_program(program).unwrap();

    let value = evaluator.get_variable("value").unwrap();
    assert_eq!(value.to_string(), "-5");
}

#[test]
fn test_zero_amount() {
    let input = r#"
        ~value is 5
        ~value up 0
        ~value down 0
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    evaluator.eval_program(program).unwrap();

    let value = evaluator.get_variable("value").unwrap();
    assert_eq!(value.to_string(), "5");
}
