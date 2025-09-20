use crate::ast::Expression;
use crate::evaluator::Evaluator;
use crate::value::Value;

/// Extract a single number argument from function arguments
pub fn extract_number_arg(args: &[Expression], evaluator: &mut Evaluator, function_name: &str) -> Result<f64, String> {
    if args.len() != 1 {
        return Err(format!("{} requires exactly 1 argument (number)", function_name));
    }

    let value = evaluator.eval_expression(args[0].clone())?;
    match value {
        Value::Number(n) => Ok(n),
        _ => Err(format!("{} argument must be a number", function_name)),
    }
}

/// Extract a single string argument from function arguments
pub fn extract_string_arg(args: &[Expression], evaluator: &mut Evaluator, function_name: &str) -> Result<String, String> {
    if args.len() != 1 {
        return Err(format!("{} requires exactly 1 argument (string)", function_name));
    }

    let value = evaluator.eval_expression(args[0].clone())?;
    match value {
        Value::String(s) => Ok(s),
        _ => Err(format!("{} argument must be a string", function_name)),
    }
}

/// Extract two number arguments from function arguments
pub fn extract_two_number_args(args: &[Expression], evaluator: &mut Evaluator, function_name: &str) -> Result<(f64, f64), String> {
    if args.len() != 2 {
        return Err(format!("{} requires exactly 2 arguments (number, number)", function_name));
    }

    let a_val = evaluator.eval_expression(args[0].clone())?;
    let b_val = evaluator.eval_expression(args[1].clone())?;

    match (a_val, b_val) {
        (Value::Number(a), Value::Number(b)) => Ok((a, b)),
        _ => Err(format!("{} arguments must be numbers", function_name)),
    }
}

/// Extract two arguments of specified types for functions like split/join
pub fn extract_string_string_args(args: &[Expression], evaluator: &mut Evaluator, function_name: &str) -> Result<(String, String), String> {
    if args.len() != 2 {
        return Err(format!("{} requires exactly 2 arguments (string, string)", function_name));
    }

    let first_val = evaluator.eval_expression(args[0].clone())?;
    let second_val = evaluator.eval_expression(args[1].clone())?;

    let first = match first_val {
        Value::String(s) => s,
        _ => return Err(format!("{} first argument must be a string", function_name)),
    };

    let second = match second_val {
        Value::String(s) => s,
        _ => return Err(format!("{} second argument must be a string", function_name)),
    };

    Ok((first, second))
}

/// Extract a list from the first argument
pub fn extract_list_arg(args: &[Expression], evaluator: &mut Evaluator, function_name: &str) -> Result<Vec<Value>, String> {
    if args.is_empty() {
        return Err(format!("{} requires at least 1 argument (list)", function_name));
    }

    let list_val = evaluator.eval_expression(args[0].clone())?;
    match list_val {
        Value::List(items) => Ok(items),
        _ => Err(format!("{} first argument must be a list", function_name)),
    }
}

/// Extract list and string arguments for functions like join
pub fn extract_list_string_args(args: &[Expression], evaluator: &mut Evaluator, function_name: &str) -> Result<(Vec<Value>, String), String> {
    if args.len() != 2 {
        return Err(format!("{} requires exactly 2 arguments (list, string)", function_name));
    }

    let list = extract_list_arg(args, evaluator, function_name)?;
    let string = extract_string_arg(&args[1..], evaluator, function_name)?;
    Ok((list, string))
}

/// Evaluate all arguments and return as a Vec of Values
pub fn evaluate_args(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Vec<Value>, String> {
    let mut result = Vec::new();
    for arg in args {
        result.push(evaluator.eval_expression(arg)?);
    }
    Ok(result)
}