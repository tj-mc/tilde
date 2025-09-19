use crate::ast::Expression;
use crate::evaluator::Evaluator;
use crate::value::Value;
use super::utils::*;

pub fn eval_split(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    let (string, delimiter) = extract_string_string_args(&args, evaluator, "split")?;
    let parts: Vec<Value> = string
        .split(&delimiter)
        .map(|s| Value::String(s.to_string()))
        .collect();
    Ok(Value::List(parts))
}

pub fn eval_join(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    let (list, delimiter) = extract_list_string_args(&args, evaluator, "join")?;

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
    let string = extract_string_arg(&args, evaluator, "trim")?;
    Ok(Value::String(string.trim().to_string()))
}

/// Converts a string to uppercase
pub fn eval_uppercase(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    let string = extract_string_arg(&args, evaluator, "uppercase")?;
    Ok(Value::String(string.to_uppercase()))
}

/// Converts a string to lowercase
pub fn eval_lowercase(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    let string = extract_string_arg(&args, evaluator, "lowercase")?;
    Ok(Value::String(string.to_lowercase()))
}


