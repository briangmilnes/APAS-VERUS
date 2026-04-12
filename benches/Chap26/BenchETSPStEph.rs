//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 26: Euclidean TSP heuristic (sequential) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap26::ETSPStEph::ETSPStEph::*;

fn make_points(n: usize) -> Vec<Point> {
    (0..n).map(|i| Point { x: i as f64, y: (i * i) as f64 % 100.0 }).collect()
}

fn bench_etsp(c: &mut Criterion) {
    let mut group = c.benchmark_group("ETSPStEph");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(400));
    for &n in &[8usize, 16] {
        let pts = make_points(n);
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, _| {
            b.iter(|| <Vec<Point> as ETSPStTrait>::etsp(&pts));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_etsp);
criterion_main!(benches);
