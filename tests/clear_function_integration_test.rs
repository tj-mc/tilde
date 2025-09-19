use tails::{evaluator::Evaluator, parser::Parser, value::Value};

#[test]
fn test_clear_simple_statement() {
    let input = r#"
        clear
        ~message is "Screen cleared"
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_program(program);

    assert!(result.is_ok());
    assert_eq!(
        evaluator.get_variable("message"),
        Some(&Value::String("Screen cleared".to_string()))
    );
}

#[test]
fn test_clear_as_expression() {
    let input = r#"
        ~result is clear
        ~message is "Clear returned"
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    // clear should return null
    assert_eq!(evaluator.get_variable("result"), Some(&Value::Null));
    assert_eq!(
        evaluator.get_variable("message"),
        Some(&Value::String("Clear returned".to_string()))
    );
}

#[test]
fn test_clear_in_conditional() {
    let input = r#"
        ~should_clear is true
        ~status is ""

        if ~should_clear (
            clear
            ~status is "cleared"
        ) else (
            ~status is "not cleared"
        )
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    assert_eq!(
        evaluator.get_variable("status"),
        Some(&Value::String("cleared".to_string()))
    );
}

#[test]
fn test_clear_in_loop() {
    let input = r#"
        ~i is 0
        ~clear_count is 0

        loop (
            if ~i >= 3 break-loop
            clear
            ~clear_count is ~clear_count + 1
            ~i is ~i + 1
        )
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    assert_eq!(evaluator.get_variable("clear_count"), Some(&Value::Number(3.0)));
    assert_eq!(evaluator.get_variable("i"), Some(&Value::Number(3.0)));
}

#[test]
fn test_clear_with_graphics_simulation() {
    let input = r#"
        ~frame is 0
        ~max_frames is 5

        loop (
            if ~frame >= ~max_frames break-loop

            clear
            say "Frame: " ~frame
            say "    🐈‍⬛"  # Cat animation

            ~frame is ~frame + 1
        )

        ~final_message is "Animation complete"
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    assert_eq!(evaluator.get_variable("frame"), Some(&Value::Number(5.0)));
    assert_eq!(
        evaluator.get_variable("final_message"),
        Some(&Value::String("Animation complete".to_string()))
    );
}

#[test]
fn test_clear_error_with_arguments() {
    let input = r#"
        ~result is clear "invalid argument"
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_program(program);

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("clear takes no arguments"));
}

#[test]
fn test_clear_error_with_multiple_arguments() {
    let input = r#"
        clear "arg1" "arg2" 123
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_program(program);

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("clear takes no arguments"));
}

#[test]
fn test_clear_with_other_functions() {
    let input = r#"
        say "Before clear"
        clear
        say "After clear"
        wait 0.1
        clear
        say "Final message"
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_program(program);

    assert!(result.is_ok());
}

#[test]
fn test_clear_chained_with_assignments() {
    let input = r#"
        ~x is 10
        ~y is 20
        clear
        ~position is ~x + ~y
        say "Position: " ~position
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    assert_eq!(evaluator.get_variable("position"), Some(&Value::Number(30.0)));
}

#[test]
fn test_clear_with_action_definition() {
    let input = r#"
        action animate ~frame_count (
            ~i is 0
            loop (
                if ~i >= ~frame_count break-loop
                clear
                say "Animation frame: " ~i
                ~i is ~i + 1
            )
            give ~frame_count
        )

        ~result is *animate 3
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    assert_eq!(evaluator.get_variable("result"), Some(&Value::Number(3.0)));
}