use std::fs;
use tilde::{evaluator::Evaluator, parser::Parser, value::Value};

fn create_test_file(filename: &str, content: &str) {
    fs::write(filename, content).unwrap();
}

fn cleanup_test_file(filename: &str) {
    fs::remove_file(filename).ok();
}

#[test]
fn test_read_simple_file() {
    let filename = "test_simple.txt";
    let content = "Hello, World!";
    create_test_file(filename, content);

    let input = format!(r#"
        ~file is read "{}"
        ~content is ~file.content
        ~size is ~file.size
        ~exists is ~file.exists
        ~error is ~file.error
    "#, filename);

    let mut parser = Parser::new(&input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    assert_eq!(
        evaluator.get_variable("content"),
        Some(&Value::String(content.to_string()))
    );
    assert_eq!(evaluator.get_variable("size"), Some(&Value::Number(13.0)));
    assert_eq!(evaluator.get_variable("exists"), Some(&Value::Boolean(true)));
    assert_eq!(evaluator.get_variable("error"), Some(&Value::Null));

    cleanup_test_file(filename);
}

#[test]
fn test_read_nonexistent_file() {
    let input = r#"
        ~file is read "nonexistent_file_12345.txt"
        ~content is ~file.content
        ~exists is ~file.exists
        ~error is ~file.error
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    assert_eq!(
        evaluator.get_variable("content"),
        Some(&Value::String("".to_string()))
    );
    assert_eq!(evaluator.get_variable("exists"), Some(&Value::Boolean(false)));
    assert_ne!(evaluator.get_variable("error"), Some(&Value::Null));
}

#[test]
fn test_read_empty_file() {
    let filename = "test_empty.txt";
    create_test_file(filename, "");

    let input = format!(r#"
        ~file is read "{}"
        ~content is ~file.content
        ~size is ~file.size
        ~exists is ~file.exists
    "#, filename);

    let mut parser = Parser::new(&input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    assert_eq!(
        evaluator.get_variable("content"),
        Some(&Value::String("".to_string()))
    );
    assert_eq!(evaluator.get_variable("size"), Some(&Value::Number(0.0)));
    assert_eq!(evaluator.get_variable("exists"), Some(&Value::Boolean(true)));

    cleanup_test_file(filename);
}

#[test]
fn test_read_multiline_file() {
    let filename = "test_multiline.txt";
    let content = "Line 1\nLine 2\nLine 3";
    create_test_file(filename, content);

    let input = format!(r#"
        ~file is read "{}"
        ~content is ~file.content
        ~size is ~file.size
    "#, filename);

    let mut parser = Parser::new(&input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    assert_eq!(
        evaluator.get_variable("content"),
        Some(&Value::String(content.to_string()))
    );
    assert_eq!(evaluator.get_variable("size"), Some(&Value::Number(20.0)));

    cleanup_test_file(filename);
}

#[test]
fn test_read_with_variables() {
    let filename = "test_variables.txt";
    let content = "Variable content";
    create_test_file(filename, content);

    let input = format!(r#"
        ~filename is "{}"
        ~file is read ~filename
        ~content is ~file.content
    "#, filename);

    let mut parser = Parser::new(&input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    assert_eq!(
        evaluator.get_variable("content"),
        Some(&Value::String(content.to_string()))
    );

    cleanup_test_file(filename);
}

#[test]
fn test_read_in_conditional() {
    let filename = "test_conditional.txt";
    let content = "Conditional test";
    create_test_file(filename, content);

    let input = format!(r#"
        ~file is read "{}"
        ~message is ""

        if ~file.exists (
            if ~file.size > 0 (
                ~message is "File exists and has content"
            ) else (
                ~message is "File exists but is empty"
            )
        ) else (
            ~message is "File does not exist"
        )
    "#, filename);

    let mut parser = Parser::new(&input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    assert_eq!(
        evaluator.get_variable("message"),
        Some(&Value::String("File exists and has content".to_string()))
    );

    cleanup_test_file(filename);
}

#[test]
fn test_read_multiple_files() {
    let file1 = "test_multi1.txt";
    let file2 = "test_multi2.txt";
    let content1 = "Content 1";
    let content2 = "Content 2";

    create_test_file(file1, content1);
    create_test_file(file2, content2);

    let input = format!(r#"
        ~file1 is read "{}"
        ~file2 is read "{}"
        ~total_size is ~file1.size + ~file2.size
        ~both_exist is ~file1.exists and ~file2.exists
    "#, file1, file2);

    let mut parser = Parser::new(&input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    assert_eq!(evaluator.get_variable("total_size"), Some(&Value::Number(18.0))); // 9 + 9
    assert_eq!(evaluator.get_variable("both_exist"), Some(&Value::Boolean(true)));

    cleanup_test_file(file1);
    cleanup_test_file(file2);
}

#[test]
fn test_read_in_loop() {
    let files = ["loop1.txt", "loop2.txt", "loop3.txt"];
    let contents = ["A", "BB", "CCC"];

    for (i, file) in files.iter().enumerate() {
        create_test_file(file, contents[i]);
    }

    let input = r#"
        ~files is ["loop1.txt", "loop2.txt", "loop3.txt"]
        ~total_size is 0
        ~i is 0
        ~files_len is length ~files

        loop (
            if ~i >= ~files_len break-loop
            ~filename is ~files.~i
            ~file is read ~filename
            if ~file.exists (
                ~total_size is ~total_size + ~file.size
            )
            ~i is ~i + 1
        )
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    assert_eq!(evaluator.get_variable("total_size"), Some(&Value::Number(6.0))); // 1 + 2 + 3

    for file in &files {
        cleanup_test_file(file);
    }
}

#[test]
fn test_read_statement_form() {
    let filename = "test_statement.txt";
    let content = "Statement test";
    create_test_file(filename, content);

    let input = format!(r#"
        read "{}"
        ~message is "Read operation completed"
    "#, filename);

    let mut parser = Parser::new(&input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_program(program);

    assert!(result.is_ok());
    assert_eq!(
        evaluator.get_variable("message"),
        Some(&Value::String("Read operation completed".to_string()))
    );

    cleanup_test_file(filename);
}

#[test]
fn test_read_error_handling() {
    let input = r#"
        ~result is read 123
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_program(program);

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("must be a string"));
}

#[test]
fn test_read_wrong_argument_count() {
    let input1 = r#"
        ~result is read "file1.txt" "file2.txt"
    "#;

    let mut parser = Parser::new(input1);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_program(program);

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("exactly 1 argument"));
}

#[test]
fn test_read_with_object_access() {
    let filename = "test_object.txt";
    let content = "Object access test";
    create_test_file(filename, content);

    let input = format!(r#"
        ~file is read "{}"

        function get-file-info ~file_obj (
            give {{
                "has_content": ~file_obj.size > 0,
                "content_length": ~file_obj.size,
                "is_valid": ~file_obj.exists
            }}
        )

        ~info is *get-file-info ~file
        ~has_content is ~info.has_content
        ~is_valid is ~info.is_valid
    "#, filename);

    let mut parser = Parser::new(&input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    assert_eq!(evaluator.get_variable("has_content"), Some(&Value::Boolean(true)));
    assert_eq!(evaluator.get_variable("is_valid"), Some(&Value::Boolean(true)));

    cleanup_test_file(filename);
}