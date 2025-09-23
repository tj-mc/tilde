use tilde::{parser::Parser, evaluator::Evaluator};

#[test]
fn test_starts_with() {
    let input = r#"
        ~text is "hello world"
        ~result1 is starts-with ~text "hello"
        ~result2 is starts-with ~text "world"
        ~result3 is starts-with ~text ""
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    evaluator.eval_program(program).unwrap();

    assert_eq!(evaluator.get_variable("result1").unwrap().to_string(), "true");
    assert_eq!(evaluator.get_variable("result2").unwrap().to_string(), "false");
    assert_eq!(evaluator.get_variable("result3").unwrap().to_string(), "true");
}

#[test]
fn test_ends_with() {
    let input = r#"
        ~text is "hello world"
        ~result1 is ends-with ~text "world"
        ~result2 is ends-with ~text "hello"
        ~result3 is ends-with ~text ""
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    evaluator.eval_program(program).unwrap();

    assert_eq!(evaluator.get_variable("result1").unwrap().to_string(), "true");
    assert_eq!(evaluator.get_variable("result2").unwrap().to_string(), "false");
    assert_eq!(evaluator.get_variable("result3").unwrap().to_string(), "true");
}

#[test]
fn test_substring() {
    let input = r#"
        ~text is "hello world"
        ~result1 is substring ~text 0 5
        ~result2 is substring ~text 6
        ~result3 is substring ~text 6 11
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    evaluator.eval_program(program).unwrap();

    assert_eq!(evaluator.get_variable("result1").unwrap().to_string(), "hello");
    assert_eq!(evaluator.get_variable("result2").unwrap().to_string(), "world");
    assert_eq!(evaluator.get_variable("result3").unwrap().to_string(), "world");
}

#[test]
fn test_replace() {
    let input = r#"
        ~text is "hello world hello"
        ~result1 is replace ~text "hello" "hi"
        ~result2 is replace ~text "world" "universe"
        ~result3 is replace ~text "xyz" "abc"
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    evaluator.eval_program(program).unwrap();

    assert_eq!(evaluator.get_variable("result1").unwrap().to_string(), "hi world hi");
    assert_eq!(evaluator.get_variable("result2").unwrap().to_string(), "hello universe hello");
    assert_eq!(evaluator.get_variable("result3").unwrap().to_string(), "hello world hello");
}

#[test]
fn test_repeat() {
    let input = r#"
        ~text is "hi"
        ~result1 is repeat ~text 3
        ~result2 is repeat ~text 0
        ~result3 is repeat ~text 1
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    evaluator.eval_program(program).unwrap();

    assert_eq!(evaluator.get_variable("result1").unwrap().to_string(), "hihihi");
    assert_eq!(evaluator.get_variable("result2").unwrap().to_string(), "");
    assert_eq!(evaluator.get_variable("result3").unwrap().to_string(), "hi");
}

#[test]
fn test_pad_left() {
    let input = r#"
        ~text is "hi"
        ~result1 is pad-left ~text 5
        ~result2 is pad-left ~text 5 "*"
        ~result3 is pad-left ~text 2
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    evaluator.eval_program(program).unwrap();

    assert_eq!(evaluator.get_variable("result1").unwrap().to_string(), "   hi");
    assert_eq!(evaluator.get_variable("result2").unwrap().to_string(), "***hi");
    assert_eq!(evaluator.get_variable("result3").unwrap().to_string(), "hi");
}

#[test]
fn test_pad_right() {
    let input = r#"
        ~text is "hi"
        ~result1 is pad-right ~text 5
        ~result2 is pad-right ~text 5 "*"
        ~result3 is pad-right ~text 2
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    evaluator.eval_program(program).unwrap();

    assert_eq!(evaluator.get_variable("result1").unwrap().to_string(), "hi   ");
    assert_eq!(evaluator.get_variable("result2").unwrap().to_string(), "hi***");
    assert_eq!(evaluator.get_variable("result3").unwrap().to_string(), "hi");
}