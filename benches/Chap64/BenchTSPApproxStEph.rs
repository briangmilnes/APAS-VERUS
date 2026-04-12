//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 64: TSP 2-approximation (sequential, ephemeral) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap05::SetStEph::SetStEph::*;
use apas_verus::Chap06::LabUnDirGraphStEph::LabUnDirGraphStEph::*;
use apas_verus::Chap64::TSPApproxStEph::TSPApproxStEph::*;
use apas_verus::Types::Types::LabEdge;
use apas_verus::vstdplus::float::float::*;
use apas_verus::SetLit;

fn w(v: f64) -> WrappedF64 {
    WrappedF64 { val: v }
}

/// Build a complete labeled undirected graph on n vertices with Euclidean-like weights.
fn make_complete_graph(n: usize) -> (
    LabUnDirGraphStEph<usize, WrappedF64>,
    SetStEph<LabEdge<usize, WrappedF64>>,
) {
    let mut vertices: SetStEph<usize> = SetLit![];
    for i in 0..n { let _ = vertices.insert(i); }

    let mut edges: SetStEph<LabEdge<usize, WrappedF64>> = SetLit![];
    for i in 0..n {
        for j in (i + 1)..n {
            let dist = w(((j - i) as f64));
            let _ = edges.insert(LabEdge(i, j, dist));
        }
    }

    let graph = <LabUnDirGraphStEph<usize, WrappedF64> as LabUnDirGraphStEphTrait<usize, WrappedF64>>::from_vertices_and_labeled_edges(vertices, edges);

    // Spanning tree: chain 0-1-2-..-(n-1).
    let mut spanning_tree: SetStEph<LabEdge<usize, WrappedF64>> = SetLit![];
    for i in 0..n - 1 {
        let _ = spanning_tree.insert(LabEdge(i, i + 1, w(1.0)));
    }

    (graph, spanning_tree)
}

fn bench_tsp_approx(c: &mut Criterion) {
    let mut group = c.benchmark_group("TSPApproxStEph");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));
    for &n in &[5usize, 8] {
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            let (graph, spanning_tree) = make_complete_graph(n);
            b.iter(|| approx_metric_tsp(&graph, &spanning_tree, &0));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_tsp_approx);
criterion_main!(benches);
