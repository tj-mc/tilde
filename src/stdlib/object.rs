use crate::ast::Expression;
use crate::evaluator::Evaluator;
use crate::value::Value;

/// Gets all keys from an object as a list
pub fn eval_keys(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("keys requires exactly one argument".to_string());
    }
    let obj_value = evaluator.eval_expression(args[0].clone())?;
    match obj_value {
        Value::Object(map) => {
            let keys: Vec<Value> = map.keys().map(|k| Value::String(k.clone())).collect();
            Ok(Value::List(keys))
        }
        _ => Err("keys can only be used on objects".to_string()),
    }
}

/// Gets all values from an object as a list
pub fn eval_values(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("values requires exactly one argument".to_string());
    }
    let obj_value = evaluator.eval_expression(args[0].clone())?;
    match obj_value {
        Value::Object(map) => {
            let values: Vec<Value> = map.values().cloned().collect();
            Ok(Value::List(values))
        }
        _ => Err("values can only be used on objects".to_string()),
    }
}

/// Checks if an object has a specific key
pub fn eval_has(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("has requires exactly two arguments".to_string());
    }
    let key_value = evaluator.eval_expression(args[0].clone())?;
    let obj_value = evaluator.eval_expression(args[1].clone())?;

    let key_str = match key_value {
        Value::String(s) => s,
        _ => return Err("has key argument must be a string".to_string()),
    };

    match obj_value {
        Value::Object(map) => Ok(Value::Boolean(map.contains_key(&key_str))),
        _ => Err("has can only be used on objects".to_string()),
    }
}