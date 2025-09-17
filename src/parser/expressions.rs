use super::Parser;
use crate::ast::*;
use crate::lexer::Token;

impl Parser {
    pub fn parse_expression(&mut self) -> Result<Expression, String> {
        self.parse_or()
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
            Token::Number(n) => {
                self.advance();
                Ok(Expression::Number(n))
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
            Token::Variable(name) => {
                self.advance();
                let mut expr = Expression::Variable(name);

                // Handle property access chain
                while *self.current_token() == Token::Dot {
                    self.advance();
                    let property = match self.current_token() {
                        Token::Identifier(prop_name) => prop_name.clone(),
                        Token::Variable(prop_name) => format!("~{}", prop_name),
                        Token::Number(n) => {
                            // Convert number to integer string if it's a whole number
                            if n.fract() == 0.0 {
                                (*n as i64).to_string()
                            } else {
                                n.to_string()
                            }
                        }
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
            Token::Identifier(name) => {
                self.advance();

                // Check if this is an action call (*name)
                if name.starts_with('*') {
                    // Parse action call arguments (space-separated until terminator)
                    let mut args = Vec::new();

                    // Parse arguments until we hit a terminator
                    while !matches!(
                        self.current_token(),
                        Token::Eof
                            | Token::Newline
                            | Token::If
                            | Token::Else
                            | Token::Loop
                            | Token::Breakloop
                            | Token::Say
                            | Token::Ask
                            | Token::Open
                            | Token::Get
                            | Token::Run
                            | Token::Wait
                            | Token::Give
                            | Token::Action
                            | Token::RightParen
                            | Token::Is
                    ) && !self.is_binary_operator(self.current_token()) {
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
                            args.push(self.parse_primary()?);
                        }
                    }

                    Ok(Expression::FunctionCall {
                        name: name[1..].to_string(), // Remove the * prefix
                        args,
                    })
                } else if *self.current_token() == Token::LeftParen {
                    // Regular function call with parentheses
                    self.advance();
                    let mut args = Vec::new();

                    // Parse arguments
                    while *self.current_token() != Token::RightParen {
                        if !args.is_empty() {
                            self.expect(Token::Comma)?;
                        }
                        args.push(self.parse_expression()?);
                    }

                    self.expect(Token::RightParen)?;

                    Ok(Expression::FunctionCall { name, args })
                } else {
                    // Check if this is a built-in function that can take arguments without parentheses
                    let builtin_functions = ["length", "append", "keys-of", "values-of", "has-key"];

                    if builtin_functions.contains(&name.as_str()) {
                        // Parse arguments like action calls (space-separated until terminator)
                        let mut args = Vec::new();

                        // Parse arguments until we hit a terminator
                        while !matches!(
                            self.current_token(),
                            Token::Eof
                                | Token::Newline
                                | Token::If
                                | Token::Else
                                | Token::Loop
                                | Token::Breakloop
                                | Token::Say
                                | Token::Ask
                                | Token::Open
                                | Token::Get
                                | Token::Run
                                | Token::Wait
                                | Token::Give
                                | Token::Action
                                | Token::LeftParen
                                | Token::RightParen
                                | Token::Is
                        ) && !self.is_binary_operator(self.current_token()) {
                            args.push(self.parse_primary()?);
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
            Token::Say => {
                self.advance();
                let mut args = Vec::new();

                // Parse all arguments until we hit a statement terminator
                while !matches!(
                    self.current_token(),
                    Token::Eof
                        | Token::Newline
                        | Token::If
                        | Token::Else
                        | Token::Loop
                        | Token::Breakloop
                        | Token::Say
                        | Token::Ask
                        | Token::Open
                        | Token::Get
                        | Token::Run
                        | Token::Wait
                        | Token::RightParen
                        | Token::Comma
                ) {
                    args.push(self.parse_expression()?);
                }

                Ok(Expression::FunctionCall {
                    name: "say".to_string(),
                    args,
                })
            }
            Token::Ask => {
                self.advance();
                let mut args = Vec::new();

                // Parse all arguments until we hit a statement terminator
                while !matches!(
                    self.current_token(),
                    Token::Eof
                        | Token::Newline
                        | Token::If
                        | Token::Else
                        | Token::Loop
                        | Token::Breakloop
                        | Token::Say
                        | Token::Ask
                        | Token::Open
                        | Token::Get
                        | Token::Run
                        | Token::Wait
                        | Token::RightParen
                        | Token::Comma
                ) {
                    args.push(self.parse_expression()?);
                }

                Ok(Expression::FunctionCall {
                    name: "ask".to_string(),
                    args,
                })
            }
            Token::Get => {
                self.advance();
                let mut args = Vec::new();

                // Parse all arguments until we hit a statement terminator
                while !matches!(
                    self.current_token(),
                    Token::Eof
                        | Token::Newline
                        | Token::If
                        | Token::Else
                        | Token::Loop
                        | Token::Breakloop
                        | Token::Say
                        | Token::Ask
                        | Token::Open
                        | Token::Get
                        | Token::Run
                        | Token::Wait
                        | Token::RightParen
                        | Token::Comma
                ) {
                    args.push(self.parse_expression()?);
                }

                Ok(Expression::FunctionCall {
                    name: "get".to_string(),
                    args,
                })
            }
            Token::Run => {
                self.advance();
                let mut args = Vec::new();

                // Parse all arguments until we hit a statement terminator
                while !matches!(
                    self.current_token(),
                    Token::Eof
                        | Token::Newline
                        | Token::If
                        | Token::Else
                        | Token::Loop
                        | Token::Breakloop
                        | Token::Say
                        | Token::Ask
                        | Token::Open
                        | Token::Get
                        | Token::Run
                        | Token::Wait
                        | Token::RightParen
                        | Token::Comma
                ) {
                    args.push(self.parse_expression()?);
                }

                Ok(Expression::FunctionCall {
                    name: "run".to_string(),
                    args,
                })
            }
            Token::Wait => {
                self.advance();
                let mut args = Vec::new();

                // Parse all arguments until we hit a statement terminator
                while !matches!(
                    self.current_token(),
                    Token::Eof
                        | Token::Newline
                        | Token::If
                        | Token::Else
                        | Token::Loop
                        | Token::Breakloop
                        | Token::Say
                        | Token::Ask
                        | Token::Open
                        | Token::Get
                        | Token::Run
                        | Token::Wait
                        | Token::RightParen
                        | Token::Comma
                ) {
                    args.push(self.parse_expression()?);
                }

                Ok(Expression::FunctionCall {
                    name: "wait".to_string(),
                    args,
                })
            }
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
                        name: "keys-of".to_string(),
                        args,
                    })
                } else {
                    let mut args = Vec::new();

                    // Parse all arguments until we hit a statement terminator
                    while !matches!(
                        self.current_token(),
                        Token::Eof
                            | Token::Newline
                            | Token::If
                            | Token::Else
                            | Token::Loop
                            | Token::Breakloop
                            | Token::Say
                            | Token::Ask
                            | Token::Open
                            | Token::Get
                            | Token::Run
                            | Token::Wait
                            | Token::RightParen
                            | Token::Comma
                    ) {
                        args.push(self.parse_expression()?);
                    }

                    Ok(Expression::FunctionCall {
                        name: "keys-of".to_string(),
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
                        name: "values-of".to_string(),
                        args,
                    })
                } else {
                    let mut args = Vec::new();

                    // Parse all arguments until we hit a statement terminator
                    while !matches!(
                        self.current_token(),
                        Token::Eof
                            | Token::Newline
                            | Token::If
                            | Token::Else
                            | Token::Loop
                            | Token::Breakloop
                            | Token::Say
                            | Token::Ask
                            | Token::Open
                            | Token::Get
                            | Token::Run
                            | Token::Wait
                            | Token::RightParen
                            | Token::Comma
                    ) {
                        args.push(self.parse_expression()?);
                    }

                    Ok(Expression::FunctionCall {
                        name: "values-of".to_string(),
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
                        name: "has-key".to_string(),
                        args,
                    })
                } else {
                    let mut args = Vec::new();

                    // Parse all arguments until we hit a statement terminator
                    while !matches!(
                        self.current_token(),
                        Token::Eof
                            | Token::Newline
                            | Token::If
                            | Token::Else
                            | Token::Loop
                            | Token::Breakloop
                            | Token::Say
                            | Token::Ask
                            | Token::Open
                            | Token::Get
                            | Token::Run
                            | Token::Wait
                            | Token::RightParen
                            | Token::Comma
                    ) {
                        args.push(self.parse_expression()?);
                    }

                    Ok(Expression::FunctionCall {
                        name: "has-key".to_string(),
                        args,
                    })
                }
            }
            Token::Length => {
                self.advance();
                let mut args = Vec::new();

                // Parse all arguments until we hit a statement terminator
                while !matches!(
                    self.current_token(),
                    Token::Eof
                        | Token::Newline
                        | Token::If
                        | Token::Else
                        | Token::Loop
                        | Token::Breakloop
                        | Token::Say
                        | Token::Ask
                        | Token::Open
                        | Token::Get
                        | Token::Run
                        | Token::Wait
                        | Token::RightParen
                        | Token::Comma
                ) {
                    args.push(self.parse_expression()?);
                }

                Ok(Expression::FunctionCall {
                    name: "length".to_string(),
                    args,
                })
            }
            Token::Append => {
                self.advance();
                let mut args = Vec::new();

                // Parse all arguments until we hit a statement terminator
                while !matches!(
                    self.current_token(),
                    Token::Eof
                        | Token::Newline
                        | Token::If
                        | Token::Else
                        | Token::Loop
                        | Token::Breakloop
                        | Token::Say
                        | Token::Ask
                        | Token::Open
                        | Token::Get
                        | Token::Run
                        | Token::Wait
                        | Token::RightParen
                        | Token::Comma
                ) {
                    args.push(self.parse_expression()?);
                }

                Ok(Expression::FunctionCall {
                    name: "append".to_string(),
                    args,
                })
            }
            Token::LeftParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect(Token::RightParen)?;
                Ok(expr)
            }
            Token::LeftBrace => self.parse_object_literal(),
            Token::LeftBracket => self.parse_list_literal(),
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
}
