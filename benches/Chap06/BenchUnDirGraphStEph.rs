//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 06: Undirected graph (sequential, ephemeral) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap06::UnDirGraphStEph::UnDirGraphStEph::*;
use apas_verus::Chap05::SetStEph::SetStEph::*;
use apas_verus::Types::Types::Edge;
use apas_verus::SetLit;

fn build_ring_graph(n: usize) -> UnDirGraphStEph<u64> {
    let mut verts: SetStEph<u64> = SetLit![];
    let mut edges: SetStEph<Edge<u64>> = SetLit![];
    for i in 0..n as u64 { let _ = verts.insert(i); }
    for i in 0..n as u64 { let _ = edges.insert(Edge(i, (i + 1) % n as u64)); }
    <UnDirGraphStEph<u64> as UnDirGraphStEphTrait<u64>>::from_sets(verts, edges)
}

fn bench_undir_graph_build(c: &mut Criterion) {
    let mut group = c.benchmark_group("UnDirGraphStEphBuild");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(200));
    for &n in &[32usize, 128] {
        group.bench_with_input(BenchmarkId::new("ring_n", n), &n, |b, &n| {
            b.iter(|| build_ring_graph(n));
        });
    }
    group.finish();
}

fn bench_undir_graph_sizeV(c: &mut Criterion) {
    let mut group = c.benchmark_group("UnDirGraphStEphSizeV");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(200));
    for &n in &[32usize, 128] {
        let g = build_ring_graph(n);
        group.bench_with_input(BenchmarkId::new("ring_n", n), &n, |b, _| {
            b.iter(|| g.sizeV());
        });
    }
    group.finish();
}

criterion_group!(benches, bench_undir_graph_build, bench_undir_graph_sizeV);
criterion_main!(benches);
