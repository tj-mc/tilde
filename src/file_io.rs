use crate::ast::Expression;
use crate::evaluator::Evaluator;
use crate::value::Value;
use std::collections::HashMap;

pub fn eval_read_positional(
    args: Vec<Expression>,
    evaluator: &mut Evaluator,
) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("read requires exactly 1 argument (file path)".to_string());
    }

    let path_val = evaluator.eval_expression(args[0].clone())?;
    let file_path = match path_val {
        Value::String(path) => path,
        _ => return Err("read argument must be a string (file path)".to_string()),
    };

    // Attempt to read the file
    let mut result = HashMap::new();

    match std::fs::read_to_string(&file_path) {
        Ok(content) => {
            // Calculate file size for metadata
            let size = match std::fs::metadata(&file_path) {
                Ok(metadata) => metadata.len() as f64,
                Err(_) => content.len() as f64, // Fallback to content length
            };

            result.insert("content".to_string(), Value::String(content));
            result.insert("size".to_string(), Value::Number(size));
            result.insert("exists".to_string(), Value::Boolean(true));
            result.insert("error".to_string(), Value::Null);
        }
        Err(err) => {
            // File operation failed - populate error details
            let exists = std::path::Path::new(&file_path).exists();
            let error_message = if !exists {
                "File not found".to_string()
            } else {
                format!("Failed to read file: {}", err)
            };

            result.insert("content".to_string(), Value::String("".to_string()));
            result.insert("size".to_string(), Value::Number(0.0));
            result.insert("exists".to_string(), Value::Boolean(exists));
            result.insert("error".to_string(), Value::String(error_message));
        }
    }

    Ok(Value::Object(result))
}

pub fn eval_write_positional(
    args: Vec<Expression>,
    evaluator: &mut Evaluator,
) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("write requires exactly 2 arguments (file path, content)".to_string());
    }

    let path_val = evaluator.eval_expression(args[0].clone())?;
    let file_path = match path_val {
        Value::String(path) => path,
        _ => return Err("write first argument must be a string (file path)".to_string()),
    };

    let content_val = evaluator.eval_expression(args[1].clone())?;
    let content = match content_val {
        Value::String(content) => content,
        Value::Number(n) => n.to_string(),
        Value::Boolean(b) => b.to_string(),
        Value::Null => "null".to_string(),
        Value::List(_) => content_val.to_string(),
        Value::Object(_) => content_val.to_string(),
        Value::Date(_) => content_val.to_string(),
        Value::Error(_) => content_val.to_string(),
        Value::Pattern(_) => content_val.to_string(),
    };

    // Attempt to write the file
    let mut result = HashMap::new();

    match std::fs::write(&file_path, &content) {
        Ok(()) => {
            // Successfully wrote the file
            result.insert("success".to_string(), Value::Boolean(true));
            result.insert("path".to_string(), Value::String(file_path));
            result.insert(
                "bytes_written".to_string(),
                Value::Number(content.len() as f64),
            );
            result.insert("error".to_string(), Value::Null);
        }
        Err(e) => {
            // Error writing file
            let error_message = format!("Failed to write file: {}", e);
            result.insert("success".to_string(), Value::Boolean(false));
            result.insert("path".to_string(), Value::String(file_path));
            result.insert("bytes_written".to_string(), Value::Number(0.0));
            result.insert("error".to_string(), Value::String(error_message));
        }
    }

    Ok(Value::Object(result))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Expression;
    use crate::evaluator::Evaluator;
    use std::fs;

    #[test]
    fn test_read_existing_file() {
        // Create a temporary test file
        let test_content = "Hello, test file!";
        let test_path = "test_read_file.txt";
        fs::write(test_path, test_content).unwrap();

        let mut evaluator = Evaluator::new();
        let args = vec![Expression::String(test_path.to_string())];

        let result = eval_read_positional(args, &mut evaluator).unwrap();

        if let Value::Object(obj) = result {
            assert_eq!(
                obj.get("content"),
                Some(&Value::String(test_content.to_string()))
            );
            assert_eq!(obj.get("exists"), Some(&Value::Boolean(true)));
            assert_eq!(obj.get("error"), Some(&Value::Null));

            if let Some(Value::Number(size)) = obj.get("size") {
                assert!(*size > 0.0);
            } else {
                panic!("Expected size to be a number");
            }
        } else {
            panic!("Expected object result");
        }

        // Clean up
        fs::remove_file(test_path).unwrap();
    }

    #[test]
    fn test_read_nonexistent_file() {
        let mut evaluator = Evaluator::new();
        let args = vec![Expression::String("nonexistent_file.txt".to_string())];

        let result = eval_read_positional(args, &mut evaluator).unwrap();

        if let Value::Object(obj) = result {
            assert_eq!(obj.get("content"), Some(&Value::String("".to_string())));
            assert_eq!(obj.get("exists"), Some(&Value::Boolean(false)));
            assert_eq!(obj.get("size"), Some(&Value::Number(0.0)));

            if let Some(Value::String(error)) = obj.get("error") {
                assert!(error.contains("File not found"));
            } else {
                panic!("Expected error message for nonexistent file");
            }
        } else {
            panic!("Expected object result");
        }
    }

    #[test]
    fn test_read_wrong_argument_count() {
        let mut evaluator = Evaluator::new();

        // Too few arguments
        let args = vec![];
        let result = eval_read_positional(args, &mut evaluator);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exactly 1 argument"));

        // Too many arguments
        let args = vec![
            Expression::String("file1.txt".to_string()),
            Expression::String("file2.txt".to_string()),
        ];
        let result = eval_read_positional(args, &mut evaluator);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exactly 1 argument"));
    }

    #[test]
    fn test_read_non_string_argument() {
        let mut evaluator = Evaluator::new();
        let args = vec![Expression::Number(123.0, false)];

        let result = eval_read_positional(args, &mut evaluator);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must be a string"));
    }

    #[test]
    fn test_read_empty_file() {
        // Create an empty test file
        let test_path = "test_empty_file.txt";
        fs::write(test_path, "").unwrap();

        let mut evaluator = Evaluator::new();
        let args = vec![Expression::String(test_path.to_string())];

        let result = eval_read_positional(args, &mut evaluator).unwrap();

        if let Value::Object(obj) = result {
            assert_eq!(obj.get("content"), Some(&Value::String("".to_string())));
            assert_eq!(obj.get("exists"), Some(&Value::Boolean(true)));
            assert_eq!(obj.get("error"), Some(&Value::Null));
            assert_eq!(obj.get("size"), Some(&Value::Number(0.0)));
        } else {
            panic!("Expected object result");
        }

        // Clean up
        fs::remove_file(test_path).unwrap();
    }

    #[test]
    fn test_read_with_variables() {
        // Test that we can read from a path stored in a variable
        let test_content = "Variable path test";
        let test_path = "test_var_path.txt";
        fs::write(test_path, test_content).unwrap();

        let mut evaluator = Evaluator::new();

        // Set up a variable with the file path
        evaluator
            .variables
            .insert("filepath".to_string(), Value::String(test_path.to_string()));

        let args = vec![Expression::Variable("filepath".to_string())];
        let result = eval_read_positional(args, &mut evaluator).unwrap();

        if let Value::Object(obj) = result {
            assert_eq!(
                obj.get("content"),
                Some(&Value::String(test_content.to_string()))
            );
            assert_eq!(obj.get("exists"), Some(&Value::Boolean(true)));
            assert_eq!(obj.get("error"), Some(&Value::Null));
        } else {
            panic!("Expected object result");
        }

        // Clean up
        fs::remove_file(test_path).unwrap();
    }

    #[test]
    fn test_write_simple_file() {
        let mut evaluator = Evaluator::new();
        let test_path = "test_write_simple.txt";
        let test_content = "Hello, write function!";

        let args = vec![
            Expression::String(test_path.to_string()),
            Expression::String(test_content.to_string()),
        ];

        let result = eval_write_positional(args, &mut evaluator).unwrap();

        if let Value::Object(obj) = result {
            assert_eq!(obj.get("success"), Some(&Value::Boolean(true)));
            assert_eq!(obj.get("path"), Some(&Value::String(test_path.to_string())));
            assert_eq!(obj.get("bytes_written"), Some(&Value::Number(22.0)));
            assert_eq!(obj.get("error"), Some(&Value::Null));
        } else {
            panic!("Expected object result");
        }

        // Verify file was actually written
        let written_content = fs::read_to_string(test_path).unwrap();
        assert_eq!(written_content, test_content);

        // Clean up
        fs::remove_file(test_path).unwrap();
    }

    #[test]
    fn test_write_number_content() {
        let mut evaluator = Evaluator::new();
        let test_path = "test_write_number.txt";

        let args = vec![
            Expression::String(test_path.to_string()),
            Expression::Number(42.5, true),
        ];

        let result = eval_write_positional(args, &mut evaluator).unwrap();

        if let Value::Object(obj) = result {
            assert_eq!(obj.get("success"), Some(&Value::Boolean(true)));
            assert_eq!(obj.get("bytes_written"), Some(&Value::Number(4.0))); // "42.5"
        }

        let written_content = fs::read_to_string(test_path).unwrap();
        assert_eq!(written_content, "42.5");

        fs::remove_file(test_path).unwrap();
    }

    #[test]
    fn test_write_boolean_content() {
        let mut evaluator = Evaluator::new();
        let test_path = "test_write_boolean.txt";

        let args = vec![
            Expression::String(test_path.to_string()),
            Expression::Boolean(true),
        ];

        let result = eval_write_positional(args, &mut evaluator).unwrap();

        if let Value::Object(obj) = result {
            assert_eq!(obj.get("success"), Some(&Value::Boolean(true)));
        }

        let written_content = fs::read_to_string(test_path).unwrap();
        assert_eq!(written_content, "true");

        fs::remove_file(test_path).unwrap();
    }

    #[test]
    fn test_write_wrong_argument_count() {
        let mut evaluator = Evaluator::new();

        // Too few arguments
        let args = vec![Expression::String("test.txt".to_string())];
        let result = eval_write_positional(args, &mut evaluator);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exactly 2 arguments"));

        // Too many arguments
        let args = vec![
            Expression::String("test.txt".to_string()),
            Expression::String("content".to_string()),
            Expression::String("extra".to_string()),
        ];
        let result = eval_write_positional(args, &mut evaluator);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exactly 2 arguments"));
    }

    #[test]
    fn test_write_non_string_path() {
        let mut evaluator = Evaluator::new();
        let args = vec![
            Expression::Number(123.0, false),
            Expression::String("content".to_string()),
        ];

        let result = eval_write_positional(args, &mut evaluator);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must be a string"));
    }

    #[test]
    fn test_write_invalid_path() {
        let mut evaluator = Evaluator::new();
        let invalid_path = "/invalid/nonexistent/directory/file.txt";

        let args = vec![
            Expression::String(invalid_path.to_string()),
            Expression::String("content".to_string()),
        ];

        let result = eval_write_positional(args, &mut evaluator).unwrap();

        if let Value::Object(obj) = result {
            assert_eq!(obj.get("success"), Some(&Value::Boolean(false)));
            assert_eq!(obj.get("bytes_written"), Some(&Value::Number(0.0)));
            assert_ne!(obj.get("error"), Some(&Value::Null));
        } else {
            panic!("Expected object result");
        }
    }

    #[test]
    fn test_write_overwrite_existing() {
        let mut evaluator = Evaluator::new();
        let test_path = "test_overwrite.txt";

        // Create initial file
        fs::write(test_path, "original content").unwrap();

        // Overwrite with new content
        let args = vec![
            Expression::String(test_path.to_string()),
            Expression::String("new content".to_string()),
        ];

        let result = eval_write_positional(args, &mut evaluator).unwrap();

        if let Value::Object(obj) = result {
            assert_eq!(obj.get("success"), Some(&Value::Boolean(true)));
            assert_eq!(obj.get("bytes_written"), Some(&Value::Number(11.0)));
        }

        // Verify content was overwritten
        let written_content = fs::read_to_string(test_path).unwrap();
        assert_eq!(written_content, "new content");

        fs::remove_file(test_path).unwrap();
    }
}
