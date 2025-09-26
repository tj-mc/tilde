use super::Parser;
use crate::ast::*;
use crate::lexer::Token;

impl Parser {
    pub fn parse_statement(&mut self) -> Result<Statement, String> {
        match self.current_token() {
            Token::Variable(name) => {
                let var_name = name.clone();
                let saved_position = self.position; // Save position BEFORE advancing
                self.advance();

                // Check for immediate assignment first
                if *self.current_token() == Token::Is {
                    // Simple variable assignment: ~var is value
                    self.advance();
                    let value = self.parse_expression()?;
                    Ok(Statement::Assignment {
                        variable: var_name,
                        value,
                    })
                } else if *self.current_token() == Token::Up {
                    // Increment: ~var up amount
                    self.advance();
                    let amount = self.parse_expression()?;
                    Ok(Statement::Increment {
                        variable: var_name,
                        amount,
                    })
                } else if *self.current_token() == Token::Down {
                    // Decrement: ~var down amount
                    self.advance();
                    let amount = self.parse_expression()?;
                    Ok(Statement::Decrement {
                        variable: var_name,
                        amount,
                    })
                } else if *self.current_token() == Token::Colon {
                    // Function chain: ~var: func1 arg; func2 arg
                    self.advance();
                    let steps = self.parse_chain_steps()?;
                    Ok(Statement::FunctionChain {
                        variable: var_name,
                        steps,
                    })
                } else if *self.current_token() == Token::Dot {
                    // This might be a property access assignment: ~var.prop is value

                    // Try to parse the property chain and check for assignment
                    let mut expr = Expression::Variable(var_name);
                    let _found_assignment = false;

                    while *self.current_token() == Token::Dot {
                        self.advance();
                        let property = match self.current_token() {
                            Token::Identifier(prop_name) => prop_name.clone(),
                            Token::Variable(prop_name) => prop_name.clone(),
                            Token::Number(n, _) => {
                                // Convert number to integer string if it's a whole number
                                if n.fract() == 0.0 {
                                    (*n as i64).to_string()
                                } else {
                                    n.to_string()
                                }
                            }
                            Token::Boolean(b) => b.to_string(),
                            _ => {
                                return Err(
                                    "Expected property name or number after '.'".to_string()
                                );
                            }
                        };
                        self.advance();

                        expr = Expression::PropertyAccess {
                            object: Box::new(expr),
                            property: property.clone(),
                        };

                        // Check if this is where the assignment happens
                        if *self.current_token() == Token::Is {
                            self.advance();
                            let value = self.parse_expression()?;

                            // Extract the base object and property for assignment
                            if let Expression::PropertyAccess { object, property } = expr {
                                return Ok(Statement::PropertyAssignment {
                                    object,
                                    property,
                                    value,
                                });
                            } else {
                                return Err("Invalid property assignment structure".to_string());
                            }
                        }
                    }

                    // If we got here, it wasn't a property assignment, reset and treat as expression
                    self.position = saved_position;
                    let expr = self.parse_expression()?;
                    Ok(Statement::Expression(expr))
                } else {
                    // Reset position and parse as expression
                    self.position -= 1; // Go back to the variable token
                    let expr = self.parse_expression()?;
                    Ok(Statement::Expression(expr))
                }
            }
            Token::If => self.parse_if(),
            Token::Loop => self.parse_loop(),
            Token::ForEach => self.parse_for_each(),
            Token::Breakloop => {
                self.advance();
                Ok(Statement::Breakloop)
            }
            Token::Open => {
                self.advance();
                let path = self.parse_expression()?;
                Ok(Statement::Open(path))
            }
            Token::LeftParen => self.parse_block(),
            Token::Function => self.parse_function_definition(),
            Token::Give => {
                self.advance();
                let expr = self.parse_expression()?;
                Ok(Statement::Give(expr))
            }
            Token::Attempt => self.parse_attempt_rescue(),
            _ => {
                let expr = self.parse_expression()?;
                Ok(Statement::Expression(expr))
            }
        }
    }

    pub fn parse_if(&mut self) -> Result<Statement, String> {
        self.expect(Token::If)?;

        let condition = self.parse_expression()?;
        let then_stmt = Box::new(self.parse_statement()?);

        let else_stmt = if *self.current_token() == Token::Else {
            self.advance();
            Some(Box::new(self.parse_statement()?))
        } else {
            None
        };

        Ok(Statement::If {
            condition,
            then_stmt,
            else_stmt,
        })
    }

    pub fn parse_loop(&mut self) -> Result<Statement, String> {
        self.expect(Token::Loop)?;
        self.expect(Token::LeftParen)?;
        self.skip_newlines();

        let mut body = Vec::new();

        while *self.current_token() != Token::RightParen && *self.current_token() != Token::Eof {
            self.skip_newlines();
            if *self.current_token() == Token::RightParen {
                break;
            }
            body.push(self.parse_statement()?);
            self.skip_newlines();
        }

        self.expect(Token::RightParen)?;

        Ok(Statement::Loop { body })
    }

    pub fn parse_for_each(&mut self) -> Result<Statement, String> {
        self.expect(Token::ForEach)?;

        // Parse variables (~item or ~item ~index)
        let mut variables = Vec::new();

        // First variable (required)
        if let Token::Variable(var) = self.current_token() {
            variables.push(var.clone());
            self.advance();
        } else {
            return Err("Expected variable after 'for-each'".to_string());
        }

        // Check for optional second variable
        if let Token::Variable(var) = self.current_token() {
            variables.push(var.clone());
            self.advance();
        }

        // Check for a third variable (which is an error)
        if let Token::Variable(_) = self.current_token() {
            return Err("for-each expects at most 2 variables".to_string());
        }

        // Expect 'in' keyword
        self.expect(Token::In)?;

        // Parse the iterable expression
        let iterable = self.parse_expression()?;

        // Parse body
        self.expect(Token::LeftParen)?;
        self.skip_newlines();

        let mut body = Vec::new();

        while *self.current_token() != Token::RightParen && *self.current_token() != Token::Eof {
            self.skip_newlines();
            if *self.current_token() == Token::RightParen {
                break;
            }
            body.push(self.parse_statement()?);
            self.skip_newlines();
        }

        self.expect(Token::RightParen)?;

        Ok(Statement::ForEach {
            variables,
            iterable,
            body,
        })
    }

    pub fn parse_block(&mut self) -> Result<Statement, String> {
        self.expect(Token::LeftParen)?;
        self.skip_newlines();

        let mut body = Vec::new();

        while *self.current_token() != Token::RightParen && *self.current_token() != Token::Eof {
            self.skip_newlines();
            if *self.current_token() == Token::RightParen {
                break;
            }
            body.push(self.parse_statement()?);
            self.skip_newlines();
        }

        self.expect(Token::RightParen)?;

        Ok(Statement::Block { body })
    }

    pub fn parse_function_definition(&mut self) -> Result<Statement, String> {
        self.expect(Token::Function)?;

        // Parse function name
        let name = match self.current_token() {
            Token::Identifier(n) => n.clone(),
            _ => return Err("Expected function name after 'function'".to_string()),
        };
        self.advance();

        // Parse parameters (space-separated variables with ~ prefix)
        let mut params = Vec::new();
        while let Token::Variable(param_name) = self.current_token() {
            params.push(param_name.clone());
            self.advance();
        }

        // Parse body block
        self.expect(Token::LeftParen)?;
        self.skip_newlines();

        let mut body = Vec::new();
        while *self.current_token() != Token::RightParen && *self.current_token() != Token::Eof {
            self.skip_newlines();
            if *self.current_token() == Token::RightParen {
                break;
            }
            body.push(self.parse_statement()?);
            self.skip_newlines();
        }

        self.expect(Token::RightParen)?;

        Ok(Statement::FunctionDefinition { name, params, body })
    }

    pub fn parse_attempt_rescue(&mut self) -> Result<Statement, String> {
        self.expect(Token::Attempt)?;
        self.expect(Token::LeftParen)?;
        self.skip_newlines();

        // Parse attempt body
        let mut attempt_body = Vec::new();
        while *self.current_token() != Token::RightParen && *self.current_token() != Token::Eof {
            self.skip_newlines();
            if *self.current_token() == Token::RightParen {
                break;
            }
            attempt_body.push(self.parse_statement()?);
            self.skip_newlines();
        }

        self.expect(Token::RightParen)?;
        self.expect(Token::Rescue)?;

        // Check for optional rescue variable
        let rescue_var = if let Token::Variable(var_name) = self.current_token() {
            let var = var_name.clone();
            self.advance();
            Some(var)
        } else {
            None
        };

        self.expect(Token::LeftParen)?;
        self.skip_newlines();

        // Parse rescue body
        let mut rescue_body = Vec::new();
        while *self.current_token() != Token::RightParen && *self.current_token() != Token::Eof {
            self.skip_newlines();
            if *self.current_token() == Token::RightParen {
                break;
            }
            rescue_body.push(self.parse_statement()?);
            self.skip_newlines();
        }

        self.expect(Token::RightParen)?;

        Ok(Statement::AttemptRescue {
            attempt_body,
            rescue_var,
            rescue_body,
        })
    }

    pub fn parse_chain_steps(&mut self) -> Result<Vec<ChainStep>, String> {
        let mut steps = Vec::new();

        // Skip initial newlines
        self.skip_newlines();

        while *self.current_token() != Token::Eof && !self.is_at_chain_boundary() {
            // Check if this is a bare variable (first step initial value)
            if let Token::Variable(var_name) = self.current_token() {
                let var_name = var_name.clone();
                self.advance();

                // If this is the first step and there are no arguments, treat it as initial value
                if steps.is_empty()
                    && (*self.current_token() == Token::Newline
                        || *self.current_token() == Token::Eof
                        || self.is_at_chain_boundary())
                {
                    // This is a bare variable as the initial value - create a special "identity" step
                    steps.push(ChainStep {
                        function_name: "@@initial-value".to_string(),
                        args: vec![Expression::Variable(var_name)],
                    });
                } else {
                    return Err(
                        "Variables can only appear as the first step in a chain".to_string()
                    );
                }
            } else {
                // Each line should be a function call with arguments
                let function_name = match self.current_token() {
                    Token::Identifier(name) => {
                        let name = name.clone();
                        self.advance();
                        name
                    }
                    Token::Block(block_name) => {
                        // Handle block syntax like core:is-even
                        let block_name = block_name.clone();
                        self.advance();

                        // Expect function name after block
                        if let Token::Identifier(func_name) = self.current_token() {
                            let func_name = func_name.clone();
                            self.advance();
                            format!("{}:{}", block_name, func_name)
                        } else {
                            return Err(format!("Expected function name after {}:", block_name));
                        }
                    }
                    _ => {
                        // If we don't see an identifier or block, we're done with the chain
                        break;
                    }
                };

                // Parse all arguments on the same line until newline
                let mut args = Vec::new();
                while *self.current_token() != Token::Newline
                    && *self.current_token() != Token::Eof
                    && !self.is_at_chain_boundary()
                {
                    args.push(self.parse_expression()?);
                }

                steps.push(ChainStep {
                    function_name,
                    args,
                });
            }

            // Skip to next line
            if *self.current_token() == Token::Newline {
                self.advance();
                // Skip any additional whitespace/newlines before next step
                self.skip_newlines();
            }
        }

        if steps.is_empty() {
            return Err("Function chain cannot be empty".to_string());
        }

        Ok(steps)
    }

    fn is_at_chain_boundary(&self) -> bool {
        match self.current_token() {
            // Only variables that start with ~ and are followed by : are chain boundaries
            Token::Variable(_) => {
                // Look ahead to see if there's a colon
                if self.position + 1 < self.tokens.len() {
                    matches!(self.tokens[self.position + 1], Token::Colon)
                } else {
                    false
                }
            }
            Token::If => true,        // Next if statement
            Token::Loop => true,      // Next loop
            Token::ForEach => true,   // Next for-each
            Token::Function => true,  // Next function definition
            Token::Attempt => true,   // Next attempt block
            Token::Give => true,      // Next give statement
            Token::Open => true,      // Next open statement
            Token::LeftParen => true, // Next block
            _ => false,
        }
    }
}
