// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Chapter 61: Vertex matching (sequential, ephemeral) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap05::SetStEph::SetStEph::*;
use apas_verus::Chap06::UnDirGraphStEph::UnDirGraphStEph::*;
use apas_verus::Chap61::VertexMatchingStEph::VertexMatchingStEph::*;
use apas_verus::SetLit;
use apas_verus::Types::Types::*;

fn make_cycle_graph(n: usize) -> UnDirGraphStEph<usize> {
    let mut vertices: SetStEph<usize> = SetLit![];
    let mut edges: SetStEph<Edge<usize>> = SetLit![];
    for i in 0..n { let _ = vertices.insert(i); }
    for i in 0..n {
        let j = (i + 1) % n;
        let _ = edges.insert(if i < j { Edge(i, j) } else { Edge(j, i) });
    }
    <UnDirGraphStEph<usize> as UnDirGraphStEphTrait<usize>>::from_sets(vertices, edges)
}

fn bench_greedy_matching(c: &mut Criterion) {
    let mut group = c.benchmark_group("VertexMatchingStEphGreedy");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(400));
    for &n in &[16usize, 32] {
        let graph = make_cycle_graph(n);
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, _| {
            b.iter(|| greedy_matching(&graph));
        });
    }
    group.finish();
}

fn bench_parallel_matching_st(c: &mut Criterion) {
    let mut group = c.benchmark_group("VertexMatchingStEphParallel");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(400));
    for &n in &[16usize, 32] {
        let graph = make_cycle_graph(n);
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, _| {
            b.iter(|| parallel_matching_st(&graph, 42));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_greedy_matching, bench_parallel_matching_st);
criterion_main!(benches);
