use tails::{evaluator::Evaluator, parser::Parser, value::Value};

#[test]
fn test_variable_assignment_and_retrieval() {
    let input = "
        ~name is \"John\"
        ~age is 25
        ~score is 3.14
    ";

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    assert_eq!(
        evaluator.get_variable("name"),
        Some(&Value::String("John".to_string()))
    );
    assert_eq!(evaluator.get_variable("age"), Some(&Value::Number(25.0)));
    assert_eq!(evaluator.get_variable("score"), Some(&Value::Number(3.14)));
}

#[test]
fn test_arithmetic_operations() {
    let input = "
        ~x is 10
        ~y is 20
        ~sum is ~x + ~y
        ~diff is ~y - ~x
    ";

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    assert_eq!(evaluator.get_variable("sum"), Some(&Value::Number(30.0)));
    assert_eq!(evaluator.get_variable("diff"), Some(&Value::Number(10.0)));
}

#[test]
fn test_comparison_operations() {
    let input = "
        ~x is 10
        ~y is 20
        ~less is ~x < ~y
        ~greater is ~x > ~y
        ~equal is ~x == ~y
        ~not_equal is ~x != ~y
    ";

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    assert_eq!(evaluator.get_variable("less"), Some(&Value::Boolean(true)));
    assert_eq!(
        evaluator.get_variable("greater"),
        Some(&Value::Boolean(false))
    );
    assert_eq!(
        evaluator.get_variable("equal"),
        Some(&Value::Boolean(false))
    );
    assert_eq!(
        evaluator.get_variable("not_equal"),
        Some(&Value::Boolean(true))
    );
}

#[test]
fn test_parenthesized_expressions() {
    let input = "
        ~result is (~x + 5)
        ~x is 10
        ~complex is (~x + 5) - (~x - 5)
    ";

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_program(program);

    assert!(result.is_err());

    let input2 = "
        ~x is 10
        ~result is (~x + 5)
        ~complex is (~x + 5) - (~x - 5)
    ";

    let mut parser = Parser::new(input2);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    assert_eq!(evaluator.get_variable("result"), Some(&Value::Number(15.0)));
    assert_eq!(
        evaluator.get_variable("complex"),
        Some(&Value::Number(10.0))
    );
}

#[test]
fn test_if_statement() {
    let input = "
        ~x is 10
        ~result is \"initial\"
        if ~x < 20 ~result is \"less than 20\" else ~result is \"greater or equal\"
    ";

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    assert_eq!(
        evaluator.get_variable("result"),
        Some(&Value::String("less than 20".to_string()))
    );
}

#[test]
fn test_loop_with_break_loop() {
    let input = "
        ~counter is 0
        loop (
            ~counter is ~counter + 1
            if ~counter >= 3 break-loop
        )
    ";

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    assert_eq!(evaluator.get_variable("counter"), Some(&Value::Number(3.0)));
}

#[test]
fn test_fibonacci_sequence() {
    let input = "
        ~a is 0
        ~b is 1
        ~count is 0
        ~max is 5
        
        loop (
            if ~count >= ~max break-loop
            ~next is ~a + ~b
            ~a is ~b
            ~b is ~next
            ~count is ~count + 1
        )
    ";

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    assert_eq!(evaluator.get_variable("count"), Some(&Value::Number(5.0)));
    assert_eq!(evaluator.get_variable("b"), Some(&Value::Number(8.0))); // 5th fibonacci number (0,1,1,2,3,5,8)
}

#[test]
fn test_nested_if_in_loop() {
    let input = "
        ~sum is 0
        ~i is 0
        
        loop (
            ~i is ~i + 1
            if ~i > 5 break-loop
            if ~i < 3 ~sum is ~sum + ~i
        )
    ";

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    // sum should be 1 + 2 = 3 (only values where i < 3)
    assert_eq!(evaluator.get_variable("sum"), Some(&Value::Number(3.0)));
    assert_eq!(evaluator.get_variable("i"), Some(&Value::Number(6.0))); // loop exits when i > 5
}

#[test]
fn test_multiplication_and_division() {
    let input = "
        ~x is 10
        ~y is 3
        ~product is ~x * ~y
        ~quotient is ~x / ~y
        ~complex is (~x + 2) * (~y - 1) / 4
    ";

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    assert_eq!(
        evaluator.get_variable("product"),
        Some(&Value::Number(30.0))
    );
    assert_eq!(
        evaluator.get_variable("quotient"),
        Some(&Value::Number(10.0 / 3.0))
    );
    assert_eq!(
        evaluator.get_variable("complex"),
        Some(&Value::Number((10.0 + 2.0) * (3.0 - 1.0) / 4.0))
    );
}

#[test]
fn test_operator_precedence() {
    let input = "
        ~result1 is 2 + 3 * 4
        ~result2 is (2 + 3) * 4
        ~result3 is 12 / 3 + 2
        ~result4 is 12 / (3 + 2)
    ";

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    assert_eq!(
        evaluator.get_variable("result1"),
        Some(&Value::Number(14.0))
    ); // 2 + (3 * 4)
    assert_eq!(
        evaluator.get_variable("result2"),
        Some(&Value::Number(20.0))
    ); // (2 + 3) * 4
    assert_eq!(evaluator.get_variable("result3"), Some(&Value::Number(6.0))); // (12 / 3) + 2
    assert_eq!(evaluator.get_variable("result4"), Some(&Value::Number(2.4))); // 12 / (3 + 2)
}

#[test]
fn test_division_by_zero() {
    let input = "
        ~x is 10
        ~result is ~x / 0
    ";

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_program(program);

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Division by zero"));
}

#[test]
fn test_if_block_syntax() {
    let input = "
        ~x is 10
        ~result is 0
        
        if ~x > 5 (
            ~result is ~x * 2
            ~message is \"big number\"
        )
    ";

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    assert_eq!(evaluator.get_variable("result"), Some(&Value::Number(20.0)));
    assert_eq!(
        evaluator.get_variable("message"),
        Some(&Value::String("big number".to_string()))
    );
}

#[test]
fn test_if_else_block_syntax() {
    let input = "
        ~x is 3
        ~category is \"unknown\"
        
        if ~x > 5 (
            ~category is \"big\"
            ~doubled is ~x * 2
        ) else (
            ~category is \"small\"
            ~halved is ~x / 2
        )
    ";

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    assert_eq!(
        evaluator.get_variable("category"),
        Some(&Value::String("small".to_string()))
    );
    assert_eq!(evaluator.get_variable("halved"), Some(&Value::Number(1.5)));
    assert_eq!(evaluator.get_variable("doubled"), None); // Should not be set
}

#[test]
fn test_multiline_if_blocks() {
    let input = "
        ~score is 85
        ~grade is \"\"
        ~comment is \"\"
        
        if ~score >= 90 (
            ~grade is \"A\"
            ~comment is \"Excellent work!\"
        ) else if ~score >= 80 (
            ~grade is \"B\"
            ~comment is \"Good job!\"
        ) else (
            ~grade is \"C\"
            ~comment is \"Keep trying!\"
        )
    ";

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    assert_eq!(
        evaluator.get_variable("grade"),
        Some(&Value::String("B".to_string()))
    );
    assert_eq!(
        evaluator.get_variable("comment"),
        Some(&Value::String("Good job!".to_string()))
    );
}

#[test]
fn test_object_creation_and_access() {
    let input = "
        ~person is {\"name\": \"Alice\" \"age\": 30}
        ~name is ~person.name
        ~age is ~person.age
    ";

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    assert_eq!(
        evaluator.get_variable("name"),
        Some(&Value::String("Alice".to_string()))
    );
    assert_eq!(evaluator.get_variable("age"), Some(&Value::Number(30.0)));

    // Check the object itself
    if let Some(Value::Object(map)) = evaluator.get_variable("person") {
        assert_eq!(map.get("name"), Some(&Value::String("Alice".to_string())));
        assert_eq!(map.get("age"), Some(&Value::Number(30.0)));
    } else {
        panic!("person should be an object");
    }
}

#[test]
fn test_property_assignment() {
    let input = "
        ~person is {\"name\": \"Alice\" \"age\": 30}
        ~person.age is 31
        ~person.city is \"New York\"
    ";

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    if let Some(Value::Object(map)) = evaluator.get_variable("person") {
        assert_eq!(map.get("name"), Some(&Value::String("Alice".to_string())));
        assert_eq!(map.get("age"), Some(&Value::Number(31.0))); // Changed from 30 to 31
        assert_eq!(
            map.get("city"),
            Some(&Value::String("New York".to_string()))
        ); // New property
    } else {
        panic!("person should be an object");
    }
}

#[test]
fn test_object_functions() {
    let input = "
        ~data is {\"x\": 10 \"y\": 20 \"z\": 30}
        ~all_keys is keys-of ~data
        ~all_values is values-of ~data
        ~has_x is has-key \"x\" ~data
        ~has_w is has-key \"w\" ~data
    ";

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    // Check keys
    if let Some(Value::List(keys)) = evaluator.get_variable("all_keys") {
        assert_eq!(keys.len(), 3);
        // Keys might be in different order, so check contains
        let key_strings: Vec<String> = keys.iter().map(|v| v.to_string()).collect();
        assert!(key_strings.contains(&"x".to_string()));
        assert!(key_strings.contains(&"y".to_string()));
        assert!(key_strings.contains(&"z".to_string()));
    } else {
        panic!("all_keys should be a list");
    }

    // Check values
    if let Some(Value::List(values)) = evaluator.get_variable("all_values") {
        assert_eq!(values.len(), 3);
        let value_nums: Vec<f64> = values
            .iter()
            .filter_map(|v| {
                if let Value::Number(n) = v {
                    Some(*n)
                } else {
                    None
                }
            })
            .collect();
        assert_eq!(value_nums.len(), 3);
        assert!(value_nums.contains(&10.0));
        assert!(value_nums.contains(&20.0));
        assert!(value_nums.contains(&30.0));
    } else {
        panic!("all_values should be a list");
    }

    // Check has-key
    assert_eq!(evaluator.get_variable("has_x"), Some(&Value::Boolean(true)));
    assert_eq!(
        evaluator.get_variable("has_w"),
        Some(&Value::Boolean(false))
    );
}

#[test]
fn test_nested_objects() {
    let input = "
        ~user is {\"name\": \"Bob\" \"age\": 25}
        ~config is {\"theme\": \"dark\" \"user\": ~user}
        ~user_name is ~config.user.name
        ~theme is ~config.theme
    ";

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    assert_eq!(
        evaluator.get_variable("user_name"),
        Some(&Value::String("Bob".to_string()))
    );
    assert_eq!(
        evaluator.get_variable("theme"),
        Some(&Value::String("dark".to_string()))
    );
}

#[test]
fn test_objects_in_conditionals() {
    let input = "
        ~settings is {\"debug\": true \"version\": 2}
        ~message is \"default\"
        
        if ~settings.debug (
            ~message is \"Debug mode enabled\"
        )
        
        if ~settings.version > 1 (
            ~advanced is true
        )
    ";

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    assert_eq!(
        evaluator.get_variable("message"),
        Some(&Value::String("Debug mode enabled".to_string()))
    );
    assert_eq!(
        evaluator.get_variable("advanced"),
        Some(&Value::Boolean(true))
    );
}

#[test]
fn test_multiline_objects() {
    let input = "
        ~config is {
            \"host\": \"localhost\"
            \"port\": 3000
            \"debug\": true
            \"database\": {
                \"name\": \"myapp\"
                \"user\": \"admin\"
                \"ssl\": false
            }
        }
        
        ~host is ~config.host
        ~db_name is ~config.database.name
        ~ssl_enabled is ~config.database.ssl
    ";

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    // Check that multiline object was parsed correctly
    assert_eq!(
        evaluator.get_variable("host"),
        Some(&Value::String("localhost".to_string()))
    );
    assert_eq!(
        evaluator.get_variable("db_name"),
        Some(&Value::String("myapp".to_string()))
    );
    assert_eq!(
        evaluator.get_variable("ssl_enabled"),
        Some(&Value::Boolean(false))
    );

    // Check the nested object structure
    if let Some(Value::Object(config)) = evaluator.get_variable("config") {
        assert_eq!(
            config.get("host"),
            Some(&Value::String("localhost".to_string()))
        );
        assert_eq!(config.get("port"), Some(&Value::Number(3000.0)));
        assert_eq!(config.get("debug"), Some(&Value::Boolean(true)));

        if let Some(Value::Object(database)) = config.get("database") {
            assert_eq!(
                database.get("name"),
                Some(&Value::String("myapp".to_string()))
            );
            assert_eq!(
                database.get("user"),
                Some(&Value::String("admin".to_string()))
            );
            assert_eq!(database.get("ssl"), Some(&Value::Boolean(false)));
        } else {
            panic!("database should be an object");
        }
    } else {
        panic!("config should be an object");
    }
}

#[test]
fn test_multiline_object_with_mixed_types() {
    let input = "
        ~data is {
            \"numbers\": 42
            \"text\": \"hello world\"
            \"flag\": true
            \"nested\": {
                \"inner_num\": 100
                \"inner_flag\": false
            }
        }
        
        ~keys is keys-of ~data
        ~nested_keys is keys-of ~data.nested
    ";

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    // Verify all types are correctly parsed
    if let Some(Value::Object(data)) = evaluator.get_variable("data") {
        assert_eq!(data.get("numbers"), Some(&Value::Number(42.0)));
        assert_eq!(
            data.get("text"),
            Some(&Value::String("hello world".to_string()))
        );
        assert_eq!(data.get("flag"), Some(&Value::Boolean(true)));

        if let Some(Value::Object(nested)) = data.get("nested") {
            assert_eq!(nested.get("inner_num"), Some(&Value::Number(100.0)));
            assert_eq!(nested.get("inner_flag"), Some(&Value::Boolean(false)));
        } else {
            panic!("nested should be an object");
        }
    } else {
        panic!("data should be an object");
    }

    // Verify keys function works with multiline objects
    if let Some(Value::List(keys)) = evaluator.get_variable("keys") {
        assert_eq!(keys.len(), 4);
        let key_strings: Vec<String> = keys.iter().map(|v| v.to_string()).collect();
        assert!(key_strings.contains(&"numbers".to_string()));
        assert!(key_strings.contains(&"text".to_string()));
        assert!(key_strings.contains(&"flag".to_string()));
        assert!(key_strings.contains(&"nested".to_string()));
    } else {
        panic!("keys should be a list");
    }
}

#[test]
fn test_multiline_object_assignment() {
    let input = "
        ~app is {
            \"name\": \"MyApp\"
            \"version\": \"1.0.0\"
            \"settings\": {
                \"theme\": \"light\"
                \"language\": \"en\"
            }
        }
        
        ~app.version is \"1.1.0\"
        ~app.settings is {\"theme\": \"dark\" \"language\": \"en\" \"timezone\": \"UTC\"}
    ";

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    // Check property assignments worked on multiline object
    if let Some(Value::Object(app)) = evaluator.get_variable("app") {
        assert_eq!(
            app.get("version"),
            Some(&Value::String("1.1.0".to_string()))
        ); // Updated

        if let Some(Value::Object(settings)) = app.get("settings") {
            assert_eq!(
                settings.get("theme"),
                Some(&Value::String("dark".to_string()))
            ); // Updated
            assert_eq!(
                settings.get("language"),
                Some(&Value::String("en".to_string()))
            ); // Updated
            assert_eq!(
                settings.get("timezone"),
                Some(&Value::String("UTC".to_string()))
            ); // Added
        } else {
            panic!("settings should be an object");
        }
    } else {
        panic!("app should be an object");
    }
}

#[test]
fn test_shell_command_execution() {
    let input = "
        ~result is run \"echo Hello World\"
        ~output is ~result.output
    ";

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    assert_eq!(
        evaluator.get_variable("output"),
        Some(&Value::String("Hello World\n".to_string()))
    );
}

#[test]
fn test_string_interpolation_basic() {
    let input = "
        ~name is \"Alice\"
        ~age is 25
        ~greeting is \"Hello `~name`, you are `~age` years old!\"
    ";

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    assert_eq!(
        evaluator.get_variable("greeting"),
        Some(&Value::String("Hello Alice, you are 25 years old!".to_string()))
    );
}

#[test]
fn test_shell_command_with_interpolation() {
    let input = "
        ~filename is \"test.txt\"
        ~command is \"echo 'Hello from Tails' > `~filename`\"
        ~result is run ~command
        ~readback is run \"cat `~filename` && rm `~filename`\"
    ";

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    // Check that the file was created and contains the expected content
    if let Some(Value::Object(readback_map)) = evaluator.get_variable("readback") {
        assert_eq!(
            readback_map.get("output"),
            Some(&Value::String("Hello from Tails\n".to_string()))
        );
    } else {
        panic!("Expected readback to be an object with output");
    }
}

#[test]
fn test_shell_commands_in_conditional() {
    let input = "
        ~check_result is run \"echo test\"
        ~message is \"\"
        if ~check_result.output == \"test\n\" (
            ~message is \"Command succeeded\"
        ) else (
            ~message is \"Command failed\"
        )
    ";

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    assert_eq!(
        evaluator.get_variable("message"),
        Some(&Value::String("Command succeeded".to_string()))
    );
}

#[test]
fn test_shell_commands_in_loop() {
    let input = "
        ~counter is 0
        loop (
            ~counter is ~counter + 1
            ~result is run \"echo Count: `~counter`\"
            ~output is ~result.output
            if ~counter >= 3 break-loop
        )
    ";

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    // Check that loop executed correctly
    assert_eq!(evaluator.get_variable("counter"), Some(&Value::Number(3.0)));

    // Check the last command output
    if let Some(Value::String(output)) = evaluator.get_variable("output") {
        assert_eq!(output, "Count: 3\n");
    } else {
        panic!("Expected output to be a string");
    }
}

#[test]
fn test_complex_interpolation_patterns() {
    let input = "
        ~username is \"Bob\"
        ~role is \"admin\"
        ~system is \"production\"
        ~cmd is \"echo User `~username` has `~role` access on `~system`\"
        ~result is run ~cmd
    ";

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    if let Some(Value::Object(result_map)) = evaluator.get_variable("result") {
        assert_eq!(
            result_map.get("output"),
            Some(&Value::String("User Bob has admin access on production\n".to_string()))
        );
    } else {
        panic!("Expected result to be an object with output");
    }
}

#[test]
fn test_shell_error_handling() {
    let input = "
        ~result is run \"ls /nonexistent/directory\"
        ~has_output is ~result.output != \"\"
    ";

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    // Command should fail and stderr should be captured in output
    if let Some(Value::Object(result_map)) = evaluator.get_variable("result") {
        if let Some(Value::String(output)) = result_map.get("output") {
            assert!(output.contains("No such file or directory") || output.contains("cannot access"));
        }
    }

    assert_eq!(
        evaluator.get_variable("has_output"),
        Some(&Value::Boolean(true))
    );
}

#[test]
fn test_nested_interpolation_and_commands() {
    let input = "
        ~base_dir is \"tmp\"
        ~file_name is \"nested_test\"
        ~extension is \"txt\"
        ~full_path is \"`~base_dir`/`~file_name`.`~extension`\"
        ~create_cmd is \"mkdir -p `~base_dir` && echo 'Nested content' > `~full_path`\"
        ~create_result is run ~create_cmd
        ~read_result is run \"cat `~full_path` && rm -rf `~base_dir`\"
    ";

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    if let Some(Value::Object(read_map)) = evaluator.get_variable("read_result") {
        assert_eq!(
            read_map.get("output"),
            Some(&Value::String("Nested content\n".to_string()))
        );
    } else {
        panic!("Expected read_result to be an object with output");
    }
}

#[test]
fn test_comments_basic_functionality() {
    let input = "
        # This is a comment at the beginning
        ~x is 10
        # Comment in the middle
        ~y is 20
        ~sum is ~x + ~y
        # Final comment
    ";

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    assert_eq!(evaluator.get_variable("x"), Some(&Value::Number(10.0)));
    assert_eq!(evaluator.get_variable("y"), Some(&Value::Number(20.0)));
    assert_eq!(evaluator.get_variable("sum"), Some(&Value::Number(30.0)));
}

#[test]
fn test_comments_in_control_structures() {
    let input = "
        # Test comments in if statements
        ~value is 15
        # Check if value is greater than 10
        if ~value > 10 (
            # Inside if block
            ~result is \"large\"
        ) else (
            # Inside else block
            ~result is \"small\"
        )
        # After if statement
    ";

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    assert_eq!(
        evaluator.get_variable("result"),
        Some(&Value::String("large".to_string()))
    );
}

#[test]
fn test_comments_in_loops() {
    let input = "
        # Initialize counter
        ~counter is 0
        # Start loop
        loop (
            # Increment counter
            ~counter is ~counter + 1
            # Check if we should break
            if ~counter >= 3 (
                # Break out of loop
                break-loop
            )
        )
        # Loop finished
    ";

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    assert_eq!(evaluator.get_variable("counter"), Some(&Value::Number(3.0)));
}

#[test]
fn test_comments_with_shell_commands() {
    let input = "
        # Test shell commands with comments
        ~greeting is \"Hello World\"
        # Execute echo command
        ~result is run \"echo `~greeting`\"
        # Check the output
        ~output is ~result.output
        # Final result should contain the greeting
    ";

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    assert_eq!(
        evaluator.get_variable("output"),
        Some(&Value::String("Hello World\n".to_string()))
    );
}

#[test]
fn test_comments_with_objects() {
    let input = "
        # Create a user object
        ~user is {\"name\": \"Alice\", \"age\": 30}
        # Access user properties
        ~name is ~user.name
        ~age is ~user.age
        # Create a summary
        ~summary is \"User `~name` is `~age` years old\"
        # End of test
    ";

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    assert_eq!(
        evaluator.get_variable("name"),
        Some(&Value::String("Alice".to_string()))
    );
    assert_eq!(evaluator.get_variable("age"), Some(&Value::Number(30.0)));
    assert_eq!(
        evaluator.get_variable("summary"),
        Some(&Value::String("User Alice is 30 years old".to_string()))
    );
}

#[test]
fn test_logical_and_operator() {
    let input = "
        ~result1 is true and true
        ~result2 is true and false
        ~result3 is false and true
        ~result4 is false and false
        ~result5 is 1 and 2
        ~result6 is 0 and 3
        ~result7 is 5 and 0
    ";

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    assert_eq!(evaluator.get_variable("result1"), Some(&Value::Boolean(true)));
    assert_eq!(evaluator.get_variable("result2"), Some(&Value::Boolean(false)));
    assert_eq!(evaluator.get_variable("result3"), Some(&Value::Boolean(false)));
    assert_eq!(evaluator.get_variable("result4"), Some(&Value::Boolean(false)));
    assert_eq!(evaluator.get_variable("result5"), Some(&Value::Number(2.0))); // 1 is truthy, return 2
    assert_eq!(evaluator.get_variable("result6"), Some(&Value::Number(0.0))); // 0 is falsy, return 0
    assert_eq!(evaluator.get_variable("result7"), Some(&Value::Number(0.0))); // 5 is truthy, return 0
}

#[test]
fn test_logical_or_operator() {
    let input = "
        ~result1 is true or true
        ~result2 is true or false
        ~result3 is false or true
        ~result4 is false or false
        ~result5 is 1 or 2
        ~result6 is 0 or 3
        ~result7 is 5 or 0
    ";

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    assert_eq!(evaluator.get_variable("result1"), Some(&Value::Boolean(true)));
    assert_eq!(evaluator.get_variable("result2"), Some(&Value::Boolean(true)));
    assert_eq!(evaluator.get_variable("result3"), Some(&Value::Boolean(true)));
    assert_eq!(evaluator.get_variable("result4"), Some(&Value::Boolean(false)));
    assert_eq!(evaluator.get_variable("result5"), Some(&Value::Number(1.0))); // 1 is truthy, return 1
    assert_eq!(evaluator.get_variable("result6"), Some(&Value::Number(3.0))); // 0 is falsy, return 3
    assert_eq!(evaluator.get_variable("result7"), Some(&Value::Number(5.0))); // 5 is truthy, return 5
}

#[test]
fn test_logical_operator_precedence() {
    let input = "
        ~result1 is true or false and false
        ~result2 is false and true or true
        ~result3 is 1 or 0 and 2
        ~result4 is 0 and 1 or 3
    ";

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    // true or (false and false) => true or false => true
    assert_eq!(evaluator.get_variable("result1"), Some(&Value::Boolean(true)));
    // (false and true) or true => false or true => true
    assert_eq!(evaluator.get_variable("result2"), Some(&Value::Boolean(true)));
    // 1 or (0 and 2) => 1 or 0 => 1
    assert_eq!(evaluator.get_variable("result3"), Some(&Value::Number(1.0)));
    // (0 and 1) or 3 => 0 or 3 => 3
    assert_eq!(evaluator.get_variable("result4"), Some(&Value::Number(3.0)));
}

#[test]
fn test_logical_operators_with_strings() {
    let input = "
        ~result1 is \"hello\" and \"world\"
        ~result2 is \"\" and \"world\"
        ~result3 is \"hello\" or \"\"
        ~result4 is \"\" or \"world\"
    ";

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    assert_eq!(
        evaluator.get_variable("result1"),
        Some(&Value::String("world".to_string()))
    ); // "hello" is truthy, return "world"
    assert_eq!(
        evaluator.get_variable("result2"),
        Some(&Value::String("".to_string()))
    ); // "" is falsy, return ""
    assert_eq!(
        evaluator.get_variable("result3"),
        Some(&Value::String("hello".to_string()))
    ); // "hello" is truthy, return "hello"
    assert_eq!(
        evaluator.get_variable("result4"),
        Some(&Value::String("world".to_string()))
    ); // "" is falsy, return "world"
}

#[test]
fn test_logical_operators_short_circuit() {
    // Test that the right operand is not evaluated when short-circuiting
    let input = "
        ~counter is 0
        action increment (
            ~counter is ~counter + 1
            give true
        )

        ~result1 is true or *increment
        ~result2 is false and *increment
    ";

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    // Counter should still be 0 because both operations short-circuited
    // true or X => true (X not evaluated)
    // false and X => false (X not evaluated)
    assert_eq!(evaluator.get_variable("counter"), Some(&Value::Number(0.0)));
    assert_eq!(evaluator.get_variable("result1"), Some(&Value::Boolean(true)));
    assert_eq!(evaluator.get_variable("result2"), Some(&Value::Boolean(false)));
}

#[test]
fn test_nested_function_calls_with_parentheses() {
    let input = "
        action sum ~first ~second (
            ~first + ~second
        )

        action multiply ~a ~b (
            ~a * ~b
        )

        ~result1 is *sum 43 (*sum 100 28)
        ~result2 is *multiply 5 (*sum 10 20)
        ~result3 is *sum (*sum 1 2) (*sum 3 4)
    ";

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    // Test nested addition: 43 + (100 + 28) = 171
    assert_eq!(evaluator.get_variable("result1"), Some(&Value::Number(171.0)));

    // Test mixed operations: 5 * (10 + 20) = 150
    assert_eq!(evaluator.get_variable("result2"), Some(&Value::Number(150.0)));

    // Test both args as nested calls: (1 + 2) + (3 + 4) = 10
    assert_eq!(evaluator.get_variable("result3"), Some(&Value::Number(10.0)));
}

#[test]
fn test_prime_number_checker() {
    let input = "
        action is-prime ~num (
            if ~num < 2 (
                give false
            ) else if ~num == 2 (
                give true
            ) else if ~num % 2 == 0 (
                give false
            ) else (
                ~is_prime is true
                ~i is 3
                loop (
                    if ~i * ~i > ~num break-loop
                    if ~num % ~i == 0 (
                        ~is_prime is false
                        break-loop
                    )
                    ~i is ~i + 2
                )
                give ~is_prime
            )
        )

        ~test1 is *is-prime 1
        ~test2 is *is-prime 2
        ~test3 is *is-prime 3
        ~test4 is *is-prime 4
        ~test5 is *is-prime 9
        ~test6 is *is-prime 11
        ~test7 is *is-prime 17
    ";

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    // Test prime number results
    assert_eq!(evaluator.get_variable("test1"), Some(&Value::Boolean(false))); // 1 is not prime
    assert_eq!(evaluator.get_variable("test2"), Some(&Value::Boolean(true)));  // 2 is prime
    assert_eq!(evaluator.get_variable("test3"), Some(&Value::Boolean(true)));  // 3 is prime
    assert_eq!(evaluator.get_variable("test4"), Some(&Value::Boolean(false))); // 4 is not prime
    assert_eq!(evaluator.get_variable("test5"), Some(&Value::Boolean(false))); // 9 is not prime
    assert_eq!(evaluator.get_variable("test6"), Some(&Value::Boolean(true)));  // 11 is prime
    assert_eq!(evaluator.get_variable("test7"), Some(&Value::Boolean(true)));  // 17 is prime
}

#[test]
fn test_action_call_in_if_condition_workaround() {
    // This test ensures the workaround for action calls in if conditions works
    let input = "
        action is-even ~num (
            give ~num % 2 == 0
        )

        ~results is []
        ~num is 1
        loop (
            if ~num > 5 break-loop
            ~even_check is *is-even ~num
            if ~even_check (
                ~results is append ~results ~num
            )
            ~num is ~num + 1
        )
    ";

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program).unwrap();

    // Should contain even numbers: [2, 4]
    if let Some(Value::List(results)) = evaluator.get_variable("results") {
        assert_eq!(results.len(), 2);
        assert_eq!(results[0], Value::Number(2.0));
        assert_eq!(results[1], Value::Number(4.0));
    } else {
        panic!("Expected results to be a list");
    }
}
