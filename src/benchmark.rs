//! Performance benchmarks for stdio-napi operations.
//!
//! This module provides benchmarking utilities for measuring the performance
//! of various stdio, terminal, and process operations.

use napi_derive::napi;
use std::io::Write;
use std::time::{Duration, Instant};

/// Benchmark result for a single operation
#[napi(object)]
pub struct BenchmarkResult {
    /// Name of the benchmark
    pub name: String,
    /// Number of iterations run
    pub iterations: u32,
    /// Total time in milliseconds
    pub total_time_ms: f64,
    /// Average time per iteration in microseconds
    pub avg_time_us: f64,
    /// Minimum time per iteration in microseconds
    pub min_time_us: f64,
    /// Maximum time per iteration in microseconds
    pub max_time_us: f64,
    /// Operations per second
    pub ops_per_second: f64,
}

/// Benchmark suite result
#[napi(object)]
pub struct BenchmarkSuiteResult {
    /// Suite name
    pub name: String,
    /// Individual benchmark results
    pub results: Vec<BenchmarkResult>,
    /// Total suite time in milliseconds
    pub total_time_ms: f64,
}

/// Runs a benchmark on print operations.
///
/// Measures the performance of stdout printing operations.
///
/// # Arguments
/// * `iterations` - Number of iterations to run (default: 1000)
///
/// # Returns
/// * `Result<BenchmarkResult, napi::Error>` - Benchmark results
///
/// # Example
/// ```javascript
/// const { benchmark_print } = require('stdio-napi');
/// const result = benchmark_print(10000);
/// console.log(`Average time: ${result.avg_time_us}µs`);
/// ```
#[napi]
pub fn benchmark_print(iterations: Option<u32>) -> napi::Result<BenchmarkResult> {
    let iters = iterations.unwrap_or(1000);
    let test_string = "Hello, World! This is a test string for benchmarking.";

    let mut times: Vec<Duration> = Vec::with_capacity(iters as usize);

    // Warmup
    for _ in 0..10 {
        print!("\r{}", test_string);
    }
    let _ = std::io::stdout().flush();

    // Benchmark
    for _ in 0..iters {
        let start = Instant::now();
        print!("\r{}", test_string);
        let _ = std::io::stdout().flush();
        times.push(start.elapsed());
    }
    println!(); // Newline after benchmark

    Ok(calculate_result("print_stdout", iters, times))
}

/// Runs a benchmark on string formatting operations.
///
/// # Arguments
/// * `iterations` - Number of iterations to run (default: 10000)
///
/// # Returns
/// * `Result<BenchmarkResult, napi::Error>` - Benchmark results
#[napi]
pub fn benchmark_string_format(iterations: Option<u32>) -> napi::Result<BenchmarkResult> {
    let iters = iterations.unwrap_or(10000);

    let mut times: Vec<Duration> = Vec::with_capacity(iters as usize);

    for _ in 0..iters {
        let start = Instant::now();
        let _ = format!("Test string: {} - {} - {}", 123, "hello", 456.789);
        times.push(start.elapsed());
    }

    Ok(calculate_result("string_format", iters, times))
}

/// Runs a benchmark on progress bar rendering.
///
/// # Arguments
/// * `iterations` - Number of iterations to run (default: 1000)
///
/// # Returns
/// * `Result<BenchmarkResult, napi::Error>` - Benchmark results
#[napi]
pub fn benchmark_progress_bar(iterations: Option<u32>) -> napi::Result<BenchmarkResult> {
    let iters = iterations.unwrap_or(1000);

    let mut times: Vec<Duration> = Vec::with_capacity(iters as usize);

    for i in 0..iters {
        let start = Instant::now();
        super::print_progress(i % 100, 100, Some(20));
        times.push(start.elapsed());
    }

    Ok(calculate_result("progress_bar", iters, times))
}

/// Runs a benchmark on spinner frame generation.
///
/// # Arguments
/// * `iterations` - Number of iterations to run (default: 10000)
///
/// # Returns
/// * `Result<BenchmarkResult, napi::Error>` - Benchmark results
#[napi]
pub fn benchmark_spinner(iterations: Option<u32>) -> napi::Result<BenchmarkResult> {
    let iters = iterations.unwrap_or(10000);

    let mut times: Vec<Duration> = Vec::with_capacity(iters as usize);

    for i in 0..iters {
        let start = Instant::now();
        let _ = super::get_spinner_frame(i);
        times.push(start.elapsed());
    }

    Ok(calculate_result("spinner_frame", iters, times))
}

/// Runs a benchmark on table rendering.
///
/// # Arguments
/// * `iterations` - Number of iterations to run (default: 100)
/// * `rows` - Number of rows in the table (default: 10)
/// * `cols` - Number of columns in the table (default: 5)
///
/// # Returns
/// * `Result<BenchmarkResult, napi::Error>` - Benchmark results
#[napi]
pub fn benchmark_table_render(
    iterations: Option<u32>,
    rows: Option<u32>,
    cols: Option<u32>,
) -> napi::Result<BenchmarkResult> {
    let iters = iterations.unwrap_or(100);
    let num_rows = rows.unwrap_or(10);
    let num_cols = cols.unwrap_or(5);

    // Create test data
    let mut table_rows: Vec<super::TableRow> = Vec::new();
    for r in 0..num_rows {
        let cells: Vec<String> = (0..num_cols).map(|c| format!("Cell-{}-{}", r, c)).collect();
        table_rows.push(super::TableRow {
            cells,
            color: None,
            is_header: Some(r == 0),
        });
    }

    let columns: Vec<super::TableColumn> = (0..num_cols)
        .map(|i| super::TableColumn {
            header: format!("Col{}", i),
            width: None,
            align: None,
            wrap: None,
            header_color: None,
            cell_color: None,
        })
        .collect();

    let config = super::TableConfig {
        columns,
        border_style: Some("single".to_string()),
        show_header: Some(true),
        padding: Some(1),
        max_width: None,
        compact: Some(false),
    };

    let mut times: Vec<Duration> = Vec::with_capacity(iters as usize);

    for _ in 0..iters {
        let start = Instant::now();
        let _ = super::render_table(table_rows.clone(), config.clone());
        times.push(start.elapsed());
    }

    Ok(calculate_result("table_render", iters, times))
}

/// Runs a benchmark on terminal size queries.
///
/// # Arguments
/// * `iterations` - Number of iterations to run (default: 1000)
///
/// # Returns
/// * `Result<BenchmarkResult, napi::Error>` - Benchmark results
#[napi]
pub fn benchmark_terminal_size(iterations: Option<u32>) -> napi::Result<BenchmarkResult> {
    let iters = iterations.unwrap_or(1000);

    let mut times: Vec<Duration> = Vec::with_capacity(iters as usize);

    for _ in 0..iters {
        let start = Instant::now();
        let _ = super::get_terminal_size();
        times.push(start.elapsed());
    }

    Ok(calculate_result("terminal_size", iters, times))
}

/// Runs a benchmark on shell escape operations.
///
/// # Arguments
/// * `iterations` - Number of iterations to run (default: 10000)
///
/// # Returns
/// * `Result<BenchmarkResult, napi::Error>` - Benchmark results
#[napi]
pub fn benchmark_shell_escape(iterations: Option<u32>) -> napi::Result<BenchmarkResult> {
    let iters = iterations.unwrap_or(10000);

    let test_inputs = vec![
        "simple",
        "with spaces",
        "with 'quotes'",
        "with \"double quotes\"",
        "with $special chars!",
        "with\nnewline",
        "unicode: 你好世界",
    ];

    let mut times: Vec<Duration> = Vec::with_capacity(iters as usize);

    for i in 0..iters {
        let input = &test_inputs[i as usize % test_inputs.len()];
        let start = Instant::now();
        let _ = super::shell_escape(input.to_string());
        times.push(start.elapsed());
    }

    Ok(calculate_result("shell_escape", iters, times))
}

/// Runs a benchmark on key combination parsing.
///
/// # Arguments
/// * `iterations` - Number of iterations to run (default: 10000)
///
/// # Returns
/// * `Result<BenchmarkResult, napi::Error>` - Benchmark results
#[napi]
pub fn benchmark_key_parsing(iterations: Option<u32>) -> napi::Result<BenchmarkResult> {
    let iters = iterations.unwrap_or(10000);

    let test_combinations = vec![
        "a",
        "ctrl+c",
        "ctrl+shift+s",
        "alt+enter",
        "ctrl+alt+delete",
        "shift+tab",
        "f1",
        "ctrl+f5",
    ];

    let mut times: Vec<Duration> = Vec::with_capacity(iters as usize);

    for i in 0..iters {
        let combo = test_combinations[i as usize % test_combinations.len()];
        let start = Instant::now();
        // Simple key combination parsing benchmark
        let parts: Vec<&str> = combo.split('+').collect();
        let mut _ctrl = false;
        let mut _alt = false;
        let mut _shift = false;
        let mut _key = String::new();
        for part in parts {
            let trimmed = part.trim().to_lowercase();
            match trimmed.as_str() {
                "ctrl" | "control" => _ctrl = true,
                "alt" | "meta" => _alt = true,
                "shift" => _shift = true,
                k => _key = k.to_string(),
            }
        }
        std::hint::black_box((_ctrl, _alt, _shift, _key));
        times.push(start.elapsed());
    }

    Ok(calculate_result("key_parsing", iters, times))
}

/// Runs a comprehensive benchmark suite.
///
/// # Arguments
/// * `iterations` - Base number of iterations for each benchmark
///
/// # Returns
/// * `Result<BenchmarkSuiteResult, napi::Error>` - Complete benchmark suite results
///
/// # Example
/// ```javascript
/// const { run_benchmark_suite } = require('stdio-napi');
/// const results = run_benchmark_suite(1000);
/// console.log(`Suite: ${results.name}`);
/// results.results.forEach(r => {
///   console.log(`${r.name}: ${r.avg_time_us}µs avg`);
/// });
/// ```
#[napi]
pub fn run_benchmark_suite(iterations: Option<u32>) -> napi::Result<BenchmarkSuiteResult> {
    let iters = iterations.unwrap_or(1000);
    let suite_start = Instant::now();

    // Run all benchmarks
    let results = vec![
        benchmark_string_format(Some(iters * 10))?,
        benchmark_spinner(Some(iters * 10))?,
        benchmark_progress_bar(Some(iters))?,
        benchmark_table_render(Some(iters / 10), Some(10), Some(5))?,
        benchmark_terminal_size(Some(iters))?,
        benchmark_shell_escape(Some(iters * 10))?,
        benchmark_key_parsing(Some(iters * 10))?,
    ];

    // Note: print benchmark is optional as it produces output
    // results.push(benchmark_print(Some(iters))?);

    let total_time = suite_start.elapsed();

    Ok(BenchmarkSuiteResult {
        name: "stdio-napi benchmarks".to_string(),
        results,
        total_time_ms: total_time.as_secs_f64() * 1000.0,
    })
}

/// Benchmarks memory allocation for string operations.
///
/// # Arguments
/// * `iterations` - Number of iterations to run (default: 10000)
/// * `string_size` - Size of strings to allocate (default: 100 bytes)
///
/// # Returns
/// * `Result<BenchmarkResult, napi::Error>` - Benchmark results
#[napi]
pub fn benchmark_string_allocation(
    iterations: Option<u32>,
    string_size: Option<u32>,
) -> napi::Result<BenchmarkResult> {
    let iters = iterations.unwrap_or(10000);
    let size = string_size.unwrap_or(100) as usize;

    let mut times: Vec<Duration> = Vec::with_capacity(iters as usize);

    for _ in 0..iters {
        let start = Instant::now();
        let s: String = (0..size).map(|_| 'x').collect();
        std::hint::black_box(s);
        times.push(start.elapsed());
    }

    Ok(calculate_result("string_allocation", iters, times))
}

/// Benchmarks vector operations.
///
/// # Arguments
/// * `iterations` - Number of iterations to run (default: 10000)
/// * `vector_size` - Size of vectors to create (default: 100)
///
/// # Returns
/// * `Result<BenchmarkResult, napi::Error>` - Benchmark results
#[napi]
pub fn benchmark_vector_operations(
    iterations: Option<u32>,
    vector_size: Option<u32>,
) -> napi::Result<BenchmarkResult> {
    let iters = iterations.unwrap_or(10000);
    let size = vector_size.unwrap_or(100) as usize;

    let mut times: Vec<Duration> = Vec::with_capacity(iters as usize);

    for _ in 0..iters {
        let start = Instant::now();
        let v: Vec<i32> = (0..size as i32).collect();
        let sum: i32 = v.iter().sum();
        std::hint::black_box(sum);
        times.push(start.elapsed());
    }

    Ok(calculate_result("vector_operations", iters, times))
}

/// Benchmarks hashmap operations.
///
/// # Arguments
/// * `iterations` - Number of iterations to run (default: 10000)
/// * `map_size` - Size of hashmap to create (default: 100)
///
/// # Returns
/// * `Result<BenchmarkResult, napi::Error>` - Benchmark results
#[napi]
pub fn benchmark_hashmap_operations(
    iterations: Option<u32>,
    map_size: Option<u32>,
) -> napi::Result<BenchmarkResult> {
    use std::collections::HashMap;

    let iters = iterations.unwrap_or(10000);
    let size = map_size.unwrap_or(100) as usize;

    let mut times: Vec<Duration> = Vec::with_capacity(iters as usize);

    for i in 0..iters {
        let start = Instant::now();
        let mut map: HashMap<String, i32> = HashMap::new();
        for j in 0..size {
            map.insert(format!("key_{}_{}", i, j), j as i32);
        }
        let _ = map.get(&format!("key_{}_{}", i, size / 2));
        std::hint::black_box(map);
        times.push(start.elapsed());
    }

    Ok(calculate_result("hashmap_operations", iters, times))
}

// Helper function to calculate benchmark results
fn calculate_result(name: &str, iterations: u32, times: Vec<Duration>) -> BenchmarkResult {
    let total_time: Duration = times.iter().sum();
    let total_time_ms = total_time.as_secs_f64() * 1000.0;

    let times_us: Vec<f64> = times
        .iter()
        .map(|d| d.as_secs_f64() * 1_000_000.0)
        .collect();

    let avg_time_us = times_us.iter().sum::<f64>() / times_us.len() as f64;
    let min_time_us = times_us.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let max_time_us = times_us.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

    let ops_per_second = if total_time.as_secs_f64() > 0.0 {
        iterations as f64 / total_time.as_secs_f64()
    } else {
        0.0
    };

    BenchmarkResult {
        name: name.to_string(),
        iterations,
        total_time_ms,
        avg_time_us,
        min_time_us,
        max_time_us,
        ops_per_second,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_benchmark_string_format() {
        let result = benchmark_string_format(Some(100)).unwrap();
        assert_eq!(result.name, "string_format");
        assert_eq!(result.iterations, 100);
        assert!(result.total_time_ms > 0.0);
        assert!(result.avg_time_us > 0.0);
    }

    #[test]
    fn test_benchmark_spinner() {
        let result = benchmark_spinner(Some(100)).unwrap();
        assert_eq!(result.name, "spinner_frame");
        assert_eq!(result.iterations, 100);
        assert!(result.ops_per_second > 0.0);
    }

    #[test]
    fn test_benchmark_table_render() {
        let result = benchmark_table_render(Some(10), Some(5), Some(3)).unwrap();
        assert_eq!(result.name, "table_render");
        assert_eq!(result.iterations, 10);
    }

    #[test]
    fn test_benchmark_shell_escape() {
        let result = benchmark_shell_escape(Some(100)).unwrap();
        assert_eq!(result.name, "shell_escape");
    }

    #[test]
    fn test_benchmark_key_parsing() {
        let result = benchmark_key_parsing(Some(100)).unwrap();
        assert_eq!(result.name, "key_parsing");
    }

    #[test]
    fn test_benchmark_string_allocation() {
        let result = benchmark_string_allocation(Some(100), Some(50)).unwrap();
        assert_eq!(result.name, "string_allocation");
    }

    #[test]
    fn test_benchmark_vector_operations() {
        let result = benchmark_vector_operations(Some(100), Some(50)).unwrap();
        assert_eq!(result.name, "vector_operations");
    }

    #[test]
    fn test_benchmark_hashmap_operations() {
        let result = benchmark_hashmap_operations(Some(100), Some(50)).unwrap();
        assert_eq!(result.name, "hashmap_operations");
    }

    #[test]
    fn test_run_benchmark_suite() {
        let result = run_benchmark_suite(Some(100)).unwrap();
        assert!(!result.results.is_empty());
        assert!(result.total_time_ms > 0.0);
    }

    #[test]
    fn test_calculate_result() {
        let times = vec![
            Duration::from_micros(100),
            Duration::from_micros(200),
            Duration::from_micros(150),
        ];

        let result = calculate_result("test", 3, times);
        assert_eq!(result.name, "test");
        assert_eq!(result.iterations, 3);
        assert_eq!(result.min_time_us, 100.0);
        assert_eq!(result.max_time_us, 200.0);
    }
}
