// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Chapter 55: Topological sort (sequential, ephemeral) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap19::ArraySeqStEph::ArraySeqStEph::*;
use apas_verus::Chap55::TopoSortStEph::TopoSortStEph::*;

fn make_dag(n: usize) -> ArraySeqStEphS<ArraySeqStEphS<usize>> {
    // DAG: each node i points to i+1 and i+2 (if in range).
    let adj: Vec<ArraySeqStEphS<usize>> = (0..n)
        .map(|i| {
            let mut neighbors = vec![];
            if i + 1 < n { neighbors.push(i + 1); }
            if i + 2 < n { neighbors.push(i + 2); }
            ArraySeqStEphS::from_vec(neighbors)
        })
        .collect();
    ArraySeqStEphS::from_vec(adj)
}

fn bench_topo_sort(c: &mut Criterion) {
    let mut group = c.benchmark_group("TopoSortStEph");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));
    for &n in &[32usize, 64] {
        group.bench_with_input(BenchmarkId::new("dag_n", n), &n, |b, &n| {
            let graph = make_dag(n);
            b.iter(|| TopoSortStEph::topo_sort(&graph));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_topo_sort);
criterion_main!(benches);
