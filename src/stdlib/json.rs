use crate::ast::Expression;
use crate::evaluator::Evaluator;
use crate::value::Value;
use std::collections::HashMap;

/// Converts a Tilde value to JSON string
pub fn eval_to_json(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("to-json requires exactly 1 argument".to_string());
    }

    let value = evaluator.eval_expression(args[0].clone())?;
    let json_value = tilde_value_to_json_value(&value)?;

    match serde_json::to_string(&json_value) {
        Ok(json_string) => Ok(Value::String(json_string)),
        Err(e) => Err(format!("JSON serialization error: {}", e)),
    }
}

/// Converts a JSON string to a Tilde value
pub fn eval_from_json(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("from-json requires exactly 1 argument".to_string());
    }

    let value = evaluator.eval_expression(args[0].clone())?;
    let json_string = match value {
        Value::String(s) => s,
        _ => return Err("from-json argument must be a string".to_string()),
    };

    match serde_json::from_str::<serde_json::Value>(&json_string) {
        Ok(json_value) => {
            let tilde_value = json_value_to_tilde_value(json_value)?;
            Ok(tilde_value)
        }
        Err(e) => Err(format!("JSON parsing error: {}", e)),
    }
}

/// Convert a Tilde Value to a serde_json::Value
fn tilde_value_to_json_value(value: &Value) -> Result<serde_json::Value, String> {
    match value {
        Value::Number(n) => Ok(serde_json::Value::Number(
            serde_json::Number::from_f64(*n).ok_or("Invalid number for JSON")?
        )),
        Value::String(s) => Ok(serde_json::Value::String(s.clone())),
        Value::Boolean(b) => Ok(serde_json::Value::Bool(*b)),
        Value::Null => Ok(serde_json::Value::Null),
        Value::List(items) => {
            let json_items: Result<Vec<serde_json::Value>, String> = items
                .iter()
                .map(tilde_value_to_json_value)
                .collect();
            Ok(serde_json::Value::Array(json_items?))
        }
        Value::Object(map) => {
            let mut json_map = serde_json::Map::new();
            for (key, val) in map {
                json_map.insert(key.clone(), tilde_value_to_json_value(val)?);
            }
            Ok(serde_json::Value::Object(json_map))
        }
        Value::Date(dt) => {
            // Convert date to ISO 8601 string
            Ok(serde_json::Value::String(dt.format("%Y-%m-%dT%H:%M:%SZ").to_string()))
        }
        Value::Error(err) => {
            // Convert error to a JSON object
            let mut error_obj = serde_json::Map::new();
            error_obj.insert("message".to_string(), serde_json::Value::String(err.message.clone()));
            error_obj.insert("code".to_string(), match &err.code {
                Some(code) => serde_json::Value::String(code.clone()),
                None => serde_json::Value::Null,
            });
            Ok(serde_json::Value::Object(error_obj))
        }
    }
}

/// Convert a serde_json::Value to a Tilde Value
fn json_value_to_tilde_value(json_value: serde_json::Value) -> Result<Value, String> {
    match json_value {
        serde_json::Value::Null => Ok(Value::Null),
        serde_json::Value::Bool(b) => Ok(Value::Boolean(b)),
        serde_json::Value::Number(n) => {
            if let Some(f) = n.as_f64() {
                Ok(Value::Number(f))
            } else {
                Err("Invalid number in JSON".to_string())
            }
        }
        serde_json::Value::String(s) => Ok(Value::String(s)),
        serde_json::Value::Array(arr) => {
            let tilde_items: Result<Vec<Value>, String> = arr
                .into_iter()
                .map(json_value_to_tilde_value)
                .collect();
            Ok(Value::List(tilde_items?))
        }
        serde_json::Value::Object(obj) => {
            let mut tilde_map = HashMap::new();
            for (key, val) in obj {
                tilde_map.insert(key, json_value_to_tilde_value(val)?);
            }
            Ok(Value::Object(tilde_map))
        }
    }
}