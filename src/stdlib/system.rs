use crate::ast::Expression;
use crate::evaluator::Evaluator;
use crate::value::Value;
use super::utils::*;

/// Get environment variable by name
/// Returns the environment variable value as a string, or null if not found
///
/// # Examples
/// ```tilde
/// ~port is env "PORT"                    # Returns "8080" or null
/// ~db_url is env "DATABASE_URL" or "localhost"  # With fallback
/// ```
pub fn eval_env(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    let var_name = extract_string_arg(&args, evaluator, "env")?;

    match std::env::var(&var_name) {
        Ok(value) => Ok(Value::String(value)),
        Err(_) => Ok(Value::Null),
    }
}