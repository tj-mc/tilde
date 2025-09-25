use crate::ast::Expression;
use crate::evaluator::Evaluator;
use crate::value::Value;
use hmac::{Hmac, Mac};
use md5::Md5;
use sha2::{Digest, Sha256};

/// Computes SHA256 hash of a string and returns it as hexadecimal
pub fn eval_sha256(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("sha256 requires exactly 1 argument".to_string());
    }

    let value = evaluator.eval_expression(args[0].clone())?;
    let string = match value {
        Value::String(s) => s,
        _ => return Err("sha256 argument must be a string".to_string()),
    };

    let mut hasher = Sha256::new();
    hasher.update(string.as_bytes());
    let result = hasher.finalize();
    let hex_string = hex::encode(result);

    Ok(Value::String(hex_string))
}

/// Computes MD5 hash of a string and returns it as hexadecimal
pub fn eval_md5(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("md5 requires exactly 1 argument".to_string());
    }

    let value = evaluator.eval_expression(args[0].clone())?;
    let string = match value {
        Value::String(s) => s,
        _ => return Err("md5 argument must be a string".to_string()),
    };

    let mut hasher = Md5::new();
    hasher.update(string.as_bytes());
    let result = hasher.finalize();
    let hex_string = hex::encode(result);

    Ok(Value::String(hex_string))
}

/// Computes HMAC-SHA256 of a message with a key and returns it as hexadecimal
pub fn eval_hmac_sha256(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("hmac-sha256 requires exactly 2 arguments".to_string());
    }

    let key_value = evaluator.eval_expression(args[0].clone())?;
    let key = match key_value {
        Value::String(s) => s,
        _ => return Err("hmac-sha256 key must be a string".to_string()),
    };

    let message_value = evaluator.eval_expression(args[1].clone())?;
    let message = match message_value {
        Value::String(s) => s,
        _ => return Err("hmac-sha256 message must be a string".to_string()),
    };

    type HmacSha256 = Hmac<Sha256>;

    let mut mac = HmacSha256::new_from_slice(key.as_bytes()).map_err(|_| "Invalid HMAC key")?;

    mac.update(message.as_bytes());
    let result = mac.finalize();
    let hex_string = hex::encode(result.into_bytes());

    Ok(Value::String(hex_string))
}
