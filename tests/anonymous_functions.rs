use tilde::evaluator::Evaluator;
use tilde::parser::Parser;

#[test]
fn test_map_with_anonymous_function() {
    let input = r#"
        ~numbers is [1, 2, 3, 4, 5]
        ~doubled is map ~numbers |~x (~x * 2)|
        ~doubled
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_program(program).unwrap();

    assert_eq!(result.to_string(), "[2, 4, 6, 8, 10]");
}

#[test]
fn test_filter_with_anonymous_function() {
    let input = r#"
        ~numbers is [1, 2, 3, 4, 5, 6]
        ~evens is filter ~numbers |~n (~n % 2 == 0)|
        ~evens
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_program(program).unwrap();

    assert_eq!(result.to_string(), "[2, 4, 6]");
}

#[test]
fn test_reduce_with_anonymous_function() {
    let input = r#"
        ~numbers is [1, 2, 3, 4, 5]
        ~sum is reduce ~numbers |~a ~b (~a + ~b)| 0
        ~sum
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_program(program).unwrap();

    assert_eq!(result.to_string(), "15");
}

#[test]
fn test_anonymous_function_with_property_access() {
    let input = r#"
        ~orders is [
            {"id": 1, "amount": 100},
            {"id": 2, "amount": 200},
            {"id": 3, "amount": 150}
        ]
        ~ids is map ~orders |~order (~order.id)|
        ~ids
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_program(program).unwrap();

    assert_eq!(result.to_string(), "[1, 2, 3]");
}

#[test]
fn test_anonymous_function_closure() {
    let input = r#"
        ~multiplier is 10
        ~numbers is [1, 2, 3]
        ~result is map ~numbers |~x (~x * ~multiplier)|
        ~result
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_program(program).unwrap();

    assert_eq!(result.to_string(), "[10, 20, 30]");
}

#[test]
fn test_anonymous_function_parameter_shadowing() {
    let input = r#"
        ~x is 100
        ~numbers is [1, 2, 3]
        ~result is map ~numbers |~x (~x * 2)|
        ~x
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_program(program).unwrap();

    // Outer ~x should remain unchanged
    assert_eq!(result.to_string(), "100");
}

#[test]
fn test_anonymous_function_scope_isolation() {
    let input = r#"
        ~numbers is [1, 2, 3]
        ~result is map ~numbers |~inner (~inner * 2)|
        ~inner
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_program(program);

    // ~inner should not be accessible outside the anonymous function
    assert!(result.is_err());
}

#[test]
fn test_nested_anonymous_functions() {
    let input = r#"
        ~matrix is [[1, 2], [3, 4], [5, 6]]
        ~doubled is map ~matrix |~row (map ~row |~x (~x * 2)|)|
        ~doubled
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_program(program).unwrap();

    assert_eq!(result.to_string(), "[[2, 4], [6, 8], [10, 12]]");
}

#[test]
fn test_chained_operations_with_anonymous_functions() {
    let input = r#"
        ~numbers is [1, 2, 3, 4, 5, 6]
        ~result is map (filter ~numbers |~n (~n % 2 == 0)|) |~x (~x * ~x)|
        ~result
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_program(program).unwrap();

    // Filter evens [2, 4, 6], then square them
    assert_eq!(result.to_string(), "[4, 16, 36]");
}

#[test]
fn test_find_with_anonymous_function() {
    let input = r#"
        ~numbers is [1, 3, 5, 7, 8, 9]
        ~first_even is find ~numbers |~n (~n % 2 == 0)|
        ~first_even
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_program(program).unwrap();

    assert_eq!(result.to_string(), "8");
}

#[test]
fn test_every_with_anonymous_function() {
    let input = r#"
        ~numbers is [2, 4, 6, 8]
        ~all_even is every ~numbers |~n (~n % 2 == 0)|
        ~all_even
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_program(program).unwrap();

    assert_eq!(result.to_string(), "true");
}

#[test]
fn test_some_with_anonymous_function() {
    let input = r#"
        ~numbers is [1, 3, 5, 8, 9]
        ~has_even is some ~numbers |~n (~n % 2 == 0)|
        ~has_even
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_program(program).unwrap();

    assert_eq!(result.to_string(), "true");
}

#[test]
fn test_sort_by_with_anonymous_function() {
    let input = r#"
        ~words is ["cat", "elephant", "dog", "bird"]
        ~sorted is sort-by ~words |~w (length ~w)|
        ~sorted
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_program(program).unwrap();

    assert_eq!(result.to_string(), "[cat, dog, bird, elephant]");
}

#[test]
fn test_group_by_with_anonymous_function() {
    let input = r#"
        ~numbers is [1, 2, 3, 4, 5, 6]
        ~grouped is group-by ~numbers |~n (~n % 2 == 0)|
        ~grouped.true
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_program(program).unwrap();

    assert_eq!(result.to_string(), "[2, 4, 6]");
}

#[test]
fn test_partition_with_anonymous_function() {
    let input = r#"
        ~numbers is [1, 2, 3, 4, 5, 6]
        ~parts is partition ~numbers |~n (~n > 3)|
        ~parts.matched
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_program(program).unwrap();

    assert_eq!(result.to_string(), "[4, 5, 6]");
}
