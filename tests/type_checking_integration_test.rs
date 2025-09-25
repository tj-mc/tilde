use tilde::{evaluator::Evaluator, parser::Parser};

#[test]
fn test_is_number() {
    let input = r#"
        ~result1 is is-number 42
        ~result2 is is-number "hello"
        ~result3 is is-number true
        ~result4 is is-number [1, 2, 3]
        ~result5 is is-number {"key": "value"}
        ~result6 is is-number 3.14
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    evaluator.eval_program(program).unwrap();

    assert_eq!(
        evaluator.get_variable("result1").unwrap().to_string(),
        "true"
    );
    assert_eq!(
        evaluator.get_variable("result2").unwrap().to_string(),
        "false"
    );
    assert_eq!(
        evaluator.get_variable("result3").unwrap().to_string(),
        "false"
    );
    assert_eq!(
        evaluator.get_variable("result4").unwrap().to_string(),
        "false"
    );
    assert_eq!(
        evaluator.get_variable("result5").unwrap().to_string(),
        "false"
    );
    assert_eq!(
        evaluator.get_variable("result6").unwrap().to_string(),
        "true"
    );
}

#[test]
fn test_is_string() {
    let input = r#"
        ~result1 is is-string "hello"
        ~result2 is is-string 42
        ~result3 is is-string true
        ~result4 is is-string [1, 2, 3]
        ~result5 is is-string {"key": "value"}
        ~result6 is is-string ""
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    evaluator.eval_program(program).unwrap();

    assert_eq!(
        evaluator.get_variable("result1").unwrap().to_string(),
        "true"
    );
    assert_eq!(
        evaluator.get_variable("result2").unwrap().to_string(),
        "false"
    );
    assert_eq!(
        evaluator.get_variable("result3").unwrap().to_string(),
        "false"
    );
    assert_eq!(
        evaluator.get_variable("result4").unwrap().to_string(),
        "false"
    );
    assert_eq!(
        evaluator.get_variable("result5").unwrap().to_string(),
        "false"
    );
    assert_eq!(
        evaluator.get_variable("result6").unwrap().to_string(),
        "true"
    );
}

#[test]
fn test_is_boolean() {
    let input = r#"
        ~result1 is is-boolean true
        ~result2 is is-boolean false
        ~result3 is is-boolean "hello"
        ~result4 is is-boolean 42
        ~result5 is is-boolean [1, 2, 3]
        ~result6 is is-boolean {"key": "value"}
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    evaluator.eval_program(program).unwrap();

    assert_eq!(
        evaluator.get_variable("result1").unwrap().to_string(),
        "true"
    );
    assert_eq!(
        evaluator.get_variable("result2").unwrap().to_string(),
        "true"
    );
    assert_eq!(
        evaluator.get_variable("result3").unwrap().to_string(),
        "false"
    );
    assert_eq!(
        evaluator.get_variable("result4").unwrap().to_string(),
        "false"
    );
    assert_eq!(
        evaluator.get_variable("result5").unwrap().to_string(),
        "false"
    );
    assert_eq!(
        evaluator.get_variable("result6").unwrap().to_string(),
        "false"
    );
}

#[test]
fn test_is_list() {
    let input = r#"
        ~result1 is is-list [1, 2, 3]
        ~result2 is is-list []
        ~result3 is is-list "hello"
        ~result4 is is-list 42
        ~result5 is is-list true
        ~result6 is is-list {"key": "value"}
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    evaluator.eval_program(program).unwrap();

    assert_eq!(
        evaluator.get_variable("result1").unwrap().to_string(),
        "true"
    );
    assert_eq!(
        evaluator.get_variable("result2").unwrap().to_string(),
        "true"
    );
    assert_eq!(
        evaluator.get_variable("result3").unwrap().to_string(),
        "false"
    );
    assert_eq!(
        evaluator.get_variable("result4").unwrap().to_string(),
        "false"
    );
    assert_eq!(
        evaluator.get_variable("result5").unwrap().to_string(),
        "false"
    );
    assert_eq!(
        evaluator.get_variable("result6").unwrap().to_string(),
        "false"
    );
}

#[test]
fn test_is_object() {
    let input = r#"
        ~result1 is is-object {"key": "value"}
        ~result2 is is-object {}
        ~result3 is is-object [1, 2, 3]
        ~result4 is is-object "hello"
        ~result5 is is-object 42
        ~result6 is is-object true
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    evaluator.eval_program(program).unwrap();

    assert_eq!(
        evaluator.get_variable("result1").unwrap().to_string(),
        "true"
    );
    assert_eq!(
        evaluator.get_variable("result2").unwrap().to_string(),
        "true"
    );
    assert_eq!(
        evaluator.get_variable("result3").unwrap().to_string(),
        "false"
    );
    assert_eq!(
        evaluator.get_variable("result4").unwrap().to_string(),
        "false"
    );
    assert_eq!(
        evaluator.get_variable("result5").unwrap().to_string(),
        "false"
    );
    assert_eq!(
        evaluator.get_variable("result6").unwrap().to_string(),
        "false"
    );
}

#[test]
fn test_is_empty() {
    let input = r#"
        ~result1 is is-empty ""
        ~result2 is is-empty []
        ~result3 is is-empty {}
        ~result4 is is-empty "hello"
        ~result5 is is-empty [1, 2, 3]
        ~result6 is is-empty {"key": "value"}
        ~result7 is is-empty 0
        ~result8 is is-empty false
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    evaluator.eval_program(program).unwrap();

    assert_eq!(
        evaluator.get_variable("result1").unwrap().to_string(),
        "true"
    );
    assert_eq!(
        evaluator.get_variable("result2").unwrap().to_string(),
        "true"
    );
    assert_eq!(
        evaluator.get_variable("result3").unwrap().to_string(),
        "true"
    );
    assert_eq!(
        evaluator.get_variable("result4").unwrap().to_string(),
        "false"
    );
    assert_eq!(
        evaluator.get_variable("result5").unwrap().to_string(),
        "false"
    );
    assert_eq!(
        evaluator.get_variable("result6").unwrap().to_string(),
        "false"
    );
    assert_eq!(
        evaluator.get_variable("result7").unwrap().to_string(),
        "false"
    );
    assert_eq!(
        evaluator.get_variable("result8").unwrap().to_string(),
        "false"
    );
}

#[test]
fn test_is_defined() {
    let input = r#"
        ~defined_var is 42
        ~result1 is is-defined ~defined_var
        ~result2 is is-defined ~undefined_var
        ~result3 is is-defined "hello"
        ~result4 is is-defined 0
        ~result5 is is-defined false
        ~result6 is is-defined ""
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    evaluator.eval_program(program).unwrap();

    assert_eq!(
        evaluator.get_variable("result1").unwrap().to_string(),
        "true"
    );
    assert_eq!(
        evaluator.get_variable("result2").unwrap().to_string(),
        "false"
    );
    assert_eq!(
        evaluator.get_variable("result3").unwrap().to_string(),
        "true"
    );
    assert_eq!(
        evaluator.get_variable("result4").unwrap().to_string(),
        "true"
    );
    assert_eq!(
        evaluator.get_variable("result5").unwrap().to_string(),
        "true"
    );
    assert_eq!(
        evaluator.get_variable("result6").unwrap().to_string(),
        "true"
    );
}
