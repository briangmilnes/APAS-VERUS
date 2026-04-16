// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Chapter 50: Optimal binary search tree (multi-threaded, persistent) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use apas_verus::Chap50::OptBinSearchTreeMtPer::OptBinSearchTreeMtPer::*;
use apas_verus::Chap30::Probability::Probability::*;
use apas_verus::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::set_parallelism;

fn bench_obst_mt_per(c: &mut Criterion) {
    set_parallelism(10);
    let mut group = c.benchmark_group("OBSTMtPer");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(400));
    for &n in &[6usize, 10] {
        group.bench_with_input(BenchmarkId::new("n_keys", n), &n, |b, &n| {
            b.iter_batched(
                || {
                    let keys: Vec<u64> = (0..n as u64).collect();
                    let p = 1.0 / n as f64;
                    let probs: Vec<Probability> = (0..n).map(|_| Probability::new(p)).collect();
                    OBSTMtPerS::<u64>::from_keys_probs(keys, probs)
                },
                |obst| obst.optimal_cost(),
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

criterion_group!(benches, bench_obst_mt_per);
criterion_main!(benches);
