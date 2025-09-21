use tilde::{evaluator::Evaluator, parser::Parser};

#[test]
fn test_date_format_basic() {
    let input = r#"
    ~meeting is date "2024-03-15T14:30:00Z"
    ~pretty is date-format ~meeting "%B %d, %Y at %I:%M %p"
    ~simple is date-format ~meeting "%Y-%m-%d"
    ~time_only is date-format ~meeting "%H:%M:%S"
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_ok());

    let pretty = evaluator.get_variable("pretty").unwrap();
    let simple = evaluator.get_variable("simple").unwrap();
    let time_only = evaluator.get_variable("time_only").unwrap();

    assert_eq!(pretty.to_string(), "March 15, 2024 at 02:30 PM");
    assert_eq!(simple.to_string(), "2024-03-15");
    assert_eq!(time_only.to_string(), "14:30:00");
}

#[test]
fn test_date_parse_basic() {
    let input = r#"
    ~parsed1 is date-parse "March 15, 2024" "%B %d, %Y"
    ~parsed2 is date-parse "15/03/2024" "%d/%m/%Y"
    ~parsed3 is date-parse "2024-03-15 14:30" "%Y-%m-%d %H:%M"
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_ok());

    let parsed1 = evaluator.get_variable("parsed1").unwrap();
    let parsed2 = evaluator.get_variable("parsed2").unwrap();
    let parsed3 = evaluator.get_variable("parsed3").unwrap();

    assert_eq!(parsed1.to_string(), "2024-03-15T00:00:00Z");
    assert_eq!(parsed2.to_string(), "2024-03-15T00:00:00Z");
    assert_eq!(parsed3.to_string(), "2024-03-15T14:30:00Z");
}

#[test]
fn test_date_format_parse_roundtrip() {
    let input = r#"
    ~original is date "2024-03-15T14:30:00Z"
    ~formatted is date-format ~original "%Y-%m-%d %H:%M:%S"
    ~parsed_back is date-parse ~formatted "%Y-%m-%d %H:%M:%S"
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_ok());

    let original = evaluator.get_variable("original").unwrap();
    let formatted = evaluator.get_variable("formatted").unwrap();
    let parsed_back = evaluator.get_variable("parsed_back").unwrap();

    assert_eq!(formatted.to_string(), "2024-03-15 14:30:00");
    assert_eq!(original, parsed_back);
}

#[test]
fn test_date_format_various_patterns() {
    let input = r#"
    ~christmas is date "2024-12-25T18:30:00Z"
    ~iso is date-format ~christmas "%Y-%m-%dT%H:%M:%SZ"
    ~us is date-format ~christmas "%m/%d/%Y"
    ~eu is date-format ~christmas "%d/%m/%Y"
    ~weekday is date-format ~christmas "%A, %B %d"
    ~short is date-format ~christmas "%b %d, %y"
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_ok());

    let iso = evaluator.get_variable("iso").unwrap();
    let us = evaluator.get_variable("us").unwrap();
    let eu = evaluator.get_variable("eu").unwrap();
    let weekday = evaluator.get_variable("weekday").unwrap();
    let short = evaluator.get_variable("short").unwrap();

    assert_eq!(iso.to_string(), "2024-12-25T18:30:00Z");
    assert_eq!(us.to_string(), "12/25/2024");
    assert_eq!(eu.to_string(), "25/12/2024");
    assert_eq!(weekday.to_string(), "Wednesday, December 25");
    assert_eq!(short.to_string(), "Dec 25, 24");
}

#[test]
fn test_date_parse_error_handling() {
    let input = r#"
    ~bad_parse is date-parse "invalid date" "%Y-%m-%d"
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Failed to parse"));
}

#[test]
fn test_date_format_error_handling() {
    let input = r#"
    ~bad_format is date-format "not-a-date" "%Y-%m-%d"
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("date-format first argument must be a date"));
}