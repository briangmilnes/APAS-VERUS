//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 52: Adjacency-table graph (sequential, persistent) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap52::AdjTableGraphStPer::AdjTableGraphStPer::*;

fn bench_adjtablegraph_stper_build(c: &mut Criterion) {
    let mut group = c.benchmark_group("AdjTableGraphStPerBuild");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(400));
    for &n in &[16usize, 32] {
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter(|| {
                let mut g = <AdjTableGraphStPer<usize> as AdjTableGraphStPerTrait<usize>>::empty();
                for i in 0..n { g = g.insert_edge(i, (i + 1) % n); }
                g
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_adjtablegraph_stper_build);
criterion_main!(benches);
