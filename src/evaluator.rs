use crate::ast::*;
use crate::value::Value;
use serde_json;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum ControlFlow {
    Continue,
    BreakLoop,
    Give(Value),
}

type EvalResult = Result<(Value, ControlFlow), String>;

#[derive(Debug, Clone)]
pub struct Function {
    pub params: Vec<String>,
    pub body: Vec<Statement>,
}

#[derive(Debug)]
pub struct Evaluator {
    pub variables: HashMap<String, Value>,
    pub functions: HashMap<String, Function>,
    scope_stack: Vec<HashMap<String, Value>>,
    max_call_depth: usize,
    pub output_buffer: Vec<String>,
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            variables: HashMap::new(),
            functions: HashMap::new(),
            scope_stack: Vec::new(),
            max_call_depth: 1000,
            output_buffer: Vec::new(),
        }
    }

    fn is_truthy(&self, value: &Value) -> bool {
        match value {
            Value::Boolean(b) => *b,
            Value::Number(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::List(l) => !l.is_empty(),
            Value::Object(o) => !o.is_empty(),
            Value::Null => false,
        }
    }

    pub fn set_variable(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }

    // Helper function to call positional evaluation functions
    fn eval_positional_function(&mut self, name: &str, args: Vec<Expression>) -> Result<Value, String> {
        match name {
            "get" => self.eval_get_positional(args),
            "run" => self.eval_run_positional(args),
            "wait" => self.eval_wait_positional(args),
            "read" => crate::file_io::eval_read_positional(args, self),
            "write" => crate::file_io::eval_write_positional(args, self),
            "clear" => crate::terminal::eval_clear_positional(args, self),
            _ => Err(format!("Unknown positional function: {}", name)),
        }
    }

    pub fn eval_program(&mut self, program: Program) -> Result<Value, String> {
        let mut last_value = Value::Null;

        for statement in program {
            last_value = self.eval_statement(statement)?;
        }

        Ok(last_value)
    }

    fn eval_statement(&mut self, stmt: Statement) -> Result<Value, String> {
        let (value, _control) = self.eval_statement_with_control(stmt)?;
        Ok(value)
    }

    fn eval_statement_with_control(&mut self, stmt: Statement) -> EvalResult {
        match stmt {
            Statement::Assignment { variable, value } => {
                let val = self.eval_expression(value)?;
                // Check if we're in a local scope
                if let Some(scope) = self.scope_stack.last_mut() {
                    scope.insert(variable, val.clone());
                } else {
                    // Otherwise assign to global scope
                    self.variables.insert(variable, val.clone());
                }
                Ok((val, ControlFlow::Continue))
            }
            Statement::PropertyAssignment {
                object,
                property,
                value,
            } => {
                let val = self.eval_expression(value)?;

                // Get the object expression (for now, only support variables)
                if let Expression::Variable(var_name) = object.as_ref() {
                    // Use scope-aware variable lookup
                    if let Ok(mut obj_value) = self.get_variable_value(var_name) {
                        match &mut obj_value {
                            Value::Object(map) => {
                                map.insert(property.clone(), val.clone());
                                self.update_variable(var_name.clone(), obj_value)?;
                                Ok((val, ControlFlow::Continue))
                            }
                            Value::List(list) => {
                                // Try to parse property as numeric index
                                if let Ok(index) = property.parse::<usize>() {
                                    // Expand list if necessary (fill with nulls)
                                    while list.len() <= index {
                                        list.push(Value::Null);
                                    }
                                    list[index] = val.clone();
                                    self.update_variable(var_name.clone(), obj_value)?;
                                    Ok((val, ControlFlow::Continue))
                                } else {
                                    Err(format!(
                                        "Cannot set non-numeric property '{}' on list",
                                        property
                                    ))
                                }
                            }
                            _ => Err(format!(
                                "Cannot set property '{}' on non-object/non-list value",
                                property
                            ))
                        }
                    } else {
                        // Create a new object or list if variable doesn't exist
                        if let Ok(index) = property.parse::<usize>() {
                            // Creating a new list
                            let mut list = Vec::new();
                            // Expand list if necessary (fill with nulls)
                            while list.len() <= index {
                                list.push(Value::Null);
                            }
                            list[index] = val.clone();
                            // Use scope-aware assignment for new variables
                            if let Some(scope) = self.scope_stack.last_mut() {
                                scope.insert(var_name.clone(), Value::List(list));
                            } else {
                                self.variables.insert(var_name.clone(), Value::List(list));
                            }
                            Ok((val, ControlFlow::Continue))
                        } else {
                            // Creating a new object
                            let mut map = std::collections::HashMap::new();
                            map.insert(property.clone(), val.clone());
                            // Use scope-aware assignment for new variables
                            if let Some(scope) = self.scope_stack.last_mut() {
                                scope.insert(var_name.clone(), Value::Object(map));
                            } else {
                                self.variables.insert(var_name.clone(), Value::Object(map));
                            }
                            Ok((val, ControlFlow::Continue))
                        }
                    }
                } else if let Expression::PropertyAccess {
                    object: nested_object,
                    property: nested_property,
                } = object.as_ref()
                {
                    // Handle nested property assignment like ~obj.nested.prop is value
                    // We need to recursively navigate and update the nested structure
                    self.assign_nested_property(
                        nested_object,
                        nested_property,
                        &property,
                        val.clone(),
                    )?;
                    Ok((val, ControlFlow::Continue))
                } else {
                    Err("Property assignment only supported on variables and single-level nesting currently".to_string())
                }
            }
            Statement::Expression(expr) => Ok((self.eval_expression(expr)?, ControlFlow::Continue)),
            Statement::If {
                condition,
                then_stmt,
                else_stmt,
            } => {
                let cond_value = self.eval_expression(condition)?;

                if cond_value.is_truthy() {
                    self.eval_statement_with_control(*then_stmt)
                } else if let Some(else_branch) = else_stmt {
                    self.eval_statement_with_control(*else_branch)
                } else {
                    Ok((Value::Null, ControlFlow::Continue))
                }
            }
            Statement::Loop { body } => loop {
                for stmt in &body {
                    let (_, control) = self.eval_statement_with_control(stmt.clone())?;
                    if control == ControlFlow::BreakLoop {
                        return Ok((Value::Null, ControlFlow::Continue));
                    }
                }
            },
            Statement::ForEach { variables, iterable, body } => {
                let iterable_value = self.eval_expression(iterable)?;

                match iterable_value {
                    Value::List(items) => {
                        for (index, item) in items.iter().enumerate() {
                            // Create a new scope for loop variables
                            let saved_vars = variables.iter().map(|var| {
                                (var.clone(), self.variables.get(var).cloned())
                            }).collect::<Vec<_>>();

                            // Set loop variables based on count
                            match variables.len() {
                                1 => {
                                    // for-each ~item in ~list
                                    self.variables.insert(variables[0].clone(), item.clone());
                                }
                                2 => {
                                    // for-each ~item ~index in ~list
                                    self.variables.insert(variables[0].clone(), item.clone());
                                    self.variables.insert(variables[1].clone(), Value::Number(index as f64));
                                }
                                _ => return Err("for-each expects 1 or 2 variables".to_string()),
                            }

                            // Execute loop body
                            for stmt in &body {
                                let (_, control) = self.eval_statement_with_control(stmt.clone())?;
                                if control == ControlFlow::BreakLoop {
                                    // Restore variables and exit
                                    for (var, old_value) in saved_vars {
                                        if let Some(value) = old_value {
                                            self.variables.insert(var, value);
                                        } else {
                                            self.variables.remove(&var);
                                        }
                                    }
                                    return Ok((Value::Null, ControlFlow::Continue));
                                }
                            }

                            // Restore variables after each iteration
                            for (var, old_value) in saved_vars {
                                if let Some(value) = old_value {
                                    self.variables.insert(var, value);
                                } else {
                                    self.variables.remove(&var);
                                }
                            }
                        }
                    }
                    Value::Object(obj) => {
                        for (key, value) in obj.iter() {
                            // Create a new scope for loop variables
                            let saved_vars = variables.iter().map(|var| {
                                (var.clone(), self.variables.get(var).cloned())
                            }).collect::<Vec<_>>();

                            // Set loop variables based on count
                            match variables.len() {
                                1 => {
                                    // for-each ~value in ~object (values only)
                                    self.variables.insert(variables[0].clone(), value.clone());
                                }
                                2 => {
                                    // for-each ~key ~value in ~object
                                    self.variables.insert(variables[0].clone(), Value::String(key.clone()));
                                    self.variables.insert(variables[1].clone(), value.clone());
                                }
                                _ => return Err("for-each expects 1 or 2 variables".to_string()),
                            }

                            // Execute loop body
                            for stmt in &body {
                                let (_, control) = self.eval_statement_with_control(stmt.clone())?;
                                if control == ControlFlow::BreakLoop {
                                    // Restore variables and exit
                                    for (var, old_value) in saved_vars {
                                        if let Some(value) = old_value {
                                            self.variables.insert(var, value);
                                        } else {
                                            self.variables.remove(&var);
                                        }
                                    }
                                    return Ok((Value::Null, ControlFlow::Continue));
                                }
                            }

                            // Restore variables after each iteration
                            for (var, old_value) in saved_vars {
                                if let Some(value) = old_value {
                                    self.variables.insert(var, value);
                                } else {
                                    self.variables.remove(&var);
                                }
                            }
                        }
                    }
                    _ => return Err("for-each can only iterate over lists and objects".to_string()),
                }
                Ok((Value::Null, ControlFlow::Continue))
            },
            Statement::Increment { variable, amount } => {
                let amount_value = self.eval_expression(amount)?;

                // Get current variable value using proper scope lookup
                let current_value = self.get_variable_value(&variable)?;

                // Ensure both values are numbers
                match (&current_value, &amount_value) {
                    (Value::Number(current), Value::Number(increment)) => {
                        let new_value = current + increment;
                        self.update_variable(variable.clone(), Value::Number(new_value))?;
                        Ok((Value::Null, ControlFlow::Continue))
                    }
                    _ => Err(format!("Cannot increment '{}': both variable and amount must be numbers", variable))
                }
            },
            Statement::Decrement { variable, amount } => {
                let amount_value = self.eval_expression(amount)?;

                // Get current variable value using proper scope lookup
                let current_value = self.get_variable_value(&variable)?;

                // Ensure both values are numbers
                match (&current_value, &amount_value) {
                    (Value::Number(current), Value::Number(decrement)) => {
                        let new_value = current - decrement;
                        self.update_variable(variable.clone(), Value::Number(new_value))?;
                        Ok((Value::Null, ControlFlow::Continue))
                    }
                    _ => Err(format!("Cannot decrement '{}': both variable and amount must be numbers", variable))
                }
            },
            Statement::Block { body } => {
                let mut last_value = Value::Null;
                for stmt in body {
                    let (value, control) = self.eval_statement_with_control(stmt)?;
                    last_value = value;
                    if control == ControlFlow::BreakLoop {
                        return Ok((last_value, ControlFlow::BreakLoop));
                    }
                }
                Ok((last_value, ControlFlow::Continue))
            }
            Statement::Breakloop => Ok((Value::Null, ControlFlow::BreakLoop)),
            Statement::Open(_) => Ok((
                Value::String("open not implemented yet".to_string()),
                ControlFlow::Continue,
            )),
            Statement::FunctionDefinition { name, params, body } => {
                // Store the action definition
                self.functions.insert(
                    name.clone(),
                    Function {
                        params: params.clone(),
                        body: body.clone(),
                    },
                );
                Ok((Value::Null, ControlFlow::Continue))
            }
            Statement::Give(expr) => {
                let value = self.eval_expression(expr)?;
                Ok((value.clone(), ControlFlow::Give(value)))
            }
        }
    }

    fn update_variable(&mut self, name: String, value: Value) -> Result<(), String> {
        // Search through scope stack to find where the variable exists
        for scope in self.scope_stack.iter_mut().rev() {
            if scope.contains_key(&name) {
                scope.insert(name, value);
                return Ok(());
            }
        }

        // If not found in any scope, check if it exists in global variables
        if self.variables.contains_key(&name) {
            self.variables.insert(name, value);
            Ok(())
        } else {
            // Variable doesn't exist anywhere - this should not happen for increment/decrement
            // since we already validated it exists in get_variable_value
            Err(format!("Variable '{}' not found for update", name))
        }
    }

    fn get_variable_value(&self, name: &str) -> Result<Value, String> {
        // CRITICAL OPTIMIZATION: Most variable lookups in recursive functions are parameters
        // in the current scope. Check the most recent scope first as a fast path.
        if let Some(current_scope) = self.scope_stack.last() {
            if let Some(value) = current_scope.get(name) {
                return Ok(value.clone());
            }
        }

        // If not in current scope, check remaining scopes (rare for recursive functions)
        if self.scope_stack.len() > 1 {
            for scope in self.scope_stack.iter().rev().skip(1) {
                if let Some(value) = scope.get(name) {
                    return Ok(value.clone());
                }
            }
        }

        // Finally check global scope (least common for recursive functions)
        if let Some(value) = self.variables.get(name) {
            Ok(value.clone())
        } else {
            Err(format!("Undefined variable: {}", name))
        }
    }


    pub fn eval_expression(&mut self, expr: Expression) -> Result<Value, String> {
        match expr {
            Expression::Number(n, _) => Ok(Value::Number(n)),
            Expression::String(s) => Ok(Value::String(s)),
            Expression::InterpolatedString(parts) => {
                let mut result = String::new();
                for part in parts {
                    match part {
                        crate::ast::InterpolationPart::Text(text) => result.push_str(&text),
                        crate::ast::InterpolationPart::Variable(var_name) => {
                            let value = self.get_variable_value(&var_name)?;
                            result.push_str(&value.to_string());
                        }
                        crate::ast::InterpolationPart::Expression(expr) => {
                            let value = self.eval_expression(expr)?;
                            result.push_str(&value.to_string());
                        }
                    }
                }
                Ok(Value::String(result))
            }
            Expression::Boolean(b) => Ok(Value::Boolean(b)),
            Expression::Variable(name) => self.get_variable_value(&name),
            Expression::List(items) => {
                let mut values = Vec::new();
                for item in items {
                    values.push(self.eval_expression(item)?);
                }
                Ok(Value::List(values))
            }
            Expression::BinaryOp { left, op, right } => {
                // Handle logical operators with short-circuit evaluation
                match op {
                    BinaryOperator::And => {
                        let left_val = self.eval_expression(*left)?;
                        if self.is_truthy(&left_val) {
                            // Left is truthy, return right operand
                            self.eval_expression(*right)
                        } else {
                            // Left is falsy, return left operand (short-circuit)
                            Ok(left_val)
                        }
                    }
                    BinaryOperator::Or => {
                        let left_val = self.eval_expression(*left)?;
                        if self.is_truthy(&left_val) {
                            // Left is truthy, return left operand (short-circuit)
                            Ok(left_val)
                        } else {
                            // Left is falsy, return right operand
                            self.eval_expression(*right)
                        }
                    }
                    _ => {
                        // For other operators, evaluate both operands first
                        let left_val = self.eval_expression(*left)?;
                        let right_val = self.eval_expression(*right)?;

                        match (left_val, right_val, op) {
                    (Value::Number(l), Value::Number(r), BinaryOperator::Add) => {
                        Ok(Value::Number(l + r))
                    }
                    (Value::Number(l), Value::Number(r), BinaryOperator::Subtract) => {
                        Ok(Value::Number(l - r))
                    }
                    (Value::Number(l), Value::Number(r), BinaryOperator::Multiply) => {
                        Ok(Value::Number(l * r))
                    }
                    (Value::Number(l), Value::Number(r), BinaryOperator::Divide) => {
                        if r == 0.0 {
                            Err("Division by zero".to_string())
                        } else {
                            Ok(Value::Number(l / r))
                        }
                    }
                    (Value::Number(l), Value::Number(r), BinaryOperator::IntegerDivide) => {
                        if r == 0.0 {
                            Err("Division by zero".to_string())
                        } else {
                            Ok(Value::Number((l / r).floor()))
                        }
                    }
                    (Value::Number(l), Value::Number(r), BinaryOperator::Modulo) => {
                        if r == 0.0 {
                            Err("Modulo by zero".to_string())
                        } else {
                            Ok(Value::Number(l % r))
                        }
                    }
                    (Value::Number(l), Value::Number(r), BinaryOperator::LessThan) => {
                        Ok(Value::Boolean(l < r))
                    }
                    (Value::Number(l), Value::Number(r), BinaryOperator::LessThanOrEqual) => {
                        Ok(Value::Boolean(l <= r))
                    }
                    (Value::Number(l), Value::Number(r), BinaryOperator::GreaterThan) => {
                        Ok(Value::Boolean(l > r))
                    }
                    (Value::Number(l), Value::Number(r), BinaryOperator::GreaterThanOrEqual) => {
                        Ok(Value::Boolean(l >= r))
                    }
                    (l, r, BinaryOperator::Equal) => Ok(Value::Boolean(l == r)),
                    (l, r, BinaryOperator::NotEqual) => Ok(Value::Boolean(l != r)),
                    (Value::String(l), Value::String(r), BinaryOperator::Add) => {
                        Ok(Value::String(format!("{}{}", l, r)))
                    }
                    _ => Err("Invalid operation".to_string()),
                        }
                    }
                }
            }
            Expression::FunctionCall { name, args } => {
                match name.as_str() {
                    "say" => {
                        let mut output = Vec::new();
                        for arg in args {
                            let value = self.eval_expression(arg)?;
                            output.push(value.to_string());
                        }
                        let message = output.join("");

                        #[cfg(target_arch = "wasm32")]
                        {
                            self.output_buffer.push(message.clone());
                        }

                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            println!("{}", message);
                        }

                        Ok(Value::String(message))
                    }
                    "get" | "run" | "wait" | "read" | "write" | "clear" => self.eval_positional_function(&name, args),
                    "ask" => {
                        #[cfg(target_arch = "wasm32")]
                        {
                            // For WASM, return a placeholder for now
                            // In a real implementation, this would use a JavaScript prompt
                            let prompt = if !args.is_empty() {
                                let mut prompt_values = Vec::new();
                                for arg in args {
                                    prompt_values.push(self.eval_expression(arg)?);
                                }
                                prompt_values
                                    .iter()
                                    .map(|v| v.to_string())
                                    .collect::<Vec<String>>()
                                    .join(" ")
                            } else {
                                String::new()
                            };

                            // Call JavaScript prompt function
                            use crate::wasm::prompt_user;
                            let result = prompt_user(&prompt);
                            Ok(Value::String(result))
                        }

                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            use std::io::{self, Write};

                            // If there are prompt arguments, evaluate and display them
                            if !args.is_empty() {
                                let mut prompt_values = Vec::new();
                                for arg in args {
                                    prompt_values.push(self.eval_expression(arg)?);
                                }

                                let prompt = prompt_values
                                    .iter()
                                    .map(|v| v.to_string())
                                    .collect::<Vec<String>>()
                                    .join(" ");

                                print!("{}", prompt);
                                io::stdout().flush().unwrap();
                            }

                            // Read user input
                            let mut input = String::new();
                            io::stdin()
                                .read_line(&mut input)
                                .map_err(|e| format!("Failed to read input: {}", e))?;

                            let input = input.trim().to_string();

                            // Try to parse as number first, fall back to string
                            if let Ok(num) = input.parse::<f64>() {
                                Ok(Value::Number(num))
                            } else {
                                Ok(Value::String(input))
                            }
                        }
                    }
                    _ => {
                        // Check for block syntax first (e.g., :core:is-even)
                        if name.starts_with(':') && name.contains(':') {
                            // Extract the block name and function name
                            let parts: Vec<&str> = name[1..].split(':').collect();
                            if parts.len() == 2 {
                                let block_name = parts[0];
                                let func_name = parts[1];

                                // Handle different blocks
                                match block_name {
                                    "core" => {
                                        // Force use of stdlib function, bypassing user definitions
                                        if let Some(func) = crate::stdlib::get_stdlib_function(func_name) {
                                            func(args, self)
                                        } else {
                                            Err(format!("Unknown core function: {}", func_name))
                                        }
                                    }
                                    _ => Err(format!("Unknown block: {}", block_name))
                                }
                            } else {
                                Err(format!("Invalid block syntax: {}", name))
                            }
                        } else {
                            // Check if this is a user-defined action first
                            if let Some(function) = self.functions.get(&name).cloned() {
                                self.eval_function(function, args)
                            } else if let Some(func) = crate::stdlib::get_stdlib_function(&name) {
                                // Check if this is a stdlib function
                                func(args, self)
                            } else {
                                Err(format!("Unknown function: {}", name))
                            }
                        }
                    }
                }
            }
            Expression::PropertyAccess { object, property } => {
                let obj_value = self.eval_expression(*object)?;
                match obj_value {
                    Value::Object(map) => {
                        match map.get(&property) {
                            Some(value) => Ok(value.clone()),
                            None => Ok(Value::Null), // Return null for non-existent properties
                        }
                    }
                    Value::List(list) => {
                        // Check if property is a variable (starts with ~)
                        let index_value = if property.starts_with('~') {
                            // Remove the ~ for lookup (variables are stored without ~ prefix everywhere)
                            let var_name_without_tilde = &property[1..];

                            // Search in scope stack first (function local variables stored without ~)
                            let mut found_value = None;
                            for scope in self.scope_stack.iter().rev() {
                                if let Some(val) = scope.get(var_name_without_tilde) {
                                    found_value = Some(val.clone());
                                    break;
                                }
                            }

                            // If not found in scope, check global variables (also stored without ~)
                            match found_value.or_else(|| self.variables.get(var_name_without_tilde).cloned()) {
                                Some(val) => val,
                                None => return Err(format!("Undefined variable: ~{}", var_name_without_tilde)),
                            }
                        } else {
                            // Try to parse property as literal numeric index
                            match property.parse::<f64>() {
                                Ok(n) => Value::Number(n),
                                Err(_) => return Err(format!(
                                    "Cannot access non-numeric property '{}' on list",
                                    property
                                )),
                            }
                        };

                        // Convert the index value to usize
                        match index_value {
                            Value::Number(n) => {
                                let index = n as usize;
                                if index < list.len() {
                                    Ok(list[index].clone())
                                } else {
                                    Ok(Value::Null) // Return null for out-of-bounds access
                                }
                            }
                            Value::String(s) => {
                                // Try to parse string as number (for cases like "1" -> 1)
                                match s.parse::<f64>() {
                                    Ok(n) => {
                                        let index = n as usize;
                                        if index < list.len() {
                                            Ok(list[index].clone())
                                        } else {
                                            Ok(Value::Null)
                                        }
                                    }
                                    Err(_) => Err(format!(
                                        "List index must be numeric, got string: '{}'",
                                        s
                                    )),
                                }
                            }
                            _ => Err(format!(
                                "List index must be a number, got: {:?}",
                                index_value
                            )),
                        }
                    }
                    _ => Err(format!(
                        "Cannot access property '{}' on non-object/non-list value",
                        property
                    )),
                }
            }
            Expression::ObjectLiteral { pairs } => {
                let mut map = std::collections::HashMap::new();
                for (key, expr) in pairs {
                    let value = self.eval_expression(expr)?;
                    map.insert(key, value);
                }
                Ok(Value::Object(map))
            }
        }
    }

    fn assign_nested_property(
        &mut self,
        object_expr: &Expression,
        nested_prop: &str,
        final_prop: &str,
        value: Value,
    ) -> Result<(), String> {
        match object_expr {
            Expression::Variable(var_name) => {
                // Base case: we're at a variable, update its nested property
                if let Ok(mut obj_value) = self.get_variable_value(var_name) {
                    if let Value::Object(map) = &mut obj_value {
                        // Get or create the nested object
                        let nested_obj = map
                            .entry(nested_prop.to_string())
                            .or_insert_with(|| Value::Object(std::collections::HashMap::new()));
                        if let Value::Object(nested_map) = nested_obj {
                            nested_map.insert(final_prop.to_string(), value);
                            self.update_variable(var_name.clone(), obj_value)?;
                            Ok(())
                        } else {
                            Err(format!(
                                "Cannot set property '{}' on non-object value",
                                nested_prop
                            ))
                        }
                    } else {
                        Err(format!(
                            "Cannot set property on non-object variable '{}'",
                            var_name
                        ))
                    }
                } else {
                    Err(format!("Undefined variable: {}", var_name))
                }
            }
            Expression::PropertyAccess { object, property } => {
                // Recursive case: we need to go deeper
                self.assign_nested_property_recursive(
                    object,
                    property,
                    nested_prop,
                    final_prop,
                    value,
                )
            }
            _ => Err("Invalid property assignment target".to_string()),
        }
    }

    fn assign_nested_property_recursive(
        &mut self,
        object_expr: &Expression,
        current_prop: &str,
        nested_prop: &str,
        final_prop: &str,
        value: Value,
    ) -> Result<(), String> {
        match object_expr {
            Expression::Variable(var_name) => {
                if let Ok(mut obj_value) = self.get_variable_value(var_name) {
                    if let Value::Object(map) = &mut obj_value {
                        // Get or create the current property's object
                        let current_obj = map
                            .entry(current_prop.to_string())
                            .or_insert_with(|| Value::Object(std::collections::HashMap::new()));
                        if let Value::Object(current_map) = current_obj {
                            // Get or create the nested object
                            let nested_obj = current_map
                                .entry(nested_prop.to_string())
                                .or_insert_with(|| Value::Object(std::collections::HashMap::new()));
                            if let Value::Object(nested_map) = nested_obj {
                                nested_map.insert(final_prop.to_string(), value);
                                self.update_variable(var_name.clone(), obj_value)?;
                                Ok(())
                            } else {
                                Err(format!(
                                    "Cannot set property '{}' on non-object value",
                                    nested_prop
                                ))
                            }
                        } else {
                            Err(format!(
                                "Cannot set property '{}' on non-object value",
                                current_prop
                            ))
                        }
                    } else {
                        Err(format!(
                            "Cannot set property on non-object variable '{}'",
                            var_name
                        ))
                    }
                } else {
                    Err(format!("Undefined variable: {}", var_name))
                }
            }
            _ => Err("Deep nested property assignment not fully supported yet".to_string()),
        }
    }

    pub fn get_variable(&self, name: &str) -> Option<&Value> {
        self.variables.get(name)
    }

    fn json_to_tails_value(json: serde_json::Value) -> Value {
        match json {
            serde_json::Value::Null => Value::Null,
            serde_json::Value::Bool(b) => Value::Boolean(b),
            serde_json::Value::Number(n) => {
                if let Some(f) = n.as_f64() {
                    Value::Number(f)
                } else {
                    Value::Number(0.0)
                }
            }
            serde_json::Value::String(s) => Value::String(s),
            serde_json::Value::Array(arr) => {
                let tails_list: Vec<Value> =
                    arr.into_iter().map(Self::json_to_tails_value).collect();
                Value::List(tails_list)
            }
            serde_json::Value::Object(obj) => {
                let mut tails_map = HashMap::new();
                for (key, value) in obj {
                    tails_map.insert(key, Self::json_to_tails_value(value));
                }
                Value::Object(tails_map)
            }
        }
    }

    fn eval_get_positional(&mut self, args: Vec<Expression>) -> Result<Value, String> {
        if args.is_empty() {
            return Err("get requires at least a URL argument".to_string());
        }

        let url_value = self.eval_expression(args[0].clone())?;
        let url = match url_value {
            Value::String(s) => s,
            _ => return Err("get URL must be a string".to_string()),
        };

        #[cfg(target_arch = "wasm32")]
        {
            // For WASM, call JavaScript fetch function
            use crate::wasm::fetch_url;
            let body = fetch_url(&url);

            // Try to parse as JSON, fall back to string if parsing fails
            match serde_json::from_str::<serde_json::Value>(&body) {
                Ok(json) => Ok(Self::json_to_tails_value(json)),
                Err(_) => Ok(Value::String(body)),
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let timeout = if args.len() > 1 {
                let timeout_value = self.eval_expression(args[1].clone())?;
                match timeout_value {
                    Value::Number(n) => n as u64,
                    _ => return Err("get timeout must be a number".to_string()),
                }
            } else {
                30 // default timeout
            };

            let config = ureq::Agent::config_builder()
                .timeout_global(Some(std::time::Duration::from_secs(timeout)))
                .build();
            let agent: ureq::Agent = config.into();
            let response = agent
                .get(&url)
                .call()
                .map_err(|e| format!("HTTP request failed: {}", e))?;

            let mut response = response;
            let body = response
                .body_mut()
                .read_to_string()
                .map_err(|e| format!("Failed to read response body: {}", e))?;

            // Try to parse as JSON, fall back to string if parsing fails
            match serde_json::from_str::<serde_json::Value>(&body) {
                Ok(json) => Ok(Self::json_to_tails_value(json)),
                Err(_) => Ok(Value::String(body)),
            }
        }
    }

    fn eval_run_positional(&mut self, args: Vec<Expression>) -> Result<Value, String> {
        if args.is_empty() {
            return Err("run requires a command argument".to_string());
        }

        let command_value = self.eval_expression(args[0].clone())?;
        let command = match command_value {
            Value::String(s) => s,
            _ => return Err("run command must be a string".to_string()),
        };

        // Execute the shell command
        let output = std::process::Command::new("sh")
            .arg("-c")
            .arg(&command)
            .output()
            .map_err(|e| format!("Failed to execute command: {}", e))?;

        // Combine stdout and stderr
        let mut result = String::from_utf8_lossy(&output.stdout).to_string();
        if !output.stderr.is_empty() {
            if !result.is_empty() {
                result.push('\n');
            }
            result.push_str(&String::from_utf8_lossy(&output.stderr));
        }

        // Create result object
        let mut result_map = std::collections::HashMap::new();
        result_map.insert("output".to_string(), Value::String(result));

        Ok(Value::Object(result_map))
    }

    fn eval_wait_positional(&mut self, args: Vec<Expression>) -> Result<Value, String> {
        if args.is_empty() {
            return Err("wait requires a duration argument".to_string());
        }

        let duration_value = self.eval_expression(args[0].clone())?;
        let seconds = match duration_value {
            Value::Number(n) => {
                if n < 0.0 {
                    return Err("wait duration cannot be negative".to_string());
                }
                n
            }
            _ => return Err("wait duration must be a number".to_string()),
        };

        let duration = std::time::Duration::from_secs_f64(seconds);
        std::thread::sleep(duration);
        Ok(Value::Null)
    }

    pub fn eval_function(&mut self, function: Function, args: Vec<Expression>) -> Result<Value, String> {
        // Check call depth to prevent stack overflow
        if self.scope_stack.len() >= self.max_call_depth {
            return Err(format!("Maximum call depth ({}) exceeded", self.max_call_depth));
        }

        // Check argument count
        if args.len() != function.params.len() {
            return Err(format!(
                "Action expects {} arguments, but {} were provided",
                function.params.len(),
                args.len()
            ));
        }

        // Evaluate arguments
        let mut arg_values = Vec::new();
        for arg in args {
            arg_values.push(self.eval_expression(arg)?);
        }

        // Performance optimization: Pre-allocate HashMap with known capacity
        // This reduces memory allocations for recursive functions
        let mut local_scope = HashMap::with_capacity(function.params.len());
        for (param, value) in function.params.iter().zip(arg_values.into_iter()) {
            local_scope.insert(param.clone(), value); // Avoid cloning value
        }

        // Push the new scope
        self.scope_stack.push(local_scope);

        // Execute action body
        let mut last_value = Value::Null;
        for stmt in function.body {
            let (value, control) = self.eval_statement_with_control(stmt)?;

            match control {
                ControlFlow::Give(return_value) => {
                    // Pop the scope and return the value
                    self.scope_stack.pop();
                    return Ok(return_value);
                }
                ControlFlow::BreakLoop => {
                    // break-loop in action should only break local loops, not return
                    last_value = value;
                }
                ControlFlow::Continue => {
                    last_value = value;
                }
            }
        }

        // Pop the scope
        self.scope_stack.pop();

        // If no explicit give, return the last value
        Ok(last_value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Parser;

    #[test]
    fn test_simple_assignment() {
        let mut parser = Parser::new("~x is 42");
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        evaluator.eval_program(program).unwrap();

        assert_eq!(evaluator.get_variable("x"), Some(&Value::Number(42.0)));
    }

    #[test]
    fn test_arithmetic() {
        let mut parser = Parser::new("~x is 10\n~y is ~x + 5");
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        evaluator.eval_program(program).unwrap();

        assert_eq!(evaluator.get_variable("y"), Some(&Value::Number(15.0)));
    }

    #[test]
    fn test_integer_division() {
        let mut parser = Parser::new("~result is 7 \\ 3");
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        evaluator.eval_program(program).unwrap();

        assert_eq!(evaluator.get_variable("result"), Some(&Value::Number(2.0)));
    }

    #[test]
    fn test_modulo() {
        let mut parser = Parser::new("~remainder is 7 % 3");
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        evaluator.eval_program(program).unwrap();

        assert_eq!(
            evaluator.get_variable("remainder"),
            Some(&Value::Number(1.0))
        );
    }

    #[test]
    fn test_modulo_even_check() {
        let mut parser = Parser::new("~n is 6\n~is_even is ~n % 2 == 0");
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        evaluator.eval_program(program).unwrap();

        assert_eq!(
            evaluator.get_variable("is_even"),
            Some(&Value::Boolean(true))
        );
    }

    #[test]
    fn test_division_by_zero_error() {
        let mut parser = Parser::new("~result is 5 \\ 0");
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        let result = evaluator.eval_program(program);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Division by zero"));
    }

    #[test]
    fn test_modulo_by_zero_error() {
        let mut parser = Parser::new("~result is 5 % 0");
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        let result = evaluator.eval_program(program);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Modulo by zero"));
    }

    #[test]
    fn test_comparison() {
        let mut parser = Parser::new("~x is 10\n~result is ~x < 20");
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        evaluator.eval_program(program).unwrap();

        assert_eq!(
            evaluator.get_variable("result"),
            Some(&Value::Boolean(true))
        );
    }

    #[test]
    fn test_ask_function_call_parsing() {
        // Test that ask parses as a function call
        let mut parser = Parser::new("~response is ask \"Enter value: \"");
        let program = parser.parse().unwrap();

        assert_eq!(program.len(), 1);
        if let Statement::Assignment { variable, value } = &program[0] {
            assert_eq!(variable, "response");
            if let Expression::FunctionCall { name, args } = value {
                assert_eq!(name, "ask");
                assert_eq!(args.len(), 1);
            } else {
                panic!("Expected function call expression");
            }
        } else {
            panic!("Expected assignment statement");
        }
    }

    #[test]
    fn test_ask_statement_parsing() {
        // Test that ask also works as a standalone statement
        let mut parser = Parser::new("ask \"Enter value: \"");
        let program = parser.parse().unwrap();

        assert_eq!(program.len(), 1);
        if let Statement::Expression(Expression::FunctionCall { name, args }) = &program[0] {
            assert_eq!(name, "ask");
            assert_eq!(args.len(), 1);
        } else {
            panic!("Expected ask function call");
        }
    }

    #[test]
    fn test_nested_property_assignment() {
        let mut parser =
            Parser::new("~app is {}\n~app.settings is {}\n~app.settings.theme is \"dark\"");
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        evaluator.eval_program(program).unwrap();

        // Check that the nested structure was created correctly
        if let Some(Value::Object(app_map)) = evaluator.get_variable("app") {
            if let Some(Value::Object(settings_map)) = app_map.get("settings") {
                assert_eq!(
                    settings_map.get("theme"),
                    Some(&Value::String("dark".to_string()))
                );
            } else {
                panic!("Expected settings to be an object");
            }
        } else {
            panic!("Expected app to be an object");
        }
    }

    #[test]
    fn test_nested_property_assignment_with_existing_data() {
        let mut parser = Parser::new(
            "~config is {\"database\": {\"host\": \"localhost\" \"port\": 5432} \"debug\": true}\n~config.database.host is \"production\"",
        );
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        evaluator.eval_program(program).unwrap();

        if let Some(Value::Object(config_map)) = evaluator.get_variable("config") {
            // Check that the host was updated
            if let Some(Value::Object(db_map)) = config_map.get("database") {
                assert_eq!(
                    db_map.get("host"),
                    Some(&Value::String("production".to_string()))
                );
                // Check that port was preserved
                assert_eq!(db_map.get("port"), Some(&Value::Number(5432.0)));
            } else {
                panic!("Expected database to be an object");
            }
            // Check that debug was preserved
            assert_eq!(config_map.get("debug"), Some(&Value::Boolean(true)));
        } else {
            panic!("Expected config to be an object");
        }
    }

    #[test]
    fn test_nested_property_assignment_creates_intermediate_objects() {
        let mut parser = Parser::new("~user is {\"name\": \"Alice\"}\n~user.profile.age is 30");
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        evaluator.eval_program(program).unwrap();

        if let Some(Value::Object(user_map)) = evaluator.get_variable("user") {
            // Check original property was preserved
            assert_eq!(
                user_map.get("name"),
                Some(&Value::String("Alice".to_string()))
            );

            // Check that intermediate profile object was created
            if let Some(Value::Object(profile_map)) = user_map.get("profile") {
                assert_eq!(profile_map.get("age"), Some(&Value::Number(30.0)));
            } else {
                panic!("Expected profile to be an object");
            }
        } else {
            panic!("Expected user to be an object");
        }
    }

    #[test]
    fn test_multiple_nested_property_assignments() {
        let mut parser = Parser::new(
            "~app is {}\n~app.ui.theme is \"dark\"\n~app.ui.fontSize is 16\n~app.data.cache is true",
        );
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        evaluator.eval_program(program).unwrap();

        if let Some(Value::Object(app_map)) = evaluator.get_variable("app") {
            // Check ui settings
            if let Some(Value::Object(ui_map)) = app_map.get("ui") {
                assert_eq!(
                    ui_map.get("theme"),
                    Some(&Value::String("dark".to_string()))
                );
                assert_eq!(ui_map.get("fontSize"), Some(&Value::Number(16.0)));
            } else {
                panic!("Expected ui to be an object");
            }

            // Check data settings
            if let Some(Value::Object(data_map)) = app_map.get("data") {
                assert_eq!(data_map.get("cache"), Some(&Value::Boolean(true)));
            } else {
                panic!("Expected data to be an object");
            }
        } else {
            panic!("Expected app to be an object");
        }
    }

    #[test]
    fn test_nested_property_access_after_assignment() {
        let mut parser = Parser::new(
            "~settings is {}\n~settings.theme is \"light\"\n~current_theme is ~settings.theme",
        );
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        evaluator.eval_program(program).unwrap();

        assert_eq!(
            evaluator.get_variable("current_theme"),
            Some(&Value::String("light".to_string()))
        );
    }

    #[test]
    fn test_wait_statement() {
        let mut parser = Parser::new("wait 0.01");
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        let start = std::time::Instant::now();
        evaluator.eval_program(program).unwrap();
        let elapsed = start.elapsed();

        // Should have waited at least 10ms
        assert!(elapsed >= std::time::Duration::from_millis(10));
    }

    #[test]
    fn test_wait_with_variable() {
        let mut parser = Parser::new("~delay is 0.01\nwait ~delay");
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        let start = std::time::Instant::now();
        evaluator.eval_program(program).unwrap();
        let elapsed = start.elapsed();

        // Should have waited at least 10ms
        assert!(elapsed >= std::time::Duration::from_millis(10));
    }

    #[test]
    fn test_wait_function_call() {
        // Test wait in assignment context
        let mut parser = Parser::new("~result is wait 0.01");
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        let start = std::time::Instant::now();
        evaluator.eval_program(program).unwrap();
        let elapsed = start.elapsed();

        // Should have waited at least 10ms
        assert!(elapsed >= std::time::Duration::from_millis(10));
        // Should return Null
        assert_eq!(evaluator.get_variable("result"), Some(&Value::Null));
    }

    #[test]
    fn test_wait_negative_seconds_error() {
        let mut parser = Parser::new("~neg is 0 - 1\nwait ~neg");
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        let result = evaluator.eval_program(program);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("cannot be negative"));
    }

    #[test]
    fn test_get_missing_url_error() {
        // Test get without url parameter
        let mut parser = Parser::new("get");
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        let result = evaluator.eval_program(program);

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .contains("get requires at least a URL argument")
        );
    }

    #[test]
    fn test_get_invalid_url_type_error() {
        // Test get with non-string url
        let mut parser = Parser::new("get 123");
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        let result = evaluator.eval_program(program);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("URL must be a string"));
    }

    #[test]
    fn test_wait_invalid_seconds_type_error() {
        // Test wait with non-numeric seconds
        let mut parser = Parser::new("wait \"invalid\"");
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        let result = evaluator.eval_program(program);

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .contains("wait duration must be a number")
        );
    }

    #[test]
    fn test_run_statement() {
        let mut parser = Parser::new("~result is run \"echo hello world\"");
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        evaluator.eval_program(program).unwrap();

        if let Some(Value::Object(result_map)) = evaluator.get_variable("result") {
            assert_eq!(
                result_map.get("output"),
                Some(&Value::String("hello world\n".to_string()))
            );
        } else {
            panic!("Expected result to be an object with output property");
        }
    }

    #[test]
    fn test_interpolated_string() {
        let mut parser = Parser::new("~name is \"world\"\n~greeting is \"Hello `~name`!\"");
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        evaluator.eval_program(program).unwrap();

        assert_eq!(
            evaluator.get_variable("greeting"),
            Some(&Value::String("Hello world!".to_string()))
        );
    }

    #[test]
    fn test_run_with_interpolation() {
        let mut parser = Parser::new("~word is \"hello\"\n~result is run \"echo `~word` world\"");
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        evaluator.eval_program(program).unwrap();

        if let Some(Value::Object(result_map)) = evaluator.get_variable("result") {
            assert_eq!(
                result_map.get("output"),
                Some(&Value::String("hello world\n".to_string()))
            );
        } else {
            panic!("Expected result to be an object with output property");
        }
    }

    #[test]
    fn test_interpolation_with_object_properties() {
        let mut parser = Parser::new("~user is {\"name\": \"Alice\" \"age\": 30}\n~message is \"User `~user.name` is `~user.age` years old\"");
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        evaluator.eval_program(program).unwrap();

        assert_eq!(
            evaluator.get_variable("message"),
            Some(&Value::String("User Alice is 30 years old".to_string()))
        );
    }

    #[test]
    fn test_interpolation_with_nested_object_properties() {
        let mut parser = Parser::new("~data is {\"user\": {\"profile\": {\"name\": \"Bob\"}}}\n~text is \"Hello `~data.user.profile.name`!\"");
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        evaluator.eval_program(program).unwrap();

        assert_eq!(
            evaluator.get_variable("text"),
            Some(&Value::String("Hello Bob!".to_string()))
        );
    }

    #[test]
    fn test_interpolation_mixed_variables_and_properties() {
        let mut parser = Parser::new("~name is \"Charlie\"\n~user is {\"age\": 25}\n~greeting is \"Hi `~name`, you are `~user.age` years old\"");
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        evaluator.eval_program(program).unwrap();

        assert_eq!(
            evaluator.get_variable("greeting"),
            Some(&Value::String("Hi Charlie, you are 25 years old".to_string()))
        );
    }

    #[test]
    fn test_interpolation_with_null_property() {
        let mut parser = Parser::new("~obj is {\"existing\": \"value\"}\n~text is \"Property: `~obj.missing`\"");
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        evaluator.eval_program(program).unwrap();

        assert_eq!(
            evaluator.get_variable("text"),
            Some(&Value::String("Property: null".to_string()))
        );
    }

    #[test]
    fn test_list_literal_creation() {
        let mut parser = Parser::new("~nums is [1, 2, 3]\n~mixed is [1, \"hello\", true]\n~empty is []");
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        evaluator.eval_program(program).unwrap();

        assert_eq!(
            evaluator.get_variable("nums"),
            Some(&Value::List(vec![
                Value::Number(1.0),
                Value::Number(2.0),
                Value::Number(3.0)
            ]))
        );

        assert_eq!(
            evaluator.get_variable("mixed"),
            Some(&Value::List(vec![
                Value::Number(1.0),
                Value::String("hello".to_string()),
                Value::Boolean(true)
            ]))
        );

        assert_eq!(
            evaluator.get_variable("empty"),
            Some(&Value::List(vec![]))
        );
    }

    #[test]
    fn test_list_indexing() {
        let mut parser = Parser::new("~nums is [10, 20, 30]\n~first is ~nums.0\n~second is ~nums.1\n~missing is ~nums.5");
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        evaluator.eval_program(program).unwrap();

        assert_eq!(
            evaluator.get_variable("first"),
            Some(&Value::Number(10.0))
        );
        assert_eq!(
            evaluator.get_variable("second"),
            Some(&Value::Number(20.0))
        );
        assert_eq!(
            evaluator.get_variable("missing"),
            Some(&Value::Null)
        );
    }

    #[test]
    fn test_list_assignment() {
        let mut parser = Parser::new("~nums is [10, 20, 30]\n~nums.1 is 99\n~nums.5 is 100");
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        evaluator.eval_program(program).unwrap();

        assert_eq!(
            evaluator.get_variable("nums"),
            Some(&Value::List(vec![
                Value::Number(10.0),
                Value::Number(99.0),
                Value::Number(30.0),
                Value::Null,
                Value::Null,
                Value::Number(100.0)
            ]))
        );
    }

    #[test]
    fn test_list_creation_through_assignment() {
        let mut parser = Parser::new("~newlist.0 is \"first\"\n~newlist.2 is \"third\"");
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        evaluator.eval_program(program).unwrap();

        assert_eq!(
            evaluator.get_variable("newlist"),
            Some(&Value::List(vec![
                Value::String("first".to_string()),
                Value::Null,
                Value::String("third".to_string())
            ]))
        );
    }

    #[test]
    fn test_list_length_function() {
        let mut parser = Parser::new("~nums is [10, 20, 30]\n~len is length ~nums\n~str_len is length \"hello\"");
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        evaluator.eval_program(program).unwrap();

        assert_eq!(
            evaluator.get_variable("len"),
            Some(&Value::Number(3.0))
        );
        assert_eq!(
            evaluator.get_variable("str_len"),
            Some(&Value::Number(5.0))
        );
    }

    #[test]
    fn test_list_append_function() {
        let mut parser = Parser::new("~nums is [10, 20]\n~new_nums is append ~nums 30");
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        evaluator.eval_program(program).unwrap();

        assert_eq!(
            evaluator.get_variable("new_nums"),
            Some(&Value::List(vec![
                Value::Number(10.0),
                Value::Number(20.0),
                Value::Number(30.0)
            ]))
        );

        // Original list should be unchanged
        assert_eq!(
            evaluator.get_variable("nums"),
            Some(&Value::List(vec![
                Value::Number(10.0),
                Value::Number(20.0)
            ]))
        );
    }

    #[test]
    fn test_nested_lists() {
        let mut parser = Parser::new("~nested is [[1, 2], [3, 4]]\n~first_row is ~nested.0\n~second_row is ~nested.1\n~element is ~second_row.0");
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        evaluator.eval_program(program).unwrap();

        assert_eq!(
            evaluator.get_variable("first_row"),
            Some(&Value::List(vec![
                Value::Number(1.0),
                Value::Number(2.0)
            ]))
        );

        assert_eq!(
            evaluator.get_variable("second_row"),
            Some(&Value::List(vec![
                Value::Number(3.0),
                Value::Number(4.0)
            ]))
        );

        assert_eq!(
            evaluator.get_variable("element"),
            Some(&Value::Number(3.0))
        );
    }

    #[test]
    fn test_list_interpolation() {
        let mut parser = Parser::new("~nums is [10, 20, 30]\n~message is \"First: `~nums.0`, Last: `~nums.2`\"");
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        evaluator.eval_program(program).unwrap();

        assert_eq!(
            evaluator.get_variable("message"),
            Some(&Value::String("First: 10, Last: 30".to_string()))
        );
    }

    #[test]
    fn test_property_access_on_undefined_variable() {
        let mut parser = Parser::new("~result is ~undefined.property");
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        let result = evaluator.eval_program(program);

        // Accessing property on undefined variable should error
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Undefined variable"));
    }

    #[test]
    fn test_property_access_invalid_type() {
        let mut parser = Parser::new("~num is 42\n~result is ~num.property");
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        let result = evaluator.eval_program(program);

        // Property access on invalid types does error
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Cannot access property"));
    }

    #[test]
    fn test_property_assignment_invalid_type() {
        let mut parser = Parser::new("~num is 42\n~num.property is \"value\"");
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        let result = evaluator.eval_program(program);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Cannot set property"));
    }

    #[test]
    fn test_arithmetic_invalid_types() {
        let mut parser = Parser::new("~result is \"hello\" + 42");
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        let result = evaluator.eval_program(program);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid operation"));
    }

    #[test]
    fn test_comparison_type_mismatch() {
        let mut parser = Parser::new("~result is \"hello\" < 42");
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        let result = evaluator.eval_program(program);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid operation"));
    }

    #[test]
    fn test_undefined_function_call() {
        let mut parser = Parser::new("~result is undefined_function 42");
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        let result = evaluator.eval_program(program);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unknown function"));
    }

    #[test]
    fn test_length_on_invalid_type() {
        let mut parser = Parser::new("~num is 42\n~result is length ~num");
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        let result = evaluator.eval_program(program);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("length can only be used on lists or strings"));
    }

    #[test]
    fn test_append_on_invalid_type() {
        let mut parser = Parser::new("~num is 42\n~result is append ~num \"item\"");
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        let result = evaluator.eval_program(program);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("append can only be used on lists"));
    }

    #[test]
    fn test_list_index_out_of_bounds() {
        let mut parser = Parser::new("~list is [1, 2, 3]\n~result is ~list.10");
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        evaluator.eval_program(program).unwrap();

        // Out of bounds should return null, not error
        assert_eq!(evaluator.get_variable("result"), Some(&Value::Null));
    }

    #[test]
    fn test_object_key_functions_invalid_args() {
        let mut parser = Parser::new("~result is keys \"not an object\"");
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        let result = evaluator.eval_program(program);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("keys can only be used on objects"));
    }

    #[test]
    fn test_values_of_invalid_args() {
        let mut parser = Parser::new("~result is values 42");
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        let result = evaluator.eval_program(program);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("values can only be used on objects"));
    }

    #[test]
    fn test_has_key_invalid_args() {
        let mut parser = Parser::new("~result is has \"key\" 42");
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        let result = evaluator.eval_program(program);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("has can only be used on objects"));
    }

    #[test]
    fn test_interpolated_string_with_function_call() {
        let mut parser = Parser::new("~name is \"World\"\n~message is \"Hello `~name`!\"\n~result is ~message");
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        evaluator.eval_program(program).unwrap();

        assert_eq!(
            evaluator.get_variable("result"),
            Some(&Value::String("Hello World!".to_string()))
        );
    }

    #[test]
    fn test_nested_property_assignment_null_intermediate() {
        let mut parser = Parser::new("~obj is {}\n~obj.nested.deep is \"value\"");
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        evaluator.eval_program(program).unwrap();

        // Should create intermediate objects
        if let Some(Value::Object(obj)) = evaluator.get_variable("obj") {
            if let Some(Value::Object(nested)) = obj.get("nested") {
                assert_eq!(nested.get("deep"), Some(&Value::String("value".to_string())));
            } else {
                panic!("Expected nested object");
            }
        } else {
            panic!("Expected object");
        }
    }

    #[test]
    fn test_block_syntax_core_functions() {
        use crate::parser::Parser;

        // Test :core:is-even
        let mut parser = Parser::new("~result is :core:is-even 4");
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        evaluator.eval_program(program).unwrap();

        assert_eq!(evaluator.get_variable("result"), Some(&Value::Boolean(true)));

        // Test :core:is-odd
        let mut parser = Parser::new("~result is :core:is-odd 5");
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        evaluator.eval_program(program).unwrap();

        assert_eq!(evaluator.get_variable("result"), Some(&Value::Boolean(true)));
    }

    #[test]
    fn test_block_syntax_priority_resolution() {
        use crate::parser::Parser;

        // Define a user function that conflicts with stdlib
        let mut parser = Parser::new("function is-even ~x ( give false )\n~user_result is is-even 4\n~core_result is :core:is-even 4");
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        evaluator.eval_program(program).unwrap();

        // User function should be called for bare name
        assert_eq!(evaluator.get_variable("user_result"), Some(&Value::Boolean(false)));

        // Core function should be called for :core: prefix
        assert_eq!(evaluator.get_variable("core_result"), Some(&Value::Boolean(true)));
    }

    #[test]
    fn test_block_syntax_unknown_block() {
        use crate::parser::Parser;

        let mut parser = Parser::new("~result is :unknown:test-func 1");
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        let result = evaluator.eval_program(program);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unknown block: unknown"));
    }

    #[test]
    fn test_block_syntax_unknown_core_function() {
        use crate::parser::Parser;

        let mut parser = Parser::new("~result is :core:unknown-function 1");
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        let result = evaluator.eval_program(program);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unknown core function: unknown-function"));
    }

    #[test]
    fn test_block_syntax_in_map_function() {
        use crate::parser::Parser;

        // Test that :core:is-even works as a function reference in map
        let mut parser = Parser::new("~nums is [1, 2, 3, 4]\n~results is map ~nums :core:is-even");
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        evaluator.eval_program(program).unwrap();

        assert_eq!(
            evaluator.get_variable("results"),
            Some(&Value::List(vec![
                Value::Boolean(false), // 1 is odd
                Value::Boolean(true),  // 2 is even
                Value::Boolean(false), // 3 is odd
                Value::Boolean(true),  // 4 is even
            ]))
        );
    }

    #[test]
    fn test_block_syntax_with_filter() {
        use crate::parser::Parser;

        // Test that :core: works with filter function
        let mut parser = Parser::new("~nums is [1, 2, 3, 4, 5]\n~evens is filter ~nums :core:is-even");
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        evaluator.eval_program(program).unwrap();

        assert_eq!(
            evaluator.get_variable("evens"),
            Some(&Value::List(vec![
                Value::Number(2.0),
                Value::Number(4.0),
            ]))
        );
    }

    #[test]
    fn test_block_syntax_mixed_with_user_functions() {
        use crate::parser::Parser;

        // Test mixing user functions and core functions in the same script
        let mut parser = Parser::new(
            "function custom-double ~x ( give ~x * 2 )\n\
             ~nums is [1, 2, 3]\n\
             ~user_doubled is map ~nums custom-double\n\
             ~core_doubled is map ~nums :core:double"
        );
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        evaluator.eval_program(program).unwrap();

        // Both should produce the same result but through different functions
        assert_eq!(
            evaluator.get_variable("user_doubled"),
            Some(&Value::List(vec![
                Value::Number(2.0),
                Value::Number(4.0),
                Value::Number(6.0),
            ]))
        );

        assert_eq!(
            evaluator.get_variable("core_doubled"),
            Some(&Value::List(vec![
                Value::Number(2.0),
                Value::Number(4.0),
                Value::Number(6.0),
            ]))
        );
    }

    #[test]
    fn test_block_syntax_chaining() {
        use crate::parser::Parser;

        // Test using core functions in sequence
        let mut parser = Parser::new(
            "~num is 5\n\
             ~doubled is :core:double ~num\n\
             ~squared is :core:square ~doubled"
        );
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        evaluator.eval_program(program).unwrap();

        assert_eq!(evaluator.get_variable("doubled"), Some(&Value::Number(10.0)));
        assert_eq!(evaluator.get_variable("squared"), Some(&Value::Number(100.0)));
    }

    #[test]
    fn test_block_syntax_edge_cases() {
        use crate::parser::Parser;

        // Test edge cases like zero, negative numbers
        let mut parser = Parser::new(
            "~zero_even is :core:is-even 0\n\
             ~neg_pos is :core:is-positive -5\n\
             ~zero_zero is :core:is-zero 0"
        );
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        evaluator.eval_program(program).unwrap();

        assert_eq!(evaluator.get_variable("zero_even"), Some(&Value::Boolean(true)));
        assert_eq!(evaluator.get_variable("neg_pos"), Some(&Value::Boolean(false)));
        assert_eq!(evaluator.get_variable("zero_zero"), Some(&Value::Boolean(true)));
    }

    #[test]
    fn test_block_syntax_library_pattern() {
        use crate::parser::Parser;

        // Test the library pattern mentioned in docs - using :core: for predictable behavior
        let mut parser = Parser::new(
            "function validate-positive ~x (\n\
                 if :core:is-positive ~x (\n\
                     give true\n\
                 ) else (\n\
                     give false\n\
                 )\n\
             )\n\
             ~result is *validate-positive 5"
        );
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        evaluator.eval_program(program).unwrap();

        assert_eq!(evaluator.get_variable("result"), Some(&Value::Boolean(true)));
    }

    #[test]
    fn test_block_syntax_multiple_blocks_parsing() {
        use crate::parser::Parser;

        // Test that multiple block syntaxes in one line parse correctly
        let mut parser = Parser::new("~a is :core:double 5\n~b is :core:square 3");
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        evaluator.eval_program(program).unwrap();

        assert_eq!(evaluator.get_variable("a"), Some(&Value::Number(10.0)));
        assert_eq!(evaluator.get_variable("b"), Some(&Value::Number(9.0)));
    }
}
