use tilde::{parser::Parser, evaluator::Evaluator};

#[test]
fn test_foreach_list_single_variable() {
    let input = r#"
        ~items is ["apple", "banana", "cherry"]
        ~result is ""
        for-each ~item in ~items (
            ~result is ~result + ~item + ","
        )
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    evaluator.eval_program(program).unwrap();

    let result = evaluator.get_variable("result").unwrap();
    assert_eq!(result.to_string(), "apple,banana,cherry,");
}

#[test]
fn test_foreach_list_with_index() {
    let input = r#"
        ~items is ["a", "b", "c"]
        ~result is ""
        for-each ~item ~index in ~items (
            ~result is ~result + "`~index`:" + ~item + ","
        )
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    evaluator.eval_program(program).unwrap();

    let result = evaluator.get_variable("result").unwrap();
    assert_eq!(result.to_string(), "0:a,1:b,2:c,");
}

#[test]
fn test_foreach_object_key_value() {
    let input = r#"
        ~person is {"name": "Alice", "age": 30}
        ~result is ""
        for-each ~key ~value in ~person (
            ~result is ~result + ~key + "=" + "`~value`" + ","
        )
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    evaluator.eval_program(program).unwrap();

    let result = evaluator.get_variable("result").unwrap().to_string();
    // Object iteration order may vary, so check for both possible orders
    assert!(result.contains("name=Alice") && result.contains("age=30"));
}

#[test]
fn test_foreach_object_values_only() {
    let input = r#"
        ~data is {"x": 10, "y": 20}
        ~sum is 0
        for-each ~value in ~data (
            ~sum is ~sum + ~value
        )
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    evaluator.eval_program(program).unwrap();

    let sum = evaluator.get_variable("sum").unwrap();
    assert_eq!(sum.to_string(), "30");
}

#[test]
fn test_foreach_empty_list() {
    let input = r#"
        ~items is []
        ~count is 0
        for-each ~item in ~items (
            ~count is ~count + 1
        )
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    evaluator.eval_program(program).unwrap();

    let count = evaluator.get_variable("count").unwrap();
    assert_eq!(count.to_string(), "0");
}

#[test]
fn test_foreach_nested_loops() {
    let input = r#"
        ~matrix is [[1, 2], [3, 4]]
        ~result is ""
        for-each ~row in ~matrix (
            for-each ~item in ~row (
                ~result is ~result + "`~item`" + ","
            )
        )
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    evaluator.eval_program(program).unwrap();

    let result = evaluator.get_variable("result").unwrap();
    assert_eq!(result.to_string(), "1,2,3,4,");
}

#[test]
fn test_foreach_break_loop() {
    let input = r#"
        ~items is ["a", "b", "c", "d"]
        ~result is ""
        for-each ~item in ~items (
            ~result is ~result + ~item
            if ~item == "b" break-loop
        )
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    evaluator.eval_program(program).unwrap();

    let result = evaluator.get_variable("result").unwrap();
    assert_eq!(result.to_string(), "ab");
}

#[test]
fn test_foreach_variable_scoping() {
    let input = r#"
        ~item is "global"
        ~items is ["local1", "local2"]
        ~results is []

        for-each ~item in ~items (
            ~results is append ~results ~item
        )

        # ~item should be restored to global value
        ~results is append ~results ~item
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    evaluator.eval_program(program).unwrap();

    let results = evaluator.get_variable("results").unwrap();
    assert_eq!(results.to_string(), "[local1, local2, global]");

    let item = evaluator.get_variable("item").unwrap();
    assert_eq!(item.to_string(), "global");
}

#[test]
fn test_foreach_invalid_iterable() {
    let input = r#"
        ~number is 42
        for-each ~item in ~number (
            say ~item
        )
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("for-each can only iterate over lists and objects"));
}

#[test]
fn test_foreach_too_many_variables() {
    let input = r#"
        ~items is ["a", "b"]
        for-each ~a ~b ~c in ~items (
            say ~a
        )
    "#;

    let mut parser = Parser::new(input);
    let result = parser.parse();
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("for-each expects at most 2 variables"));
}

#[test]
fn test_foreach_parser_errors() {
    // Missing variable
    let input1 = "for-each in ~items ( say \"test\" )";
    let mut parser1 = Parser::new(input1);
    let result1 = parser1.parse();
    assert!(result1.is_err());
    assert!(result1.unwrap_err().contains("Expected variable after 'for-each'"));

    // Missing 'in' keyword
    let input2 = "for-each ~item ~items ( say ~item )";
    let mut parser2 = Parser::new(input2);
    let result2 = parser2.parse();
    assert!(result2.is_err());
}