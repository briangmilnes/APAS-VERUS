// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Chapter 61: Vertex matching (multi-threaded, ephemeral) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap05::SetStEph::SetStEph::*;
use apas_verus::Chap06::UnDirGraphMtEph::UnDirGraphMtEph::*;
use apas_verus::Chap61::VertexMatchingMtEph::VertexMatchingMtEph::*;
use apas_verus::SetLit;
use apas_verus::Types::Types::*;
use apas_verus::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::set_parallelism;

fn make_cycle_graph(n: usize) -> UnDirGraphMtEph<usize> {
    let mut vertices: SetStEph<usize> = SetLit![];
    let mut edges: SetStEph<Edge<usize>> = SetLit![];
    for i in 0..n { let _ = vertices.insert(i); }
    for i in 0..n {
        let j = (i + 1) % n;
        let _ = edges.insert(if i < j { Edge(i, j) } else { Edge(j, i) });
    }
    <UnDirGraphMtEph<usize> as UnDirGraphMtEphTrait<usize>>::from_sets(vertices, edges)
}

fn bench_parallel_matching_mt(c: &mut Criterion) {
    set_parallelism(10);
    let mut group = c.benchmark_group("VertexMatchingMtEph");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(400));
    for &n in &[16usize, 32] {
        let graph = make_cycle_graph(n);
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, _| {
            b.iter(|| parallel_matching_mt(&graph, 42));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_parallel_matching_mt);
criterion_main!(benches);
