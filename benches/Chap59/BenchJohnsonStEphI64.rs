//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 59: Johnson all-pairs shortest paths (sequential, ephemeral) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap05::SetStEph::SetStEph::*;
use apas_verus::Chap06::WeightedDirGraphStEphI128::WeightedDirGraphStEphI128::*;
use apas_verus::Chap59::JohnsonStEphI64::JohnsonStEphI64::*;
use apas_verus::Types::Types::WeightedEdge;
use apas_verus::SetLit;

fn make_chain_graph(n: usize) -> WeightedDirGraphStEphI128<usize> {
    let mut vertices: SetStEph<usize> = SetLit![];
    for i in 0..n { let _ = vertices.insert(i); }
    let mut edges: SetStEph<WeightedEdge<usize, i128>> = SetLit![];
    for i in 0..n - 1 {
        let _ = edges.insert(WeightedEdge(i, i + 1, (i as i128) + 1));
    }
    WeightedDirGraphStEphI128::from_weighed_edges(vertices, edges)
}

fn bench_johnson(c: &mut Criterion) {
    let mut group = c.benchmark_group("JohnsonStEphI64");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));
    for &n in &[8usize, 16] {
        group.bench_with_input(BenchmarkId::new("chain_n", n), &n, |b, &n| {
            let graph = make_chain_graph(n);
            b.iter(|| johnson_apsp(&graph));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_johnson);
criterion_main!(benches);
