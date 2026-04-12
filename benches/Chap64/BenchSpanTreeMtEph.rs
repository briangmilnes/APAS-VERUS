//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 64: Spanning tree (multi-threaded, ephemeral) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap05::SetStEph::SetStEph::*;
use apas_verus::Chap06::UnDirGraphMtEph::UnDirGraphMtEph::*;
use apas_verus::Chap64::SpanTreeMtEph::SpanTreeMtEph::*;
use apas_verus::SetLit;
use apas_verus::Types::Types::*;

fn make_cycle_graph(n: usize) -> UnDirGraphMtEph<usize> {
    let mut vertices: SetStEph<usize> = SetLit![];
    let mut edges: SetStEph<Edge<usize>> = SetLit![];
    for i in 0..n { let _ = vertices.insert(i); }
    for i in 0..n { let _ = edges.insert(Edge(i, (i + 1) % n)); }
    <UnDirGraphMtEph<usize> as UnDirGraphMtEphTrait<usize>>::from_sets(vertices, edges)
}

fn bench_spanning_tree_mt(c: &mut Criterion) {
    let mut group = c.benchmark_group("SpanTreeMtEph");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(400));
    for &n in &[16usize, 32] {
        let graph = make_cycle_graph(n);
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, _| {
            b.iter(|| spanning_tree_star_contraction_mt(&graph, 42));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_spanning_tree_mt);
criterion_main!(benches);
