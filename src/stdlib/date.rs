use crate::ast::Expression;
use crate::evaluator::Evaluator;
use crate::value::Value;
use chrono::{DateTime, Datelike, Duration, NaiveDate, NaiveDateTime, TimeZone, Timelike, Utc};

/// Returns the current UTC datetime
/// Usage: now
pub fn eval_now(args: Vec<Expression>, _evaluator: &mut Evaluator) -> Result<Value, String> {
    if !args.is_empty() {
        return Err("now requires no arguments".to_string());
    }

    let now = Utc::now();
    Ok(Value::Date(now))
}

/// Creates a date from a string - supports both date and datetime formats
/// Usage: date "2024-03-15" or date "2024-03-15T14:30:00Z"
pub fn eval_date(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("date requires exactly 1 argument".to_string());
    }

    let date_str_val = evaluator.eval_expression(args[0].clone())?;
    let date_str = match date_str_val {
        Value::String(s) => s,
        _ => return Err("date argument must be a string".to_string()),
    };

    // Try parsing as full datetime first (ISO 8601)
    if let Ok(datetime) = DateTime::parse_from_rfc3339(&date_str) {
        return Ok(Value::Date(datetime.with_timezone(&Utc)));
    }

    // Try parsing as date only (YYYY-MM-DD)
    if let Ok(naive_date) = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d") {
        let datetime = Utc.from_utc_datetime(&naive_date.and_hms_opt(0, 0, 0).unwrap());
        return Ok(Value::Date(datetime));
    }

    Err(format!(
        "Invalid date format '{}'. Expected YYYY-MM-DD or ISO 8601 format",
        date_str
    ))
}

/// Adds a specified number of days to a date
/// Usage: date-add date days
pub fn eval_date_add(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("date-add requires exactly 2 arguments (date, days)".to_string());
    }

    let date_val = evaluator.eval_expression(args[0].clone())?;
    let days_val = evaluator.eval_expression(args[1].clone())?;

    let date = match date_val {
        Value::Date(d) => d,
        _ => return Err("date-add first argument must be a date".to_string()),
    };

    let days = match days_val {
        Value::Number(n) => n as i64,
        _ => return Err("date-add second argument must be a number".to_string()),
    };

    match date.checked_add_signed(Duration::days(days)) {
        Some(new_date) => Ok(Value::Date(new_date)),
        None => Err("Date arithmetic overflow".to_string()),
    }
}

/// Subtracts a specified number of days from a date
/// Usage: date-subtract date days
pub fn eval_date_subtract(
    args: Vec<Expression>,
    evaluator: &mut Evaluator,
) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("date-subtract requires exactly 2 arguments (date, days)".to_string());
    }

    let date_val = evaluator.eval_expression(args[0].clone())?;
    let days_val = evaluator.eval_expression(args[1].clone())?;

    let date = match date_val {
        Value::Date(d) => d,
        _ => return Err("date-subtract first argument must be a date".to_string()),
    };

    let days = match days_val {
        Value::Number(n) => n as i64,
        _ => return Err("date-subtract second argument must be a number".to_string()),
    };

    match date.checked_sub_signed(Duration::days(days)) {
        Some(new_date) => Ok(Value::Date(new_date)),
        None => Err("Date arithmetic overflow".to_string()),
    }
}

/// Calculates the difference between two dates and returns an object with all units
/// Usage: date-diff date1 date2
/// Returns: {"days": N, "hours": N, "minutes": N, "seconds": N, "milliseconds": N}
pub fn eval_date_diff(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    let (date1, date2) = extract_two_dates(args, evaluator, "date-diff")?;
    let diff = date2.signed_duration_since(date1);

    let mut result = std::collections::HashMap::new();
    result.insert("days".to_string(), Value::Number(diff.num_days() as f64));
    result.insert("hours".to_string(), Value::Number(diff.num_hours() as f64));
    result.insert(
        "minutes".to_string(),
        Value::Number(diff.num_minutes() as f64),
    );
    result.insert(
        "seconds".to_string(),
        Value::Number(diff.num_seconds() as f64),
    );
    result.insert(
        "milliseconds".to_string(),
        Value::Number(diff.num_milliseconds() as f64),
    );

    Ok(Value::Object(result))
}

/// Formats a date using a custom format string
/// Usage: date-format date format_string
pub fn eval_date_format(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("date-format requires exactly 2 arguments (date, format)".to_string());
    }

    let date_val = evaluator.eval_expression(args[0].clone())?;
    let format_val = evaluator.eval_expression(args[1].clone())?;

    let date = match date_val {
        Value::Date(d) => d,
        _ => return Err("date-format first argument must be a date".to_string()),
    };

    let format_str = match format_val {
        Value::String(s) => s,
        _ => return Err("date-format second argument must be a string".to_string()),
    };

    let formatted = date.format(&format_str).to_string();
    Ok(Value::String(formatted))
}

/// Parses a date from a string using a custom format
/// Usage: date-parse date_string format_string
pub fn eval_date_parse(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("date-parse requires exactly 2 arguments (date_string, format)".to_string());
    }

    let date_str_val = evaluator.eval_expression(args[0].clone())?;
    let format_val = evaluator.eval_expression(args[1].clone())?;

    let date_str = match date_str_val {
        Value::String(s) => s,
        _ => return Err("date-parse first argument must be a string".to_string()),
    };

    let format_str = match format_val {
        Value::String(s) => s,
        _ => return Err("date-parse second argument must be a string".to_string()),
    };

    // Try to parse as datetime with timezone first
    if let Ok(datetime) = DateTime::parse_from_str(&date_str, &format_str) {
        return Ok(Value::Date(datetime.with_timezone(&Utc)));
    }

    // Try to parse as naive datetime (includes time but no timezone)
    if let Ok(naive_datetime) = NaiveDateTime::parse_from_str(&date_str, &format_str) {
        let datetime = Utc.from_utc_datetime(&naive_datetime);
        return Ok(Value::Date(datetime));
    }

    // Try to parse as naive date (date only)
    if let Ok(naive_date) = NaiveDate::parse_from_str(&date_str, &format_str) {
        let datetime = Utc.from_utc_datetime(&naive_date.and_hms_opt(0, 0, 0).unwrap());
        return Ok(Value::Date(datetime));
    }

    Err(format!(
        "Failed to parse '{}' using format '{}'",
        date_str, format_str
    ))
}

/// Helper function for date component extraction
fn extract_date_component<F>(
    args: Vec<Expression>,
    evaluator: &mut Evaluator,
    function_name: &str,
    extractor: F,
) -> Result<Value, String>
where
    F: FnOnce(DateTime<Utc>) -> f64,
{
    if args.len() != 1 {
        return Err(format!(
            "{} requires exactly 1 argument (date)",
            function_name
        ));
    }

    let date_val = evaluator.eval_expression(args[0].clone())?;
    let date = match date_val {
        Value::Date(d) => d,
        _ => return Err(format!("{} argument must be a date", function_name)),
    };

    Ok(Value::Number(extractor(date)))
}

/// Extracts the year from a date
/// Usage: date-year date
pub fn eval_date_year(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    extract_date_component(args, evaluator, "date-year", |d| d.year() as f64)
}

/// Extracts the month from a date (1-12)
/// Usage: date-month date
pub fn eval_date_month(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    extract_date_component(args, evaluator, "date-month", |d| d.month() as f64)
}

/// Extracts the day from a date (1-31)
/// Usage: date-day date
pub fn eval_date_day(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    extract_date_component(args, evaluator, "date-day", |d| d.day() as f64)
}

/// Extracts the hour from a date (0-23)
/// Usage: date-hour date
pub fn eval_date_hour(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    extract_date_component(args, evaluator, "date-hour", |d| d.hour() as f64)
}

/// Extracts the minute from a date (0-59)
/// Usage: date-minute date
pub fn eval_date_minute(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    extract_date_component(args, evaluator, "date-minute", |d| d.minute() as f64)
}

/// Extracts the second from a date (0-59)
/// Usage: date-second date
pub fn eval_date_second(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    extract_date_component(args, evaluator, "date-second", |d| d.second() as f64)
}

/// Extracts the weekday from a date (0=Sunday, 1=Monday, ..., 6=Saturday)
/// Usage: date-weekday date
pub fn eval_date_weekday(
    args: Vec<Expression>,
    evaluator: &mut Evaluator,
) -> Result<Value, String> {
    extract_date_component(args, evaluator, "date-weekday", |d| {
        // Convert chrono weekday to common convention: Sunday=0, Monday=1, ..., Saturday=6
        match d.weekday().num_days_from_monday() {
            0 => 1.0, // Monday -> 1
            1 => 2.0, // Tuesday -> 2
            2 => 3.0, // Wednesday -> 3
            3 => 4.0, // Thursday -> 4
            4 => 5.0, // Friday -> 5
            5 => 6.0, // Saturday -> 6
            6 => 0.0, // Sunday -> 0
            _ => unreachable!(),
        }
    })
}

/// Checks if the first date is before the second date
/// Usage: date-before date1 date2
pub fn eval_date_before(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    let (date1, date2) = extract_two_dates(args, evaluator, "date-before")?;
    Ok(Value::Boolean(date1 < date2))
}

/// Checks if the first date is after the second date
/// Usage: date-after date1 date2
pub fn eval_date_after(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    let (date1, date2) = extract_two_dates(args, evaluator, "date-after")?;
    Ok(Value::Boolean(date1 > date2))
}

/// Checks if two dates are equal
/// Usage: date-equal date1 date2
pub fn eval_date_equal(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    let (date1, date2) = extract_two_dates(args, evaluator, "date-equal")?;
    Ok(Value::Boolean(date1 == date2))
}

/// Helper function for date comparison operations
fn extract_two_dates(
    args: Vec<Expression>,
    evaluator: &mut Evaluator,
    function_name: &str,
) -> Result<(DateTime<Utc>, DateTime<Utc>), String> {
    if args.len() != 2 {
        return Err(format!(
            "{} requires exactly 2 arguments (date1, date2)",
            function_name
        ));
    }

    let date1_val = evaluator.eval_expression(args[0].clone())?;
    let date2_val = evaluator.eval_expression(args[1].clone())?;

    let date1 = match date1_val {
        Value::Date(d) => d,
        _ => return Err(format!("{} first argument must be a date", function_name)),
    };

    let date2 = match date2_val {
        Value::Date(d) => d,
        _ => return Err(format!("{} second argument must be a date", function_name)),
    };

    Ok((date1, date2))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Expression;
    use crate::evaluator::Evaluator;
    use chrono::{Datelike, Utc};

    // === NOW FUNCTION TESTS ===

    #[test]
    fn test_now_no_args() {
        let mut evaluator = Evaluator::new();
        let args = vec![];

        // Capture system time before and after the call
        let before = Utc::now();
        let result = eval_now(args, &mut evaluator).unwrap();
        let after = Utc::now();

        match result {
            Value::Date(dt) => {
                // Verify the returned time is between before and after (within reasonable bounds)
                assert!(dt >= before, "Returned time should be after start time");
                assert!(dt <= after, "Returned time should be before end time");

                // Verify it's a reasonable year (sanity check)
                assert!(dt.year() >= 2024, "Should be current year or later");
            }
            _ => panic!("Expected Date value"),
        }
    }

    #[test]
    fn test_now_with_args_fails() {
        let mut evaluator = Evaluator::new();
        let args = vec![Expression::String("invalid".to_string())];

        let result = eval_now(args, &mut evaluator);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("no arguments"));
    }

    #[test]
    fn test_now_multiple_calls_progression() {
        let mut evaluator = Evaluator::new();
        let args = vec![];

        let result1 = eval_now(args.clone(), &mut evaluator).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(10));
        let result2 = eval_now(args, &mut evaluator).unwrap();

        match (result1, result2) {
            (Value::Date(dt1), Value::Date(dt2)) => {
                // Time should progress (dt2 >= dt1)
                assert!(dt2 >= dt1, "Time should progress forward");

                // Should be very close (within 1 second)
                let diff = dt2.signed_duration_since(dt1);
                assert!(diff.num_seconds() < 1, "Calls should be very close in time");
            }
            _ => panic!("Both results should be Date values"),
        }
    }

    // === DATE FUNCTION TESTS ===

    #[test]
    fn test_date_date_only_format() {
        let mut evaluator = Evaluator::new();
        let args = vec![Expression::String("2024-03-15".to_string())];

        let result = eval_date(args, &mut evaluator).unwrap();
        match result {
            Value::Date(dt) => {
                assert_eq!(dt.format("%Y-%m-%d").to_string(), "2024-03-15");
                assert_eq!(dt.format("%H:%M:%S").to_string(), "00:00:00");
            }
            _ => panic!("Expected Date value"),
        }
    }

    #[test]
    fn test_date_full_datetime_format() {
        let mut evaluator = Evaluator::new();
        let args = vec![Expression::String("2024-03-15T14:30:45Z".to_string())];

        let result = eval_date(args, &mut evaluator).unwrap();
        match result {
            Value::Date(dt) => {
                assert_eq!(
                    dt.format("%Y-%m-%dT%H:%M:%SZ").to_string(),
                    "2024-03-15T14:30:45Z"
                );
            }
            _ => panic!("Expected Date value"),
        }
    }

    #[test]
    fn test_date_with_timezone_converts_to_utc() {
        let mut evaluator = Evaluator::new();
        let args = vec![Expression::String("2024-03-15T14:30:00+02:00".to_string())];

        let result = eval_date(args, &mut evaluator).unwrap();
        match result {
            Value::Date(dt) => {
                // Should be converted to UTC (14:30 + 2:00 offset = 12:30 UTC)
                assert_eq!(
                    dt.format("%Y-%m-%dT%H:%M:%SZ").to_string(),
                    "2024-03-15T12:30:00Z"
                );
            }
            _ => panic!("Expected Date value"),
        }
    }

    #[test]
    fn test_date_leap_year() {
        let mut evaluator = Evaluator::new();
        let args = vec![Expression::String("2024-02-29".to_string())];

        let result = eval_date(args, &mut evaluator).unwrap();
        match result {
            Value::Date(dt) => {
                assert_eq!(dt.format("%Y-%m-%d").to_string(), "2024-02-29");
            }
            _ => panic!("Expected Date value"),
        }
    }

    #[test]
    fn test_date_invalid_leap_year() {
        let mut evaluator = Evaluator::new();
        let args = vec![Expression::String("2023-02-29".to_string())];

        let result = eval_date(args, &mut evaluator);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid date format"));
    }

    #[test]
    fn test_date_various_invalid_formats() {
        let mut evaluator = Evaluator::new();

        let invalid_dates = vec![
            "March 15, 2024",
            "15/03/2024",
            "2024/03/15",
            "15-03-2024",
            "2024-13-01", // Invalid month
            "2024-02-30", // Invalid day
            "",
            "not-a-date",
            "2024-03-15T25:00:00Z", // Invalid hour
            "2024-03-32",           // Invalid day
            "2024-00-15",           // Invalid month (zero)
        ];

        for invalid_date in invalid_dates {
            let args = vec![Expression::String(invalid_date.to_string())];
            let result = eval_date(args, &mut evaluator);
            assert!(
                result.is_err(),
                "Should fail for invalid date: {}",
                invalid_date
            );
            assert!(
                result.unwrap_err().contains("Invalid date format"),
                "Error should mention format for: {}",
                invalid_date
            );
        }
    }

    #[test]
    fn test_date_wrong_argument_count() {
        let mut evaluator = Evaluator::new();

        // No arguments
        let args = vec![];
        let result = eval_date(args, &mut evaluator);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exactly 1 argument"));

        // Too many arguments
        let args = vec![
            Expression::String("2024-03-15".to_string()),
            Expression::String("extra".to_string()),
        ];
        let result = eval_date(args, &mut evaluator);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exactly 1 argument"));
    }

    #[test]
    fn test_date_wrong_argument_type() {
        let mut evaluator = Evaluator::new();

        let args = vec![Expression::Number(42.0, false)];
        let result = eval_date(args, &mut evaluator);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must be a string"));
    }

    #[test]
    fn test_date_edge_cases() {
        let mut evaluator = Evaluator::new();

        // Year boundaries
        let test_cases = vec![
            ("1970-01-01T00:00:00Z", "1970-01-01T00:00:00Z"), // Unix epoch
            ("2000-01-01", "2000-01-01T00:00:00Z"),           // Y2K
            ("2038-01-19T03:14:07Z", "2038-01-19T03:14:07Z"), // Near 32-bit timestamp limit
        ];

        for (input, expected) in test_cases {
            let args = vec![Expression::String(input.to_string())];
            let result = eval_date(args, &mut evaluator).unwrap();

            match result {
                Value::Date(dt) => {
                    assert_eq!(dt.format("%Y-%m-%dT%H:%M:%SZ").to_string(), expected);
                }
                _ => panic!("Expected Date value for input: {}", input),
            }
        }
    }

    #[test]
    fn test_date_display_format() {
        let mut evaluator = Evaluator::new();
        let args = vec![Expression::String("2024-03-15T14:30:45Z".to_string())];

        let result = eval_date(args, &mut evaluator).unwrap();

        // Test that the Display implementation formats correctly
        assert_eq!(result.to_string(), "2024-03-15T14:30:45Z");
    }

    #[test]
    fn test_date_truthy_behavior() {
        let mut evaluator = Evaluator::new();
        let args = vec![Expression::String("2024-03-15".to_string())];

        let result = eval_date(args, &mut evaluator).unwrap();

        // Test that dates are always truthy
        assert!(result.is_truthy());
    }

    // === INTEGRATION TESTS ===

    #[test]
    fn test_date_functions_in_stdlib_context() {
        // Test that the functions work through the stdlib registry
        use crate::stdlib::get_stdlib_function;

        let now_fn = get_stdlib_function("now").expect("now function should be registered");
        let date_fn = get_stdlib_function("date").expect("date function should be registered");

        let mut evaluator = Evaluator::new();

        // Test now function
        let result = now_fn(vec![], &mut evaluator).unwrap();
        assert!(matches!(result, Value::Date(_)));

        // Test date function
        let args = vec![Expression::String("2024-03-15".to_string())];
        let result = date_fn(args, &mut evaluator).unwrap();
        assert!(matches!(result, Value::Date(_)));
    }

    #[test]
    fn test_date_equality() {
        let mut evaluator = Evaluator::new();

        // Create two identical dates
        let args1 = vec![Expression::String("2024-03-15T14:30:00Z".to_string())];
        let args2 = vec![Expression::String("2024-03-15T14:30:00Z".to_string())];

        let date1 = eval_date(args1, &mut evaluator).unwrap();
        let date2 = eval_date(args2, &mut evaluator).unwrap();

        assert_eq!(date1, date2);
    }

    #[test]
    fn test_date_different_formats_same_time() {
        let mut evaluator = Evaluator::new();

        // Same time with different timezone formats
        let args1 = vec![Expression::String("2024-03-15T14:30:00Z".to_string())];
        let args2 = vec![Expression::String("2024-03-15T16:30:00+02:00".to_string())];

        let date1 = eval_date(args1, &mut evaluator).unwrap();
        let date2 = eval_date(args2, &mut evaluator).unwrap();

        assert_eq!(date1, date2);
    }

    // === DATE ARITHMETIC TESTS ===

    #[test]
    fn test_date_arithmetic_functions_registered() {
        // Test that the arithmetic functions are registered in stdlib
        use crate::stdlib::get_stdlib_function;

        let date_add_fn = get_stdlib_function("date-add");
        let date_subtract_fn = get_stdlib_function("date-subtract");
        let date_diff_fn = get_stdlib_function("date-diff");

        assert!(date_add_fn.is_some());
        assert!(date_subtract_fn.is_some());
        assert!(date_diff_fn.is_some());
    }

    #[test]
    fn test_date_arithmetic_argument_validation() {
        let mut evaluator = Evaluator::new();

        // Test wrong number of arguments
        let result = eval_date_add(vec![], &mut evaluator);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exactly 2 arguments"));

        let result = eval_date_subtract(vec![Expression::Number(1.0, false)], &mut evaluator);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exactly 2 arguments"));

        let result = eval_date_diff(vec![Expression::Number(1.0, false)], &mut evaluator);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exactly 2 arguments"));
    }
}
