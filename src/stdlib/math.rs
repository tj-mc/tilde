use crate::ast::Expression;
use crate::evaluator::Evaluator;
use crate::value::Value;
use super::utils::*;

pub fn eval_absolute(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    let number = extract_number_arg(&args, evaluator, "absolute")?;
    Ok(Value::Number(number.abs()))
}

pub fn eval_square_root(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    let number = extract_number_arg(&args, evaluator, "square-root")?;
    if number < 0.0 {
        return Err("square-root argument must be non-negative".to_string());
    }
    Ok(Value::Number(number.sqrt()))
}