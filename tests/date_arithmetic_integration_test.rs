use tails::{evaluator::Evaluator, parser::Parser};

#[test]
fn test_date_arithmetic_basic() {
    let input = r#"
    ~start is date "2024-03-15"
    ~plus_ten is date-add ~start 10
    ~minus_five is date-subtract ~start 5
    ~diff is date-diff ~start ~plus_ten
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_ok());

    // Check the results
    let plus_ten = evaluator.get_variable("plus_ten").unwrap();
    let minus_five = evaluator.get_variable("minus_five").unwrap();
    let diff = evaluator.get_variable("diff").unwrap();

    assert_eq!(plus_ten.to_string(), "2024-03-25T00:00:00Z");
    assert_eq!(minus_five.to_string(), "2024-03-10T00:00:00Z");

    // diff is now an object with multiple time units
    if let tails::value::Value::Object(diff_obj) = diff {
        let days = diff_obj.get("days").unwrap();
        assert_eq!(days.to_string(), "10");
    } else {
        panic!("Expected diff to be an object");
    }
}

#[test]
fn test_date_arithmetic_with_datetime() {
    let input = r#"
    ~meeting is date "2024-03-15T14:30:00Z"
    ~next_week is date-add ~meeting 7
    ~days_until is date-diff ~meeting ~next_week
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_ok());

    let next_week = evaluator.get_variable("next_week").unwrap();
    let days_until = evaluator.get_variable("days_until").unwrap();

    assert_eq!(next_week.to_string(), "2024-03-22T14:30:00Z");

    // days_until is now an object
    if let tails::value::Value::Object(diff_obj) = days_until {
        let days = diff_obj.get("days").unwrap();
        assert_eq!(days.to_string(), "7");
    } else {
        panic!("Expected days_until to be an object");
    }
}

#[test]
fn test_date_arithmetic_negative_diff() {
    let input = r#"
    ~earlier is date "2024-03-10"
    ~later is date "2024-03-20"
    ~forward_diff is date-diff ~earlier ~later
    ~backward_diff is date-diff ~later ~earlier
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_ok());

    let forward_diff = evaluator.get_variable("forward_diff").unwrap();
    let backward_diff = evaluator.get_variable("backward_diff").unwrap();

    // Both diffs are now objects
    if let tails::value::Value::Object(forward_obj) = forward_diff {
        let days = forward_obj.get("days").unwrap();
        assert_eq!(days.to_string(), "10");
    } else {
        panic!("Expected forward_diff to be an object");
    }

    if let tails::value::Value::Object(backward_obj) = backward_diff {
        let days = backward_obj.get("days").unwrap();
        assert_eq!(days.to_string(), "-10");
    } else {
        panic!("Expected backward_diff to be an object");
    }
}

#[test]
fn test_date_arithmetic_month_boundary() {
    let input = r#"
    ~end_of_jan is date "2024-01-25"
    ~early_feb is date-add ~end_of_jan 10
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_ok());

    let early_feb = evaluator.get_variable("early_feb").unwrap();
    assert_eq!(early_feb.to_string(), "2024-02-04T00:00:00Z");
}

#[test]
fn test_date_arithmetic_year_boundary() {
    let input = r#"
    ~end_of_year is date "2023-12-25"
    ~new_year is date-add ~end_of_year 10
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_ok());

    let new_year = evaluator.get_variable("new_year").unwrap();
    assert_eq!(new_year.to_string(), "2024-01-04T00:00:00Z");
}

#[test]
fn test_date_arithmetic_error_handling() {
    let input = r#"
    ~bad_add is date-add "not-a-date" 5
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("date-add first argument must be a date"));
}