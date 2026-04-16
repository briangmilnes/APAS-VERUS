// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Chapter 26: Euclidean TSP heuristic (multi-threaded, ephemeral) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap26::ETSPMtEph::ETSPMtEph::*;
use apas_verus::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::set_parallelism;

fn make_points(n: usize) -> Vec<Point> {
    (0..n).map(|i| Point { x: i as f64, y: (i * i) as f64 % 100.0 }).collect()
}

fn bench_etsp_parallel(c: &mut Criterion) {
    set_parallelism(10);
    let mut group = c.benchmark_group("ETSPMtEph");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(400));
    for &n in &[8usize, 16] {
        let pts = make_points(n);
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, _| {
            b.iter(|| <Vec<Point> as ETSPMtTrait>::etsp_parallel(&pts));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_etsp_parallel);
criterion_main!(benches);
