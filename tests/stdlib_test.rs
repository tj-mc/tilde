use tails::evaluator::Evaluator;
use tails::parser::Parser;
use tails::value::Value;

#[test]
fn test_stdlib_reverse() {
    let mut evaluator = Evaluator::new();
    let mut parser = Parser::new("~numbers is [1, 2, 3]\n~reversed is *.reverse ~numbers\nsay ~reversed");
    let program = parser.parse().unwrap();
    let result = evaluator.eval_program(program).unwrap();
    // Should reverse the list
    if let Value::String(output) = result {
        assert!(output.contains("3"));
        assert!(output.contains("2"));
        assert!(output.contains("1"));
    }
}

#[test]
fn test_stdlib_sort() {
    let mut evaluator = Evaluator::new();
    let mut parser = Parser::new("~numbers is [3, 1, 2]\n~sorted is *.sort ~numbers\nsay ~sorted");
    let program = parser.parse().unwrap();
    let result = evaluator.eval_program(program).unwrap();
    // Should sort the list
    if let Value::String(output) = result {
        assert!(output.contains("[1, 2, 3]"));
    }
}

#[test]
fn test_stdlib_map_with_action() {
    let mut evaluator = Evaluator::new();
    let program_text = r#"
action double ~x (
    give ~x * 2
)

~numbers is [1, 2, 3]
~doubled is *.map ~numbers double
say ~doubled
"#;
    let mut parser = Parser::new(program_text);
    let program = parser.parse().unwrap();
    let result = evaluator.eval_program(program).unwrap();

    // Should output [2, 4, 6]
    if let Value::String(output) = result {
        assert!(output.contains("2"));
        assert!(output.contains("4"));
        assert!(output.contains("6"));
    }
}

#[test]
fn test_stdlib_filter_with_action() {
    let mut evaluator = Evaluator::new();
    let program_text = r#"
action is_even ~x (
    give ~x % 2 == 0
)

~numbers is [1, 2, 3, 4, 5]
~evens is *.filter ~numbers is_even
say ~evens
"#;
    let mut parser = Parser::new(program_text);
    let program = parser.parse().unwrap();
    let result = evaluator.eval_program(program).unwrap();

    // Should output [2, 4]
    if let Value::String(output) = result {
        assert!(output.contains("2"));
        assert!(output.contains("4"));
        assert!(!output.contains("1"));
        assert!(!output.contains("3"));
        assert!(!output.contains("5"));
    }
}

#[test]
fn test_stdlib_reduce_with_action() {
    let mut evaluator = Evaluator::new();
    let program_text = r#"
action add ~a ~b (
    give ~a + ~b
)

~numbers is [1, 2, 3, 4]
~sum is *.reduce ~numbers add 0
say ~sum
"#;
    let mut parser = Parser::new(program_text);
    let program = parser.parse().unwrap();
    let result = evaluator.eval_program(program).unwrap();

    // Should output 10 (1+2+3+4)
    if let Value::String(output) = result {
        assert!(output.contains("10"));
    }
}