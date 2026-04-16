// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Chapter 57: Dijkstra SSSP (sequential, ephemeral, f64 weights) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap05::SetStEph::SetStEph::*;
use apas_verus::Chap06::WeightedDirGraphStEphF64::WeightedDirGraphStEphF64::*;
use apas_verus::Chap57::DijkstraStEphF64::DijkstraStEphF64::*;
use apas_verus::Types::Types::WeightedEdge;
use apas_verus::vstdplus::float::float::*;
use apas_verus::SetLit;

fn w(v: f64) -> WrappedF64 {
    WrappedF64 { val: v }
}

fn make_chain_graph_f64(n: usize) -> WeightedDirGraphStEphF64<usize> {
    let mut vertices: SetStEph<usize> = SetLit![];
    for i in 0..n { let _ = vertices.insert(i); }
    let mut edges: SetStEph<WeightedEdge<usize, WrappedF64>> = SetLit![];
    for i in 0..n - 1 {
        let _ = edges.insert(WeightedEdge(i, i + 1, w((i as f64) + 1.0)));
    }
    <WeightedDirGraphStEphF64<usize> as WeightedDirGraphStEphF64Trait<usize>>::from_weighed_edges(vertices, edges)
}

fn bench_dijkstra_f64(c: &mut Criterion) {
    let mut group = c.benchmark_group("DijkstraStEphF64");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));
    for &n in &[16usize, 32] {
        group.bench_with_input(BenchmarkId::new("chain_n", n), &n, |b, &n| {
            let graph = make_chain_graph_f64(n);
            b.iter(|| dijkstra(&graph, 0));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_dijkstra_f64);
criterion_main!(benches);
