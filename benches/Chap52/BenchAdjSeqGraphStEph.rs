//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 52: Adjacency-sequence graph (sequential, ephemeral) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap52::AdjSeqGraphStEph::AdjSeqGraphStEph::*;

fn bench_adj_seq_graph(c: &mut Criterion) {
    let mut group = c.benchmark_group("AdjSeqGraphStEph");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));
    for &n in &[16usize, 32] {
        group.bench_with_input(BenchmarkId::new("set_edge", n), &n, |b, &n| {
            let mut graph: AdjSeqGraphStEph = AdjSeqGraphStEphTrait::new(n);
            b.iter(|| {
                graph.set_edge(0, 1, true);
                graph.out_neighbors(0)
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_adj_seq_graph);
criterion_main!(benches);
