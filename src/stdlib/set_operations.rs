use crate::ast::Expression;
use crate::evaluator::Evaluator;
use crate::value::Value;
use std::collections::HashSet;

/// Returns the union of two lists (all unique elements from both lists)
pub fn eval_union(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("union requires exactly 2 arguments (list1, list2)".to_string());
    }

    let list1_val = evaluator.eval_expression(args[0].clone())?;
    let list2_val = evaluator.eval_expression(args[1].clone())?;

    let list1 = match list1_val {
        Value::List(l) => l,
        _ => return Err("union first argument must be a list".to_string()),
    };

    let list2 = match list2_val {
        Value::List(l) => l,
        _ => return Err("union second argument must be a list".to_string()),
    };

    let mut seen = HashSet::new();
    let mut result = Vec::new();

    // Add all items from list1
    for item in list1 {
        let key = value_to_hash_key(&item)?;
        if seen.insert(key) {
            result.push(item);
        }
    }

    // Add unique items from list2
    for item in list2 {
        let key = value_to_hash_key(&item)?;
        if seen.insert(key) {
            result.push(item);
        }
    }

    Ok(Value::List(result))
}

/// Returns the difference of two lists (elements in list1 but not in list2)
pub fn eval_difference(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("difference requires exactly 2 arguments (list1, list2)".to_string());
    }

    let list1_val = evaluator.eval_expression(args[0].clone())?;
    let list2_val = evaluator.eval_expression(args[1].clone())?;

    let list1 = match list1_val {
        Value::List(l) => l,
        _ => return Err("difference first argument must be a list".to_string()),
    };

    let list2 = match list2_val {
        Value::List(l) => l,
        _ => return Err("difference second argument must be a list".to_string()),
    };

    // Create a set of items from list2 for efficient lookup
    let mut list2_set = HashSet::new();
    for item in list2 {
        let key = value_to_hash_key(&item)?;
        list2_set.insert(key);
    }

    // Keep items from list1 that are not in list2
    let mut result = Vec::new();
    for item in list1 {
        let key = value_to_hash_key(&item)?;
        if !list2_set.contains(&key) {
            result.push(item);
        }
    }

    Ok(Value::List(result))
}

/// Returns the intersection of two lists (elements that appear in both lists)
pub fn eval_intersection(
    args: Vec<Expression>,
    evaluator: &mut Evaluator,
) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("intersection requires exactly 2 arguments (list1, list2)".to_string());
    }

    let list1_val = evaluator.eval_expression(args[0].clone())?;
    let list2_val = evaluator.eval_expression(args[1].clone())?;

    let list1 = match list1_val {
        Value::List(l) => l,
        _ => return Err("intersection first argument must be a list".to_string()),
    };

    let list2 = match list2_val {
        Value::List(l) => l,
        _ => return Err("intersection second argument must be a list".to_string()),
    };

    // Create a set of items from list2 for efficient lookup
    let mut list2_set = HashSet::new();
    for item in list2 {
        let key = value_to_hash_key(&item)?;
        list2_set.insert(key);
    }

    // Keep items from list1 that are also in list2, avoiding duplicates
    let mut result = Vec::new();
    let mut seen = HashSet::new();
    for item in list1 {
        let key = value_to_hash_key(&item)?;
        if list2_set.contains(&key) && seen.insert(key) {
            result.push(item);
        }
    }

    Ok(Value::List(result))
}

/// Helper function to convert a Value into a string key for hashing
/// This enables set operations on mixed types
fn value_to_hash_key(value: &Value) -> Result<String, String> {
    match value {
        Value::Number(n) => {
            if n.fract() == 0.0 {
                Ok(format!("n:{}", *n as i64))
            } else {
                Ok(format!("n:{}", n))
            }
        }
        Value::String(s) => Ok(format!("s:{}", s)),
        Value::Boolean(b) => Ok(format!("b:{}", b)),
        Value::Null => Ok("null".to_string()),
        Value::Date(dt) => Ok(format!("d:{}", dt.format("%Y-%m-%dT%H:%M:%SZ"))),
        Value::Error(err) => Ok(format!("e:{}", err.message)),
        Value::List(_) => {
            Err("Set operations cannot be performed on lists containing other lists".to_string())
        }
        Value::Object(_) => {
            Err("Set operations cannot be performed on lists containing objects".to_string())
        }
        Value::Pattern(pattern) => Ok(format!("p:{}", pattern.notation())),
    }
}
