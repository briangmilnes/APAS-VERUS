//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 52: Adjacency-matrix graph (multi-threaded, ephemeral) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use apas_verus::Chap52::AdjMatrixGraphMtEph::AdjMatrixGraphMtEph::*;

fn bench_adjmatrixgraph_mt_eph_set_edge(c: &mut Criterion) {
    let mut group = c.benchmark_group("AdjMatrixGraphMtEphSetEdge");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(400));
    for &n in &[16usize, 32] {
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter_batched(
                || <AdjMatrixGraphMtEph as AdjMatrixGraphMtEphTrait>::new(n),
                |mut g| {
                    for i in 0..n { g.set_edge(i, (i + 1) % n, true); }
                    g
                },
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

fn bench_adjmatrixgraph_mt_eph_neighbors(c: &mut Criterion) {
    let mut group = c.benchmark_group("AdjMatrixGraphMtEphNeighbors");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(200));
    for &n in &[16usize, 32] {
        let g = <AdjMatrixGraphMtEph as AdjMatrixGraphMtEphTrait>::new(n);
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, _| {
            b.iter(|| g.out_neighbors(0));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_adjmatrixgraph_mt_eph_set_edge, bench_adjmatrixgraph_mt_eph_neighbors);
criterion_main!(benches);
