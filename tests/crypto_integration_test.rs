use tilde::{parser::Parser, evaluator::Evaluator};

#[test]
fn test_sha256_basic() {
    let input = r#"
        ~text is "Hello, World!"
        ~hash is sha256 ~text
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    evaluator.eval_program(program).unwrap();

    // Expected SHA256 hash of "Hello, World!"
    assert_eq!(evaluator.get_variable("hash").unwrap().to_string(), "dffd6021bb2bd5b0af676290809ec3a53191dd81c7f70a4b28688a362182986f");
}

#[test]
fn test_sha256_empty_string() {
    let input = r#"
        ~empty is ""
        ~hash is sha256 ~empty
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    evaluator.eval_program(program).unwrap();

    // Expected SHA256 hash of empty string
    assert_eq!(evaluator.get_variable("hash").unwrap().to_string(), "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
}

#[test]
fn test_hmac_sha256_basic() {
    let input = r#"
        ~message is "Hello, World!"
        ~key is "secret"
        ~hmac is hmac-sha256 ~key ~message
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    evaluator.eval_program(program).unwrap();

    // Expected HMAC-SHA256 of "Hello, World!" with key "secret"
    assert_eq!(evaluator.get_variable("hmac").unwrap().to_string(), "fcfaffa7fef86515c7beb6b62d779fa4ccf092f2e61c164376054271252821ff");
}

#[test]
fn test_hmac_sha256_aws_style() {
    let input = r#"
        ~string_to_sign is "AWS4-HMAC-SHA256\n20230101T000000Z\n20230101/us-east-1/s3/aws4_request\nhash"
        ~signing_key is "AWS4wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY"
        ~signature is hmac-sha256 ~signing_key ~string_to_sign
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    evaluator.eval_program(program).unwrap();

    // This should produce a valid signature (exact value depends on implementation)
    let signature = evaluator.get_variable("signature").unwrap().to_string();
    assert_eq!(signature.len(), 64); // SHA256 hex string should be 64 characters
}

#[test]
fn test_md5_basic() {
    let input = r#"
        ~text is "Hello, World!"
        ~hash is md5 ~text
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    evaluator.eval_program(program).unwrap();

    // Expected MD5 hash of "Hello, World!"
    assert_eq!(evaluator.get_variable("hash").unwrap().to_string(), "65a8e27d8879283831b664bd8b7f0ad4");
}

#[test]
fn test_crypto_roundtrip() {
    let input = r#"
        ~original is "test data for AWS S3"
        ~hash is sha256 ~original
        ~encoded is base64-encode ~hash
        ~decoded is base64-decode ~encoded
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    evaluator.eval_program(program).unwrap();

    // The decoded hash should match the original hash
    assert_eq!(evaluator.get_variable("decoded").unwrap().to_string(), evaluator.get_variable("hash").unwrap().to_string());
}

#[test]
fn test_hmac_sha256_invalid_args() {
    let input = r#"
        ~result is hmac-sha256 "only_one_arg"
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    // This should result in an error
    let result = evaluator.eval_program(program);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("requires exactly 2 arguments"));
}