use super::utils::*;
use crate::ast::Expression;
use crate::evaluator::Evaluator;
use crate::value::Value;

pub fn eval_absolute(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    let number = extract_number_arg(&args, evaluator, "absolute")?;
    Ok(Value::Number(number.abs()))
}

pub fn eval_square_root(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    let number = extract_number_arg(&args, evaluator, "square-root")?;
    if number < 0.0 {
        return Err("square-root argument must be non-negative".to_string());
    }
    Ok(Value::Number(number.sqrt()))
}

// Trigonometric functions
pub fn eval_sin(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    let radians = extract_number_arg(&args, evaluator, "sin")?;
    Ok(Value::Number(radians.sin()))
}

pub fn eval_cos(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    let radians = extract_number_arg(&args, evaluator, "cos")?;
    Ok(Value::Number(radians.cos()))
}

pub fn eval_tan(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    let radians = extract_number_arg(&args, evaluator, "tan")?;
    Ok(Value::Number(radians.tan()))
}

// Inverse trigonometric functions
pub fn eval_asin(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    let value = extract_number_arg(&args, evaluator, "asin")?;
    if !(-1.0..=1.0).contains(&value) {
        return Err("asin argument must be between -1 and 1".to_string());
    }
    Ok(Value::Number(value.asin()))
}

pub fn eval_acos(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    let value = extract_number_arg(&args, evaluator, "acos")?;
    if !(-1.0..=1.0).contains(&value) {
        return Err("acos argument must be between -1 and 1".to_string());
    }
    Ok(Value::Number(value.acos()))
}

pub fn eval_atan(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    let value = extract_number_arg(&args, evaluator, "atan")?;
    Ok(Value::Number(value.atan()))
}

pub fn eval_atan2(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    let (y, x) = extract_two_number_args(&args, evaluator, "atan2")?;
    Ok(Value::Number(y.atan2(x)))
}

// Logarithmic and exponential functions
pub fn eval_log(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() == 1 {
        // Natural logarithm
        let value = extract_number_arg(&args, evaluator, "log")?;
        if value <= 0.0 {
            return Err("log argument must be positive".to_string());
        }
        Ok(Value::Number(value.ln()))
    } else if args.len() == 2 {
        // Logarithm with custom base
        let value = extract_number_value(&args[0], evaluator)?;
        let base = extract_number_value(&args[1], evaluator)?;
        if value <= 0.0 {
            return Err("log argument must be positive".to_string());
        }
        if base <= 0.0 || base == 1.0 {
            return Err("log base must be positive and not equal to 1".to_string());
        }
        Ok(Value::Number(value.log(base)))
    } else {
        Err("log requires 1 or 2 arguments: value, [base]".to_string())
    }
}

pub fn eval_log10(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    let value = extract_number_arg(&args, evaluator, "log10")?;
    if value <= 0.0 {
        return Err("log10 argument must be positive".to_string());
    }
    Ok(Value::Number(value.log10()))
}

pub fn eval_exp(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    let value = extract_number_arg(&args, evaluator, "exp")?;
    Ok(Value::Number(value.exp()))
}

pub fn eval_pow(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    let (base, exponent) = extract_two_number_args(&args, evaluator, "pow")?;
    Ok(Value::Number(base.powf(exponent)))
}

// Rounding functions
pub fn eval_round(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() == 1 {
        let value = extract_number_arg(&args, evaluator, "round")?;
        Ok(Value::Number(value.round()))
    } else if args.len() == 2 {
        let value = extract_number_value(&args[0], evaluator)?;
        let precision = extract_number_value(&args[1], evaluator)? as i32;
        if precision < 0 {
            return Err("round precision must be non-negative".to_string());
        }
        let multiplier = 10_f64.powi(precision);
        Ok(Value::Number((value * multiplier).round() / multiplier))
    } else {
        Err("round requires 1 or 2 arguments: value, [precision]".to_string())
    }
}

pub fn eval_floor(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    let value = extract_number_arg(&args, evaluator, "floor")?;
    Ok(Value::Number(value.floor()))
}

pub fn eval_ceil(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    let value = extract_number_arg(&args, evaluator, "ceil")?;
    Ok(Value::Number(value.ceil()))
}

// Math constants
pub fn eval_pi(_args: Vec<Expression>, _evaluator: &mut Evaluator) -> Result<Value, String> {
    Ok(Value::Number(std::f64::consts::PI))
}

pub fn eval_e(_args: Vec<Expression>, _evaluator: &mut Evaluator) -> Result<Value, String> {
    Ok(Value::Number(std::f64::consts::E))
}
