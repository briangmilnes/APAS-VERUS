//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 55: Topological sort (sequential, persistent) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap19::ArraySeqStPer::ArraySeqStPer::*;
use apas_verus::Chap55::TopoSortStPer::TopoSortStPer::*;

fn make_dag(n: usize) -> ArraySeqStPerS<ArraySeqStPerS<usize>> {
    let adj: Vec<ArraySeqStPerS<usize>> = (0..n)
        .map(|i| {
            let mut neighbors = vec![];
            if i + 1 < n { neighbors.push(i + 1); }
            if i + 2 < n { neighbors.push(i + 2); }
            ArraySeqStPerS { seq: neighbors }
        })
        .collect();
    ArraySeqStPerS { seq: adj }
}

fn bench_topo_sort_per(c: &mut Criterion) {
    let mut group = c.benchmark_group("TopoSortStPer");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));
    for &n in &[32usize, 64] {
        group.bench_with_input(BenchmarkId::new("dag_n", n), &n, |b, &n| {
            let graph = make_dag(n);
            b.iter(|| TopoSortStPer::topo_sort(&graph));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_topo_sort_per);
criterion_main!(benches);
