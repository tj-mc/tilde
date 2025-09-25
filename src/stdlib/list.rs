use super::utils::*;
use crate::ast::Expression;
use crate::evaluator::Evaluator;
use crate::value::Value;

/// Fast iterative Fibonacci implementation
fn fibonacci_iterative(n: u32) -> f64 {
    if n <= 1 {
        return n as f64;
    }

    let mut a = 0.0;
    let mut b = 1.0;

    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }

    b
}

// Helper macros for native function optimization
macro_rules! native_map {
    ($list:expr, $transform:expr) => {
        Ok(Value::List($list.into_iter().map($transform).collect()))
    };
}

macro_rules! native_filter {
    ($list:expr, $predicate:expr) => {
        Ok(Value::List($list.into_iter().filter($predicate).collect()))
    };
}

macro_rules! native_reduce {
    ($list:expr, $acc:expr, $reducer:expr) => {
        $list.iter().try_fold($acc, $reducer)
    };
}

/// Generic helper that works with both named functions and anonymous functions
fn eval_function_expression_on_item(
    func_expr: &Expression,
    item: &Value,
    evaluator: &mut Evaluator,
) -> Result<Value, String> {
    match func_expr {
        Expression::AnonymousFunction { params, body } => {
            if params.len() != 1 {
                return Err("Function must take exactly 1 parameter".to_string());
            }
            evaluator.push_scope();
            evaluator.set_local_variable(&params[0], item.clone());
            let result = evaluator.eval_expression(*body.clone())?;
            evaluator.pop_scope();
            Ok(result)
        }
        Expression::Variable(name) => eval_function_on_item(name, item, evaluator),
        Expression::FunctionCall { name, args } if args.is_empty() => eval_function_on_item(name, item, evaluator),
        _ => Err("Expected function name or anonymous function".to_string()),
    }
}

/// Helper function to evaluate a predicate function (either user function or stdlib function)
fn eval_predicate(
    function_name: &str,
    item: &Value,
    evaluator: &mut Evaluator,
) -> Result<Value, String> {
    eval_function_on_item(function_name, item, evaluator)
}

/// Helper function to evaluate any function (transformer or predicate) on an item
fn eval_function_on_item(
    function_name: &str,
    item: &Value,
    evaluator: &mut Evaluator,
) -> Result<Value, String> {
    // Check for block syntax first (e.g., core:is-even)
    if function_name.contains(':') {
        let parts: Vec<&str> = function_name.split(':').collect();
        if parts.len() == 2 {
            let block_name = parts[0];
            let func_name = parts[1];

            // Handle different blocks
            match block_name {
                "core" => {
                    // Force use of stdlib function, bypassing user definitions
                    if let Some(stdlib_func) = crate::stdlib::get_stdlib_function(func_name) {
                        // Convert the value to an expression for the stdlib function
                        let item_expr = match item {
                            Value::Number(n) => Expression::Number(*n, false),
                            Value::String(s) => Expression::String(s.clone()),
                            Value::Boolean(b) => Expression::Boolean(*b),
                            // For complex types, stdlib functions should handle them appropriately
                            _ => {
                                return Err(format!(
                                    "Core function {} cannot be used in higher-order functions with complex values",
                                    func_name
                                ));
                            }
                        };
                        return stdlib_func(vec![item_expr], evaluator);
                    } else {
                        return Err(format!("Unknown core function: {}", func_name));
                    }
                }
                _ => return Err(format!("Unknown block: {}", block_name)),
            }
        } else {
            return Err(format!("Invalid block syntax: {}", function_name));
        }
    }

    // First try user functions
    if let Some(function) = evaluator.functions.get(function_name) {
        let function = function.clone();
        if function.params.len() != 1 {
            return Err(format!(
                "Function {} must take exactly one parameter",
                function_name
            ));
        }

        let arg_expr = Expression::Variable(function.params[0].clone());
        evaluator.set_variable(function.params[0].clone(), item.clone());
        return evaluator.eval_function(function, vec![arg_expr]);
    }

    // Then try stdlib functions
    if let Some(stdlib_func) = crate::stdlib::get_stdlib_function(function_name) {
        // For stdlib functions, we pass the item as a direct argument
        let item_expr = match item {
            Value::Number(n) => Expression::Number(*n, false),
            Value::String(s) => Expression::String(s.clone()),
            Value::Boolean(b) => Expression::Boolean(*b),
            // For complex types, only allow if the function can handle them
            _ => {
                return Err(format!(
                    "Stdlib function {} cannot be used in higher-order functions with complex values",
                    function_name
                ));
            }
        };
        return stdlib_func(vec![item_expr], evaluator);
    }

    Err(format!("Unknown function: {}", function_name))
}

/// Transforms each element in a list using the provided function.
///
/// # Arguments
/// * `list` - The list to transform
/// * `function_name` - Name of the function that takes one parameter and returns a value
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

    // Check if second argument is an anonymous function
    if let Expression::AnonymousFunction { params, body } = &args[1] {
        // Handle anonymous function
        if params.len() != 1 {
            return Err("map anonymous function must take exactly 1 parameter".to_string());
        }

        let mut result = Vec::new();
        for item in list {
            // Push new scope with parameter binding
            evaluator.push_scope();
            evaluator.set_local_variable(&params[0], item);
            let mapped = evaluator.eval_expression(*body.clone())?;
            evaluator.pop_scope();
            result.push(mapped);
        }
        return Ok(Value::List(result));
    }

    // Get function name for the transformation
    let function_name = match &args[1] {
        Expression::Variable(name) => name.clone(),
        Expression::FunctionCall { name, args } if args.is_empty() => name.clone(),
        _ => {
            return Err(
                "map second argument must be a function name or anonymous function".to_string(),
            );
        }
    };

    // Native fast paths for common stdlib functions
    match function_name.as_str() {
        "double" => native_map!(list, |v| match v {
            Value::Number(n) => Value::Number(n * 2.0),
            _ => v,
        }),
        "square" => native_map!(list, |v| match v {
            Value::Number(n) => Value::Number(n * n),
            _ => v,
        }),
        "triple" => native_map!(list, |v| match v {
            Value::Number(n) => Value::Number(n * 3.0),
            _ => v,
        }),
        "quadruple" => native_map!(list, |v| match v {
            Value::Number(n) => Value::Number(n * 4.0),
            _ => v,
        }),
        "half" => native_map!(list, |v| match v {
            Value::Number(n) => Value::Number(n / 2.0),
            _ => v,
        }),
        "increment" => native_map!(list, |v| match v {
            Value::Number(n) => Value::Number(n + 1.0),
            _ => v,
        }),
        "decrement" => native_map!(list, |v| match v {
            Value::Number(n) => Value::Number(n - 1.0),
            _ => v,
        }),
        "uppercase" => native_map!(list, |v| match v {
            Value::String(s) => Value::String(s.to_uppercase()),
            _ => v,
        }),
        "lowercase" => native_map!(list, |v| match v {
            Value::String(s) => Value::String(s.to_lowercase()),
            _ => v,
        }),
        "absolute" => native_map!(list, |v| match v {
            Value::Number(n) => Value::Number(n.abs()),
            _ => v,
        }),
        "square-root" => native_map!(list, |v| match v {
            Value::Number(n) => {
                if n >= 0.0 {
                    Value::Number(n.sqrt())
                } else {
                    v // Keep original for negative numbers
                }
            }
            _ => v,
        }),
        "trim" => native_map!(list, |v| match v {
            Value::String(s) => Value::String(s.trim().to_string()),
            _ => v,
        }),
        "fibonacci" => native_map!(list, |v| match v {
            Value::Number(n) => {
                if n >= 0.0 && n <= 1476.0 {
                    // Max safe fibonacci in f64
                    Value::Number(fibonacci_iterative(n as u32))
                } else {
                    v // Keep original for out-of-range values
                }
            }
            _ => v,
        }),
        _ => {
            // Slow path: Use the helper function to evaluate the function on each item
            let mut result = Vec::new();
            for item in list {
                let mapped_value = eval_function_on_item(&function_name, &item, evaluator)?;
                result.push(mapped_value);
            }
            Ok(Value::List(result))
        }
    }
}

/// Filters elements in a list using the provided predicate function.
///
/// # Arguments
/// * `list` - The list to filter
/// * `function_name` - Name of the function that takes one parameter and returns a boolean
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

    // Check if second argument is an anonymous function
    if let Expression::AnonymousFunction { params, body } = &args[1] {
        // Handle anonymous function
        if params.len() != 1 {
            return Err("filter anonymous function must take exactly 1 parameter".to_string());
        }

        let mut result = Vec::new();
        for item in list {
            // Push new scope with parameter binding
            evaluator.push_scope();
            evaluator.set_local_variable(&params[0], item.clone());
            let predicate_result = evaluator.eval_expression(*body.clone())?;
            evaluator.pop_scope();

            // Check if the predicate is truthy
            if evaluator.is_truthy(&predicate_result) {
                result.push(item);
            }
        }
        return Ok(Value::List(result));
    }

    let function_name = match &args[1] {
        Expression::Variable(name) => name.clone(),
        Expression::FunctionCall { name, args } if args.is_empty() => name.clone(),
        _ => {
            return Err(
                "filter second argument must be a function name or anonymous function".to_string(),
            );
        }
    };

    // Native fast paths for common stdlib predicates
    match function_name.as_str() {
        "is-even" => native_filter!(list, |v| match v {
            Value::Number(n) => (*n as i64) % 2 == 0,
            _ => false,
        }),
        "is-odd" => native_filter!(list, |v| match v {
            Value::Number(n) => (*n as i64) % 2 != 0,
            _ => false,
        }),
        "is-positive" => native_filter!(list, |v| match v {
            Value::Number(n) => *n > 0.0,
            _ => false,
        }),
        "is-negative" => native_filter!(list, |v| match v {
            Value::Number(n) => *n < 0.0,
            _ => false,
        }),
        "is-zero" => native_filter!(list, |v| match v {
            Value::Number(n) => *n == 0.0,
            _ => false,
        }),
        _ => {
            // Slow path: Use the helper function to evaluate the predicate on each item
            let mut result = Vec::new();
            for item in list {
                let predicate_result = eval_predicate(&function_name, &item, evaluator)?;
                if let Value::Boolean(true) = predicate_result {
                    result.push(item);
                }
            }
            Ok(Value::List(result))
        }
    }
}

pub fn eval_reduce(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 3 {
        return Err(
            "reduce requires exactly 3 arguments (list, function_name, initial_value)".to_string(),
        );
    }

    let list_val = evaluator.eval_expression(args[0].clone())?;
    let list = match list_val {
        Value::List(items) => items,
        _ => return Err("reduce first argument must be a list".to_string()),
    };

    // Check if second argument is an anonymous function
    if let Expression::AnonymousFunction { params, body } = &args[1] {
        // Handle anonymous function
        if params.len() != 2 {
            return Err("reduce anonymous function must take exactly 2 parameters".to_string());
        }

        let mut accumulator = evaluator.eval_expression(args[2].clone())?;

        for item in list {
            // Push new scope with parameters
            evaluator.push_scope();
            evaluator.set_local_variable(&params[0], accumulator);
            evaluator.set_local_variable(&params[1], item);
            accumulator = evaluator.eval_expression(*body.clone())?;
            evaluator.pop_scope();
        }

        return Ok(accumulator);
    }

    let function_name = match &args[1] {
        Expression::Variable(name) => name.clone(),
        Expression::FunctionCall { name, args } if args.is_empty() => name.clone(),
        _ => {
            return Err(
                "reduce second argument must be a function name or anonymous function".to_string(),
            );
        }
    };

    let accumulator = evaluator.eval_expression(args[2].clone())?;

    // Native fast paths for common reduce functions
    match function_name.as_str() {
        "add" => {
            let result = native_reduce!(list, accumulator, |acc, item| match (acc, item) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
                _ => Err("add requires numbers".to_string()),
            })?;
            return Ok(result);
        }
        "multiply" => {
            let result = native_reduce!(list, accumulator, |acc, item| match (acc, item) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),
                _ => Err("multiply requires numbers".to_string()),
            })?;
            return Ok(result);
        }
        "max" => {
            let result = native_reduce!(list, accumulator, |acc, item| match (acc, item) {
                (Value::Number(a), Value::Number(b)) =>
                    Ok(Value::Number(if *b > a { *b } else { a })),
                _ => Err("max requires numbers".to_string()),
            })?;
            return Ok(result);
        }
        "min" => {
            let result = native_reduce!(list, accumulator, |acc, item| match (acc, item) {
                (Value::Number(a), Value::Number(b)) =>
                    Ok(Value::Number(if *b < a { *b } else { a })),
                _ => Err("min requires numbers".to_string()),
            })?;
            return Ok(result);
        }
        _ => {}
    }

    // Check if it's a stdlib function first
    if let Some(stdlib_func) = crate::stdlib::get_stdlib_function(&function_name) {
        let mut accumulator = accumulator;
        for item in list {
            // Create arguments for the accumulator and current item
            let acc_expr = match &accumulator {
                Value::Number(n) => Expression::Number(*n, false),
                Value::String(s) => Expression::String(s.clone()),
                Value::Boolean(b) => Expression::Boolean(*b),
                _ => {
                    return Err(
                        "reduce can only work with numbers, strings, and booleans for now"
                            .to_string(),
                    );
                }
            };

            let item_expr = match &item {
                Value::Number(n) => Expression::Number(*n, false),
                Value::String(s) => Expression::String(s.clone()),
                Value::Boolean(b) => Expression::Boolean(*b),
                _ => {
                    return Err(
                        "reduce can only work with numbers, strings, and booleans for now"
                            .to_string(),
                    );
                }
            };

            accumulator = stdlib_func(vec![acc_expr, item_expr], evaluator)?;
        }
        return Ok(accumulator);
    }

    // Check if the function exists (user-defined function)
    if !evaluator.functions.contains_key(&function_name) {
        return Err(format!("Function '{}' not found", function_name));
    }

    let function = evaluator.functions.get(&function_name).unwrap().clone();

    // Check that the function takes exactly two params
    if function.params.len() != 2 {
        return Err(format!(
            "Function '{}' must take exactly two parameters for reduce",
            function_name
        ));
    }

    let mut accumulator = evaluator.eval_expression(args[2].clone())?;

    for item in list {
        // Create arguments for the accumulator and current item
        let acc_expr = match &accumulator {
            Value::Number(n) => Expression::Number(*n, false),
            Value::String(s) => Expression::String(s.clone()),
            Value::Boolean(b) => Expression::Boolean(*b),
            _ => {
                return Err(
                    "reduce can only work with numbers, strings, and booleans for now".to_string(),
                );
            }
        };

        let item_expr = match &item {
            Value::Number(n) => Expression::Number(*n, false),
            Value::String(s) => Expression::String(s.clone()),
            Value::Boolean(b) => Expression::Boolean(*b),
            _ => {
                return Err(
                    "reduce can only work with numbers, strings, and booleans for now".to_string(),
                );
            }
        };

        accumulator = evaluator.eval_function(function.clone(), vec![acc_expr, item_expr])?;
    }

    Ok(accumulator)
}

pub fn eval_sort(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("sort requires exactly 1 argument (list)".to_string());
    }
    let mut list = extract_list_arg(&args, evaluator, "sort")?;

    // Sort based on value type
    list.sort_by(|a, b| match (a, b) {
        (Value::Number(a), Value::Number(b)) => {
            a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)
        }
        (Value::String(a), Value::String(b)) => a.cmp(b),
        (Value::Boolean(a), Value::Boolean(b)) => a.cmp(b),
        _ => std::cmp::Ordering::Equal,
    });

    Ok(Value::List(list))
}

pub fn eval_reverse(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("reverse requires exactly 1 argument (list)".to_string());
    }
    let mut list = extract_list_arg(&args, evaluator, "reverse")?;
    list.reverse();
    Ok(Value::List(list))
}

/// Find the first item in a list that matches a predicate function
/// Returns the first matching item, or null if no match found
pub fn eval_find(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("find requires exactly 2 arguments (list, function_name)".to_string());
    }

    let list_val = evaluator.eval_expression(args[0].clone())?;
    let list = match list_val {
        Value::List(l) => l,
        _ => return Err("find can only be used on lists".to_string()),
    };

    for item in list {
        let predicate_result = eval_function_expression_on_item(&args[1], &item, evaluator)?;

        if predicate_result.is_truthy() {
            return Ok(item);
        }
    }

    Ok(Value::Null)
}

/// Find the index of the first item in a list that matches a predicate function
/// Returns the index as a number, or null if no match found
pub fn eval_find_index(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("find-index requires exactly 2 arguments (list, function_name)".to_string());
    }

    let list_val = evaluator.eval_expression(args[0].clone())?;
    let list = match list_val {
        Value::List(l) => l,
        _ => return Err("find-index can only be used on lists".to_string()),
    };

    let function_name = match &args[1] {
        Expression::Variable(name) => name.clone(),
        Expression::FunctionCall {
            name,
            args: fn_args,
        } if fn_args.is_empty() => name.clone(),
        _ => return Err("find-index requires a function name as the second argument".to_string()),
    };

    for (index, item) in list.iter().enumerate() {
        let predicate_result = eval_predicate(&function_name, item, evaluator)?;

        if predicate_result.is_truthy() {
            return Ok(Value::Number(index as f64));
        }
    }

    Ok(Value::Null)
}

/// Find the last item in a list that matches a predicate function
/// Returns the last matching item, or null if no match found
pub fn eval_find_last(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("find-last requires exactly 2 arguments (list, function_name)".to_string());
    }

    let list_val = evaluator.eval_expression(args[0].clone())?;
    let list = match list_val {
        Value::List(l) => l,
        _ => return Err("find-last can only be used on lists".to_string()),
    };

    let function_name = match &args[1] {
        Expression::Variable(name) => name.clone(),
        Expression::FunctionCall {
            name,
            args: fn_args,
        } if fn_args.is_empty() => name.clone(),
        _ => return Err("find-last requires a function name as the second argument".to_string()),
    };

    let mut last_match = Value::Null;

    for item in list {
        let predicate_result = eval_predicate(&function_name, &item, evaluator)?;

        if predicate_result.is_truthy() {
            last_match = item;
        }
    }

    Ok(last_match)
}

/// Test if all items in a list match a predicate function
/// Returns true if all items match, false otherwise
pub fn eval_every(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("every requires exactly 2 arguments (list, function_name)".to_string());
    }

    let list_val = evaluator.eval_expression(args[0].clone())?;
    let list = match list_val {
        Value::List(l) => l,
        _ => return Err("every can only be used on lists".to_string()),
    };

    for item in list {
        let predicate_result = eval_function_expression_on_item(&args[1], &item, evaluator)?;

        if !predicate_result.is_truthy() {
            return Ok(Value::Boolean(false));
        }
    }

    Ok(Value::Boolean(true))
}

/// Test if any item in a list matches a predicate function
/// Returns true if at least one item matches, false otherwise
pub fn eval_some(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("some requires exactly 2 arguments (list, function_name)".to_string());
    }

    let list_val = evaluator.eval_expression(args[0].clone())?;
    let list = match list_val {
        Value::List(l) => l,
        _ => return Err("some can only be used on lists".to_string()),
    };

    for item in list {
        let predicate_result = eval_function_expression_on_item(&args[1], &item, evaluator)?;

        if predicate_result.is_truthy() {
            return Ok(Value::Boolean(true));
        }
    }

    Ok(Value::Boolean(false))
}

/// Remove items from a list that match a predicate function
/// Returns a new list with matching items removed
pub fn eval_remove_if(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("remove-if requires exactly 2 arguments (list, function_name)".to_string());
    }

    let list_val = evaluator.eval_expression(args[0].clone())?;
    let list = match list_val {
        Value::List(l) => l,
        _ => return Err("remove-if can only be used on lists".to_string()),
    };

    let function_name = match &args[1] {
        Expression::Variable(name) => name.clone(),
        Expression::FunctionCall {
            name,
            args: fn_args,
        } if fn_args.is_empty() => name.clone(),
        _ => return Err("remove-if requires a function name as the second argument".to_string()),
    };

    let mut result = Vec::new();

    for item in list {
        let predicate_result = eval_predicate(&function_name, &item, evaluator)?;

        if !predicate_result.is_truthy() {
            result.push(item);
        }
    }

    Ok(Value::List(result))
}

/// Count items in a list that match a predicate function
/// Returns the count as a number
pub fn eval_count_if(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("count-if requires exactly 2 arguments (list, function_name)".to_string());
    }

    let list_val = evaluator.eval_expression(args[0].clone())?;
    let list = match list_val {
        Value::List(l) => l,
        _ => return Err("count-if can only be used on lists".to_string()),
    };

    let function_name = match &args[1] {
        Expression::Variable(name) => name.clone(),
        Expression::FunctionCall {
            name,
            args: fn_args,
        } if fn_args.is_empty() => name.clone(),
        _ => return Err("count-if requires a function name as the second argument".to_string()),
    };

    let mut count = 0;

    for item in list {
        let predicate_result = eval_predicate(&function_name, &item, evaluator)?;

        if predicate_result.is_truthy() {
            count += 1;
        }
    }

    Ok(Value::Number(count as f64))
}

/// Take items from the beginning of a list while a predicate is true
/// Returns a new list with items taken while predicate returns true
pub fn eval_take_while(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("take-while requires exactly 2 arguments (list, function_name)".to_string());
    }

    let list_val = evaluator.eval_expression(args[0].clone())?;
    let list = match list_val {
        Value::List(l) => l,
        _ => return Err("take-while can only be used on lists".to_string()),
    };

    let function_name = match &args[1] {
        Expression::Variable(name) => name.clone(),
        Expression::FunctionCall {
            name,
            args: fn_args,
        } if fn_args.is_empty() => name.clone(),
        _ => return Err("take-while requires a function name as the second argument".to_string()),
    };

    let mut result = Vec::new();

    for item in list {
        let predicate_result = eval_predicate(&function_name, &item, evaluator)?;

        if predicate_result.is_truthy() {
            result.push(item);
        } else {
            break;
        }
    }

    Ok(Value::List(result))
}

/// Drop items from the beginning of a list while a predicate is true
/// Returns a new list with remaining items after dropping while predicate returns true
pub fn eval_drop_while(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("drop-while requires exactly 2 arguments (list, function_name)".to_string());
    }

    let list_val = evaluator.eval_expression(args[0].clone())?;
    let list = match list_val {
        Value::List(l) => l,
        _ => return Err("drop-while can only be used on lists".to_string()),
    };

    let function_name = match &args[1] {
        Expression::Variable(name) => name.clone(),
        Expression::FunctionCall {
            name,
            args: fn_args,
        } if fn_args.is_empty() => name.clone(),
        _ => return Err("drop-while requires a function name as the second argument".to_string()),
    };

    let mut drop_index = 0;

    for (index, item) in list.iter().enumerate() {
        let predicate_result = eval_predicate(&function_name, item, evaluator)?;

        if predicate_result.is_truthy() {
            drop_index = index + 1;
        } else {
            break;
        }
    }

    let result = list.into_iter().skip(drop_index).collect();
    Ok(Value::List(result))
}

/// Partitions a list into two lists based on a predicate function
/// Returns an object with `matched` and `unmatched` properties
pub fn eval_partition(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("partition requires exactly 2 arguments (list, function_name)".to_string());
    }
    let list_val = evaluator.eval_expression(args[0].clone())?;
    let list = match list_val {
        Value::List(l) => l,
        _ => return Err("partition can only be used on lists".to_string()),
    };

    let mut matched = Vec::new();
    let mut unmatched = Vec::new();

    for item in list {
        let predicate_result = eval_function_expression_on_item(&args[1], &item, evaluator)?;

        if predicate_result.is_truthy() {
            matched.push(item);
        } else {
            unmatched.push(item);
        }
    }

    let mut result_map = std::collections::HashMap::new();
    result_map.insert("matched".to_string(), Value::List(matched));
    result_map.insert("unmatched".to_string(), Value::List(unmatched));

    Ok(Value::Object(result_map))
}

/// Groups list elements by the result of applying a function to each element
/// Returns an object where keys are the function results and values are lists of matching elements
pub fn eval_group_by(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("group-by requires exactly 2 arguments (list, function_name)".to_string());
    }
    let list_val = evaluator.eval_expression(args[0].clone())?;
    let list = match list_val {
        Value::List(l) => l,
        _ => return Err("group-by can only be used on lists".to_string()),
    };

    let mut groups = std::collections::HashMap::new();

    for item in list {
        let key_result = eval_function_expression_on_item(&args[1], &item, evaluator)?;
        let key_string = match key_result {
            Value::Number(n) => {
                if n.fract() == 0.0 {
                    (n as i64).to_string()
                } else {
                    n.to_string()
                }
            }
            Value::String(s) => s,
            Value::Boolean(b) => b.to_string(),
            _ => {
                return Err("group-by function must return a number, string, or boolean".to_string());
            }
        };

        groups.entry(key_string).or_insert_with(Vec::new).push(item);
    }

    let result_map: std::collections::HashMap<String, Value> = groups
        .into_iter()
        .map(|(k, v)| (k, Value::List(v)))
        .collect();

    Ok(Value::Object(result_map))
}

/// Sorts a list by applying a function to each element and sorting by the results
pub fn eval_sort_by(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("sort-by requires exactly 2 arguments (list, function_name)".to_string());
    }
    let list_val = evaluator.eval_expression(args[0].clone())?;
    let list = match list_val {
        Value::List(l) => l,
        _ => return Err("sort-by can only be used on lists".to_string()),
    };

    // Create pairs of (sort_key, original_item)
    let mut keyed_items = Vec::new();
    for item in list {
        let sort_key = eval_function_expression_on_item(&args[1], &item, evaluator)?;
        keyed_items.push((sort_key, item));
    }

    // Sort by the keys
    keyed_items.sort_by(|(a, _), (b, _)| match (a, b) {
        (Value::Number(n1), Value::Number(n2)) => {
            n1.partial_cmp(n2).unwrap_or(std::cmp::Ordering::Equal)
        }
        (Value::String(s1), Value::String(s2)) => s1.cmp(s2),
        (Value::Boolean(b1), Value::Boolean(b2)) => b1.cmp(b2),
        _ => std::cmp::Ordering::Equal,
    });

    // Extract the sorted items
    let sorted_items: Vec<Value> = keyed_items.into_iter().map(|(_, item)| item).collect();
    Ok(Value::List(sorted_items))
}

/// Creates a list of consecutive numbers from 1 to the specified length
/// Usage: list length
/// Returns: [1, 2, 3, ..., length]
/// Fast O(n) implementation with pre-allocated vector
pub fn eval_list(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("list requires exactly 1 argument (length)".into());
    }

    let length_val = match &args[0] {
        Expression::Number(n, _) => Value::Number(*n), // Fast path for literals
        expr => evaluator.eval_expression(expr.clone())?, // Fallback for complex expressions
    };
    let length = match length_val {
        Value::Number(n) => {
            if n < 0.0 {
                return Err("list length cannot be negative".into());
            }
            if n > 1_000_000.0 {
                return Err("list length cannot exceed 1,000,000 for performance".into());
            }
            n as usize
        }
        _ => return Err("list length must be a number".into()),
    };

    // Fast pre-allocated vector - O(n) time, O(n) space
    let mut result = Vec::with_capacity(length);
    for i in 1..=length {
        result.push(Value::Number(i as f64));
    }

    Ok(Value::List(result))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Expression;
    use crate::evaluator::Evaluator;
    use crate::value::Value;

    #[test]
    fn test_list_basic() {
        let mut evaluator = Evaluator::new();
        let args = vec![Expression::Number(5.0, false)];

        let result = eval_list(args, &mut evaluator).unwrap();

        match result {
            Value::List(list) => {
                assert_eq!(list.len(), 5);
                assert_eq!(list[0], Value::Number(1.0));
                assert_eq!(list[1], Value::Number(2.0));
                assert_eq!(list[2], Value::Number(3.0));
                assert_eq!(list[3], Value::Number(4.0));
                assert_eq!(list[4], Value::Number(5.0));
            }
            _ => panic!("Expected list"),
        }
    }

    #[test]
    fn test_list_zero() {
        let mut evaluator = Evaluator::new();
        let args = vec![Expression::Number(0.0, false)];

        let result = eval_list(args, &mut evaluator).unwrap();

        match result {
            Value::List(list) => {
                assert_eq!(list.len(), 0);
            }
            _ => panic!("Expected empty list"),
        }
    }

    #[test]
    fn test_list_large() {
        let mut evaluator = Evaluator::new();
        let args = vec![Expression::Number(1000.0, false)];

        let result = eval_list(args, &mut evaluator).unwrap();

        match result {
            Value::List(list) => {
                assert_eq!(list.len(), 1000);
                assert_eq!(list[0], Value::Number(1.0));
                assert_eq!(list[999], Value::Number(1000.0));
            }
            _ => panic!("Expected list"),
        }
    }

    #[test]
    fn test_list_negative_error() {
        let mut evaluator = Evaluator::new();
        let args = vec![Expression::Number(-5.0, false)];

        let result = eval_list(args, &mut evaluator);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("cannot be negative"));
    }

    #[test]
    fn test_list_too_large_error() {
        let mut evaluator = Evaluator::new();
        let args = vec![Expression::Number(2_000_000.0, false)];

        let result = eval_list(args, &mut evaluator);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("cannot exceed 1,000,000"));
    }

    #[test]
    fn test_list_wrong_type_error() {
        let mut evaluator = Evaluator::new();
        let args = vec![Expression::String("hello".to_string())];

        let result = eval_list(args, &mut evaluator);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must be a number"));
    }

    #[test]
    fn test_list_wrong_args_error() {
        let mut evaluator = Evaluator::new();
        let args = vec![
            Expression::Number(5.0, false),
            Expression::Number(10.0, false),
        ];

        let result = eval_list(args, &mut evaluator);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exactly 1 argument"));
    }
}
