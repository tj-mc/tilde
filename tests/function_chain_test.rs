use tilde::ast::*;
use tilde::parser::Parser;

#[test]
fn test_parse_simple_function_chain() {
    let input = r#"
        ~result:
           reverse [1, 2, 3]
           length
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    assert_eq!(program.len(), 1);

    if let Statement::FunctionChain { variable, steps } = &program[0] {
        assert_eq!(variable, "result");
        assert_eq!(steps.len(), 2);

        // First step: reverse [1, 2, 3]
        assert_eq!(steps[0].function_name, "reverse");
        assert_eq!(steps[0].args.len(), 1);

        // Second step: length (no args, takes previous result)
        assert_eq!(steps[1].function_name, "length");
        assert_eq!(steps[1].args.len(), 0);
    } else {
        panic!("Expected FunctionChain, got {:?}", program[0]);
    }
}

#[test]
fn test_parse_chain_with_multiple_args() {
    let input = r#"
        ~result:
           map [1, 2, 3] double
           filter is-even
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    assert_eq!(program.len(), 1);

    if let Statement::FunctionChain { variable, steps } = &program[0] {
        assert_eq!(variable, "result");
        assert_eq!(steps.len(), 2);

        // First step: map [1, 2, 3] double
        assert_eq!(steps[0].function_name, "map");
        assert_eq!(steps[0].args.len(), 2);

        // Second step: filter is-even
        assert_eq!(steps[1].function_name, "filter");
        assert_eq!(steps[1].args.len(), 1);
    } else {
        panic!("Expected FunctionChain, got {:?}", program[0]);
    }
}

#[test]
fn test_parse_chain_with_variables() {
    let input = r#"
        ~numbers is [5, 1, 3, 2, 4]
        ~result:
           sort ~numbers
           reverse
           length
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    assert_eq!(program.len(), 2);

    // Second statement should be the chain
    if let Statement::FunctionChain { variable, steps } = &program[1] {
        assert_eq!(variable, "result");
        assert_eq!(steps.len(), 3);

        // Check variable usage in first step
        if let Expression::Variable(var) = &steps[0].args[0] {
            assert_eq!(var, "numbers");
        } else {
            panic!("Expected Variable(numbers), got {:?}", steps[0].args[0]);
        }
    } else {
        panic!("Expected FunctionChain, got {:?}", program[1]);
    }
}

#[test]
fn test_parse_mixed_chain_and_regular_syntax() {
    let input = r#"
        ~numbers is [1, 2, 3, 4, 5]
        ~processed:
           sort ~numbers
           reverse
        ~final is length ~processed
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse().unwrap();

    assert_eq!(program.len(), 3);

    // First: regular assignment
    if let Statement::Assignment { .. } = &program[0] {
        // Good!
    } else {
        panic!("Expected Assignment, got {:?}", program[0]);
    }

    // Second: function chain
    if let Statement::FunctionChain { .. } = &program[1] {
        // Good!
    } else {
        panic!("Expected FunctionChain, got {:?}", program[1]);
    }

    // Third: regular assignment using chain result
    if let Statement::Assignment { .. } = &program[2] {
        // Good!
    } else {
        panic!("Expected Assignment, got {:?}", program[2]);
    }
}
