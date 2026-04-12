//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 63: Graph connectivity (sequential, ephemeral) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap05::SetStEph::SetStEph::*;
use apas_verus::Chap06::UnDirGraphStEph::UnDirGraphStEph::*;
use apas_verus::Chap63::ConnectivityStEph::ConnectivityStEph::*;
use apas_verus::Types::Types::*;
use apas_verus::SetLit;

fn make_ring_graph(n: usize) -> UnDirGraphStEph<usize> {
    let mut vertices: SetStEph<usize> = SetLit![];
    for i in 0..n { let _ = vertices.insert(i); }
    let mut edges: SetStEph<Edge<usize>> = SetLit![];
    for i in 0..n { let _ = edges.insert(Edge(i, (i + 1) % n)); }
    <UnDirGraphStEph<usize> as UnDirGraphStEphTrait<usize>>::from_sets(vertices, edges)
}

fn bench_connectivity(c: &mut Criterion) {
    let mut group = c.benchmark_group("ConnectivityStEph");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));
    for &n in &[16usize, 32] {
        group.bench_with_input(BenchmarkId::new("ring_n", n), &n, |b, &n| {
            let graph = make_ring_graph(n);
            b.iter(|| count_components(&graph));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_connectivity);
criterion_main!(benches);
