use super::Parser;
use crate::ast::*;
use crate::lexer::Token;

impl Parser {
    pub fn parse_expression(&mut self) -> Result<Expression, String> {
        self.parse_or()
    }

    // Helper function to parse simple function calls with arguments until terminator
    fn parse_simple_function_call(&mut self, function_name: &str) -> Result<Expression, String> {
        self.advance();
        let mut args = Vec::new();

        // Parse all arguments until we hit a statement terminator
        while !self.is_expression_terminator() {
            args.push(self.parse_expression()?);
        }

        Ok(Expression::FunctionCall {
            name: function_name.to_string(),
            args,
        })
    }

    pub fn parse_or(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_and()?;

        while matches!(self.current_token(), Token::Or) {
            let op = BinaryOperator::Or;
            self.advance();
            let right = self.parse_and()?;
            expr = Expression::BinaryOp {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    pub fn parse_and(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_comparison()?;

        while matches!(self.current_token(), Token::And) {
            let op = BinaryOperator::And;
            self.advance();
            let right = self.parse_comparison()?;
            expr = Expression::BinaryOp {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    pub fn parse_comparison(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_addition()?;

        while matches!(
            self.current_token(),
            Token::LessThan
                | Token::LessThanOrEqual
                | Token::GreaterThan
                | Token::GreaterThanOrEqual
                | Token::Equal
                | Token::NotEqual
        ) {
            let op = match self.current_token() {
                Token::LessThan => BinaryOperator::LessThan,
                Token::LessThanOrEqual => BinaryOperator::LessThanOrEqual,
                Token::GreaterThan => BinaryOperator::GreaterThan,
                Token::GreaterThanOrEqual => BinaryOperator::GreaterThanOrEqual,
                Token::Equal => BinaryOperator::Equal,
                Token::NotEqual => BinaryOperator::NotEqual,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_addition()?;
            expr = Expression::BinaryOp {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    pub fn parse_addition(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_multiplication()?;

        while matches!(self.current_token(), Token::Plus | Token::Minus) {
            let op = match self.current_token() {
                Token::Plus => BinaryOperator::Add,
                Token::Minus => BinaryOperator::Subtract,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_multiplication()?;
            expr = Expression::BinaryOp {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    pub fn parse_multiplication(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_primary()?;

        while matches!(
            self.current_token(),
            Token::Multiply | Token::Divide | Token::IntegerDivide | Token::Modulo
        ) {
            let op = match self.current_token() {
                Token::Multiply => BinaryOperator::Multiply,
                Token::Divide => BinaryOperator::Divide,
                Token::IntegerDivide => BinaryOperator::IntegerDivide,
                Token::Modulo => BinaryOperator::Modulo,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_primary()?;
            expr = Expression::BinaryOp {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    pub fn parse_primary(&mut self) -> Result<Expression, String> {
        match self.current_token().clone() {
            Token::Number(n, was_float) => {
                self.advance();
                Ok(Expression::Number(n, was_float))
            }
            Token::String(s) => {
                self.advance();
                Ok(Expression::String(s))
            }
            Token::Pipe => self.parse_anonymous_function(),
            Token::InterpolatedString(parts) => {
                self.advance();
                Ok(Expression::InterpolatedString(parts))
            }
            Token::Boolean(b) => {
                self.advance();
                Ok(Expression::Boolean(b))
            }
            Token::Variable(name) => {
                self.advance();
                let mut expr = Expression::Variable(name);

                // Handle property access chain
                while *self.current_token() == Token::Dot {
                    self.advance();
                    let property = match self.current_token() {
                        Token::Identifier(prop_name) => prop_name.clone(),
                        Token::Variable(prop_name) => format!("~{}", prop_name),
                        Token::Number(n, _) => {
                            // Convert number to integer string if it's a whole number
                            if n.fract() == 0.0 {
                                (*n as i64).to_string()
                            } else {
                                n.to_string()
                            }
                        }
                        Token::Boolean(b) => b.to_string(),
                        _ => return Err("Expected property name or number after '.'".to_string()),
                    };
                    self.advance();

                    expr = Expression::PropertyAccess {
                        object: Box::new(expr),
                        property,
                    };
                }

                Ok(expr)
            }
            Token::Block(block_name) => {
                self.advance();

                // Expect a function name after the block
                if let Token::Identifier(func_name) = self.current_token() {
                    let full_name = format!("{}:{}", block_name, func_name);
                    self.advance();

                    // Parse arguments like any stdlib function
                    let mut args = Vec::new();
                    while !self.is_expression_terminator()
                        && !matches!(self.current_token(), Token::Is)
                        && !self.is_binary_operator(self.current_token())
                    {
                        // Check if we're at a parenthesized expression
                        if *self.current_token() == Token::LeftParen {
                            let arg = self.parse_primary()?;
                            args.push(arg);

                            // If next token is an operator, we should stop parsing args
                            if self.is_binary_operator(self.current_token()) {
                                break;
                            }
                        } else {
                            args.push(self.parse_function_argument()?);
                        }
                    }

                    Ok(Expression::FunctionCall {
                        name: full_name,
                        args,
                    })
                } else {
                    Err(format!("Expected function name after {}:", block_name))
                }
            }
            Token::Identifier(name) => {
                self.advance();

                // Check if this is an function call (*name)
                if let Some(stripped) = name.strip_prefix('*') {
                    // Parse function call arguments (space-separated until terminator)
                    let mut args = Vec::new();

                    // Parse arguments until we hit a terminator
                    while !self.is_expression_terminator()
                        && !matches!(self.current_token(), Token::Is)
                        && !self.is_binary_operator(self.current_token())
                    {
                        // Check if we're at a parenthesized expression that might be part of a calculation
                        if *self.current_token() == Token::LeftParen {
                            // Save position to potentially backtrack
                            let _saved_pos = self.position;
                            let arg = self.parse_primary()?;
                            args.push(arg);

                            // If next token is an operator, we should stop parsing args
                            if self.is_binary_operator(self.current_token()) {
                                break;
                            }
                        } else {
                            // For function calls, parse arguments one by one without property access
                            args.push(self.parse_function_argument()?);
                        }
                    }

                    Ok(Expression::FunctionCall {
                        name: stripped.to_string(), // Remove the * prefix
                        args,
                    })
                } else {
                    // Check if this is a built-in function that can take arguments without parentheses
                    let is_builtin = [].contains(&name.as_str()); // All functions are now in stdlib
                    let is_stdlib =
                        crate::stdlib::get_stdlib_function_names().contains(&name.as_str());

                    if is_builtin || is_stdlib {
                        // Parse arguments like function calls (space-separated until terminator)
                        let mut args = Vec::new();

                        // Parse arguments until we hit a terminator
                        while !self.is_expression_terminator()
                            && !matches!(self.current_token(), Token::Is)
                            && !self.is_binary_operator(self.current_token())
                        {
                            // Check if we're at a parenthesized expression
                            if *self.current_token() == Token::LeftParen {
                                let arg = self.parse_primary()?;
                                args.push(arg);

                                // If next token is an operator, we should stop parsing args
                                if self.is_binary_operator(self.current_token()) {
                                    break;
                                }
                            } else {
                                args.push(self.parse_function_argument()?);
                            }
                        }

                        Ok(Expression::FunctionCall { name, args })
                    } else {
                        // Regular identifier with no arguments
                        Ok(Expression::FunctionCall {
                            name,
                            args: Vec::new(),
                        })
                    }
                }
            }
            Token::Say => self.parse_simple_function_call("say"),
            Token::Ask => self.parse_simple_function_call("ask"),
            Token::Get => self.parse_simple_function_call("get"),
            Token::Post => self.parse_simple_function_call("post"),
            Token::Put => self.parse_simple_function_call("put"),
            Token::Delete => self.parse_simple_function_call("delete"),
            Token::Patch => self.parse_simple_function_call("patch"),
            Token::Http => self.parse_simple_function_call("http"),
            Token::Run => self.parse_simple_function_call("run"),
            Token::Wait => self.parse_simple_function_call("wait"),
            Token::Random => self.parse_simple_function_call("random"),
            Token::Read => self.parse_simple_function_call("read"),
            Token::Write => self.parse_simple_function_call("write"),
            Token::Clear => self.parse_simple_function_call("clear"),
            Token::KeysOf => {
                self.advance();
                if *self.current_token() == Token::LeftParen {
                    self.advance();
                    let mut args = Vec::new();
                    while *self.current_token() != Token::RightParen {
                        if !args.is_empty() {
                            self.expect(Token::Comma)?;
                        }
                        args.push(self.parse_expression()?);
                    }
                    self.expect(Token::RightParen)?;
                    Ok(Expression::FunctionCall {
                        name: "keys".to_string(),
                        args,
                    })
                } else {
                    let mut args = Vec::new();

                    // Parse all arguments until we hit a statement terminator
                    while !self.is_expression_terminator() {
                        args.push(self.parse_expression()?);
                    }

                    Ok(Expression::FunctionCall {
                        name: "keys".to_string(),
                        args,
                    })
                }
            }
            Token::ValuesOf => {
                self.advance();
                if *self.current_token() == Token::LeftParen {
                    self.advance();
                    let mut args = Vec::new();
                    while *self.current_token() != Token::RightParen {
                        if !args.is_empty() {
                            self.expect(Token::Comma)?;
                        }
                        args.push(self.parse_expression()?);
                    }
                    self.expect(Token::RightParen)?;
                    Ok(Expression::FunctionCall {
                        name: "values".to_string(),
                        args,
                    })
                } else {
                    let mut args = Vec::new();

                    // Parse all arguments until we hit a statement terminator
                    while !self.is_expression_terminator() {
                        args.push(self.parse_expression()?);
                    }

                    Ok(Expression::FunctionCall {
                        name: "values".to_string(),
                        args,
                    })
                }
            }
            Token::HasKey => {
                self.advance();
                if *self.current_token() == Token::LeftParen {
                    self.advance();
                    let mut args = Vec::new();
                    while *self.current_token() != Token::RightParen {
                        if !args.is_empty() {
                            self.expect(Token::Comma)?;
                        }
                        args.push(self.parse_expression()?);
                    }
                    self.expect(Token::RightParen)?;
                    Ok(Expression::FunctionCall {
                        name: "has".to_string(),
                        args,
                    })
                } else {
                    let mut args = Vec::new();

                    // Parse all arguments until we hit a statement terminator
                    while !self.is_expression_terminator() {
                        args.push(self.parse_expression()?);
                    }

                    Ok(Expression::FunctionCall {
                        name: "has".to_string(),
                        args,
                    })
                }
            }
            Token::LeftParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect(Token::RightParen)?;
                Ok(expr)
            }
            Token::LeftBrace => self.parse_object_literal(),
            Token::LeftBracket => self.parse_list_literal(),
            Token::Dot => {
                // Parse dot-prefixed stdlib function reference like .is-even
                self.advance();
                match self.current_token().clone() {
                    Token::Identifier(name) => {
                        self.advance();
                        Ok(Expression::Variable(format!(".{}", name)))
                    }
                    _ => Err(
                        "Expected identifier after '.' for stdlib function reference".to_string(),
                    ),
                }
            }
            _ => Err(format!("Unexpected token: {:?}", self.current_token())),
        }
    }

    pub fn parse_named_args(&mut self) -> Result<Vec<(String, Expression)>, String> {
        let mut args = Vec::new();

        while !matches!(
            self.current_token(),
            Token::Eof | Token::Newline | Token::RightParen
        ) {
            if !args.is_empty() {
                self.expect(Token::Comma)?;
            }

            // Parse the key
            let key = match self.current_token() {
                Token::Identifier(name) => name.clone(),
                _ => return Err("Expected identifier for named argument key".to_string()),
            };
            self.advance();

            self.expect(Token::Colon)?;

            // Parse the value
            let value = self.parse_expression()?;
            args.push((key, value));
        }

        Ok(args)
    }

    /// Parse a single function argument without property access chains
    /// This prevents ~var .prop from being parsed as property access
    fn parse_function_argument(&mut self) -> Result<Expression, String> {
        match self.current_token().clone() {
            Token::Number(n, was_float) => {
                self.advance();
                Ok(Expression::Number(n, was_float))
            }
            Token::String(s) => {
                self.advance();
                Ok(Expression::String(s))
            }
            Token::InterpolatedString(parts) => {
                self.advance();
                Ok(Expression::InterpolatedString(parts))
            }
            Token::Boolean(b) => {
                self.advance();
                Ok(Expression::Boolean(b))
            }
            Token::Pipe => {
                // Parse anonymous function
                self.parse_anonymous_function()
            }
            Token::Variable(name) => {
                self.advance();
                let mut expr = Expression::Variable(name);

                // Handle property access chain for variables
                while *self.current_token() == Token::Dot {
                    self.advance();
                    let property = match self.current_token() {
                        Token::Identifier(prop_name) => prop_name.clone(),
                        Token::Variable(prop_name) => format!("~{}", prop_name),
                        Token::Number(n, _) => {
                            // Convert number to integer string if it's a whole number
                            if n.fract() == 0.0 {
                                (*n as i64).to_string()
                            } else {
                                n.to_string()
                            }
                        }
                        Token::Boolean(b) => b.to_string(),
                        _ => return Err("Expected property name or number after '.'".to_string()),
                    };
                    self.advance();

                    expr = Expression::PropertyAccess {
                        object: Box::new(expr),
                        property,
                    };
                }

                Ok(expr)
            }
            Token::Block(block_name) => {
                self.advance();

                // Expect a function name after the block
                if let Token::Identifier(func_name) = self.current_token() {
                    let full_name = format!("{}:{}", block_name, func_name);
                    self.advance();

                    // For function arguments, return a function reference (no arguments)
                    Ok(Expression::FunctionCall {
                        name: full_name,
                        args: Vec::new(),
                    })
                } else {
                    Err(format!("Expected function name after {}:", block_name))
                }
            }
            Token::Identifier(name) => {
                self.advance();
                // Don't allow function calls in function arguments - just identifiers
                Ok(Expression::FunctionCall {
                    name,
                    args: Vec::new(),
                })
            }
            Token::Dot => {
                // Parse dot-prefixed stdlib function reference like .is-even
                self.advance();
                match self.current_token().clone() {
                    Token::Identifier(name) => {
                        self.advance();
                        Ok(Expression::Variable(format!(".{}", name)))
                    }
                    _ => Err(
                        "Expected identifier after '.' for stdlib function reference".to_string(),
                    ),
                }
            }
            Token::LeftParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect(Token::RightParen)?;
                Ok(expr)
            }
            Token::LeftBrace => self.parse_object_literal(),
            Token::LeftBracket => self.parse_list_literal(),
            _ => Err(format!(
                "Unexpected token in function argument: {:?}",
                self.current_token()
            )),
        }
    }

    /// Parse an anonymous function: |~param1 ~param2 (body)|
    fn parse_anonymous_function(&mut self) -> Result<Expression, String> {
        // Expect opening pipe
        self.expect(Token::Pipe)?;

        // Parse parameters
        let mut params = Vec::new();

        // Parameters are variables before the opening parenthesis
        while *self.current_token() != Token::LeftParen {
            match self.current_token() {
                Token::Variable(name) => {
                    params.push(name.clone());
                    self.advance();
                }
                _ => {
                    return Err(format!(
                        "Expected parameter variable in anonymous function, got: {:?}",
                        self.current_token()
                    ));
                }
            }
        }

        // Must have at least one parameter
        if params.is_empty() {
            return Err("Anonymous function must have at least one parameter".to_string());
        }

        // Parse body between parentheses
        self.expect(Token::LeftParen)?;
        let body = Box::new(self.parse_expression()?);
        self.expect(Token::RightParen)?;

        // Expect closing pipe
        self.expect(Token::Pipe)?;

        Ok(Expression::AnonymousFunction { params, body })
    }
}
