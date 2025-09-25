use super::utils::*;
use crate::ast::Expression;
use crate::evaluator::Evaluator;
use crate::value::Value;

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

/// Checks if a string starts with a prefix
pub fn eval_starts_with(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    let (string, prefix) = extract_string_string_args(&args, evaluator, "starts-with")?;
    Ok(Value::Boolean(string.starts_with(&prefix)))
}

/// Checks if a string ends with a suffix
pub fn eval_ends_with(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    let (string, suffix) = extract_string_string_args(&args, evaluator, "ends-with")?;
    Ok(Value::Boolean(string.ends_with(&suffix)))
}

/// Extracts a substring from a string
pub fn eval_substring(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() < 2 || args.len() > 3 {
        return Err("substring requires 2 or 3 arguments: string, start, [length]".to_string());
    }

    let string = extract_string_value(&args[0], evaluator)?;
    let start = extract_number_value(&args[1], evaluator)? as usize;

    let chars: Vec<char> = string.chars().collect();

    if start >= chars.len() {
        return Ok(Value::String(String::new()));
    }

    let end = if args.len() == 3 {
        let length = extract_number_value(&args[2], evaluator)? as usize;
        std::cmp::min(start + length, chars.len())
    } else {
        chars.len()
    };

    let result: String = chars[start..end].iter().collect();
    Ok(Value::String(result))
}

/// Replaces all occurrences of a substring with another string
pub fn eval_replace(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 3 {
        return Err("replace requires 3 arguments: string, old, new".to_string());
    }

    let string = extract_string_value(&args[0], evaluator)?;
    let old = extract_string_value(&args[1], evaluator)?;
    let new = extract_string_value(&args[2], evaluator)?;

    Ok(Value::String(string.replace(&old, &new)))
}

/// Repeats a string a specified number of times
pub fn eval_repeat(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    let (string, count) = extract_string_number_args(&args, evaluator, "repeat")?;

    if count < 0.0 {
        return Err("repeat count cannot be negative".to_string());
    }

    let count = count as usize;
    Ok(Value::String(string.repeat(count)))
}

/// Pads a string on the left to a specified length
pub fn eval_pad_left(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() < 2 || args.len() > 3 {
        return Err("pad-left requires 2 or 3 arguments: string, length, [char]".to_string());
    }

    let string = extract_string_value(&args[0], evaluator)?;
    let target_length = extract_number_value(&args[1], evaluator)? as usize;

    let pad_char = if args.len() == 3 {
        let pad_str = extract_string_value(&args[2], evaluator)?;
        if pad_str.len() != 1 {
            return Err("pad character must be exactly one character".to_string());
        }
        pad_str.chars().next().unwrap()
    } else {
        ' '
    };

    if string.len() >= target_length {
        Ok(Value::String(string))
    } else {
        let padding_needed = target_length - string.len();
        let padding: String = std::iter::repeat(pad_char).take(padding_needed).collect();
        Ok(Value::String(format!("{}{}", padding, string)))
    }
}

/// Pads a string on the right to a specified length
pub fn eval_pad_right(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() < 2 || args.len() > 3 {
        return Err("pad-right requires 2 or 3 arguments: string, length, [char]".to_string());
    }

    let string = extract_string_value(&args[0], evaluator)?;
    let target_length = extract_number_value(&args[1], evaluator)? as usize;

    let pad_char = if args.len() == 3 {
        let pad_str = extract_string_value(&args[2], evaluator)?;
        if pad_str.len() != 1 {
            return Err("pad character must be exactly one character".to_string());
        }
        pad_str.chars().next().unwrap()
    } else {
        ' '
    };

    if string.len() >= target_length {
        Ok(Value::String(string))
    } else {
        let padding_needed = target_length - string.len();
        let padding: String = std::iter::repeat(pad_char).take(padding_needed).collect();
        Ok(Value::String(format!("{}{}", string, padding)))
    }
}
