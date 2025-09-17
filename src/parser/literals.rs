use super::Parser;
use crate::ast::*;
use crate::lexer::Token;

impl Parser {
    pub fn parse_object_literal(&mut self) -> Result<Expression, String> {
        self.expect(Token::LeftBrace)?;
        self.skip_newlines();

        let mut pairs = Vec::new();

        while *self.current_token() != Token::RightBrace && *self.current_token() != Token::Eof {
            self.skip_newlines();

            if *self.current_token() == Token::RightBrace {
                break;
            }

            // Parse key
            let key = match self.current_token().clone() {
                Token::String(s) => s,
                Token::Identifier(s) => s,
                _ => return Err("Expected string or identifier key in object literal".to_string()),
            };
            self.advance();

            self.expect(Token::Colon)?;
            self.skip_newlines();

            // Parse value
            let value = self.parse_expression()?;
            pairs.push((key, value));

            self.skip_newlines();

            // Handle comma or end - be more tolerant
            if *self.current_token() == Token::Comma {
                self.advance();
                self.skip_newlines();
            } else if *self.current_token() == Token::RightBrace {
                // End of object - this is fine
            } else {
                // Allow missing commas between object properties
                self.skip_newlines();
            }
        }

        self.expect(Token::RightBrace)?;

        Ok(Expression::ObjectLiteral { pairs })
    }

    pub fn parse_list_literal(&mut self) -> Result<Expression, String> {
        self.expect(Token::LeftBracket)?;
        self.skip_newlines();

        let mut items = Vec::new();

        while *self.current_token() != Token::RightBracket && *self.current_token() != Token::Eof {
            self.skip_newlines();

            if *self.current_token() == Token::RightBracket {
                break;
            }

            // Parse list item
            let item = self.parse_expression()?;
            items.push(item);

            self.skip_newlines();

            // Handle comma or end - be more tolerant
            if *self.current_token() == Token::Comma {
                self.advance();
                self.skip_newlines();
            } else if *self.current_token() == Token::RightBracket {
                // End of list - this is fine
            } else {
                // Allow missing commas between list items (more lenient parsing)
                self.skip_newlines();
            }
        }

        self.expect(Token::RightBracket)?;

        Ok(Expression::List(items))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    fn parse_expression(input: &str) -> Result<Expression, String> {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new_from_tokens(tokens);
        parser.parse_expression()
    }

    #[test]
    fn test_empty_object_literal() {
        let result = parse_expression("{}").unwrap();
        match result {
            Expression::ObjectLiteral { pairs } => {
                assert!(pairs.is_empty());
            }
            _ => panic!("Expected object literal"),
        }
    }

    #[test]
    fn test_object_literal_with_string_keys() {
        let result = parse_expression(r#"{"name": "Alice", "age": 30}"#).unwrap();
        match result {
            Expression::ObjectLiteral { pairs } => {
                assert_eq!(pairs.len(), 2);
                assert_eq!(pairs[0].0, "name");
                assert_eq!(pairs[1].0, "age");
            }
            _ => panic!("Expected object literal"),
        }
    }

    #[test]
    fn test_object_literal_with_identifier_keys() {
        let result = parse_expression("{name: \"Alice\", age: 30}").unwrap();
        match result {
            Expression::ObjectLiteral { pairs } => {
                assert_eq!(pairs.len(), 2);
                assert_eq!(pairs[0].0, "name");
                assert_eq!(pairs[1].0, "age");
            }
            _ => panic!("Expected object literal"),
        }
    }

    #[test]
    fn test_object_literal_invalid_key() {
        let result = parse_expression("{123: \"value\"}");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Expected string or identifier key"));
    }

    #[test]
    fn test_object_literal_missing_colon() {
        let result = parse_expression("{name \"Alice\"}");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Expected Colon"));
    }

    #[test]
    fn test_object_literal_unclosed() {
        let result = parse_expression("{name: \"Alice\"");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Expected RightBrace"));
    }

    #[test]
    fn test_empty_list_literal() {
        let result = parse_expression("[]").unwrap();
        match result {
            Expression::List(items) => {
                assert!(items.is_empty());
            }
            _ => panic!("Expected list literal"),
        }
    }

    #[test]
    fn test_list_literal_with_items() {
        let result = parse_expression("[1, \"hello\", true]").unwrap();
        match result {
            Expression::List(items) => {
                assert_eq!(items.len(), 3);
            }
            _ => panic!("Expected list literal"),
        }
    }

    #[test]
    fn test_list_literal_unclosed() {
        let result = parse_expression("[1, 2, 3");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Expected RightBracket"));
    }

    #[test]
    fn test_nested_object_in_list() {
        let result = parse_expression("[{name: \"Alice\"}, {name: \"Bob\"}]").unwrap();
        match result {
            Expression::List(items) => {
                assert_eq!(items.len(), 2);
                for item in items {
                    match item {
                        Expression::ObjectLiteral { .. } => {},
                        _ => panic!("Expected object in list"),
                    }
                }
            }
            _ => panic!("Expected list literal"),
        }
    }

    #[test]
    fn test_nested_list_in_object() {
        let result = parse_expression("{items: [1, 2, 3]}").unwrap();
        match result {
            Expression::ObjectLiteral { pairs } => {
                assert_eq!(pairs.len(), 1);
                assert_eq!(pairs[0].0, "items");
                match &pairs[0].1 {
                    Expression::List(items) => {
                        assert_eq!(items.len(), 3);
                    }
                    _ => panic!("Expected list in object"),
                }
            }
            _ => panic!("Expected object literal"),
        }
    }
}
