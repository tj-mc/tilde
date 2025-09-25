use crate::ast::Expression;
use crate::evaluator::Evaluator;
use crate::value::Value;
use std::fs;
use std::path::Path;

/// Checks if a file exists
pub fn eval_file_exists(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("file-exists requires exactly 1 argument".to_string());
    }

    let value = evaluator.eval_expression(args[0].clone())?;
    let path = match value {
        Value::String(s) => s,
        _ => return Err("file-exists argument must be a string".to_string()),
    };

    let exists = Path::new(&path).is_file();
    Ok(Value::Boolean(exists))
}

/// Checks if a directory exists
pub fn eval_dir_exists(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("dir-exists requires exactly 1 argument".to_string());
    }

    let value = evaluator.eval_expression(args[0].clone())?;
    let path = match value {
        Value::String(s) => s,
        _ => return Err("dir-exists argument must be a string".to_string()),
    };

    let exists = Path::new(&path).is_dir();
    Ok(Value::Boolean(exists))
}

/// Gets the size of a file in bytes
pub fn eval_file_size(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("file-size requires exactly 1 argument".to_string());
    }

    let value = evaluator.eval_expression(args[0].clone())?;
    let path = match value {
        Value::String(s) => s,
        _ => return Err("file-size argument must be a string".to_string()),
    };

    match fs::metadata(&path) {
        Ok(metadata) => Ok(Value::Number(metadata.len() as f64)),
        Err(e) => Err(format!("Error getting file size: {}", e)),
    }
}
