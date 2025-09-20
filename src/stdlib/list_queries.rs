use crate::ast::Expression;
use crate::evaluator::Evaluator;
use crate::value::Value;
use crate::stdlib::utils::evaluate_args;

/// Find the index of the first occurrence of a value in a list
/// Usage: index-of list value
/// Returns: index as number or null if not found
pub fn eval_index_of(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("index-of requires exactly 2 arguments: list and value".to_string());
    }

    let (list, value) = match &evaluate_args(args, evaluator)?[..] {
        [list_val, value_val] => (list_val.clone(), value_val.clone()),
        _ => return Err("index-of: invalid arguments".to_string()),
    };

    let list = match list {
        Value::List(l) => l,
        _ => return Err("index-of: first argument must be a list".to_string()),
    };

    for (index, item) in list.iter().enumerate() {
        if *item == value {
            return Ok(Value::Number(index as f64));
        }
    }

    Ok(Value::Null)
}

/// Check if a list contains a specific value
/// Usage: contains list value
/// Returns: true if value is in list, false otherwise
pub fn eval_contains(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("contains requires exactly 2 arguments: list and value".to_string());
    }

    let (list, value) = match &evaluate_args(args, evaluator)?[..] {
        [list_val, value_val] => (list_val.clone(), value_val.clone()),
        _ => return Err("contains: invalid arguments".to_string()),
    };

    let list = match list {
        Value::List(l) => l,
        _ => return Err("contains: first argument must be a list".to_string()),
    };

    Ok(Value::Boolean(list.contains(&value)))
}

/// Extract a slice from a list
/// Usage: slice list start [end]
/// Returns: new list containing elements from start to end (exclusive)
/// If end is not provided, slice to the end of the list
pub fn eval_slice(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() < 2 || args.len() > 3 {
        return Err("slice requires 2-3 arguments: list, start, and optional end".to_string());
    }

    let evaluated_args = evaluate_args(args, evaluator)?;
    let list = match &evaluated_args[0] {
        Value::List(l) => l,
        _ => return Err("slice: first argument must be a list".to_string()),
    };

    let start = match &evaluated_args[1] {
        Value::Number(n) => {
            if n.fract() != 0.0 {
                return Err("slice: start index must be an integer".to_string());
            }
            let start_idx = *n as i32;
            if start_idx < 0 {
                return Err("slice: start index must be non-negative".to_string());
            }
            start_idx as usize
        }
        _ => return Err("slice: start index must be a number".to_string()),
    };

    let end = if evaluated_args.len() == 3 {
        match &evaluated_args[2] {
            Value::Number(n) => {
                if n.fract() != 0.0 {
                    return Err("slice: end index must be an integer".to_string());
                }
                let end_idx = *n as i32;
                if end_idx < 0 {
                    return Err("slice: end index must be non-negative".to_string());
                }
                Some(end_idx as usize)
            }
            _ => return Err("slice: end index must be a number".to_string()),
        }
    } else {
        None
    };

    if start > list.len() {
        return Ok(Value::List(vec![]));
    }

    let end = end.unwrap_or(list.len());
    if end < start {
        return Err("slice: end index must be greater than or equal to start index".to_string());
    }

    let slice_end = std::cmp::min(end, list.len());
    let result = list[start..slice_end].to_vec();

    Ok(Value::List(result))
}

/// Concatenate multiple lists into one
/// Usage: concat list1 list2 [list3 ...]
/// Returns: new list containing all elements from input lists
pub fn eval_concat(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.is_empty() {
        return Err("concat requires at least 1 argument".to_string());
    }

    let evaluated_args = evaluate_args(args, evaluator)?;
    let mut result = Vec::new();

    for arg in evaluated_args {
        match arg {
            Value::List(l) => {
                result.extend(l);
            }
            _ => return Err("concat: all arguments must be lists".to_string()),
        }
    }

    Ok(Value::List(result))
}

/// Get the first n elements from a list
/// Usage: take list n
/// Returns: new list with first n elements
pub fn eval_take(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("take requires exactly 2 arguments: list and count".to_string());
    }

    let (list, count) = match &evaluate_args(args, evaluator)?[..] {
        [list_val, count_val] => (list_val.clone(), count_val.clone()),
        _ => return Err("take: invalid arguments".to_string()),
    };

    let list = match list {
        Value::List(l) => l,
        _ => return Err("take: first argument must be a list".to_string()),
    };

    let count = match count {
        Value::Number(n) => {
            if n.fract() != 0.0 {
                return Err("take: count must be an integer".to_string());
            }
            let count_val = n as i32;
            if count_val < 0 {
                return Err("take: count must be non-negative".to_string());
            }
            count_val as usize
        }
        _ => return Err("take: second argument must be a number".to_string()),
    };

    let take_count = std::cmp::min(count, list.len());
    let result = list[..take_count].to_vec();

    Ok(Value::List(result))
}

/// Skip the first n elements and return the rest
/// Usage: drop list n
/// Returns: new list without first n elements
pub fn eval_drop(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("drop requires exactly 2 arguments: list and count".to_string());
    }

    let (list, count) = match &evaluate_args(args, evaluator)?[..] {
        [list_val, count_val] => (list_val.clone(), count_val.clone()),
        _ => return Err("drop: invalid arguments".to_string()),
    };

    let list = match list {
        Value::List(l) => l,
        _ => return Err("drop: first argument must be a list".to_string()),
    };

    let count = match count {
        Value::Number(n) => {
            if n.fract() != 0.0 {
                return Err("drop: count must be an integer".to_string());
            }
            let count_val = n as i32;
            if count_val < 0 {
                return Err("drop: count must be non-negative".to_string());
            }
            count_val as usize
        }
        _ => return Err("drop: second argument must be a number".to_string()),
    };

    if count >= list.len() {
        return Ok(Value::List(vec![]));
    }

    let result = list[count..].to_vec();
    Ok(Value::List(result))
}

