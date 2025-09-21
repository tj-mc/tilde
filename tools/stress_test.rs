use std::time::Instant;
use tilde::{evaluator::Evaluator, parser::Parser};

fn main() {
    println!("💥 Tails Language Stress Tests");
    println!("==============================\n");

    // Memory stress tests
    stress_test_memory();

    // Performance stress tests
    stress_test_performance();

    // Edge case stress tests
    stress_test_edge_cases();

    println!("✅ All stress tests completed!");
}

fn stress_test_memory() {
    println!("🧠 MEMORY STRESS TESTS");
    println!("---------------------");

    // Large variable count
    println!("Testing large variable count...");
    let start = Instant::now();
    let mut large_program = String::new();
    for i in 0..1000 {
        large_program.push_str(&format!("~var{} is {}\n", i, i * 2));
    }

    let mut parser = Parser::new(&large_program);
    let program = parser.parse().expect("Failed to parse large program");
    let mut evaluator = Evaluator::new();
    evaluator
        .eval_program(program)
        .expect("Failed to evaluate large program");

    println!("  ✅ 1000 variables: {:?}", start.elapsed());

    // Deep string concatenation
    println!("Testing deep string concatenation...");
    let start = Instant::now();
    let concat_program = r#"
        ~text is "Start"
        ~counter is 0
        loop (
            ~text is ~text + " + More text here with some length"
            ~counter is ~counter + 1
            if ~counter >= 100 break-loop
        )
    "#;

    let mut parser = Parser::new(concat_program);
    let program = parser.parse().expect("Failed to parse concat program");
    let mut evaluator = Evaluator::new();
    evaluator
        .eval_program(program)
        .expect("Failed to evaluate concat program");

    println!("  ✅ 100 string concatenations: {:?}", start.elapsed());

    // Deep loop nesting stress
    println!("Testing deep computation...");
    let start = Instant::now();
    let computation_program = r#"
        ~result is 1
        ~outer is 0
        loop (
            ~inner is 0
            loop (
                ~result is ~result + 1
                ~inner is ~inner + 1
                if ~inner >= 50 break-loop
            )
            ~outer is ~outer + 1
            if ~outer >= 20 break-loop
        )
    "#;

    let mut parser = Parser::new(computation_program);
    let program = parser.parse().expect("Failed to parse computation program");
    let mut evaluator = Evaluator::new();
    evaluator
        .eval_program(program)
        .expect("Failed to evaluate computation program");

    println!(
        "  ✅ Nested loops (20x50 = 1000 iterations): {:?}",
        start.elapsed()
    );
    println!();
}

fn stress_test_performance() {
    println!("⚡ PERFORMANCE STRESS TESTS");
    println!("-------------------------");

    // Fibonacci stress test
    println!("Testing Fibonacci sequence calculation...");
    let start = Instant::now();
    let fib_program = r#"
        ~a is 0
        ~b is 1
        ~count is 0
        ~target is 500
        
        loop (
            if ~count >= ~target break-loop
            ~next is ~a + ~b
            ~a is ~b
            ~b is ~next
            ~count is ~count + 1
        )
    "#;

    let mut parser = Parser::new(fib_program);
    let program = parser.parse().expect("Failed to parse fibonacci");
    let mut evaluator = Evaluator::new();
    evaluator
        .eval_program(program)
        .expect("Failed to evaluate fibonacci");

    println!("  ✅ Fibonacci 500: {:?}", start.elapsed());

    // Comparison operations stress
    println!("Testing comparison operations...");
    let start = Instant::now();
    let comparison_program = r#"
        ~counter is 0
        ~matches is 0
        loop (
            if ~counter == 100 ~matches is ~matches + 1
            if ~counter > 100 ~matches is ~matches + 1  
            if ~counter < 100 ~matches is ~matches + 1
            if ~counter >= 100 ~matches is ~matches + 1
            if ~counter <= 100 ~matches is ~matches + 1
            if ~counter != 100 ~matches is ~matches + 1
            ~counter is ~counter + 1
            if ~counter >= 200 break-loop
        )
    "#;

    let mut parser = Parser::new(comparison_program);
    let program = parser.parse().expect("Failed to parse comparisons");
    let mut evaluator = Evaluator::new();
    evaluator
        .eval_program(program)
        .expect("Failed to evaluate comparisons");

    println!("  ✅ 1200 comparison operations: {:?}", start.elapsed());
    println!();
}

fn stress_test_edge_cases() {
    println!("🔬 EDGE CASE STRESS TESTS");
    println!("------------------------");

    // Very large numbers
    println!("Testing large number operations...");
    let start = Instant::now();
    let large_num_program = r#"
        ~big is 999999999
        ~result is ~big + ~big
        ~result is ~result + ~big
        ~comparison is ~result > ~big
    "#;

    let mut parser = Parser::new(large_num_program);
    let program = parser.parse().expect("Failed to parse large numbers");
    let mut evaluator = Evaluator::new();
    evaluator
        .eval_program(program)
        .expect("Failed to evaluate large numbers");

    println!("  ✅ Large number operations: {:?}", start.elapsed());

    // Empty and null operations
    println!("Testing edge case values...");
    let start = Instant::now();
    let edge_program = r#"
        ~empty_string is ""
        ~zero is 0
        ~comparison1 is ~empty_string == ""
        ~comparison2 is ~zero == 0
        ~truthiness1 is ~empty_string
        ~truthiness2 is ~zero
    "#;

    let mut parser = Parser::new(edge_program);
    let program = parser.parse().expect("Failed to parse edge cases");
    let mut evaluator = Evaluator::new();
    evaluator
        .eval_program(program)
        .expect("Failed to evaluate edge cases");

    println!("  ✅ Edge case values: {:?}", start.elapsed());

    // Complex expression parsing
    println!("Testing complex expressions...");
    let start = Instant::now();
    let complex_expr_program = r#"
        ~a is 10
        ~b is 20  
        ~c is 30
        ~result is (~a + ~b) > (~c - ~a) 
        ~result2 is (~a + (~b - ~c)) == (~a + ~b - ~c)
        ~result3 is (~a < ~b) == (~b > ~a)
    "#;

    let mut parser = Parser::new(complex_expr_program);
    let program = parser.parse().expect("Failed to parse complex expressions");
    let mut evaluator = Evaluator::new();
    evaluator
        .eval_program(program)
        .expect("Failed to evaluate complex expressions");

    println!("  ✅ Complex expressions: {:?}", start.elapsed());
    println!();
}
