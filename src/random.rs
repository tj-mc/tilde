use crate::ast::Expression;
use crate::evaluator::Evaluator;
use crate::value::Value;
use rand::Rng;

pub fn eval_random_positional(
    args: Vec<Expression>,
    evaluator: &mut Evaluator,
) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("random requires exactly 2 arguments".to_string());
    }

    // Check if arguments were originally float literals
    let min_was_float = matches!(&args[0], Expression::Number(_, true));
    let max_was_float = matches!(&args[1], Expression::Number(_, true));

    let min_val = evaluator.eval_expression(args[0].clone())?;
    let max_val = evaluator.eval_expression(args[1].clone())?;

    let (min, max) = match (min_val, max_val) {
        (Value::Number(min), Value::Number(max)) => (min, max),
        _ => return Err("random arguments must be numbers".to_string()),
    };

    if min > max {
        return Err("random minimum value cannot be greater than maximum value".to_string());
    }

    // If either argument was a float literal, return float; otherwise check fractional parts
    let should_return_float =
        min_was_float || max_was_float || min.fract() != 0.0 || max.fract() != 0.0;

    let mut rng = rand::thread_rng();

    if should_return_float {
        // At least one was a float literal or has fractional part, return random float
        let result = rng.gen_range(min..=max);
        Ok(Value::Number(result))
    } else {
        // Both are integers, return random integer in range (inclusive)
        let min_int = min as i64;
        let max_int = max as i64;
        let result = rng.gen_range(min_int..=max_int) as f64;
        Ok(Value::Number(result))
    }
}

/// Wrapper for stdlib compatibility
pub fn eval_random_positional_wrapper(
    args: Vec<Expression>,
    evaluator: &mut Evaluator,
) -> Result<Value, String> {
    eval_random_positional(args, evaluator)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Expression;
    use crate::evaluator::Evaluator;

    #[test]
    fn test_random_integer_range() {
        let mut evaluator = Evaluator::new();
        let args = vec![
            Expression::Number(1.0, false),
            Expression::Number(5.0, false),
        ];

        // Test multiple times to ensure it's in range
        for _ in 0..10 {
            let result = eval_random_positional(args.clone(), &mut evaluator).unwrap();
            if let Value::Number(n) = result {
                assert!((1.0..=5.0).contains(&n));
                // Should be an integer (no fractional part)
                assert_eq!(n.fract(), 0.0);
            } else {
                panic!("Expected number result");
            }
        }
    }

    #[test]
    fn test_random_float_range() {
        let mut evaluator = Evaluator::new();
        let args = vec![
            Expression::Number(0.0, false),
            Expression::Number(1.5, false),
        ];

        // Test multiple times to ensure it's in range
        for _ in 0..10 {
            let result = eval_random_positional(args.clone(), &mut evaluator).unwrap();
            if let Value::Number(n) = result {
                assert!((0.0..=1.5).contains(&n));
            } else {
                panic!("Expected number result");
            }
        }
    }

    #[test]
    fn test_random_mixed_types() {
        let mut evaluator = Evaluator::new();
        // One integer, one float - should return float
        let args = vec![
            Expression::Number(1.0, false),
            Expression::Number(2.5, false),
        ];

        let result = eval_random_positional(args, &mut evaluator).unwrap();
        if let Value::Number(n) = result {
            assert!((1.0..=2.5).contains(&n));
        } else {
            panic!("Expected number result");
        }
    }

    #[test]
    fn test_random_same_values() {
        let mut evaluator = Evaluator::new();
        let args = vec![
            Expression::Number(5.0, false),
            Expression::Number(5.0, false),
        ];

        let result = eval_random_positional(args, &mut evaluator).unwrap();
        if let Value::Number(n) = result {
            assert_eq!(n, 5.0);
        } else {
            panic!("Expected number result");
        }
    }

    #[test]
    fn test_random_wrong_argument_count() {
        let mut evaluator = Evaluator::new();

        // Too few arguments
        let args = vec![Expression::Number(1.0, false)];
        let result = eval_random_positional(args, &mut evaluator);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exactly 2 arguments"));

        // Too many arguments
        let args = vec![
            Expression::Number(1.0, false),
            Expression::Number(2.0, false),
            Expression::Number(3.0, false),
        ];
        let result = eval_random_positional(args, &mut evaluator);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exactly 2 arguments"));
    }

    #[test]
    fn test_random_non_numeric_arguments() {
        let mut evaluator = Evaluator::new();
        let args = vec![
            Expression::String("hello".to_string()),
            Expression::Number(5.0, false),
        ];

        let result = eval_random_positional(args, &mut evaluator);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must be numbers"));
    }

    #[test]
    fn test_random_min_greater_than_max() {
        let mut evaluator = Evaluator::new();
        let args = vec![
            Expression::Number(10.0, false),
            Expression::Number(5.0, false),
        ];

        let result = eval_random_positional(args, &mut evaluator);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .contains("minimum value cannot be greater than maximum")
        );
    }

    #[test]
    fn test_random_negative_range() {
        let mut evaluator = Evaluator::new();
        let args = vec![
            Expression::Number(-5.0, false),
            Expression::Number(-1.0, false),
        ];

        let result = eval_random_positional(args, &mut evaluator).unwrap();
        if let Value::Number(n) = result {
            assert!((-5.0..=-1.0).contains(&n));
            assert_eq!(n.fract(), 0.0); // Should be integer
        } else {
            panic!("Expected number result");
        }
    }
}
