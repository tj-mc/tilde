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

// ============================================================================
// ERROR CASE TESTS
// ============================================================================

#[test]
fn test_map_errors() {
    let mut evaluator = Evaluator::new();

    // Test: Wrong number of arguments
    let program_text = "~result is *.map [1, 2, 3]";
    let mut parser = Parser::new(program_text);
    let program = parser.parse().unwrap();
    let result = evaluator.eval_program(program);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("map requires exactly 2 arguments"));

    // Test: First argument not a list
    let program_text = "~result is *.map \"not a list\" double";
    let mut parser = Parser::new(program_text);
    let program = parser.parse().unwrap();
    let result = evaluator.eval_program(program);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("map first argument must be a list"));

    // Test: Function doesn't exist
    let program_text = "~result is *.map [1, 2, 3] nonexistent";
    let mut parser = Parser::new(program_text);
    let program = parser.parse().unwrap();
    let result = evaluator.eval_program(program);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Action 'nonexistent' not found"));

    // Test: Function has wrong number of parameters
    let mut evaluator = Evaluator::new();
    let program_text = r#"
action bad_func ~a ~b (give ~a + ~b)
~result is *.map [1, 2, 3] bad_func
"#;
    let mut parser = Parser::new(program_text);
    let program = parser.parse().unwrap();
    let result = evaluator.eval_program(program);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("must take exactly one parameter"));
}

#[test]
fn test_filter_errors() {
    let mut evaluator = Evaluator::new();

    // Test: Wrong number of arguments
    let program_text = "~result is *.filter [1, 2, 3]";
    let mut parser = Parser::new(program_text);
    let program = parser.parse().unwrap();
    let result = evaluator.eval_program(program);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("filter requires exactly 2 arguments"));

    // Test: First argument not a list
    let program_text = "~result is *.filter \"not a list\" is_even";
    let mut parser = Parser::new(program_text);
    let program = parser.parse().unwrap();
    let result = evaluator.eval_program(program);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("filter first argument must be a list"));
}

#[test]
fn test_reduce_errors() {
    let mut evaluator = Evaluator::new();

    // Test: Wrong number of arguments
    let program_text = "~result is *.reduce [1, 2, 3] add";
    let mut parser = Parser::new(program_text);
    let program = parser.parse().unwrap();
    let result = evaluator.eval_program(program);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("reduce requires exactly 3 arguments"));

    // Test: Function needs two parameters
    let mut evaluator = Evaluator::new();
    let program_text = r#"
action bad_func ~x (give ~x)
~result is *.reduce [1, 2, 3] bad_func 0
"#;
    let mut parser = Parser::new(program_text);
    let program = parser.parse().unwrap();
    let result = evaluator.eval_program(program);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("must take exactly two parameters"));
}

#[test]
fn test_sort_errors() {
    let mut evaluator = Evaluator::new();

    // Test: Wrong number of arguments
    let program_text = "~result is *.sort [1, 2, 3] extra";
    let mut parser = Parser::new(program_text);
    let program = parser.parse().unwrap();
    let result = evaluator.eval_program(program);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("sort requires exactly 1 argument"));

    // Test: Argument not a list
    let program_text = "~result is *.sort \"not a list\"";
    let mut parser = Parser::new(program_text);
    let program = parser.parse().unwrap();
    let result = evaluator.eval_program(program);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("sort argument must be a list"));
}

// ============================================================================
// EDGE CASE TESTS
// ============================================================================

#[test]
fn test_empty_list_operations() {
    let mut evaluator = Evaluator::new();

    // Test: Map with empty list
    let program_text = r#"
action double ~x (give ~x * 2)
~result is *.map [] double
say ~result
"#;
    let mut parser = Parser::new(program_text);
    let program = parser.parse().unwrap();
    let result = evaluator.eval_program(program).unwrap();
    if let Value::String(output) = result {
        assert!(output.contains("[]"));
    }

    // Test: Filter with empty list
    let mut evaluator = Evaluator::new();
    let program_text = r#"
action is_even ~x (give ~x % 2 == 0)
~result is *.filter [] is_even
say ~result
"#;
    let mut parser = Parser::new(program_text);
    let program = parser.parse().unwrap();
    let result = evaluator.eval_program(program).unwrap();
    if let Value::String(output) = result {
        assert!(output.contains("[]"));
    }

    // Test: Sort with empty list
    let mut evaluator = Evaluator::new();
    let program_text = "~result is *.sort []\nsay ~result";
    let mut parser = Parser::new(program_text);
    let program = parser.parse().unwrap();
    let result = evaluator.eval_program(program).unwrap();
    if let Value::String(output) = result {
        assert!(output.contains("[]"));
    }
}

#[test]
fn test_single_element_lists() {
    let mut evaluator = Evaluator::new();

    // Test: Map with single element
    let program_text = r#"
action double ~x (give ~x * 2)
~result is *.map [5] double
say ~result
"#;
    let mut parser = Parser::new(program_text);
    let program = parser.parse().unwrap();
    let result = evaluator.eval_program(program).unwrap();
    if let Value::String(output) = result {
        assert!(output.contains("10"));
    }

    // Test: Reduce with single element
    let mut evaluator = Evaluator::new();
    let program_text = r#"
action add ~a ~b (give ~a + ~b)
~result is *.reduce [5] add 0
say ~result
"#;
    let mut parser = Parser::new(program_text);
    let program = parser.parse().unwrap();
    let result = evaluator.eval_program(program).unwrap();
    if let Value::String(output) = result {
        assert!(output.contains("5"));
    }
}

#[test]
fn test_string_operations_edge_cases() {
    let mut evaluator = Evaluator::new();

    // Test: Split with empty string
    let program_text = "~result is *.split \"\" \",\"\nsay ~result";
    let mut parser = Parser::new(program_text);
    let program = parser.parse().unwrap();
    let result = evaluator.eval_program(program).unwrap();
    if let Value::String(output) = result {
        assert!(output.contains("[]")); // Empty string becomes empty list when split
    }

    // Test: Join empty list
    let mut evaluator = Evaluator::new();
    let program_text = "~result is *.join [] \",\"\nsay ~result";
    let mut parser = Parser::new(program_text);
    let program = parser.parse().unwrap();
    let result = evaluator.eval_program(program).unwrap();
    if let Value::String(output) = result {
        assert!(output.contains("\"\"") || output == ""); // Should be empty string
    }

    // Test: Trim string with only whitespace
    let mut evaluator = Evaluator::new();
    let program_text = "~result is *.trim \"   \"\nsay \"'\" ~result \"'\"";
    let mut parser = Parser::new(program_text);
    let program = parser.parse().unwrap();
    let result = evaluator.eval_program(program).unwrap();
    if let Value::String(output) = result {
        assert!(output.contains("''"));
    }
}

#[test]
fn test_math_operations_edge_cases() {
    let mut evaluator = Evaluator::new();

    // Test: Square root of 0
    let program_text = "~result is *.square-root 0\nsay ~result";
    let mut parser = Parser::new(program_text);
    let program = parser.parse().unwrap();
    let result = evaluator.eval_program(program).unwrap();
    if let Value::String(output) = result {
        assert!(output.contains("0"));
    }

    // Test: Square root of 1
    let mut evaluator = Evaluator::new();
    let program_text = "~result is *.square-root 1\nsay ~result";
    let mut parser = Parser::new(program_text);
    let program = parser.parse().unwrap();
    let result = evaluator.eval_program(program).unwrap();
    if let Value::String(output) = result {
        assert!(output.contains("1"));
    }

    // Test: Absolute of 0
    let mut evaluator = Evaluator::new();
    let program_text = "~result is *.absolute 0\nsay ~result";
    let mut parser = Parser::new(program_text);
    let program = parser.parse().unwrap();
    let result = evaluator.eval_program(program).unwrap();
    if let Value::String(output) = result {
        assert!(output.contains("0"));
    }
}