use tilde::{parser::Parser, evaluator::Evaluator};

#[test]
fn test_base64_encoding() {
    let input = r#"
        ~text is "Hello, World!"
        ~encoded is base64-encode ~text
        ~decoded is base64-decode ~encoded
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    evaluator.eval_program(program).unwrap();

    assert_eq!(evaluator.get_variable("encoded").unwrap().to_string(), "SGVsbG8sIFdvcmxkIQ==");
    assert_eq!(evaluator.get_variable("decoded").unwrap().to_string(), "Hello, World!");
}

#[test]
fn test_base64_empty_string() {
    let input = r#"
        ~empty is ""
        ~encoded is base64-encode ~empty
        ~decoded is base64-decode ~encoded
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    evaluator.eval_program(program).unwrap();

    assert_eq!(evaluator.get_variable("encoded").unwrap().to_string(), "");
    assert_eq!(evaluator.get_variable("decoded").unwrap().to_string(), "");
}

#[test]
fn test_url_encoding() {
    let input = r#"
        ~text is "Hello World & Special Characters!"
        ~encoded is url-encode ~text
        ~decoded is url-decode ~encoded
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    evaluator.eval_program(program).unwrap();

    assert_eq!(evaluator.get_variable("encoded").unwrap().to_string(), "Hello%20World%20%26%20Special%20Characters%21");
    assert_eq!(evaluator.get_variable("decoded").unwrap().to_string(), "Hello World & Special Characters!");
}

#[test]
fn test_url_encoding_aws_style() {
    let input = r#"
        ~path is "/my-bucket/my key.txt"
        ~encoded is url-encode ~path
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    evaluator.eval_program(program).unwrap();

    assert_eq!(evaluator.get_variable("encoded").unwrap().to_string(), "%2Fmy-bucket%2Fmy%20key.txt");
}

#[test]
fn test_base64_invalid_decode() {
    let input = r#"
        ~invalid is "Not base64!"
        ~result is base64-decode ~invalid
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    // This should result in an error
    let result = evaluator.eval_program(program);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid base64"));
}