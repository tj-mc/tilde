use crate::ast::Expression;
use crate::evaluator::Evaluator;
use crate::value::Value;

pub fn eval_split(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("split requires exactly 2 arguments (string, delimiter)".to_string());
    }

    let string_val = evaluator.eval_expression(args[0].clone())?;
    let string = match string_val {
        Value::String(s) => s,
        _ => return Err("split first argument must be a string".to_string()),
    };

    let delimiter_val = evaluator.eval_expression(args[1].clone())?;
    let delimiter = match delimiter_val {
        Value::String(d) => d,
        _ => return Err("split second argument must be a string".to_string()),
    };

    let parts: Vec<Value> = string
        .split(&delimiter)
        .map(|s| Value::String(s.to_string()))
        .collect();

    Ok(Value::List(parts))
}

pub fn eval_join(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("join requires exactly 2 arguments (list, delimiter)".to_string());
    }

    let list_val = evaluator.eval_expression(args[0].clone())?;
    let list = match list_val {
        Value::List(items) => items,
        _ => return Err("join first argument must be a list".to_string()),
    };

    let delimiter_val = evaluator.eval_expression(args[1].clone())?;
    let delimiter = match delimiter_val {
        Value::String(d) => d,
        _ => return Err("join second argument must be a string".to_string()),
    };

    let strings: Result<Vec<String>, String> = list
        .into_iter()
        .map(|item| match item {
            Value::String(s) => Ok(s),
            Value::Number(n) => Ok(n.to_string()),
            Value::Boolean(b) => Ok(b.to_string()),
            _ => Err("join can only work with strings, numbers, or booleans".to_string()),
        })
        .collect();

    match strings {
        Ok(strs) => Ok(Value::String(strs.join(&delimiter))),
        Err(e) => Err(e),
    }
}

pub fn eval_trim(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("trim requires exactly 1 argument (string)".to_string());
    }

    let string_val = evaluator.eval_expression(args[0].clone())?;
    let string = match string_val {
        Value::String(s) => s,
        _ => return Err("trim argument must be a string".to_string()),
    };

    Ok(Value::String(string.trim().to_string()))
}