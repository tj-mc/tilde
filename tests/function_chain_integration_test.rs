use tilde::evaluator::Evaluator;
use tilde::parser::Parser;

#[test]
fn test_simple_chain_integration() {
    let input = "~result: reverse [1, 2, 3]";

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_program(program).unwrap();

    assert_eq!(result.to_string(), "[3, 2, 1]");
}

#[test]
fn test_chain_with_length() {
    let input = "~result: length [1, 2, 3, 4, 5]";

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_program(program).unwrap();

    assert_eq!(result.to_string(), "5");
}

#[test]
fn test_chain_with_variables() {
    let input = "~numbers is [1, 2, 3]\n~result: reverse ~numbers";

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_program(program).unwrap();

    assert_eq!(result.to_string(), "[3, 2, 1]");
}

#[test]
fn test_chain_with_math() {
    let input = "~nums is [1, 2, 3, 4, 5]\n~result: length ~nums";

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_program(program).unwrap();

    assert_eq!(result.to_string(), "5");
}

#[test]
fn test_chain_with_multiple_steps() {
    let input = "~nums is [1, 2, 3, 4, 5, 6]\n~result: reverse ~nums";

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_program(program).unwrap();

    assert_eq!(result.to_string(), "[6, 5, 4, 3, 2, 1]");
}

#[test]
fn test_chain_with_string_ops() {
    let input = "~words is [\"hello\", \"world\", \"test\"]\n~result: join ~words \" \"";

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_program(program).unwrap();

    assert_eq!(result.to_string(), "hello world test");
}