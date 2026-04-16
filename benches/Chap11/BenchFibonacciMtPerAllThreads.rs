// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Chapter 11: Parallel Fibonacci (multi-threaded, persistent) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap11::FibonacciMtPerAllThreads::FibonacciMtPerAllThreads::*;
use apas_verus::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::set_parallelism;

fn bench_fib_mt(c: &mut Criterion) {
    set_parallelism(10);
    let mut group = c.benchmark_group("FibMtPerAllThreads");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));
    // Keep n small — parallel recursive fib spawns many threads.
    for &n in &[10u64, 20] {
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter(|| fib(n));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_fib_mt);
criterion_main!(benches);
