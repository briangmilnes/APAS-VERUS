//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 52: Adjacency-matrix graph (sequential, persistent) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap52::AdjMatrixGraphStPer::AdjMatrixGraphStPer::*;

fn bench_adjmatrixgraph_stper_neighbors(c: &mut Criterion) {
    let mut group = c.benchmark_group("AdjMatrixGraphStPerNeighbors");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(200));
    for &n in &[16usize, 32] {
        let g = <AdjMatrixGraphStPer as AdjMatrixGraphStPerTrait>::new(n);
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, _| {
            b.iter(|| g.out_neighbors(0));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_adjmatrixgraph_stper_neighbors);
criterion_main!(benches);
