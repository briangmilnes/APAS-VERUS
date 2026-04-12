//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 65: Kruskal MST (sequential, ephemeral) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap05::SetStEph::SetStEph::*;
use apas_verus::Chap06::LabUnDirGraphStEph::LabUnDirGraphStEph::*;
use apas_verus::Chap65::KruskalStEph::KruskalStEph::*;
use apas_verus::SetLit;
use apas_verus::Types::Types::*;

fn make_chain_graph(n: usize) -> LabUnDirGraphStEph<usize, u64> {
    let mut vertices: SetStEph<usize> = SetLit![];
    for i in 0..n { let _ = vertices.insert(i); }
    let mut edges: SetStEph<LabEdge<usize, u64>> = SetLit![];
    for i in 0..n - 1 {
        let _ = edges.insert(LabEdge(i, i + 1, (i + 1) as u64));
    }
    <LabUnDirGraphStEph<usize, u64> as LabUnDirGraphStEphTrait<usize, u64>>::from_vertices_and_labeled_edges(vertices, edges)
}

fn bench_kruskal(c: &mut Criterion) {
    let mut group = c.benchmark_group("KruskalStEph");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));
    for &n in &[16usize, 32] {
        group.bench_with_input(BenchmarkId::new("chain_n", n), &n, |b, &n| {
            let graph = make_chain_graph(n);
            b.iter(|| kruskal_mst(&graph));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_kruskal);
criterion_main!(benches);
