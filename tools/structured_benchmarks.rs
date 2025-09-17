use std::fs::OpenOptions;
use std::io::Write;
use std::time::Instant;
use tails::{evaluator::Evaluator, lexer::Lexer, parser::Parser};

#[derive(Debug)]
struct BenchmarkResult {
    test_name: String,
    iterations: u32,
    #[allow(dead_code)]
    total_time_ns: u128,
    avg_time_ns: u128,
    ops_per_sec: u64,
}

impl BenchmarkResult {
    fn new(name: &str, iterations: u32, total_time: std::time::Duration) -> Self {
        let total_ns = total_time.as_nanos();
        let avg_ns = total_ns / iterations as u128;
        let ops_per_sec = if avg_ns > 0 {
            (1_000_000_000u128 / avg_ns) as u64
        } else {
            0
        };

        BenchmarkResult {
            test_name: name.to_string(),
            iterations,
            total_time_ns: total_ns,
            avg_time_ns: avg_ns,
            ops_per_sec,
        }
    }
}

fn benchmark_operation<F>(name: &str, iterations: u32, mut f: F) -> BenchmarkResult
where
    F: FnMut(),
{
    // Warmup
    for _ in 0..std::cmp::min(100, iterations / 10) {
        f();
    }

    // Actual benchmark
    let start = Instant::now();
    for _ in 0..iterations {
        f();
    }
    let duration = start.elapsed();

    BenchmarkResult::new(name, iterations, duration)
}

fn main() {
    let mut results = Vec::new();

    println!("📊 Running Structured Benchmarks...");

    // Lexer benchmarks
    let simple_code = "~x is 42";
    let result = benchmark_operation("lexer_simple", 10000, || {
        let mut lexer = Lexer::new(simple_code);
        let _tokens = lexer.tokenize();
    });
    results.push(result);

    let complex_code = r#"
        ~name is ask "Enter name: "
        if ~name == "test" say "Hello test" else say "Hello stranger"
        loop (
            ~counter is ~counter + 1
            if ~counter >= 10 break-loop
        )
    "#;
    let result = benchmark_operation("lexer_complex", 1000, || {
        let mut lexer = Lexer::new(complex_code);
        let _tokens = lexer.tokenize();
    });
    results.push(result);

    // Parser benchmarks
    let result = benchmark_operation("parser_simple", 5000, || {
        let mut parser = Parser::new("~x is 42 + 58");
        let _program = parser.parse().unwrap();
    });
    results.push(result);

    let result = benchmark_operation("parser_complex", 1000, || {
        let mut parser = Parser::new(complex_code);
        let _program = parser.parse().unwrap();
    });
    results.push(result);

    // Evaluator benchmarks
    let mut simple_parser = Parser::new("~x is 42 + 58");
    let simple_program = simple_parser.parse().unwrap();
    let result = benchmark_operation("evaluator_simple", 5000, || {
        let mut evaluator = Evaluator::new();
        let _result = evaluator.eval_program(simple_program.clone()).unwrap();
    });
    results.push(result);

    let fib_code = r#"
        ~a is 0
        ~b is 1
        ~count is 0
        loop (
            if ~count >= 10 break-loop
            ~next is ~a + ~b
            ~a is ~b
            ~b is ~next
            ~count is ~count + 1
        )
    "#;
    let mut fib_parser = Parser::new(fib_code);
    let fib_program = fib_parser.parse().unwrap();
    let result = benchmark_operation("evaluator_fibonacci10", 1000, || {
        let mut evaluator = Evaluator::new();
        let _result = evaluator.eval_program(fib_program.clone()).unwrap();
    });
    results.push(result);

    // Display results
    println!("\n📈 Benchmark Results:");
    println!("=====================");
    for result in &results {
        println!(
            "{:<25} {:>8} ops | {:>6.2}μs avg | {:>8} ops/sec",
            result.test_name,
            result.iterations,
            result.avg_time_ns as f64 / 1000.0,
            result.ops_per_sec
        );
    }

    // Write to CSV for tracking over time
    write_csv_results(&results);
    write_json_results(&results);
}

fn write_csv_results(results: &[BenchmarkResult]) {
    use std::process::Command;

    // Get git info
    let commit = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // Create CSV header if file doesn't exist
    let file_exists = std::path::Path::new("benchmark_results/benchmarks.csv").exists();
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("benchmark_results/benchmarks.csv")
        .expect("Failed to open CSV file");

    if !file_exists {
        writeln!(
            file,
            "timestamp,commit,test_name,iterations,avg_time_ns,ops_per_sec"
        )
        .unwrap();
    }

    // Write results
    for result in results {
        writeln!(
            file,
            "{},{},{},{},{},{}",
            timestamp,
            commit,
            result.test_name,
            result.iterations,
            result.avg_time_ns,
            result.ops_per_sec
        )
        .unwrap();
    }

    println!("\n📝 Results appended to benchmark_results/benchmarks.csv");
}

fn write_json_results(results: &[BenchmarkResult]) {
    use std::process::Command;

    let commit = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let filename = format!("benchmark_results/bench_{}.json", timestamp);
    let mut file = std::fs::File::create(&filename).expect("Failed to create JSON file");

    writeln!(file, "{{").unwrap();
    writeln!(file, r#"  "timestamp": {},"#, timestamp).unwrap();
    writeln!(file, r#"  "commit": "{}","#, commit).unwrap();
    writeln!(file, r#"  "rustc_version": "rustc --version","#).unwrap();
    writeln!(file, r#"  "results": ["#).unwrap();

    for (i, result) in results.iter().enumerate() {
        let comma = if i < results.len() - 1 { "," } else { "" };
        writeln!(file, r#"    {{"#).unwrap();
        writeln!(file, r#"      "test_name": "{}"."#, result.test_name).unwrap();
        writeln!(file, r#"      "iterations": {},"#, result.iterations).unwrap();
        writeln!(file, r#"      "avg_time_ns": {},"#, result.avg_time_ns).unwrap();
        writeln!(file, r#"      "ops_per_sec": {}"#, result.ops_per_sec).unwrap();
        writeln!(file, r#"    }}{}"#, comma).unwrap();
    }

    writeln!(file, r#"  ]"#).unwrap();
    writeln!(file, r#"}}"#).unwrap();

    println!("📝 Detailed results saved to {}", filename);
}
