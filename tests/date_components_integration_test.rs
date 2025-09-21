use tilde::{evaluator::Evaluator, parser::Parser};

#[test]
fn test_date_component_extraction() {
    let input = r#"
    ~christmas is date "2024-12-25T18:30:45Z"
    ~year is date-year ~christmas
    ~month is date-month ~christmas
    ~day is date-day ~christmas
    ~hour is date-hour ~christmas
    ~minute is date-minute ~christmas
    ~second is date-second ~christmas
    ~weekday is date-weekday ~christmas
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_ok());

    let year = evaluator.get_variable("year").unwrap();
    let month = evaluator.get_variable("month").unwrap();
    let day = evaluator.get_variable("day").unwrap();
    let hour = evaluator.get_variable("hour").unwrap();
    let minute = evaluator.get_variable("minute").unwrap();
    let second = evaluator.get_variable("second").unwrap();
    let weekday = evaluator.get_variable("weekday").unwrap();

    assert_eq!(year.to_string(), "2024");
    assert_eq!(month.to_string(), "12");
    assert_eq!(day.to_string(), "25");
    assert_eq!(hour.to_string(), "18");
    assert_eq!(minute.to_string(), "30");
    assert_eq!(second.to_string(), "45");
    assert_eq!(weekday.to_string(), "3"); // Wednesday
}

#[test]
fn test_date_components_midnight() {
    let input = r#"
    ~midnight is date "2024-01-01T00:00:00Z"
    ~hour is date-hour ~midnight
    ~minute is date-minute ~midnight
    ~second is date-second ~midnight
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_ok());

    let hour = evaluator.get_variable("hour").unwrap();
    let minute = evaluator.get_variable("minute").unwrap();
    let second = evaluator.get_variable("second").unwrap();

    assert_eq!(hour.to_string(), "0");
    assert_eq!(minute.to_string(), "0");
    assert_eq!(second.to_string(), "0");
}

#[test]
fn test_date_components_date_only() {
    let input = r#"
    ~simple is date "2024-07-04"
    ~year is date-year ~simple
    ~month is date-month ~simple
    ~day is date-day ~simple
    ~hour is date-hour ~simple
    ~minute is date-minute ~simple
    ~second is date-second ~simple
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_ok());

    let year = evaluator.get_variable("year").unwrap();
    let month = evaluator.get_variable("month").unwrap();
    let day = evaluator.get_variable("day").unwrap();
    let hour = evaluator.get_variable("hour").unwrap();
    let minute = evaluator.get_variable("minute").unwrap();
    let second = evaluator.get_variable("second").unwrap();

    assert_eq!(year.to_string(), "2024");
    assert_eq!(month.to_string(), "7");
    assert_eq!(day.to_string(), "4");
    assert_eq!(hour.to_string(), "0");   // Defaults to midnight
    assert_eq!(minute.to_string(), "0");
    assert_eq!(second.to_string(), "0");
}

#[test]
fn test_weekday_calculation() {
    let input = r#"
    # Test various weekdays
    ~monday is date "2024-03-11"       # Monday
    ~tuesday is date "2024-03-12"      # Tuesday
    ~wednesday is date "2024-03-13"    # Wednesday
    ~thursday is date "2024-03-14"     # Thursday
    ~friday is date "2024-03-15"       # Friday
    ~saturday is date "2024-03-16"     # Saturday
    ~sunday is date "2024-03-17"       # Sunday

    ~mon_wd is date-weekday ~monday
    ~tue_wd is date-weekday ~tuesday
    ~wed_wd is date-weekday ~wednesday
    ~thu_wd is date-weekday ~thursday
    ~fri_wd is date-weekday ~friday
    ~sat_wd is date-weekday ~saturday
    ~sun_wd is date-weekday ~sunday
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_ok());

    let mon_wd = evaluator.get_variable("mon_wd").unwrap();
    let tue_wd = evaluator.get_variable("tue_wd").unwrap();
    let wed_wd = evaluator.get_variable("wed_wd").unwrap();
    let thu_wd = evaluator.get_variable("thu_wd").unwrap();
    let fri_wd = evaluator.get_variable("fri_wd").unwrap();
    let sat_wd = evaluator.get_variable("sat_wd").unwrap();
    let sun_wd = evaluator.get_variable("sun_wd").unwrap();

    // 0=Sunday, 1=Monday, 2=Tuesday, 3=Wednesday, 4=Thursday, 5=Friday, 6=Saturday
    assert_eq!(mon_wd.to_string(), "1");
    assert_eq!(tue_wd.to_string(), "2");
    assert_eq!(wed_wd.to_string(), "3");
    assert_eq!(thu_wd.to_string(), "4");
    assert_eq!(fri_wd.to_string(), "5");
    assert_eq!(sat_wd.to_string(), "6");
    assert_eq!(sun_wd.to_string(), "0");
}

#[test]
fn test_date_components_leap_year() {
    let input = r#"
    ~leap_day is date "2024-02-29"
    ~year is date-year ~leap_day
    ~month is date-month ~leap_day
    ~day is date-day ~leap_day
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_ok());

    let year = evaluator.get_variable("year").unwrap();
    let month = evaluator.get_variable("month").unwrap();
    let day = evaluator.get_variable("day").unwrap();

    assert_eq!(year.to_string(), "2024");
    assert_eq!(month.to_string(), "2");
    assert_eq!(day.to_string(), "29");
}

#[test]
fn test_date_components_error_handling() {
    let input = r#"
    ~bad_year is date-year "not-a-date"
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("date-year argument must be a date"));
}