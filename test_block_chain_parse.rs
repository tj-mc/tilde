#[cfg(test)]
mod test {
    use tilde::parser::Parser;
    use tilde::ast::*;

    #[test]
    fn test_parse_block_in_chain() {
        let input = "~result:\n    filter [1,2,3,4] core:is-even";
        let mut parser = Parser::new(input);
        let program = parser.parse().unwrap();

        if let Statement::FunctionChain { variable, steps } = &program[0] {
            println!("Variable: {}", variable);
            println!("Steps: {:?}", steps);
            assert_eq!(steps[0].function_name, "filter");
            assert_eq!(steps[0].args.len(), 2);
        } else {
            panic!("Expected FunctionChain, got {:?}", program[0]);
        }
    }
}