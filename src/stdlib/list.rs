use crate::ast::Expression;
use crate::evaluator::Evaluator;
use crate::value::Value;
use super::utils::*;

/// Helper function to evaluate a predicate function (either user action or stdlib function)
fn eval_predicate(function_name: &str, item: &Value, evaluator: &mut Evaluator) -> Result<Value, String> {
    eval_function_on_item(function_name, item, evaluator)
}

/// Helper function to evaluate any function (transformer or predicate) on an item
fn eval_function_on_item(function_name: &str, item: &Value, evaluator: &mut Evaluator) -> Result<Value, String> {
    // First try user actions
    if let Some(action) = evaluator.actions.get(function_name) {
        let action = action.clone();
        if action.params.len() != 1 {
            return Err(format!("Function {} must take exactly one parameter", function_name));
        }

        let arg_expr = Expression::Variable(action.params[0].clone());
        evaluator.set_variable(action.params[0].clone(), item.clone());
        return evaluator.eval_action(action, vec![arg_expr]);
    }

    // Then try stdlib functions
    if let Some(stdlib_func) = crate::stdlib::get_stdlib_function(function_name) {
        // For stdlib functions, we pass the item as a direct argument
        let item_expr = match item {
            Value::Number(n) => Expression::Number(*n, false),
            Value::String(s) => Expression::String(s.clone()),
            Value::Boolean(b) => Expression::Boolean(*b),
            Value::List(_) => return Err(format!("Stdlib function {} cannot operate on lists directly", function_name)),
            Value::Object(_) => return Err(format!("Stdlib function {} cannot operate on objects directly", function_name)),
            Value::Null => return Err(format!("Stdlib function {} cannot operate on null values", function_name)),
        };
        return stdlib_func(vec![item_expr], evaluator);
    }

    Err(format!("Unknown function: {}", function_name))
}

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
        _ => return Err("map second argument must be a function name".to_string()),
    };

    // Check if it's a stdlib function first
    if let Some(stdlib_func) = crate::stdlib::get_stdlib_function(&function_name) {
        let mut result = Vec::new();
        for item in list {
            // Create expression for the item
            let arg_expr = match &item {
                Value::Number(n) => Expression::Number(*n, false),
                Value::String(s) => Expression::String(s.clone()),
                Value::Boolean(b) => Expression::Boolean(*b),
                _ => return Err("map can only work with numbers, strings, and booleans for now".to_string()),
            };

            let mapped_value = stdlib_func(vec![arg_expr], evaluator)?;
            result.push(mapped_value);
        }
        return Ok(Value::List(result));
    }

    // Check if the action exists (user-defined function)
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
        _ => return Err("filter second argument must be a function name (like 'is_even' or '.is-even' for stdlib)".to_string()),
    };

    // Check if it's a stdlib function first
    if let Some(stdlib_func) = crate::stdlib::get_stdlib_function(&function_name) {
            let mut result = Vec::new();
            for item in list {
                // Create expression for the item
                let arg_expr = match &item {
                    Value::Number(n) => Expression::Number(*n, false),
                    Value::String(s) => Expression::String(s.clone()),
                    Value::Boolean(b) => Expression::Boolean(*b),
                    _ => return Err("filter can only work with numbers, strings, and booleans for now".to_string()),
                };

                let predicate_result = stdlib_func(vec![arg_expr], evaluator)?;
                if let Value::Boolean(true) = predicate_result {
                    result.push(item);
                }
            }
            return Ok(Value::List(result));
    }

    // Check if the action exists (user-defined function)
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
        _ => return Err("reduce second argument must be a function name (like 'add' or '.add' for stdlib)".to_string()),
    };

    let mut accumulator = evaluator.eval_expression(args[2].clone())?;

    // Check if it's a stdlib function first
    if let Some(stdlib_func) = crate::stdlib::get_stdlib_function(&function_name) {
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

                accumulator = stdlib_func(vec![acc_expr, item_expr], evaluator)?;
            }
            return Ok(accumulator);
    }

    // Check if the action exists (user-defined function)
    if !evaluator.actions.contains_key(&function_name) {
        return Err(format!("Action '{}' not found", function_name));
    }

    let action = evaluator.actions.get(&function_name).unwrap().clone();

    // Check that the action takes exactly two params
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
    let mut list = extract_list_arg(&args, evaluator, "sort")?;

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

    let function_name = match &args[1] {
        Expression::Variable(name) => name.clone(),
        Expression::FunctionCall { name, args: fn_args } if fn_args.is_empty() => name.clone(),
        _ => return Err("find requires a function name as the second argument".to_string()),
    };

    for item in list {
        let predicate_result = eval_predicate(&function_name, &item, evaluator)?;

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
        Expression::FunctionCall { name, args: fn_args } if fn_args.is_empty() => name.clone(),
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
        Expression::FunctionCall { name, args: fn_args } if fn_args.is_empty() => name.clone(),
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

    let function_name = match &args[1] {
        Expression::Variable(name) => name.clone(),
        Expression::FunctionCall { name, args: fn_args } if fn_args.is_empty() => name.clone(),
        _ => return Err("every requires a function name as the second argument".to_string()),
    };

    for item in list {
        let predicate_result = eval_predicate(&function_name, &item, evaluator)?;

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

    let function_name = match &args[1] {
        Expression::Variable(name) => name.clone(),
        Expression::FunctionCall { name, args: fn_args } if fn_args.is_empty() => name.clone(),
        _ => return Err("some requires a function name as the second argument".to_string()),
    };

    for item in list {
        let predicate_result = eval_predicate(&function_name, &item, evaluator)?;

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
        Expression::FunctionCall { name, args: fn_args } if fn_args.is_empty() => name.clone(),
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
        Expression::FunctionCall { name, args: fn_args } if fn_args.is_empty() => name.clone(),
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
        Expression::FunctionCall { name, args: fn_args } if fn_args.is_empty() => name.clone(),
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
        Expression::FunctionCall { name, args: fn_args } if fn_args.is_empty() => name.clone(),
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
    let function_name = match &args[1] {
        Expression::Variable(name) => name.clone(),
        Expression::FunctionCall { name, args: fn_args } if fn_args.is_empty() => name.clone(),
        _ => return Err("partition requires a function name as the second argument".to_string()),
    };

    let mut matched = Vec::new();
    let mut unmatched = Vec::new();

    for item in list {
        let predicate_result = eval_predicate(&function_name, &item, evaluator)?;

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
    let function_name = match &args[1] {
        Expression::Variable(name) => name.clone(),
        Expression::FunctionCall { name, args: fn_args } if fn_args.is_empty() => name.clone(),
        _ => return Err("group-by requires a function name as the second argument".to_string()),
    };

    let mut groups = std::collections::HashMap::new();

    for item in list {
        let key_result = eval_function_on_item(&function_name, &item, evaluator)?;
        let key_string = match key_result {
            Value::Number(n) => {
                if n.fract() == 0.0 {
                    (n as i64).to_string()
                } else {
                    n.to_string()
                }
            },
            Value::String(s) => s,
            Value::Boolean(b) => b.to_string(),
            _ => return Err("group-by function must return a number, string, or boolean".to_string()),
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
    let function_name = match &args[1] {
        Expression::Variable(name) => name.clone(),
        Expression::FunctionCall { name, args: fn_args } if fn_args.is_empty() => name.clone(),
        _ => return Err("sort-by requires a function name as the second argument".to_string()),
    };

    // Create pairs of (sort_key, original_item)
    let mut keyed_items = Vec::new();
    for item in list {
        let sort_key = eval_function_on_item(&function_name, &item, evaluator)?;
        keyed_items.push((sort_key, item));
    }

    // Sort by the keys
    keyed_items.sort_by(|(a, _), (b, _)| {
        match (a, b) {
            (Value::Number(n1), Value::Number(n2)) => n1.partial_cmp(n2).unwrap_or(std::cmp::Ordering::Equal),
            (Value::String(s1), Value::String(s2)) => s1.cmp(s2),
            (Value::Boolean(b1), Value::Boolean(b2)) => b1.cmp(b2),
            _ => std::cmp::Ordering::Equal,
        }
    });

    // Extract the sorted items
    let sorted_items: Vec<Value> = keyed_items.into_iter().map(|(_, item)| item).collect();
    Ok(Value::List(sorted_items))
}