//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 52: Adjacency-matrix graph (multi-threaded, persistent) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap52::AdjMatrixGraphMtPer::AdjMatrixGraphMtPer::*;
use apas_verus::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::set_parallelism;

fn bench_adjmatrixgraph_mt_per_num_edges(c: &mut Criterion) {
    set_parallelism(10);
    let mut group = c.benchmark_group("AdjMatrixGraphMtPerNumEdges");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(200));
    for &n in &[16usize, 32] {
        let g = <AdjMatrixGraphMtPer as AdjMatrixGraphMtPerTrait>::new(n);
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, _| {
            b.iter(|| g.num_edges());
        });
    }
    group.finish();
}

criterion_group!(benches, bench_adjmatrixgraph_mt_per_num_edges);
criterion_main!(benches);
