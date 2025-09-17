use std::time::Instant;
use tails::{evaluator::Evaluator, lexer::Lexer, parser::Parser};

// Simple benchmarking without external dependencies
fn benchmark<F>(name: &str, iterations: u32, f: F)
where
    F: Fn(),
{
    println!("Benchmarking {name}...");
    let start = Instant::now();

    for _ in 0..iterations {
        f();
    }

    let duration = start.elapsed();
    let avg_microseconds = duration.as_micros() as f64 / iterations as f64;
    let ops_per_sec = 1_000_000.0 / avg_microseconds;

    println!("  {iterations} iterations in {duration:?}");
    println!("  Average: {avg_microseconds:.2}μs per operation");
    println!("  Throughput: {ops_per_sec:.0} ops/sec");
    println!();
}

fn main() {
    println!("🔥 Tails Language Benchmarks (Zero Dependencies)");
    println!("==================================================\n");

    // Lexer benchmarks
    benchmark_lexer();

    // Parser benchmarks
    benchmark_parser();

    // Evaluator benchmarks
    benchmark_evaluator();

    // End-to-end benchmarks
    benchmark_full_pipeline();

    println!("✅ Benchmarks complete!");
}

fn benchmark_lexer() {
    println!("📖 LEXER BENCHMARKS");
    println!("-----------------");

    // Simple tokenization
    let simple_code = "~x is 42";
    benchmark("Simple tokenization", 10000, || {
        let mut lexer = Lexer::new(simple_code);
        let _tokens = lexer.tokenize();
    });

    // Complex tokenization
    let complex_code = r#"
        ~name is ask "Enter name: "
        ~age is ask "Enter age: "
        if ~age >= 18 say "Hello adult" ~name else say "Hello minor" ~name
        loop (
            ~counter is ~counter + 1
            if ~counter >= 10 break-loop
            say "Count:" ~counter
        )
    "#;
    benchmark("Complex tokenization", 1000, || {
        let mut lexer = Lexer::new(complex_code);
        let _tokens = lexer.tokenize();
    });
}

fn benchmark_parser() {
    println!("🌳 PARSER BENCHMARKS");
    println!("------------------");

    // Simple parsing
    let simple_code = "~x is 42 + 58";
    benchmark("Simple parsing", 5000, || {
        let mut parser = Parser::new(simple_code);
        let _program = parser.parse().unwrap();
    });

    // Complex parsing
    let complex_code = r#"
        ~fibonacci is 0
        ~prev is 1
        ~count is 0
        
        loop (
            if ~count >= 10 break-loop
            ~temp is ~fibonacci + ~prev
            ~prev is ~fibonacci
            ~fibonacci is ~temp
            ~count is ~count + 1
            say "Fibonacci" ~count ":" ~fibonacci
        )
    "#;
    benchmark("Complex parsing", 1000, || {
        let mut parser = Parser::new(complex_code);
        let _program = parser.parse().unwrap();
    });
}

fn benchmark_evaluator() {
    println!("⚡ EVALUATOR BENCHMARKS");
    println!("--------------------");

    // Simple evaluation
    let simple_code = "~x is 42 + 58";
    let mut parser = Parser::new(simple_code);
    let program = parser.parse().unwrap();

    benchmark("Simple evaluation", 5000, || {
        let mut evaluator = Evaluator::new();
        let _result = evaluator.eval_program(program.clone()).unwrap();
    });

    // Arithmetic evaluation
    let arithmetic_code = r#"
        ~a is 100
        ~b is 50
        ~sum is ~a + ~b
        ~diff is ~a - ~b
        ~comparison is ~sum > ~diff
    "#;
    let mut parser = Parser::new(arithmetic_code);
    let program = parser.parse().unwrap();

    benchmark("Arithmetic evaluation", 2000, || {
        let mut evaluator = Evaluator::new();
        let _result = evaluator.eval_program(program.clone()).unwrap();
    });

    // Loop evaluation
    let loop_code = r#"
        ~counter is 0
        loop (
            ~counter is ~counter + 1
            if ~counter >= 50 break-loop
        )
    "#;
    let mut parser = Parser::new(loop_code);
    let program = parser.parse().unwrap();

    benchmark("Loop evaluation (50 iterations)", 200, || {
        let mut evaluator = Evaluator::new();
        let _result = evaluator.eval_program(program.clone()).unwrap();
    });
}

fn benchmark_full_pipeline() {
    println!("🔄 FULL PIPELINE BENCHMARKS");
    println!("-------------------------");

    let fibonacci_program = r#"
        ~a is 0
        ~b is 1
        ~count is 0
        
        loop (
            if ~count >= 20 break-loop
            ~next is ~a + ~b
            ~a is ~b
            ~b is ~next
            ~count is ~count + 1
        )
    "#;

    benchmark("Full pipeline (Fibonacci 20)", 500, || {
        let mut parser = Parser::new(fibonacci_program);
        let program = parser.parse().unwrap();
        let mut evaluator = Evaluator::new();
        let _result = evaluator.eval_program(program).unwrap();
    });

    // Memory allocation stress test
    let string_program = r#"
        ~text is "Hello"
        ~counter is 0
        loop (
            ~text is ~text + " World"
            ~counter is ~counter + 1
            if ~counter >= 10 break-loop
        )
    "#;

    benchmark("String concatenation stress test", 1000, || {
        let mut parser = Parser::new(string_program);
        let program = parser.parse().unwrap();
        let mut evaluator = Evaluator::new();
        let _result = evaluator.eval_program(program).unwrap();
    });
}
