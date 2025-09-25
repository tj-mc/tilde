use crate::ast::Expression;
use crate::evaluator::Evaluator;
use crate::value::Value;

/// Checks if a value is a number
pub fn eval_is_number(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("is-number requires exactly 1 argument".to_string());
    }

    let value = evaluator.eval_expression(args[0].clone())?;
    let is_number = matches!(value, Value::Number(_));
    Ok(Value::Boolean(is_number))
}

/// Checks if a value is a string
pub fn eval_is_string(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("is-string requires exactly 1 argument".to_string());
    }

    let value = evaluator.eval_expression(args[0].clone())?;
    let is_string = matches!(value, Value::String(_));
    Ok(Value::Boolean(is_string))
}

/// Checks if a value is a boolean
pub fn eval_is_boolean(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("is-boolean requires exactly 1 argument".to_string());
    }

    let value = evaluator.eval_expression(args[0].clone())?;
    let is_boolean = matches!(value, Value::Boolean(_));
    Ok(Value::Boolean(is_boolean))
}

/// Checks if a value is a list
pub fn eval_is_list(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("is-list requires exactly 1 argument".to_string());
    }

    let value = evaluator.eval_expression(args[0].clone())?;
    let is_list = matches!(value, Value::List(_));
    Ok(Value::Boolean(is_list))
}

/// Checks if a value is an object
pub fn eval_is_object(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("is-object requires exactly 1 argument".to_string());
    }

    let value = evaluator.eval_expression(args[0].clone())?;
    let is_object = matches!(value, Value::Object(_));
    Ok(Value::Boolean(is_object))
}

/// Checks if a value is null
pub fn eval_is_null(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("is-null requires exactly 1 argument".to_string());
    }

    let value = evaluator.eval_expression(args[0].clone())?;
    let is_null = matches!(value, Value::Null);
    Ok(Value::Boolean(is_null))
}

/// Checks if a value is empty (empty string, list, or object)
pub fn eval_is_empty(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("is-empty requires exactly 1 argument".to_string());
    }

    let value = evaluator.eval_expression(args[0].clone())?;
    let is_empty = match value {
        Value::String(s) => s.is_empty(),
        Value::List(l) => l.is_empty(),
        Value::Object(o) => o.is_empty(),
        _ => false,
    };
    Ok(Value::Boolean(is_empty))
}

/// Checks if a variable is defined
/// This function differs from others as it takes a variable name as an expression
pub fn eval_is_defined(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("is-defined requires exactly 1 argument".to_string());
    }

    // For is-defined, we only care about variables, not general expressions
    match &args[0] {
        Expression::Variable(_var_name) => {
            // Try to get the variable value - if it succeeds, the variable is defined
            match evaluator.eval_expression(args[0].clone()) {
                Ok(_) => Ok(Value::Boolean(true)),
                Err(_) => Ok(Value::Boolean(false)),
            }
        }
        _ => {
            // For non-variable expressions, they are "defined" if they can be evaluated
            match evaluator.eval_expression(args[0].clone()) {
                Ok(_) => Ok(Value::Boolean(true)),
                Err(_) => Ok(Value::Boolean(false)),
            }
        }
    }
}
