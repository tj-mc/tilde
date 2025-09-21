use crate::ast::Expression;
use crate::evaluator::Evaluator;
use crate::value::Value;

/// Terminal control operations for Tilde
/// Handles terminal manipulation commands like clear screen

pub fn eval_clear_positional(args: Vec<Expression>, _evaluator: &mut Evaluator) -> Result<Value, String> {
    if !args.is_empty() {
        return Err("clear takes no arguments".to_string());
    }

    // Send ANSI escape sequence to clear screen and move cursor to top-left
    // This works on most modern terminals (Unix, Windows Terminal, etc.)
    #[cfg(target_arch = "wasm32")]
    {
        // For WASM/web environments, we can't directly clear the terminal
        // This would need to be handled by JavaScript
        use crate::wasm::clear_console;
        clear_console();
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        // ANSI escape sequence: ESC[c (clear/reset screen) + ESC[H (move cursor to home)
        print!("\x1bc\x1b[H");
        // Ensure the escape sequence is immediately sent to terminal
        use std::io::{self, Write};
        io::stdout().flush().unwrap();
    }

    Ok(Value::Null)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Expression;
    use crate::evaluator::Evaluator;

    #[test]
    fn test_clear_no_arguments() {
        let mut evaluator = Evaluator::new();
        let args = vec![];

        let result = eval_clear_positional(args, &mut evaluator).unwrap();
        assert_eq!(result, Value::Null);
    }

    #[test]
    fn test_clear_with_arguments_error() {
        let mut evaluator = Evaluator::new();
        let args = vec![Expression::String("invalid".to_string())];

        let result = eval_clear_positional(args, &mut evaluator);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("clear takes no arguments"));
    }

    #[test]
    fn test_clear_with_multiple_arguments_error() {
        let mut evaluator = Evaluator::new();
        let args = vec![
            Expression::String("arg1".to_string()),
            Expression::Number(42.0, false),
        ];

        let result = eval_clear_positional(args, &mut evaluator);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("clear takes no arguments"));
    }
}