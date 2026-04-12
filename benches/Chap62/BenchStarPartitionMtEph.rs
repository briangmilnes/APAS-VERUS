//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 62: Star partition (multi-threaded, ephemeral) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap05::SetStEph::SetStEph::*;
use apas_verus::Chap06::UnDirGraphMtEph::UnDirGraphMtEph::*;
use apas_verus::Chap62::StarPartitionMtEph::StarPartitionMtEph::*;
use apas_verus::SetLit;
use apas_verus::Types::Types::*;

fn make_cycle_graph(n: usize) -> UnDirGraphMtEph<usize> {
    let mut vertices: SetStEph<usize> = SetLit![];
    let mut edges: SetStEph<Edge<usize>> = SetLit![];
    for i in 0..n { let _ = vertices.insert(i); }
    for i in 0..n { let _ = edges.insert(Edge(i, (i + 1) % n)); }
    <UnDirGraphMtEph<usize> as UnDirGraphMtEphTrait<usize>>::from_sets(vertices, edges)
}

fn bench_parallel_star_partition(c: &mut Criterion) {
    let mut group = c.benchmark_group("StarPartitionMtEph");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(400));
    for &n in &[16usize, 32] {
        let graph = make_cycle_graph(n);
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, _| {
            b.iter(|| parallel_star_partition(&graph, 42));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_parallel_star_partition);
criterion_main!(benches);
