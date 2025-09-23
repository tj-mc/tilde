use tilde::{parser::Parser, evaluator::Evaluator};

#[test]
fn test_to_json() {
    let input = r#"
        ~str_result is to-json "hello"
        ~num_result is to-json 42
        ~bool_result is to-json true
        ~list_result is to-json [1, 2, "three"]
        ~obj_result is to-json {"name": "John", "age": 30}
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    evaluator.eval_program(program).unwrap();

    assert_eq!(evaluator.get_variable("str_result").unwrap().to_string(), "\"hello\"");
    assert_eq!(evaluator.get_variable("num_result").unwrap().to_string(), "42.0");
    assert_eq!(evaluator.get_variable("bool_result").unwrap().to_string(), "true");
    assert_eq!(evaluator.get_variable("list_result").unwrap().to_string(), "[1.0,2.0,\"three\"]");

    let obj_json = evaluator.get_variable("obj_result").unwrap().to_string();
    // JSON object order might vary, so just check that it contains the expected keys
    assert!(obj_json.contains("\"name\":\"John\""));
    assert!(obj_json.contains("\"age\":30.0"));
}

#[test]
fn test_from_json() {
    let input = r#"
        ~str_result is from-json "\"hello\""
        ~num_result is from-json "42.0"
        ~bool_result is from-json "true"
        ~list_result is from-json "[1,2,\"three\"]"
        ~obj_result is from-json "{\"name\":\"John\",\"age\":30}"
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    evaluator.eval_program(program).unwrap();

    assert_eq!(evaluator.get_variable("str_result").unwrap().to_string(), "hello");
    assert_eq!(evaluator.get_variable("num_result").unwrap().to_string(), "42");
    assert_eq!(evaluator.get_variable("bool_result").unwrap().to_string(), "true");

    // Check list result
    if let Some(tilde::value::Value::List(list)) = evaluator.get_variable("list_result") {
        assert_eq!(list.len(), 3);
        assert_eq!(list[0].to_string(), "1");
        assert_eq!(list[1].to_string(), "2");
        assert_eq!(list[2].to_string(), "three");
    } else {
        panic!("Expected a list");
    }

    // Check object result
    if let Some(tilde::value::Value::Object(obj)) = evaluator.get_variable("obj_result") {
        assert_eq!(obj.get("name").unwrap().to_string(), "John");
        assert_eq!(obj.get("age").unwrap().to_string(), "30");
    } else {
        panic!("Expected an object");
    }
}

#[test]
fn test_json_roundtrip() {
    let input = r#"
        ~original is {"name": "Alice", "age": 25}
        ~json_str is to-json ~original
        ~restored is from-json ~json_str
        ~alice_name is ~restored.name
        ~alice_age is ~restored.age
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    evaluator.eval_program(program).unwrap();

    assert_eq!(evaluator.get_variable("alice_name").unwrap().to_string(), "Alice");
    assert_eq!(evaluator.get_variable("alice_age").unwrap().to_string(), "25");
}

#[test]
fn test_from_json_invalid() {
    let input = r#"
        ~result is from-json "invalid json {"
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    // This should result in an error
    let result = evaluator.eval_program(program);
    assert!(result.is_err());
}