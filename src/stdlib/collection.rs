use crate::ast::Expression;
use crate::evaluator::Evaluator;
use crate::value::Value;

/// Gets the length of a list or string
pub fn eval_length(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("length requires exactly one argument".to_string());
    }
    let value = evaluator.eval_expression(args[0].clone())?;
    match value {
        Value::List(list) => Ok(Value::Number(list.len() as f64)),
        Value::String(s) => Ok(Value::Number(s.len() as f64)),
        _ => Err("length can only be used on lists or strings".to_string()),
    }
}

/// Appends an item to a list and returns the new list
pub fn eval_append(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("append requires exactly two arguments".to_string());
    }

    let list_value = evaluator.eval_expression(args[0].clone())?;
    let item_value = evaluator.eval_expression(args[1].clone())?;

    match list_value {
        Value::List(mut list) => {
            list.push(item_value);
            Ok(Value::List(list))
        }
        _ => Err("append can only be used on lists".to_string()),
    }
}
