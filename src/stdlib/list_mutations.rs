use crate::ast::Expression;
use crate::evaluator::Evaluator;
use crate::stdlib::utils::evaluate_args;
use crate::value::Value;

/// Remove first occurrence of a value from a list
/// Usage: remove list value
/// Returns: new list with first occurrence of value removed
pub fn eval_remove(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("remove requires exactly 2 arguments: list and value".to_string());
    }

    let (list, value) = match &evaluate_args(args, evaluator)?[..] {
        [list_val, value_val] => (list_val.clone(), value_val.clone()),
        _ => return Err("remove: invalid arguments".to_string()),
    };

    let list = match list {
        Value::List(l) => l,
        _ => return Err("remove: first argument must be a list".to_string()),
    };

    let mut result = Vec::new();
    let mut found = false;

    for item in list {
        if !found && item == value {
            found = true;
            continue;
        }
        result.push(item);
    }

    Ok(Value::List(result))
}

/// Remove item at specific index from a list
/// Usage: remove-at list index
/// Returns: new list with item at index removed
pub fn eval_remove_at(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("remove-at requires exactly 2 arguments: list and index".to_string());
    }

    let (list, index) = match &evaluate_args(args, evaluator)?[..] {
        [list_val, index_val] => (list_val.clone(), index_val.clone()),
        _ => return Err("remove-at: invalid arguments".to_string()),
    };

    let list = match list {
        Value::List(l) => l,
        _ => return Err("remove-at: first argument must be a list".to_string()),
    };

    let index = match index {
        Value::Number(n) => {
            if n.fract() != 0.0 {
                return Err("remove-at: index must be an integer".to_string());
            }
            n as usize
        }
        _ => return Err("remove-at: second argument must be a number".to_string()),
    };

    if index >= list.len() {
        return Err(format!(
            "remove-at: index {} out of bounds for list of length {}",
            index,
            list.len()
        ));
    }

    let mut result = Vec::new();
    for (i, item) in list.into_iter().enumerate() {
        if i != index {
            result.push(item);
        }
    }

    Ok(Value::List(result))
}

/// Insert value at specific index in a list
/// Usage: insert list index value
/// Returns: new list with value inserted at index
pub fn eval_insert(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 3 {
        return Err("insert requires exactly 3 arguments: list, index, and value".to_string());
    }

    let (list, index, value) = match &evaluate_args(args, evaluator)?[..] {
        [list_val, index_val, value_val] => {
            (list_val.clone(), index_val.clone(), value_val.clone())
        }
        _ => return Err("insert: invalid arguments".to_string()),
    };

    let list = match list {
        Value::List(l) => l,
        _ => return Err("insert: first argument must be a list".to_string()),
    };

    let index = match index {
        Value::Number(n) => {
            if n.fract() != 0.0 {
                return Err("insert: index must be an integer".to_string());
            }
            let idx = n as i32;
            if idx < 0 {
                return Err("insert: index must be non-negative".to_string());
            }
            idx as usize
        }
        _ => return Err("insert: second argument must be a number".to_string()),
    };

    if index > list.len() {
        return Err(format!(
            "insert: index {} out of bounds for list of length {} (max insertable index is {})",
            index,
            list.len(),
            list.len()
        ));
    }

    let mut result = list;
    result.insert(index, value);

    Ok(Value::List(result))
}

/// Set value at specific index in a list
/// Usage: set-at list index value
/// Returns: new list with value at index replaced
pub fn eval_set_at(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 3 {
        return Err("set-at requires exactly 3 arguments: list, index, and value".to_string());
    }

    let (list, index, value) = match &evaluate_args(args, evaluator)?[..] {
        [list_val, index_val, value_val] => {
            (list_val.clone(), index_val.clone(), value_val.clone())
        }
        _ => return Err("set-at: invalid arguments".to_string()),
    };

    let list = match list {
        Value::List(l) => l,
        _ => return Err("set-at: first argument must be a list".to_string()),
    };

    let index = match index {
        Value::Number(n) => {
            if n.fract() != 0.0 {
                return Err("set-at: index must be an integer".to_string());
            }
            n as usize
        }
        _ => return Err("set-at: second argument must be a number".to_string()),
    };

    if index >= list.len() {
        return Err(format!(
            "set-at: index {} out of bounds for list of length {}",
            index,
            list.len()
        ));
    }

    let mut result = list;
    result[index] = value;

    Ok(Value::List(result))
}

/// Remove and return the last element from a list
/// Usage: pop list
/// Returns: object with {value: last_element, list: new_list_without_last}
pub fn eval_pop(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("pop requires exactly 1 argument: list".to_string());
    }

    let list = match &evaluate_args(args, evaluator)?[..] {
        [list_val] => list_val.clone(),
        _ => return Err("pop: invalid arguments".to_string()),
    };

    let mut list = match list {
        Value::List(l) => l,
        _ => return Err("pop: argument must be a list".to_string()),
    };

    if list.is_empty() {
        return Err("pop: cannot pop from empty list".to_string());
    }

    let popped = list.pop().unwrap();

    let mut result = std::collections::HashMap::new();
    result.insert("value".to_string(), popped);
    result.insert("list".to_string(), Value::List(list));

    Ok(Value::Object(result))
}

/// Remove and return the first element from a list
/// Usage: shift list
/// Returns: object with {value: first_element, list: new_list_without_first}
pub fn eval_shift(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("shift requires exactly 1 argument: list".to_string());
    }

    let list = match &evaluate_args(args, evaluator)?[..] {
        [list_val] => list_val.clone(),
        _ => return Err("shift: invalid arguments".to_string()),
    };

    let mut list = match list {
        Value::List(l) => l,
        _ => return Err("shift: argument must be a list".to_string()),
    };

    if list.is_empty() {
        return Err("shift: cannot shift from empty list".to_string());
    }

    let shifted = list.remove(0);

    let mut result = std::collections::HashMap::new();
    result.insert("value".to_string(), shifted);
    result.insert("list".to_string(), Value::List(list));

    Ok(Value::Object(result))
}

/// Add element to the beginning of a list
/// Usage: unshift list value
/// Returns: new list with value prepended
pub fn eval_unshift(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("unshift requires exactly 2 arguments: list and value".to_string());
    }

    let (list, value) = match &evaluate_args(args, evaluator)?[..] {
        [list_val, value_val] => (list_val.clone(), value_val.clone()),
        _ => return Err("unshift: invalid arguments".to_string()),
    };

    let mut list = match list {
        Value::List(l) => l,
        _ => return Err("unshift: first argument must be a list".to_string()),
    };

    list.insert(0, value);

    Ok(Value::List(list))
}
