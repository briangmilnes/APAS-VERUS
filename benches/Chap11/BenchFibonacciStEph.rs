//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 11: Fibonacci benchmarks (iterative vs recursive memoized).

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap11::FibonacciStEph::FibonacciStEph::*;

fn bench_fib_iterative(c: &mut Criterion) {
    let mut group = c.benchmark_group("FibIterative");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(200));
    for &n in &[30u64, 46] {
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter(|| fib(n));
        });
    }
    group.finish();
}

fn bench_fib_recursive(c: &mut Criterion) {
    let mut group = c.benchmark_group("FibRecursive");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(200));
    // Recursive fib is exponential — keep n small.
    for &n in &[20u64, 30] {
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter(|| fib_recursive(n));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_fib_iterative, bench_fib_recursive);
criterion_main!(benches);
