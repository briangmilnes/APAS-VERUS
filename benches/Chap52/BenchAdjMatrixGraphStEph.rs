// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Chapter 52: Adjacency-matrix graph (sequential, ephemeral) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap52::AdjMatrixGraphStEph::AdjMatrixGraphStEph::*;

fn bench_adj_matrix_graph(c: &mut Criterion) {
    let mut group = c.benchmark_group("AdjMatrixGraphStEph");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));
    for &n in &[16usize, 32] {
        group.bench_with_input(BenchmarkId::new("set_edge", n), &n, |b, &n| {
            let mut graph: AdjMatrixGraphStEph = AdjMatrixGraphStEphTrait::new(n);
            b.iter(|| {
                graph.set_edge(0, 1, true);
                graph.out_neighbors(0)
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_adj_matrix_graph);
criterion_main!(benches);
