//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 52: Adjacency-table graph (sequential, ephemeral) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap52::AdjTableGraphStEph::AdjTableGraphStEph::*;

fn bench_adj_table_graph(c: &mut Criterion) {
    let mut group = c.benchmark_group("AdjTableGraphStEph");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));
    for &n in &[16usize, 32] {
        group.bench_with_input(BenchmarkId::new("insert_edge", n), &n, |b, &n| {
            b.iter(|| {
                let mut g = AdjTableGraphStEph::<usize>::empty();
                for i in 0..n {
                    g.insert_edge(i, (i + 1) % n);
                }
                g.num_edges()
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_adj_table_graph);
criterion_main!(benches);
