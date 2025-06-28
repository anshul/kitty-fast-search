use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use kitty_fast_search::search::SearchEngine;
use std::time::Duration;

fn generate_test_data(lines: usize) -> String {
    let mut data = String::with_capacity(lines * 50);
    for i in 0..lines {
        data.push_str(&format!("Line {}: Some test data with error messages and logs\n", i));
        if i % 100 == 0 {
            data.push_str("ERROR: This is an error message\n");
        }
        if i % 200 == 0 {
            data.push_str("WARN: This is a warning message\n");
        }
        if i % 500 == 0 {
            data.push_str(&format!("CRITICAL: System failure at line {}\n", i));
        }
    }
    data
}

fn generate_log_data(lines: usize) -> String {
    let mut data = String::with_capacity(lines * 100);
    let levels = ["INFO", "WARN", "ERROR", "DEBUG", "TRACE"];
    let services = ["web-server", "database", "cache", "auth-service", "api-gateway"];
    
    for i in 0..lines {
        let level = levels[i % levels.len()];
        let service = services[i % services.len()];
        data.push_str(&format!(
            "2023-12-{:02} {:02}:{:02}:{:02} {} [{}] Request processed in {}ms\n",
            (i % 30) + 1,
            (i % 24),
            (i % 60),
            (i % 60),
            level,
            service,
            i % 1000
        ));
    }
    data
}

fn search_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("search_performance");
    group.sample_size(50);
    group.measurement_time(Duration::from_secs(10));
    
    // Test different buffer sizes
    for &size in [1_000, 10_000, 100_000, 500_000, 1_000_000].iter() {
        let data = generate_test_data(size);
        let log_data = generate_log_data(size);
        
        group.throughput(Throughput::Elements(size as u64));
        
        // Simple literal search
        group.bench_with_input(
            BenchmarkId::new("literal_search", size),
            &data,
            |b, data| {
                let engine = SearchEngine::new(size * 2, false, false).unwrap();
                b.iter(|| {
                    black_box(engine.search_text(black_box(data), black_box("ERROR")))
                })
            },
        );
        
        // Case-insensitive search
        group.bench_with_input(
            BenchmarkId::new("case_insensitive", size),
            &data,
            |b, data| {
                let engine = SearchEngine::new(size * 2, false, false).unwrap();
                b.iter(|| {
                    black_box(engine.search_text(black_box(data), black_box("error")))
                })
            },
        );
        
        // Regex search
        group.bench_with_input(
            BenchmarkId::new("regex_search", size),
            &data,
            |b, data| {
                let engine = SearchEngine::new(size * 2, false, true).unwrap();
                b.iter(|| {
                    black_box(engine.search_text(black_box(data), black_box(r"ERROR|WARN|CRITICAL")))
                })
            },
        );
        
        // Complex regex search on log data
        group.bench_with_input(
            BenchmarkId::new("complex_regex", size),
            &log_data,
            |b, data| {
                let engine = SearchEngine::new(size * 2, true, true).unwrap();
                b.iter(|| {
                    black_box(engine.search_text(
                        black_box(data), 
                        black_box(r"\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2} (ERROR|WARN)")
                    ))
                })
            },
        );
        
        // Search with caching (second search should be faster)
        group.bench_with_input(
            BenchmarkId::new("cached_search", size),
            &data,
            |b, data| {
                let engine = SearchEngine::new(size * 2, false, false).unwrap();
                // Prime the cache
                let _ = engine.search_text(data, "ERROR");
                b.iter(|| {
                    black_box(engine.search_text(black_box(data), black_box("ERROR")))
                })
            },
        );
    }
    
    group.finish();
}

fn memory_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_usage");
    group.sample_size(20);
    
    // Test memory efficiency with large buffers
    for &size in [100_000, 500_000, 1_000_000, 2_000_000].iter() {
        let data = generate_log_data(size);
        
        group.bench_with_input(
            BenchmarkId::new("large_buffer", size),
            &data,
            |b, data| {
                b.iter(|| {
                    let engine = SearchEngine::new(size * 2, false, false).unwrap();
                    let results = engine.search_text(black_box(data), black_box("ERROR")).unwrap();
                    black_box(results.len());
                })
            },
        );
    }
    
    group.finish();
}

fn pattern_complexity_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("pattern_complexity");
    let data = generate_log_data(100_000);
    
    let patterns = vec![
        ("simple", "ERROR"),
        ("word_boundary", r"\bERROR\b"),
        ("multiple_terms", r"ERROR|WARN|INFO"),
        ("timestamp", r"\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}"),
        ("complex", r"(\d{4}-\d{2}-\d{2}).*?(ERROR|CRITICAL).*?(\d+ms)"),
    ];
    
    for (name, pattern) in patterns {
        group.bench_with_input(
            BenchmarkId::new("pattern", name),
            &(data.as_str(), pattern),
            |b, (data, pattern)| {
                let engine = SearchEngine::new(200_000, true, true).unwrap();
                b.iter(|| {
                    black_box(engine.search_text(black_box(data), black_box(pattern)))
                })
            },
        );
    }
    
    group.finish();
}

criterion_group!(benches, search_benchmark, memory_benchmark, pattern_complexity_benchmark);
criterion_main!(benches);