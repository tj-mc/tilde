use crate::ast::Expression;
use crate::evaluator::Evaluator;
use crate::value::Value;

/// Encodes a string to base64
pub fn eval_base64_encode(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("base64-encode requires exactly 1 argument".to_string());
    }

    let value = evaluator.eval_expression(args[0].clone())?;
    let string = match value {
        Value::String(s) => s,
        _ => return Err("base64-encode argument must be a string".to_string()),
    };

    let encoded = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, string.as_bytes());
    Ok(Value::String(encoded))
}

/// Decodes a base64 string
pub fn eval_base64_decode(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("base64-decode requires exactly 1 argument".to_string());
    }

    let value = evaluator.eval_expression(args[0].clone())?;
    let string = match value {
        Value::String(s) => s,
        _ => return Err("base64-decode argument must be a string".to_string()),
    };

    match base64::Engine::decode(&base64::engine::general_purpose::STANDARD, &string) {
        Ok(bytes) => {
            match String::from_utf8(bytes) {
                Ok(decoded) => Ok(Value::String(decoded)),
                Err(_) => Err("Invalid UTF-8 in decoded base64 data".to_string()),
            }
        }
        Err(_) => Err("Invalid base64 input".to_string()),
    }
}

/// URL-encodes a string
pub fn eval_url_encode(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("url-encode requires exactly 1 argument".to_string());
    }

    let value = evaluator.eval_expression(args[0].clone())?;
    let string = match value {
        Value::String(s) => s,
        _ => return Err("url-encode argument must be a string".to_string()),
    };

    let encoded = urlencoding::encode(&string);
    Ok(Value::String(encoded.to_string()))
}

/// URL-decodes a string
pub fn eval_url_decode(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("url-decode requires exactly 1 argument".to_string());
    }

    let value = evaluator.eval_expression(args[0].clone())?;
    let string = match value {
        Value::String(s) => s,
        _ => return Err("url-decode argument must be a string".to_string()),
    };

    match urlencoding::decode(&string) {
        Ok(decoded) => Ok(Value::String(decoded.to_string())),
        Err(_) => Err("Invalid URL encoding".to_string()),
    }
}