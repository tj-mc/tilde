use crate::ast::Expression;
use crate::evaluator::Evaluator;
use crate::stdlib::utils::evaluate_args;
use crate::value::Value;

/// Flatten a nested list structure into a single list
/// Usage: flatten list [depth]
/// Returns: new list with nested lists flattened
/// If depth is provided, only flatten that many levels (default: infinite)
pub fn eval_flatten(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.is_empty() || args.len() > 2 {
        return Err("flatten requires 1-2 arguments: list and optional depth".to_string());
    }

    let evaluated_args = evaluate_args(args, evaluator)?;
    let list = match &evaluated_args[0] {
        Value::List(l) => l,
        _ => return Err("flatten: first argument must be a list".to_string()),
    };

    let depth = if evaluated_args.len() == 2 {
        match &evaluated_args[1] {
            Value::Number(n) => {
                if n.fract() != 0.0 {
                    return Err("flatten: depth must be an integer".to_string());
                }
                let depth_val = *n as i32;
                if depth_val < 0 {
                    return Err("flatten: depth must be non-negative".to_string());
                }
                Some(depth_val as usize)
            }
            _ => return Err("flatten: depth must be a number".to_string()),
        }
    } else {
        None
    };

    fn flatten_recursive(
        list: &[Value],
        max_depth: Option<usize>,
        current_depth: usize,
    ) -> Vec<Value> {
        let mut result = Vec::new();

        for item in list {
            match item {
                Value::List(nested) => {
                    if max_depth.is_none() || current_depth < max_depth.unwrap() {
                        result.extend(flatten_recursive(nested, max_depth, current_depth + 1));
                    } else {
                        result.push(item.clone());
                    }
                }
                _ => result.push(item.clone()),
            }
        }

        result
    }

    let result = flatten_recursive(list, depth, 0);
    Ok(Value::List(result))
}

/// Remove duplicate values from a list, preserving order
/// Usage: unique list
/// Returns: new list with duplicates removed
pub fn eval_unique(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("unique requires exactly 1 argument: list".to_string());
    }

    let list = match &evaluate_args(args, evaluator)?[..] {
        [list_val] => list_val.clone(),
        _ => return Err("unique: invalid arguments".to_string()),
    };

    let list = match list {
        Value::List(l) => l,
        _ => return Err("unique: argument must be a list".to_string()),
    };

    let mut result = Vec::new();
    let mut seen = std::collections::HashSet::new();

    for item in list {
        // Create a simple hash key for the value
        let key = match &item {
            Value::Number(n) => format!("n:{}", n),
            Value::String(s) => format!("s:{}", s),
            Value::Boolean(b) => format!("b:{}", b),
            Value::Null => "null".to_string(),
            Value::List(_) => {
                // For lists, we'll use the debug representation as a simple hash
                format!("l:{:?}", item)
            }
            Value::Object(_) => {
                // For objects, we'll use the debug representation as a simple hash
                format!("o:{:?}", item)
            }
            Value::Date(dt) => format!("d:{}", dt.format("%Y-%m-%dT%H:%M:%SZ")),
            Value::Error(err) => format!("e:{}", err.message),
            Value::Pattern(pattern) => format!("p:{}", pattern.notation()),
        };

        if seen.insert(key) {
            result.push(item);
        }
    }

    Ok(Value::List(result))
}

/// Combine two lists element-wise into pairs
/// Usage: zip list1 list2
/// Returns: new list of [element1, element2] pairs
/// Length is determined by the shorter list
pub fn eval_zip(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("zip requires exactly 2 arguments: list1 and list2".to_string());
    }

    let (list1, list2) = match &evaluate_args(args, evaluator)?[..] {
        [list1_val, list2_val] => (list1_val.clone(), list2_val.clone()),
        _ => return Err("zip: invalid arguments".to_string()),
    };

    let list1 = match list1 {
        Value::List(l) => l,
        _ => return Err("zip: first argument must be a list".to_string()),
    };

    let list2 = match list2 {
        Value::List(l) => l,
        _ => return Err("zip: second argument must be a list".to_string()),
    };

    let mut result = Vec::new();
    let min_len = std::cmp::min(list1.len(), list2.len());

    for i in 0..min_len {
        result.push(Value::List(vec![list1[i].clone(), list2[i].clone()]));
    }

    Ok(Value::List(result))
}

/// Partition a list into chunks of specified size
/// Usage: chunk list size
/// Returns: new list of lists, each containing 'size' elements (last chunk may be smaller)
pub fn eval_chunk(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("chunk requires exactly 2 arguments: list and size".to_string());
    }

    let (list, size) = match &evaluate_args(args, evaluator)?[..] {
        [list_val, size_val] => (list_val.clone(), size_val.clone()),
        _ => return Err("chunk: invalid arguments".to_string()),
    };

    let list = match list {
        Value::List(l) => l,
        _ => return Err("chunk: first argument must be a list".to_string()),
    };

    let size = match size {
        Value::Number(n) => {
            if n.fract() != 0.0 {
                return Err("chunk: size must be an integer".to_string());
            }
            let size_val = n as i32;
            if size_val <= 0 {
                return Err("chunk: size must be positive".to_string());
            }
            size_val as usize
        }
        _ => return Err("chunk: size must be a number".to_string()),
    };

    let mut result = Vec::new();
    let mut i = 0;

    while i < list.len() {
        let end = std::cmp::min(i + size, list.len());
        result.push(Value::List(list[i..end].to_vec()));
        i += size;
    }

    Ok(Value::List(result))
}

/// Transpose a matrix (list of lists)
/// Usage: transpose matrix
/// Returns: transposed matrix where rows become columns
pub fn eval_transpose(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("transpose requires exactly 1 argument: matrix".to_string());
    }

    let matrix = match &evaluate_args(args, evaluator)?[..] {
        [matrix_val] => matrix_val.clone(),
        _ => return Err("transpose: invalid arguments".to_string()),
    };

    let matrix = match matrix {
        Value::List(rows) => rows,
        _ => return Err("transpose: argument must be a list".to_string()),
    };

    if matrix.is_empty() {
        return Ok(Value::List(vec![]));
    }

    // Validate that all elements are lists and find max length
    let mut max_len = 0;
    let mut row_lists = Vec::new();

    for row in matrix {
        match row {
            Value::List(l) => {
                max_len = std::cmp::max(max_len, l.len());
                row_lists.push(l);
            }
            _ => return Err("transpose: all elements must be lists".to_string()),
        }
    }

    if max_len == 0 {
        return Ok(Value::List(vec![]));
    }

    // Transpose the matrix
    let mut result = Vec::new();
    for col_idx in 0..max_len {
        let mut column = Vec::new();
        for row in &row_lists {
            if col_idx < row.len() {
                column.push(row[col_idx].clone());
            } else {
                column.push(Value::Null);
            }
        }
        result.push(Value::List(column));
    }

    Ok(Value::List(result))
}
