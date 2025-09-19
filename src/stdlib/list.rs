use crate::ast::Expression;
use crate::evaluator::Evaluator;
use crate::value::Value;

/// Transforms each element in a list using the provided function.
///
/// # Arguments
/// * `list` - The list to transform
/// * `function_name` - Name of the action that takes one parameter and returns a value
///
/// # Returns
/// A new list with each element transformed by the function
///
/// # Errors
/// * If first argument is not a list
/// * If second argument is not a valid function name
/// * If the function doesn't exist or doesn't take exactly one parameter

pub fn eval_map(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("map requires exactly 2 arguments (list, function_name)".to_string());
    }

    let list_val = evaluator.eval_expression(args[0].clone())?;
    let list = match list_val {
        Value::List(items) => items,
        _ => return Err("map first argument must be a list".to_string()),
    };

    // Get function name for the transformation
    let function_name = match &args[1] {
        Expression::Variable(name) => name.clone(),
        Expression::FunctionCall { name, args } if args.is_empty() => name.clone(),
        _ => return Err("map second argument must be a function name (use function name without *, like 'double')".to_string()),
    };

    // Check if the action exists
    if !evaluator.actions.contains_key(&function_name) {
        return Err(format!("Action '{}' not found", function_name));
    }

    let action = evaluator.actions.get(&function_name).unwrap().clone();

    // Check that the action takes exactly one parameter
    if action.params.len() != 1 {
        return Err(format!("Action '{}' must take exactly one parameter for map", function_name));
    }

    let mut result = Vec::new();
    for item in list {
        // Create a single-element argument list with the current item
        let arg_expr = match &item {
            Value::Number(n) => Expression::Number(*n, false),
            Value::String(s) => Expression::String(s.clone()),
            Value::Boolean(b) => Expression::Boolean(*b),
            _ => return Err("map can only work with numbers, strings, and booleans for now".to_string()),
        };

        let mapped_value = evaluator.eval_action(action.clone(), vec![arg_expr])?;
        result.push(mapped_value);
    }

    Ok(Value::List(result))
}

/// Filters elements in a list using the provided predicate function.
///
/// # Arguments
/// * `list` - The list to filter
/// * `function_name` - Name of the action that takes one parameter and returns a boolean
///
/// # Returns
/// A new list containing only elements for which the predicate returns true
///
/// # Errors
/// * If first argument is not a list
/// * If second argument is not a valid function name
/// * If the function doesn't exist or doesn't take exactly one parameter
pub fn eval_filter(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("filter requires exactly 2 arguments (list, function_name)".to_string());
    }

    let list_val = evaluator.eval_expression(args[0].clone())?;
    let list = match list_val {
        Value::List(items) => items,
        _ => return Err("filter first argument must be a list".to_string()),
    };

    let function_name = match &args[1] {
        Expression::Variable(name) => name.clone(),
        Expression::FunctionCall { name, args } if args.is_empty() => name.clone(),
        _ => return Err("filter second argument must be a function name (use function name without *, like 'is_even')".to_string()),
    };

    // Check if the action exists
    if !evaluator.actions.contains_key(&function_name) {
        return Err(format!("Action '{}' not found", function_name));
    }

    let action = evaluator.actions.get(&function_name).unwrap().clone();

    // Check that the action takes exactly one parameter
    if action.params.len() != 1 {
        return Err(format!("Action '{}' must take exactly one parameter for filter", function_name));
    }

    let mut result = Vec::new();
    for item in list {
        // Create a single-element argument list with the current item
        let arg_expr = match &item {
            Value::Number(n) => Expression::Number(*n, false),
            Value::String(s) => Expression::String(s.clone()),
            Value::Boolean(b) => Expression::Boolean(*b),
            _ => return Err("filter can only work with numbers, strings, and booleans for now".to_string()),
        };

        let predicate_result = evaluator.eval_action(action.clone(), vec![arg_expr])?;
        if let Value::Boolean(true) = predicate_result {
            result.push(item);
        }
    }

    Ok(Value::List(result))
}

pub fn eval_reduce(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 3 {
        return Err("reduce requires exactly 3 arguments (list, function_name, initial_value)".to_string());
    }

    let list_val = evaluator.eval_expression(args[0].clone())?;
    let list = match list_val {
        Value::List(items) => items,
        _ => return Err("reduce first argument must be a list".to_string()),
    };

    let function_name = match &args[1] {
        Expression::Variable(name) => name.clone(),
        Expression::FunctionCall { name, args } if args.is_empty() => name.clone(),
        _ => return Err("reduce second argument must be a function name (use function name without *, like 'add')".to_string()),
    };

    // Check if the action exists
    if !evaluator.actions.contains_key(&function_name) {
        return Err(format!("Action '{}' not found", function_name));
    }

    let action = evaluator.actions.get(&function_name).unwrap().clone();

    // Check that the action takes exactly two parameters
    if action.params.len() != 2 {
        return Err(format!("Action '{}' must take exactly two parameters for reduce", function_name));
    }

    let mut accumulator = evaluator.eval_expression(args[2].clone())?;

    for item in list {
        // Create arguments for the accumulator and current item
        let acc_expr = match &accumulator {
            Value::Number(n) => Expression::Number(*n, false),
            Value::String(s) => Expression::String(s.clone()),
            Value::Boolean(b) => Expression::Boolean(*b),
            _ => return Err("reduce can only work with numbers, strings, and booleans for now".to_string()),
        };

        let item_expr = match &item {
            Value::Number(n) => Expression::Number(*n, false),
            Value::String(s) => Expression::String(s.clone()),
            Value::Boolean(b) => Expression::Boolean(*b),
            _ => return Err("reduce can only work with numbers, strings, and booleans for now".to_string()),
        };

        accumulator = evaluator.eval_action(action.clone(), vec![acc_expr, item_expr])?;
    }

    Ok(accumulator)
}

pub fn eval_sort(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("sort requires exactly 1 argument (list)".to_string());
    }

    let list_val = evaluator.eval_expression(args[0].clone())?;
    let mut list = match list_val {
        Value::List(items) => items,
        _ => return Err("sort argument must be a list".to_string()),
    };

    // Sort based on value type
    list.sort_by(|a, b| {
        match (a, b) {
            (Value::Number(a), Value::Number(b)) => a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal),
            (Value::String(a), Value::String(b)) => a.cmp(b),
            (Value::Boolean(a), Value::Boolean(b)) => a.cmp(b),
            _ => std::cmp::Ordering::Equal,
        }
    });

    Ok(Value::List(list))
}

pub fn eval_reverse(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("reverse requires exactly 1 argument (list)".to_string());
    }

    let list_val = evaluator.eval_expression(args[0].clone())?;
    let mut list = match list_val {
        Value::List(items) => items,
        _ => return Err("reverse argument must be a list".to_string()),
    };

    list.reverse();
    Ok(Value::List(list))
}