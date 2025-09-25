use std::env;
use std::fs;
use std::process;
use std::time::Instant;
use tilde::evaluator::Evaluator;
use tilde::lexer::Lexer;
use tilde::parser::Parser;

mod performance_analysis;
use performance_analysis::PerformanceAnalyzer;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <script.tde>", args[0]);
        process::exit(1);
    }

    let filename = &args[1];
    let contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Error reading file '{}': {}", filename, err);
            process::exit(1);
        }
    };

    let mut analyzer = PerformanceAnalyzer::new();

    // Overall execution time
    let start_total = Instant::now();

    // Lexing phase
    let mut lexer = analyzer.measure("lexing", || Lexer::new(&contents));

    let tokens = analyzer.measure("tokenization", || lexer.tokenize());

    // Parsing phase
    let program = analyzer.measure("parsing", || {
        let mut parser = Parser::new_from_tokens(tokens);
        parser.parse()
    });

    let program = match program {
        Ok(program) => program,
        Err(err) => {
            eprintln!("Parse error: {}", err);
            process::exit(1);
        }
    };

    // Evaluation phase
    let mut evaluator = Evaluator::new();
    let _result = analyzer.measure("evaluation", || evaluator.eval_program(program));

    let total_time = start_total.elapsed();

    // Report performance breakdown
    analyzer.report("Performance Breakdown");
    println!(
        "Total execution time: {:.2}ms",
        total_time.as_secs_f64() * 1000.0
    );
}
