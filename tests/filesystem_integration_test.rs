use tilde::{parser::Parser, evaluator::Evaluator};
use std::fs;

#[test]
fn test_file_exists_true() {
    // Create a temporary test file
    let test_file = "/tmp/tilde_test_file.txt";
    fs::write(test_file, "test content").unwrap();

    let input = format!(r#"
        ~file_path is "{}"
        ~exists is file-exists ~file_path
    "#, test_file);

    let mut parser = Parser::new(&input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    evaluator.eval_program(program).unwrap();

    assert_eq!(evaluator.get_variable("exists").unwrap().to_string(), "true");

    // Clean up
    fs::remove_file(test_file).ok();
}

#[test]
fn test_file_exists_false() {
    let input = r#"
        ~file_path is "/nonexistent/file/path.txt"
        ~exists is file-exists ~file_path
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    evaluator.eval_program(program).unwrap();

    assert_eq!(evaluator.get_variable("exists").unwrap().to_string(), "false");
}

#[test]
fn test_file_size() {
    // Create a temporary test file with known content
    let test_file = "/tmp/tilde_test_size.txt";
    let content = "Hello, World! This is a test file.";
    fs::write(test_file, content).unwrap();

    let input = format!(r#"
        ~file_path is "{}"
        ~size is file-size ~file_path
    "#, test_file);

    let mut parser = Parser::new(&input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    evaluator.eval_program(program).unwrap();

    assert_eq!(evaluator.get_variable("size").unwrap().to_string(), content.len().to_string());

    // Clean up
    fs::remove_file(test_file).ok();
}

#[test]
fn test_file_size_nonexistent() {
    let input = r#"
        ~file_path is "/nonexistent/file/path.txt"
        ~size is file-size ~file_path
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    // This should result in an error
    let result = evaluator.eval_program(program);
    assert!(result.is_err());
    let error_msg = result.unwrap_err();
    assert!(error_msg.contains("No such file or directory") || error_msg.contains("file not found"));
}


#[test]
fn test_dir_exists() {
    let input = r#"
        ~dir_path is "/tmp"
        ~exists is dir-exists ~dir_path
        ~nonexistent is dir-exists "/nonexistent/directory"
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    evaluator.eval_program(program).unwrap();

    assert_eq!(evaluator.get_variable("exists").unwrap().to_string(), "true");
    assert_eq!(evaluator.get_variable("nonexistent").unwrap().to_string(), "false");
}

#[test]
fn test_filesystem_integration() {
    // Create a test file manually for integration testing
    let test_file = "/tmp/tilde_integration_test.txt";
    let content = "Integration test content";
    fs::write(test_file, content).unwrap();

    let input = format!(r#"
        ~file_path is "{}"
        ~exists_check is file-exists ~file_path
        ~size_check is file-size ~file_path
    "#, test_file);

    let mut parser = Parser::new(&input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    evaluator.eval_program(program).unwrap();

    assert_eq!(evaluator.get_variable("exists_check").unwrap().to_string(), "true");
    assert_eq!(evaluator.get_variable("size_check").unwrap().to_string(), content.len().to_string());

    // Clean up
    fs::remove_file(test_file).ok();
}