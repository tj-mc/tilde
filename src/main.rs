use std::env;
use std::fs;
use std::io::{self, Write};
use tilde::{evaluator::Evaluator, parser::Parser};

fn version_string() -> String {
    format!("~tilde v{}", env!("CARGO_PKG_VERSION"))
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "--version" | "-v" => {
                println!("{}", version_string());
                return;
            }
            "--help" | "-h" => {
                print_help();
                return;
            }
            _ => {
                // File execution mode
                run_file(&args[1]);
            }
        }
    } else {
        // REPL mode
        run_repl();
    }
}

fn run_file(filename: &str) {
    let contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", filename, e);
            return;
        }
    };

    let mut parser = Parser::new(&contents);
    let program = match parser.parse() {
        Ok(program) => program,
        Err(e) => {
            eprintln!("Parse error: {}", e);
            return;
        }
    };

    let mut evaluator = Evaluator::new();
    if let Err(e) = evaluator.eval_program(program) {
        eprintln!("Runtime error: {}", e);
    }
}

fn run_repl() {
    println!("{}", version_string());
    println!("Type 'exit' to quit\n");

    let mut evaluator = Evaluator::new();
    let mut input_buffer = String::new();
    let mut is_multiline = false;

    loop {
        // Show appropriate prompt
        if is_multiline {
            print!("... ");
        } else {
            print!("> ");
        }
        io::stdout().flush().unwrap();

        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                continue;
            }
        }

        let line = line.trim();

        // Handle exit command
        if line == "exit" && input_buffer.is_empty() {
            break;
        }

        // Handle empty lines
        if line.is_empty() {
            if is_multiline {
                // Empty line in multiline mode - try to execute what we have
                if should_execute_buffer(&input_buffer) {
                    execute_buffer(&mut evaluator, &input_buffer);
                    input_buffer.clear();
                    is_multiline = false;
                }
                // Otherwise continue collecting input
            }
            continue;
        }

        // Add line to buffer
        if !input_buffer.is_empty() {
            input_buffer.push('\n');
        }
        input_buffer.push_str(line);

        // Check if we need to continue collecting input
        if needs_more_input(&input_buffer) {
            is_multiline = true;
            continue;
        }

        // Execute the complete input
        execute_buffer(&mut evaluator, &input_buffer);
        input_buffer.clear();
        is_multiline = false;
    }
}

fn needs_more_input(input: &str) -> bool {
    let mut paren_count = 0;
    let mut brace_count = 0;
    let mut in_string = false;
    let mut escape_next = false;
    let mut string_char = '\0';

    for ch in input.chars() {
        if escape_next {
            escape_next = false;
            continue;
        }

        if ch == '\\' && in_string {
            escape_next = true;
            continue;
        }

        if ch == '"' && !in_string {
            in_string = true;
            string_char = ch;
            continue;
        }

        if ch == string_char && in_string {
            in_string = false;
            continue;
        }

        if in_string {
            continue;
        }

        match ch {
            '(' => paren_count += 1,
            ')' => paren_count -= 1,
            '{' => brace_count += 1,
            '}' => brace_count -= 1,
            _ => {}
        }
    }

    // Need more input if we have unclosed brackets/parens
    paren_count > 0 || brace_count > 0
}

fn should_execute_buffer(buffer: &str) -> bool {
    // If buffer is just whitespace, don't execute
    if buffer.trim().is_empty() {
        return false;
    }

    // If we still need more input, don't execute
    !needs_more_input(buffer)
}

fn print_help() {
    println!("{}", version_string());
    println!("A simple, readable scripting language");
    println!("");
    println!("USAGE:");
    println!("  tilde                  Start interactive REPL");
    println!("  tilde <file>          Run a Tilde script file");
    println!("  tilde --version       Show version information");
    println!("  tilde --help          Show this help message");
    println!("");
    println!("EXAMPLES:");
    println!("  tilde                 # Start REPL");
    println!("  tilde hello.tde     # Run hello.tde");
    println!("");
    println!("LANGUAGE FEATURES:");
    println!("  Variables:    ~name is \"Hello\"");
    println!("  Output:       say ~name");
    println!("  Input:        ~input is ask \"What's your name?\"");
    println!("  Objects:      ~user is {{name: \"Alice\", age: 30}}");
    println!("  Control:      if ~age > 18 then say \"Adult\" else say \"Minor\"");
}

fn execute_buffer(evaluator: &mut Evaluator, buffer: &str) {
    let mut parser = Parser::new(buffer);
    match parser.parse() {
        Ok(program) => match evaluator.eval_program(program) {
            Ok(_) => {}
            Err(e) => println!("Runtime error: {}", e),
        },
        Err(e) => println!("Parse error: {}", e),
    }
}
