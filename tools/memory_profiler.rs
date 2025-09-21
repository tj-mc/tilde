use std::alloc::{GlobalAlloc, Layout, System};
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;
use tilde::{evaluator::Evaluator, parser::Parser};

// Custom allocator to track memory usage (zero dependencies!)
struct TrackingAllocator {
    inner: System,
    allocated: AtomicUsize,
    peak_allocated: AtomicUsize,
    allocation_count: AtomicUsize,
}

impl TrackingAllocator {
    const fn new() -> Self {
        TrackingAllocator {
            inner: System,
            allocated: AtomicUsize::new(0),
            peak_allocated: AtomicUsize::new(0),
            allocation_count: AtomicUsize::new(0),
        }
    }

    fn current_usage(&self) -> usize {
        self.allocated.load(Ordering::Relaxed)
    }

    fn peak_usage(&self) -> usize {
        self.peak_allocated.load(Ordering::Relaxed)
    }

    fn allocation_count(&self) -> usize {
        self.allocation_count.load(Ordering::Relaxed)
    }

    fn reset(&self) {
        self.allocated.store(0, Ordering::Relaxed);
        self.peak_allocated.store(0, Ordering::Relaxed);
        self.allocation_count.store(0, Ordering::Relaxed);
    }
}

unsafe impl GlobalAlloc for TrackingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe {
            let ptr = self.inner.alloc(layout);
            if !ptr.is_null() {
                let size = layout.size();
                let old_allocated = self.allocated.fetch_add(size, Ordering::Relaxed);
                let new_allocated = old_allocated + size;

                // Update peak if necessary
                let mut peak = self.peak_allocated.load(Ordering::Relaxed);
                while new_allocated > peak {
                    match self.peak_allocated.compare_exchange_weak(
                        peak,
                        new_allocated,
                        Ordering::Relaxed,
                        Ordering::Relaxed,
                    ) {
                        Ok(_) => break,
                        Err(x) => peak = x,
                    }
                }

                self.allocation_count.fetch_add(1, Ordering::Relaxed);
            }
            ptr
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe {
            self.inner.dealloc(ptr, layout);
            self.allocated.fetch_sub(layout.size(), Ordering::Relaxed);
        }
    }
}

#[global_allocator]
static ALLOCATOR: TrackingAllocator = TrackingAllocator::new();

#[derive(Debug)]
struct MemoryResult {
    test_name: String,
    duration_ns: u128,
    final_allocated: usize,
    peak_allocated: usize,
    allocation_count: usize,
    avg_allocation_size: usize,
}

fn measure_memory<F, R>(name: &str, f: F) -> (R, MemoryResult)
where
    F: FnOnce() -> R,
{
    ALLOCATOR.reset();
    let start_time = Instant::now();

    let result = f();

    let duration = start_time.elapsed();
    let final_allocated = ALLOCATOR.current_usage();
    let peak_allocated = ALLOCATOR.peak_usage();
    let allocation_count = ALLOCATOR.allocation_count();
    let avg_allocation_size = if allocation_count > 0 {
        peak_allocated / allocation_count
    } else {
        0
    };

    let memory_result = MemoryResult {
        test_name: name.to_string(),
        duration_ns: duration.as_nanos(),
        final_allocated,
        peak_allocated,
        allocation_count,
        avg_allocation_size,
    };

    println!("Memory Test: {}", name);
    println!("  Duration: {:?}", duration);
    println!("  Final allocated: {} bytes", final_allocated);
    println!("  Peak allocated: {} bytes", peak_allocated);
    println!("  Total allocations: {}", allocation_count);
    println!("  Avg allocation size: {} bytes", avg_allocation_size);
    println!();

    (result, memory_result)
}

fn main() {
    println!("🧠 Tails Language Memory Profiler");
    println!("==================================\n");

    let mut results = Vec::new();

    // Test 1: Simple parsing memory usage
    let (_, result) = measure_memory("Simple Parsing", || {
        let code = "~x is 42 + 58";
        let mut parser = Parser::new(code);
        parser.parse().unwrap()
    });
    results.push(result);

    // Test 2: Complex parsing memory usage
    let (_, result) = measure_memory("Complex Parsing", || {
        let code = r#"
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
        let mut parser = Parser::new(code);
        parser.parse().unwrap()
    });
    results.push(result);

    // Test 3: Evaluation memory usage
    let (_, result) = measure_memory("Simple Evaluation", || {
        let code = "~x is 42 + 58";
        let mut parser = Parser::new(code);
        let program = parser.parse().unwrap();
        let mut evaluator = Evaluator::new();
        evaluator.eval_program(program).unwrap()
    });
    results.push(result);

    // Test 4: String concatenation memory growth
    let (_, result) = measure_memory("String Concatenation Growth", || {
        let code = r#"
            ~text is "Start"
            ~counter is 0
            loop (
                ~text is ~text + " + More text"
                ~counter is ~counter + 1
                if ~counter >= 20 break-loop
            )
        "#;
        let mut parser = Parser::new(code);
        let program = parser.parse().unwrap();
        let mut evaluator = Evaluator::new();
        evaluator.eval_program(program).unwrap()
    });
    results.push(result);

    // Test 5: Variable storage memory usage
    let (_, result) = measure_memory("Many Variables", || {
        let mut code = String::new();
        for i in 0..100 {
            code.push_str(&format!("~var{} is {}\n", i, i * 2));
        }
        let mut parser = Parser::new(&code);
        let program = parser.parse().unwrap();
        let mut evaluator = Evaluator::new();
        evaluator.eval_program(program).unwrap()
    });
    results.push(result);

    // Write to CSV
    write_memory_csv(&results);
}

fn write_memory_csv(results: &[MemoryResult]) {
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
    let file_exists = std::path::Path::new("benchmark_results/memory.csv").exists();
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("benchmark_results/memory.csv")
        .expect("Failed to open memory CSV file");

    if !file_exists {
        writeln!(file, "timestamp,commit,test_name,duration_ns,final_allocated,peak_allocated,allocation_count,avg_allocation_size").unwrap();
    }

    // Write results
    for result in results {
        writeln!(
            file,
            "{},{},{},{},{},{},{},{}",
            timestamp,
            commit,
            result.test_name,
            result.duration_ns,
            result.final_allocated,
            result.peak_allocated,
            result.allocation_count,
            result.avg_allocation_size
        )
        .unwrap();
    }

    println!("📝 Memory results saved to benchmark_results/memory.csv");
}
