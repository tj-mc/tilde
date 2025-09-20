use super::Parser;
use crate::ast::*;
use crate::lexer::Token;

impl Parser {

    pub fn parse_statement(&mut self) -> Result<Statement, String> {
        match self.current_token() {
            Token::Variable(name) => {
                let var_name = name.clone();
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
                } else if *self.current_token() == Token::Dot {
                    // This might be a property access assignment: ~var.prop is value
                    let saved_position = self.position;

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
                            _ => return Err("Expected property name or number after '.'".to_string()),
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

        Ok(Statement::ForEach { variables, iterable, body })
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

        Ok(Statement::FunctionDefinition {
            name,
            params,
            body,
        })
    }
}
