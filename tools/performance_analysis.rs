use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Simple performance analysis tool to identify bottlenecks
/// without adding complex dependencies
pub struct PerformanceAnalyzer {
    measurements: HashMap<String, Vec<Duration>>,
}

impl PerformanceAnalyzer {
    pub fn new() -> Self {
        PerformanceAnalyzer {
            measurements: HashMap::new(),
        }
    }

    pub fn measure<F, R>(&mut self, operation: &str, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        let start = Instant::now();
        let result = f();
        let elapsed = start.elapsed();

        self.measurements
            .entry(operation.to_string())
            .or_insert_with(Vec::new)
            .push(elapsed);

        result
    }

    pub fn report(&self, title: &str) {
        println!("\n=== {} ===", title);

        let mut total_time = Duration::from_nanos(0);
        let mut sorted_ops: Vec<_> = self.measurements.iter().collect();

        // Sort by total time spent (descending)
        sorted_ops.sort_by(|a, b| {
            let sum_b: Duration = b.1.iter().sum();
            let sum_a: Duration = a.1.iter().sum();
            sum_b.cmp(&sum_a)
        });

        for (operation, times) in sorted_ops {
            let total: Duration = times.iter().sum();
            let avg = if !times.is_empty() {
                total / times.len() as u32
            } else {
                Duration::from_nanos(0)
            };
            let count = times.len();

            total_time += total;

            println!(
                "{:<30} {:>8.2}ms total ({:>6.2}ms avg × {} calls)",
                operation,
                total.as_secs_f64() * 1000.0,
                avg.as_secs_f64() * 1000.0,
                count
            );
        }

        println!("{}", "─".repeat(60));
        println!(
            "{:<30} {:>8.2}ms",
            "TOTAL TIME",
            total_time.as_secs_f64() * 1000.0
        );
        println!();
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.measurements.clear();
    }
}

impl Default for PerformanceAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
