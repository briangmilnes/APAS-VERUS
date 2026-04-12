//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 50: Optimal binary search tree (sequential, persistent) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap50::OptBinSearchTreeStPer::OptBinSearchTreeStPer::*;
use apas_verus::Chap30::Probability::Probability::*;

fn bench_obst_per(c: &mut Criterion) {
    let mut group = c.benchmark_group("OBSTStPer");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));
    for &n in &[6usize, 10] {
        group.bench_with_input(BenchmarkId::new("n_keys", n), &n, |b, &n| {
            let keys: Vec<u64> = (0..n as u64).collect();
            let p = 1.0 / n as f64;
            let probs: Vec<Probability> = (0..n).map(|_| Probability::new(p)).collect();
            let obst = OBSTStPerS::<u64>::from_keys_probs(keys, probs);
            b.iter(|| obst.optimal_cost());
        });
    }
    group.finish();
}

criterion_group!(benches, bench_obst_per);
criterion_main!(benches);
