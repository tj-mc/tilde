use tilde::{parser::Parser, evaluator::Evaluator};

#[test]
fn test_merge_objects() {
    let input = r#"
        ~obj1 is {"name": "John", "age": 30}
        ~obj2 is {"city": "New York", "age": 35}
        ~merged is merge ~obj1 ~obj2
        ~name is ~merged.name
        ~age is ~merged.age
        ~city is ~merged.city
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    evaluator.eval_program(program).unwrap();

    assert_eq!(evaluator.get_variable("name").unwrap().to_string(), "John");
    assert_eq!(evaluator.get_variable("age").unwrap().to_string(), "35"); // obj2 overwrites obj1
    assert_eq!(evaluator.get_variable("city").unwrap().to_string(), "New York");
}

#[test]
fn test_pick_fields() {
    let input = r#"
        ~original is {"name": "Alice", "age": 25, "city": "Boston", "country": "USA"}
        ~picked is pick ~original ["name", "city"]
        ~name is ~picked.name
        ~city is ~picked.city
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    evaluator.eval_program(program).unwrap();

    assert_eq!(evaluator.get_variable("name").unwrap().to_string(), "Alice");
    assert_eq!(evaluator.get_variable("city").unwrap().to_string(), "Boston");

    // Should not have age or country
    if let Some(tilde::value::Value::Object(obj)) = evaluator.get_variable("picked") {
        assert!(!obj.contains_key("age"));
        assert!(!obj.contains_key("country"));
        assert_eq!(obj.len(), 2);
    } else {
        panic!("Expected an object");
    }
}

#[test]
fn test_omit_fields() {
    let input = r#"
        ~original is {"name": "Bob", "age": 40, "password": "secret", "email": "bob@example.com"}
        ~safe is omit ~original ["password"]
        ~name is ~safe.name
        ~age is ~safe.age
        ~email is ~safe.email
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    evaluator.eval_program(program).unwrap();

    assert_eq!(evaluator.get_variable("name").unwrap().to_string(), "Bob");
    assert_eq!(evaluator.get_variable("age").unwrap().to_string(), "40");
    assert_eq!(evaluator.get_variable("email").unwrap().to_string(), "bob@example.com");

    // Should not have password
    if let Some(tilde::value::Value::Object(obj)) = evaluator.get_variable("safe") {
        assert!(!obj.contains_key("password"));
        assert_eq!(obj.len(), 3);
    } else {
        panic!("Expected an object");
    }
}

#[test]
fn test_object_get() {
    let input = r#"
        ~obj is {"user": {"profile": {"name": "Charlie"}}}
        ~name is object-get ~obj "user.profile.name"
        ~missing is object-get ~obj "user.settings.theme"
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    evaluator.eval_program(program).unwrap();

    assert_eq!(evaluator.get_variable("name").unwrap().to_string(), "Charlie");

    // Missing path should return null
    if let Some(tilde::value::Value::Null) = evaluator.get_variable("missing") {
        // This is expected
    } else {
        panic!("Expected null for missing path");
    }
}

#[test]
fn test_object_set() {
    let input = r#"
        ~obj is {"user": {"profile": {"name": "David"}}}
        ~updated is object-set ~obj "user.profile.age" 28
        ~age is object-get ~updated "user.profile.age"
        ~name is object-get ~updated "user.profile.name"
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    evaluator.eval_program(program).unwrap();

    assert_eq!(evaluator.get_variable("age").unwrap().to_string(), "28");
    assert_eq!(evaluator.get_variable("name").unwrap().to_string(), "David");
}

#[test]
fn test_object_deep_merge() {
    let input = r#"
        ~obj1 is {"user": {"profile": {"name": "Eve"}, "settings": {"theme": "dark"}}}
        ~obj2 is {"user": {"profile": {"age": 30}, "permissions": ["read"]}}
        ~merged is deep-merge ~obj1 ~obj2
        ~name is object-get ~merged "user.profile.name"
        ~age is object-get ~merged "user.profile.age"
        ~theme is object-get ~merged "user.settings.theme"
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    evaluator.eval_program(program).unwrap();

    assert_eq!(evaluator.get_variable("name").unwrap().to_string(), "Eve");
    assert_eq!(evaluator.get_variable("age").unwrap().to_string(), "30");
    assert_eq!(evaluator.get_variable("theme").unwrap().to_string(), "dark");
}

#[test]
fn test_merge_error_non_object() {
    let input = r#"
        ~obj1 is {"name": "test"}
        ~not_obj is "string"
        ~result is merge ~obj1 ~not_obj
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    // This should result in an error
    let result = evaluator.eval_program(program);
    assert!(result.is_err());
    let error_msg = result.unwrap_err();
    assert!(error_msg.contains("must be objects"));
}

#[test]
fn test_pick_error_non_list() {
    let input = r#"
        ~obj is {"name": "test", "age": 25}
        ~result is pick ~obj "name"
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    // This should result in an error
    let result = evaluator.eval_program(program);
    assert!(result.is_err());
    let error_msg = result.unwrap_err();
    assert!(error_msg.contains("must be a list"));
}