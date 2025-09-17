use crate::ast::*;
use crate::lexer::{Lexer, Token};

mod expressions;
mod literals;
mod statements;

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();

        Parser {
            tokens,
            position: 0,
        }
    }

    pub fn new_from_tokens(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }

    pub(crate) fn current_token(&self) -> &Token {
        self.tokens.get(self.position).unwrap_or(&Token::Eof)
    }

    #[allow(dead_code)]
    pub(crate) fn peek_token(&self) -> &Token {
        self.tokens.get(self.position + 1).unwrap_or(&Token::Eof)
    }

    pub(crate) fn advance(&mut self) {
        if self.position < self.tokens.len() {
            self.position += 1;
        }
    }

    pub(crate) fn expect(&mut self, expected: Token) -> Result<(), String> {
        if *self.current_token() == expected {
            self.advance();
            Ok(())
        } else {
            Err(format!(
                "Expected {:?}, got {:?}",
                expected,
                self.current_token()
            ))
        }
    }

    pub(crate) fn skip_newlines(&mut self) {
        while *self.current_token() == Token::Newline {
            self.advance();
        }
    }

    pub(crate) fn is_binary_operator(&self, token: &Token) -> bool {
        matches!(
            token,
            Token::Plus
                | Token::Minus
                | Token::Multiply
                | Token::Divide
                | Token::IntegerDivide
                | Token::Modulo
                | Token::LessThan
                | Token::LessThanOrEqual
                | Token::GreaterThan
                | Token::GreaterThanOrEqual
                | Token::Equal
                | Token::NotEqual
        )
    }

    pub fn parse(&mut self) -> Result<Program, String> {
        let mut statements = Vec::new();

        while *self.current_token() != Token::Eof {
            self.skip_newlines();
            if *self.current_token() == Token::Eof {
                break;
            }

            let stmt = self.parse_statement()?;
            statements.push(stmt);
            self.skip_newlines();
        }

        Ok(statements)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_assignment() {
        let mut parser = Parser::new("~x is 42");
        let program = parser.parse().unwrap();

        assert_eq!(program.len(), 1);
        match &program[0] {
            Statement::Assignment { variable, value } => {
                assert_eq!(variable, "x");
                assert_eq!(*value, Expression::Number(42.0));
            }
            _ => panic!("Expected assignment statement"),
        }
    }

    #[test]
    fn test_parse_expression() {
        let mut parser = Parser::new("~result is ~x + ~y");
        let program = parser.parse().unwrap();

        assert_eq!(program.len(), 1);
        match &program[0] {
            Statement::Assignment { variable, value } => {
                assert_eq!(variable, "result");
                match value {
                    Expression::BinaryOp { left, op, right } => {
                        assert_eq!(**left, Expression::Variable("x".to_string()));
                        assert_eq!(*op, BinaryOperator::Add);
                        assert_eq!(**right, Expression::Variable("y".to_string()));
                    }
                    _ => panic!("Expected binary operation"),
                }
            }
            _ => panic!("Expected assignment statement"),
        }
    }

    #[test]
    fn test_parse_say_multiple_args() {
        let mut parser = Parser::new("say \"Hello\" ~name");
        let program = parser.parse().unwrap();

        assert_eq!(program.len(), 1);
        match &program[0] {
            Statement::Say(args) => {
                assert_eq!(args.len(), 2);
                assert_eq!(args[0], Expression::String("Hello".to_string()));
                assert_eq!(args[1], Expression::Variable("name".to_string()));
            }
            _ => panic!("Expected say statement"),
        }
    }

    #[test]
    fn test_parse_say_single_arg() {
        let mut parser = Parser::new("say \"Hello, World!\"");
        let program = parser.parse().unwrap();

        assert_eq!(program.len(), 1);
        match &program[0] {
            Statement::Say(args) => {
                assert_eq!(args.len(), 1);
                assert_eq!(args[0], Expression::String("Hello, World!".to_string()));
            }
            _ => panic!("Expected say statement"),
        }
    }

    #[test]
    fn test_parse_if_block() {
        let mut parser = Parser::new("if ~x > 5 say \"big\"");
        let program = parser.parse().unwrap();

        assert_eq!(program.len(), 1);
        match &program[0] {
            Statement::If {
                condition,
                then_stmt,
                else_stmt,
            } => {
                match condition {
                    Expression::BinaryOp { left, op, right } => {
                        assert_eq!(**left, Expression::Variable("x".to_string()));
                        assert_eq!(*op, BinaryOperator::GreaterThan);
                        assert_eq!(**right, Expression::Number(5.0));
                    }
                    _ => panic!("Expected comparison in condition"),
                }

                match then_stmt.as_ref() {
                    Statement::Say(args) => {
                        assert_eq!(args.len(), 1);
                        assert_eq!(args[0], Expression::String("big".to_string()));
                    }
                    _ => panic!("Expected say statement in then clause"),
                }

                assert!(else_stmt.is_none());
            }
            _ => panic!("Expected if statement"),
        }
    }

    #[test]
    fn test_parse_multiplication() {
        let mut parser = Parser::new("~result is ~x * 3");
        let program = parser.parse().unwrap();

        match &program[0] {
            Statement::Assignment { value, .. } => match value {
                Expression::BinaryOp { left, op, right } => {
                    assert_eq!(**left, Expression::Variable("x".to_string()));
                    assert_eq!(*op, BinaryOperator::Multiply);
                    assert_eq!(**right, Expression::Number(3.0));
                }
                _ => panic!("Expected multiplication"),
            },
            _ => panic!("Expected assignment"),
        }
    }

    #[test]
    fn test_parse_division() {
        let mut parser = Parser::new("~result is ~x / 2");
        let program = parser.parse().unwrap();

        match &program[0] {
            Statement::Assignment { value, .. } => match value {
                Expression::BinaryOp { left, op, right } => {
                    assert_eq!(**left, Expression::Variable("x".to_string()));
                    assert_eq!(*op, BinaryOperator::Divide);
                    assert_eq!(**right, Expression::Number(2.0));
                }
                _ => panic!("Expected division"),
            },
            _ => panic!("Expected assignment"),
        }
    }

    #[test]
    fn test_parse_operator_precedence() {
        let mut parser = Parser::new("~result is ~x + ~y * 2");
        let program = parser.parse().unwrap();

        match &program[0] {
            Statement::Assignment { value, .. } => match value {
                Expression::BinaryOp {
                    left,
                    op: BinaryOperator::Add,
                    right,
                } => {
                    assert_eq!(**left, Expression::Variable("x".to_string()));
                    match right.as_ref() {
                        Expression::BinaryOp {
                            left: mult_left,
                            op: BinaryOperator::Multiply,
                            right: mult_right,
                        } => {
                            assert_eq!(**mult_left, Expression::Variable("y".to_string()));
                            assert_eq!(**mult_right, Expression::Number(2.0));
                        }
                        _ => panic!("Expected multiplication with higher precedence"),
                    }
                }
                _ => panic!("Expected addition at top level"),
            },
            _ => panic!("Expected assignment"),
        }
    }

    #[test]
    fn test_parse_property_assignment() {
        let mut parser = Parser::new("~obj.name is \"John\"");
        let program = parser.parse().unwrap();

        match &program[0] {
            Statement::PropertyAssignment {
                object,
                property,
                value,
            } => {
                assert_eq!(**object, Expression::Variable("obj".to_string()));
                assert_eq!(property, "name");
                assert_eq!(*value, Expression::String("John".to_string()));
            }
            _ => panic!("Expected property assignment"),
        }
    }

    #[test]
    fn test_parse_object_literal() {
        let mut parser = Parser::new("~obj is {name: \"John\", age: 30}");
        let program = parser.parse().unwrap();

        match &program[0] {
            Statement::Assignment { value, .. } => match value {
                Expression::ObjectLiteral { pairs } => {
                    assert_eq!(pairs.len(), 2);
                    assert_eq!(pairs[0].0, "name");
                    assert_eq!(pairs[0].1, Expression::String("John".to_string()));
                    assert_eq!(pairs[1].0, "age");
                    assert_eq!(pairs[1].1, Expression::Number(30.0));
                }
                _ => panic!("Expected object literal"),
            },
            _ => panic!("Expected assignment"),
        }
    }

    #[test]
    fn test_parse_object_functions() {
        let mut parser = Parser::new("~keys is keys-of(~obj)");
        let program = parser.parse().unwrap();

        match &program[0] {
            Statement::Assignment { value, .. } => match value {
                Expression::FunctionCall { name, args } => {
                    assert_eq!(name, "keys-of");
                    assert_eq!(args.len(), 1);
                    assert_eq!(args[0], Expression::Variable("obj".to_string()));
                }
                _ => panic!("Expected function call"),
            },
            _ => panic!("Expected assignment"),
        }
    }

    #[test]
    fn test_parse_get_statement() {
        let mut parser = Parser::new("get \"http://example.com\"");
        let program = parser.parse().unwrap();

        match &program[0] {
            Statement::Get(args) => {
                assert_eq!(args.len(), 1);
                assert_eq!(
                    args[0],
                    Expression::String("http://example.com".to_string())
                );
            }
            _ => panic!("Expected get statement"),
        }
    }

    #[test]
    fn test_parse_get_function_call() {
        let mut parser = Parser::new("~response is get \"http://example.com\"");
        let program = parser.parse().unwrap();

        match &program[0] {
            Statement::Assignment { value, .. } => {
                match value {
                    Expression::FunctionCall { name, args: _ } => {
                        assert_eq!(name, "get");
                        // The args should be parsed as named arguments within the function call
                        // This is a simplified test - actual implementation may vary
                    }
                    _ => panic!("Expected function call"),
                }
            }
            _ => panic!("Expected assignment"),
        }
    }

    #[test]
    fn test_parse_get_with_timeout() {
        let mut parser = Parser::new("get \"http://example.com\" 5000");
        let program = parser.parse().unwrap();

        match &program[0] {
            Statement::Get(args) => {
                assert_eq!(args.len(), 2);
                assert_eq!(
                    args[0],
                    Expression::String("http://example.com".to_string())
                );
                assert_eq!(args[1], Expression::Number(5000.0));
            }
            _ => panic!("Expected get statement"),
        }
    }

    #[test]
    fn test_parse_wait_statement() {
        let mut parser = Parser::new("wait 2");
        let program = parser.parse().unwrap();

        match &program[0] {
            Statement::Wait(args) => {
                assert_eq!(args.len(), 1);
                assert_eq!(args[0], Expression::Number(2.0));
            }
            _ => panic!("Expected wait statement"),
        }
    }

    #[test]
    fn test_parse_wait_function_call() {
        let mut parser = Parser::new("~result is wait 3");
        let program = parser.parse().unwrap();

        match &program[0] {
            Statement::Assignment { value, .. } => {
                match value {
                    Expression::FunctionCall { name, args: _ } => {
                        assert_eq!(name, "wait");
                        // Similar to get function call test
                    }
                    _ => panic!("Expected function call"),
                }
            }
            _ => panic!("Expected assignment"),
        }
    }

    #[test]
    fn test_parse_ask_with_prompt() {
        let mut parser = Parser::new("~name is ask \"What's your name?\"");
        let program = parser.parse().unwrap();

        match &program[0] {
            Statement::Assignment { value, .. } => match value {
                Expression::FunctionCall { name, args } => {
                    assert_eq!(name, "ask");
                    assert_eq!(args.len(), 1);
                    assert_eq!(args[0], Expression::String("What's your name?".to_string()));
                }
                _ => panic!("Expected ask function call"),
            },
            _ => panic!("Expected assignment"),
        }
    }

    #[test]
    fn test_parse_ask_no_prompt() {
        let mut parser = Parser::new("~input is ask");
        let program = parser.parse().unwrap();

        match &program[0] {
            Statement::Assignment { value, .. } => match value {
                Expression::FunctionCall { name, args } => {
                    assert_eq!(name, "ask");
                    assert_eq!(args.len(), 0);
                }
                _ => panic!("Expected ask function call"),
            },
            _ => panic!("Expected assignment"),
        }
    }

    #[test]
    fn test_parse_block_statement() {
        let mut parser = Parser::new("(\n    say \"hello\"\n    say \"world\"\n)");
        let program = parser.parse().unwrap();

        assert_eq!(program.len(), 1);
        match &program[0] {
            Statement::Block { body } => {
                assert_eq!(body.len(), 2);
                match &body[0] {
                    Statement::Say(args) => {
                        assert_eq!(args.len(), 1);
                        assert_eq!(args[0], Expression::String("hello".to_string()));
                    }
                    _ => panic!("Expected say statement"),
                }
                match &body[1] {
                    Statement::Say(args) => {
                        assert_eq!(args.len(), 1);
                        assert_eq!(args[0], Expression::String("world".to_string()));
                    }
                    _ => panic!("Expected say statement"),
                }
            }
            _ => panic!("Expected block statement"),
        }
    }

    #[test]
    fn test_parse_if_with_block() {
        let mut parser = Parser::new("if ~x > 5 (\n    say \"big\"\n    say \"number\"\n)");
        let program = parser.parse().unwrap();

        assert_eq!(program.len(), 1);
        match &program[0] {
            Statement::If {
                condition,
                then_stmt,
                else_stmt,
            } => {
                match condition {
                    Expression::BinaryOp { left, op, right } => {
                        assert_eq!(**left, Expression::Variable("x".to_string()));
                        assert_eq!(*op, BinaryOperator::GreaterThan);
                        assert_eq!(**right, Expression::Number(5.0));
                    }
                    _ => panic!("Expected comparison in condition"),
                }

                match then_stmt.as_ref() {
                    Statement::Block { body } => {
                        assert_eq!(body.len(), 2);
                        match &body[0] {
                            Statement::Say(args) => {
                                assert_eq!(args.len(), 1);
                                assert_eq!(args[0], Expression::String("big".to_string()));
                            }
                            _ => panic!("Expected say statement"),
                        }
                        match &body[1] {
                            Statement::Say(args) => {
                                assert_eq!(args.len(), 1);
                                assert_eq!(args[0], Expression::String("number".to_string()));
                            }
                            _ => panic!("Expected say statement"),
                        }
                    }
                    _ => panic!("Expected block statement in then clause"),
                }

                assert!(else_stmt.is_none());
            }
            _ => panic!("Expected if statement"),
        }
    }

    #[test]
    fn test_parse_nested_blocks() {
        let mut parser =
            Parser::new("(\n    say \"outer\"\n    (\n        say \"inner\"\n    )\n)");
        let program = parser.parse().unwrap();

        assert_eq!(program.len(), 1);
        match &program[0] {
            Statement::Block { body } => {
                assert_eq!(body.len(), 2);
                match &body[0] {
                    Statement::Say(args) => {
                        assert_eq!(args.len(), 1);
                        assert_eq!(args[0], Expression::String("outer".to_string()));
                    }
                    _ => panic!("Expected say statement"),
                }
                match &body[1] {
                    Statement::Block { body: inner_body } => {
                        assert_eq!(inner_body.len(), 1);
                        match &inner_body[0] {
                            Statement::Say(args) => {
                                assert_eq!(args.len(), 1);
                                assert_eq!(args[0], Expression::String("inner".to_string()));
                            }
                            _ => panic!("Expected say statement"),
                        }
                    }
                    _ => panic!("Expected block statement"),
                }
            }
            _ => panic!("Expected block statement"),
        }
    }

    #[test]
    fn test_parse_run_statement() {
        let mut parser = Parser::new("run \"echo hello\"");
        let stmt = parser.parse_statement().unwrap();

        match stmt {
            Statement::Run(args) => {
                assert_eq!(args.len(), 1);
                assert_eq!(args[0], Expression::String("echo hello".to_string()));
            }
            _ => panic!("Expected run statement"),
        }
    }

    #[test]
    fn test_parse_run_assignment() {
        let mut parser = Parser::new("~result is run \"echo hello\"");
        let stmt = parser.parse_statement().unwrap();

        match stmt {
            Statement::Assignment { variable, value } => {
                assert_eq!(variable, "result");
                match value {
                    Expression::FunctionCall { name, args } => {
                        assert_eq!(name, "run");
                        assert_eq!(args.len(), 1);
                        assert_eq!(args[0], Expression::String("echo hello".to_string()));
                    }
                    _ => panic!("Expected run function call"),
                }
            }
            _ => panic!("Expected assignment"),
        }
    }

    #[test]
    fn test_parse_interpolated_string() {
        let mut parser = Parser::new("~greeting is \"Hello `~name`!\"");
        let stmt = parser.parse_statement().unwrap();

        match stmt {
            Statement::Assignment { variable, value } => {
                assert_eq!(variable, "greeting");
                match value {
                    Expression::InterpolatedString(parts) => {
                        assert_eq!(parts.len(), 3);
                        assert_eq!(parts[0], InterpolationPart::Text("Hello ".to_string()));
                        assert_eq!(parts[1], InterpolationPart::Variable("name".to_string()));
                        assert_eq!(parts[2], InterpolationPart::Text("!".to_string()));
                    }
                    _ => panic!("Expected interpolated string"),
                }
            }
            _ => panic!("Expected assignment"),
        }
    }
}
