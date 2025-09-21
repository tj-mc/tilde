use tilde::{evaluator::Evaluator, parser::Parser, value::Value};

#[test]
fn test_date_basic_usage() {
    let input = r#"
    ~current is now
    ~specific is date "2024-03-15"
    ~datetime is date "2024-03-15T14:30:00Z"
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_ok());

    // Check that variables were set correctly
    let current = evaluator.get_variable("current").unwrap();
    let specific = evaluator.get_variable("specific").unwrap();
    let datetime = evaluator.get_variable("datetime").unwrap();

    assert!(matches!(current, Value::Date(_)));
    assert!(matches!(specific, Value::Date(_)));
    assert!(matches!(datetime, Value::Date(_)));

    // Check specific date parsing
    assert_eq!(specific.to_string(), "2024-03-15T00:00:00Z");
    assert_eq!(datetime.to_string(), "2024-03-15T14:30:00Z");
}

#[test]
fn test_date_in_conditionals() {
    let input = r#"
    ~my-date is date "2024-03-15"
    if ~my-date (
        ~result is "date is truthy"
    ) else (
        ~result is "date is falsy"
    )
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_ok());

    let result_var = evaluator.get_variable("result").unwrap();
    assert_eq!(result_var.to_string(), "date is truthy");
}

#[test]
fn test_date_string_interpolation() {
    let input = r#"
    ~my-date is date "2024-03-15T14:30:00Z"
    ~message is "The event is scheduled for `~my-date`"
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_ok());

    let message = evaluator.get_variable("message").unwrap();
    assert_eq!(message.to_string(), "The event is scheduled for 2024-03-15T14:30:00Z");
}

#[test]
fn test_date_in_lists() {
    let input = r#"
    ~date1 is date "2024-01-01"
    ~date2 is date "2024-06-15"
    ~date3 is date "2024-12-31"
    ~dates is [~date1, ~date2, ~date3]
    ~count is length ~dates
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_ok());

    let dates = evaluator.get_variable("dates").unwrap();
    let count = evaluator.get_variable("count").unwrap();

    if let Value::List(date_list) = dates {
        assert_eq!(date_list.len(), 3);
        assert!(matches!(date_list[0], Value::Date(_)));
        assert!(matches!(date_list[1], Value::Date(_)));
        assert!(matches!(date_list[2], Value::Date(_)));
    } else {
        panic!("Expected List value");
    }

    assert_eq!(count.to_string(), "3");
}

#[test]
fn test_date_in_objects() {
    let input = r#"
    ~event is {
        "name": "Birthday Party",
        "date": date "2024-07-20T18:00:00Z",
        "location": "Home"
    }
    ~event-date is ~event.date
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_ok());

    let event = evaluator.get_variable("event").unwrap();
    let event_date = evaluator.get_variable("event-date").unwrap();

    if let Value::Object(obj) = event {
        let date_value = obj.get("date").unwrap();
        assert!(matches!(date_value, Value::Date(_)));
        assert_eq!(date_value.to_string(), "2024-07-20T18:00:00Z");
    } else {
        panic!("Expected Object value");
    }

    assert!(matches!(event_date, Value::Date(_)));
    assert_eq!(event_date.to_string(), "2024-07-20T18:00:00Z");
}

#[test]
fn test_date_assignment_to_list_indices() {
    let input = r#"
    ~dates is []
    ~dates.0 is date "2024-01-01"
    ~dates.1 is date "2024-02-01"
    ~first-date is ~dates.0
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_ok());

    let dates = evaluator.get_variable("dates").unwrap();
    let first_date = evaluator.get_variable("first-date").unwrap();

    if let Value::List(date_list) = dates {
        assert_eq!(date_list.len(), 2);
        assert!(matches!(date_list[0], Value::Date(_)));
        assert_eq!(date_list[0].to_string(), "2024-01-01T00:00:00Z");
    } else {
        panic!("Expected List value");
    }

    assert!(matches!(first_date, Value::Date(_)));
    assert_eq!(first_date.to_string(), "2024-01-01T00:00:00Z");
}

#[test]
fn test_date_error_handling() {
    let input = r#"
    ~bad-date is date "invalid-date"
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid date format"));
}

#[test]
fn test_now_vs_date_difference() {
    let input = r#"
    ~now-time is now
    ~static-time is date "2020-01-01T00:00:00Z"
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_ok());

    let now_time = evaluator.get_variable("now-time").unwrap();
    let static_time = evaluator.get_variable("static-time").unwrap();

    assert!(matches!(now_time, Value::Date(_)));
    assert!(matches!(static_time, Value::Date(_)));

    // Static time should be exactly what we specified
    assert_eq!(static_time.to_string(), "2020-01-01T00:00:00Z");

    // Now time should not equal the static time
    assert_ne!(now_time, static_time);
}

#[test]
fn test_date_with_timezone_conversion() {
    let input = r#"
    ~utc-date is date "2024-03-15T14:30:00Z"
    ~offset-date is date "2024-03-15T16:30:00+02:00"
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_ok());

    let utc_date = evaluator.get_variable("utc-date").unwrap();
    let offset_date = evaluator.get_variable("offset-date").unwrap();

    // Both should represent the same moment in time (both converted to UTC)
    assert_eq!(utc_date, offset_date);
    assert_eq!(utc_date.to_string(), "2024-03-15T14:30:00Z");
    assert_eq!(offset_date.to_string(), "2024-03-15T14:30:00Z");
}

#[test]
fn test_multiple_date_operations() {
    let input = r#"
    ~dates is []
    ~dates.0 is now
    ~dates.1 is date "2024-01-01"
    ~dates.2 is date "2024-06-15T12:00:00Z"

    ~event is {
        "start": date "2024-12-25T09:00:00Z",
        "end": date "2024-12-25T17:00:00Z"
    }

    ~summary is "Event from `~event.start` to `~event.end`"
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_ok());

    let dates = evaluator.get_variable("dates").unwrap();
    let event = evaluator.get_variable("event").unwrap();
    let summary = evaluator.get_variable("summary").unwrap();

    // Verify dates list
    if let Value::List(date_list) = dates {
        assert_eq!(date_list.len(), 3);
        assert!(matches!(date_list[0], Value::Date(_))); // now
        assert_eq!(date_list[1].to_string(), "2024-01-01T00:00:00Z");
        assert_eq!(date_list[2].to_string(), "2024-06-15T12:00:00Z");
    } else {
        panic!("Expected List value");
    }

    // Verify event object
    if let Value::Object(obj) = event {
        let start = obj.get("start").unwrap();
        let end = obj.get("end").unwrap();
        assert_eq!(start.to_string(), "2024-12-25T09:00:00Z");
        assert_eq!(end.to_string(), "2024-12-25T17:00:00Z");
    } else {
        panic!("Expected Object value");
    }

    // Verify string interpolation worked
    assert_eq!(summary.to_string(), "Event from 2024-12-25T09:00:00Z to 2024-12-25T17:00:00Z");
}