use tails::{evaluator::Evaluator, parser::Parser};

#[test]
fn test_date_comparison_basic() {
    let input = r#"
    ~earlier is date "2024-03-15T10:00:00Z"
    ~later is date "2024-03-15T14:00:00Z"

    ~is_before is date-before ~earlier ~later
    ~is_after is date-after ~earlier ~later
    ~is_equal is date-equal ~earlier ~later

    ~reverse_before is date-before ~later ~earlier
    ~reverse_after is date-after ~later ~earlier
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_ok());

    let is_before = evaluator.get_variable("is_before").unwrap();
    let is_after = evaluator.get_variable("is_after").unwrap();
    let is_equal = evaluator.get_variable("is_equal").unwrap();
    let reverse_before = evaluator.get_variable("reverse_before").unwrap();
    let reverse_after = evaluator.get_variable("reverse_after").unwrap();

    assert_eq!(is_before.to_string(), "true");
    assert_eq!(is_after.to_string(), "false");
    assert_eq!(is_equal.to_string(), "false");
    assert_eq!(reverse_before.to_string(), "false");
    assert_eq!(reverse_after.to_string(), "true");
}

#[test]
fn test_date_equality() {
    let input = r#"
    ~date1 is date "2024-03-15T14:30:00Z"
    ~date2 is date "2024-03-15T14:30:00Z"
    ~date3 is date "2024-03-15T16:30:00+02:00"  # Same moment in different timezone
    ~date4 is date "2024-03-15T14:30:01Z"       # One second different

    ~same_exact is date-equal ~date1 ~date2
    ~same_timezone_converted is date-equal ~date1 ~date3
    ~different_second is date-equal ~date1 ~date4
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_ok());

    let same_exact = evaluator.get_variable("same_exact").unwrap();
    let same_timezone_converted = evaluator.get_variable("same_timezone_converted").unwrap();
    let different_second = evaluator.get_variable("different_second").unwrap();

    assert_eq!(same_exact.to_string(), "true");
    assert_eq!(same_timezone_converted.to_string(), "true");
    assert_eq!(different_second.to_string(), "false");
}

#[test]
fn test_date_comparison_with_dates_only() {
    let input = r#"
    ~yesterday is date "2024-03-14"
    ~today is date "2024-03-15"
    ~tomorrow is date "2024-03-16"

    ~yesterday_before_today is date-before ~yesterday ~today
    ~tomorrow_after_today is date-after ~tomorrow ~today
    ~today_equal_today is date-equal ~today ~today
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_ok());

    let yesterday_before_today = evaluator.get_variable("yesterday_before_today").unwrap();
    let tomorrow_after_today = evaluator.get_variable("tomorrow_after_today").unwrap();
    let today_equal_today = evaluator.get_variable("today_equal_today").unwrap();

    assert_eq!(yesterday_before_today.to_string(), "true");
    assert_eq!(tomorrow_after_today.to_string(), "true");
    assert_eq!(today_equal_today.to_string(), "true");
}

#[test]
fn test_date_comparison_in_conditionals() {
    let input = r#"
    ~deadline is date "2024-12-31"
    ~current is date "2024-03-15"

    if date-before ~current ~deadline (
        ~status is "on-time"
    ) else (
        ~status is "overdue"
    )

    if date-after ~current ~deadline (
        ~urgency is "past-due"
    ) else (
        ~urgency is "still-time"
    )
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_ok());

    let status = evaluator.get_variable("status").unwrap();
    let urgency = evaluator.get_variable("urgency").unwrap();

    assert_eq!(status.to_string(), "on-time");
    assert_eq!(urgency.to_string(), "still-time");
}

#[test]
fn test_date_comparison_error_handling() {
    let input = r#"
    ~bad_before is date-before "not-a-date" "also-not-a-date"
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("date-before first argument must be a date"));
}

#[test]
fn test_date_comparison_year_boundaries() {
    let input = r#"
    ~end_of_year is date "2023-12-31T23:59:59Z"
    ~start_of_year is date "2024-01-01T00:00:00Z"

    ~is_before_new_year is date-before ~end_of_year ~start_of_year
    ~is_after_new_year is date-after ~start_of_year ~end_of_year
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_ok());

    let is_before_new_year = evaluator.get_variable("is_before_new_year").unwrap();
    let is_after_new_year = evaluator.get_variable("is_after_new_year").unwrap();

    assert_eq!(is_before_new_year.to_string(), "true");
    assert_eq!(is_after_new_year.to_string(), "true");
}