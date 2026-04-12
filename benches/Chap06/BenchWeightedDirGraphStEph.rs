//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 06: Weighted directed graph (I64 weights) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap06::WeightedDirGraphStEphI64::WeightedDirGraphStEphI64::*;
use apas_verus::Chap05::SetStEph::SetStEph::*;
use apas_verus::Types::Types::WeightedEdge;
use apas_verus::SetLit;

fn build_weighted_path(n: usize) -> WeightedDirGraphStEphI64<u64> {
    let mut verts: SetStEph<u64> = SetLit![];
    let mut edges: SetStEph<WeightedEdge<u64, i64>> = SetLit![];
    for i in 0..n as u64 { let _ = verts.insert(i); }
    for i in 0..(n as u64 - 1) { let _ = edges.insert(WeightedEdge(i, i + 1, 1i64)); }
    WeightedDirGraphStEphI64::<u64>::from_weighed_edges(verts, edges)
}

fn bench_weighted_graph_build(c: &mut Criterion) {
    let mut group = c.benchmark_group("WeightedDirGraphBuild");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(200));
    for &n in &[32usize, 128] {
        group.bench_with_input(BenchmarkId::new("path_n", n), &n, |b, &n| {
            b.iter(|| build_weighted_path(n));
        });
    }
    group.finish();
}

fn bench_weighted_graph_neighbors(c: &mut Criterion) {
    let mut group = c.benchmark_group("WeightedDirGraphNeighbors");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(200));
    for &n in &[32usize, 128] {
        let g = build_weighted_path(n);
        let mid = n as u64 / 2;
        group.bench_with_input(BenchmarkId::new("out_n", n), &n, |b, _| {
            b.iter(|| g.out_neighbors_weighed(&mid));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_weighted_graph_build, bench_weighted_graph_neighbors);
criterion_main!(benches);
