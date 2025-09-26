use tilde::evaluator::Evaluator;
use tilde::parser::Parser;

#[test]
fn test_arithmetic_nesting() {
    let input = "((((1 + 2) * 3) - 4) / 2)"; // This is a statement, last expression is returned
    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_program(program).unwrap();
    assert_eq!(result.to_string(), "2.5");
}

#[test]
fn test_deep_arithmetic_nesting() {
    let input = "(((((((1 + 1) + 1) + 1) + 1) + 1) + 1) + 1)";
    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_program(program).unwrap();
    assert_eq!(result.to_string(), "8");
}

#[test]
fn test_nested_anonymous_functions() {
    let input = r#"
        ~numbers is [1, 2, 3]
        map ~numbers |~x (map [10, 20] |~y (~x * ~y)|)|
    "#;
    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_program(program).unwrap();
    assert_eq!(result.to_string(), "[[10, 20], [20, 40], [30, 60]]");
}

#[test]
fn test_triple_nested_anonymous_functions() {
    let input = r#"
        ~data is [[1, 2], [3, 4]]
        map ~data |~row (map ~row |~x (map [10, 100] |~y (~x + ~y)|)|)|
    "#;
    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_program(program).unwrap();
    assert_eq!(
        result.to_string(),
        "[[[11, 101], [12, 102]], [[13, 103], [14, 104]]]"
    );
}

#[test]
fn test_mixed_nested_expressions() {
    let input = r#"
        ~numbers is [1, 2, 3, 4, 5]
        map (filter ~numbers |~n (~n % 2 == 0)|) |~x (~x * (~x + 1) / 2)|
    "#;
    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_program(program).unwrap();
    assert_eq!(result.to_string(), "[3, 10]");
}

#[test]
fn test_ultimate_one_liner() {
    let input = r#"reduce (map (filter [1, 2, 3, 4, 5, 6, 7, 8, 9, 10] |~n (~n % 2 == 0)|) |~x (~x * ~x)|) |~a ~b (~a + ~b)| 0"#;
    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_program(program).unwrap();
    assert_eq!(result.to_string(), "220");
}

#[test]
fn test_five_level_nested_map() {
    let input = r#"
        ~x is 1
        map (map (map (map (map [~x] |~a (~a + 1)|) |~b (~b + 1)|) |~c (~c + 1)|) |~d (~d + 1)|) |~e (~e + 1)|
    "#;
    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_program(program).unwrap();
    assert_eq!(result.to_string(), "[6]");
}

#[test]
fn test_nested_with_property_access() {
    let input = r#"
        ~objects is [{"value": 10}, {"value": 20}, {"value": 30}]
        map (filter ~objects |~obj (~obj.value > 15)|) |~item (~item.value * 2)|
    "#;
    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_program(program).unwrap();
    assert_eq!(result.to_string(), "[40, 60]");
}
