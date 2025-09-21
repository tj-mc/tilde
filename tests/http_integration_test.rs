use tilde::{evaluator::Evaluator, parser::Parser, value::Value};

// Note: These tests use httpbin.org which provides testing endpoints
// In a real environment, you might want to use a local test server

#[test]
fn test_http_get_basic() {
    let input = r#"
    ~response is get "https://httpbin.org/get"
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_ok());

    let response = evaluator.get_variable("response").unwrap();
    if let Value::Object(resp_map) = response {
        // Check that we got a successful response
        assert!(resp_map.contains_key("status"));
        assert!(resp_map.contains_key("headers"));
        assert!(resp_map.contains_key("body"));

        if let Some(Value::Number(status)) = resp_map.get("status") {
            assert_eq!(*status, 200.0);
        }

        if let Some(Value::Boolean(ok)) = resp_map.get("ok") {
            assert!(*ok);
        }
    } else {
        panic!("Expected response to be an object");
    }
}

#[test]
fn test_http_get_with_headers() {
    let input = r#"
    ~response is get "https://httpbin.org/get" {
        "headers": {
            "User-Agent": "Tails/1.0",
            "Accept": "application/json"
        },
        "timeout": 10000
    }
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_ok());

    let response = evaluator.get_variable("response").unwrap();
    if let Value::Object(resp_map) = response {
        if let Some(Value::Number(status)) = resp_map.get("status") {
            assert_eq!(*status, 200.0);
        }

        // Check that the request included our headers (httpbin echoes them back)
        if let Some(Value::Object(body)) = resp_map.get("body") {
            if let Some(Value::Object(headers)) = body.get("headers") {
                if let Some(Value::String(user_agent)) = headers.get("User-Agent") {
                    assert_eq!(user_agent, "Tails/1.0");
                }
            }
        }
    }
}

#[test]
fn test_http_post_with_json() {
    let input = r#"
    ~data is {
        "name": "John Doe",
        "email": "john@example.com",
        "age": 30
    }

    ~response is post "https://httpbin.org/post" {
        "body": ~data,
        "headers": {
            "Content-Type": "application/json"
        }
    }
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    if let Err(e) = &result {
        eprintln!("POST test error: {}", e);
    }
    assert!(result.is_ok());

    let response = evaluator.get_variable("response").unwrap();
    if let Value::Object(resp_map) = response {
        if let Some(Value::Number(status)) = resp_map.get("status") {
            assert_eq!(*status, 200.0);
        }

        // Check that our JSON data was received
        if let Some(Value::Object(body)) = resp_map.get("body") {
            if let Some(Value::Object(json_data)) = body.get("json") {
                if let Some(Value::String(name)) = json_data.get("name") {
                    assert_eq!(name, "John Doe");
                }
                if let Some(Value::Number(age)) = json_data.get("age") {
                    assert_eq!(*age, 30.0);
                }
            }
        }
    }
}

#[test]
fn test_http_put_request() {
    let input = r#"
    ~response is put "https://httpbin.org/put" {
        "body": "Updated data",
        "headers": {
            "Content-Type": "text/plain"
        }
    }
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_ok());

    let response = evaluator.get_variable("response").unwrap();
    if let Value::Object(resp_map) = response {
        if let Some(Value::Number(status)) = resp_map.get("status") {
            assert_eq!(*status, 200.0);
        }

        // Check that the PUT data was received
        if let Some(Value::Object(body)) = resp_map.get("body") {
            if let Some(Value::String(data)) = body.get("data") {
                assert_eq!(data, "Updated data");
            }
        }
    }
}

#[test]
fn test_http_delete_request() {
    let input = r#"
    ~response is delete "https://httpbin.org/delete"
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_ok());

    let response = evaluator.get_variable("response").unwrap();
    if let Value::Object(resp_map) = response {
        if let Some(Value::Number(status)) = resp_map.get("status") {
            assert_eq!(*status, 200.0);
        }
    }
}

#[test]
fn test_http_patch_request() {
    let input = r#"
    ~response is patch "https://httpbin.org/patch" {
        "body": "Patched data",
        "headers": {
            "Content-Type": "text/plain"
        }
    }
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    if let Err(e) = &result {
        eprintln!("PATCH test error: {}", e);
    }
    assert!(result.is_ok());

    let response = evaluator.get_variable("response").unwrap();
    if let Value::Object(resp_map) = response {
        if let Some(Value::Number(status)) = resp_map.get("status") {
            assert_eq!(*status, 200.0);
        }
    }
}

#[test]
fn test_http_generic_request() {
    let input = r#"
    ~response is http "GET" "https://httpbin.org/get" {
        "headers": {
            "X-Custom-Header": "test-value"
        }
    }
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_ok());

    let response = evaluator.get_variable("response").unwrap();
    if let Value::Object(resp_map) = response {
        if let Some(Value::Number(status)) = resp_map.get("status") {
            assert_eq!(*status, 200.0);
        }
    }
}

#[test]
fn test_http_error_handling_with_attempt_rescue() {
    let input = r#"
    ~error_caught is false
    ~status_code is 0

    attempt (
        ~response is get "https://httpbin.org/status/404"
    ) rescue ~error (
        ~error_caught is true
        ~status_code is ~error.context.status
    )
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_ok());

    let error_caught = evaluator.get_variable("error_caught").unwrap();
    assert_eq!(error_caught, &Value::Boolean(true));

    let status_code = evaluator.get_variable("status_code").unwrap();
    assert_eq!(status_code, &Value::Number(404.0));
}

#[test]
fn test_http_timeout_error() {
    let input = r#"
    ~timeout_error is false

    attempt (
        ~response is get "https://httpbin.org/delay/10" {
            "timeout": 1000
        }
    ) rescue ~error (
        ~timeout_error is contains ~error.message "timeout"
    )
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_ok());

    let timeout_error = evaluator.get_variable("timeout_error").unwrap();
    assert_eq!(timeout_error, &Value::Boolean(true));
}

#[test]
fn test_http_invalid_url_error() {
    let input = r#"
    ~error_caught is false

    attempt (
        ~response is get "not-a-valid-url"
    ) rescue ~error (
        ~error_caught is true
    )
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_ok());

    let error_caught = evaluator.get_variable("error_caught").unwrap();
    assert_eq!(error_caught, &Value::Boolean(true));
}

#[test]
fn test_http_response_headers_access() {
    let input = r#"
    ~response is get "https://httpbin.org/get"
    ~content_type is ~response.headers.content-type
    ~server is ~response.headers.server
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_ok());

    let content_type = evaluator.get_variable("content_type");
    assert!(content_type.is_some());

    let server = evaluator.get_variable("server");
    assert!(server.is_some());
}

#[test]
fn test_http_response_timing() {
    let input = r#"
    ~response is get "https://httpbin.org/get"
    ~response_time is ~response.response_time_ms
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    if let Err(e) = &result {
        eprintln!("Timing test error: {}", e);
    }
    assert!(result.is_ok());

    let response_time = evaluator.get_variable("response_time").unwrap();
    if let Value::Number(time_ms) = response_time {
        assert!(*time_ms > 0.0);
        assert!(*time_ms < 30000.0); // Should be less than 30 seconds
    }
}

#[test]
fn test_http_json_response_parsing() {
    let input = r#"
    ~response is get "https://httpbin.org/json"
    ~slide_author is ~response.body.slideshow.author
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_ok());

    let slide_author = evaluator.get_variable("slide_author");
    assert!(slide_author.is_some());
}

#[test]
fn test_http_complex_workflow() {
    let input = r#"
    # First, get some data
    ~get_response is get "https://httpbin.org/get"

    # Then post that data somewhere else
    ~post_response is post "https://httpbin.org/post" {
        "body": ~get_response.body,
        "headers": {
            "Content-Type": "application/json",
            "X-Workflow": "complex-test"
        }
    }

    # Check both responses were successful
    ~both_successful is ~get_response.ok and ~post_response.ok
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_program(program);
    assert!(result.is_ok());

    let both_successful = evaluator.get_variable("both_successful").unwrap();
    assert_eq!(both_successful, &Value::Boolean(true));
}