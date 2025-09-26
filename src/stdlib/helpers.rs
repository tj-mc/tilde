use super::utils::*;
use crate::ast::Expression;
use crate::evaluator::Evaluator;
use crate::value::Value;

/// Common predicate functions for filtering
/// Checks if a number is even
pub fn eval_is_even(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    let number = extract_number_arg(&args, evaluator, "is-even")?;
    Ok(Value::Boolean((number as i64) % 2 == 0))
}

/// Checks if a number is odd
pub fn eval_is_odd(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    let number = extract_number_arg(&args, evaluator, "is-odd")?;
    Ok(Value::Boolean((number as i64) % 2 != 0))
}

/// Checks if a number is positive (> 0)
pub fn eval_is_positive(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    let number = extract_number_arg(&args, evaluator, "is-positive")?;
    Ok(Value::Boolean(number > 0.0))
}

/// Checks if a number is negative (< 0)
pub fn eval_is_negative(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    let number = extract_number_arg(&args, evaluator, "is-negative")?;
    Ok(Value::Boolean(number < 0.0))
}

/// Checks if a number is zero
pub fn eval_is_zero(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    let number = extract_number_arg(&args, evaluator, "is-zero")?;
    Ok(Value::Boolean(number == 0.0))
}

/// Common transformation functions
/// Doubles a number (multiplies by 2)
pub fn eval_double(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    let number = extract_number_arg(&args, evaluator, "double")?;
    Ok(Value::Number(number * 2.0))
}

/// Triples a number (multiplies by 3)
pub fn eval_triple(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    let number = extract_number_arg(&args, evaluator, "triple")?;
    Ok(Value::Number(number * 3.0))
}

/// Quadruples a number (multiplies by 4)
pub fn eval_quadruple(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    let number = extract_number_arg(&args, evaluator, "quadruple")?;
    Ok(Value::Number(number * 4.0))
}

/// Halves a number (divides by 2)
pub fn eval_half(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    let number = extract_number_arg(&args, evaluator, "half")?;
    Ok(Value::Number(number / 2.0))
}

/// Squares a number (multiplies by itself)
pub fn eval_square(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    let number = extract_number_arg(&args, evaluator, "square")?;
    Ok(Value::Number(number * number))
}

/// Increments a number by 1
pub fn eval_increment(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    let number = extract_number_arg(&args, evaluator, "increment")?;
    Ok(Value::Number(number + 1.0))
}

/// Decrements a number by 1
pub fn eval_decrement(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    let number = extract_number_arg(&args, evaluator, "decrement")?;
    Ok(Value::Number(number - 1.0))
}

/// Common reduction functions
/// Adds two numbers
pub fn eval_add(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    let (a, b) = extract_two_number_args(&args, evaluator, "add")?;
    Ok(Value::Number(a + b))
}

/// Multiplies two numbers
pub fn eval_multiply(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    let (a, b) = extract_two_number_args(&args, evaluator, "multiply")?;
    Ok(Value::Number(a * b))
}

/// Returns the maximum of two numbers
pub fn eval_max(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    let (a, b) = extract_two_number_args(&args, evaluator, "max")?;
    Ok(Value::Number(if a > b { a } else { b }))
}

/// Returns the minimum of two numbers
pub fn eval_min(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    let (a, b) = extract_two_number_args(&args, evaluator, "min")?;
    Ok(Value::Number(if a < b { a } else { b }))
}

/// Fast iterative Fibonacci calculation
pub fn eval_fibonacci(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    let number = extract_number_arg(&args, evaluator, "fibonacci")?;

    if number < 0.0 {
        return Err("fibonacci requires a non-negative number".to_string());
    }

    if number > 1476.0 {
        return Err("fibonacci input too large (max 1476 to prevent overflow)".to_string());
    }

    let n = number as u32;

    if n <= 1 {
        return Ok(Value::Number(n as f64));
    }

    let mut a = 0.0;
    let mut b = 1.0;

    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }

    Ok(Value::Number(b))
}
