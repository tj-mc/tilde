use crate::ast::*;
use crate::http::{HttpClient, HttpRequest, parse_http_options};
use crate::value::Value;
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
    last_error: Option<Value>,
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
            max_call_depth: 100,
            output_buffer: Vec::new(),
            last_error: None,
        }
    }

    pub fn is_truthy(&self, value: &Value) -> bool {
        match value {
            Value::Boolean(b) => *b,
            Value::Number(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::List(l) => !l.is_empty(),
            Value::Object(o) => !o.is_empty(),
            Value::Date(_) => true,   // Dates are always truthy
            Value::Error(_) => false, // Errors are falsy
            Value::Null => false,
        }
    }

    pub fn set_variable(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }

    // Scope management helpers for anonymous functions
    pub fn push_scope(&mut self) {
        self.scope_stack.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        self.scope_stack.pop();
    }

    pub fn set_local_variable(&mut self, name: &str, value: Value) {
        if let Some(scope) = self.scope_stack.last_mut() {
            scope.insert(name.to_string(), value);
        }
    }

    // Helper function to call positional evaluation functions
    fn eval_positional_function(
        &mut self,
        name: &str,
        args: Vec<Expression>,
    ) -> Result<Value, String> {
        match name {
            "get" => self.eval_http_get(args),
            "post" => self.eval_http_post(args),
            "put" => self.eval_http_put(args),
            "delete" => self.eval_http_delete(args),
            "patch" => self.eval_http_patch(args),
            "http" => self.eval_http_request(args),
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
                            )),
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
            Statement::ForEach {
                variables,
                iterable,
                body,
            } => {
                let iterable_value = self.eval_expression(iterable)?;

                match iterable_value {
                    Value::List(items) => {
                        for (index, item) in items.iter().enumerate() {
                            // Create a new scope for loop variables
                            let saved_vars = variables
                                .iter()
                                .map(|var| (var.clone(), self.variables.get(var).cloned()))
                                .collect::<Vec<_>>();

                            // Set loop variables based on count
                            match variables.len() {
                                1 => {
                                    // for-each ~item in ~list
                                    self.variables.insert(variables[0].clone(), item.clone());
                                }
                                2 => {
                                    // for-each ~item ~index in ~list
                                    self.variables.insert(variables[0].clone(), item.clone());
                                    self.variables
                                        .insert(variables[1].clone(), Value::Number(index as f64));
                                }
                                _ => return Err("for-each expects 1 or 2 variables".to_string()),
                            }

                            // Execute loop body
                            for stmt in &body {
                                let (_, control) =
                                    self.eval_statement_with_control(stmt.clone())?;
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
                            let saved_vars = variables
                                .iter()
                                .map(|var| (var.clone(), self.variables.get(var).cloned()))
                                .collect::<Vec<_>>();

                            // Set loop variables based on count
                            match variables.len() {
                                1 => {
                                    // for-each ~value in ~object (values only)
                                    self.variables.insert(variables[0].clone(), value.clone());
                                }
                                2 => {
                                    // for-each ~key ~value in ~object
                                    self.variables
                                        .insert(variables[0].clone(), Value::String(key.clone()));
                                    self.variables.insert(variables[1].clone(), value.clone());
                                }
                                _ => return Err("for-each expects 1 or 2 variables".to_string()),
                            }

                            // Execute loop body
                            for stmt in &body {
                                let (_, control) =
                                    self.eval_statement_with_control(stmt.clone())?;
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
            }
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
                    _ => Err(format!(
                        "Cannot increment '{}': both variable and amount must be numbers",
                        variable
                    )),
                }
            }
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
                    _ => Err(format!(
                        "Cannot decrement '{}': both variable and amount must be numbers",
                        variable
                    )),
                }
            }
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
            Statement::AttemptRescue {
                attempt_body,
                rescue_var,
                rescue_body,
            } => self.eval_attempt_rescue(attempt_body, rescue_var, rescue_body),
            Statement::FunctionChain { variable, steps } => {
                let result = self.eval_function_chain(&steps)?;
                // Store result in variable like a regular assignment
                if let Some(scope) = self.scope_stack.last_mut() {
                    scope.insert(variable, result.clone());
                } else {
                    self.variables.insert(variable, result.clone());
                }
                Ok((result, ControlFlow::Continue))
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
        self.eval_expression_with_context(expr, false, None)
    }

    fn eval_expression_with_context(
        &mut self,
        expr: Expression,
        _in_tail_position: bool,
        _current_function: Option<&str>,
    ) -> Result<Value, String> {
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
                            (
                                Value::Number(l),
                                Value::Number(r),
                                BinaryOperator::LessThanOrEqual,
                            ) => Ok(Value::Boolean(l <= r)),
                            (Value::Number(l), Value::Number(r), BinaryOperator::GreaterThan) => {
                                Ok(Value::Boolean(l > r))
                            }
                            (
                                Value::Number(l),
                                Value::Number(r),
                                BinaryOperator::GreaterThanOrEqual,
                            ) => Ok(Value::Boolean(l >= r)),
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
                    "get" | "post" | "put" | "delete" | "patch" | "http" | "run" | "wait"
                    | "read" | "write" | "clear" => self.eval_positional_function(&name, args),
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
                        // Check for block syntax first (e.g., core:is-even)
                        if name.contains(':') {
                            // Extract the block name and function name
                            let parts: Vec<&str> = name.split(':').collect();
                            if parts.len() == 2 {
                                let block_name = parts[0];
                                let func_name = parts[1];

                                // Handle different blocks
                                match block_name {
                                    "core" => {
                                        // Force use of stdlib function, bypassing user definitions
                                        if let Some(func) =
                                            crate::stdlib::get_stdlib_function(func_name)
                                        {
                                            func(args, self)
                                        } else {
                                            Err(format!("Unknown core function: {}", func_name))
                                        }
                                    }
                                    _ => Err(format!("Unknown block: {}", block_name)),
                                }
                            } else {
                                Err(format!("Invalid block syntax: {}", name))
                            }
                        } else {
                            // Check if this is a user-defined function
                            let function = if let Some(function) = self.functions.get(&name) {
                                function.clone()
                            } else {
                                // Function not found, try stdlib
                                if let Some(func) = crate::stdlib::get_stdlib_function(&name) {
                                    return func(args, self);
                                } else {
                                    return Err(format!("Unknown function: {}", name));
                                }
                            };
                            self.eval_function_with_name(function, args, Some(name.clone()))
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
                            match found_value
                                .or_else(|| self.variables.get(var_name_without_tilde).cloned())
                            {
                                Some(val) => val,
                                None => {
                                    return Err(format!(
                                        "Undefined variable: ~{}",
                                        var_name_without_tilde
                                    ));
                                }
                            }
                        } else {
                            // Try to parse property as literal numeric index
                            match property.parse::<f64>() {
                                Ok(n) => Value::Number(n),
                                Err(_) => {
                                    return Err(format!(
                                        "Cannot access non-numeric property '{}' on list",
                                        property
                                    ));
                                }
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
                    Value::Error(err) => {
                        match property.as_str() {
                            "message" => Ok(Value::String(err.message.clone())),
                            "code" => match &err.code {
                                Some(code) => Ok(Value::String(code.clone())),
                                None => Ok(Value::Null),
                            },
                            "source" => match &err.source {
                                Some(source) => Ok(Value::String(source.clone())),
                                None => Ok(Value::Null),
                            },
                            "context" => Ok(Value::Object(err.context.clone())),
                            _ => Ok(Value::Null), // Return null for non-existent properties
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
            Expression::AnonymousFunction { .. } => {
                // Anonymous functions are not evaluated as standalone values
                // They are only evaluated in the context of map/filter/reduce
                Err("Anonymous functions can only be used as arguments to functions like map, filter, and reduce".to_string())
            }
            Expression::FunctionChainExpression { steps, input } => {
                // Evaluate the input expression first
                let mut result = self.eval_expression(*input)?;

                // Apply each function in the chain
                for step in steps {
                    result = self.eval_chain_step(&step, Some(result))?;
                }

                Ok(result)
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

    // HTTP GET request
    fn eval_http_get(&mut self, args: Vec<Expression>) -> Result<Value, String> {
        if args.is_empty() {
            return Err("get requires at least a URL argument".to_string());
        }

        let url_value = self.eval_expression(args[0].clone())?;
        let url = match url_value {
            Value::String(s) => s,
            _ => return Err("get URL must be a string".to_string()),
        };

        // Parse options (headers, timeout, auth, etc.)
        let options = if args.len() > 1 {
            Some(self.eval_expression(args[1].clone())?)
        } else {
            None
        };

        let (headers, _, timeout_ms, query_params) = parse_http_options(options)?;
        let final_url = crate::http::build_url_with_query(&url, query_params);
        let request = HttpRequest::new("GET", &final_url)
            .with_headers(headers)
            .with_timeout(timeout_ms);

        match HttpClient::execute(request) {
            Ok(response) => Ok(response.to_tails_value()),
            Err(error_value) => {
                self.last_error = Some(error_value.clone());
                Err(Self::error_value_to_string(&error_value))
            }
        }
    }

    // HTTP POST request
    fn eval_http_post(&mut self, args: Vec<Expression>) -> Result<Value, String> {
        if args.is_empty() {
            return Err("post requires at least a URL argument".to_string());
        }

        let url_value = self.eval_expression(args[0].clone())?;
        let url = match url_value {
            Value::String(s) => s,
            _ => return Err("post URL must be a string".to_string()),
        };

        // Parse options (headers, body, timeout, auth, etc.)
        let options = if args.len() > 1 {
            Some(self.eval_expression(args[1].clone())?)
        } else {
            None
        };

        let (headers, body, timeout_ms, query_params) = parse_http_options(options)?;
        let final_url = crate::http::build_url_with_query(&url, query_params);
        let mut request = HttpRequest::new("POST", &final_url)
            .with_headers(headers)
            .with_timeout(timeout_ms);

        if let Some(body_str) = body {
            request = request.with_body(body_str);
        }

        match HttpClient::execute(request) {
            Ok(response) => Ok(response.to_tails_value()),
            Err(error_value) => {
                self.last_error = Some(error_value.clone());
                Err(Self::error_value_to_string(&error_value))
            }
        }
    }

    // HTTP PUT request
    fn eval_http_put(&mut self, args: Vec<Expression>) -> Result<Value, String> {
        if args.is_empty() {
            return Err("put requires at least a URL argument".to_string());
        }

        let url_value = self.eval_expression(args[0].clone())?;
        let url = match url_value {
            Value::String(s) => s,
            _ => return Err("put URL must be a string".to_string()),
        };

        let options = if args.len() > 1 {
            Some(self.eval_expression(args[1].clone())?)
        } else {
            None
        };

        let (headers, body, timeout_ms, query_params) = parse_http_options(options)?;
        let final_url = crate::http::build_url_with_query(&url, query_params);
        let mut request = HttpRequest::new("PUT", &final_url)
            .with_headers(headers)
            .with_timeout(timeout_ms);

        if let Some(body_str) = body {
            request = request.with_body(body_str);
        }

        match HttpClient::execute(request) {
            Ok(response) => Ok(response.to_tails_value()),
            Err(error_value) => {
                self.last_error = Some(error_value.clone());
                Err(Self::error_value_to_string(&error_value))
            }
        }
    }

    // HTTP DELETE request
    fn eval_http_delete(&mut self, args: Vec<Expression>) -> Result<Value, String> {
        if args.is_empty() {
            return Err("delete requires at least a URL argument".to_string());
        }

        let url_value = self.eval_expression(args[0].clone())?;
        let url = match url_value {
            Value::String(s) => s,
            _ => return Err("delete URL must be a string".to_string()),
        };

        let options = if args.len() > 1 {
            Some(self.eval_expression(args[1].clone())?)
        } else {
            None
        };

        let (headers, _, timeout_ms, query_params) = parse_http_options(options)?;
        let final_url = crate::http::build_url_with_query(&url, query_params);
        let request = HttpRequest::new("DELETE", &final_url)
            .with_headers(headers)
            .with_timeout(timeout_ms);

        match HttpClient::execute(request) {
            Ok(response) => Ok(response.to_tails_value()),
            Err(error_value) => {
                self.last_error = Some(error_value.clone());
                Err(Self::error_value_to_string(&error_value))
            }
        }
    }

    // HTTP PATCH request
    fn eval_http_patch(&mut self, args: Vec<Expression>) -> Result<Value, String> {
        if args.is_empty() {
            return Err("patch requires at least a URL argument".to_string());
        }

        let url_value = self.eval_expression(args[0].clone())?;
        let url = match url_value {
            Value::String(s) => s,
            _ => return Err("patch URL must be a string".to_string()),
        };

        let options = if args.len() > 1 {
            Some(self.eval_expression(args[1].clone())?)
        } else {
            None
        };

        let (headers, body, timeout_ms, query_params) = parse_http_options(options)?;
        let final_url = crate::http::build_url_with_query(&url, query_params);
        let mut request = HttpRequest::new("PATCH", &final_url)
            .with_headers(headers)
            .with_timeout(timeout_ms);

        if let Some(body_str) = body {
            request = request.with_body(body_str);
        }

        match HttpClient::execute(request) {
            Ok(response) => Ok(response.to_tails_value()),
            Err(error_value) => {
                self.last_error = Some(error_value.clone());
                Err(Self::error_value_to_string(&error_value))
            }
        }
    }

    // Generic HTTP request
    fn eval_http_request(&mut self, args: Vec<Expression>) -> Result<Value, String> {
        if args.len() < 2 {
            return Err("http requires method and URL arguments".to_string());
        }

        let method_value = self.eval_expression(args[0].clone())?;
        let method = match method_value {
            Value::String(s) => s,
            _ => return Err("http method must be a string".to_string()),
        };

        let url_value = self.eval_expression(args[1].clone())?;
        let url = match url_value {
            Value::String(s) => s,
            _ => return Err("http URL must be a string".to_string()),
        };

        let options = if args.len() > 2 {
            Some(self.eval_expression(args[2].clone())?)
        } else {
            None
        };

        let (headers, body, timeout_ms, query_params) = parse_http_options(options)?;
        let final_url = crate::http::build_url_with_query(&url, query_params);
        let mut request = HttpRequest::new(&method, &final_url)
            .with_headers(headers)
            .with_timeout(timeout_ms);

        if let Some(body_str) = body {
            request = request.with_body(body_str);
        }

        match HttpClient::execute(request) {
            Ok(response) => Ok(response.to_tails_value()),
            Err(error_value) => {
                self.last_error = Some(error_value.clone());
                Err(Self::error_value_to_string(&error_value))
            }
        }
    }

    // Convert error value to string for Rust error propagation
    fn error_value_to_string(error_value: &Value) -> String {
        if let Value::Error(err) = error_value {
            err.message.clone()
        } else {
            "Unknown error".to_string()
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

    pub fn eval_function(
        &mut self,
        function: Function,
        args: Vec<Expression>,
    ) -> Result<Value, String> {
        self.eval_function_with_name(function, args, None)
    }

    fn eval_function_with_name(
        &mut self,
        function: Function,
        args: Vec<Expression>,
        function_name: Option<String>,
    ) -> Result<Value, String> {
        // Check call depth to prevent stack overflow
        if self.scope_stack.len() >= self.max_call_depth {
            return Err(format!(
                "Maximum call depth ({}) exceeded",
                self.max_call_depth
            ));
        }

        // Check argument count
        if args.len() != function.params.len() {
            return Err(format!(
                "Function expects {} arguments, but {} were provided",
                function.params.len(),
                args.len()
            ));
        }

        // Critical optimization: Pre-allocate and reuse argument vector
        let mut current_args = Vec::with_capacity(function.params.len());
        for arg in args {
            current_args.push(self.eval_expression(arg)?);
        }

        // Critical optimization: Pre-populate scope to avoid repeated HashMap operations
        let mut local_scope = HashMap::with_capacity(function.params.len());
        for (param, value) in function.params.iter().zip(current_args.iter()) {
            local_scope.insert(param.clone(), value.clone());
        }

        // Push the pre-populated scope
        self.scope_stack.push(local_scope);

        // Tail call optimization loop
        let mut found_tail_call = false;
        loop {
            // Hot path optimization: Only update scope on tail calls
            if found_tail_call {
                if let Some(scope) = self.scope_stack.last_mut() {
                    // Reuse existing keys, just update values to avoid allocations
                    for (param, value) in function.params.iter().zip(current_args.iter()) {
                        scope.insert(param.clone(), value.clone());
                    }
                }
                found_tail_call = false; // Reset for next iteration
            }

            // Execute function body
            let mut last_value = Value::Null;

            for stmt in &function.body {
                if std::env::var("TILDE_DEBUG_TAIL_CALL").is_ok() {
                    eprintln!("Executing statement: {:?}", stmt);
                }
                let (value, control) = self.eval_statement_with_control(stmt.clone())?;
                if std::env::var("TILDE_DEBUG_TAIL_CALL").is_ok() {
                    eprintln!("Statement result: value={:?}, control={:?}", value, control);
                }

                match control {
                    ControlFlow::Give(return_value) => {
                        // Check if this is a tail call to the same function
                        if let Some(ref fn_name) = function_name {
                            if std::env::var("TILDE_DEBUG_TAIL_CALL").is_ok() {
                                eprintln!("Checking statement for tail call: {:?}", stmt);
                            }
                            if let Some(tail_args) = self.detect_tail_call(&stmt, fn_name) {
                                // Evaluate new arguments for tail call
                                let mut new_args = Vec::new();
                                for arg_expr in tail_args {
                                    new_args.push(self.eval_expression(arg_expr)?);
                                }
                                current_args = new_args;
                                found_tail_call = true;
                                break; // Break inner loop to restart with new args
                            }
                        }

                        // Not a tail call, cache result and return
                        self.scope_stack.pop();

                        return Ok(return_value);
                    }
                    ControlFlow::BreakLoop => {
                        last_value = value;
                    }
                    ControlFlow::Continue => {
                        last_value = value;
                    }
                }
            }

            // If no tail call was found, cache result and exit the loop
            if !found_tail_call {
                self.scope_stack.pop();

                return Ok(last_value);
            }

            // Continue loop with new arguments (tail call optimization)
        }
    }

    /// Detects if a statement is a tail call to the specified function
    /// Returns the argument expressions if it's a tail call, None otherwise
    fn detect_tail_call(&self, stmt: &Statement, function_name: &str) -> Option<Vec<Expression>> {
        self.find_tail_call_in_statement(stmt, function_name)
    }

    /// Recursively search for tail calls in any statement structure
    fn find_tail_call_in_statement(
        &self,
        stmt: &Statement,
        function_name: &str,
    ) -> Option<Vec<Expression>> {
        match stmt {
            // Direct function call in give statement
            Statement::Give(Expression::FunctionCall { name, args }) => {
                if name == function_name {
                    if std::env::var("TILDE_DEBUG_TAIL_CALL").is_ok() {
                        eprintln!("TAIL CALL DETECTED: {} with {} args", name, args.len());
                    }
                    Some(args.clone())
                } else {
                    None
                }
            }
            // Check if/else branches
            Statement::If {
                condition: _,
                then_stmt,
                else_stmt,
            } => {
                if std::env::var("TILDE_DEBUG_TAIL_CALL").is_ok() {
                    eprintln!(
                        "Checking if statement: then={:?}, else={:?}",
                        then_stmt, else_stmt
                    );
                }
                // Check the then branch
                if let Some(tail_args) = self.find_tail_call_in_statement(then_stmt, function_name)
                {
                    return Some(tail_args);
                }
                // Check the else branch if it exists
                if let Some(else_branch) = else_stmt {
                    if let Some(tail_args) =
                        self.find_tail_call_in_statement(else_branch, function_name)
                    {
                        return Some(tail_args);
                    }
                }
                None
            }
            // Check block statements (look at the last statement)
            Statement::Block { body } => {
                if let Some(last_stmt) = body.last() {
                    self.find_tail_call_in_statement(last_stmt, function_name)
                } else {
                    None
                }
            }
            // Try/catch blocks
            Statement::AttemptRescue {
                attempt_body,
                rescue_var: _,
                rescue_body,
            } => {
                // Check the attempt body (last statement could be tail)
                if let Some(last_stmt) = attempt_body.last() {
                    if let Some(tail_args) =
                        self.find_tail_call_in_statement(last_stmt, function_name)
                    {
                        return Some(tail_args);
                    }
                }
                // Check the rescue body (last statement could be tail)
                if let Some(last_stmt) = rescue_body.last() {
                    if let Some(tail_args) =
                        self.find_tail_call_in_statement(last_stmt, function_name)
                    {
                        return Some(tail_args);
                    }
                }
                None
            }
            // For other statement types, no tail call possible
            _ => None,
        }
    }

    /// Create a hash key from function arguments for memoization

    fn eval_attempt_rescue(
        &mut self,
        attempt_body: Vec<Statement>,
        rescue_var: Option<String>,
        rescue_body: Vec<Statement>,
    ) -> EvalResult {
        // Try to execute the attempt block
        let mut last_value = Value::Null;
        for stmt in attempt_body {
            match self.eval_statement_with_control(stmt) {
                Ok((value, control)) => {
                    last_value = value;
                    match control {
                        ControlFlow::Continue => {}
                        // If we get BreakLoop or Give, propagate it
                        flow => return Ok((last_value, flow)),
                    }
                }
                Err(error_msg) => {
                    // Check if we have a structured error from HTTP or use simple error
                    let error_value = if let Some(structured_error) = self.last_error.take() {
                        structured_error
                    } else {
                        Value::Error(crate::value::ErrorValue::new(error_msg))
                    };

                    // If rescue variable is specified, bind the error to it
                    if let Some(var_name) = rescue_var {
                        self.set_variable(var_name, error_value);
                    }

                    // Execute rescue block
                    let mut rescue_value = Value::Null;
                    for rescue_stmt in rescue_body {
                        match self.eval_statement_with_control(rescue_stmt) {
                            Ok((value, control)) => {
                                rescue_value = value;
                                match control {
                                    ControlFlow::Continue => {}
                                    // If we get BreakLoop or Give in rescue, propagate it
                                    flow => return Ok((rescue_value, flow)),
                                }
                            }
                            Err(rescue_error) => {
                                // Error in rescue block still propagates
                                return Err(rescue_error);
                            }
                        }
                    }

                    return Ok((rescue_value, ControlFlow::Continue));
                }
            }
        }

        // No error occurred, return the last value from attempt block
        Ok((last_value, ControlFlow::Continue))
    }
    fn eval_function_chain(&mut self, steps: &[ChainStep]) -> Result<Value, String> {
        if steps.is_empty() {
            return Err("Function chain cannot be empty".to_string());
        }

        // Start with the result of the first step
        let mut current_value = self.eval_chain_step(&steps[0], None)?;

        // Pipe the result through each remaining step
        for step in steps.iter().skip(1) {
            current_value = self.eval_chain_step(step, Some(current_value))?;
        }

        Ok(current_value)
    }

    fn eval_chain_step(&mut self, step: &ChainStep, input_value: Option<Value>) -> Result<Value, String> {
        // Convert input value to expression if present
        let mut expr_args = Vec::new();
        if let Some(value) = input_value {
            let input_expr = match value {
                Value::Number(n) => Expression::Number(n, false),
                Value::String(s) => Expression::String(s),
                Value::Boolean(b) => Expression::Boolean(b),
                Value::List(items) => {
                    let expr_items: Vec<Expression> = items.into_iter().map(|item| match item {
                        Value::Number(n) => Expression::Number(n, false),
                        Value::String(s) => Expression::String(s),
                        Value::Boolean(b) => Expression::Boolean(b),
                        _ => Expression::String(item.to_string()),
                    }).collect();
                    Expression::List(expr_items)
                },
                _ => Expression::String(value.to_string()),
            };
            expr_args.push(input_expr);
        }

        // Add the step's explicit arguments (don't evaluate them here)
        for arg in &step.args {
            expr_args.push(arg.clone());
        }

        // Call the function directly with expressions
        let func_call = Expression::FunctionCall {
            name: step.function_name.clone(),
            args: expr_args,
        };
        self.eval_expression(func_call)
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
        let mut parser = Parser::new(
            "~user is {\"name\": \"Alice\" \"age\": 30}\n~message is \"User `~user.name` is `~user.age` years old\"",
        );
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
        let mut parser = Parser::new(
            "~data is {\"user\": {\"profile\": {\"name\": \"Bob\"}}}\n~text is \"Hello `~data.user.profile.name`!\"",
        );
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
        let mut parser = Parser::new(
            "~name is \"Charlie\"\n~user is {\"age\": 25}\n~greeting is \"Hi `~name`, you are `~user.age` years old\"",
        );
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        evaluator.eval_program(program).unwrap();

        assert_eq!(
            evaluator.get_variable("greeting"),
            Some(&Value::String(
                "Hi Charlie, you are 25 years old".to_string()
            ))
        );
    }

    #[test]
    fn test_interpolation_with_null_property() {
        let mut parser =
            Parser::new("~obj is {\"existing\": \"value\"}\n~text is \"Property: `~obj.missing`\"");
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
        let mut parser =
            Parser::new("~nums is [1, 2, 3]\n~mixed is [1, \"hello\", true]\n~empty is []");
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

        assert_eq!(evaluator.get_variable("empty"), Some(&Value::List(vec![])));
    }

    #[test]
    fn test_list_indexing() {
        let mut parser = Parser::new(
            "~nums is [10, 20, 30]\n~first is ~nums.0\n~second is ~nums.1\n~missing is ~nums.5",
        );
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        evaluator.eval_program(program).unwrap();

        assert_eq!(evaluator.get_variable("first"), Some(&Value::Number(10.0)));
        assert_eq!(evaluator.get_variable("second"), Some(&Value::Number(20.0)));
        assert_eq!(evaluator.get_variable("missing"), Some(&Value::Null));
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
        let mut parser = Parser::new(
            "~nums is [10, 20, 30]\n~len is length ~nums\n~str_len is length \"hello\"",
        );
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        evaluator.eval_program(program).unwrap();

        assert_eq!(evaluator.get_variable("len"), Some(&Value::Number(3.0)));
        assert_eq!(evaluator.get_variable("str_len"), Some(&Value::Number(5.0)));
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
            Some(&Value::List(vec![Value::Number(10.0), Value::Number(20.0)]))
        );
    }

    #[test]
    fn test_nested_lists() {
        let mut parser = Parser::new(
            "~nested is [[1, 2], [3, 4]]\n~first_row is ~nested.0\n~second_row is ~nested.1\n~element is ~second_row.0",
        );
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        evaluator.eval_program(program).unwrap();

        assert_eq!(
            evaluator.get_variable("first_row"),
            Some(&Value::List(vec![Value::Number(1.0), Value::Number(2.0)]))
        );

        assert_eq!(
            evaluator.get_variable("second_row"),
            Some(&Value::List(vec![Value::Number(3.0), Value::Number(4.0)]))
        );

        assert_eq!(evaluator.get_variable("element"), Some(&Value::Number(3.0)));
    }

    #[test]
    fn test_list_interpolation() {
        let mut parser =
            Parser::new("~nums is [10, 20, 30]\n~message is \"First: `~nums.0`, Last: `~nums.2`\"");
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
        assert!(
            result
                .unwrap_err()
                .contains("length can only be used on lists or strings")
        );
    }

    #[test]
    fn test_append_on_invalid_type() {
        let mut parser = Parser::new("~num is 42\n~result is append ~num \"item\"");
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        let result = evaluator.eval_program(program);

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .contains("append can only be used on lists")
        );
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
        assert!(
            result
                .unwrap_err()
                .contains("keys can only be used on objects")
        );
    }

    #[test]
    fn test_values_of_invalid_args() {
        let mut parser = Parser::new("~result is values 42");
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        let result = evaluator.eval_program(program);

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .contains("values can only be used on objects")
        );
    }

    #[test]
    fn test_has_key_invalid_args() {
        let mut parser = Parser::new("~result is has \"key\" 42");
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        let result = evaluator.eval_program(program);

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .contains("has can only be used on objects")
        );
    }

    #[test]
    fn test_interpolated_string_with_function_call() {
        let mut parser =
            Parser::new("~name is \"World\"\n~message is \"Hello `~name`!\"\n~result is ~message");
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
                assert_eq!(
                    nested.get("deep"),
                    Some(&Value::String("value".to_string()))
                );
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

        // Test core:is-even
        let mut parser = Parser::new("~result is core:is-even 4");
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        evaluator.eval_program(program).unwrap();

        assert_eq!(
            evaluator.get_variable("result"),
            Some(&Value::Boolean(true))
        );

        // Test core:is-odd
        let mut parser = Parser::new("~result is core:is-odd 5");
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        evaluator.eval_program(program).unwrap();

        assert_eq!(
            evaluator.get_variable("result"),
            Some(&Value::Boolean(true))
        );
    }

    #[test]
    fn test_block_syntax_priority_resolution() {
        use crate::parser::Parser;

        // Define a user function that conflicts with stdlib
        let mut parser = Parser::new(
            "function is-even ~x ( give false )\n~user_result is is-even 4\n~core_result is core:is-even 4",
        );
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        evaluator.eval_program(program).unwrap();

        // User function should be called for bare name
        assert_eq!(
            evaluator.get_variable("user_result"),
            Some(&Value::Boolean(false))
        );

        // Core function should be called for core: prefix
        assert_eq!(
            evaluator.get_variable("core_result"),
            Some(&Value::Boolean(true))
        );
    }


    #[test]
    fn test_block_syntax_unknown_core_function() {
        use crate::parser::Parser;

        let mut parser = Parser::new("~result is core:unknown-function 1");
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        let result = evaluator.eval_program(program);

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .contains("Unknown core function: unknown-function")
        );
    }

    #[test]
    fn test_block_syntax_in_map_function() {
        use crate::parser::Parser;

        // Test that core:is-even works as a function reference in map
        let mut parser = Parser::new("~nums is [1, 2, 3, 4]\n~results is map ~nums core:is-even");
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

        // Test that core: works with filter function
        let mut parser =
            Parser::new("~nums is [1, 2, 3, 4, 5]\n~evens is filter ~nums core:is-even");
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        evaluator.eval_program(program).unwrap();

        assert_eq!(
            evaluator.get_variable("evens"),
            Some(&Value::List(vec![Value::Number(2.0), Value::Number(4.0),]))
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
             ~core_doubled is map ~nums core:double",
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
             ~doubled is core:double ~num\n\
             ~squared is core:square ~doubled",
        );
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        evaluator.eval_program(program).unwrap();

        assert_eq!(
            evaluator.get_variable("doubled"),
            Some(&Value::Number(10.0))
        );
        assert_eq!(
            evaluator.get_variable("squared"),
            Some(&Value::Number(100.0))
        );
    }

    #[test]
    fn test_block_syntax_edge_cases() {
        use crate::parser::Parser;

        // Test edge cases like zero, negative numbers
        let mut parser = Parser::new(
            "~zero_even is core:is-even 0\n\
             ~neg_pos is core:is-positive -5\n\
             ~zero_zero is core:is-zero 0",
        );
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        evaluator.eval_program(program).unwrap();

        assert_eq!(
            evaluator.get_variable("zero_even"),
            Some(&Value::Boolean(true))
        );
        assert_eq!(
            evaluator.get_variable("neg_pos"),
            Some(&Value::Boolean(false))
        );
        assert_eq!(
            evaluator.get_variable("zero_zero"),
            Some(&Value::Boolean(true))
        );
    }

    #[test]
    fn test_block_syntax_library_pattern() {
        use crate::parser::Parser;

        // Test the library pattern mentioned in docs - using core: for predictable behavior
        // Simplified to single line to avoid multiline parsing complexities
        let mut parser = Parser::new("function validate-positive ~x ( give core:is-positive ~x )\n~result is *validate-positive 5");
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        evaluator.eval_program(program).unwrap();

        assert_eq!(
            evaluator.get_variable("result"),
            Some(&Value::Boolean(true))
        );
    }

    #[test]
    fn test_block_syntax_multiple_blocks_parsing() {
        use crate::parser::Parser;

        // Test that multiple block syntaxes in one line parse correctly
        let mut parser = Parser::new("~a is core:double 5\n~b is core:square 3");
        let program = parser.parse().unwrap();

        let mut evaluator = Evaluator::new();
        evaluator.eval_program(program).unwrap();

        assert_eq!(evaluator.get_variable("a"), Some(&Value::Number(10.0)));
        assert_eq!(evaluator.get_variable("b"), Some(&Value::Number(9.0)));
    }
}
