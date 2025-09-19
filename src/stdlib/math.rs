use crate::ast::Expression;
use crate::evaluator::Evaluator;
use crate::value::Value;

pub fn eval_absolute(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("absolute requires exactly 1 argument (number)".to_string());
    }

    let number_val = evaluator.eval_expression(args[0].clone())?;
    let number = match number_val {
        Value::Number(n) => n,
        _ => return Err("absolute argument must be a number".to_string()),
    };

    Ok(Value::Number(number.abs()))
}

pub fn eval_square_root(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("square-root requires exactly 1 argument (number)".to_string());
    }

    let number_val = evaluator.eval_expression(args[0].clone())?;
    let number = match number_val {
        Value::Number(n) => n,
        _ => return Err("square-root argument must be a number".to_string()),
    };

    if number < 0.0 {
        return Err("square-root argument must be non-negative".to_string());
    }

    Ok(Value::Number(number.sqrt()))
}