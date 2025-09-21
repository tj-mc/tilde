use crate::ast::{Expression, InterpolationPart};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literals
    Number(f64, bool), // value, was_float_literal
    String(String),
    InterpolatedString(Vec<InterpolationPart>),
    Boolean(bool),
    Variable(String),
    Identifier(String),
    Block(String), // For :block_name: syntax

    // Keywords
    Is,
    If,
    Else,
    Loop,
    ForEach,
    Breakloop,
    Say,
    Ask,
    Open,
    Get,
    Run,
    Scrape,
    Wait,
    In,
    Otherwise,
    List,
    KeysOf,
    ValuesOf,
    HasKey,
    Function,
    Give,
    And,
    Or,
    Random,
    Read,
    Write,
    Clear,
    Up,
    Down,
    Attempt,
    Rescue,

    // Operators
    Plus,
    Minus,
    Multiply,
    Divide,
    IntegerDivide,
    Modulo,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Equal,
    NotEqual,

    // Delimiters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Colon,
    Comma,
    Dot,

    // End of line/input
    Newline,
    Eof,
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    current_char: Option<char>,
    after_block: bool,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let chars: Vec<char> = input.chars().collect();
        let current_char = if chars.is_empty() {
            None
        } else {
            Some(chars[0])
        };

        Lexer {
            input: chars,
            position: 0,
            current_char,
            after_block: false,
        }
    }

    fn advance(&mut self) {
        self.position += 1;
        self.current_char = if self.position < self.input.len() {
            Some(self.input[self.position])
        } else {
            None
        };
    }

    #[allow(dead_code)]
    fn peek(&self) -> Option<char> {
        if self.position + 1 < self.input.len() {
            Some(self.input[self.position + 1])
        } else {
            None
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char {
            if ch == ' ' || ch == '\t' || ch == '\r' {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn read_number(&mut self) -> (f64, bool) {
        let mut num_str = String::new();
        let mut has_decimal = false;

        while let Some(ch) = self.current_char {
            if ch.is_ascii_digit() {
                num_str.push(ch);
                self.advance();
            } else if ch == '.' && !has_decimal {
                has_decimal = true;
                num_str.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        let value = num_str.parse().unwrap_or(0.0);
        (value, has_decimal)
    }

    // Helper function to handle escape sequences
    fn handle_escape_sequence(&mut self, string: &mut String) {
        // Handle escape sequences
        self.advance();
        if let Some(escaped_char) = self.current_char {
            match escaped_char {
                'n' => string.push('\n'),
                't' => string.push('\t'),
                'r' => string.push('\r'),
                '\\' => string.push('\\'),
                '"' => string.push('"'),
                '\'' => string.push('\''),
                _ => {
                    // For unknown escape sequences, keep the backslash
                    string.push('\\');
                    string.push(escaped_char);
                }
            }
            self.advance();
        } else {
            string.push('\\');
        }
    }

    fn read_string(&mut self) -> String {
        let quote_char = self.current_char.unwrap();
        self.advance();

        let mut string = String::new();
        while let Some(ch) = self.current_char {
            if ch == quote_char {
                self.advance();
                break;
            }

            if ch == '\\' {
                self.handle_escape_sequence(&mut string);
            } else {
                string.push(ch);
                self.advance();
            }
        }

        string
    }

    fn read_interpolated_string(&mut self) -> Vec<InterpolationPart> {
        let quote_char = self.current_char.unwrap();
        self.advance();

        let mut parts = Vec::new();
        let mut current_text = String::new();

        while let Some(ch) = self.current_char {
            if ch == quote_char {
                self.advance();
                break;
            }

            if ch == '`' {
                // Save any accumulated text
                if !current_text.is_empty() {
                    parts.push(InterpolationPart::Text(current_text.clone()));
                    current_text.clear();
                }

                // Skip the opening backtick
                self.advance();

                // Expect a tilde for variable
                if self.current_char == Some('~') {
                    self.advance(); // Skip the ~
                    let var_name = self.read_identifier();

                    // Check for property access
                    if self.current_char == Some('.') {
                        let mut properties = vec![var_name];
                        while self.current_char == Some('.') {
                            self.advance(); // Skip the '.'
                            let prop_name = self.read_identifier();
                            properties.push(prop_name);
                        }

                        // Build property access expression
                        let mut expr = Expression::Variable(properties[0].clone());
                        for prop in &properties[1..] {
                            expr = Expression::PropertyAccess {
                                object: Box::new(expr),
                                property: prop.clone(),
                            };
                        }
                        parts.push(InterpolationPart::Expression(expr));
                    } else {
                        parts.push(InterpolationPart::Variable(var_name));
                    }
                }

                // Skip the closing backtick
                if self.current_char == Some('`') {
                    self.advance();
                }
            } else if ch == '\\' {
                // Handle escape sequences in interpolated strings too
                self.handle_escape_sequence(&mut current_text);
            } else {
                current_text.push(ch);
                self.advance();
            }
        }

        // Add any remaining text
        if !current_text.is_empty() {
            parts.push(InterpolationPart::Text(current_text));
        }

        parts
    }

    fn read_identifier(&mut self) -> String {
        let mut ident = String::new();

        while let Some(ch) = self.current_char {
            if ch.is_alphanumeric() || ch == '-' || ch == '_' {
                ident.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        ident
    }

    fn read_variable(&mut self) -> String {
        self.advance(); // skip '~'
        self.read_identifier()
    }

    fn skip_comment(&mut self) {
        // Skip the '#' character
        self.advance();

        // Skip everything until newline or end of file
        while let Some(ch) = self.current_char {
            if ch == '\n' {
                break; // Stop at newline, but don't consume it
            }
            self.advance();
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        // Reset after_block flag for most tokens (set to true only when we produce a Block token)
        let was_after_block = self.after_block;
        self.after_block = false;

        match self.current_char {
            None => Token::Eof,
            Some('\n') => {
                self.advance();
                Token::Newline
            }
            Some('#') => {
                // Skip comment until end of line
                self.skip_comment();
                self.next_token() // Get the next token after the comment
            }
            Some('~') => {
                let var = self.read_variable();
                Token::Variable(var)
            }
            Some('"') => {
                // First peek ahead to see if this string contains backticks
                let original_pos = self.position;
                let original_char = self.current_char;

                // Skip opening quote and check for backticks
                self.advance();
                let mut has_backticks = false;
                while let Some(ch) = self.current_char {
                    if ch == '"' {
                        break;
                    }
                    if ch == '`' {
                        has_backticks = true;
                        break;
                    }
                    self.advance();
                }

                // Reset position
                self.position = original_pos;
                self.current_char = original_char;

                if has_backticks {
                    let parts = self.read_interpolated_string();
                    Token::InterpolatedString(parts)
                } else {
                    let string = self.read_string();
                    Token::String(string)
                }
            }
            Some('(') => {
                self.advance();
                Token::LeftParen
            }
            Some(')') => {
                self.advance();
                Token::RightParen
            }
            Some('{') => {
                self.advance();
                Token::LeftBrace
            }
            Some('}') => {
                self.advance();
                Token::RightBrace
            }
            Some('[') => {
                self.advance();
                Token::LeftBracket
            }
            Some(']') => {
                self.advance();
                Token::RightBracket
            }
            Some(':') => {
                // Check for block syntax :block_name:
                if self.peek().map_or(false, |c| c.is_alphabetic()) {
                    self.advance(); // consume first :
                    let block_name = self.read_identifier();
                    if self.current_char == Some(':') {
                        self.advance(); // consume second :
                        self.after_block = true;
                        Token::Block(block_name)
                    } else {
                        // If no closing :, treat as regular colon followed by identifier
                        // This is a bit tricky - we need to backtrack
                        // For now, let's require the closing colon
                        Token::Colon
                    }
                } else {
                    self.advance();
                    Token::Colon
                }
            }
            Some(',') => {
                self.advance();
                Token::Comma
            }
            Some('+') => {
                self.advance();
                Token::Plus
            }
            Some('-') => {
                self.advance();
                // Check if this is a negative number literal
                if let Some(ch) = self.current_char {
                    if ch.is_ascii_digit() {
                        let (num, was_float) = self.read_number();
                        return Token::Number(-num, was_float);
                    }
                }
                Token::Minus
            }
            Some('*') => {
                self.advance();
                // Check if this is an function call (*identifier) or multiplication
                if let Some(ch) = self.current_char {
                    if ch.is_alphabetic() {
                        // This is an function call
                        let name = self.read_identifier();
                        return Token::Identifier(format!("*{}", name));
                    }
                }
                Token::Multiply
            }
            Some('/') => {
                self.advance();
                Token::Divide
            }
            Some('\\') => {
                self.advance();
                Token::IntegerDivide
            }
            Some('%') => {
                self.advance();
                Token::Modulo
            }
            Some('.') => {
                self.advance();
                Token::Dot
            }
            Some('<') => {
                self.advance();
                if self.current_char == Some('=') {
                    self.advance();
                    Token::LessThanOrEqual
                } else {
                    Token::LessThan
                }
            }
            Some('>') => {
                self.advance();
                if self.current_char == Some('=') {
                    self.advance();
                    Token::GreaterThanOrEqual
                } else {
                    Token::GreaterThan
                }
            }
            Some('=') => {
                self.advance();
                if self.current_char == Some('=') {
                    self.advance();
                    Token::Equal
                } else {
                    self.next_token()
                }
            }
            Some('!') => {
                self.advance();
                if self.current_char == Some('=') {
                    self.advance();
                    Token::NotEqual
                } else {
                    self.next_token()
                }
            }
            Some(ch) if ch.is_ascii_digit() => {
                let (num, was_float) = self.read_number();
                Token::Number(num, was_float)
            }
            Some(ch) if ch.is_alphabetic() => {
                let ident = self.read_identifier();

                // If we're after a block, treat everything as an identifier
                if was_after_block {
                    Token::Identifier(ident)
                } else {
                    match ident.as_str() {
                        "is" => Token::Is,
                        "if" => Token::If,
                        "else" => Token::Else,
                        "loop" => Token::Loop,
                        "for-each" => Token::ForEach,
                        "break-loop" => Token::Breakloop,
                        "say" => Token::Say,
                        "ask" => Token::Ask,
                        "open" => Token::Open,
                        "get" => Token::Get,
                        "run" => Token::Run,
                        "scrape" => Token::Scrape,
                        "wait" => Token::Wait,
                        "in" => Token::In,
                        "otherwise" => Token::Otherwise,
                        "list" => Token::List,
                        "keys-of" => Token::KeysOf,
                        "values-of" => Token::ValuesOf,
                        "has-key" => Token::HasKey,
                        "function" => Token::Function,
                        "give" => Token::Give,
                        "and" => Token::And,
                        "or" => Token::Or,
                        "random" => Token::Random,
                        "read" => Token::Read,
                        "write" => Token::Write,
                        "clear" => Token::Clear,
                        "up" => Token::Up,
                        "down" => Token::Down,
                        "attempt" => Token::Attempt,
                        "rescue" => Token::Rescue,
                        "true" => Token::Boolean(true),
                        "false" => Token::Boolean(false),
                        _ => Token::Identifier(ident),
                    }
                }
            }
            Some(_) => {
                self.advance();
                self.next_token()
            }
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        loop {
            let token = self.next_token();
            if token == Token::Eof {
                tokens.push(token);
                break;
            }
            tokens.push(token);
        }

        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_assignment() {
        let mut lexer = Lexer::new("~counter is 0");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0], Token::Variable("counter".to_string()));
        assert_eq!(tokens[1], Token::Is);
        assert_eq!(tokens[2], Token::Number(0.0, false));
    }

    #[test]
    fn test_string_literal() {
        let mut lexer = Lexer::new("~name is \"fergus\"");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0], Token::Variable("name".to_string()));
        assert_eq!(tokens[1], Token::Is);
        assert_eq!(tokens[2], Token::String("fergus".to_string()));
    }

    #[test]
    fn test_operators() {
        let mut lexer = Lexer::new("< <= > >= == !=");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0], Token::LessThan);
        assert_eq!(tokens[1], Token::LessThanOrEqual);
        assert_eq!(tokens[2], Token::GreaterThan);
        assert_eq!(tokens[3], Token::GreaterThanOrEqual);
        assert_eq!(tokens[4], Token::Equal);
        assert_eq!(tokens[5], Token::NotEqual);
    }

    #[test]
    fn test_arithmetic_operators() {
        let mut lexer = Lexer::new("+ - * / \\ %");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0], Token::Plus);
        assert_eq!(tokens[1], Token::Minus);
        assert_eq!(tokens[2], Token::Multiply);
        assert_eq!(tokens[3], Token::Divide);
        assert_eq!(tokens[4], Token::IntegerDivide);
        assert_eq!(tokens[5], Token::Modulo);
    }

    #[test]
    fn test_expression() {
        let mut lexer = Lexer::new("(~counter + 1)");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0], Token::LeftParen);
        assert_eq!(tokens[1], Token::Variable("counter".to_string()));
        assert_eq!(tokens[2], Token::Plus);
        assert_eq!(tokens[3], Token::Number(1.0, false));
        assert_eq!(tokens[4], Token::RightParen);
    }

    #[test]
    fn test_function_keywords() {
        let mut lexer = Lexer::new("function give");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0], Token::Function);
        assert_eq!(tokens[1], Token::Give);
    }

    #[test]
    fn test_function_call() {
        let mut lexer = Lexer::new("*add 3 5");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0], Token::Identifier("*add".to_string()));
        assert_eq!(tokens[1], Token::Number(3.0, false));
        assert_eq!(tokens[2], Token::Number(5.0, false));
    }

    #[test]
    fn test_multiplication_vs_function_call() {
        let mut lexer = Lexer::new("3 * 5 *multiply");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0], Token::Number(3.0, false));
        assert_eq!(tokens[1], Token::Multiply);
        assert_eq!(tokens[2], Token::Number(5.0, false));
        assert_eq!(tokens[3], Token::Identifier("*multiply".to_string()));
    }

    #[test]
    fn test_run_token() {
        let mut lexer = Lexer::new("run \"echo hello\"");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0], Token::Run);
        assert_eq!(tokens[1], Token::String("echo hello".to_string()));
    }

    #[test]
    fn test_interpolated_string() {
        let mut lexer = Lexer::new("\"Hello `~name`!\"");
        let tokens = lexer.tokenize();

        if let Token::InterpolatedString(parts) = &tokens[0] {
            assert_eq!(parts.len(), 3);
            assert_eq!(parts[0], InterpolationPart::Text("Hello ".to_string()));
            assert_eq!(parts[1], InterpolationPart::Variable("name".to_string()));
            assert_eq!(parts[2], InterpolationPart::Text("!".to_string()));
        } else {
            panic!("Expected InterpolatedString token");
        }
    }

    #[test]
    fn test_regular_string_without_backticks() {
        let mut lexer = Lexer::new("\"Hello world\"");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0], Token::String("Hello world".to_string()));
    }

    #[test]
    fn test_single_line_comment() {
        let mut lexer = Lexer::new("# This is a comment\n~x is 5");
        let tokens = lexer.tokenize();

        // Comment should be completely skipped
        assert_eq!(tokens[0], Token::Newline);
        assert_eq!(tokens[1], Token::Variable("x".to_string()));
        assert_eq!(tokens[2], Token::Is);
        assert_eq!(tokens[3], Token::Number(5.0, false));
    }

    #[test]
    fn test_comment_at_end_of_file() {
        let mut lexer = Lexer::new("~x is 10\n# Final comment");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0], Token::Variable("x".to_string()));
        assert_eq!(tokens[1], Token::Is);
        assert_eq!(tokens[2], Token::Number(10.0, false));
        assert_eq!(tokens[3], Token::Newline);
        // Comment at end should be skipped, leaving EOF
        assert_eq!(tokens[4], Token::Eof);
    }

    #[test]
    fn test_multiple_comments() {
        let mut lexer = Lexer::new("# First comment\n# Second comment\n~y is 20");
        let tokens = lexer.tokenize();

        // Both comments should be skipped
        assert_eq!(tokens[0], Token::Newline);
        assert_eq!(tokens[1], Token::Newline);
        assert_eq!(tokens[2], Token::Variable("y".to_string()));
        assert_eq!(tokens[3], Token::Is);
        assert_eq!(tokens[4], Token::Number(20.0, false));
    }

    #[test]
    fn test_comment_with_special_characters() {
        let mut lexer = Lexer::new("# Comment with symbols: !@#$%^&*()+={}[]|\\:;\"'<>,.?/\n~z is true");
        let tokens = lexer.tokenize();

        // Comment with special chars should be completely ignored
        assert_eq!(tokens[0], Token::Newline);
        assert_eq!(tokens[1], Token::Variable("z".to_string()));
        assert_eq!(tokens[2], Token::Is);
        assert_eq!(tokens[3], Token::Boolean(true));
    }

    #[test]
    fn test_interpolated_string_with_property_access() {
        let mut lexer = Lexer::new("\"Hello `~user.name`!\"");
        let tokens = lexer.tokenize();

        if let Token::InterpolatedString(parts) = &tokens[0] {
            assert_eq!(parts.len(), 3);
            assert_eq!(parts[0], InterpolationPart::Text("Hello ".to_string()));

            // Check that the property access expression was created correctly
            if let InterpolationPart::Expression(Expression::PropertyAccess { object, property }) = &parts[1] {
                if let Expression::Variable(var_name) = object.as_ref() {
                    assert_eq!(var_name, "user");
                    assert_eq!(property, "name");
                } else {
                    panic!("Expected Variable expression in property access");
                }
            } else {
                panic!("Expected Expression interpolation part with property access");
            }

            assert_eq!(parts[2], InterpolationPart::Text("!".to_string()));
        } else {
            panic!("Expected InterpolatedString token");
        }
    }

    #[test]
    fn test_interpolated_string_with_nested_property_access() {
        let mut lexer = Lexer::new("\"Value: `~data.user.profile.name`\"");
        let tokens = lexer.tokenize();

        if let Token::InterpolatedString(parts) = &tokens[0] {
            assert_eq!(parts.len(), 2);
            assert_eq!(parts[0], InterpolationPart::Text("Value: ".to_string()));

            // Check the nested property access: data.user.profile.name
            if let InterpolationPart::Expression(Expression::PropertyAccess { object, property }) = &parts[1] {
                assert_eq!(property, "name");

                // Should be data.user.profile
                if let Expression::PropertyAccess { object: nested_obj, property: nested_prop } = object.as_ref() {
                    assert_eq!(nested_prop, "profile");

                    // Should be data.user
                    if let Expression::PropertyAccess { object: deep_obj, property: deep_prop } = nested_obj.as_ref() {
                        assert_eq!(deep_prop, "user");

                        // Should be data variable
                        if let Expression::Variable(var_name) = deep_obj.as_ref() {
                            assert_eq!(var_name, "data");
                        } else {
                            panic!("Expected Variable at the root of nested property access");
                        }
                    } else {
                        panic!("Expected nested PropertyAccess");
                    }
                } else {
                    panic!("Expected nested PropertyAccess");
                }
            } else {
                panic!("Expected Expression interpolation part with nested property access");
            }
        } else {
            panic!("Expected InterpolatedString token");
        }
    }

    #[test]
    fn test_interpolated_string_mixed_variables_and_properties() {
        let mut lexer = Lexer::new("\"Hi `~name`, age `~user.age`\"");
        let tokens = lexer.tokenize();

        if let Token::InterpolatedString(parts) = &tokens[0] {
            assert_eq!(parts.len(), 4);
            assert_eq!(parts[0], InterpolationPart::Text("Hi ".to_string()));
            assert_eq!(parts[1], InterpolationPart::Variable("name".to_string()));
            assert_eq!(parts[2], InterpolationPart::Text(", age ".to_string()));

            // Check property access for user.age
            if let InterpolationPart::Expression(Expression::PropertyAccess { object, property }) = &parts[3] {
                if let Expression::Variable(var_name) = object.as_ref() {
                    assert_eq!(var_name, "user");
                    assert_eq!(property, "age");
                } else {
                    panic!("Expected Variable expression in property access");
                }
            } else {
                panic!("Expected Expression interpolation part with property access");
            }
        } else {
            panic!("Expected InterpolatedString token");
        }
    }

    #[test]
    fn test_block_syntax_lexing() {
        let mut lexer = Lexer::new(":core:");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0], Token::Block("core".to_string()));
        assert_eq!(tokens[1], Token::Eof);
    }

    #[test]
    fn test_block_syntax_with_function() {
        let mut lexer = Lexer::new(":core:is-even 4");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0], Token::Block("core".to_string()));
        assert_eq!(tokens[1], Token::Identifier("is-even".to_string()));
        assert_eq!(tokens[2], Token::Number(4.0, false));
        assert_eq!(tokens[3], Token::Eof);
    }

    #[test]
    fn test_regular_colon_vs_block_syntax() {
        let mut lexer = Lexer::new("key: value :core:function");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0], Token::Identifier("key".to_string()));
        assert_eq!(tokens[1], Token::Colon);
        assert_eq!(tokens[2], Token::Identifier("value".to_string()));
        assert_eq!(tokens[3], Token::Block("core".to_string()));
        assert_eq!(tokens[4], Token::Identifier("function".to_string()));
        assert_eq!(tokens[5], Token::Eof);
    }

    #[test]
    fn test_multiple_blocks() {
        let mut lexer = Lexer::new(":core:is-even :math:sqrt");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0], Token::Block("core".to_string()));
        assert_eq!(tokens[1], Token::Identifier("is-even".to_string()));
        assert_eq!(tokens[2], Token::Block("math".to_string()));
        assert_eq!(tokens[3], Token::Identifier("sqrt".to_string()));
        assert_eq!(tokens[4], Token::Eof);
    }
}
