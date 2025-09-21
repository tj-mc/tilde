use std::fs;
use tails::{evaluator::Evaluator, parser::Parser, value::Value};

fn cleanup_test_file(filename: &str) {
    fs::remove_file(filename).ok();
}

#[test]
fn test_write_simple_file() {
    let filename = "test_write_simple.txt";
    let content = "Hello, World!";
    cleanup_test_file(filename);

    let input = format!(r#"
        ~result is write "{}" "{}"
        ~success is ~result.success
        ~path is ~result.path
        ~bytes_written is ~result.bytes_written
        ~error is ~result.error
    "#, filename, content);

    let mut parser = Parser::new(&input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    assert_eq!(evaluator.get_variable("success"), Some(&Value::Boolean(true)));
    assert_eq!(evaluator.get_variable("path"), Some(&Value::String(filename.to_string())));
    assert_eq!(evaluator.get_variable("bytes_written"), Some(&Value::Number(13.0)));
    assert_eq!(evaluator.get_variable("error"), Some(&Value::Null));

    // Verify file was actually written
    let written_content = fs::read_to_string(filename).unwrap();
    assert_eq!(written_content, content);

    cleanup_test_file(filename);
}

#[test]
fn test_write_number_content() {
    let filename = "test_write_number.txt";
    cleanup_test_file(filename);

    let input = format!(r#"
        ~result is write "{}" 42.5
        ~success is ~result.success
        ~bytes_written is ~result.bytes_written
    "#, filename);

    let mut parser = Parser::new(&input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    assert_eq!(evaluator.get_variable("success"), Some(&Value::Boolean(true)));
    assert_eq!(evaluator.get_variable("bytes_written"), Some(&Value::Number(4.0))); // "42.5"

    // Verify file content
    let written_content = fs::read_to_string(filename).unwrap();
    assert_eq!(written_content, "42.5");

    cleanup_test_file(filename);
}

#[test]
fn test_write_boolean_content() {
    let filename = "test_write_boolean.txt";
    cleanup_test_file(filename);

    let input = format!(r#"
        ~result is write "{}" true
        ~success is ~result.success
    "#, filename);

    let mut parser = Parser::new(&input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    assert_eq!(evaluator.get_variable("success"), Some(&Value::Boolean(true)));

    let written_content = fs::read_to_string(filename).unwrap();
    assert_eq!(written_content, "true");

    cleanup_test_file(filename);
}

#[test]
fn test_write_with_variables() {
    let filename = "test_write_variables.txt";
    let content = "Variable content";
    cleanup_test_file(filename);

    let input = format!(r#"
        ~filepath is "{}"
        ~text is "{}"
        ~result is write ~filepath ~text
        ~success is ~result.success
    "#, filename, content);

    let mut parser = Parser::new(&input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    assert_eq!(evaluator.get_variable("success"), Some(&Value::Boolean(true)));

    let written_content = fs::read_to_string(filename).unwrap();
    assert_eq!(written_content, content);

    cleanup_test_file(filename);
}

#[test]
fn test_write_overwrite_existing() {
    let filename = "test_write_overwrite.txt";
    cleanup_test_file(filename);

    // Create initial file
    fs::write(filename, "original content").unwrap();

    let input = format!(r#"
        ~result is write "{}" "new content"
        ~success is ~result.success
        ~bytes_written is ~result.bytes_written
    "#, filename);

    let mut parser = Parser::new(&input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    assert_eq!(evaluator.get_variable("success"), Some(&Value::Boolean(true)));
    assert_eq!(evaluator.get_variable("bytes_written"), Some(&Value::Number(11.0)));

    // Verify content was overwritten
    let written_content = fs::read_to_string(filename).unwrap();
    assert_eq!(written_content, "new content");

    cleanup_test_file(filename);
}

#[test]
fn test_write_invalid_path() {
    let invalid_path = "/invalid/nonexistent/directory/file.txt";

    let input = format!(r#"
        ~result is write "{}" "content"
        ~success is ~result.success
        ~bytes_written is ~result.bytes_written
        ~error is ~result.error
    "#, invalid_path);

    let mut parser = Parser::new(&input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    assert_eq!(evaluator.get_variable("success"), Some(&Value::Boolean(false)));
    assert_eq!(evaluator.get_variable("bytes_written"), Some(&Value::Number(0.0)));
    assert_ne!(evaluator.get_variable("error"), Some(&Value::Null));
}

#[test]
fn test_write_in_conditional() {
    let filename = "test_write_conditional.txt";
    cleanup_test_file(filename);

    let input = format!(r#"
        ~should_write is true
        ~message is ""

        if ~should_write (
            ~result is write "{}" "conditional write"
            if ~result.success (
                ~message is "Write successful"
            ) else (
                ~message is "Write failed"
            )
        ) else (
            ~message is "Skipped write"
        )
    "#, filename);

    let mut parser = Parser::new(&input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    assert_eq!(
        evaluator.get_variable("message"),
        Some(&Value::String("Write successful".to_string()))
    );

    let written_content = fs::read_to_string(filename).unwrap();
    assert_eq!(written_content, "conditional write");

    cleanup_test_file(filename);
}

#[test]
fn test_write_multiple_files() {
    let file1 = "test_write_multi1.txt";
    let file2 = "test_write_multi2.txt";
    let content1 = "Content 1";
    let content2 = "Content 2";

    cleanup_test_file(file1);
    cleanup_test_file(file2);

    let input = format!(r#"
        ~result1 is write "{}" "{}"
        ~result2 is write "{}" "{}"
        ~both_success is ~result1.success and ~result2.success
        ~total_bytes is ~result1.bytes_written + ~result2.bytes_written
    "#, file1, content1, file2, content2);

    let mut parser = Parser::new(&input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    assert_eq!(evaluator.get_variable("both_success"), Some(&Value::Boolean(true)));
    assert_eq!(evaluator.get_variable("total_bytes"), Some(&Value::Number(18.0))); // 9 + 9

    // Verify both files were written
    let content1_actual = fs::read_to_string(file1).unwrap();
    let content2_actual = fs::read_to_string(file2).unwrap();
    assert_eq!(content1_actual, content1);
    assert_eq!(content2_actual, content2);

    cleanup_test_file(file1);
    cleanup_test_file(file2);
}

#[test]
fn test_write_in_loop() {
    let files = ["loop_write1.txt", "loop_write2.txt", "loop_write3.txt"];
    let contents = ["A", "BB", "CCC"];

    for file in &files {
        cleanup_test_file(file);
    }

    let input = r#"
        ~files is ["loop_write1.txt", "loop_write2.txt", "loop_write3.txt"]
        ~contents is ["A", "BB", "CCC"]
        ~total_bytes is 0
        ~i is 0
        ~files_len is length ~files

        loop (
            if ~i >= ~files_len break-loop
            ~filename is ~files.~i
            ~content is ~contents.~i
            ~result is write ~filename ~content
            if ~result.success (
                ~total_bytes is ~total_bytes + ~result.bytes_written
            )
            ~i is ~i + 1
        )
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    assert_eq!(evaluator.get_variable("total_bytes"), Some(&Value::Number(6.0))); // 1 + 2 + 3

    // Verify all files were written correctly
    for (i, file) in files.iter().enumerate() {
        let written_content = fs::read_to_string(file).unwrap();
        assert_eq!(written_content, contents[i]);
        cleanup_test_file(file);
    }
}

#[test]
fn test_write_statement_form() {
    let filename = "test_write_statement.txt";
    cleanup_test_file(filename);

    let input = format!(r#"
        write "{}" "Statement test"
        ~message is "Write operation completed"
    "#, filename);

    let mut parser = Parser::new(&input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_program(program);

    assert!(result.is_ok());
    assert_eq!(
        evaluator.get_variable("message"),
        Some(&Value::String("Write operation completed".to_string()))
    );

    // Verify file was written
    let written_content = fs::read_to_string(filename).unwrap();
    assert_eq!(written_content, "Statement test");

    cleanup_test_file(filename);
}

#[test]
fn test_write_error_handling() {
    let input = r#"
        ~result is write 123 "content"
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_program(program);

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("must be a string"));
}

#[test]
fn test_write_wrong_argument_count() {
    let input1 = r#"
        ~result is write "file.txt"
    "#;

    let mut parser = Parser::new(input1);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_program(program);

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("exactly 2 arguments"));
}

#[test]
fn test_write_with_object_content() {
    let filename = "test_write_object.txt";
    cleanup_test_file(filename);

    let input = format!(r#"
        ~data is {{"name": "Alice", "age": 30}}
        ~result is write "{}" ~data
        ~success is ~result.success
    "#, filename);

    let mut parser = Parser::new(&input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    assert_eq!(evaluator.get_variable("success"), Some(&Value::Boolean(true)));

    // Object should be written as its string representation
    let written_content = fs::read_to_string(filename).unwrap();
    assert!(written_content.contains("name"));
    assert!(written_content.contains("Alice"));

    cleanup_test_file(filename);
}

#[test]
fn test_write_with_function() {
    let filename = "test_write_function.txt";
    cleanup_test_file(filename);

    let input = format!(r#"
        function write-data ~file ~content (
            ~result is write ~file ~content
            give ~result
        )

        ~result is *write-data "{}" "Action test"
        ~success is ~result.success
    "#, filename);

    let mut parser = Parser::new(&input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    assert_eq!(evaluator.get_variable("success"), Some(&Value::Boolean(true)));

    let written_content = fs::read_to_string(filename).unwrap();
    assert_eq!(written_content, "Action test");

    cleanup_test_file(filename);
}