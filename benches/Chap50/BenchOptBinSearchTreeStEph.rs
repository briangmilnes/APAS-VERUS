//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 50: Optimal binary search tree (sequential, ephemeral) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use apas_verus::Chap50::OptBinSearchTreeStEph::OptBinSearchTreeStEph::*;
use apas_verus::Chap30::Probability::Probability::*;

fn bench_obst(c: &mut Criterion) {
    let mut group = c.benchmark_group("OBSTStEph");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));
    for &n in &[6usize, 10] {
        group.bench_with_input(BenchmarkId::new("n_keys", n), &n, |b, &n| {
            b.iter_batched(
                || {
                    let keys: Vec<u64> = (0..n as u64).collect();
                    let p = 1.0 / n as f64;
                    let probs: Vec<Probability> = (0..n).map(|_| Probability::new(p)).collect();
                    OBSTStEphS::<u64>::from_keys_probs(keys, probs)
                },
                |mut obst| obst.optimal_cost(),
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

criterion_group!(benches, bench_obst);
criterion_main!(benches);
