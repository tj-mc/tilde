use crate::ast::Expression;
use crate::evaluator::Evaluator;
use crate::value::Value;
use std::collections::HashMap;

/// Merges two objects, with the second object overwriting fields from the first
pub fn eval_merge(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("merge requires exactly 2 arguments".to_string());
    }

    let obj1_val = evaluator.eval_expression(args[0].clone())?;
    let obj2_val = evaluator.eval_expression(args[1].clone())?;

    let obj1 = match obj1_val {
        Value::Object(o) => o,
        _ => return Err("merge arguments must be objects".to_string()),
    };

    let obj2 = match obj2_val {
        Value::Object(o) => o,
        _ => return Err("merge arguments must be objects".to_string()),
    };

    let mut result = obj1.clone();
    for (key, value) in obj2 {
        result.insert(key, value);
    }

    Ok(Value::Object(result))
}

/// Picks specified fields from an object
pub fn eval_pick(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("pick requires exactly 2 arguments".to_string());
    }

    let obj_val = evaluator.eval_expression(args[0].clone())?;
    let fields_val = evaluator.eval_expression(args[1].clone())?;

    let obj = match obj_val {
        Value::Object(o) => o,
        _ => return Err("pick first argument must be an object".to_string()),
    };

    let fields = match fields_val {
        Value::List(list) => list,
        _ => return Err("pick second argument must be a list".to_string()),
    };

    let mut result = HashMap::new();
    for field_val in fields {
        match field_val {
            Value::String(field_name) => {
                if let Some(value) = obj.get(&field_name) {
                    result.insert(field_name, value.clone());
                }
            },
            _ => return Err("pick field names must be strings".to_string()),
        }
    }

    Ok(Value::Object(result))
}

/// Omits specified fields from an object
pub fn eval_omit(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("omit requires exactly 2 arguments".to_string());
    }

    let obj_val = evaluator.eval_expression(args[0].clone())?;
    let fields_val = evaluator.eval_expression(args[1].clone())?;

    let obj = match obj_val {
        Value::Object(o) => o,
        _ => return Err("omit first argument must be an object".to_string()),
    };

    let fields = match fields_val {
        Value::List(list) => list,
        _ => return Err("omit second argument must be a list".to_string()),
    };

    let mut omit_set = std::collections::HashSet::new();
    for field_val in fields {
        match field_val {
            Value::String(field_name) => {
                omit_set.insert(field_name);
            },
            _ => return Err("omit field names must be strings".to_string()),
        }
    }

    let mut result = HashMap::new();
    for (key, value) in obj {
        if !omit_set.contains(&key) {
            result.insert(key, value);
        }
    }

    Ok(Value::Object(result))
}

/// Gets a value from an object using a dot-separated path
pub fn eval_object_get(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("object-get requires exactly 2 arguments".to_string());
    }

    let obj_val = evaluator.eval_expression(args[0].clone())?;
    let path_val = evaluator.eval_expression(args[1].clone())?;

    let path = match path_val {
        Value::String(s) => s,
        _ => return Err("object-get path must be a string".to_string()),
    };

    let parts: Vec<&str> = path.split('.').collect();
    let mut current = &obj_val;

    for part in parts {
        match current {
            Value::Object(obj) => {
                if let Some(value) = obj.get(part) {
                    current = value;
                } else {
                    return Ok(Value::Null);
                }
            },
            _ => return Ok(Value::Null),
        }
    }

    Ok(current.clone())
}

/// Sets a value in an object using a dot-separated path
pub fn eval_object_set(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 3 {
        return Err("object-set requires exactly 3 arguments".to_string());
    }

    let obj_val = evaluator.eval_expression(args[0].clone())?;
    let path_val = evaluator.eval_expression(args[1].clone())?;
    let value_val = evaluator.eval_expression(args[2].clone())?;

    let path = match path_val {
        Value::String(s) => s,
        _ => return Err("object-set path must be a string".to_string()),
    };

    let parts: Vec<&str> = path.split('.').collect();
    let mut result = obj_val.clone();

    set_nested_value(&mut result, &parts, value_val)?;

    Ok(result)
}

/// Helper function to set a nested value in an object
fn set_nested_value(obj: &mut Value, path: &[&str], value: Value) -> Result<(), String> {
    if path.is_empty() {
        return Err("Empty path".to_string());
    }

    if path.len() == 1 {
        match obj {
            Value::Object(map) => {
                map.insert(path[0].to_string(), value);
                Ok(())
            },
            _ => Err("Cannot set property on non-object".to_string()),
        }
    } else {
        match obj {
            Value::Object(map) => {
                let key = path[0].to_string();
                if !map.contains_key(&key) {
                    map.insert(key.clone(), Value::Object(HashMap::new()));
                }
                if let Some(nested) = map.get_mut(&key) {
                    set_nested_value(nested, &path[1..], value)
                } else {
                    Err("Failed to access nested object".to_string())
                }
            },
            _ => Err("Cannot access property on non-object".to_string()),
        }
    }
}

/// Deep merges two objects recursively
pub fn eval_deep_merge(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("deep-merge requires exactly 2 arguments".to_string());
    }

    let obj1_val = evaluator.eval_expression(args[0].clone())?;
    let obj2_val = evaluator.eval_expression(args[1].clone())?;

    let result = deep_merge_values(obj1_val, obj2_val)?;
    Ok(result)
}

/// Helper function to recursively merge two values
fn deep_merge_values(val1: Value, val2: Value) -> Result<Value, String> {
    match (val1, val2) {
        (Value::Object(mut obj1), Value::Object(obj2)) => {
            for (key, val2) in obj2 {
                if let Some(val1) = obj1.remove(&key) {
                    obj1.insert(key, deep_merge_values(val1, val2)?);
                } else {
                    obj1.insert(key, val2);
                }
            }
            Ok(Value::Object(obj1))
        },
        (_, val2) => Ok(val2),
    }
}