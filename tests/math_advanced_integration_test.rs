use tilde::{parser::Parser, evaluator::Evaluator};

#[test]
fn test_trigonometric_functions() {
    let input = r#"
        ~result1 is sin 0
        ~result2 is cos 0
        ~result3 is tan 0
        ~result4 is sin (pi / 2)
        ~result5 is cos (pi / 2)
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    evaluator.eval_program(program).unwrap();

    // sin(0) = 0
    let sin_0 = evaluator.get_variable("result1").unwrap().to_string().parse::<f64>().unwrap();
    assert!((sin_0 - 0.0).abs() < 1e-10);

    // cos(0) = 1
    let cos_0 = evaluator.get_variable("result2").unwrap().to_string().parse::<f64>().unwrap();
    assert!((cos_0 - 1.0).abs() < 1e-10);

    // tan(0) = 0
    let tan_0 = evaluator.get_variable("result3").unwrap().to_string().parse::<f64>().unwrap();
    assert!((tan_0 - 0.0).abs() < 1e-10);

    // sin(π/2) ≈ 1
    let sin_pi_2 = evaluator.get_variable("result4").unwrap().to_string().parse::<f64>().unwrap();
    assert!((sin_pi_2 - 1.0).abs() < 1e-10);

    // cos(π/2) ≈ 0
    let cos_pi_2 = evaluator.get_variable("result5").unwrap().to_string().parse::<f64>().unwrap();
    assert!((cos_pi_2 - 0.0).abs() < 1e-10);
}

#[test]
fn test_inverse_trigonometric_functions() {
    let input = r#"
        ~result1 is asin 0
        ~result2 is acos 1
        ~result3 is atan 0
        ~result4 is atan2 1 1
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    evaluator.eval_program(program).unwrap();

    // asin(0) = 0
    let asin_0 = evaluator.get_variable("result1").unwrap().to_string().parse::<f64>().unwrap();
    assert!((asin_0 - 0.0).abs() < 1e-10);

    // acos(1) = 0
    let acos_1 = evaluator.get_variable("result2").unwrap().to_string().parse::<f64>().unwrap();
    assert!((acos_1 - 0.0).abs() < 1e-10);

    // atan(0) = 0
    let atan_0 = evaluator.get_variable("result3").unwrap().to_string().parse::<f64>().unwrap();
    assert!((atan_0 - 0.0).abs() < 1e-10);

    // atan2(1,1) = π/4
    let atan2_result = evaluator.get_variable("result4").unwrap().to_string().parse::<f64>().unwrap();
    assert!((atan2_result - std::f64::consts::PI / 4.0).abs() < 1e-10);
}

#[test]
fn test_logarithmic_and_exponential_functions() {
    let input = r#"
        ~result1 is log e
        ~result2 is log10 100
        ~result3 is exp 0
        ~result4 is exp 1
        ~result5 is log 8 2
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    evaluator.eval_program(program).unwrap();

    // log(e) = 1
    let log_e = evaluator.get_variable("result1").unwrap().to_string().parse::<f64>().unwrap();
    assert!((log_e - 1.0).abs() < 1e-10);

    // log10(100) = 2
    let log10_100 = evaluator.get_variable("result2").unwrap().to_string().parse::<f64>().unwrap();
    assert!((log10_100 - 2.0).abs() < 1e-10);

    // exp(0) = 1
    let exp_0 = evaluator.get_variable("result3").unwrap().to_string().parse::<f64>().unwrap();
    assert!((exp_0 - 1.0).abs() < 1e-10);

    // exp(1) = e
    let exp_1 = evaluator.get_variable("result4").unwrap().to_string().parse::<f64>().unwrap();
    assert!((exp_1 - std::f64::consts::E).abs() < 1e-10);

    // log(8, 2) = 3
    let log_8_base_2 = evaluator.get_variable("result5").unwrap().to_string().parse::<f64>().unwrap();
    assert!((log_8_base_2 - 3.0).abs() < 1e-10);
}

#[test]
fn test_power_and_rounding_functions() {
    let input = r#"
        ~result1 is pow 2 3
        ~result2 is round 3.14159 2
        ~result3 is floor 3.7
        ~result4 is ceil 3.2
        ~result5 is round 3.5
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    evaluator.eval_program(program).unwrap();

    // pow(2, 3) = 8
    assert_eq!(evaluator.get_variable("result1").unwrap().to_string(), "8");

    // round(3.14159, 2) = 3.14
    let rounded = evaluator.get_variable("result2").unwrap().to_string().parse::<f64>().unwrap();
    assert!((rounded - 3.14).abs() < 1e-10);

    // floor(3.7) = 3
    assert_eq!(evaluator.get_variable("result3").unwrap().to_string(), "3");

    // ceil(3.2) = 4
    assert_eq!(evaluator.get_variable("result4").unwrap().to_string(), "4");

    // round(3.5) = 4
    assert_eq!(evaluator.get_variable("result5").unwrap().to_string(), "4");
}

#[test]
fn test_math_constants() {
    let input = r#"
        ~pi_val is pi
        ~e_val is e
        ~pi_times_2 is pi * 2
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    evaluator.eval_program(program).unwrap();

    // Test pi constant
    let pi_val = evaluator.get_variable("pi_val").unwrap().to_string().parse::<f64>().unwrap();
    assert!((pi_val - std::f64::consts::PI).abs() < 1e-10);

    // Test e constant
    let e_val = evaluator.get_variable("e_val").unwrap().to_string().parse::<f64>().unwrap();
    assert!((e_val - std::f64::consts::E).abs() < 1e-10);

    // Test that constants can be used in expressions
    let pi_times_2 = evaluator.get_variable("pi_times_2").unwrap().to_string().parse::<f64>().unwrap();
    assert!((pi_times_2 - 2.0 * std::f64::consts::PI).abs() < 1e-10);
}